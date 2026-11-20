use crate::{
    memory::Value,
    vm::{error::RuntimeError, register::Register},
};

use super::Execution;

impl Execution {
    pub fn reg_count(&self) -> Result<usize, RuntimeError> {
        let (count, home) = {
            let exec = self.lock()?;
            (exec.registers.len(), exec.home.clone())
        };
        Ok(count
            + match home {
                Some(home) => home.reg_count()?,
                None => 0,
            })
    }

    pub fn set(&self, reg: Register, value: Value) -> Result<(), RuntimeError> {
        match reg {
            Register::Number(0, index) => self.internal_set_register(index, value),
            Register::Number(level, index) if level > 0 => {
                if let Some(home) = self.home()? {
                    home.set(Register::Number(level - 1, index), value)
                } else {
                    Err(RuntimeError::InvalidRegister(reg))
                }
            }
            Register::Result => {
                if let Some(home) = self.home()? {
                    home.set(Register::Result, value)
                } else {
                    self.internal_set_register(0, value)
                }
            }
            _ => Err(RuntimeError::InvalidRegister(reg)),
        }
    }

    fn internal_set_register(&self, reg: usize, value: Value) -> Result<(), RuntimeError> {
        let mut exec = self.lock()?;
        let registers = &mut exec.registers;
        if reg >= registers.len() {
            let register = Register::Number(0, reg);
            let len = reg
                .checked_add(1)
                .ok_or(RuntimeError::RegisterAllocation(register))?;
            registers
                .try_reserve(len - registers.len())
                .map_err(|_| RuntimeError::RegisterAllocation(register))?;
            registers.resize(len, Value::Undefined);
        }
        registers[reg] = value;
        Ok(())
    }

    fn internal_get_register(&self, reg: usize) -> Result<Option<Value>, RuntimeError> {
        let exec = self.lock()?;
        Ok(Some(
            exec.registers.get(reg).cloned().unwrap_or(Value::Undefined),
        ))
    }

    pub fn get(&self, reg: Register) -> Result<Option<Value>, RuntimeError> {
        match reg {
            Register::Result => {
                if let Some(home) = self.home()? {
                    home.get(reg)
                } else {
                    self.internal_get_register(0)
                }
            }
            Register::Receiver => {
                if let Some(home) = self.home()? {
                    home.get(reg)
                } else {
                    self.internal_get_register(1)
                }
            }
            Register::Number(0, index) => self.internal_get_register(index),
            Register::Number(level, index) if level > 0 => {
                if let Some(home) = self.home()? {
                    home.get(Register::Number(level - 1, index))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn registers(&self) -> Result<Vec<String>, RuntimeError> {
        let registers = self.lock()?.registers.clone();
        Ok(registers.iter().map(|v| format!("{}", v)).collect())
    }
}
