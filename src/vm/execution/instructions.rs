use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use tracing::trace;

use crate::{
    memory::{SmalltalkString, Value},
    vm::{error::RuntimeError, instruction::Instruction, register::Register},
};

use super::{Execution, NonLocalReturn};

// Keep the execution trace target when moving instruction handlers into a submodule.
impl Execution {
    pub(super) fn execute_instruction(
        &self,
        instr: &Instruction,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        trace!(target: "st::vm::execution", "EXECUTE: {}", instr);
        match instr {
            Instruction::LoadImm { dst, value } => {
                self.set(*dst, value.clone())?;
                Ok(())
            }
            Instruction::CreateString { dst, value } => {
                self.set(*dst, Value::String(SmalltalkString::new(value)?))?;
                Ok(())
            }
            Instruction::LoadReceiver { .. } => Err("LoadReceiver is not implemented".into()),
            Instruction::LoadGlobal { dst, var_name } => {
                self.set(*dst, self.get_global(var_name)?)?;
                Ok(())
            }
            Instruction::StoreLocal { .. } => Err("StoreLocal is not implemented".into()),
            Instruction::LoadInstanceVar { dst, index } => {
                if let Some(rec) = self.get(Register::Receiver)? {
                    match rec {
                        Value::Object(smalltalk_object) => {
                            let value = smalltalk_object.get_instance_var(*index)?;
                            self.set(*dst, value)?;
                            Ok(())
                        }
                        Value::Class(smalltalk_class) => {
                            let value = smalltalk_class.meta()?.get_instance_var(*index)?;
                            self.set(*dst, value)?;
                            Ok(())
                        }
                        _ => Err("LOAD_IVAR: Receiver has no instance variables".into()),
                    }
                } else {
                    Err("LOAD_IVAR: Receiver is not defined".into())
                }
            }
            Instruction::StoreInstanceVar { src, index } => {
                if let Some(value) = self.get(*src)? {
                    if let Some(Value::Object(smalltalk_object)) = self.get(Register::Receiver)? {
                        smalltalk_object.set_instance_var(*index, value)?;
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
                if let Some(value) = self.get(*src)? {
                    self.set(*dst, value)?;
                    Ok(())
                } else {
                    Err(RuntimeError::RegisterNotAssigned(*src).into())
                }
            }
            Instruction::CreateArray { dst, elements } => {
                let mut values = Vec::new();
                values
                    .try_reserve_exact(elements.len())
                    .map_err(|error| format!("Cannot allocate array: {error}"))?;
                for reg in elements {
                    values.push(
                        self.get(*reg)?
                            .ok_or(RuntimeError::RegisterNotAssigned(*reg))?,
                    );
                }
                self.set(*dst, Value::Array(Arc::new(Mutex::new(values))))?;
                Ok(())
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => self.execute_send_message(*dst, *receiver, args, selector),
            Instruction::Return { src } => {
                if let Some(value) = self.get(*src)? {
                    if self.is_block_activation()? {
                        let target = self.non_local_return_target()?;
                        Err(Box::new(NonLocalReturn {
                            target,
                            value: value.clone(),
                        }))
                    } else {
                        self.set(Register::Result, value.clone())?;
                        self.halt()?;
                        trace!(target: "st::vm::execution", "RETURN: {}", value);
                        Ok(())
                    }
                } else {
                    Err(RuntimeError::RegisterNotAssigned(*src).into())
                }
            }
            Instruction::CreateBlock { dst, prog_id } => {
                self.execute_create_block(*dst, *prog_id)?;
                Ok(())
            }
            Instruction::CallBlock { .. } => Err("CallBlock is not implemented".into()),
            Instruction::CallBlockWithArgs { .. } => {
                Err("CallBlockWithArgs is not implemented".into())
            }
            Instruction::Nop => Ok(()),
            Instruction::Unsupported { message } => Err((*message).into()),
            x => Err(format!("Unsupported instruction: {x}").into()),
        }
    }

    fn execute_create_block(
        &self,
        dst: Register,
        block_num: usize,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        {
            let exec = self.lock()?;
            if exec.code.blocks().get(block_num).is_none() {
                return Err(RuntimeError::BlockNotFound(block_num).into());
            }
        }
        let block = self.create_block(self.level()? + 1, block_num)?;
        self.set(dst, Value::Execution(block))?;
        Ok(())
    }

    fn get_global(&self, var_name: &'static str) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.vm()?.get_global(var_name)
    }
}
