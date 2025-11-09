use core::time;

use st::{
    compiler::{compile_method, compile_statements},
    init_tracing,
    memory::Value,
    parser::topdown::parse_eval,
    time_and_print, time_and_trace, time_execution,
    vm::VirtualMachine,
};
use tracing::trace;

#[test]
fn eval_simple_add() {
    let src = "^1+2";
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    init_tracing();
    let result = &vm.execute_method(cm).unwrap();
    if let st::memory::Value::Integer(3) = result {
    } else {
        panic!("Expected Integer(3), got {:?}", result);
    }
}

#[test]
fn create_class_with_method() {
    init_tracing();
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    vm.define_class("Point", Some("Object"), vec!["x", "y"])
        .unwrap();
    vm.define_method("Point", "x:", vec!["new_x"], "x := new_x")
        .unwrap();
    vm.define_method("Point", "y:", vec!["new_y"], "y := new_y")
        .unwrap();

    vm.define_method("Integer", "@", vec!["y"], "^(Point basicNew) x: self; y: y")
        .unwrap();

    let src = "^17 @ 42";
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(format!("{}", result), "<Object Class Point [17, 42]>");
}

#[test]
fn read_point_file() {
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        vm.compile_file("kernel", "Magnitude.st")?;
        vm.compile_file("kernel", "Number.st")?;
        vm.compile_file("kernel", "Integer.st")?;
        vm.compile_file("kernel", "Point.st")?;
        Ok(())
    })() {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            panic!("failed loading file.");
        }
    }

    let src = "^(17 @ 42)+1";
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    init_tracing();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(format!("{}", result), "<Object Class Point [18, 43]>");
}

#[test]
fn block_eval() {
    let src = "  | adder1 adder2 |
        adder1 := [:x :y | x + y ].
        adder2 := [:x | adder1 value: 1 value: x].
        ^adder2 value: 2";
    init_tracing();

    let vm = Box::leak(Box::new(VirtualMachine::new()));
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!("3", format!("{}", result));
}

#[test]
fn equal_objects() {
    let src = "1 = 2";
    init_tracing();

    let vm = Box::leak(Box::new(VirtualMachine::new()));
    vm.compile_file("kernel", "Object.st").unwrap();
    vm.compile_file("kernel", "Magnitude.st").unwrap();
    vm.compile_file("kernel", "Number.st").unwrap();
    vm.compile_file("kernel", "Integer.st").unwrap();
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!("false", format!("{}", result));
}

#[test]
fn recursive_factorial() {
    let src = r#"  | factorial |
        factorial := [:n| n = 1 ifTrue: [1] ifFalse: [(factorial value: n - 1) * n]].
        (1 to: 10) collect: factorial"#;
    init_tracing();
    trace!("load files");
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        time_and_trace("Load Object", || {vm.compile_file("kernel", "Object.st")})?;
        time_and_trace("Load Behavior", || {vm.compile_file("kernel", "Behavior.st")})?;
        time_and_trace("Load ClassDesc", || {vm.compile_file("kernel", "ClassDesc.st")})?;
        time_and_trace("Load Class", || {vm.compile_file("kernel", "Class.st")})?;
        time_and_trace("Load Matnitude", || {vm.compile_file("kernel", "Magnitude.st")})?;
        time_and_trace("Load Number", || vm.compile_file("kernel", "Number.st"))?;
        time_and_trace("Load Integer", || vm.compile_file("kernel", "Integer.st"))?;
        time_and_trace("Load Iterable", || vm.compile_file("kernel", "Iterable.st"))?;
        time_and_trace("Load Collection", || {vm.compile_file("kernel", "Collection.st")})?;
        time_and_trace("Load SeqCollect", || {vm.compile_file("kernel", "SeqCollect.st")})?;
        time_and_trace("Load ArrayColl", || {vm.compile_file("kernel", "ArrayColl.st")})?;
        time_and_trace("Load Interval", || vm.compile_file("kernel", "Interval.st"))?;
        time_and_trace("Load Boolean", || vm.compile_file("kernel", "Boolean.st"))?;
        time_and_trace("Load True", || vm.compile_file("kernel", "True.st"))?;
        time_and_trace("Load False", || vm.compile_file("kernel", "False.st"))?;
        time_and_trace("Load Array", || vm.compile_file("kernel", "Array.st"))?;
        time_and_trace("Load HashedColl", || vm.compile_file("kernel", "HashedColl.st"))?;
        time_and_trace("Load Dictionary", || vm.compile_file("kernel", "Dictionary.st"))?;
        time_and_trace("Load Builtins", || {vm.compile_file("kernel", "Builtins.st")})?;
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
        Ok(_) => {}
    }
    // Uncomment to see the result:
    // println!("Result: {:?}", result);
}

#[test]
fn timing_examples() {
    init_tracing();

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

    // Example 3: Trace timing (will appear in logs/trace file)
    let result = time_and_trace("traced_operation", || {
        let mut sum = 0;
        for i in 1..1000 {
            sum += i;
        }
        sum
    });
    println!("Trace timing result: {}", result);
}
