use tracing::{instrument, trace};

use crate::{
    compiler::compile_method,
    memory::Value,
    parser::topdown::{SmalltalkNode, SmalltalkParser},
    try_get_project_root,
};

use super::VirtualMachine;

// Keep source-loading traces under their original target after splitting the VM module.
impl VirtualMachine {
    pub fn define_method(
        &self,
        class_name: &'static str,
        selector: &'static str,
        parameter_names: Vec<&'static str>,
        src: &'static str,
    ) -> Result<(), String> {
        trace!(
            target: "st::vm",
            "Defining method {}>>{} with params {:?} from source: {}",
            class_name, selector, parameter_names, src
        );

        let mut parser = SmalltalkParser::new(src)?;
        let (node, pragmas) = parser
            .parse_eval_with_pragmas()
            .map_err(|e| format!("Failed to parse method body: {}", e))?;
        if parser.current_token() != "EOF" {
            return Err(format!(
                "Unexpected token after method body: {}",
                parser.current_token()
            ));
        }

        self.define_method_from_node(class_name, false, selector, parameter_names, node, pragmas)
    }

    fn define_method_from_node(
        &self,
        class_name: &'static str,
        meta: bool,
        selector: &'static str,
        parameter_names: Vec<&'static str>,
        node: crate::parser::topdown::SmalltalkNode,
        pragmas: Vec<String>,
    ) -> Result<(), String> {
        let class_value = {
            let s = self.state.st.as_dictionary()?;
            s.lock()
                .map_err(|e| format!("Global dictionary lock: {e}"))?
                .get(class_name)
                .cloned()
                .ok_or_else(|| format!("Class '{}' not found", class_name))?
        };
        match class_value {
            Value::Class(mut cls) => {
                if meta {
                    cls = cls.meta()?.class()?;
                }
                trace!(target: "st::vm", "compiling for {}", class_name);
                let mut cm = compile_method(cls.instance_vars()?, parameter_names, &node)?;
                let mut primitives = pragmas
                    .iter()
                    .filter_map(|pragma| pragma.strip_prefix("primitive:").map(str::trim));
                let primitive = primitives.next().map(str::to_owned);
                if primitives.next().is_some() {
                    return Err(format!(
                        "Multiple primitive pragmas in {class_name}>>{selector}"
                    ));
                }
                cm.set_primitive(primitive);
                for (i, inst) in cm.instructions().iter().enumerate() {
                    trace!(target: "st::vm", "{:04}: {}", i, inst);
                }
                cls.insert_method(selector, cm.into())?;
            }
            _ => {
                return Err(format!("Class '{}' not found", class_name));
            }
        }

        Ok(())
    }

    pub fn compile_src(&self, src: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut parser = SmalltalkParser::new(&src)?;

        while parser.current_token() == "IDENTIFIER" {
            let class_def = parser.parse_class_definition()?;
            match class_def {
                SmalltalkNode::MessageInvoke { receiver, messages } => {
                    let SmalltalkNode::Identifier(parent) = *receiver else {
                        return Err("Class definition requires a class name".into());
                    };
                    let [
                        SmalltalkNode::Message {
                            selector,
                            arguments,
                        },
                    ] = messages.as_slice()
                    else {
                        return Err("Expected one class definition message".into());
                    };
                    match (*selector, arguments.as_slice()) {
                        ("extend", []) => {
                            self.get_class(parent)?;
                            parser.expect("[")?;
                            self.process_body(&mut parser, parent)?;
                        }
                        ("subclass:", [SmalltalkNode::Identifier(child)]) => {
                            trace!(target: "st::vm", "Defining class {} as subclass of {}", child, parent);
                            parser.expect("[")?;
                            let inst_vars = parser.parse_temporaries()?;
                            trace!(target: "st::vm", "Instance variables: {:?}", inst_vars);
                            let pragmas = parser.pragmas()?;
                            if parent == "nil" {
                                self.define_class(*child, None, inst_vars)?;
                            } else {
                                self.define_class(*child, Some(parent), inst_vars)?;
                            }
                            for pragma in pragmas {
                                if let Some(shape) = pragma.strip_prefix("shape:") {
                                    if let Value::Class(cls) = self.get_class(child)? {
                                        cls.set_instance_shape(shape.trim())?;
                                    }
                                }
                            }
                            self.process_body(&mut parser, *child)?;
                        }
                        _ => {
                            return Err(
                                format!("Invalid class definition message #{selector}").into()
                            );
                        }
                    }
                }
                x => {
                    return Err(format!("Expected class definition, found {x:?}").into());
                }
            }
        }
        if parser.current_token() != "EOF" {
            return Err(format!(
                "Unexpected token after class definition: {}",
                parser.current_token()
            )
            .into());
        }
        Ok(())
    }

    #[instrument(target = "st::vm", skip(self))]
    pub fn compile_file(
        &self,
        package: &str,
        file: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        trace!(target: "st::vm", "Compiling file: {}::{}", package, file);
        let file_path = try_get_project_root()?
            .join("smalltalk")
            .join(package)
            .join(file);
        let src = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {e}", file_path.display()))?;

        trace!(target: "st::vm", "content length: {}", src.len());
        match self.compile_src(&src) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error compiling file {}::{}: {}", package, file, e).into()),
        }
    }

    fn process_body(
        &self,
        parser: &mut SmalltalkParser,
        class_name: &'static str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            match parser.current_token() {
                "IDENTIFIER" if parser.current_value() == class_name => {
                    let (c, s) = parser.parse_class_reference()?;
                    let (sel, params) = parser.parse_method_pattern()?;
                    trace!(target: "st::vm", "Method pattern: {} {:?}", sel, params);
                    parser.expect("[")?;
                    let (body, pragmas) = parser.parse_eval_with_pragmas()?;
                    parser.expect("]")?;
                    self.define_method_from_node(class_name, s, sel, params, body, pragmas)?;
                    trace!(target: "st::vm", "Class reference: {} {}", c, s);
                }
                "IDENTIFIER" => {
                    let ident = parser.current_value();
                    trace!(target: "st::vm", "Identifier: {}", ident);
                    parser.advance();
                    if parser.current_token() == "ASSIGN" {
                        parser.advance();
                        let expr = parser.parse_primary()?;
                        trace!(target: "st::vm", "Assignment to {}: {:?}", ident, expr);
                        parser.expect(".")?;
                        self.define_class_variable(class_name, ident, expr)?;
                    } else {
                        trace!(target: "st::vm", "Method pattern: {} []", ident);
                        parser.expect("[")?;
                        let (body, pragmas) = parser.parse_eval_with_pragmas()?;
                        parser.expect("]")?;
                        self.define_method_from_node(
                            class_name,
                            false,
                            ident,
                            vec![],
                            body,
                            pragmas,
                        )?;
                    }
                }
                "KEYWORD" | "BINARY" => {
                    let (sel, params) = parser.parse_method_pattern()?;
                    trace!(target: "st::vm", "Method pattern: {} {:?}", sel, params);
                    parser.expect("[")?;
                    let (body, pragmas) = parser.parse_eval_with_pragmas()?;
                    parser.expect("]")?;
                    self.define_method_from_node(class_name, false, sel, params, body, pragmas)?;
                }
                "]" => {
                    trace!(target: "st::vm", "end of class definition");
                    parser.expect("]")?;
                    break;
                }
                x => {
                    return Err(format!("Unexpected token: {x}\n{}", parser.get_context(5)).into());
                }
            }
        }
        Ok(())
    }

    fn define_class_variable(
        &self,
        class_name: &'static str,
        ident: &'static str,
        _expr: SmalltalkNode,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Value::Class(cls) = self.get_class(class_name)? {
            cls.meta()?.class()?.insert_variable(ident)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing::{Event, Subscriber, span::Attributes};
    use tracing_subscriber::{Layer, layer::Context, prelude::*};

    use super::VirtualMachine;

    struct SourceTraceTargets(Arc<Mutex<Vec<(&'static str, &'static str)>>>);

    impl<S: Subscriber> Layer<S> for SourceTraceTargets {
        fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
            let metadata = event.metadata();
            if metadata.module_path() == Some("st::vm::source") {
                self.0
                    .lock()
                    .unwrap()
                    .push((metadata.name(), metadata.target()));
            }
        }

        fn on_new_span(&self, attrs: &Attributes<'_>, _: &tracing::span::Id, _: Context<'_, S>) {
            let metadata = attrs.metadata();
            if metadata.module_path() == Some("st::vm::source") {
                self.0
                    .lock()
                    .unwrap()
                    .push((metadata.name(), metadata.target()));
            }
        }
    }

    #[test]
    fn source_loading_keeps_original_trace_target() {
        let targets = Arc::new(Mutex::new(Vec::new()));
        let subscriber = tracing_subscriber::registry().with(SourceTraceTargets(targets.clone()));
        tracing::subscriber::with_default(subscriber, || {
            let vm = VirtualMachine::new();
            vm.define_method("Object", "answer", vec![], "^42").unwrap();
            vm.compile_src("Object extend [ answer [ ^42 ] ]").unwrap();
            vm.compile_file("kernel", "Object.st").unwrap();
        });

        let targets = targets.lock().unwrap();
        assert!(targets.iter().any(|(name, _)| *name == "compile_file"));
        assert!(targets.iter().any(|(name, _)| *name != "compile_file"));
        assert!(targets.iter().all(|(_, target)| *target == "st::vm"));
    }
}
