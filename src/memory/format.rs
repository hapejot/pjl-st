use std::sync::{Arc, Mutex};

use super::{SmalltalkClass, SmalltalkObject, Value};

enum FormatAncestor {
    Array(Arc<Mutex<Vec<Value>>>),
    Object(SmalltalkObject),
}

fn format_value(
    value: &Value,
    f: &mut std::fmt::Formatter<'_>,
    ancestors: &mut Vec<FormatAncestor>,
) -> std::fmt::Result {
    match value {
        Value::Array(array) => {
            if ancestors.iter().any(|ancestor| {
                matches!(
                    ancestor, FormatAncestor::Array(other) if Arc::ptr_eq(other, array)
                )
            }) {
                return write!(f, "#(...)");
            }
            let values = match array.try_lock() {
                Ok(values) => values.clone(),
                Err(error) => return write!(f, "<Array unavailable: {error}>"),
            };
            ancestors.push(FormatAncestor::Array(array.clone()));
            write!(f, "#(")?;
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    write!(f, " ")?;
                }
                format_value(value, f, ancestors)?;
            }
            ancestors.pop();
            write!(f, ")")
        }
        Value::ByteArray(bytes) => {
            let bytes = match bytes.try_lock() {
                Ok(bytes) => bytes,
                Err(error) => return write!(f, "<ByteArray unavailable: {error}>"),
            };
            write!(f, "#[")?;
            for (index, byte) in bytes.iter().enumerate() {
                if index > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{byte}")?;
            }
            write!(f, "]")
        }
        Value::Object(object) => {
            let (class, values) = match object.0.try_lock() {
                Ok(data) => (data.class.clone(), data.values.clone()),
                Err(error) => return write!(f, "<Object unavailable: {error}>"),
            };
            if ancestors.iter().any(|ancestor| {
                matches!(
                    ancestor, FormatAncestor::Object(other) if Arc::ptr_eq(&other.0, &object.0)
                )
            }) {
                return write!(f, "<Object {class} [...]>");
            }
            ancestors.push(FormatAncestor::Object(object.clone()));
            write!(f, "<Object {class} [")?;
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                format_value(value, f, ancestors)?;
            }
            ancestors.pop();
            write!(f, "]>")
        }
        Value::String(s) => s.try_format(f),
        Value::Symbol(s) => write!(f, "{s}"),
        Value::Integer(i) => write!(f, "{i}"),
        Value::Float(fl) => write!(f, "{fl}"),
        Value::Boolean(b) => write!(f, "{b}"),
        Value::Character(c) => write!(f, "${c}"),
        Value::Dictionary(_) => write!(f, "<Dictionary>"),
        Value::Method(_) => write!(f, "<Method>"),
        Value::NativeMethod(_) => write!(f, "<NativeMethod>"),
        Value::Class(c) => match c.0.try_lock() {
            Ok(data) => write!(f, "<Class {}>", data.name),
            Err(error) => write!(f, "<Class unavailable: {error}>"),
        },
        Value::Execution(_) => write!(f, "<Execution>"),
        Value::Undefined => write!(f, "undefined"),
        Value::Nil => write!(f, "nil"),
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_value(self, f, &mut Vec::new())
    }
}

impl std::fmt::Display for SmalltalkClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.try_lock() {
            Ok(data) => write!(f, "Class {}", data.name),
            Err(error) => write!(f, "Class unavailable: {error}"),
        }
    }
}
