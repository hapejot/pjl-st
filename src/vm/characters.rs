use std::error::Error;

use crate::{
    memory::{SmalltalkString, Value},
    vm::execution::Execution,
};

pub(super) fn as_string(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Character(character), []) = (receiver, args.as_slice()) else {
        return Err("Character asString requires a character and no arguments".into());
    };
    let mut buffer = [0; 4];
    Ok(Value::String(SmalltalkString::new(
        character.encode_utf8(&mut buffer),
    )?))
}

pub(super) fn basic_new(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Class(_), []) = (receiver, args.as_slice()) else {
        return Err("Character basicNew requires a class and no arguments".into());
    };
    Err("Character instances cannot be created with basicNew".into())
}
