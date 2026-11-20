use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{
    compiler::compile_statements,
    debug::Debugger,
    memory::Value,
    parser::topdown::parse_eval,
    vm::{VirtualMachine, execution::ExecutionLimits},
};

/// Error reported while initializing or using an interpreter.
#[derive(Debug)]
pub enum InterpreterError {
    Bootstrap {
        path: PathBuf,
        source: Box<dyn Error + Send + Sync>,
    },
    Source(Box<dyn Error + Send + Sync>),
    Parse(Box<dyn Error + Send + Sync>),
    Compile(String),
    Runtime(Box<dyn Error + Send + Sync>),
    Busy(&'static str),
    StatePoisoned,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bootstrap { path, source } => {
                write!(f, "Bootstrap {}: {source}", path.display())
            }
            Self::Source(source) => write!(f, "Smalltalk source: {source}"),
            Self::Parse(source) => write!(f, "Parse: {source}"),
            Self::Compile(source) => write!(f, "Compile: {source}"),
            Self::Runtime(source) => write!(f, "Runtime: {source}"),
            Self::Busy(operation) => write!(f, "Interpreter busy: {operation}"),
            Self::StatePoisoned => write!(f, "Interpreter source state is poisoned"),
        }
    }
}

impl Error for InterpreterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bootstrap { source, .. }
            | Self::Source(source)
            | Self::Parse(source)
            | Self::Runtime(source) => Some(source.as_ref()),
            Self::Compile(_) | Self::Busy(_) | Self::StatePoisoned => None,
        }
    }
}

/// An independent Smalltalk interpreter. No tracing or terminal I/O is initialized.
pub struct Interpreter {
    vm: VirtualMachine,
    source_gate: AtomicUsize,
}

// The remaining states count evaluations; loading and poison are exclusive.
const SOURCE_LOADING: usize = 1 << (usize::BITS - 1);
const SOURCE_POISONED: usize = 1 << (usize::BITS - 2);

struct SourceLease<'a> {
    gate: &'a AtomicUsize,
    loading: bool,
}

impl Drop for SourceLease<'_> {
    fn drop(&mut self) {
        if self.loading {
            let state = if std::thread::panicking() {
                SOURCE_POISONED
            } else {
                0
            };
            self.gate.store(state, Ordering::Release);
        } else {
            self.gate.fetch_sub(1, Ordering::Release);
        }
    }
}

impl Interpreter {
    /// Load `boot.yaml` and the `smalltalk` directory under the given root.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, InterpreterError> {
        let root = root.as_ref();
        Self::with_boot_file(root.join("boot.yaml"), root.join("smalltalk"))
    }

    /// Load a boot manifest and its sources from explicitly supplied paths.
    pub fn with_boot_file(
        boot_file: impl AsRef<Path>,
        source_root: impl AsRef<Path>,
    ) -> Result<Self, InterpreterError> {
        let boot_file = boot_file.as_ref();
        let bootstrap_error =
            |path: &Path, source: Box<dyn Error + Send + Sync>| InterpreterError::Bootstrap {
                path: path.to_owned(),
                source,
            };
        let manifest = std::fs::read_to_string(boot_file)
            .map_err(|error| bootstrap_error(boot_file, Box::new(error)))?;
        let manifest: serde_yaml::Value = serde_yaml::from_str(&manifest)
            .map_err(|error| bootstrap_error(boot_file, Box::new(error)))?;
        let packages = manifest
            .as_mapping()
            .ok_or_else(|| bootstrap_error(boot_file, "Boot file must be a mapping".into()))?;

        let vm =
            VirtualMachine::try_new().map_err(|error| bootstrap_error(boot_file, error.into()))?;
        for (package, files) in packages {
            let package = package.as_str().ok_or_else(|| {
                bootstrap_error(boot_file, "Boot package must be a string".into())
            })?;
            let files = files.as_sequence().ok_or_else(|| {
                bootstrap_error(
                    boot_file,
                    "Boot package must contain a list of files".into(),
                )
            })?;
            for file in files {
                let file = file.as_str().ok_or_else(|| {
                    bootstrap_error(boot_file, "Boot file entry must be a string".into())
                })?;
                let path = source_root.as_ref().join(package).join(file);
                let source = std::fs::read_to_string(&path)
                    .map_err(|error| bootstrap_error(&path, Box::new(error)))?;
                vm.compile_src(&source)
                    .map_err(|error| bootstrap_error(&path, error))?;
            }
        }
        Ok(Self {
            vm,
            source_gate: AtomicUsize::new(0),
        })
    }

    fn evaluation_lease(&self) -> Result<SourceLease<'_>, InterpreterError> {
        let mut state = self.source_gate.load(Ordering::Acquire);
        loop {
            if state == SOURCE_LOADING {
                return Err(InterpreterError::Busy(
                    "cannot evaluate while source is loading",
                ));
            }
            if state == SOURCE_POISONED {
                return Err(InterpreterError::StatePoisoned);
            }
            if state >= SOURCE_POISONED - 1 {
                return Err(InterpreterError::Busy("too many concurrent evaluations"));
            }
            match self.source_gate.compare_exchange_weak(
                state,
                state + 1,
                Ordering::Acquire,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    return Ok(SourceLease {
                        gate: &self.source_gate,
                        loading: false,
                    });
                }
                Err(current) => state = current,
            }
        }
    }

    fn source_lease(&self) -> Result<SourceLease<'_>, InterpreterError> {
        match self.source_gate.compare_exchange(
            0,
            SOURCE_LOADING,
            Ordering::Acquire,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(SourceLease {
                gate: &self.source_gate,
                loading: true,
            }),
            Err(SOURCE_POISONED) => Err(InterpreterError::StatePoisoned),
            Err(_) => Err(InterpreterError::Busy(
                "cannot load source during evaluation or another load",
            )),
        }
    }

    /// Add Smalltalk definitions; returns `Busy` if another operation is active.
    pub fn load_source(&self, source: &str) -> Result<(), InterpreterError> {
        let _lease = self.source_lease()?;
        self.vm
            .compile_src(source)
            .map_err(InterpreterError::Source)
    }

    /// Evaluate statements concurrently; returns `Busy` while source is loading.
    pub fn eval(&self, source: &str) -> Result<Value, InterpreterError> {
        let _lease = self.evaluation_lease()?;
        let node = parse_eval(source).map_err(InterpreterError::Parse)?;
        let method = compile_statements(&node).map_err(InterpreterError::Compile)?;
        self.vm
            .execute_method(method)
            .map_err(InterpreterError::Runtime)
    }

    /// Evaluate statements with limits shared by their method and block calls.
    pub fn eval_with_limits(
        &self,
        source: &str,
        limits: ExecutionLimits,
    ) -> Result<Value, InterpreterError> {
        let _lease = self.evaluation_lease()?;
        let node = parse_eval(source).map_err(InterpreterError::Parse)?;
        let method = compile_statements(&node).map_err(InterpreterError::Compile)?;
        self.vm
            .execute_method_with_limits(method, limits)
            .map_err(InterpreterError::Runtime)
    }

    /// Install an optional debugger without transferring control of the host process.
    pub fn set_debugger(&self, debugger: Arc<dyn Debugger>) -> Result<(), InterpreterError> {
        self.vm
            .set_debugger(debugger)
            .map_err(InterpreterError::Runtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_loading_excludes_evaluation_and_poison_is_reported() {
        let interpreter = Interpreter::new(env!("CARGO_MANIFEST_DIR")).unwrap();
        let lease = interpreter.source_lease().unwrap();
        assert!(matches!(
            interpreter.eval("^1"),
            Err(InterpreterError::Busy(_))
        ));
        assert!(matches!(
            interpreter.load_source("Object subclass: BusyClass []"),
            Err(InterpreterError::Busy(_))
        ));
        drop(lease);
        assert!(matches!(interpreter.eval("^1"), Ok(Value::Integer(1))));

        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _lease = interpreter.source_lease().unwrap();
                panic!("simulate a panic while loading");
            }))
            .is_err()
        );
        assert!(matches!(
            interpreter.eval("^1"),
            Err(InterpreterError::StatePoisoned)
        ));
        assert!(matches!(
            interpreter.load_source("Object subclass: PoisonedClass []"),
            Err(InterpreterError::StatePoisoned)
        ));
    }
}
