use st::{
    memory::Value,
    vm::{CompiledMethod, Execution, VirtualMachine},
};

#[test]
fn test_execution() {
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    let cm = CompiledMethod::new(vec![], vec![], 0);
    let e = Execution::new(vm, Value::Integer(17), vec![], cm);
    e.set(0, Value::Integer(42));
    let e2 = e.create_block(0);
    assert_eq!(format!("{}", e.get(0)), "42");
    assert_eq!(format!("{}", e.get(1)), "17");
    assert_eq!(format!("{}", e2.get(0)), "42");
    assert_eq!(format!("{}", e2.get(1)), "17");

    e2.set(2, Value::Integer(99));
    assert_eq!(format!("{}", e2.get(2)), "99");


    let x = e2.registers();
    assert_eq!(x.len(), 3);


    e.set(0, Value::Integer(55));
    assert_eq!(format!("{}", e2.get(0)), "55");

    e.set(2, Value::Integer(200));
    assert_eq!(format!("{}", e2.get(2)), "99");

    e2.set(0, Value::Integer(100));
    assert_eq!(format!("{}", e.get(0)), "100");    
    assert_eq!(format!("{}", e2.get(0)), "100");    



}
