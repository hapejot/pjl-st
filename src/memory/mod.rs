use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use pjl_static_strings::StringTable;
use tracing::trace;

use crate::vm::{CompiledMethod, Execution};

#[derive(Debug, Clone)]
pub struct SmalltalkObject(Arc<Mutex<SmalltalkObjectData>>);
impl SmalltalkObject {
    pub fn new(class: SmalltalkClass) -> Self {
        Self(Arc::new(Mutex::new(SmalltalkObjectData {
            class,
            values: vec![],
        })))
    }

    pub fn class(&self) -> SmalltalkClass {
        self.0.lock().unwrap().class.clone()
    }

    pub fn set_instance_var(&self, index: usize, value: Value) {
        let mut data = self.0.lock().unwrap();
        if index >= data.values.len() {
            data.values.resize(index + 1, Value::Undefined);
        }
        data.values[index] = value;
        trace!("Set instance var[{}] to {}", index, data.values[index]);
    }

    pub fn get_instance_var(&self, index: usize) -> Value {
        let data = self.0.lock().unwrap();
        if index >= data.values.len() {
            Value::Undefined
        } else {
            data.values[index].clone()
        }
    }

    pub fn get_instance_vars(&self) -> Vec<Value> {
        let data = self.0.lock().unwrap();
        data.values.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SmalltalkObjectData {
    pub class: SmalltalkClass,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct SmalltalkClass(Arc<Mutex<SmalltalkClassData>>);
impl SmalltalkClass {
    pub(crate) fn lookup_method(&self, selector: &str) -> Option<Value> {
        self.0.lock().unwrap().methods.get(selector).cloned()
    }

    pub(crate) fn new(name: &'static str, vars: Vec<&'static str>) -> Self {
        let cls = SmalltalkClass(Arc::new(Mutex::new(SmalltalkClassData {
            name,
            parent: None,
            methods: HashMap::new(),
            vars,
        })));
        cls
    }

    pub(crate) fn insert_method(&self, arg: &'static str, integer_add: Value) {
        let mut methods = self.0.lock().unwrap();
        trace!("Inserting method {}", arg);
        (*methods).methods.insert(arg, integer_add);
    }

    pub(crate) fn instance_vars(&self) -> Vec<&'static str> {
        self.0.lock().unwrap().vars.clone()
    }
}
#[derive(Debug, Clone)]
pub struct SmalltalkClassData {
    pub parent: Option<SmalltalkObject>,
    pub methods: HashMap<&'static str, Value>,
    name: &'static str,
    vars: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(&'static str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(SmalltalkObject),
    Dictionary(Arc<Mutex<HashMap<&'static str, Value>>>),
    Array(Arc<Mutex<Vec<Value>>>),
    Method(CompiledMethod),
    NativeMethod(fn(&mut Execution, Value, Vec<Value>) -> Result<Value, String>),
    Class(SmalltalkClass),
    Undefined,
}
impl Value {
    pub(crate) fn as_dictionary(&self) -> Result<Arc<Mutex<HashMap<&'static str, Value>>>, String> {
        match self {
            Value::Dictionary(hash_map) => Ok(hash_map.clone()),
            _ => Err("Expected Dictionary value".into()),
        }
    }
}

impl From<SmalltalkClass> for Value {
    fn from(class: SmalltalkClass) -> Self {
        Value::Class(class)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(StringTable::get(&value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(StringTable::get(value))
    }
}

impl From<CompiledMethod> for Value {
    fn from(value: CompiledMethod) -> Self {
        Value::Method(value)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Object(o) => write!(
                f,
                "<Object {} [{}]>",
                o.class(),
                o.get_instance_vars()
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Dictionary(_) => write!(f, "<Dictionary>"),
            Value::Array(_) => write!(f, "<Array>"),
            Value::Method(_) => write!(f, "<Method>"),
            Value::NativeMethod(_) => write!(f, "<NativeMethod>"),
            Value::Class(c) => write!(f, "<Class {}>", c.0.lock().unwrap().name),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

impl std::fmt::Display for SmalltalkClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0.lock().unwrap();
        write!(f, "Class {}", data.name)
    }
}
