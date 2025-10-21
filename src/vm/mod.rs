use std::{fmt::Display, sync::Arc};

use crate::memory::Value;

/// Three-address code instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Basic operations
    /// Load immediate value into register: LOAD_IMM dst, value
    LoadImm { dst: usize, value: Value },

    /// Load variable into register: LOAD_VAR dst, var_name
    LoadGlobal { dst: usize, var_name: &'static str },

    /// Store register value to variable: STORE_VAR src, var_name
    StoreLocal { src: usize, var_name: &'static str },

    /// Load instance variable by index: LOAD_IVAR dst, index
    LoadInstanceVar { dst: usize, index: usize },

    /// Store to instance variable by index: STORE_IVAR src, index
    StoreInstanceVar { src: usize, index: usize },

    /// Copy register to register: MOVE dst, src
    Move { dst: usize, src: usize },

    // Method call
    CallMethod {
        dst: usize,
        receiver: usize,
        args: Vec<usize>,
        selector: &'static str,
    },

    // Stack operations
    /// Push register onto stack: PUSH src
    Push { src: usize },

    /// Pop stack into register: POP dst
    Pop { dst: usize },

    // Special operations
    /// Return value: RETURN src
    Return { src: usize },

    /// Create block closure: CREATE_BLOCK dst, block_id, captured_vars
    CreateBlock {
        dst: usize,
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
                write!(f, "LOAD_IMM r{}, {:?}", dst, value)
            }
            Instruction::Move { dst, src } => {
                write!(f, "MOVE r{}, r{}", dst, src)
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => {
                write!(
                    f,
                    "CALL_METHOD r{}, r{} -> {}({})",
                    dst,
                    receiver,
                    selector,
                    args.iter()
                        .map(|a| format!("r{}", a))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Bytecode program
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    /// Instructions in the program
    pub instructions: Vec<Instruction>,
    // Label to instruction index mapping
    // pub labels: HashMap<String, usize>,
    // Block sub-programs (indexed by block_id)
    pub blocks: Vec<CompiledBlock>,
    pub parameter_count: usize,
}

#[derive(Debug, Clone)]
pub struct CompiledBlock {
    pub instructions: Vec<Instruction>,
    pub parameter_count: usize,
}

#[derive(Debug, Clone)]
pub struct MethodExecution {
    pub code: CompiledMethod,
    pub ip: usize,
    pub registers: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct BlockExecution {
    pub home: Arc<MethodExecution>,
    pub code: CompiledBlock,
    pub ip: usize,
    pub registers: Vec<Value>,
}
