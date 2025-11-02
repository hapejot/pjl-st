use crate::debug::Debugger;
use std::{
    collections::HashMap,
    default,
    error::Error,
    fmt::Display,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    vec,
};

use tracing::{instrument, trace};

use crate::{
    compiler::compile_method,
    get_smalltalk_file_path,
    memory::{SmalltalkClass, SmalltalkObject, Value},
    parser::topdown::{SmalltalkNode, SmalltalkParser, parse_eval},
};

#[derive(Debug)]
enum RuntimeError {
    GlobalNotFound(&'static str),
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::GlobalNotFound(var_name) => {
                write!(f, "Global name {} not defined", var_name)
            }
        }
    }
}

/// Three-address code instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    // Basic operations
    /// Load immediate value into register: LOAD_IMM dst, value
    LoadImm { dst: usize, value: Value },

    /// load the receiver into the register: LOAD_RECEIVER dst
    LoadReceiver { dst: usize },

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
                write!(f, "r{} <- '{:?}'", dst, value)
            }
            Instruction::Move { dst, src } => {
                write!(f, "r{} <- r{}", dst, src)
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => {
                write!(
                    f,
                    "r{} <- {} r{} {}",
                    dst,
                    selector,
                    receiver,
                    args.iter()
                        .map(|a| format!("r{}", a))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Instruction::StoreInstanceVar { src, index } => {
                write!(f, "ivar[{}] <- r{}", index, src)
            }
            Instruction::LoadGlobal { dst, var_name } => write!(f, "r{} <- {}", dst, var_name),
            _ => write!(f, "{:?}", self),
        }
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
}

impl CompiledMethod {
    pub fn new(
        instructions: Vec<Instruction>,
        blocks: Vec<CompiledBlock>,
        parameter_count: usize,
    ) -> Self {
        Self {
            data: Arc::new(CompiledMethodData {
                instructions,
                blocks,
                parameter_count,
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
}

#[derive(Debug, Clone, Default)]
pub struct CompiledBlock {
    pub instructions: Vec<Instruction>,
    pub parameter_count: usize,
}

#[derive(Debug, Clone)]
pub struct Execution(Arc<Mutex<ExecutionData>>);

#[derive(Debug)]
struct ExecutionData {
    pub vm: &'static VirtualMachine,
    pub home: Option<Execution>,
    pub code: CompiledMethod,
    pub ip: usize,
    pub block: Option<usize>,
    pub registers: Vec<Value>,
}

#[derive(Clone)]
pub struct VirtualMachine {
    integer_class: Value,
    float_class: Value,
    meta_class: Value,
    execution_class: Value,
    st: Value,
    debugger: Option<&'static dyn Debugger>,
}

impl std::fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualMachine")
            .field("integer_class", &self.integer_class)
            .field("float_class", &self.float_class)
            .field("meta_class", &self.meta_class)
            .field("execution_class", &self.execution_class)
            .field("st", &self.st)
            .field("debugger", &"<debugger>")
            .finish()
    }
}

fn integer_add(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer addition requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        _ => Err("Integer addition requires integer arguments".into()),
    }
}

fn execution_value1(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    match receiver {
        Value::Execution(a) => {
            a.set(2, args[0].clone());
            a.execute()
        }
        _ => Err("Execution value1 requires execution arguments".into()),
    }
}

fn execution_value2(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    match receiver {
        Value::Execution(a) => {
            a.set(2, args[0].clone());
            a.set(3, args[1].clone());
            a.execute()
        }
        _ => Err("Execution value1 requires execution arguments".into()),
    }
}


fn meta_class_new(
    _vm: &Execution,
    receiver: Value,
    _args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    match receiver {
        Value::Class(cls) => {
            let obj = SmalltalkObject::new(cls.clone());
            Ok(Value::Object(obj))
        }
        _ => Err("MetaClass new called on non-class value".into()),
    }
}

impl Execution {
    pub fn new(
        vm: &'static VirtualMachine,
        receiver: Value,
        args: Vec<Value>,
        code: CompiledMethod,
    ) -> Self {
        let mut registers = vec![receiver.clone(), receiver.clone()];
        registers.extend(args);
        Self(Arc::new(Mutex::new(ExecutionData {
            vm,
            home: None,
            code,
            ip: 0,
            block: None,
            registers,
        })))
    }

    pub fn reg_count(&self) -> usize {
        match self.home().clone() {
            Some(home) => {
                let exec = self.0.try_lock().unwrap();
                let registers = &exec.registers;
                let n = home.reg_count();
                n + registers.len()
            }
            None => {
                let exec = self.0.try_lock().unwrap();
                let registers = &exec.registers;
                registers.len()
            }
        }
    }

    pub fn set(&self, reg: usize, value: Value) {
        let home = self.home();
        match home {
            Some(home) => {
                let n = home.reg_count();
                if reg < n {
                    // home.set(reg, value);
                    return;
                } else {
                    self.internal_set_register(reg - n, value);
                    return;
                }
            }
            None => {
                self.internal_set_register(reg, value);
            }
        }
    }

    fn home(&self) -> Option<Execution> {
        let exec = self.0.try_lock().unwrap();
        exec.home.clone()
    }

    fn internal_set_register(&self, reg: usize, value: Value) {
        let mut exec = self.0.try_lock().unwrap();
        let registers = &mut exec.registers;
        if reg >= registers.len() {
            registers.resize(reg + 1, Value::Undefined);
        }
        registers[reg] = value;
    }

    pub fn get(&self, reg: usize) -> Value {
        match self.home().clone() {
            Some(home) => {
                let mut exec = self.0.try_lock().unwrap();
                let registers = &mut exec.registers;
                let n = home.reg_count();
                if reg < n {
                    home.get(reg)
                } else {
                    registers[reg - n].clone()
                }
            }
            None => {
                let mut exec = self.0.try_lock().unwrap();
                let registers = &mut exec.registers;
                let r = if reg >= registers.len() {
                    return Value::Undefined;
                } else {
                    return registers[reg].clone();
                };
            }
        }
    }

    #[instrument(skip(self))]
    pub fn execute(&self) -> Result<Value, Box<dyn Error>> {
        let (instructions, debugger) = {
            let exec = self.0.try_lock().unwrap();
            let inst = match exec.block {
                Some(n) => exec.code.blocks()[n].instructions.clone(),
                None => exec.code.instructions().clone(),
            };
            let deb = exec.vm.debugger.clone();
            (inst, deb)
        };

        while self.ip() < instructions.len() {
            // Call debugger before executing instruction
            if let Some(debugger) = debugger {
                debugger.before_execute(self.clone());
            }

            let instr = &instructions[self.ip()];
            self.execute_instruction(instr)?;

            // Call debugger after executing instruction
            if let Some(debugger) = debugger {
                debugger.after_execute(self.clone());
            }
            self.next_ip();
        }

        self.dump_registers();
        Ok(self.get(0))
    }

    fn dump_registers(&self) {
        // let registers = self.registers.try_lock().unwrap();
        // for (idx, reg) in registers.iter().enumerate() {
        //     trace!("r{}: {}", idx, reg);
        // }
    }

    fn execute_instruction(&self, instr: &Instruction) -> Result<(), Box<dyn Error>> {
        trace!("EXECUTE: {}", instr);
        match instr {
            Instruction::LoadImm { dst, value } => {
                self.set(*dst, value.clone());
                Ok(())
            }
            Instruction::LoadReceiver { dst } => {
                todo!()
            }
            Instruction::LoadGlobal { dst, var_name } => {
                self.set(*dst, self.get_global(var_name)?);
                Ok(())
            }
            Instruction::StoreLocal { src, var_name } => todo!(),
            Instruction::LoadInstanceVar { dst, index } => {
                if let Value::Object(smalltalk_object) = self.get(1) {
                    let value = smalltalk_object.get_instance_var(*index);
                    self.set(*dst, value);
                    Ok(())
                } else {
                    self.dump_registers();
                    Err("LOAD_IVAR: Receiver is not an object".into())
                }
            }
            Instruction::StoreInstanceVar { src, index } => {
                let value = self.get(*src);
                if let Value::Object(smalltalk_object) = self.get(1) {
                    smalltalk_object.set_instance_var(*index, value);
                    Ok(())
                } else {
                    self.dump_registers();
                    Err("STORE_IVAR: Receiver is not an object".into())
                }
            }
            Instruction::Move { dst, src } => {
                let value = self.get(*src);
                self.set(*dst, value);
                Ok(())
            }
            Instruction::CallMethod {
                dst,
                receiver,
                args,
                selector,
            } => self.execute_send_message(*dst, *receiver, args, selector),
            Instruction::Push { src } => todo!(),
            Instruction::Pop { dst } => todo!(),
            Instruction::Return { src } => {
                let value = self.get(*src);
                self.set(0, value.clone());
                trace!("RETURN: {}", value);
                Ok(())
            }
            Instruction::CreateBlock { dst, prog_id } => {
                self.execute_create_block(*dst, *prog_id)?;
                Ok(())
            }
            Instruction::CallBlock { dst, block_reg } => todo!(),
            Instruction::CallBlockWithArgs {
                dst,
                block_reg,
                args,
            } => todo!(),
            Instruction::Nop => Ok(()),
            Instruction::End => todo!(),
        }
    }

    fn execute_create_block(&self, dst: usize, block_num: usize) -> Result<(), Box<dyn Error>> {
        let block = self.create_block(block_num);
        self.set(dst, Value::Execution(block));
        Ok(())
    }

    pub fn create_block(&self, block_num: usize) -> Execution {
        let exec = self.0.try_lock().unwrap();
        let registers = vec![];
        let vm = exec.vm;
        let code = exec.code.clone();
        let home = Some(self.clone());
        let block = Execution(Arc::new(Mutex::new(ExecutionData {
            vm,
            home,
            code,
            ip: 0,
            block: Some(block_num),
            registers,
        })));
        block
    }

    fn execute_send_message(
        &self,
        dst: usize,
        receiver: usize,
        args: &Vec<usize>,
        selector: &'static str,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let r = self.get(receiver);
        let send_args = args.iter().map(|a| self.get(*a)).collect::<Vec<_>>();
        trace!(
            "args: {}",
            send_args
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(", ")
        );
        trace!("resolve class for {r}");
        let c = self.lookup_class(&r)?;
        trace!("lookup method {selector} in  {c}");
        let m = self.lookup_method_in_class(c, selector)?;
        match m {
            Value::NativeMethod(func) => {
                let result = func(self, r, send_args)?;
                self.set(dst, result);
                Ok(())
            }
            Value::Method(cm) => {
                let mut exec = Execution::new(self.vm(), r, send_args, cm.clone());
                let result = exec.execute()?;
                self.set(dst, result);
                Ok(())
            }
            _ => Err("Only native methods are supported in this VM version".into()),
        }
    }

    fn lookup_class(&self, r: &Value) -> Result<Value, String> {
        match r {
            Value::String(_) => todo!(),
            Value::Integer(_) => Ok(self.vm().integer_class.clone()),
            Value::Float(_) => Ok(self.vm().float_class.clone()),
            Value::Character(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::Object(smalltalk_object) => {
                let class_value = smalltalk_object.class();
                Ok(class_value.into())
            }
            Value::Dictionary(hash_map) => todo!(),
            Value::Array(values) => todo!(),
            Value::Method(compiled_method) => todo!(),
            Value::NativeMethod(_) => todo!(),
            Value::Class(cls) => Ok(Value::Class(cls.meta())),
            Value::Execution(e) => Ok(self.vm().execution_class.clone()),
            Value::Undefined => todo!(),
        }
    }

    fn lookup_method_in_class(&self, c: Value, selector: &str) -> Result<Value, String> {
        match c {
            Value::Class(cls) => {
                if let Some(m) = cls.lookup_method(selector) {
                    Ok(m.clone())
                } else {
                    let mut parent = cls.parent();
                    while let Some(p) = parent {
                        if let Some(m) = p.lookup_method(selector) {
                            return Ok(m.clone());
                        }
                        trace!("Method {} not found in {}, checking parent", selector, p);
                        parent = p.parent();
                    }
                    Err(format!("Method '{}' not found", selector))
                }
            }
            _ => Err("Invalid class value".into()),
        }
    }

    pub fn instructions(&self) -> Vec<Instruction> {
        let exec = self.0.try_lock().unwrap();
        match exec.block {
            Some(n) => exec.code.blocks()[n].instructions.clone(),
            None => exec.code.instructions().clone(),
        }
    }

    pub fn ip(&self) -> usize {
        let exec = self.0.try_lock().unwrap();
        exec.ip
    }

    pub fn next_ip(&self) {
        let mut exec = self.0.try_lock().unwrap();
        exec.ip += 1;
    }

    pub(crate) fn registers(&self) -> Vec<String> {
        match self.home() {
            Some(home) => {
                let mut r = home.registers();
                let exec = self.0.try_lock().unwrap();
                r.extend(exec.registers.iter().map(|v| format!("{}", v)));
                r
            }
            None => {
                let exec = self.0.try_lock().unwrap();
                exec.registers.iter().map(|v| format!("{}", v)).collect()
            }
        }
    }

    fn get_global(&self, var_name: &'static str) -> Result<Value, Box<dyn Error>> {
        let exec = self.0.try_lock().unwrap();
        exec.vm.get_global(var_name)
    }

    fn vm(&self) -> &'static VirtualMachine {
        let exec = self.0.try_lock().unwrap();
        exec.vm
    }
}

impl VirtualMachine {
    pub fn new() -> Self {
        let object_class = SmalltalkClass::new("Object", None, vec![]);
        let number_class = SmalltalkClass::new("Number", Some(object_class.clone()), vec![]);
        let integer_class = SmalltalkClass::new("Integer", Some(number_class.clone()), vec![]);
        let float_class = SmalltalkClass::new("Float", Some(number_class.clone()), vec![]);
        let execution_class = SmalltalkClass::new("Execution", Some(object_class.clone()), vec![]);
        integer_class.insert_method("+", Value::NativeMethod(integer_add));
        execution_class.insert_method("value:", Value::NativeMethod(execution_value1));
        execution_class.insert_method("value:value:", Value::NativeMethod(execution_value2));
        // integer_methods.insert("-", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("*", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("/", Value::Method(Arc::new(CompiledMethod::default())));

        let mut st = HashMap::new();
        st.insert("Integer", integer_class.clone().into());
        st.insert("Number", number_class.clone().into());
        st.insert("Object", object_class.clone().into());
        st.insert("Execution", execution_class.clone().into());

        let meta_class = SmalltalkClass::new("MetaClass", None, vec![]);
        meta_class.insert_method("basicNew", Value::NativeMethod(meta_class_new));
        object_class.set_meta(meta_class.clone());

        Self {
            integer_class: integer_class.into(),
            float_class: float_class.into(),
            meta_class: meta_class.into(),
            execution_class: execution_class.into(),
            st: Value::Dictionary(Arc::new(Mutex::new(st))),
            debugger: None,
        }
    }

    pub fn set_debugger<T: Debugger + 'static>(&mut self, debugger: &'static T) {
        self.debugger = Some(debugger);
    }

    pub fn execute_method(&'static self, method: CompiledMethod) -> Result<Value, Box<dyn Error>> {
        let mut exec = Execution::new(self, Value::Undefined, vec![], method);
        exec.execute()
    }

    pub fn define_class(
        &self,
        name: &'static str,
        parent: Option<&'static str>,
        instance_vars: Vec<&'static str>,
    ) -> Result<(), String> {
        let parent = match parent {
            Some(pname) => {
                let s = self.st.as_dictionary()?;
                match s.try_lock().unwrap().get(pname) {
                    Some(Value::Class(cls)) => Some(cls.clone()),
                    _ => {
                        return Err(format!("Parent class '{}' not found", pname));
                    }
                }
            }
            None => None,
        };

        {
            let dict = self.st.as_dictionary()?;
            if let Ok(ref mut dict) = dict.try_lock() {
                if dict.contains_key(name) {
                    trace!("Class {} already defined", name);
                } else {
                    dict.insert(
                        name,
                        SmalltalkClass::new(name, parent, instance_vars).into(),
                    );
                }
            }
        }
        Ok(())
    }

    pub fn define_method(
        &self,
        class_name: &'static str,
        selector: &'static str,
        parameter_names: Vec<&'static str>,
        src: &'static str,
    ) -> Result<(), String> {
        trace!(
            "Defining method {}>>{} with params {:?} from source: {}",
            class_name, selector, parameter_names, src
        );

        let node = parse_eval(src).map_err(|e| format!("Failed to parse method body: {}", e))?;

        self.define_method_from_node(class_name, false, selector, parameter_names, node)
    }

    fn define_method_from_node(
        &self,
        class_name: &'static str,
        meta: bool,
        selector: &'static str,
        parameter_names: Vec<&'static str>,
        node: crate::parser::topdown::SmalltalkNode,
    ) -> Result<(), String> {
        let class_value = {
            let s = self.st.as_dictionary()?;
            s.try_lock()
                .unwrap()
                .get(class_name)
                .cloned()
                .expect(&format!("Class '{}' not found", class_name))
        };
        match class_value {
            Value::Class(mut cls) => {
                if meta {
                    cls = cls.meta();
                }

                let cm = compile_method(cls.instance_vars(), parameter_names, &node)?;
                for (i, inst) in cm.instructions().iter().enumerate() {
                    trace!("{:04}: {}", i, inst);
                }
                cls.insert_method(selector, cm.into());
            }
            _ => {
                return Err(format!("Class '{}' not found", class_name));
            }
        }

        Ok(())
    }

    fn get_global(&self, var_name: &'static str) -> Result<Value, Box<dyn std::error::Error>> {
        let value = {
            let s = self.st.as_dictionary()?;
            s.try_lock()
                .unwrap()
                .get(var_name)
                .cloned()
                .ok_or(RuntimeError::GlobalNotFound(var_name))?
        };
        Ok(value)
    }

    pub fn compile_src(&self, src: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut parser = SmalltalkParser::new(&src)?;

        while parser.current_token() == "IDENTIFIER" {
            let class_def = parser.parse_class_definition()?;
            match class_def {
                SmalltalkNode::MessageInvoke { receiver, messages }
                    if matches!(
                        messages.first(),
                        Some(&SmalltalkNode::Message {
                            selector: "extend",
                            arguments: _
                        })
                    ) =>
                {
                    if let SmalltalkNode::Identifier(classname) = *receiver {
                        trace!("extending class {}", classname);
                        parser.expect("[")?;
                        self.process_body(&mut parser, classname)?;
                    }
                }
                SmalltalkNode::MessageInvoke { receiver, messages } => {
                    if let Some(SmalltalkNode::Message {
                        selector: _,
                        arguments,
                    }) = messages.first()
                    {
                        if let (
                            SmalltalkNode::Identifier(parent),
                            SmalltalkNode::Identifier(child),
                        ) = (*receiver, arguments.first().unwrap())
                        {
                            trace!("Defining class {} as subclass of {}", child, parent);
                            parser.expect("[")?;
                            let inst_vars = parser.parse_temporaries()?;
                            trace!("Instance variables: {:?}", inst_vars);
                            let _o = parser.pragmas()?;
                            if parent == "nil" {
                                self.define_class(*child, None, inst_vars).unwrap();
                            } else {
                                self.define_class(*child, Some(parent), inst_vars).unwrap();
                            }
                            self.process_body(&mut parser, *child)?;
                        }
                    }
                }
                x => {
                    println!("Parsed class definition: {:#?}", x);
                    return Err("Expected class definition".into());
                }
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn compile_file(
        &self,
        package: &str,
        file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src = {
            let file_path = get_smalltalk_file_path(package, file);
            let mut f = File::open(&file_path)
                .expect(&format!("Failed to open file: {}", file_path.display()));
            let mut src = String::new();
            f.read_to_string(&mut src).expect("Failed to read file");
            src
        };

        trace!("content length: {}", src.len());
        self.compile_src(&src)
    }

    fn process_body(
        &self,
        parser: &mut SmalltalkParser,
        class_name: &'static str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match parser.current_token() {
                "IDENTIFIER" if parser.current_value() == class_name => {
                    let (c, s) = parser.parse_class_reference()?;
                    let (sel, params) = parser.parse_method_pattern()?;
                    trace!("Method pattern: {} {:?}", sel, params);
                    parser.expect("[")?;
                    let body = parser.parse_eval()?;
                    parser.expect("]")?;
                    self.define_method_from_node(class_name, s, sel, params, body)?;
                    trace!("Class reference: {} {}", c, s);
                }
                "IDENTIFIER" => {
                    let ident = parser.current_value();
                    trace!("Identifier: {}", ident);
                    parser.advance();
                    if parser.current_token() == "ASSIGN" {
                        parser.advance();
                        let expr = parser.parse_primary()?;
                        trace!("Assignment to {}: {:?}", ident, expr);
                        parser.expect(".")?;
                        self.define_class_variable(class_name, ident, expr)?;
                    } else {
                        trace!("Method pattern: {} []", ident);
                        parser.expect("[")?;
                        let body = parser.parse_eval()?;
                        parser.expect("]")?;
                        self.define_method_from_node(class_name, false, ident, vec![], body)?;
                    }
                }
                "KEYWORD" | "BINARY" => {
                    let (sel, params) = parser.parse_method_pattern()?;
                    trace!("Method pattern: {} {:?}", sel, params);
                    parser.expect("[")?;
                    let body = parser.parse_eval()?;
                    parser.expect("]")?;
                    self.define_method_from_node(class_name, false, sel, params, body)?;
                }
                "]" => {
                    trace!("end of class definition");
                    parser.expect("]")?;
                    break;
                }
                x => {
                    todo!("token: {x}\n{}", parser.get_context(5));
                }
            }
        }
        Ok(())
    }

    fn define_class_variable(
        &self,
        class_name: &'static str,
        ident: &'static str,
        _expr: SmalltalkNode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Value::Class(cls) = self.get_class(class_name)? {
            cls.meta().insert_variable(ident)?;
        }
        Ok(())
    }

    fn get_class(&self, class_name: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let dict = self.st.as_dictionary()?;
        if let Ok(dict) = dict.try_lock() {
            let c = dict.get(class_name);
            if let Some(c) = c {
                Ok(c.clone())
            } else {
                Err("class not found".into())
            }
        } else {
            Err("class not found".into())
        }
    }
}
