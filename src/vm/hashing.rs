use std::hash::{DefaultHasher, Hash, Hasher};

use crate::memory::Value;

use super::symbols;

fn finish<T: Hash>(value: T) -> i64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

pub(super) fn bucket_hash(value: &Value) -> Result<Option<i64>, String> {
    Ok(match value {
        Value::Integer(value) => Some(finish((0u8, *value))),
        Value::Float(value) => {
            let integral = value.is_finite()
                && value.fract() == 0.0
                && *value >= -9_223_372_036_854_775_808.0
                && *value < 9_223_372_036_854_775_808.0;
            if integral {
                Some(finish((0u8, *value as i64)))
            } else {
                Some(finish((1u8, value.to_bits())))
            }
        }
        Value::Symbol(value) => Some(finish((2u8, symbols::text(value)?))),
        _ => None,
    })
}

pub(super) fn value_hash(value: &Value) -> Result<i64, String> {
    match value {
        Value::String(value) => Ok(finish((3u8, value.to_rust_string()?))),
        _ => bucket_hash(value)?.ok_or_else(|| "No value hash for receiver".to_string()),
    }
}
