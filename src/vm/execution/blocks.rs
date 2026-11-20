use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{memory::Value, vm::error::RuntimeError};

use super::{Execution, ExecutionData};

impl Execution {
    pub(crate) fn invoke_block(
        &self,
        caller: &Execution,
        args: &[Value],
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let (vm, home, code, block_id, display, level, msg, parameter_count) = {
            let exec = self.lock()?;
            let block_id = exec.block.ok_or(RuntimeError::BlockNotCallable)?;
            let parameter_count = exec
                .code
                .blocks()
                .get(block_id)
                .ok_or(RuntimeError::BlockNotFound(block_id))?
                .parameter_count;
            (
                exec.vm.clone(),
                exec.home.clone(),
                exec.code.clone(),
                block_id,
                exec.display.clone(),
                exec.level + 1,
                exec.msg.clone(),
                parameter_count,
            )
        };

        if args.len() != parameter_count {
            return Err(RuntimeError::BlockArgumentCount {
                expected: parameter_count,
                actual: args.len(),
            }
            .into());
        }

        let (budget, call_depth, run_id) = caller.child_context()?;
        let mut registers = vec![Value::Nil; parameter_count + 1];
        registers[1..].clone_from_slice(args);
        let activation = Self(Arc::new(Mutex::new(ExecutionData {
            vm,
            home,
            code,
            ip: 0,
            block: Some(block_id),
            registers,
            argument_count: args.len(),
            display,
            msg,
            level,
            budget,
            call_depth,
            run_id,
            active: true,
            halted: false,
        })));
        activation.execute()
    }

    pub(super) fn is_block_activation(&self) -> Result<bool, RuntimeError> {
        Ok(self.lock()?.block.is_some())
    }

    pub(super) fn non_local_return_target(&self) -> Result<Execution, RuntimeError> {
        let run_id = self.lock()?.run_id.clone();
        let mut frame = self.clone();
        loop {
            let (is_block, home, active, same_run) = {
                let exec = frame.lock()?;
                (
                    exec.block.is_some(),
                    exec.home.clone(),
                    exec.active,
                    Arc::ptr_eq(&run_id, &exec.run_id),
                )
            };
            if !is_block {
                if !active {
                    return Err(RuntimeError::BlockCannotReturn);
                }
                if !same_run {
                    return Err(RuntimeError::BlockCannotReturnAcrossExecutions);
                }
                return Ok(frame);
            }
            frame = home.ok_or(RuntimeError::BlockCannotReturn)?;
        }
    }

    pub fn create_block(&self, level: usize, block_num: usize) -> Result<Execution, RuntimeError> {
        let exec = self.lock()?;
        let registers = vec![];
        let vm = exec.vm.clone();
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
            argument_count: 0,
            display,
            msg: format!("Block #{}", block_num),
            level,
            budget: None,
            call_depth: 1,
            run_id: exec.run_id.clone(),
            active: false,
            halted: false,
        })));
        Ok(block)
    }
}
