use crate::{
    debug::Debugger,
    vm::{
        block::{CompiledBlock, CompiledMethod},
        error::RuntimeError,
        execution::Execution,
        register::Register,
    },
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
    vec,
};

use pjl_st_macros::st_primitive;
use tracing::{instrument, trace};

use crate::{
    compiler::compile_method,
    get_smalltalk_file_path,
    memory::{SmalltalkClass, SmalltalkObject, Value},
    parser::topdown::{SmalltalkNode, SmalltalkParser, parse_eval},
};

pub mod block;
pub mod error;
pub mod execution;
pub mod instruction;
pub mod register;

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


#[st_primitive]
fn st_integer_add(v1: Value, v2:Value) -> Result<Value, Box<dyn Error>> {
    match (v1, v2) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
        _ => Err("Integer addition requires integer arguments".into()),
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

fn integer_sub(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer addition requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
        _ => Err("Integer addition requires integer arguments".into()),
    }
}

fn integer_mul(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer addition requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
        _ => Err("Integer addition requires integer arguments".into()),
    }
}

fn integer_div(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer addition requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
        _ => Err("Integer addition requires integer arguments".into()),
    }
}

fn integer_gt(_vm: &Execution, receiver: Value, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer greater than requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
        _ => Err("Integer greater than requires integer arguments".into()),
    }
}

fn integer_ge(_vm: &Execution, receiver: Value, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer greater than requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
        _ => Err("Integer greater than requires integer arguments".into()),
    }
}

fn object_eq(_vm: &Execution, receiver: Value, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer greater than requires 1 argument".into());
    }
    Err("Object equality not implemented yet".into())
}

fn integer_eq(_vm: &Execution, receiver: Value, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    if args.len() != 1 {
        return Err("Integer equality requires 1 argument".into());
    }
    match (&receiver, &args[0]) {
        (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
        _ => Err("Integer equality requires integer arguments".into()),
    }
}

fn execution_value0(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    match receiver {
        Value::Execution(a) => a.execute(),
        _ => Err("Execution value0 requires execution arguments".into()),
    }
}

fn execution_value1(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error>> {
    match receiver {
        Value::Execution(a) => {
            a.set(Register::Receiver, args[0].clone());
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
            a.set(Register::Number(0, 1), args[0].clone());
            a.set(Register::Number(0, 2), args[1].clone());
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

impl VirtualMachine {
    pub fn new() -> Self {
        let object_class = SmalltalkClass::new("Object", None, vec![]);
        let meta_class = SmalltalkClass::new(
            "Behavior",
            Some(object_class.clone()),
            vec![
                "superClass",
                "methodDictionary",
                "instanceSpec",
                "subClasses",
                "instanceVariables",
            ],
        );
        let class_description =
            SmalltalkClass::new("ClassDescription", Some(meta_class.clone()), vec![]);
        let class_class = SmalltalkClass::new(
            "Class",
            Some(class_description.clone()),
            vec![
                "name",
                "comment",
                "category",
                "environment",
                "classVariables",
                "sharedPools",
                "pragmaHandlers",
            ],
        );
        object_class.set_meta(class_class.clone());

        let number_class = SmalltalkClass::new("Number", Some(object_class.clone()), vec![]);
        let integer_class = SmalltalkClass::new("Integer", Some(number_class.clone()), vec![]);
        let float_class = SmalltalkClass::new("Float", Some(number_class.clone()), vec![]);
        let execution_class = SmalltalkClass::new("Execution", Some(object_class.clone()), vec![]);
        let nil_class = SmalltalkClass::new("Nil", Some(object_class.clone()), vec![]);
        let undefined_class =
            SmalltalkClass::new("UndefinedObject", Some(object_class.clone()), vec![]);

        integer_class.insert_method("+", Value::NativeMethod(integer_add));
        integer_class.insert_method("-", Value::NativeMethod(integer_sub));
        integer_class.insert_method("*", Value::NativeMethod(integer_mul));
        integer_class.insert_method("/", Value::NativeMethod(integer_div));
        integer_class.insert_method(">", Value::NativeMethod(integer_gt));
        integer_class.insert_method(">=", Value::NativeMethod(integer_ge));
        integer_class.insert_method("=", Value::NativeMethod(integer_eq));
        execution_class.insert_method("value", Value::NativeMethod(execution_value0));
        execution_class.insert_method("value:", Value::NativeMethod(execution_value1));
        execution_class.insert_method("value:value:", Value::NativeMethod(execution_value2));
        // integer_methods.insert("-", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("*", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("/", Value::Method(Arc::new(CompiledMethod::default())));
        // -------------------------------------------------------
        // initialize smalltalk dictionary

        let mut st = HashMap::new();

        st.insert("Object", object_class.clone().into());
        st.insert("Behavior", meta_class.clone().into());
        st.insert("ClassDescription", class_description.clone().into());
        st.insert("Class", class_class.clone().into());

        st.insert("Integer", integer_class.clone().into());
        st.insert("Number", number_class.clone().into());
        st.insert("Execution", execution_class.clone().into());
        st.insert("Nil", nil_class.clone().into());
        st.insert("nil", Value::Nil);
        st.insert("UndefinedObject", undefined_class.clone().into());
        meta_class.insert_method("basicNew", Value::NativeMethod(meta_class_new));
        // object_class.set_meta(meta_class.clone());
        st.insert("Behavior", meta_class.clone().into());

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
        let exec = Execution::new(self, Value::Undefined, vec![], method);
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                    cls = cls.meta().class();
                }
                trace!("compiling for {}", cls.name());
                let cm = compile_method(cls.instance_vars(), parameter_names, &node)?;
                if let Value::Method(cm) = &cm {
                    for (i, inst) in cm.instructions().iter().enumerate() {
                        trace!("{:04}: {}", i, inst);
                    }
                }
                cls.insert_method(selector, cm.into());
            }
            _ => {
                return Err(format!("Class '{}' not found", class_name).into());
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
                            let pragmas = parser.pragmas()?;
                            if pragmas.iter().any(|p| p.starts_with("primitive: ")) {
                                todo!("handle primitive")
                            }
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
        trace!("Compiling file: {}::{}", package, file);
        let src = {
            let file_path = get_smalltalk_file_path(package, file);
            let mut f = File::open(&file_path)
                .expect(&format!("Failed to open file: {}", file_path.display()));
            let mut src = String::new();
            f.read_to_string(&mut src).expect("Failed to read file");
            src
        };

        trace!("content length: {}", src.len());
        match self.compile_src(&src) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error compiling file {}::{}: {}", package, file, e).into()),
        }
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
            cls.meta().class().insert_variable(ident)?;
        }
        Ok(())
    }

    pub fn get_class(&self, class_name: &str) -> Result<Value, Box<dyn std::error::Error>> {
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

    pub fn get_static_method(
        &self,
        class_name: &'static str,
        selector: &'static str,
    ) -> Result<CompiledMethod, Box<dyn std::error::Error>> {
        let cls = self.get_class(class_name)?;
        if let Value::Class(cls) = cls {
            if let Some(Value::Method(method)) = cls.meta().class().lookup_method(selector) {
                Ok(method.clone())
            } else {
                Err("method not found".into())
            }
        } else {
            Err("class not found".into())
        }
    }
}
