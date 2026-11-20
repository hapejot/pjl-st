use std::error::Error;

use pjl_static_strings::StringTable;

use crate::{
    memory::{SmalltalkString, Value},
    vm::execution::Execution,
};

pub(super) fn text(symbol: &str) -> Result<&str, String> {
    let text = symbol.strip_prefix('#').ok_or("Symbol must start with #")?;
    if text.chars().any(|character| u32::from(character) > 0xffff) {
        return Err("Symbol contains a character outside the BMP".into());
    }
    Ok(text)
}

fn intern_value(value: Value) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match value {
        Value::String(value) => {
            let value = value.to_rust_string()?;
            Ok(Value::Symbol(StringTable::get(&format!("#{value}"))))
        }
        Value::Symbol(symbol) => {
            text(symbol)?;
            Ok(Value::Symbol(symbol))
        }
        _ => Err("Symbol intern: requires a string or symbol".into()),
    }
}

pub(super) fn intern(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Class(class), [value]) = (receiver, args.as_slice()) else {
        return Err("Symbol intern: requires a string or symbol".into());
    };
    if class.name()? != "Symbol" {
        return Err("Symbol intern: requires Symbol as receiver".into());
    }
    intern_value(value.clone())
}

pub(super) fn as_symbol(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if !args.is_empty() {
        return Err("asSymbol takes no arguments".into());
    }
    intern_value(receiver)
}

pub(super) fn as_string(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Symbol(symbol), []) = (receiver, args.as_slice()) else {
        return Err("Symbol asString requires a symbol and no arguments".into());
    };
    Ok(Value::String(SmalltalkString::new(text(symbol)?)?))
}

pub(super) fn size(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Symbol(symbol), []) = (receiver, args.as_slice()) else {
        return Err("Symbol size requires a symbol and no arguments".into());
    };
    Ok(Value::Integer(i64::try_from(
        text(symbol)?.chars().count(),
    )?))
}

pub(super) fn at(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Symbol(symbol), [Value::Integer(index)]) = (receiver, args.as_slice()) else {
        return Err("Symbol at: requires an integer index".into());
    };
    let text = text(symbol)?;
    let offset = index
        .checked_sub(1)
        .and_then(|offset| usize::try_from(offset).ok());
    match offset.and_then(|offset| text.chars().nth(offset)) {
        Some(character) => Ok(Value::Character(character)),
        None => Err(format!(
            "Symbol index {index} out of bounds (size {})",
            text.chars().count()
        )
        .into()),
    }
}

pub(super) fn equal(
    _: &Execution,
    receiver: Value,
    args: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let (Value::Symbol(symbol), [other]) = (receiver, args.as_slice()) else {
        return Err("Symbol = requires one argument".into());
    };
    let symbol = text(symbol)?;
    match other {
        Value::Symbol(other) => Ok(Value::Boolean(symbol == text(other)?)),
        _ => Ok(Value::Boolean(false)),
    }
}

pub(super) fn immutable(
    _: &Execution,
    receiver: Value,
    _: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let Value::Symbol(symbol) = receiver else {
        return Err("Symbol mutation requires a symbol".into());
    };
    text(symbol)?;
    Err("Cannot modify a Symbol".into())
}
