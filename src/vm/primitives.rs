use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    memory::{SmalltalkObject, Value},
    vm::execution::Execution,
};

pub(super) fn integer_bit_and(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Integer(value), [Value::Integer(other)]) = (receiver, args.as_slice()) else {
        return Err("bitAnd: requires an integer argument".into());
    };
    Ok(Value::Integer(value & other))
}

pub(super) fn integer_bit_shift(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Integer(value), [Value::Integer(shift)]) = (receiver, args.as_slice()) else {
        return Err("bitShift: requires an integer argument".into());
    };
    let result = if *shift < 0 {
        let amount = shift.unsigned_abs();
        if amount >= 64 {
            value >> 63
        } else {
            value >> amount
        }
    } else if value == 0 {
        0
    } else if *shift >= 64 {
        return Err("bitShift: integer overflow".into());
    } else {
        let wide = (value as i128) << *shift;
        i64::try_from(wide).map_err(|_| "bitShift: integer overflow")?
    };
    Ok(Value::Integer(result))
}

pub(super) fn execution_value0(
    caller: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match receiver {
        Value::Execution(a) => a.invoke_block(caller, &args),
        _ => Err("Execution value requires execution context".into()),
    }
}

pub(super) fn execution_value1(
    caller: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match receiver {
        Value::Execution(a) => a.invoke_block(caller, &args),
        _ => Err("Execution value1 requires execution arguments".into()),
    }
}

pub(super) fn execution_value2(
    caller: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match receiver {
        Value::Execution(a) => a.invoke_block(caller, &args),
        _ => Err("Execution value1 requires execution arguments".into()),
    }
}

pub(super) fn execution_while_true(
    caller: &Execution,
    receiver: &Execution,
    body: &Execution,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    loop {
        match receiver.invoke_block(caller, &[])? {
            Value::Boolean(true) => {
                body.invoke_block(caller, &[])?;
            }
            Value::Boolean(false) => return Ok(Value::Nil),
            _ => return Err("whileTrue: condition must answer a Boolean".into()),
        }
    }
}

pub(super) fn array_new(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let Value::Class(cls) = receiver else {
        return Err("Array basicNew: requires Array as receiver".into());
    };
    if cls.name()? != "Array" {
        return Err("Array basicNew: requires Array as receiver".into());
    }
    let [Value::Integer(size)] = args.as_slice() else {
        return Err("Array size must be a non-negative integer".into());
    };
    let size = usize::try_from(*size).map_err(|_| "Array size must be a non-negative integer")?;
    let mut values = Vec::new();
    values.try_reserve_exact(size)?;
    values.resize(size, Value::Nil);
    Ok(Value::Array(Arc::new(Mutex::new(values))))
}

fn array_index(index: &Value, len: usize) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let Value::Integer(index) = index else {
        return Err("Array index must be an integer".into());
    };
    let offset = index.checked_sub(1).and_then(|i| usize::try_from(i).ok());
    match offset {
        Some(i) if i < len => Ok(i),
        _ => Err(format!("Array index {index} out of bounds (size {len})").into()),
    }
}

pub(super) fn array_at(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Array(array), [index]) = (receiver, args.as_slice()) else {
        return Err("Array at: requires one index".into());
    };
    let array = array.lock().map_err(|e| format!("Array lock: {e}"))?;
    Ok(array[array_index(index, array.len())?].clone())
}

pub(super) fn array_at_put(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Array(array), [index, value]) = (receiver, args.as_slice()) else {
        return Err("Array at:put: requires an index and value".into());
    };
    let mut array = array.lock().map_err(|e| format!("Array lock: {e}"))?;
    let index = array_index(index, array.len())?;
    array[index] = value.clone();
    Ok(value.clone())
}

pub(super) fn array_size(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let Value::Array(array) = receiver else {
        return Err("Array size requires an array".into());
    };
    if !args.is_empty() {
        return Err("Array size takes no arguments".into());
    }
    let array = array.lock().map_err(|e| format!("Array lock: {e}"))?;
    Ok(Value::Integer(i64::try_from(array.len())?))
}

pub(super) fn meta_class_new(
    _vm: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if !args.is_empty() {
        return Err("basicNew takes no arguments".into());
    }
    match receiver {
        Value::Class(cls) => {
            if cls.is_variable()? {
                return Err("basicNew requires a fixed-size class".into());
            }
            let obj = SmalltalkObject::new(cls.clone());
            Ok(Value::Object(obj))
        }
        _ => Err("MetaClass new called on non-class value".into()),
    }
}
