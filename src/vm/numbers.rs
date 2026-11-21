use std::{cmp::Ordering, error::Error};

use crate::{memory::Value, vm::execution::Execution};

type ResultValue = Result<Value, Box<dyn Error + Send + Sync>>;

pub(super) fn float_basic_new(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    let (Value::Class(_), []) = (receiver, args.as_slice()) else {
        return Err("Float basicNew requires a class and no arguments".into());
    };
    Err("Float instances cannot be created with basicNew".into())
}

#[derive(Clone, Copy)]
enum Number {
    Integer(i64),
    Float(f64),
}

impl Number {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(value) => Some(Self::Integer(*value)),
            Value::Float(value) => Some(Self::Float(*value)),
            _ => None,
        }
    }

    fn as_float(self) -> f64 {
        match self {
            Self::Integer(value) => value as f64,
            Self::Float(value) => value,
        }
    }

    fn compare(self, other: Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Some(a.cmp(&b)),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(&b),
            (Self::Integer(a), Self::Float(b)) => compare_integer_float(a, b),
            (Self::Float(a), Self::Integer(b)) => {
                compare_integer_float(b, a).map(Ordering::reverse)
            }
        }
    }
}

fn compare_integer_float(integer: i64, float: f64) -> Option<Ordering> {
    if float.is_nan() {
        return None;
    }
    if float >= 9_223_372_036_854_775_808.0 {
        return Some(Ordering::Less);
    }
    if float < -9_223_372_036_854_775_808.0 {
        return Some(Ordering::Greater);
    }
    let truncated = float.trunc() as i64;
    match integer.cmp(&truncated) {
        Ordering::Equal => 0.0_f64.partial_cmp(&float.fract()),
        ordering => Some(ordering),
    }
}

#[derive(Clone, Copy)]
enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn arithmetic(receiver: Value, args: Vec<Value>, operation: Arithmetic) -> ResultValue {
    let [other] = args.as_slice() else {
        return Err("Numeric operation requires one argument".into());
    };
    let (Some(a), Some(b)) = (Number::from_value(&receiver), Number::from_value(other)) else {
        return Err("Numeric operation requires number arguments".into());
    };
    if let (Number::Integer(a), Number::Integer(b)) = (a, b) {
        let value = match operation {
            Arithmetic::Add => a.checked_add(b),
            Arithmetic::Subtract => a.checked_sub(b),
            Arithmetic::Multiply => a.checked_mul(b),
            Arithmetic::Divide if b == 0 => return Err("Integer division by zero".into()),
            Arithmetic::Divide => a.checked_div(b),
        };
        let overflow = match operation {
            Arithmetic::Add => "Integer addition overflow",
            Arithmetic::Subtract => "Integer subtraction overflow",
            Arithmetic::Multiply => "Integer multiplication overflow",
            Arithmetic::Divide => "Integer division overflow",
        };
        return value.map(Value::Integer).ok_or_else(|| overflow.into());
    }
    let (a, b) = (a.as_float(), b.as_float());
    if matches!(operation, Arithmetic::Divide) && b == 0.0 {
        return Err("Float division by zero".into());
    }
    Ok(Value::Float(match operation {
        Arithmetic::Add => a + b,
        Arithmetic::Subtract => a - b,
        Arithmetic::Multiply => a * b,
        Arithmetic::Divide => a / b,
    }))
}

fn compare(receiver: Value, args: Vec<Value>, predicate: fn(Ordering) -> bool) -> ResultValue {
    let [other] = args.as_slice() else {
        return Err("Numeric comparison requires one argument".into());
    };
    let (Some(a), Some(b)) = (Number::from_value(&receiver), Number::from_value(other)) else {
        return Err("Numeric comparison requires number arguments".into());
    };
    Ok(Value::Boolean(a.compare(b).is_some_and(predicate)))
}

pub(super) fn add(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    arithmetic(receiver, args, Arithmetic::Add)
}

pub(super) fn subtract(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    arithmetic(receiver, args, Arithmetic::Subtract)
}

pub(super) fn multiply(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    arithmetic(receiver, args, Arithmetic::Multiply)
}

pub(super) fn divide(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    arithmetic(receiver, args, Arithmetic::Divide)
}

pub(super) fn less(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    compare(receiver, args, |ordering| ordering == Ordering::Less)
}

pub(super) fn less_equal(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    compare(receiver, args, |ordering| ordering != Ordering::Greater)
}

pub(super) fn greater(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    compare(receiver, args, |ordering| ordering == Ordering::Greater)
}

pub(super) fn greater_equal(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    compare(receiver, args, |ordering| ordering != Ordering::Less)
}

pub(super) fn equal(_: &Execution, receiver: Value, args: Vec<Value>) -> ResultValue {
    let (Some(a), [other]) = (Number::from_value(&receiver), args.as_slice()) else {
        return Err("Numeric equality requires a number and one argument".into());
    };
    Ok(Value::Boolean(
        Number::from_value(other).is_some_and(|b| a.compare(b) == Some(Ordering::Equal)),
    ))
}
