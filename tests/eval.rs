mod support;

use st::{
    compiler::compile_statements, parser::topdown::parse_eval, time_and_print, time_and_trace,
    time_execution, vm::VirtualMachine,
};
use tracing::trace;

fn boot_vm() -> VirtualMachine {
    let boot =
        std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("boot.yaml"))
            .unwrap();
    let boot: serde_yaml::Value = serde_yaml::from_str(&boot).unwrap();
    let mut sources = Vec::new();
    for (package, files) in boot.as_mapping().unwrap() {
        for file in files.as_sequence().unwrap() {
            sources.push((package.as_str().unwrap(), file.as_str().unwrap()));
        }
    }
    support::vm_with_sources(&sources)
}

#[test]
fn array_allocation_and_indexing() {
    let vm = boot_vm();
    for (src, expected) in [
        ("^(Array new: 2) size", "2"),
        ("^Array new: 0", "#()"),
        ("^(Array new: 1) at: 1", "nil"),
        ("| a | a := Array new: 2. a at: 2 put: 42. ^a at: 2", "42"),
        ("^(Array new: 1) at: 1 put: 7", "7"),
        ("| a | a := Array new: 1. a at: 1 put: a. ^a", "#(#(...))"),
    ] {
        let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
        assert_eq!(
            format!("{}", vm.execute_method(cm).unwrap()),
            expected,
            "{src}"
        );
    }
    for (src, expected) in [
        ("^(Array new: 1) at: 0", "index 0 out of bounds"),
        ("^(Array new: 1) at: 2 put: 3", "index 2 out of bounds"),
        ("^(Array new: 1) at: true", "index must be an integer"),
        ("^Array new: -1", "size must be a non-negative integer"),
    ] {
        let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
        let error = vm.execute_method(cm).unwrap_err();
        assert!(error.to_string().contains(expected), "{src}: {error}");
    }
}

#[test]
fn factorial_file_returns_all_results() {
    let vm = boot_vm();
    let src = std::fs::read_to_string(st::get_project_file_path("factorial.st")).unwrap();
    let cm = compile_statements(&parse_eval(&src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(
        format!("{result}"),
        "#(1 2 6 24 120 720 5040 40320 362880 3628800)"
    );
    let st::memory::Value::Array(values) = result else {
        panic!("Expected an Array, got {result:?}");
    };
    let values = values.lock().unwrap();
    let actual: Vec<_> = values
        .iter()
        .map(|value| match value {
            st::memory::Value::Integer(n) => *n,
            other => panic!("Expected Integer, got {other:?}"),
        })
        .collect();
    assert_eq!(
        actual,
        [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]
    );
}

#[test]
fn primitive_pragmas_use_fallback_when_unavailable() {
    let vm = VirtualMachine::new();
    vm.define_method(
        "Integer",
        "missingPrimitive",
        vec![],
        "<primitive: VMpr_Missing> ^42",
    )
    .unwrap();
    vm.define_method(
        "Integer",
        "wrongReceiver",
        vec![],
        "<primitive: VMpr_Behavior_basicNew> ^17",
    )
    .unwrap();
    vm.define_method(
        "Integer",
        "afterTemps",
        vec![],
        "| a | <primitive: VMpr_Missing> ^23",
    )
    .unwrap();
    for (src, expected) in [
        ("^1 missingPrimitive", "42"),
        ("^1 wrongReceiver", "17"),
        ("^1 afterTemps", "23"),
    ] {
        let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
        assert_eq!(format!("{}", vm.execute_method(cm).unwrap()), expected);
    }
}

#[test]
fn class_instance_specs_include_shape_and_inherited_slots() {
    let vm = boot_vm();
    vm.compile_file("kernel", "Behavior.st").unwrap();
    vm.define_class("ParentSlots", Some("Object"), vec!["a"])
        .unwrap();
    vm.define_class("ChildSlots", Some("ParentSlots"), vec!["b", "c"])
        .unwrap();
    vm.compile_src("Array subclass: ArrayChild [ <shape: #inherit> ]")
        .unwrap();
    for (src, expected) in [
        ("^Interval instanceSpec", "12288"),
        ("^ChildSlots instanceSpec", "12288"),
        ("^Array instanceSpec", "31"),
        ("^ArrayChild instanceSpec", "31"),
        ("^31 bitAt: 5", "1"),
        ("^1 ~= 0", "true"),
        ("^Array isVariable", "true"),
        ("^Interval isVariable", "false"),
        ("^Interval instSize", "3"),
        ("^ChildSlots instSize", "3"),
        ("^Array instSize", "0"),
        ("^7 bitShift: 3", "56"),
        ("^1 bitShift: -65", "0"),
        ("^-1 bitShift: -65", "-1"),
        ("^31 bitAnd: 7", "7"),
        ("^true == true", "true"),
        ("^true == false", "false"),
        ("| a | a := Array new: 0. ^a == a", "true"),
        ("^(Array new: 0) == (Array new: 0)", "false"),
    ] {
        let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
        assert_eq!(
            format!("{}", vm.execute_method(cm).unwrap()),
            expected,
            "{src}"
        );
    }
    let cm = compile_statements(&parse_eval("^Object shapes").unwrap()).unwrap();
    let st::memory::Value::Array(shapes) = vm.execute_method(cm).unwrap() else {
        panic!("Expected an array of shapes");
    };
    let shapes = shapes.lock().unwrap();
    assert!(matches!(shapes[0], st::memory::Value::Symbol("#int8")));
    assert!(matches!(shapes[12], st::memory::Value::Nil));
    assert!(matches!(shapes[15], st::memory::Value::Symbol("#pointer")));
    let cm = compile_statements(&parse_eval("^1 bitShift: 63").unwrap()).unwrap();
    assert!(
        vm.execute_method(cm)
            .unwrap_err()
            .to_string()
            .contains("overflow")
    );
}

#[test]
fn variable_class_basic_new_uses_smalltalk_fallback() {
    let vm = boot_vm();
    vm.compile_file("kernel", "Behavior.st").unwrap();
    vm.define_class("Dictionary", Some("Object"), vec![])
        .unwrap();
    vm.compile_file("kernel", "Builtins.st").unwrap();
    let cm = compile_statements(&parse_eval("^(Array basicNew) size").unwrap()).unwrap();
    assert_eq!(format!("{}", vm.execute_method(cm).unwrap()), "0");
}

#[test]
fn eval_simple_add() {
    let src = "^1+2";
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let vm = VirtualMachine::new();
    let result = &vm.execute_method(cm).unwrap();
    if let st::memory::Value::Integer(3) = result {
    } else {
        panic!("Expected Integer(3), got {:?}", result);
    }
}

#[test]
fn create_class_with_method() {
    let vm = VirtualMachine::new();
    vm.define_class("Point", Some("Object"), vec!["x", "y"])
        .unwrap();
    vm.define_method("Point", "x:", vec!["new_x"], "x := new_x")
        .unwrap();
    vm.define_method("Point", "y:", vec!["new_y"], "y := new_y")
        .unwrap();

    vm.define_method("Integer", "@", vec!["y"], "^(Point basicNew) x: self; y: y")
        .unwrap();

    let src = "^17 @ 42";
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(format!("{}", result), "<Object Class Point [17, 42]>");
}

#[test]
fn read_point_file() {
    let vm = support::vm_with_sources(&[
        ("kernel", "Magnitude.st"),
        ("kernel", "Number.st"),
        ("kernel", "Integer.st"),
        ("kernel", "Point.st"),
    ]);

    let src = "^(17 @ 42)+1";
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(format!("{}", result), "<Object Class Point [18, 43]>");
}

#[test]
fn block_eval() {
    let src = "  | adder1 adder2 |
        adder1 := [:x :y | x + y ].
        adder2 := [:x | adder1 value: 1 value: x].
        ^adder2 value: 2";

    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!("3", format!("{}", result));
}

#[test]
fn block_can_be_invoked_repeatedly() {
    let src = "| count increment | count := 0. increment := [count := count + 1]. increment value. ^increment value";
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();

    let result = vm.execute_method(cm).unwrap();
    assert_eq!("2", format!("{}", result));
}

#[test]
fn empty_block_answers_nil() {
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval("^[] value").unwrap()).unwrap();

    let result = vm.execute_method(cm).unwrap();
    assert_eq!("nil", format!("{}", result));
}

#[test]
fn block_rejects_wrong_argument_count() {
    let src = "[:x | x] value: 1 value: 2";
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();

    let error = vm.execute_method(cm).unwrap_err();
    assert_eq!(
        "Block expects 1 argument(s), but received 2",
        error.source().unwrap().to_string()
    );
}

#[test]
fn return_from_block_exits_home_method() {
    let src = "[:x | ^x] value: 17. ^42";
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();

    let result = vm.execute_method(cm).unwrap();
    assert_eq!("17", format!("{}", result));
}

#[test]
fn return_from_nested_block_exits_home_method() {
    let src = "[[^17] value] value. ^42";
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();

    let result = vm.execute_method(cm).unwrap();
    assert_eq!("17", format!("{}", result));
}

#[test]
fn escaped_block_cannot_return_from_inactive_method() {
    let vm = VirtualMachine::new();
    vm.define_class("Holder", Some("Object"), vec!["block"])
        .unwrap();
    vm.define_method("Holder", "save", vec![], "block := [^1]")
        .unwrap();
    vm.define_method("Holder", "evaluate", vec![], "^block value")
        .unwrap();

    let src = "| holder | holder := Holder basicNew. holder save. ^holder evaluate";
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let error = vm.execute_method(cm).unwrap_err();
    assert_eq!(
        "Block cannot return because its home method is no longer active",
        error.source().unwrap().to_string()
    );
}

#[test]
fn runtime_error_shows_smalltalk_stack() {
    let vm = VirtualMachine::new();
    vm.define_class("Parent", Some("Object"), vec![]).unwrap();
    vm.define_class("Child", Some("Parent"), vec![]).unwrap();
    vm.define_method("Parent", "run", vec![], "^[self fail] value")
        .unwrap();
    vm.define_method("Parent", "fail", vec![], "^self missingSelector")
        .unwrap();
    let cm = compile_statements(&parse_eval("^(Child basicNew) run").unwrap()).unwrap();

    let error = vm.execute_method(cm).unwrap_err();
    let message = error.to_string();
    assert!(message.starts_with("Child does not understand #missingSelector"));
    let frames: Vec<_> = message
        .lines()
        .filter_map(|line| line.strip_prefix("  at "))
        .collect();
    assert_eq!(frames.len(), 4);
    assert!(frames[0].starts_with("Parent>>fail [ip "));
    assert!(frames[0].ends_with("send #missingSelector to a Child"));
    assert!(frames[1].starts_with("Block #0 [ip "));
    assert!(frames[2].starts_with("Parent>>run [ip "));
    assert!(frames[3].starts_with("<script> [ip "));
}

#[test]
fn runtime_error_identifies_send_to_undefined() {
    let vm = VirtualMachine::new();
    let cm = compile_statements(&parse_eval("| x | ^x bitAt: 5").unwrap()).unwrap();

    let message = vm.execute_method(cm).unwrap_err().to_string();
    assert!(message.starts_with("Cannot send a message to an undefined value"));
    assert!(message.contains("send #bitAt: to undefined"));
}

#[test]
fn equal_objects() {
    let src = "1 = 2";

    let vm = support::vm_with_sources(&[
        ("kernel", "Object.st"),
        ("kernel", "Magnitude.st"),
        ("kernel", "Number.st"),
        ("kernel", "Integer.st"),
    ]);
    let cm = compile_statements(&parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!("false", format!("{}", result));
}

#[test]
fn recursive_factorial() {
    let src = r#"  | factorial |
        factorial := [:n| n = 1 ifTrue: [1] ifFalse: [(factorial value: n - 1) * n]].
        (1 to: 10) collect: factorial"#;
    trace!("load files");
    let vm = VirtualMachine::new();
    match (|| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        time_and_trace("Load Object", || vm.compile_file("kernel", "Object.st"))?;
        time_and_trace("Load Behavior", || vm.compile_file("kernel", "Behavior.st"))?;
        time_and_trace("Load ClassDesc", || {
            vm.compile_file("kernel", "ClassDesc.st")
        })?;
        time_and_trace("Load Class", || vm.compile_file("kernel", "Class.st"))?;
        time_and_trace("Load Matnitude", || {
            vm.compile_file("kernel", "Magnitude.st")
        })?;
        time_and_trace("Load Number", || vm.compile_file("kernel", "Number.st"))?;
        time_and_trace("Load Integer", || vm.compile_file("kernel", "Integer.st"))?;
        time_and_trace("Load Iterable", || vm.compile_file("kernel", "Iterable.st"))?;
        time_and_trace("Load Collection", || {
            vm.compile_file("kernel", "Collection.st")
        })?;
        time_and_trace("Load SeqCollect", || {
            vm.compile_file("kernel", "SeqCollect.st")
        })?;
        time_and_trace("Load ArrayColl", || {
            vm.compile_file("kernel", "ArrayColl.st")
        })?;
        time_and_trace("Load Interval", || vm.compile_file("kernel", "Interval.st"))?;
        time_and_trace("Load Boolean", || vm.compile_file("kernel", "Boolean.st"))?;
        time_and_trace("Load True", || vm.compile_file("kernel", "True.st"))?;
        time_and_trace("Load False", || vm.compile_file("kernel", "False.st"))?;
        time_and_trace("Load Array", || vm.compile_file("kernel", "Array.st"))?;
        time_and_trace("Load HashedColl", || {
            vm.compile_file("kernel", "HashedColl.st")
        })?;
        time_and_trace("Load Dictionary", || {
            vm.compile_file("kernel", "Dictionary.st")
        })?;
        time_and_trace("Load Builtins", || vm.compile_file("kernel", "Builtins.st"))?;
        vm.compile_file("local", "Execution.st")?;
        Ok(())
    })() {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            panic!("failed loading file.");
        }
    }

    trace!("compile");
    let cm = time_and_print("Compilation", || {
        compile_statements(&parse_eval(src).unwrap()).unwrap()
    });

    trace!("eval");
    let (result, duration) = time_execution(|| vm.execute_method(cm));

    println!("Execution took: {:?}", duration);
    match result {
        Err(e) => {
            panic!("failed execution: {e}");
        }
        Ok(st::memory::Value::Array(values)) => {
            let values = values.lock().unwrap();
            let actual: Vec<_> = values
                .iter()
                .map(|value| match value {
                    st::memory::Value::Integer(n) => *n,
                    other => panic!("Expected Integer, got {other:?}"),
                })
                .collect();
            assert_eq!(
                actual,
                [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800]
            );
        }
        Ok(value) => panic!("Expected Array, got {value:?}"),
    }
    // Uncomment to see the result:
    // println!("Result: {:?}", result);
}

#[test]
fn timing_examples() {
    // Example 1: Basic timing with time_execution
    let (result, duration) = time_execution(|| {
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(10));
        42
    });
    println!(
        "Basic timing - Result: {}, Duration: {:?}",
        result, duration
    );

    // Example 2: Print timing with description
    let result = time_and_print("Sleep operation", || {
        std::thread::sleep(std::time::Duration::from_millis(5));
        "completed"
    });
    println!("Print timing result: {}", result);

    // Example 3: Trace timing with a subscriber local to this test.
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_test_writer()
        .finish();
    let result = tracing::subscriber::with_default(subscriber, || {
        time_and_trace("traced_operation", || {
            let mut sum = 0;
            for i in 1..1000 {
                sum += i;
            }
            sum
        })
    });
    println!("Trace timing result: {}", result);
}
