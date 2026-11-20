use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    debug::Debugger,
    memory::{SmalltalkClass, Value},
    vm::{block::CompiledMethod, error::RuntimeError, execution::Execution},
};

use self::primitives::{array_at, array_at_put, array_new, array_size};

mod bootstrap;
mod characters;
mod primitives;
mod source;
mod strings;
mod symbols;

pub mod block;
pub mod error;
pub mod execution;
pub mod instruction;
pub mod register;

#[derive(Clone)]
pub struct VirtualMachine {
    state: Arc<VirtualMachineState>,
}

struct VirtualMachineState {
    integer_class: Value,
    float_class: Value,
    meta_class: Value,
    execution_class: Value,
    st: Value,
    debugger: Mutex<Option<Arc<dyn Debugger>>>,
}

impl std::fmt::Debug for VirtualMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualMachine")
            .field("integer_class", &self.state.integer_class)
            .field("float_class", &self.state.float_class)
            .field("meta_class", &self.state.meta_class)
            .field("execution_class", &self.state.execution_class)
            .field("st", &self.state.st)
            .field("debugger", &"<debugger>")
            .finish()
    }
}

impl VirtualMachine {
    pub fn set_debugger(
        &self,
        debugger: Arc<dyn Debugger>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let previous = {
            let mut slot = self
                .state
                .debugger
                .lock()
                .map_err(|e| format!("Debugger lock: {e}"))?;
            slot.replace(debugger)
        };
        drop(previous);
        Ok(())
    }

    pub fn execute_method(
        &self,
        method: CompiledMethod,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let exec = Execution::new(self, Value::Undefined, vec![], method, 0);
        exec.execute()
    }

    pub fn execute_method_with_limits(
        &self,
        method: CompiledMethod,
        limits: execution::ExecutionLimits,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        Execution::new_with_limits(self, method, limits).execute()
    }

    pub fn define_class(
        &self,
        name: &'static str,
        parent: Option<&'static str>,
        instance_vars: Vec<&'static str>,
    ) -> Result<(), String> {
        let parent = match parent {
            Some(pname) => {
                let s = self.state.st.as_dictionary()?;
                match s
                    .lock()
                    .map_err(|e| format!("Global dictionary lock: {e}"))?
                    .get(pname)
                {
                    Some(Value::Class(cls)) => Some(cls.clone()),
                    _ => {
                        return Err(format!("Parent class '{}' not found", pname));
                    }
                }
            }
            None => None,
        };

        {
            let dict = self.state.st.as_dictionary()?;
            let mut dict = dict
                .lock()
                .map_err(|e| format!("Global dictionary lock: {e}"))?;
            if !dict.contains_key(name) {
                let class = SmalltalkClass::new(name, parent, instance_vars)?;
                if name == "Array" {
                    class
                        .meta()?
                        .class()?
                        .insert_method("basicNew:", Value::NativeMethod(array_new))?;
                    class.insert_method("at:", Value::NativeMethod(array_at))?;
                    class.insert_method("at:put:", Value::NativeMethod(array_at_put))?;
                    class.insert_method("size", Value::NativeMethod(array_size))?;
                    class.insert_method("basicSize", Value::NativeMethod(array_size))?;
                }
                if name == "Character" {
                    class
                        .meta()?
                        .class()?
                        .insert_method("basicNew", Value::NativeMethod(characters::basic_new))?;
                    class.insert_method("asString", Value::NativeMethod(characters::as_string))?;
                }
                if name == "String" {
                    class
                        .meta()?
                        .class()?
                        .insert_method("basicNew:", Value::NativeMethod(strings::new))?;
                    class.insert_method("size", Value::NativeMethod(strings::size))?;
                    class.insert_method("basicSize", Value::NativeMethod(strings::size))?;
                    class.insert_method("at:", Value::NativeMethod(strings::at))?;
                    class.insert_method("basicAt:", Value::NativeMethod(strings::at))?;
                    class.insert_method("at:put:", Value::NativeMethod(strings::at_put))?;
                    class.insert_method("basicAt:put:", Value::NativeMethod(strings::at_put))?;
                    class.insert_method("=", Value::NativeMethod(strings::equal))?;
                    class.insert_method(",", Value::NativeMethod(strings::concatenate))?;
                    class.insert_method("asString", Value::NativeMethod(strings::as_string))?;
                    class.insert_method("asSymbol", Value::NativeMethod(symbols::as_symbol))?;
                    class.insert_method("copy", Value::NativeMethod(strings::copy))?;
                }
                if name == "Symbol" {
                    class
                        .meta()?
                        .class()?
                        .insert_method("intern:", Value::NativeMethod(symbols::intern))?;
                    class.insert_method("size", Value::NativeMethod(symbols::size))?;
                    class.insert_method("basicSize", Value::NativeMethod(symbols::size))?;
                    class.insert_method("at:", Value::NativeMethod(symbols::at))?;
                    class.insert_method("basicAt:", Value::NativeMethod(symbols::at))?;
                    class.insert_method("at:put:", Value::NativeMethod(symbols::immutable))?;
                    class.insert_method("basicAt:put:", Value::NativeMethod(symbols::immutable))?;
                    class.insert_method("=", Value::NativeMethod(symbols::equal))?;
                    class.insert_method("asString", Value::NativeMethod(symbols::as_string))?;
                    class.insert_method("asSymbol", Value::NativeMethod(symbols::as_symbol))?;
                    class.insert_method("copy", Value::NativeMethod(symbols::as_symbol))?;
                }
                dict.insert(name, class.into());
            }
        }
        Ok(())
    }

    fn get_global(
        &self,
        var_name: &'static str,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let value = {
            let s = self.state.st.as_dictionary()?;
            s.lock()
                .map_err(|e| format!("Global dictionary lock: {e}"))?
                .get(var_name)
                .cloned()
                .ok_or(RuntimeError::GlobalNotFound(var_name))?
        };
        Ok(value)
    }

    pub fn get_class(
        &self,
        class_name: &str,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let dict = self.state.st.as_dictionary()?;
        let dict = dict
            .lock()
            .map_err(|e| format!("Global dictionary lock: {e}"))?;
        dict.get(class_name)
            .cloned()
            .ok_or_else(|| format!("Class '{class_name}' not found").into())
    }

    pub fn get_static_method(
        &self,
        class_name: &'static str,
        selector: &'static str,
    ) -> Result<CompiledMethod, Box<dyn std::error::Error + Send + Sync>> {
        let cls = self.get_class(class_name)?;
        if let Value::Class(cls) = cls {
            if let Some(Value::Method(method)) = cls.meta()?.class()?.lookup_method(selector)? {
                Ok(method.clone())
            } else {
                Err("method not found".into())
            }
        } else {
            Err("class not found".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::instruction::Instruction;
    use std::sync::{Weak, mpsc};

    struct NoopDebugger;

    impl Debugger for NoopDebugger {
        fn before_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }

        fn after_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }
    }

    struct ReentrantDebugger {
        state: Weak<VirtualMachineState>,
        result: mpsc::Sender<Result<(), String>>,
    }

    impl Debugger for ReentrantDebugger {
        fn before_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }

        fn after_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }
    }

    impl Drop for ReentrantDebugger {
        fn drop(&mut self) {
            let result = self
                .state
                .upgrade()
                .ok_or_else(|| "VM dropped".to_string())
                .and_then(|state| {
                    VirtualMachine { state }
                        .set_debugger(Arc::new(NoopDebugger))
                        .map_err(|error| error.to_string())
                });
            let _ = self.result.send(result);
        }
    }

    #[test]
    fn replacing_debugger_releases_lock_before_dropping_previous() {
        let vm = VirtualMachine::new();
        let (sender, receiver) = mpsc::channel();
        vm.set_debugger(Arc::new(ReentrantDebugger {
            state: Arc::downgrade(&vm.state),
            result: sender,
        }))
        .unwrap();
        vm.set_debugger(Arc::new(NoopDebugger)).unwrap();
        assert!(receiver.recv().unwrap().is_ok());
    }

    #[test]
    fn poisoned_debugger_lock_errors_are_returned() {
        let vm = VirtualMachine::new();
        let method = CompiledMethod::new(vec![Instruction::Nop], vec![], 0);
        let other = vm.clone();
        assert!(
            std::thread::spawn(move || {
                let _guard = other.state.debugger.lock().unwrap();
                panic!("poison debugger lock");
            })
            .join()
            .is_err()
        );
        for error in [
            vm.set_debugger(Arc::new(NoopDebugger)).unwrap_err(),
            vm.execute_method(method).unwrap_err(),
        ] {
            let error = error.to_string();
            assert!(
                error.contains("Debugger lock:") && error.contains("poisoned"),
                "{error}"
            );
        }
    }
}
