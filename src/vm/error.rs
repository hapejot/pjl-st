use std::{error::Error, fmt};

use crate::vm::register::Register;

#[derive(Debug)]
pub(crate) struct ExecutionError {
    cause: Box<dyn Error + Send + Sync>,
    frames: Vec<String>,
}

impl ExecutionError {
    pub(crate) fn at(
        cause: Box<dyn Error + Send + Sync>,
        frame: String,
    ) -> Box<dyn Error + Send + Sync> {
        let mut error = match cause.downcast::<Self>() {
            Ok(error) => error,
            Err(cause) => Box::new(Self {
                cause,
                frames: Vec::new(),
            }),
        };
        error.frames.push(frame);
        error
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nSmalltalk stack (innermost first):", self.cause)?;
        for frame in &self.frames {
            write!(f, "\n  at {frame}")?;
        }
        Ok(())
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    GlobalNotFound(&'static str),
    RegisterNotAssigned(Register),
    InvalidRegister(Register),
    RegisterAllocation(Register),
    ExecutionLock(String),
    InstructionLimitExceeded,
    CallDepthLimitExceeded {
        max: usize,
    },
    CallDepthOverflow,
    BlockNotCallable,
    BlockNotFound(usize),
    BlockArgumentCount {
        expected: usize,
        actual: usize,
    },
    MethodArgumentCount {
        method: String,
        expected: usize,
        actual: usize,
    },
    BlockCannotReturn,
    BlockCannotReturnAcrossExecutions,
    VirtualMachineDropped,
}

impl std::error::Error for RuntimeError {}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::GlobalNotFound(var_name) => {
                write!(f, "Global name {} not defined", var_name)
            }
            RuntimeError::RegisterNotAssigned(reg) => {
                write!(f, "Register {} not assigned", reg)
            }
            RuntimeError::InvalidRegister(reg) => write!(f, "Invalid destination register {reg}"),
            RuntimeError::RegisterAllocation(reg) => {
                write!(f, "Cannot allocate destination register {reg}")
            }
            RuntimeError::ExecutionLock(error) => write!(f, "Execution lock: {error}"),
            RuntimeError::InstructionLimitExceeded => write!(f, "Instruction limit exceeded"),
            RuntimeError::CallDepthLimitExceeded { max } => {
                write!(f, "Call depth limit exceeded (maximum {max})")
            }
            RuntimeError::CallDepthOverflow => write!(f, "Call depth overflow"),
            RuntimeError::BlockNotCallable => write!(f, "Execution is not a block closure"),
            RuntimeError::BlockNotFound(index) => write!(f, "Block {} not found", index),
            RuntimeError::BlockArgumentCount { expected, actual } => write!(
                f,
                "Block expects {} argument(s), but received {}",
                expected, actual
            ),
            RuntimeError::MethodArgumentCount {
                method,
                expected,
                actual,
            } => write!(
                f,
                "Method {method} expects {expected} argument(s), but received {actual}"
            ),
            RuntimeError::BlockCannotReturn => {
                write!(
                    f,
                    "Block cannot return because its home method is no longer active"
                )
            }
            RuntimeError::BlockCannotReturnAcrossExecutions => {
                write!(
                    f,
                    "Block cannot return to a home method in a different execution"
                )
            }
            RuntimeError::VirtualMachineDropped => write!(f, "Virtual machine has been dropped"),
        }
    }
}
