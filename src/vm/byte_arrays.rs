use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{memory::Value, vm::execution::Execution};

pub(super) fn new(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Class(class), [Value::Integer(size)]) = (receiver, args.as_slice()) else {
        return Err("ByteArray basicNew: requires a non-negative integer size".into());
    };
    if class.name()? != "ByteArray" {
        return Err("ByteArray basicNew: requires ByteArray as receiver".into());
    }
    let size = usize::try_from(*size).map_err(|_| "ByteArray size must be non-negative")?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size)?;
    bytes.resize(size, 0);
    Ok(Value::ByteArray(Arc::new(Mutex::new(bytes))))
}

fn index(index: i64, size: usize) -> Result<usize, String> {
    match index.checked_sub(1).and_then(|i| usize::try_from(i).ok()) {
        Some(index) if index < size => Ok(index),
        _ => Err(format!(
            "ByteArray index {index} out of bounds (size {size})"
        )),
    }
}

pub(super) fn size(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::ByteArray(bytes), []) = (receiver, args.as_slice()) else {
        return Err("ByteArray size requires a byte array and no arguments".into());
    };
    let bytes = bytes
        .lock()
        .map_err(|error| format!("ByteArray lock: {error}"))?;
    Ok(Value::Integer(i64::try_from(bytes.len())?))
}

pub(super) fn at(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::ByteArray(bytes), [Value::Integer(position)]) = (receiver, args.as_slice()) else {
        return Err("ByteArray at: requires an integer index".into());
    };
    let bytes = bytes
        .lock()
        .map_err(|error| format!("ByteArray lock: {error}"))?;
    Ok(Value::Integer(i64::from(
        bytes[index(*position, bytes.len())?],
    )))
}

pub(super) fn at_put(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::ByteArray(bytes), [Value::Integer(position), Value::Integer(value)]) =
        (receiver, args.as_slice())
    else {
        return Err("ByteArray at:put: requires an integer index and byte".into());
    };
    let byte = u8::try_from(*value).map_err(|_| "ByteArray value must be between 0 and 255")?;
    let mut bytes = bytes
        .lock()
        .map_err(|error| format!("ByteArray lock: {error}"))?;
    let position = index(*position, bytes.len())?;
    bytes[position] = byte;
    Ok(Value::Integer(*value))
}

pub(super) fn copy(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::ByteArray(bytes), []) = (receiver, args.as_slice()) else {
        return Err("ByteArray copy requires a byte array and no arguments".into());
    };
    let bytes = bytes
        .lock()
        .map_err(|error| format!("ByteArray lock: {error}"))?;
    let mut result = Vec::new();
    result.try_reserve_exact(bytes.len())?;
    result.extend_from_slice(&bytes);
    Ok(Value::ByteArray(Arc::new(Mutex::new(result))))
}
