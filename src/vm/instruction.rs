use crate::memory::Value;
use crate::vm::register::Register;
use std::fmt::Display;
/// Three-address code instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    // Basic operations
    /// Load immediate value into register: LOAD_IMM dst, value
    LoadImm { dst: Register, value: Value },

    /// load the receiver into the register: LOAD_RECEIVER dst
    LoadReceiver { dst: Register },

    /// Load variable into register: LOAD_VAR dst, var_name
    LoadGlobal {
        dst: Register,
        var_name: &'static str,
    },

    /// Store register value to variable: STORE_VAR src, var_name
    StoreLocal {
        src: Register,
        var_name: &'static str,
    },

    /// Load instance variable by index: LOAD_IVAR dst, index
    LoadInstanceVar { dst: Register, index: usize },

    /// Store to instance variable by index: STORE_IVAR src, index
    StoreInstanceVar { src: Register, index: usize },

    /// Copy register to register: MOVE dst, src
    Move { dst: Register, src: Register },
    /// Move  non local registers
    MoveToLocal {
        dst: usize,
        src_level: usize,
        src_index: usize,
    },
    // Method call
    CallMethod {
        dst: Register,
        receiver: Register,
        args: Vec<Register>,
        selector: &'static str,
    },

    // Special operations
    /// Return value: RETURN src
    Return { src: Register },

    /// Create block closure: CREATE_BLOCK dst, block_id, captured_vars
    CreateBlock {
        dst: Register,
        prog_id: usize,
        // captured_vars: Vec<usize>,
    },

    /// Call block with value: CALL_BLOCK dst, block_reg
    CallBlock { dst: usize, block_reg: usize },

    /// Call block with arguments: CALL_BLOCK_ARGS dst, block_reg, args
    CallBlockWithArgs {
        dst: usize,
        block_reg: usize,
        args: Vec<usize>,
    },

    /// No operation: NOP
    Nop,

    /// Halt execution: HALT
    End,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::LoadImm { dst, value } => {
                write!(f, "{} <- '{:?}'", dst, value)
            }
            Instruction::Move { dst, src } => {
                write!(f, "{} <- {}", dst, src)
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => {
                write!(
                    f,
                    "{} <- {} {} {}",
                    dst,
                    selector,
                    receiver,
                    args.iter()
                        .map(|a| format!("{}", a))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Instruction::StoreInstanceVar { src, index } => {
                write!(f, "ivar[{}] <- {}", index, src)
            }
            Instruction::LoadGlobal { dst, var_name } => write!(f, "{} <- {}", dst, var_name),
            Instruction::CreateBlock { dst, prog_id } => {
                write!(f, "{} <- Block #{}", dst, prog_id)
            }
            Instruction::Return { src } => {
                write!(f, "Return {}", src)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
