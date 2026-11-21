use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    vec,
};

use pjl_static_strings::StringTable;
use tracing::trace;

use crate::vm::{block::CompiledMethod, execution::Execution};

mod format;
mod string;

pub use string::SmalltalkString;

#[derive(Debug, Clone)]
pub struct SmalltalkObject(Arc<Mutex<SmalltalkObjectData>>);
impl SmalltalkObject {
    pub fn new(class: SmalltalkClass) -> Self {
        Self(Arc::new(Mutex::new(SmalltalkObjectData {
            class,
            values: vec![],
        })))
    }

    pub fn class(&self) -> Result<SmalltalkClass, String> {
        Ok(self
            .0
            .lock()
            .map_err(|e| format!("Object lock: {e}"))?
            .class
            .clone())
    }

    pub fn set_instance_var(&self, index: usize, value: Value) -> Result<(), String> {
        let logged_value = value.clone();
        let mut data = self.0.lock().map_err(|e| format!("Object lock: {e}"))?;
        if index >= data.values.len() {
            let len = index
                .checked_add(1)
                .ok_or_else(|| format!("Cannot allocate instance variable {index}"))?;
            let additional = len - data.values.len();
            data.values
                .try_reserve(additional)
                .map_err(|e| format!("Cannot allocate instance variable {index}: {e}"))?;
            data.values.resize(len, Value::Undefined);
        }
        data.values[index] = value;
        drop(data);
        trace!("Set instance var[{}] to {}", index, logged_value);
        Ok(())
    }

    pub fn get_instance_var(&self, index: usize) -> Result<Value, String> {
        let data = self.0.lock().map_err(|e| format!("Object lock: {e}"))?;
        if index >= data.values.len() {
            Ok(Value::Undefined)
        } else {
            Ok(data.values[index].clone())
        }
    }

    pub fn get_instance_vars(&self) -> Result<Vec<Value>, String> {
        let data = self.0.lock().map_err(|e| format!("Object lock: {e}"))?;
        Ok(data.values.clone())
    }
}

#[derive(Debug, Clone)]
pub struct SmalltalkObjectData {
    pub class: SmalltalkClass,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct SmalltalkClass(Arc<Mutex<SmalltalkClassData>>);
impl SmalltalkClass {
    pub fn lookup_method(&self, selector: &str) -> Result<Option<Value>, String> {
        let data = self.0.lock().map_err(|e| format!("Class lock: {e}"))?;
        Ok(data.methods.get(selector).cloned())
    }

    pub fn method_names(&self) -> Result<Vec<&'static str>, String> {
        Ok(self
            .0
            .lock()
            .map_err(|e| format!("Class lock: {e}"))?
            .methods
            .keys()
            .cloned()
            .collect::<Vec<&'static str>>())
    }

    pub(crate) fn new(
        name: &'static str,
        parent: Option<SmalltalkClass>,
        vars: Vec<&'static str>,
    ) -> Result<Self, String> {
        let mut names: HashSet<_> = match &parent {
            Some(parent) => parent.instance_vars()?.into_iter().collect(),
            None => HashSet::new(),
        };
        for &var in &vars {
            if !names.insert(var) {
                return Err(format!(
                    "Instance variable {var} is already defined in class {name} or an ancestor"
                ));
            }
        }
        let metaname = get_meta_name(name);

        let meta_parent = match &parent {
            Some(p) => Some(p.meta()?.class()?),
            None => None,
        };

        let metacls = SmalltalkClass(Arc::new(Mutex::new(SmalltalkClassData {
            name: metaname,
            parent: meta_parent,
            meta: None,
            methods: HashMap::new(),
            vars: vec![],
        })));
        let inherited_spec = match &parent {
            Some(p) => match p.meta()?.get_instance_var(2)? {
                Value::Integer(spec) => spec,
                _ => return Err(format!("Class {} has no instance specification", p.name()?)),
            },
            None => 0,
        };
        let slots = i64::try_from(vars.len()).map_err(|_| "Too many instance variables")?;
        let spec = (inherited_spec >> 12)
            .checked_add(slots)
            .and_then(|n| n.checked_mul(1 << 12))
            .ok_or("Instance specification overflow")?;
        let metaobj = SmalltalkObject(Arc::new(Mutex::new(SmalltalkObjectData {
            class: metacls,
            values: vec![Value::Undefined, Value::Undefined, Value::Integer(spec)],
        })));
        let cls = SmalltalkClass(Arc::new(Mutex::new(SmalltalkClassData {
            name,
            parent,
            meta: Some(metaobj),
            methods: HashMap::new(),
            vars,
        })));
        Ok(cls)
    }

    pub(crate) fn insert_method(
        &self,
        arg: &'static str,
        integer_add: Value,
    ) -> Result<(), String> {
        trace!("Inserting method {}", arg);
        let mut methods = self.0.lock().map_err(|e| format!("Class lock: {e}"))?;
        (*methods).methods.insert(arg, integer_add);
        Ok(())
    }

    pub(crate) fn instance_vars(&self) -> Result<Vec<&'static str>, String> {
        let mut classes = Vec::new();
        let mut current = Some(self.clone());
        while let Some(class) = current {
            let data = class.0.lock().map_err(|e| format!("Class lock: {e}"))?;
            classes.push(data.vars.clone());
            current = data.parent.clone();
        }
        Ok(classes.into_iter().rev().flatten().collect())
    }

    pub(crate) fn parent(&self) -> Result<Option<SmalltalkClass>, String> {
        Ok(self
            .0
            .lock()
            .map_err(|e| format!("Class lock: {e}"))?
            .parent
            .clone())
    }

    pub fn name(&self) -> Result<&'static str, String> {
        Ok(self.0.lock().map_err(|e| format!("Class lock: {e}"))?.name)
    }

    pub(crate) fn meta(&self) -> Result<SmalltalkObject, String> {
        self.0
            .lock()
            .map_err(|e| format!("Class lock: {e}"))?
            .meta
            .clone()
            .ok_or_else(|| "Class has no metaclass object".to_string())
    }

    pub(crate) fn set_meta(&self, meta_class: SmalltalkClass) -> Result<(), String> {
        let spec = self.meta()?.get_instance_var(2)?;
        let meta_obj = SmalltalkObject(Arc::new(Mutex::new(SmalltalkObjectData {
            class: meta_class,
            values: vec![Value::Undefined, Value::Undefined, spec],
        })));
        let mut data = self.0.lock().map_err(|e| format!("Class lock: {e}"))?;
        data.meta = Some(meta_obj);
        Ok(())
    }

    pub(crate) fn is_variable(&self) -> Result<bool, String> {
        match self.meta()?.get_instance_var(2)? {
            Value::Integer(spec) => Ok(spec & 16 != 0),
            _ => Err(format!(
                "Class {} has no instance specification",
                self.name()?
            )),
        }
    }

    pub(crate) fn set_instance_shape(&self, shape: &str) -> Result<(), String> {
        let shape_bits = match shape {
            "#inherit" => match self.parent()? {
                Some(parent) => match parent.meta()?.get_instance_var(2)? {
                    Value::Integer(spec) => spec & 31,
                    _ => {
                        return Err(format!(
                            "Class {} has no instance specification",
                            parent.name()?
                        ));
                    }
                },
                None => 0,
            },
            "nil" => 0,
            "#int8" => 16,
            "#byte" => 17,
            "#character" => 18,
            "#short" => 19,
            "#ushort" => 20,
            "#int" => 21,
            "#uint" => 22,
            "#float" => 23,
            "#int64" => 24,
            "#uint64" => 25,
            "#double" => 26,
            "#utf32" => 27,
            "#pointer" => 31,
            "#word" if std::mem::size_of::<std::os::raw::c_long>() == 4 => 22,
            "#word" => 25,
            _ => return Err(format!("Unsupported instance shape {shape}")),
        };
        let meta = self.meta()?;
        let Value::Integer(spec) = meta.get_instance_var(2)? else {
            return Err(format!(
                "Class {} has no instance specification",
                self.name()?
            ));
        };
        meta.set_instance_var(2, Value::Integer((spec & !31) | shape_bits))?;
        Ok(())
    }

    pub(crate) fn insert_variable(
        &self,
        ident: &'static str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.0
            .lock()
            .map_err(|e| format!("Class lock: {e}"))?
            .vars
            .push(ident);
        Ok(())
    }
}

pub fn get_meta_name(name: &'static str) -> &'static str {
    StringTable::get(&format!("@{}", name))
}
#[derive(Debug, Clone)]
pub struct SmalltalkClassData {
    pub parent: Option<SmalltalkClass>,
    pub meta: Option<SmalltalkObject>,
    pub methods: HashMap<&'static str, Value>,
    name: &'static str,
    vars: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(SmalltalkString),
    Symbol(&'static str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    Object(SmalltalkObject),
    Dictionary(Arc<Mutex<HashMap<&'static str, Value>>>),
    Array(Arc<Mutex<Vec<Value>>>),
    ByteArray(Arc<Mutex<Vec<u8>>>),
    Method(CompiledMethod),
    NativeMethod(
        fn(
            &Execution,
            Value,
            Vec<Value>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>,
    ),
    Class(SmalltalkClass),
    Execution(Execution),
    Undefined,
    Nil,
}
impl Value {
    pub(crate) fn same_identity(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a.same_identity(b),
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a.to_bits() == b.to_bits(),
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Character(a), Value::Character(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => Arc::ptr_eq(&a.0, &b.0),
            (Value::Dictionary(a), Value::Dictionary(b)) => Arc::ptr_eq(a, b),
            (Value::Array(a), Value::Array(b)) => Arc::ptr_eq(a, b),
            (Value::ByteArray(a), Value::ByteArray(b)) => Arc::ptr_eq(a, b),
            (Value::Method(a), Value::Method(b)) => a.same_identity(b),
            (Value::NativeMethod(a), Value::NativeMethod(b)) => std::ptr::fn_addr_eq(*a, *b),
            (Value::Class(a), Value::Class(b)) => Arc::ptr_eq(&a.0, &b.0),
            (Value::Execution(a), Value::Execution(b)) => a.same_activation(b),
            (Value::Nil, Value::Nil) | (Value::Undefined, Value::Undefined) => true,
            _ => false,
        }
    }

    pub(crate) fn as_dictionary(&self) -> Result<Arc<Mutex<HashMap<&'static str, Value>>>, String> {
        match self {
            Value::Dictionary(hash_map) => Ok(hash_map.clone()),
            _ => Err("Expected Dictionary value".into()),
        }
    }
}

impl From<SmalltalkClass> for Value {
    fn from(class: SmalltalkClass) -> Self {
        Value::Class(class)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl TryFrom<&str> for Value {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Value::String(SmalltalkString::new(value)?))
    }
}

impl TryFrom<String> for Value {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<CompiledMethod> for Value {
    fn from(value: CompiledMethod) -> Self {
        Value::Method(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{VirtualMachine, instruction::Instruction, register::Register};

    #[test]
    fn instance_write_waits_for_lock_contention() {
        let class = SmalltalkClass::new("TestClass", None, vec![]).unwrap();
        let object = SmalltalkObject::new(class);
        let guard = object.0.lock().unwrap();
        let other = object.clone();
        let (started, waiting) = std::sync::mpsc::channel();
        let (done, result) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            started.send(()).unwrap();
            done.send(other.set_instance_var(0, Value::Nil)).unwrap();
        });
        waiting.recv().unwrap();
        assert!(
            result
                .recv_timeout(std::time::Duration::from_millis(50))
                .is_err()
        );
        drop(guard);
        assert!(
            result
                .recv_timeout(std::time::Duration::from_secs(2))
                .unwrap()
                .is_ok()
        );
        thread.join().unwrap();
    }

    #[test]
    fn object_lock_failure_preserves_runtime_error_and_stack() {
        let vm = VirtualMachine::new();
        let Value::Class(class) = vm.get_class("Object").unwrap() else {
            panic!("expected class")
        };
        let object = SmalltalkObject::new(class);
        let method = CompiledMethod::new(
            vec![Instruction::CallMethod {
                dst: Register::Result,
                receiver: Register::Receiver,
                args: vec![],
                selector: "missing",
            }],
            vec![],
            0,
        );
        let execution = Execution::new(&vm, Value::Object(object.clone()), vec![], method, 0);
        let other = object.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison object lock");
            })
            .join()
            .is_err()
        );
        let error = execution.execute().unwrap_err().to_string();
        assert!(error.contains("Object lock"), "{error}");
        assert!(error.contains("<script> [ip 0]: send #missing"), "{error}");
        assert!(error.contains("receiver class unavailable"), "{error}");
    }

    #[test]
    fn class_lock_errors_propagate_during_lookup_and_class_creation() {
        let vm = VirtualMachine::new();
        let Value::Class(class) = vm.get_class("Object").unwrap() else {
            panic!("expected class")
        };
        let other = class.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison class lock");
            })
            .join()
            .is_err()
        );
        assert!(class.name().unwrap_err().contains("Class lock"));
        assert!(
            class
                .lookup_method("missing")
                .unwrap_err()
                .contains("Class lock")
        );
        assert!(
            vm.define_class("Child", Some("Object"), vec![])
                .unwrap_err()
                .contains("Class lock")
        );
    }

    #[test]
    fn compiling_subclass_method_reports_ancestor_lock_errors() {
        let vm = VirtualMachine::new();
        vm.define_class("Child", Some("Object"), vec!["local"])
            .unwrap();
        let Value::Class(parent) = vm.get_class("Object").unwrap() else {
            panic!("expected class")
        };
        let other = parent.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison class lock");
            })
            .join()
            .is_err()
        );
        let error = vm
            .define_method("Child", "local", vec![], "^local")
            .unwrap_err();
        assert!(error.contains("Class lock"), "{error}");
    }

    #[test]
    fn invalid_parent_spec_is_reported_without_registering_child() {
        let vm = VirtualMachine::new();
        let Value::Class(class) = vm.get_class("Object").unwrap() else {
            panic!("expected class")
        };
        class
            .meta()
            .unwrap()
            .set_instance_var(2, Value::Nil)
            .unwrap();
        let error = vm
            .define_class("Child", Some("Object"), vec![])
            .unwrap_err();
        assert!(
            error.contains("Class Object has no instance specification"),
            "{error}"
        );
        assert!(vm.get_class("Child").is_err());
    }

    #[test]
    fn poisoned_object_and_class_locks_are_reported() {
        let class = SmalltalkClass::new("TestClass", None, vec![]).unwrap();
        let object = SmalltalkObject::new(class.clone());
        let other = object.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison object lock");
            })
            .join()
            .is_err()
        );
        assert!(
            object
                .get_instance_var(0)
                .unwrap_err()
                .contains("Object lock")
        );
        let other = class.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.0.lock().unwrap();
                panic!("poison class lock");
            })
            .join()
            .is_err()
        );
        assert!(
            class
                .lookup_method("missing")
                .unwrap_err()
                .contains("Class lock")
        );
    }
}
