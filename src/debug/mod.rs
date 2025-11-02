use crate::vm::Execution;

pub mod terminal;

pub trait Debugger: Send + Sync {
    fn before_execute(&self, execution: Execution);
    fn after_execute(&self, execution: Execution);
}