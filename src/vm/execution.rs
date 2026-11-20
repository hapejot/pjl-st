use std::{
    error::Error,
    fmt::Display,
    sync::{
        Arc, Mutex, MutexGuard, Weak,
        atomic::{AtomicU64, Ordering},
    },
};

use tracing::trace;

use crate::{
    memory::Value,
    vm::{
        VirtualMachine, VirtualMachineState,
        block::CompiledMethod,
        error::{ExecutionError, RuntimeError},
        instruction::Instruction,
        register::Register,
    },
};

mod blocks;
mod dispatch;
mod instructions;
mod registers;

#[derive(Debug, Clone)]
pub struct Execution(Arc<Mutex<ExecutionData>>);

/// Optional limits for one top-level execution. `None` means unlimited.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionLimits {
    /// Maximum bytecode instructions executed across all frames.
    pub max_instructions: Option<u64>,
    /// Maximum active Smalltalk frames, including the top-level frame.
    pub max_call_depth: Option<usize>,
}

#[derive(Debug)]
struct ExecutionBudget {
    remaining: Option<AtomicU64>,
    max_call_depth: Option<usize>,
}

impl ExecutionBudget {
    fn new(limits: ExecutionLimits) -> Self {
        Self {
            remaining: limits.max_instructions.map(AtomicU64::new),
            max_call_depth: limits.max_call_depth,
        }
    }

    fn consume_instruction(&self) -> Result<(), RuntimeError> {
        if let Some(remaining) = &self.remaining {
            let mut value = remaining.load(Ordering::Relaxed);
            loop {
                let next = value
                    .checked_sub(1)
                    .ok_or(RuntimeError::InstructionLimitExceeded)?;
                match remaining.compare_exchange_weak(
                    value,
                    next,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(current) => value = current,
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ExecutionData {
    vm: Weak<VirtualMachineState>,
    pub home: Option<Execution>,
    pub code: CompiledMethod,
    pub ip: usize,
    pub block: Option<usize>,
    pub registers: Vec<Value>,
    pub argument_count: usize,
    pub display: Vec<Execution>,
    pub msg: String,
    pub level: usize,
    budget: Option<Arc<ExecutionBudget>>,
    call_depth: usize,
    run_id: Arc<()>,
    pub active: bool,
    pub halted: bool,
}

#[derive(Debug)]
struct NonLocalReturn {
    target: Execution,
    value: Value,
}

impl Display for NonLocalReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Non-local return")
    }
}

impl Error for NonLocalReturn {}

impl Execution {
    pub fn new(
        vm: &VirtualMachine,
        receiver: Value,
        args: Vec<Value>,
        code: CompiledMethod,
        level: usize,
    ) -> Self {
        Self::new_with_budget(vm, receiver, args, code, level, None, 1, Arc::new(()))
    }

    pub(crate) fn new_with_limits(
        vm: &VirtualMachine,
        code: CompiledMethod,
        limits: ExecutionLimits,
    ) -> Self {
        Self::new_with_budget(
            vm,
            Value::Undefined,
            vec![],
            code,
            0,
            Some(Arc::new(ExecutionBudget::new(limits))),
            1,
            Arc::new(()),
        )
    }

    fn new_with_budget(
        vm: &VirtualMachine,
        receiver: Value,
        args: Vec<Value>,
        code: CompiledMethod,
        level: usize,
        budget: Option<Arc<ExecutionBudget>>,
        call_depth: usize,
        run_id: Arc<()>,
    ) -> Self {
        let argument_count = args.len();
        let mut registers = vec![receiver.clone(), receiver.clone()];
        registers.extend(args);
        Self(Arc::new(Mutex::new(ExecutionData {
            vm: Arc::downgrade(&vm.state),
            home: None,
            code,
            ip: 0,
            block: None,
            registers,
            argument_count,
            display: vec![],
            msg: String::new(),
            level,
            budget,
            call_depth,
            run_id,
            active: true,
            halted: false,
        })))
    }

    fn lock(&self) -> Result<MutexGuard<'_, ExecutionData>, RuntimeError> {
        self.0
            .lock()
            .map_err(|error| RuntimeError::ExecutionLock(error.to_string()))
    }

    pub fn home(&self) -> Result<Option<Execution>, RuntimeError> {
        Ok(self.lock()?.home.clone())
    }

    // #[tracing::instrument(skip(self))]
    /// returns the value in the Result register after execution or error if the register is not set
    pub fn execute(&self) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let _vm = self.vm()?;
        let result = self
            .check_call_depth()
            .and_then(|()| self.check_method_arguments())
            .map_err(Box::<dyn Error + Send + Sync>::from)
            .and_then(|()| self.execute_instructions());
        match (result, self.set_active(false)) {
            (Err(error), Ok(())) if error.is::<RuntimeError>() => {
                Err(ExecutionError::at(error, self.entry_frame()))
            }
            (result, Ok(())) => result,
            (Ok(_), Err(error)) => Err(error.into()),
            (Err(error), Err(cleanup)) => Err(ExecutionError::at(
                error,
                format!("<failed to deactivate execution: {cleanup}>"),
            )),
        }
    }

    fn entry_frame(&self) -> String {
        match self.msg() {
            Ok(msg) if !msg.is_empty() => format!("{msg} [entry]"),
            Ok(_) => "<script> [entry]".to_string(),
            Err(error) => format!("<stack frame unavailable: {error}>"),
        }
    }

    fn check_call_depth(&self) -> Result<(), RuntimeError> {
        let exec = self.lock()?;
        if let Some(max) = exec
            .budget
            .as_ref()
            .and_then(|budget| budget.max_call_depth)
        {
            if exec.call_depth > max {
                return Err(RuntimeError::CallDepthLimitExceeded { max });
            }
        }
        Ok(())
    }

    fn child_context(
        &self,
    ) -> Result<(Option<Arc<ExecutionBudget>>, usize, Arc<()>), RuntimeError> {
        let exec = self.lock()?;
        Ok((
            exec.budget.clone(),
            exec.call_depth
                .checked_add(1)
                .ok_or(RuntimeError::CallDepthOverflow)?,
            exec.run_id.clone(),
        ))
    }

    fn check_argument_count(
        method: String,
        expected: usize,
        actual: usize,
    ) -> Result<(), RuntimeError> {
        if expected != actual {
            return Err(RuntimeError::MethodArgumentCount {
                method,
                expected,
                actual,
            });
        }
        Ok(())
    }

    fn check_method_arguments(&self) -> Result<(), RuntimeError> {
        let exec = self.lock()?;
        if exec.block.is_none() {
            let name = if exec.msg.is_empty() {
                "<anonymous>"
            } else {
                &exec.msg
            };
            Self::check_argument_count(
                name.to_owned(),
                exec.code.parameter_count(),
                exec.argument_count,
            )?;
        }
        Ok(())
    }

    fn execute_instructions(&self) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let (instructions, vm, is_block, budget) = {
            let exec = self.lock()?;
            let inst = match exec.block {
                Some(n) => exec
                    .code
                    .blocks()
                    .get(n)
                    .ok_or(RuntimeError::BlockNotFound(n))?
                    .instructions
                    .clone(),
                None => exec.code.instructions().clone(),
            };
            let vm = exec
                .vm
                .upgrade()
                .ok_or(RuntimeError::VirtualMachineDropped)?;
            (inst, vm, exec.block.is_some(), exec.budget.clone())
        };
        let debugger = vm
            .debugger
            .lock()
            .map_err(|e| format!("Debugger lock: {e}"))?
            .clone();

        while self.ip()? < instructions.len() {
            let instr = &instructions[self.ip()?];
            if let Some(budget) = &budget {
                budget
                    .consume_instruction()
                    .map_err(|error| self.error_at(error.into(), instr))?;
            }
            // Call debugger before executing instruction
            if let Some(debugger) = &debugger {
                debugger
                    .before_execute(self.clone())
                    .map_err(|error| self.error_at(error, instr))?;
            }

            match self.execute_instruction(instr) {
                Ok(()) => {}
                Err(error) => {
                    if let Some(ret) = error.downcast_ref::<NonLocalReturn>() {
                        if self.same_activation(&ret.target) {
                            self.set(Register::Result, ret.value.clone())?;
                            self.halt()?;
                        } else {
                            return Err(error);
                        }
                    } else {
                        return Err(self.error_at(error, instr));
                    }
                }
            }
            // Call debugger after executing instruction
            if let Some(debugger) = &debugger {
                debugger
                    .after_execute(self.clone())
                    .map_err(|error| self.error_at(error, instr))?;
            }
            if self.is_halted()? {
                break;
            }
            self.next_ip()?;
        }

        self.dump_registers();
        let result_register = if is_block {
            Register::Number(0, 0)
        } else {
            Register::Result
        };
        let r = self
            .get(result_register)?
            .ok_or(RuntimeError::RegisterNotAssigned(result_register).into());
        r
    }

    fn error_at(
        &self,
        error: Box<dyn Error + Send + Sync>,
        instr: &Instruction,
    ) -> Box<dyn Error + Send + Sync> {
        let frame = match self.error_frame(instr) {
            Ok(frame) => frame,
            Err(error) => format!("<stack frame unavailable: {error}>"),
        };
        ExecutionError::at(error, frame)
    }

    fn error_frame(&self, instr: &Instruction) -> Result<String, Box<dyn Error + Send + Sync>> {
        let msg = self.msg()?;
        let name = if msg.is_empty() { "<script>" } else { &msg };
        let operation = match instr {
            Instruction::CallMethod {
                selector, receiver, ..
            } => {
                let receiver = match self.get(*receiver)? {
                    Some(Value::Object(obj)) => match obj.class().and_then(|class| class.name()) {
                        Ok(class) => format!("a {class}"),
                        Err(error) => format!("<receiver class unavailable: {error}>"),
                    },
                    Some(value) => value.to_string(),
                    None => format!("unassigned register {receiver}"),
                };
                format!("send #{selector} to {receiver}")
            }
            _ => instr.to_string(),
        };
        Ok(format!("{name} [ip {}]: {operation}", self.ip()?))
    }

    pub(crate) fn same_activation(&self, other: &Execution) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    fn set_active(&self, active: bool) -> Result<(), RuntimeError> {
        self.lock()?.active = active;
        Ok(())
    }

    fn halt(&self) -> Result<(), RuntimeError> {
        self.lock()?.halted = true;
        Ok(())
    }

    fn is_halted(&self) -> Result<bool, RuntimeError> {
        Ok(self.lock()?.halted)
    }

    fn dump_registers(&self) {
        // let registers = self.registers.try_lock().unwrap();
        // for (idx, reg) in registers.iter().enumerate() {
        //     trace!("{}: {}", idx, reg);
        // }
    }

    pub fn instructions(&self) -> Result<Vec<Instruction>, RuntimeError> {
        let exec = self.lock()?;
        match exec.block {
            Some(n) => Ok(exec
                .code
                .blocks()
                .get(n)
                .ok_or(RuntimeError::BlockNotFound(n))?
                .instructions
                .clone()),
            None => Ok(exec.code.instructions().clone()),
        }
    }

    pub fn ip(&self) -> Result<usize, RuntimeError> {
        Ok(self.lock()?.ip)
    }

    pub fn msg(&self) -> Result<String, RuntimeError> {
        Ok(self.lock()?.msg.clone())
    }

    pub fn set_msg(&self, msg: String) -> Result<(), RuntimeError> {
        self.lock()?.msg = msg;
        Ok(())
    }

    pub fn next_ip(&self) -> Result<(), RuntimeError> {
        self.lock()?.ip += 1;
        Ok(())
    }

    fn vm(&self) -> Result<VirtualMachine, RuntimeError> {
        let exec = self.lock()?;
        let state = exec
            .vm
            .upgrade()
            .ok_or(RuntimeError::VirtualMachineDropped)?;
        Ok(VirtualMachine { state })
    }

    pub fn dump_to_trace_level(&self, level: usize) -> Result<(), RuntimeError> {
        trace!("Execution IP: {}", self.ip()?);
        let registers = self.registers()?;
        for (idx, reg) in registers.iter().enumerate() {
            trace!("R{}.{}: {}", level, idx, reg);
        }

        if let Some(home) = self.home()? {
            trace!("Home Execution:");
            home.dump_to_trace_level(level + 1)?;
        }
        Ok(())
    }
    pub fn dump_to_trace(&self) -> Result<(), RuntimeError> {
        self.dump_to_trace_level(0)
    }

    pub fn level(&self) -> Result<usize, RuntimeError> {
        Ok(self.lock()?.level)
    }
}

impl Display for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (level, ip, msg, home) = match self.0.try_lock() {
            Ok(exec) => (exec.level, exec.ip, exec.msg.clone(), exec.home.clone()),
            Err(error) => return write!(f, "<execution unavailable: {error}>"),
        };
        write!(
            f,
            "Execution level: {}, ip: {}, msg: '{}' {}",
            level,
            ip,
            msg,
            home.as_ref()
                .map_or(String::new(), |h| format!("Home: {}", h))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_access_waits_for_lock_contention() {
        let vm = VirtualMachine::new();
        let execution = Execution::new(
            &vm,
            Value::Nil,
            vec![],
            CompiledMethod::new(vec![], vec![], 0),
            0,
        );
        let guard = execution.0.lock().unwrap();
        assert!(execution.to_string().contains("execution unavailable"));
        let other = execution.clone();
        let (started, waiting) = std::sync::mpsc::channel();
        let (done, result) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            started.send(()).unwrap();
            done.send(other.set(Register::Result, Value::Nil)).unwrap();
        });
        waiting.recv().unwrap();
        assert!(
            result
                .recv_timeout(std::time::Duration::from_millis(50))
                .is_err()
        );
        drop(guard);
        assert!(
            result
                .recv_timeout(std::time::Duration::from_secs(2))
                .unwrap()
                .is_ok()
        );
        thread.join().unwrap();
    }

    #[test]
    fn poisoned_execution_lock_is_reported() {
        let vm = VirtualMachine::new();
        let execution = Execution::new(
            &vm,
            Value::Nil,
            vec![],
            CompiledMethod::new(vec![], vec![], 0),
            0,
        );
        let other = execution.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison execution lock");
            })
            .join()
            .is_err()
        );
        assert!(matches!(
            execution.registers(),
            Err(RuntimeError::ExecutionLock(_))
        ));
        assert!(
            execution
                .execute()
                .unwrap_err()
                .to_string()
                .contains("Execution lock")
        );
    }

    #[test]
    fn cleanup_lock_failure_does_not_replace_original_error() {
        struct PoisoningDebugger;
        impl crate::debug::Debugger for PoisoningDebugger {
            fn before_execute(
                &self,
                execution: Execution,
            ) -> Result<(), Box<dyn Error + Send + Sync>> {
                assert!(
                    std::thread::spawn(move || {
                        let _guard = execution.0.lock().unwrap();
                        panic!("poison execution lock");
                    })
                    .join()
                    .is_err()
                );
                Err("debugger failure".into())
            }

            fn after_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
                Ok(())
            }
        }

        let vm = VirtualMachine::new();
        vm.set_debugger(Arc::new(PoisoningDebugger)).unwrap();
        let execution = Execution::new(
            &vm,
            Value::Nil,
            vec![],
            CompiledMethod::new(vec![Instruction::Nop], vec![], 0),
            0,
        );
        let error = execution.execute().unwrap_err().to_string();
        assert!(error.contains("debugger failure"), "{error}");
        assert!(
            error.contains("stack frame unavailable: Execution lock"),
            "{error}"
        );
        assert!(
            error.contains("failed to deactivate execution: Execution lock"),
            "{error}"
        );
    }
}
