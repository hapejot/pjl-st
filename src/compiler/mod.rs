use std::{collections::HashMap, vec};

use tracing::trace;

use crate::{parser::topdown::SmalltalkNode, vm::Instruction};

#[derive(Debug, Clone)]
struct SmalltalkCompiler {
    code: Vec<Instruction>,
    register: Vec<bool>,
    var_allocation: HashMap<&'static str, usize>,
    blocks: Vec<SmalltalkCompiler>,
}

impl SmalltalkCompiler {
    pub fn new() -> Self {
        let code = Vec::new();
        let var_allocation = HashMap::new();
        let mut s = Self {
            code,
            register: vec![],
            var_allocation,
            blocks: vec![],
        };
        s.allocate_register(); // allocate first slot for result
        s
    }

    fn allocate_register(&mut self) -> usize {
        match self.register.iter().position(|x| !*x) {
            Some(n) => {
                self.register[n] = true;
                n
            }
            None => {
                self.register.push(true);
                self.register.len() - 1
            }
        }
    }

    fn free_register(&mut self, reg: usize) {
        if reg < self.register.len() {
            self.register[reg] = false;
        }
    }

    fn compile_node(&mut self, dst: usize, node: &SmalltalkNode) -> Result<usize, String> {
        match node {
            SmalltalkNode::Block {
                parameters,
                temps,
                body,
            } => {
                let mut block = SmalltalkCompiler::new();
                let block_dst = 0; // allocate first slot for result
                for x in parameters {
                    let n = block.allocate_register();
                    block.var_allocation.insert(x, n);
                }
                for x in temps {
                    let n = block.allocate_register();
                    block.var_allocation.insert(x, n);
                }
                block.compile_node(block_dst, body)?;

                self.blocks.push(block);
                self.add_instruction(Instruction::CreateBlock {
                    dst,
                    prog_id: self.blocks.len() - 1,
                });

                Ok(dst)
            }
            SmalltalkNode::Return(expression) => self.compile_node(dst, expression),
            SmalltalkNode::Identifier(name) => match self.var_allocation.get(name) {
                Some(index) => {
                    self.add_instruction(Instruction::Move { dst, src: *index });
                    Ok(dst)
                }
                None => {
                    self.add_instruction(Instruction::LoadGlobal {
                        dst,
                        var_name: name,
                    });
                    Ok(dst)
                }
            },
            SmalltalkNode::Message {
                receiver,
                selector,
                arguments,
            } => {
                let r = self.compile_node(dst, receiver)?;
                let mut args = vec![];
                for arg in arguments {
                    let n = self.allocate_register();
                    args.push(self.compile_node(n, arg)?);
                }
                let inst = Instruction::CallMethod {
                    dst,
                    receiver: r,
                    args: args.clone(),
                    selector,
                };
                for a in args.iter() {
                    self.free_register(*a);
                }
                self.add_instruction(inst);
                Ok(dst)
            }
            SmalltalkNode::Value(value) => {
                self.add_instruction(Instruction::LoadImm {
                    dst,
                    value: value.clone(),
                });
                Ok(dst)
            }
            SmalltalkNode::Statements(loc, ls) => {
                for n in loc.iter() {
                    let r = self.allocate_register();
                    self.var_allocation.insert(n, r);
                }
                let mut last_reg = dst;
                for stmt in ls.iter() {
                    last_reg = self.compile_node(dst, stmt)?;
                }
                Ok(last_reg)
            }
            SmalltalkNode::Assignment { variable, value } => {
                let dst = self
                    .var_allocation
                    .get(variable)
                    .cloned().unwrap();
                let r = self.compile_node(dst, value)?;
                Ok(r)
            }
            x => {
                todo!("{:?}", x);
            }
        }
    }

    fn add_instruction(&mut self, inst: Instruction) {
        self.code.push(inst);
    }

    pub fn dump_to_trace(&self) {
        trace!("registers: {}", self.register.len());
        self.var_allocation.iter().for_each(|(k, v)| {
            trace!("  var {} -> r{}", k, v);
        });
        for (i, x) in self.code.iter().enumerate() {
            trace!("  {:04}: {:}", i, x);
        }

        for (i, b) in self.blocks.iter().enumerate() {
            trace!("Block #{}:", i);
            b.dump_to_trace();
        }
    }
}

pub fn compile(node: &SmalltalkNode) {
    let mut c = SmalltalkCompiler::new();
    if let Ok(r) = c.compile_node(0, node) {
        trace!("Compiled code [{r}]");
        c.dump_to_trace();
    }
}
