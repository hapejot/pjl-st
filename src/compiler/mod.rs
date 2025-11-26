use pjl_static_strings::StringTable;
use std::io::Write;
use std::{
    fs::OpenOptions,
    sync::{Arc, Mutex},
    vec,
};
use tracing::{error, instrument, trace};

use crate::{
    memory::Value,
    parser::topdown::SmalltalkNode,
    vm::{
        block::CompiledBlock, block::CompiledMethod, instruction::Instruction, register::Register,
    },
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
        let index = self.allocation.len();
        if self.allocation.contains(&var_name) {
            Err(format!("Variable {} already allocated", var_name))
        } else {
            self.allocation.push(var_name);
            Ok(index)
        }
    }

    pub fn get(&self, var_name: &'static str) -> Option<Register> {
        match self.parent {
            Some(ref parent) => match self.allocation.iter().position(|&x| x == var_name) {
                Some(index) => Some(Register::Number(0, index)),
                None => {
                    if let Some(Register::Number(level, pos)) = parent.get(var_name) {
                        Some(Register::Number(level + 1, pos))
                    } else {
                        None
                    }
                }
            },
            None => self
                .allocation
                .iter()
                .position(|&x| x == var_name)
                .map(|index| Register::Number(0, index)),
        }
    }

    pub fn create_child(&self) -> Result<VariableAllocation, String> {
        let child = VariableAllocation {
            parent: Some(Box::new(self.clone())),
            allocation: vec![],
        };
        Ok(child)
    }

    pub fn dump_to_trace_level(&self, level: usize) {
        match self.parent.as_ref() {
            Some(parent) => {
                parent.dump_to_trace_level(level + 1);
            }
            None => {}
        }
        for (v, k) in self.allocation.iter().enumerate() {
            trace!("  var {} -> r{}.{}", k, level, v);
        }
    }
    pub fn dump_to_trace(&self) {
        self.dump_to_trace_level(0);
    }
}
#[derive(Debug, Clone)]
struct SmalltalkCompiler {
    code: Vec<Instruction>,
    var_allocation: VariableAllocation,
    blocks: Vec<CompiledBlock>,
    instance_vars: Vec<&'static str>,
    block_offset: usize,
    block_depth: usize,
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
            block_offset: 0,
            block_depth: 0,
        };
        s
    }

    fn allocate_register(&mut self) -> Register {
        let n = self.var_allocation.allocation.len();
        let q = self
            .var_allocation
            .add(StringTable::get(format!("r{}", n).as_str()))
            .unwrap();
        Register::Number(0, q)
    }

    fn allocate_var(&mut self, var_name: &'static str) -> Register {
        let q = self.var_allocation.add(var_name).unwrap();
        Register::Number(0, q)
    }

    fn free_register(&mut self, _reg: Register) {}

    #[instrument(skip(self, node))]
    fn compile_node(
        &mut self,
        dst: Option<Register>,
        node: &SmalltalkNode,
    ) -> Result<Register, String> {
        trace!(node = ?node, "Compiling node");
        match node {
            SmalltalkNode::Block {
                parameters,
                temps,
                body,
            } => self.compile_block(parameters, temps, dst, body),
            SmalltalkNode::Return(expression) => {
                let dst = self.compile_node(Some(Register::Result), expression)?;
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
                let mut last_reg: Register = Register::Number(0, 0);
                for stmt in ls.iter() {
                    last_reg = self.compile_node(dst, stmt)?;
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
                Ok(dst.unwrap_or(Register::Number(0, 0)))
            }
            SmalltalkNode::Nil => {
                // don't do anything, this happens when an empty block is defined.
                Ok(dst.unwrap_or(Register::Number(0, 0)))
            }
            x => {
                todo!("{:?}", x);
            }
        }
    }

    fn compile_block(
        &mut self,
        parameters: &Vec<&'static str>,
        temps: &Vec<&'static str>,
        dst: Option<Register>,
        body: &SmalltalkNode,
    ) -> Result<Register, String> {
        let dst = match dst {
            Some(d) => d,
            None => self.allocate_register(),
        };
        let mut block = SmalltalkCompiler::new();
        block.set_offset(1);
        block.set_depth(self.block_depth + 1);
        block.instance_vars = self.instance_vars.clone();
        block.var_allocation = self.var_allocation.create_child()?;

        let block_dst = block.allocate_register();
        for x in parameters {
            let _n = block.allocate_var(x);
        }
        for temp_name in temps {
            let _n = block.allocate_var(temp_name);
        }

        let r = block.compile_node(Some(block_dst), body)?;
        if r != block_dst {
            block.add_instruction(Instruction::Move {
                dst: Register::Result,
                src: r,
            });
        }

        for b in block.blocks.iter() {
            self.blocks.push(b.clone());
        }

        self.blocks.push(CompiledBlock {
            instructions: block.code,
            parameter_count: parameters.len(),
            depth: block.block_depth,
        });
        self.add_instruction(Instruction::CreateBlock {
            dst,
            prog_id: self.blocks.len() - 1,
        });

        Ok(dst)
    }

    fn add_instruction(&mut self, inst: Instruction) {
        self.code.push(inst);
    }

    #[allow(dead_code)]
    pub fn dump_to_trace(&self) {
        trace!("Compiled Method:");

        self.var_allocation.dump_to_trace();
        for (i, x) in self.code.iter().enumerate() {
            trace!("  {:04}: {:}", i, x);
        }

        for (i, b) in self.blocks.iter().enumerate() {
            trace!("Block #{}:", i);
            b.dump_to_trace();
        }
    }

    fn set_offset(&mut self, offset: usize) {
        self.block_offset = offset;
    }

    fn set_depth(&mut self, depth: usize) {
        self.block_depth = depth;
    }
}

pub fn compile_statements(node: &SmalltalkNode) -> Result<CompiledMethod, String> {
    let mut c = SmalltalkCompiler::new();
    let r = c.compile_node(None, node)?;
    if r != Register::Number(0, 0) {
        c.add_instruction(Instruction::Move {
            dst: Register::Number(0, 0),
            src: r,
        });
    }
    let parameter_count = 0;
    let m = CompiledMethod::new(c.code, c.blocks, parameter_count);
    return Ok(m);
}

fn dummy(
    _vm: &crate::vm::execution::Execution,
    _receiver: Value,
    _args: Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(Value::Undefined)
}

#[allow(dead_code)]
fn sample_method(
    _vm: &crate::vm::execution::Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if args.len() != 1 {
        return Err("Integer addition requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        _ => Err("Integer addition requires integer arguments".into()),
    }
}



#[instrument(skip(node))]
pub fn compile_method(
    instance_vars: Vec<&'static str>,
    par: Vec<&'static str>,
    node: &SmalltalkNode,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut c = SmalltalkCompiler::new();
    c.instance_vars = instance_vars;
    for p in par {
        c.var_allocation.add(p)?;
    }

    // c.add_instruction(Instruction::Move {
    //     dst: Register::Result,
    //     src: Register::Receiver,
    // });

    if let SmalltalkNode::Primitive(p, _) = node {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("primitive.log")?;
        writeln!(f, "{p}")?;
        return Ok(Value::NativeMethod(dummy));
    } else {
        let _r = c.compile_node(None, node)?;

        let m = CompiledMethod::new(c.code, c.blocks, c.var_allocation.allocation.len());
        return Ok(Value::Method(m));
    }
}
