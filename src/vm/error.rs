use crate::vm::register::Register;

#[derive(Debug)]
pub enum RuntimeError {
    GlobalNotFound(&'static str),
    RegisterNotAssigned(Register),
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
        }
    }
}
