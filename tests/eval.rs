use st::{compiler::compile_method, init_tracing, parser::topdown::parse_eval, vm::VirtualMachine};

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
fn recursive_factorial() {
    let src = r#"  | factorial |
        factorial := [:n| n = 1 ifTrue: [1] ifFalse: [(factorial value: n - 1) * n]].
        (1 to: 10) collect: factorial"#;


    let vm = Box::leak(Box::new(VirtualMachine::new()));
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        vm.compile_file("kernel", "Magnitude.st")?;
        vm.compile_file("kernel", "Number.st")?;
        vm.compile_file("kernel", "Integer.st")?;
        Ok(())
    })() {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
            panic!("failed loading file.");
        }
    }

    init_tracing();

    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();

}