use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SmalltalkString(Arc<Mutex<Vec<u16>>>);

impl SmalltalkString {
    pub fn new(value: &str) -> Result<Self, String> {
        if value.chars().any(|character| u32::from(character) > 0xffff) {
            return Err("String contains a character outside the BMP".into());
        }
        let mut units = Vec::new();
        units
            .try_reserve_exact(value.chars().count())
            .map_err(|error| format!("Cannot allocate String: {error}"))?;
        units.extend(value.encode_utf16());
        Ok(Self(Arc::new(Mutex::new(units))))
    }

    pub fn to_rust_string(&self) -> Result<String, String> {
        let units = self
            .0
            .lock()
            .map_err(|error| format!("String lock: {error}"))?;
        String::from_utf16(&units).map_err(|error| format!("Invalid String: {error}"))
    }

    pub(crate) fn with_size(size: usize) -> Result<Self, String> {
        let mut units = Vec::new();
        units
            .try_reserve_exact(size)
            .map_err(|error| format!("Cannot allocate String: {error}"))?;
        units.resize(size, 0);
        Ok(Self(Arc::new(Mutex::new(units))))
    }

    pub(crate) fn len(&self) -> Result<usize, String> {
        Ok(self
            .0
            .lock()
            .map_err(|error| format!("String lock: {error}"))?
            .len())
    }

    fn index(index: i64, len: usize) -> Result<usize, String> {
        match index
            .checked_sub(1)
            .and_then(|offset| usize::try_from(offset).ok())
        {
            Some(offset) if offset < len => Ok(offset),
            _ => Err(format!("String index {index} out of bounds (size {len})")),
        }
    }

    pub(crate) fn at(&self, index: i64) -> Result<char, String> {
        let units = self
            .0
            .lock()
            .map_err(|error| format!("String lock: {error}"))?;
        let unit = units[Self::index(index, units.len())?];
        char::from_u32(u32::from(unit)).ok_or_else(|| "Invalid String character".into())
    }

    pub(crate) fn at_put(&self, index: i64, character: char) -> Result<(), String> {
        let unit = u16::try_from(u32::from(character))
            .map_err(|_| "String character must be in the BMP")?;
        let mut units = self
            .0
            .lock()
            .map_err(|error| format!("String lock: {error}"))?;
        let index = Self::index(index, units.len())?;
        units[index] = unit;
        Ok(())
    }

    pub(crate) fn same_identity(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    pub(crate) fn try_format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.try_lock() {
            Ok(units) => match String::from_utf16(&units) {
                Ok(value) => write!(f, "{value}"),
                Err(error) => write!(f, "<String invalid: {error}>"),
            },
            Err(error) => write!(f, "<String unavailable: {error}>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmalltalkString;
    use crate::memory::Value;

    #[test]
    fn poisoned_string_lock_is_reported() {
        let value = SmalltalkString::new("text").unwrap();
        let other = value.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison String lock");
            })
            .join()
            .is_err()
        );
        assert!(value.to_rust_string().unwrap_err().contains("String lock"));
        assert!(value.len().unwrap_err().contains("String lock"));
        assert!(value.at(1).unwrap_err().contains("String lock"));
        assert!(value.at_put(1, 'a').unwrap_err().contains("String lock"));
        assert!(format!("{}", Value::String(value)).contains("String unavailable"));
    }
}
