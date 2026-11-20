use std::{
    error::Error,
    path::Path,
    sync::{
        Arc, Barrier, Mutex, Weak,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    time::Duration,
};

use st::{
    Interpreter, InterpreterError,
    debug::Debugger,
    memory::Value,
    vm::execution::{Execution, ExecutionLimits},
};

fn interpreter() -> Interpreter {
    Interpreter::new(env!("CARGO_MANIFEST_DIR")).unwrap()
}

#[test]
fn interpreter_errors_cross_threads_with_original_causes_and_stacks() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<InterpreterError>();

    let interpreter = Arc::new(interpreter());
    interpreter
        .load_source("Object subclass: ThreadError [ run [ ^[1 + 2] value ] ]")
        .unwrap();
    let other = interpreter.clone();
    let error = std::thread::spawn(move || {
        other.eval_with_limits(
            "^(ThreadError basicNew) run",
            ExecutionLimits {
                max_instructions: None,
                max_call_depth: Some(2),
            },
        )
    })
    .join()
    .unwrap()
    .unwrap_err();
    let runtime = error.source().unwrap();
    assert!(
        runtime.to_string().contains("ThreadError>>run"),
        "{runtime}"
    );
    assert!(runtime.to_string().contains("Smalltalk stack"), "{runtime}");
    assert!(matches!(
        runtime
            .source()
            .and_then(|cause| cause.downcast_ref::<st::vm::error::RuntimeError>()),
        Some(st::vm::error::RuntimeError::CallDepthLimitExceeded { max: 2 })
    ));

    let error = std::thread::spawn(move || interpreter.eval("^"))
        .join()
        .unwrap()
        .unwrap_err();
    assert!(matches!(error, InterpreterError::Parse(_)));
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<st::parser::topdown::ParseError>()
            .is_some()
    );

    let error = std::thread::spawn(|| {
        Interpreter::with_boot_file(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("missing-boot-file-for-test"),
            env!("CARGO_MANIFEST_DIR"),
        )
        .err()
        .unwrap()
    })
    .join()
    .unwrap();
    assert!(matches!(error, InterpreterError::Bootstrap { .. }));
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .is_some()
    );
}

#[test]
fn debugger_error_type_survives_thread_and_execution_wrapping() {
    #[derive(Debug)]
    struct DebuggerFailure;
    impl std::fmt::Display for DebuggerFailure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "debugger failure")
        }
    }
    impl Error for DebuggerFailure {}
    struct FailingDebugger;
    impl Debugger for FailingDebugger {
        fn before_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Err(Box::new(DebuggerFailure))
        }
        fn after_execute(&self, _: Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }
    }

    let interpreter = Arc::new(interpreter());
    interpreter.set_debugger(Arc::new(FailingDebugger)).unwrap();
    let error = std::thread::spawn(move || interpreter.eval("^1"))
        .join()
        .unwrap()
        .unwrap_err();
    let runtime = error.source().unwrap();
    assert!(runtime.to_string().contains("Smalltalk stack"), "{runtime}");
    assert!(runtime.source().unwrap().is::<DebuggerFailure>());
}

#[test]
fn shared_interpreter_executes_concurrently_with_independent_limits() {
    const THREADS: usize = 4;
    struct Rendezvous {
        started: AtomicUsize,
        barrier: Barrier,
    }
    impl Debugger for Rendezvous {
        fn before_execute(
            &self,
            execution: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            if execution.msg()?.is_empty()
                && execution.ip()? == 0
                && self.started.fetch_add(1, Ordering::SeqCst) < THREADS
            {
                self.barrier.wait();
            }
            Ok(())
        }
        fn after_execute(
            &self,
            _: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Interpreter>();
    let interpreter = Arc::new(interpreter());
    interpreter
        .load_source(
            "Object subclass: ParallelExample [
                | field |
                array [ ^#(0) ]
                write: value [ ^self array at: 1 put: value ]
                object [ ^#(nil) ]
                field: value [ field := value. ^field ]
                sharedBlock [ ^#(nil) ]
                installBlock [ | n | n := 0. self sharedBlock at: 1 put: [ n := n + 1. n ] ]
                tick [ ^(self sharedBlock at: 1) value ]
            ]",
        )
        .unwrap();
    interpreter
        .eval("| x | x := ParallelExample basicNew. x object at: 1 put: x. x installBlock")
        .unwrap();
    interpreter
        .set_debugger(Arc::new(Rendezvous {
            started: AtomicUsize::new(0),
            barrier: Barrier::new(THREADS),
        }))
        .unwrap();

    let threads: Vec<_> = (0..THREADS)
        .map(|index| {
            let interpreter = interpreter.clone();
            std::thread::spawn(move || {
                let source = "^(ParallelExample basicNew) write: 42";
                let first = interpreter.eval_with_limits(
                    source,
                    ExecutionLimits {
                        max_instructions: Some(if index == 0 { 1 } else { 100 }),
                        max_call_depth: Some(5),
                    },
                );
                if index == 0 {
                    let error = first.unwrap_err().to_string();
                    assert!(error.contains("Instruction limit exceeded"), "{error}");
                } else {
                    assert!(matches!(first, Ok(Value::Integer(42))), "{first:?}");
                }
                for _ in 0..30 {
                    let value = interpreter.eval(source)?;
                    assert!(matches!(value, Value::Integer(42)), "{value:?}");
                    let value =
                        interpreter.eval("^((ParallelExample basicNew) object at: 1) field: 42")?;
                    assert!(matches!(value, Value::Integer(42)), "{value:?}");
                    let value = interpreter.eval("^(ParallelExample basicNew) tick")?;
                    assert!(
                        matches!(value, Value::Integer(value) if value > 0),
                        "{value:?}"
                    );
                }
                Ok::<_, InterpreterError>(())
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap().unwrap();
    }
}

#[test]
fn concurrent_load_is_rejected_and_nonlocal_return_cannot_cross_runs() {
    struct PauseDebugger {
        started: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    impl Debugger for PauseDebugger {
        fn before_execute(
            &self,
            execution: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            if execution.msg()? == "ParallelReturn>>pause" && execution.ip()? == 0 {
                self.started.send(())?;
                self.release
                    .lock()
                    .map_err(|error| error.to_string())?
                    .recv()?;
            }
            Ok(())
        }
        fn after_execute(
            &self,
            _: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    let interpreter = Arc::new(interpreter());
    interpreter
        .load_source(
            "Object subclass: ParallelReturn [
                registry [ ^#(nil) ]
                pause [ ^1 ]
                start [ self registry at: 1 put: [ ^33 ]. self pause. ^42 ]
            ]",
        )
        .unwrap();
    let (started, waiting) = mpsc::channel();
    let (release, resume) = mpsc::channel();
    interpreter
        .set_debugger(Arc::new(PauseDebugger {
            started,
            release: Mutex::new(resume),
        }))
        .unwrap();
    let other = interpreter.clone();
    let thread = std::thread::spawn(move || other.eval("^(ParallelReturn basicNew) start"));
    waiting.recv_timeout(Duration::from_secs(5)).unwrap();
    let load = interpreter.load_source("Object subclass: LateClass []");
    let block = interpreter.eval("^((ParallelReturn basicNew) registry at: 1) value");
    release.send(()).unwrap();
    let first = thread.join().unwrap();

    assert!(matches!(load, Err(InterpreterError::Busy(_))));
    let error = block.unwrap_err().to_string();
    assert!(error.contains("Block cannot return"), "{error}");
    assert!(error.contains("different execution"), "{error}");
    assert!(error.contains("Smalltalk stack"), "{error}");
    assert!(matches!(first, Ok(Value::Integer(42))), "{first:?}");
    interpreter
        .load_source("Object subclass: LateClass []")
        .unwrap();
}

#[test]
fn debugger_can_reenter_interpreter_without_holding_vm_locks() {
    struct ReentrantDebugger {
        interpreter: Weak<Interpreter>,
        result: mpsc::Sender<(bool, bool)>,
    }
    impl Debugger for ReentrantDebugger {
        fn before_execute(
            &self,
            _: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let interpreter = self.interpreter.upgrade().ok_or("interpreter dropped")?;
            interpreter.set_debugger(Arc::new(TestDebugger))?;
            let load_denied = matches!(
                interpreter.load_source("Object subclass: ReentrantClass []"),
                Err(InterpreterError::Busy(_))
            );
            let nested_ok = matches!(interpreter.eval("^1"), Ok(Value::Integer(1)));
            self.result.send((load_denied, nested_ok))?;
            Ok(())
        }
        fn after_execute(
            &self,
            _: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    let interpreter = Arc::new(interpreter());
    let (sender, receiver) = mpsc::channel();
    interpreter
        .set_debugger(Arc::new(ReentrantDebugger {
            interpreter: Arc::downgrade(&interpreter),
            result: sender,
        }))
        .unwrap();
    assert!(matches!(interpreter.eval("^2"), Ok(Value::Integer(2))));
    assert_eq!(
        receiver.recv_timeout(Duration::from_secs(2)).unwrap(),
        (true, true)
    );
}

#[test]
fn limits_apply_across_methods_blocks_and_escaped_closures() {
    struct CountInstructions {
        count: AtomicUsize,
        first_block: AtomicUsize,
    }

    impl Debugger for CountInstructions {
        fn before_execute(
            &self,
            execution: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let count = self.count.fetch_add(1, Ordering::Relaxed) + 1;
            if execution.msg()?.starts_with("Block #") {
                let _ = self.first_block.compare_exchange(
                    0,
                    count,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                );
            }
            Ok(())
        }

        fn after_execute(
            &self,
            _: Execution,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    let interpreter = interpreter();
    interpreter
        .load_source(
            "Object subclass: LimitExample [
                recurse [ ^self recurse ]
                callBlock [ ^[1 + 2] value ]
                registry [ ^#(nil) ]
                makeBlock [ ^[1 + 2] ]
            ]",
        )
        .unwrap();
    let limits = |instructions, depth| ExecutionLimits {
        max_instructions: instructions,
        max_call_depth: depth,
    };
    for (source, depth, succeeds) in [
        ("^1 + 2", 0, false),
        ("^1 + 2", 1, true),
        ("^(LimitExample basicNew) callBlock", 1, false),
        ("^(LimitExample basicNew) callBlock", 2, false),
        ("^(LimitExample basicNew) callBlock", 3, true),
        ("^(LimitExample basicNew) recurse", 8, false),
    ] {
        let result = interpreter.eval_with_limits(source, limits(Some(1000), Some(depth)));
        if succeeds {
            assert!(
                matches!(result, Ok(Value::Integer(3))),
                "{source}: {result:?}"
            );
        } else {
            let error = result.unwrap_err().to_string();
            assert!(
                error.contains("Call depth limit exceeded"),
                "{source}: {error}"
            );
            assert!(error.contains("Smalltalk stack"), "{source}: {error}");
        }
    }
    let error = interpreter
        .eval_with_limits("^(LimitExample basicNew) recurse", limits(Some(15), None))
        .unwrap_err()
        .to_string();
    assert!(error.contains("Instruction limit exceeded"), "{error}");
    assert!(error.contains("LimitExample>>recurse"), "{error}");

    interpreter
        .eval("| h | h := LimitExample basicNew. h registry at: 1 put: h makeBlock")
        .unwrap();
    let source = "^((LimitExample basicNew) registry at: 1) value";
    let counter = Arc::new(CountInstructions {
        count: AtomicUsize::new(0),
        first_block: AtomicUsize::new(0),
    });
    interpreter.set_debugger(counter.clone()).unwrap();
    assert!(matches!(interpreter.eval(source), Ok(Value::Integer(3))));
    let before_block = counter
        .first_block
        .load(Ordering::Relaxed)
        .checked_sub(1)
        .unwrap();
    let error = interpreter
        .eval_with_limits(
            source,
            limits(Some(u64::try_from(before_block).unwrap()), None),
        )
        .unwrap_err();
    assert!(
        error.to_string().contains("Instruction limit exceeded"),
        "{error}"
    );
    assert!(error.to_string().contains("Block #"), "{error}");
    let error = interpreter
        .eval_with_limits(source, limits(None, Some(1)))
        .unwrap_err();
    assert!(
        error.to_string().contains("Call depth limit exceeded"),
        "{error}"
    );
    assert!(matches!(
        interpreter.eval_with_limits(source, limits(Some(100), Some(3))),
        Ok(Value::Integer(3))
    ));
}

#[test]
fn evaluates_scripts_and_keeps_definitions_per_interpreter() {
    let first = interpreter();
    let second = interpreter();
    first
        .load_source("Object subclass: HostExample [ answer [ ^42 ] ]")
        .unwrap();

    assert!(matches!(
        first.eval("^(HostExample basicNew) answer"),
        Ok(Value::Integer(42))
    ));
    assert!(matches!(first.eval("^1+2"), Ok(Value::Integer(3))));
    assert!(matches!(first.eval("^1+2*3-4/5"), Ok(Value::Integer(1))));
    assert!(matches!(first.eval("^1 + -4"), Ok(Value::Integer(-3))));
    assert_eq!(format!("{}", first.eval("^#(1 -2)").unwrap()), "#(1 -2)");
    assert!(matches!(
        second.eval("^HostExample"),
        Err(InterpreterError::Runtime(_))
    ));
}

#[test]
fn inherited_instance_variables_have_distinct_slots_and_are_accessible() {
    let interpreter = interpreter();
    interpreter
        .load_source(
            "Object subclass: SlotRoot [
                | first |
                first [ ^first ]
                first: value [ first := value ]
            ]
            SlotRoot subclass: SlotMiddle [
                | second |
                second [ ^second ]
                second: value [ second := value ]
            ]
            SlotMiddle subclass: SlotLeaf [
                | third |
                third [ ^third ]
                third: value [ third := value ]
                replaceFirst: value [ [ first := value ] value ]
                sum [ ^first + second + third ]
            ]",
        )
        .unwrap();

    let result = interpreter
        .eval(
            "| x y |
            x := SlotLeaf basicNew.
            y := SlotLeaf basicNew.
            x first: 1. x second: 2. x third: 3.
            y first: 9. y second: 8. y third: 7.
            x replaceFirst: 4.
            ^(x first * 100) + (x second * 10) + x third",
        )
        .unwrap();
    assert!(matches!(result, Value::Integer(423)));
    assert!(matches!(
        interpreter.eval(
            "| x | x := SlotLeaf basicNew.
            x first: 1. x second: 2. x third: 3. ^x sum"
        ),
        Ok(Value::Integer(6))
    ));
}

#[test]
fn duplicate_instance_variables_are_rejected() {
    let interpreter = interpreter();
    interpreter
        .load_source("Object subclass: SlotParent [ | first | ]")
        .unwrap();
    for (source, name) in [
        ("SlotParent subclass: SlotChild [ | first | ]", "SlotChild"),
        (
            "Object subclass: SlotDuplicate [ | value value | ]",
            "SlotDuplicate",
        ),
    ] {
        let error = interpreter.load_source(source).unwrap_err();
        assert!(error.to_string().contains("Instance variable"), "{error}");
        assert!(error.to_string().contains("already defined"), "{error}");
        assert!(matches!(
            interpreter.eval(&format!("^{name}")),
            Err(InterpreterError::Runtime(_))
        ));
    }
}

#[test]
fn dynamic_arrays_evaluate_elements_in_order_and_capture_values() {
    let interpreter = interpreter();
    for (source, expected) in [
        ("^{}", "#()"),
        ("^{1 + 2. 4 * 5}", "#(3 20)"),
        ("^{{1 + 2}. {4}}", "#(#(3) #(4))"),
        ("| x | x := 10. ^{x. x := x + 1. x}", "#(10 11 11)"),
        ("| x | x := 8. x := {x}. ^x", "#(8)"),
        ("^{1. 2.}", "#(1 2)"),
    ] {
        let value = interpreter.eval(source).unwrap();
        assert_eq!(format!("{value}"), expected, "{source}");
    }
    for source in ["^{1 2}", "^{1..2}", "^{1"] {
        assert!(
            matches!(interpreter.eval(source), Err(InterpreterError::Parse(_))),
            "{source}"
        );
    }
    let error = interpreter.eval("^{1. 1 / 0. 3}").unwrap_err().to_string();
    assert!(
        error.contains("division by zero") && error.contains("Smalltalk stack"),
        "{error}"
    );
}

#[test]
fn dynamic_arrays_are_fresh_on_each_method_and_block_invocation() {
    let interpreter = interpreter();
    interpreter
        .load_source("Object subclass: ArrayMaker [ make [ ^{1} ] ]")
        .unwrap();
    for source in [
        "| maker first second |
         maker := ArrayMaker basicNew. first := maker make. second := maker make.
         first at: 1 put: 9. ^second at: 1",
        "| block first second |
         block := [{1}]. first := block value. second := block value.
         first at: 1 put: 9. ^second at: 1",
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::Integer(1))),
            "{source}"
        );
    }
}

#[test]
fn literal_arrays_support_symbols_pseudovariables_and_nested_arrays() {
    let interpreter = interpreter();
    let Value::Array(array) = interpreter
        .eval("^#(1 -2 1.5 'text' $x #named bare nil true false (4 #(5)) + at:put: foo: bar: 'don''t' #[1 2])")
        .unwrap()
    else {
        panic!("expected array")
    };
    let array = array.lock().unwrap();
    assert_eq!(array.len(), 17);
    assert!(matches!(array[0], Value::Integer(1)));
    assert!(matches!(array[1], Value::Integer(-2)));
    assert!(matches!(array[2], Value::Float(value) if value == 1.5));
    assert!(matches!(&array[3], Value::String(value) if value.to_rust_string().unwrap() == "text"));
    assert!(matches!(array[4], Value::Character('x')));
    assert!(matches!(array[5], Value::Symbol("#named")));
    assert!(matches!(array[6], Value::Symbol("#bare")));
    assert!(matches!(array[7], Value::Nil));
    assert!(matches!(array[8], Value::Boolean(true)));
    assert!(matches!(array[9], Value::Boolean(false)));
    let Value::Array(nested) = &array[10] else {
        panic!("expected nested array")
    };
    let nested = nested.lock().unwrap();
    assert!(matches!(nested[0], Value::Integer(4)));
    let Value::Array(inner) = &nested[1] else {
        panic!("expected inner array")
    };
    assert!(matches!(inner.lock().unwrap()[0], Value::Integer(5)));
    assert!(matches!(array[11], Value::Symbol("#+")));
    assert!(matches!(array[12], Value::Symbol("#at:put:")));
    assert!(matches!(array[13], Value::Symbol("#foo:")));
    assert!(matches!(array[14], Value::Symbol("#bar:")));
    assert!(
        matches!(&array[15], Value::String(value) if value.to_rust_string().unwrap() == "don't")
    );
    let Value::Array(bytes) = &array[16] else {
        panic!("expected array")
    };
    assert!(matches!(bytes.lock().unwrap()[1], Value::Integer(2)));
    assert!(
        matches!(interpreter.eval("^'don''t'"), Ok(Value::String(value)) if value.to_rust_string().unwrap() == "don't")
    );
    assert!(matches!(
        interpreter.eval("^#(missing)"),
        Ok(Value::Array(_))
    ));
    assert!(matches!(
        interpreter.eval("^#[true]"),
        Err(InterpreterError::Runtime(_))
    ));
}

#[test]
fn reports_parse_compile_runtime_and_bootstrap_errors() {
    let interpreter = interpreter();
    for source in ["^", "^1 )", "^9223372036854775808", "^1 foo;"] {
        assert!(
            matches!(interpreter.eval(source), Err(InterpreterError::Parse(_))),
            "{source}"
        );
    }
    assert!(matches!(
        interpreter.eval("| x x | ^1"),
        Err(InterpreterError::Compile(_))
    ));
    let error = interpreter.eval("^#([1])").unwrap_err();
    assert!(matches!(error, InterpreterError::Runtime(_)));
    assert!(error.to_string().contains("Non-literal array elements"));
    for (source, message) in [
        ("^1 / 0", "division by zero"),
        ("^9223372036854775807 + 1", "overflow"),
        ("^MissingClass", "MissingClass"),
    ] {
        let error = interpreter.eval(source).unwrap_err();
        assert!(matches!(error, InterpreterError::Runtime(_)));
        assert!(error.to_string().contains(message), "{error}");
        assert!(error.to_string().contains("Smalltalk stack"), "{error}");
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let error = match Interpreter::with_boot_file(root.join("boot.yaml"), root.join("missing")) {
        Ok(_) => panic!("missing bootstrap source should fail"),
        Err(error) => error,
    };
    assert!(matches!(error, InterpreterError::Bootstrap { .. }));
    assert!(error.to_string().contains("Object.st"), "{error}");
}

#[test]
fn rejects_invalid_class_definitions() {
    let interpreter = interpreter();
    for source in [
        "Object nonsense: BadClass []",
        "Object subclass: 1 []",
        "MissingClass extend []",
        "Object extend",
    ] {
        let error = interpreter.load_source(source).unwrap_err();
        assert!(
            matches!(error, InterpreterError::Source(_)),
            "{source}: {error}"
        );
    }
    assert!(matches!(
        interpreter.eval("^BadClass"),
        Err(InterpreterError::Runtime(_))
    ));
}

#[test]
fn cyclic_values_can_be_formatted_and_traced() {
    let interpreter = interpreter();
    interpreter
        .load_source("Object subclass: Link [ | link | link: value [ link := value ] ]")
        .unwrap();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(std::io::sink)
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        let object = interpreter
            .eval("| x | x := Link basicNew. x link: x. ^x")
            .unwrap();
        assert_eq!(
            format!("{object}"),
            "<Object Class Link [<Object Class Link [...]>]>"
        );

        let mixed = interpreter
            .eval("| x a | x := Link basicNew. a := Array new: 1. a at: 1 put: x. x link: a. ^x")
            .unwrap();
        assert_eq!(
            format!("{mixed}"),
            "<Object Class Link [#(<Object Class Link [...]>)]>"
        );
    });
}

struct TestDebugger;

impl Debugger for TestDebugger {
    fn before_execute(&self, _: Execution) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn after_execute(&self, _: Execution) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[test]
fn escaped_block_does_not_keep_interpreter_or_debugger_alive() {
    let interpreter = interpreter();
    let debugger = Arc::new(TestDebugger);
    let weak_debugger = Arc::downgrade(&debugger);
    interpreter.set_debugger(debugger.clone()).unwrap();

    let Value::Execution(block) = interpreter.eval("^[]").unwrap() else {
        panic!("expected block");
    };
    drop(debugger);
    drop(interpreter);
    assert!(weak_debugger.upgrade().is_none());
    assert!(
        block
            .execute()
            .unwrap_err()
            .to_string()
            .contains("Virtual machine has been dropped")
    );
}

#[test]
fn characters_support_basic_messages_and_string_conversion() {
    let interpreter = interpreter();
    assert!(matches!(
        interpreter.eval("^$ä class"),
        Ok(Value::Class(class)) if class.name().unwrap() == "Character"
    ));
    for (source, expected) in [
        ("^$a isCharacter", true),
        ("^1 isCharacter", false),
        ("^$a = $a", true),
        ("^$a = $b", false),
        ("^$a = 'a'", false),
        ("^$a == $a", true),
        ("^$a == $b", false),
        ("^('äΩ' at: 2) = $Ω", true),
        ("^(#named at: 1) == $n", true),
        ("^(#($x) at: 1) isCharacter", true),
        ("^$😀 = $😀", true),
        ("^$a asString == $a asString", false),
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::Boolean(actual)) if actual == expected),
            "{source}"
        );
    }
    for (source, expected) in [
        ("^$ä asString", "ä"),
        ("^('äΩ' at: 2) asString", "Ω"),
        ("^((String new: 1) at: 1) asString", "\0"),
        (
            "| text | text := $a asString. text at: 1 put: $z. ^$a asString",
            "a",
        ),
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::String(value)) if value.to_rust_string().unwrap() == expected),
            "{source}"
        );
    }
    for (source, expected) in [
        ("^$😀 asString", "outside the BMP"),
        ("^Character basicNew", "cannot be created with basicNew"),
    ] {
        let error = interpreter.eval(source).unwrap_err().to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
}

#[test]
fn bmp_strings_support_indexing_mutation_and_basic_messages() {
    let interpreter = interpreter();
    assert!(matches!(
        interpreter.eval("^'äΩ' size"),
        Ok(Value::Integer(2))
    ));
    assert!(matches!(
        interpreter.eval("^'äΩ' at: 2"),
        Ok(Value::Character('Ω'))
    ));
    assert!(matches!(
        interpreter.eval("^String new: 2"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "\0\0"
    ));
    assert!(matches!(
        interpreter.eval("| text alias | text := 'abc'. alias := text. alias at: 2 put: $Ω. ^text"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "aΩc"
    ));
    assert!(matches!(
        interpreter.eval("| text | text := String new: 2. text at: 1 put: $ä. text at: 2 put: $Ω. ^text"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "äΩ"
    ));
    assert!(matches!(
        interpreter.eval("^'ab' , 'ä'"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "abä"
    ));
    assert!(matches!(
        interpreter.eval("^'ab' = 'ab'"),
        Ok(Value::Boolean(true))
    ));
    assert!(matches!(
        interpreter.eval("^'ab' = #ab"),
        Ok(Value::Boolean(false))
    ));
    assert!(matches!(
        interpreter.eval("^'ab' isString"),
        Ok(Value::Boolean(true))
    ));
    assert!(matches!(
        interpreter.eval("^'ab' class"),
        Ok(Value::Class(class)) if class.name().unwrap() == "String"
    ));
    assert!(matches!(
        interpreter.eval("| text | text := 'a'. ^text == text"),
        Ok(Value::Boolean(true))
    ));
    assert!(matches!(
        interpreter.eval("^'a' == 'a'"),
        Ok(Value::Boolean(false))
    ));
    assert!(matches!(
        interpreter.eval("| text result | text := 'a'. result := text , text. text at: 1 put: $z. ^result"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "aa"
    ));
    assert!(matches!(
        interpreter.eval("^'ab' asString"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "ab"
    ));
    assert!(matches!(
        interpreter.eval("| text copied | text := 'ab'. copied := text copy. text at: 1 put: $z. ^copied"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "ab"
    ));
}

#[test]
fn string_literals_are_fresh_in_methods_blocks_and_literal_arrays() {
    let interpreter = interpreter();
    interpreter
        .load_source("Object subclass: StringMaker [ make [ ^'ab' ] nested [ ^#(('ab')) ] ]")
        .unwrap();
    for source in [
        "| maker first second | maker := StringMaker basicNew. first := maker make. second := maker make. first at: 1 put: $z. ^second",
        "| block first second | block := ['ab']. first := block value. second := block value. first at: 1 put: $z. ^second",
        "| maker first second | maker := StringMaker basicNew. first := maker nested. second := maker nested. ((first at: 1) at: 1) at: 1 put: $z. ^(second at: 1) at: 1",
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::String(value)) if value.to_rust_string().unwrap() == "ab"),
            "{source}"
        );
    }
}

#[test]
fn strings_reject_invalid_indices_characters_and_non_bmp_input() {
    let interpreter = interpreter();
    for (source, expected) in [
        ("^'a' at: 0", "out of bounds"),
        ("^'a' at: 2", "out of bounds"),
        ("^'a' at: true", "integer index"),
        ("^'a' at: 1 put: 1", "integer index and a character"),
        ("^'a' at: 1 put: $😀", "BMP"),
        ("^String new: -1", "non-negative integer"),
        ("^'a' , 1", "string argument"),
    ] {
        let error = interpreter.eval(source).unwrap_err().to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
    let error = interpreter.eval("^'😀'").unwrap_err();
    assert!(matches!(error, InterpreterError::Parse(_)), "{error}");
    assert!(error.to_string().contains("outside the BMP"), "{error}");
    assert!(Value::try_from("😀").is_err());
    assert!(
        matches!(Value::try_from("äΩ"), Ok(Value::String(value)) if value.to_rust_string().unwrap() == "äΩ")
    );
}

#[test]
fn symbols_are_distinct_from_mutable_strings() {
    let interpreter = interpreter();
    assert!(matches!(
        interpreter.eval("^#named"),
        Ok(Value::Symbol("#named"))
    ));
    assert!(matches!(
        interpreter.eval("^'#named'"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "#named"
    ));
    assert!(matches!(
        interpreter.eval("^#named size"),
        Ok(Value::Integer(5))
    ));
    assert!(
        matches!(interpreter.eval("^#named class"), Ok(Value::Class(class)) if class.name().unwrap() == "Symbol")
    );
    for (source, expected) in [
        ("^#named isSymbol", true),
        ("^#named isString", false),
        ("^'named' isSymbol", false),
        ("^#named = #named", true),
        ("^#named = #other", false),
        ("^#named = 'named'", false),
        ("^'named' = #named", false),
        ("^#named == 'named' asSymbol", true),
        ("^#named asSymbol == #named", true),
        ("^#named copy == #named", true),
        ("^#named asString == #named asString", false),
        ("^(Symbol intern: 'named') == #named", true),
        ("^(Symbol intern: #named) == #named", true),
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::Boolean(actual)) if actual == expected),
            "{source}"
        );
    }
    let Value::Symbol(literal) = interpreter.eval("^#named").unwrap() else {
        panic!("expected Symbol")
    };
    let Value::Symbol(converted) = interpreter.eval("^'named' asSymbol").unwrap() else {
        panic!("expected Symbol")
    };
    assert!(std::ptr::eq(literal, converted));
}

#[test]
fn symbols_convert_strings_and_support_bmp_indexing() {
    let interpreter = interpreter();
    for (source, expected) in [
        ("^#named asString", "named"),
        ("^#at:put: asString", "at:put:"),
        ("^#+ asString", "+"),
        ("^'#named' asSymbol asString", "#named"),
        ("^'' asSymbol asString", ""),
        ("^(String new: 1) asSymbol asString", "\0"),
        (
            "| text sym | text := 'äΩ'. sym := text asSymbol. text at: 1 put: $z. ^sym asString",
            "äΩ",
        ),
        (
            "| text | text := #named asString. text at: 1 put: $z. ^#named asString",
            "named",
        ),
    ] {
        assert!(
            matches!(interpreter.eval(source), Ok(Value::String(value)) if value.to_rust_string().unwrap() == expected),
            "{source}"
        );
    }
    assert!(matches!(
        interpreter.eval("^'äΩ' asSymbol size"),
        Ok(Value::Integer(2))
    ));
    assert!(matches!(
        interpreter.eval("^'äΩ' asSymbol basicSize"),
        Ok(Value::Integer(2))
    ));
    assert!(matches!(
        interpreter.eval("^'äΩ' asSymbol at: 2"),
        Ok(Value::Character('Ω'))
    ));
    assert!(matches!(
        interpreter.eval("^'äΩ' asSymbol basicAt: 1"),
        Ok(Value::Character('ä'))
    ));
    for (source, expected) in [
        ("^#named at: 0", "out of bounds"),
        ("^#named at: 6", "out of bounds"),
        ("^'' asSymbol at: 1", "out of bounds"),
        ("^#named at: true", "integer index"),
        ("^#named at: 1 put: $z", "Cannot modify a Symbol"),
        ("^#named basicAt: 1 put: $z", "Cannot modify a Symbol"),
        ("^Symbol intern: 1", "requires a string or symbol"),
    ] {
        let error = interpreter.eval(source).unwrap_err().to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
}

#[test]
fn concurrent_symbol_interning_reuses_the_same_value() {
    let interpreter = Arc::new(interpreter());
    let barrier = Arc::new(Barrier::new(4));
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let interpreter = interpreter.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                for _ in 0..20 {
                    assert!(matches!(
                        interpreter.eval("^('äΩ' asSymbol) == ('äΩ' asSymbol)"),
                        Ok(Value::Boolean(true))
                    ));
                }
                match interpreter.eval("^'äΩ' asSymbol").unwrap() {
                    Value::Symbol(symbol) => symbol,
                    other => panic!("expected Symbol, got {other:?}"),
                }
            })
        })
        .collect();
    let symbols: Vec<_> = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect();
    assert!(
        symbols
            .iter()
            .all(|symbol| std::ptr::eq(symbols[0], *symbol))
    );
}

#[test]
fn shared_string_mutations_are_synchronized() {
    let interpreter = Arc::new(interpreter());
    interpreter
        .load_source("Object subclass: SharedString [ registry [ ^#(nil) ] ]")
        .unwrap();
    interpreter
        .eval("(SharedString basicNew) registry at: 1 put: '....'")
        .unwrap();

    let threads: Vec<_> = (0..4)
        .map(|index| {
            let interpreter = interpreter.clone();
            std::thread::spawn(move || {
                let source = format!(
                    "^((SharedString basicNew) registry at: 1) at: {} put: ${}",
                    index + 1,
                    ['a', 'b', 'c', 'd'][index]
                );
                for _ in 0..10 {
                    interpreter.eval(&source).unwrap();
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
    assert!(matches!(
        interpreter.eval("^(SharedString basicNew) registry at: 1"),
        Ok(Value::String(value)) if value.to_rust_string().unwrap() == "abcd"
    ));
}
