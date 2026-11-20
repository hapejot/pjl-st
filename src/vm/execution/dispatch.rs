use std::error::Error;

use tracing::trace;

use crate::{
    memory::{SmalltalkClass, Value},
    vm::{error::RuntimeError, register::Register},
};

use super::Execution;

// Preserve the original tracing target when moving dispatch into a submodule.
impl Execution {
    /// Execute method call (send message)
    /// dst: destination register
    /// receiver: receiver register
    /// args: argument registers
    /// selector: method selector
    /// Returns: Result<(), Box<dyn Error + Send + Sync>>
    /// Errors:
    /// - RuntimeError::RegisterNotAssigned if receiver or any argument register is not assigned
    /// - String error if method not found or other errors occur
    pub(super) fn execute_send_message(
        &self,
        dst: Register,
        receiver: Register,
        args: &Vec<Register>,
        selector: &'static str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let r = self
            .get(receiver)?
            .ok_or(RuntimeError::RegisterNotAssigned(receiver))?;
        let mut send_args = vec![];
        for a in args {
            send_args.push(self.get(*a)?.ok_or(RuntimeError::RegisterNotAssigned(*a))?);
        }
        trace!(target: "st::vm::execution",
            "args: {}",
            send_args
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(", ")
        );
        trace!(target: "st::vm::execution", "resolve class for {r}");
        let c = self.lookup_class(&r)?;
        trace!(target: "st::vm::execution", "lookup method {selector} in  {c}");
        let (owner, m) = self.lookup_method_in_class(c, selector)?;
        match m {
            Value::NativeMethod(func) => {
                let expected = if selector.contains(':') {
                    selector.bytes().filter(|&byte| byte == b':').count()
                } else if selector.starts_with(|c: char| !c.is_ascii_alphabetic() && c != '_') {
                    1
                } else {
                    0
                };
                Self::check_argument_count(
                    format!("{owner}>>{selector}"),
                    expected,
                    send_args.len(),
                )?;
                let result = func(self, r, send_args)?;
                self.set(dst, result)?;
                Ok(())
            }
            Value::Method(cm) => {
                Self::check_argument_count(
                    format!("{owner}>>{selector}"),
                    cm.parameter_count(),
                    send_args.len(),
                )?;
                if let Some(primitive) = cm.primitive() {
                    if let Some(result) = self.try_primitive(primitive, &r, &send_args)? {
                        self.set(dst, result)?;
                        return Ok(());
                    }
                    trace!(target: "st::vm::execution",
                        "Primitive {primitive} unavailable or failed; running Smalltalk fallback"
                    );
                }
                let (budget, call_depth, run_id) = self.child_context()?;
                let exec = Execution::new_with_budget(
                    &self.vm()?,
                    r,
                    send_args,
                    cm.clone(),
                    self.level()? + 1,
                    budget,
                    call_depth,
                    run_id,
                );
                exec.set_msg(format!("{owner}>>{selector}"))?;
                let result = exec.execute()?;
                self.set(dst, result)?;
                Ok(())
            }
            _ => Err(format!("{owner}>>{selector} is not executable").into()),
        }
    }

    fn try_primitive(
        &self,
        primitive: &str,
        receiver: &Value,
        args: &[Value],
    ) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        match (primitive, receiver, args) {
            ("VMpr_Behavior_basicNew", Value::Class(cls), []) => {
                if cls.is_variable()? {
                    Ok(None)
                } else {
                    Ok(Some(crate::vm::primitives::meta_class_new(
                        self,
                        receiver.clone(),
                        args.to_vec(),
                    )?))
                }
            }
            ("VMpr_Object_class", _, []) => Ok(Some(self.lookup_class(receiver)?)),
            ("VMpr_Object_identity", _, [other]) => {
                Ok(Some(Value::Boolean(receiver.same_identity(other))))
            }
            _ => Ok(None),
        }
    }

    fn lookup_class(&self, r: &Value) -> Result<Value, Box<dyn Error + Send + Sync>> {
        match r {
            Value::String(_) => self.vm()?.get_class("String"),
            Value::Symbol(symbol) => {
                crate::vm::symbols::text(symbol)?;
                self.vm()?.get_class("Symbol")
            }
            Value::Integer(_) => Ok(self.vm()?.state.integer_class.clone()),
            Value::Float(_) => Ok(self.vm()?.state.float_class.clone()),
            Value::Character(_) => self.vm()?.get_class("Character"),
            Value::Boolean(b) => match b {
                true => self.vm()?.get_class("True"),
                false => self.vm()?.get_class("False"),
            },
            Value::Object(smalltalk_object) => {
                let class_value = smalltalk_object.class()?;
                Ok(class_value.into())
            }
            Value::Dictionary(_) => {
                Err("Message dispatch for Dictionary is not implemented".into())
            }
            Value::Array(_) => self.vm()?.get_class("Array"),
            Value::Method(_) | Value::NativeMethod(_) => {
                Err("Message dispatch for method values is not implemented".into())
            }
            Value::Class(cls) => Ok(Value::Class(cls.meta()?.class()?)),
            Value::Execution(_) => Ok(self.vm()?.state.execution_class.clone()),
            Value::Undefined => Err("Cannot send a message to an undefined value".into()),
            Value::Nil => self.vm()?.get_class("Nil"),
        }
    }

    fn lookup_method_in_class(
        &self,
        class: Value,
        selector: &str,
    ) -> Result<(&'static str, Value), String> {
        let Value::Class(class) = class else {
            return Err("Invalid class value".into());
        };
        let receiver_class = class.name()?;
        let mut current = Some(class);
        while let Some(class) = current {
            if let Some(method) = class.lookup_method(selector)? {
                return Ok((class.name()?, method));
            }
            current = class.parent()?;
        }
        Err(format!(
            "{receiver_class} does not understand #{selector} (not found in its class hierarchy)"
        ))
    }

    pub fn lookup_method(c: Value, selector: &str) -> Result<Value, String> {
        match c {
            Value::Class(cls) => Self::lookup_method_int(cls, selector),

            _ => Err("Invalid class value".into()),
        }
    }

    fn lookup_method_int(cls: SmalltalkClass, selector: &str) -> Result<Value, String> {
        let name = cls.name()?;
        trace!(target: "st::vm::execution", "Looking up method '{}' in class '{}'", selector, name);
        if let Some(m) = cls.lookup_method(selector)? {
            Ok(m.clone())
        } else {
            for x in cls.method_names()? {
                trace!(target: "st::vm::execution", "Available method in {}: {}", name, x);
            }
            if let Some(parent) = cls.parent()? {
                Self::lookup_method_int(parent, selector)
            } else {
                Err(format!(
                    "Method '{}' not found in class {} or its parents",
                    selector, name
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing::{Event, Subscriber};
    use tracing_subscriber::{Layer, layer::Context, prelude::*};

    use crate::{compiler::compile_statements, parser::topdown::parse_eval, vm::VirtualMachine};

    use super::Execution;

    struct ExecutionTraceTargets(Arc<Mutex<Vec<(&'static str, &'static str)>>>);

    impl<S: Subscriber> Layer<S> for ExecutionTraceTargets {
        fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
            let metadata = event.metadata();
            if let Some(
                module @ ("st::vm::execution::dispatch" | "st::vm::execution::instructions"),
            ) = metadata.module_path()
            {
                self.0.lock().unwrap().push((module, metadata.target()));
            }
        }
    }

    #[test]
    fn execution_modules_keep_original_trace_target() {
        let vm = VirtualMachine::new();
        vm.define_method(
            "Integer",
            "missingPrimitive",
            vec![],
            "<primitive: VMpr_Missing> ^42",
        )
        .unwrap();
        let method = compile_statements(&parse_eval("^1 missingPrimitive").unwrap()).unwrap();

        let targets = Arc::new(Mutex::new(Vec::new()));
        let subscriber =
            tracing_subscriber::registry().with(ExecutionTraceTargets(targets.clone()));
        tracing::subscriber::with_default(subscriber, || {
            assert_eq!(format!("{}", vm.execute_method(method).unwrap()), "42");
            assert!(Execution::lookup_method(vm.get_class("Integer").unwrap(), "missing").is_err());
        });

        let targets = targets.lock().unwrap();
        assert!(
            targets
                .iter()
                .filter(|(module, _)| *module == "st::vm::execution::dispatch")
                .count()
                >= 5,
            "expected dispatch and lookup traces"
        );
        assert!(
            targets
                .iter()
                .filter(|(module, _)| *module == "st::vm::execution::instructions")
                .count()
                >= 2,
            "expected instruction traces"
        );
        assert!(
            targets
                .iter()
                .all(|(_, target)| *target == "st::vm::execution")
        );
    }
}
