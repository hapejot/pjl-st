use std::sync::Arc;

use tracing::trace;

use crate::vm::instruction::Instruction;

#[derive(Debug, Clone, Default)]
pub struct CompiledBlock {
    pub instructions: Vec<Instruction>,
    pub parameter_count: usize,
    pub depth: usize,
}
impl CompiledBlock {
    pub(crate) fn dump_to_trace(&self) {
        self.instructions.iter().enumerate().for_each(|(i, inst)| {
            trace!("  {:04}: {}", i, inst);
        });
    }
}

/// Bytecode program
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    data: Arc<CompiledMethodData>,
}

#[derive(Debug, Clone)]
pub struct CompiledMethodData {
    /// Instructions in the program
    pub instructions: Vec<Instruction>,
    // Label to instruction index mapping
    // pub labels: HashMap<String, usize>,
    // Block sub-programs (indexed by block_id)
    pub blocks: Vec<CompiledBlock>,
    pub parameter_count: usize,
    pub depth: usize,
}

impl CompiledMethod {
    pub fn new(
        instructions: Vec<Instruction>,
        blocks: Vec<CompiledBlock>,
        parameter_count: usize,
    ) -> Self {
        let depth = 0;
        Self {
            data: Arc::new(CompiledMethodData {
                instructions,
                blocks,
                parameter_count,
                depth,
            }),
        }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.data.instructions
    }

    pub fn blocks(&self) -> &Vec<CompiledBlock> {
        &self.data.blocks
    }

    pub fn parameter_count(&self) -> usize {
        self.data.parameter_count
    }

    pub fn dump_to_trace(&self) {
        trace!("Compiled Method:");
        self.data.blocks.iter().enumerate().for_each(|(i, block)| {
            trace!("Block #{} depth: {} parameters: {}", i, block.depth, block.parameter_count);
            block.instructions.iter().enumerate().for_each(|(j, inst)| {
                trace!("  {:04}: {}", j, inst);
            });
        });
        self.data
            .instructions
            .iter()
            .enumerate()
            .for_each(|(i, inst)| {
                trace!("{:04}: {}", i, inst);
            });
    }
}
