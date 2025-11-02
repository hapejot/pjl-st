use std::{
    sync::{Arc, Mutex},
    vec,
};

use pjl_static_strings::StringTable;
use tracing::{error, instrument, trace};

use crate::{
    memory::Value,
    parser::topdown::SmalltalkNode,
    vm::{CompiledBlock, CompiledMethod, Instruction},
};

#[derive(Debug, Clone)]
pub struct VariableAllocation {
    parent: Option<Box<VariableAllocation>>,
    allocation: Vec<&'static str>,
}

impl VariableAllocation {
    pub fn new() -> Self {
        Self {
            parent: None,
            allocation: vec![],
        }
    }

    pub fn add(&mut self, var_name: &'static str) -> Result<usize, String> {
        match self.parent {
            Some(ref parent) => {
                if self.allocation.contains(&var_name) {
                    Err(format!("Variable {} already allocated", var_name))
                } else {
                    let index = self.allocation.len() + parent.allocation.len();
                    self.allocation.push(var_name);
                    Ok(index)
                }
            }
            None => {
                if self.allocation.contains(&var_name) {
                    Err(format!("Variable {} already allocated", var_name))
                } else {
                    let index = self.allocation.len();
                    self.allocation.push(var_name);
                    Ok(index)
                }
            }
        }
    }

    pub fn get(&self, var_name: &'static str) -> Option<usize> {
        match self.parent {
            Some(ref parent) => match self.allocation.iter().position(|&x| x == var_name) {
                Some(index) => Some(index + parent.allocation.len()),
                None => parent.get(var_name),
            },
            None => self.allocation.iter().position(|&x| x == var_name),
        }
    }

    pub fn create_child(&self) -> Result<VariableAllocation, String> {
        let child = VariableAllocation {
            parent: Some(Box::new(self.clone())),
            allocation: vec![],
        };
        Ok(child)
    }

    fn dump_to_trace(&self) {
        match self.parent.as_ref() {
            Some(parent) => {
                parent.dump_to_trace();
                for (v, k) in self.allocation.iter().enumerate() {
                    trace!("  var {} -> r{}", k, v + parent.allocation.len());
                }
            }
            None => {
                for (v, k) in self.allocation.iter().enumerate() {
                    trace!("  var {} -> r{}", k, v);
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
struct SmalltalkCompiler {
    code: Vec<Instruction>,
    var_allocation: VariableAllocation,
    blocks: Vec<SmalltalkCompiler>,
    instance_vars: Vec<&'static str>,
}

impl SmalltalkCompiler {
    pub fn new() -> Self {
        let code = Vec::new();
        let mut var_allocation = VariableAllocation::new();
        var_allocation.add("@result").unwrap();
        var_allocation.add("self").unwrap();
        let s = Self {
            code,
            var_allocation,
            blocks: vec![],
            instance_vars: vec![],
        };
        s
    }

    fn allocate_register(&mut self) -> usize {
        let n = self.var_allocation.allocation.len();
        self.var_allocation
            .add(StringTable::get(format!("r{}", n).as_str()))
            .unwrap()
    }

    fn free_register(&mut self, _reg: usize) {}

    #[instrument(skip(self, node))]
    fn compile_node(&mut self, dst: Option<usize>, node: &SmalltalkNode) -> Result<usize, String> {
        trace!(node = ?node, "Compiling node");
        match node {
            SmalltalkNode::Block {
                parameters,
                temps,
                body,
            } => {
                let dst = match dst {
                    Some(d) => d,
                    None => self.allocate_register(),
                };
                let mut block = SmalltalkCompiler::new();
                block.instance_vars = self.instance_vars.clone();
                block.var_allocation = self.var_allocation.create_child()?;
                let block_dst = 0; // allocate first slot for result
                for x in parameters {
                    let _n = block.var_allocation.add(x).unwrap();
                }
                for _x in temps {
                    let _n = block.allocate_register();
                }
                block.compile_node(Some(block_dst), body)?;

                self.blocks.push(block);
                self.add_instruction(Instruction::CreateBlock {
                    dst,
                    prog_id: self.blocks.len() - 1,
                });

                Ok(dst)
            }
            SmalltalkNode::Return(expression) => {
                let dst = self.compile_node(Some(0), expression)?;
                self.add_instruction(Instruction::Return { src: dst });
                Ok(dst)
            }
            SmalltalkNode::Identifier(name) => match self.var_allocation.get(name) {
                Some(index) => match dst {
                    Some(dst) => {
                        self.add_instruction(Instruction::Move { dst, src: index });
                        Ok(dst)
                    }
                    None => Ok(index),
                },
                None => match self.instance_vars.iter().position(|x| x == name) {
                    Some(index) => {
                        let dst = match dst {
                            Some(d) => d,
                            None => self.allocate_register(),
                        };
                        self.add_instruction(Instruction::LoadInstanceVar { dst, index });
                        return Ok(dst);
                    }
                    None => {
                        let dst = match dst {
                            Some(d) => d,
                            None => self.allocate_register(),
                        };
                        self.add_instruction(Instruction::LoadGlobal {
                            dst,
                            var_name: name,
                        });
                        Ok(dst)
                    }
                },
            },
            SmalltalkNode::MessageInvoke { receiver, messages } => {
                let dst = match dst {
                    Some(d) => d,
                    None => self.allocate_register(),
                };
                let r = self.compile_node(None, receiver)?;
                for x in messages {
                    if let SmalltalkNode::Message {
                        selector,
                        arguments,
                    } = x
                    {
                        let mut args = vec![];
                        for arg in arguments {
                            args.push(self.compile_node(None, arg)?);
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
                    }
                }
                Ok(dst)
            }
            SmalltalkNode::Symbol(s) => {
                let dst = match dst {
                    Some(d) => d,
                    None => self.allocate_register(),
                };
                self.add_instruction(Instruction::LoadImm {
                    dst,
                    value: Value::String(*s),
                });
                Ok(dst)
            }
            SmalltalkNode::Value(value) => {
                let dst = match dst {
                    Some(d) => d,
                    None => self.allocate_register(),
                };
                self.add_instruction(Instruction::LoadImm {
                    dst,
                    value: value.clone(),
                });
                Ok(dst)
            }
            SmalltalkNode::Array(a) => {
                let dst = match dst {
                    Some(d) => d,
                    None => self.allocate_register(),
                };
                let elements = a
                    .iter()
                    .map(|x| {
                        if let SmalltalkNode::Value(v) = x {
                            v.clone()
                        } else {
                            Value::Undefined
                        }
                    })
                    .collect::<Vec<_>>();
                self.add_instruction(Instruction::LoadImm {
                    dst,
                    value: Value::Array(Arc::new(Mutex::new(elements))),
                });
                Ok(dst)
            }
            SmalltalkNode::Statements(loc, ls) => {
                for n in loc.iter() {
                    let _r = self.var_allocation.add(n).unwrap();
                }
                let mut last_reg = 0;
                for stmt in ls.iter() {
                    last_reg = self.compile_node(None, stmt)?;
                }
                Ok(last_reg)
            }
            SmalltalkNode::Assignment { variable, value } => {
                if let Some(dst) = self.var_allocation.get(variable) {
                    let r = self.compile_node(Some(dst), value)?;
                    Ok(r)
                } else {
                    if let Some(index) = self.instance_vars.iter().position(|x| x == variable) {
                        let r = self.compile_node(None, value)?;
                        self.add_instruction(Instruction::StoreInstanceVar { src: r, index });
                        Ok(r)
                    } else {
                        // if let Some(index) =
                        error!("assignment to {} from {:?} failed", variable, value);
                        error!("var allocation {:?}", self.var_allocation);
                        error!("instance vars {:?}", self.instance_vars);
                        return Err(format!("Variable {} not found", variable));
                    }
                }
            }
            SmalltalkNode::CascadeReceiver => {
                self.add_instruction(Instruction::Nop);
                Ok(dst.unwrap_or(0))
            }
            SmalltalkNode::Nil => {
                // don't do anything, this happens when an empty block is defined.
                Ok(dst.unwrap_or(0))
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
        self.var_allocation.dump_to_trace();
        for (i, x) in self.code.iter().enumerate() {
            trace!("  {:04}: {:}", i, x);
        }

        for (i, b) in self.blocks.iter().enumerate() {
            trace!("Block #{}:", i);
            b.dump_to_trace();
        }
    }
}

#[instrument(skip(node))]
pub fn compile_method(
    instance_vars: Vec<&'static str>,
    par: Vec<&'static str>,
    node: &SmalltalkNode,
) -> Result<CompiledMethod, String> {
    let mut c = SmalltalkCompiler::new();
    c.instance_vars = instance_vars;
    for p in par {
        c.var_allocation.add(p)?;
    }

    let r = c.compile_node(Some(0), node)?;
    trace!("Compiled code [{r}]");
    c.dump_to_trace();
    let m = CompiledMethod::new(
        c.code,
        c.blocks
            .into_iter()
            .map(|b| CompiledBlock {
                instructions: b.code,
                parameter_count: b.var_allocation.allocation.len(),
            })
            .collect(),
        c.var_allocation.allocation.len(),
    );
    return Ok(m);
}
