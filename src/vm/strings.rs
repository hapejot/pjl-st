use std::error::Error;

use crate::{
    memory::{SmalltalkString, Value},
    vm::execution::Execution,
};

pub(super) fn new(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Class(class), [Value::Integer(size)]) = (receiver, args.as_slice()) else {
        return Err("String basicNew: requires a non-negative integer size".into());
    };
    if class.name()? != "String" {
        return Err("String basicNew: requires String as receiver".into());
    }
    let size = usize::try_from(*size).map_err(|_| "String size must be a non-negative integer")?;
    Ok(Value::String(SmalltalkString::with_size(size)?))
}

pub(super) fn size(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), []) = (receiver, args.as_slice()) else {
        return Err("String size requires a string and no arguments".into());
    };
    Ok(Value::Integer(i64::try_from(value.len()?)?))
}

pub(super) fn at(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), [Value::Integer(index)]) = (receiver, args.as_slice()) else {
        return Err("String at: requires an integer index".into());
    };
    Ok(Value::Character(value.at(*index)?))
}

pub(super) fn at_put(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), [Value::Integer(index), Value::Character(character)]) =
        (receiver, args.as_slice())
    else {
        return Err("String at:put: requires an integer index and a character".into());
    };
    value.at_put(*index, *character)?;
    Ok(Value::Character(*character))
}

pub(super) fn equal(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), [other]) = (receiver, args.as_slice()) else {
        return Err("String = requires one argument".into());
    };
    let Value::String(other) = other else {
        return Ok(Value::Boolean(false));
    };
    if value.same_identity(other) {
        value.len()?;
        return Ok(Value::Boolean(true));
    }
    Ok(Value::Boolean(
        value.to_rust_string()? == other.to_rust_string()?,
    ))
}

pub(super) fn concatenate(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), [Value::String(other)]) = (receiver, args.as_slice()) else {
        return Err("String , requires a string argument".into());
    };
    let mut result = value.to_rust_string()?;
    let other = other.to_rust_string()?;
    result.try_reserve(other.len())?;
    result.push_str(&other);
    Ok(Value::String(SmalltalkString::new(&result)?))
}

pub(super) fn as_string(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match (receiver, args.as_slice()) {
        (Value::String(value), []) => Ok(Value::String(value)),
        _ => Err("String asString requires a string and no arguments".into()),
    }
}

pub(super) fn copy(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::String(value), []) = (receiver, args.as_slice()) else {
        return Err("String copy requires a string and no arguments".into());
    };
    Ok(Value::String(SmalltalkString::new(
        &value.to_rust_string()?,
    )?))
}
