use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use tracing::trace;

use crate::{
    memory::Value,
    vm::{
        VirtualMachine, block::CompiledMethod, error::RuntimeError, instruction::Instruction,
        register::Register,
    },
};

#[derive(Debug, Clone)]
pub struct Execution(Arc<Mutex<ExecutionData>>);

#[derive(Debug)]
struct ExecutionData {
    pub vm: &'static VirtualMachine,
    pub home: Option<Execution>,
    pub code: CompiledMethod,
    pub ip: usize,
    pub block: Option<usize>,
    pub registers: Vec<Value>,
    pub display: Vec<Execution>,
}

impl Execution {
    pub fn new(
        vm: &'static VirtualMachine,
        receiver: Value,
        args: Vec<Value>,
        code: CompiledMethod,
    ) -> Self {
        let mut registers = vec![receiver.clone(), receiver.clone()];
        registers.extend(args);
        Self(Arc::new(Mutex::new(ExecutionData {
            vm,
            home: None,
            code,
            ip: 0,
            block: None,
            registers,
            display: vec![],
        })))
    }

    pub fn reg_count(&self) -> usize {
        match self.home().clone() {
            Some(home) => {
                let exec = self.0.try_lock().unwrap();
                let registers = &exec.registers;
                let n = home.reg_count();
                n + registers.len()
            }
            None => {
                let exec = self.0.try_lock().unwrap();
                let registers = &exec.registers;
                registers.len()
            }
        }
    }

    pub fn set(&self, reg: Register, value: Value) {
        match reg {
            Register::Number(0, index) => {
                self.internal_set_register(index, value);
            }
            Register::Number(level, index) if level > 0 => {
                if let Some(home) = self.home() {
                    home.set(Register::Number(level - 1, index), value);
                }
            }
            Register::Result => {
                self.internal_set_register(0, value);
            }            
            _ => {
                panic!("Invalid register {:?}", reg);
            }
        }
    }

    fn home(&self) -> Option<Execution> {
        let exec = self.0.try_lock().unwrap();
        exec.home.clone()
    }

    fn internal_set_register(&self, reg: usize, value: Value) {
        let mut exec = self.0.try_lock().unwrap();
        let registers = &mut exec.registers;
        if reg >= registers.len() {
            registers.resize(reg + 1, Value::Undefined);
        }
        registers[reg] = value;
    }

    fn internal_get_register(&self, reg: usize) -> Option<Value> {
        let mut exec = self.0.try_lock().unwrap();
        let registers = &mut exec.registers;
        if reg >= registers.len() {
            registers.resize(reg + 1, Value::Undefined);
        }
        registers.get(reg).cloned()
    }

    pub fn get(&self, reg: Register) -> Option<Value> {
        match reg {
            Register::Result => {
                if let Some(home) = self.home() {
                    home.get(reg)
                } else {
                    self.internal_get_register(0)
                }
            }
            Register::Receiver => {
                if let Some(home) = self.home() {
                    home.get(reg)
                } else {
                    self.internal_get_register(1)
                }
            }
            Register::Number(0, index) => self.internal_get_register(index),
            Register::Number(level, index) if level > 0 => {
                if let Some(home) = self.home() {
                    home.get(Register::Number(level - 1, index))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn execute(&self) -> Result<Value, Box<dyn Error>> {
        let (instructions, debugger) = {
            let exec = self.0.try_lock().unwrap();
            let inst = match exec.block {
                Some(n) => exec.code.blocks()[n].instructions.clone(),
                None => exec.code.instructions().clone(),
            };
            let deb = exec.vm.debugger.clone();
            (inst, deb)
        };

        while self.ip() < instructions.len() {
            // Call debugger before executing instruction
            if let Some(debugger) = debugger {
                debugger.before_execute(self.clone());
            }

            let instr = &instructions[self.ip()];
            self.execute_instruction(instr)?;

            // Call debugger after executing instruction
            if let Some(debugger) = debugger {
                debugger.after_execute(self.clone());
            }
            self.next_ip();
        }

        self.dump_registers();
        self.get(Register::Result)
            .ok_or(RuntimeError::RegisterNotAssigned(Register::Result).into())
    }

    fn dump_registers(&self) {
        // let registers = self.registers.try_lock().unwrap();
        // for (idx, reg) in registers.iter().enumerate() {
        //     trace!("{}: {}", idx, reg);
        // }
    }

    fn execute_instruction(&self, instr: &Instruction) -> Result<(), Box<dyn Error>> {
        trace!("EXECUTE: {}", instr);
        match instr {
            Instruction::LoadImm { dst, value } => {
                self.set(*dst, value.clone());
                Ok(())
            }
            Instruction::LoadReceiver { dst } => {
                todo!()
            }
            Instruction::LoadGlobal { dst, var_name } => {
                self.set(*dst, self.get_global(var_name)?);
                Ok(())
            }
            Instruction::StoreLocal { src, var_name } => todo!(),
            Instruction::LoadInstanceVar { dst, index } => {
                if let Some(Value::Object(smalltalk_object)) = self.get(Register::Receiver) {
                    let value = smalltalk_object.get_instance_var(*index);
                    self.set(*dst, value);
                    Ok(())
                } else {
                    self.dump_registers();
                    Err("LOAD_IVAR: Receiver is not an object".into())
                }
            }
            Instruction::StoreInstanceVar { src, index } => {
                if let Some(value) = self.get(*src) {
                    if let Some(Value::Object(smalltalk_object)) = self.get(Register::Receiver) {
                        smalltalk_object.set_instance_var(*index, value);
                        Ok(())
                    } else {
                        self.dump_registers();
                        Err("STORE_IVAR: Receiver is not an object".into())
                    }
                } else {
                    Err(RuntimeError::RegisterNotAssigned(*src).into())
                }
            }
            Instruction::Move { dst, src } => {
                if let Some(value) = self.get(*src) {
                    self.set(*dst, value);
                    Ok(())
                } else {
                    Err(RuntimeError::RegisterNotAssigned(*src).into())
                }
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => self.execute_send_message(*dst, *receiver, args, selector),
            Instruction::Return { src } => {
                if let Some(value) = self.get(*src) {
                    self.set(Register::Result, value.clone());
                    trace!("RETURN: {}", value);
                    Ok(())
                } else {
                    Err(RuntimeError::RegisterNotAssigned(*src).into())
                }
            }
            Instruction::CreateBlock { dst, prog_id } => {
                self.execute_create_block(*dst, *prog_id)?;
                Ok(())
            }
            Instruction::CallBlock { dst, block_reg } => todo!(),
            Instruction::CallBlockWithArgs {
                dst,
                block_reg,
                args,
            } => todo!(),
            Instruction::Nop => Ok(()),
            x => todo!("{x}"),
        }
    }

    fn execute_create_block(&self, dst: Register, block_num: usize) -> Result<(), Box<dyn Error>> {
        let block = self.create_block(block_num);
        self.set(dst, Value::Execution(block));
        Ok(())
    }

    pub fn create_block(&self, block_num: usize) -> Execution {
        let exec = self.0.try_lock().unwrap();
        let registers = vec![];
        let vm = exec.vm;
        let code = exec.code.clone();
        let home = Some(self.clone());
        let mut display = exec.display.clone();
        display.insert(0, self.clone());

        let block = Execution(Arc::new(Mutex::new(ExecutionData {
            vm,
            home,
            code,
            ip: 0,
            block: Some(block_num),
            registers,
            display,
        })));
        block
    }

    fn execute_send_message(
        &self,
        dst: Register,
        receiver: Register,
        args: &Vec<Register>,
        selector: &'static str,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let r = self
            .get(receiver)
            .ok_or(RuntimeError::RegisterNotAssigned(receiver))?;
        let mut send_args = vec![];
        for a in args {
            send_args.push(self.get(*a).ok_or(RuntimeError::RegisterNotAssigned(*a))?);
        }
        trace!(
            "args: {}",
            send_args
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(", ")
        );
        trace!("resolve class for {r}");
        let c = self.lookup_class(&r)?;
        trace!("lookup method {selector} in  {c}");
        let m = self.lookup_method_in_class(c, selector)?;
        match m {
            Value::NativeMethod(func) => {
                let result = func(self, r, send_args)?;
                self.set(dst, result);
                Ok(())
            }
            Value::Method(cm) => {
                let exec = Execution::new(self.vm(), r, send_args, cm.clone());
                let result = exec.execute()?;
                self.set(dst, result);
                Ok(())
            }
            _ => Err("Only native methods are supported in this VM version".into()),
        }
    }

    fn lookup_class(&self, r: &Value) -> Result<Value, Box<dyn Error>> {
        match r {
            Value::String(_) => todo!(),
            Value::Integer(_) => Ok(self.vm().integer_class.clone()),
            Value::Float(_) => Ok(self.vm().float_class.clone()),
            Value::Character(_) => todo!(),
            Value::Boolean(b) => match b {
                true => self.vm().get_class("True"),
                false => self.vm().get_class("False"),
            },
            Value::Object(smalltalk_object) => {
                let class_value = smalltalk_object.class();
                Ok(class_value.into())
            }
            Value::Dictionary(hash_map) => todo!(),
            Value::Array(values) => todo!(),
            Value::Method(compiled_method) => todo!(),
            Value::NativeMethod(_) => todo!(),
            Value::Class(cls) => Ok(Value::Class(cls.meta())),
            Value::Execution(e) => Ok(self.vm().execution_class.clone()),
            Value::Undefined => todo!(),
            Value::Nil => self.vm().get_class("Nil"),
        }
    }

    fn lookup_method_in_class(&self, c: Value, selector: &str) -> Result<Value, String> {
        match c {
            Value::Class(cls) => {
                if let Some(m) = cls.lookup_method(selector) {
                    Ok(m.clone())
                } else {
                    let mut parent = cls.parent();
                    let mut backtrace = vec![cls.name()];
                    while let Some(p) = parent {
                        if let Some(m) = p.lookup_method(selector) {
                            return Ok(m.clone());
                        }
                        backtrace.push(p.name());
                        trace!("Method {} not found in {}, checking parent {:?}", selector, p, p.parent());
                        parent = p.parent();
                    }
                    Err(format!("Method '{}' not found in classes: {:}", selector, backtrace.join(", ")))
                }
            }
            _ => Err("Invalid class value".into()),
        }
    }

    pub fn instructions(&self) -> Vec<Instruction> {
        let exec = self.0.try_lock().unwrap();
        match exec.block {
            Some(n) => exec.code.blocks()[n].instructions.clone(),
            None => exec.code.instructions().clone(),
        }
    }

    pub fn ip(&self) -> usize {
        let exec = self.0.try_lock().unwrap();
        exec.ip
    }

    pub fn next_ip(&self) {
        let mut exec = self.0.try_lock().unwrap();
        exec.ip += 1;
    }

    pub fn registers(&self) -> Vec<String> {
        let exec = self.0.try_lock().unwrap();
        exec.registers.iter().map(|v| format!("{}", v)).collect()
    }

    fn get_global(&self, var_name: &'static str) -> Result<Value, Box<dyn Error>> {
        let exec = self.0.try_lock().unwrap();
        exec.vm.get_global(var_name)
    }

    fn vm(&self) -> &'static VirtualMachine {
        let exec = self.0.try_lock().unwrap();
        exec.vm
    }

    pub fn dump_to_trace_level(&self, level: usize) {
        trace!("Execution IP: {}", self.ip());
        let registers = self.registers();
        for (idx, reg) in registers.iter().enumerate() {
            trace!("R{}.{}: {}", level, idx, reg);
        }

        if let Some(home) = self.home() {
            trace!("Home Execution:");
            home.dump_to_trace_level(level + 1);
        }
    }
    pub fn dump_to_trace(&self) {
        self.dump_to_trace_level(0);
    }
}
