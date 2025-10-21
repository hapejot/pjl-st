use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub struct SmalltalkObject(Arc<SmalltalkObjectData>);

#[derive(Debug, Clone, PartialEq)]
pub struct SmalltalkObjectData {
    pub class: SmalltalkObject,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(&'static str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(SmalltalkObject),
    Dictionary(HashMap<&'static str, Value>),
    Array(Vec<Value>),
    Undefined,
}

pub struct MethodTable {
    pub methods: HashMap<&'static str, SmalltalkObject>,
}
