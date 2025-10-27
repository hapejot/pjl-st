use st::{compiler::compile_method, init_tracing, parser::topdown::parse_eval, vm::VirtualMachine};

#[test]
fn test_simple_add() {
    let src = "1+2";
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let vm = Box::leak(Box::new(st::vm::VirtualMachine::new()));
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
    let vm = Box::leak(Box::new(st::vm::VirtualMachine::new()));
    vm.define_class("Point", vec!["x", "y"]).unwrap();
    vm.define_method("Point", "x:", vec!["new_x"], "x := new_x")
        .unwrap();
    vm.define_method("Point", "y:", vec!["new_y"], "y := new_y")
        .unwrap();

    vm.define_method(
        "Integer",
        "@",
        vec!["y"],
        "|r| r := Point new. r x: self. r y: y. ^r",
    )
    .unwrap();

    let src = "^17 @ 42";
    let cm = compile_method(vec![], vec![], &parse_eval(src).unwrap()).unwrap();
    let result = vm.execute_method(cm).unwrap();
    assert_eq!(format!("{}", result), "<Object Class Point [17, 42]>");
}

#[test]
fn read_point_file() {
    init_tracing();
    let vm = Box::leak(Box::new(st::vm::VirtualMachine::new()));
    match vm.compile_file("kernel", "Point") {
        Err(e) => {
            println!("Error compiling Point.st: {}", e);
            panic!();
        }
        Ok(_) => {}
    }
}
