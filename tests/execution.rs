use st::{
    init_tracing,
    memory::Value,
    vm::{VirtualMachine, block::CompiledMethod, execution::Execution, register::Register},
};

#[test]
fn test_execution() {
    init_tracing();
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    let cm = CompiledMethod::new(vec![], vec![], 0);
    let e = Execution::new(vm, Value::Integer(17), vec![], cm);
    e.set(Register::Number(0, 0), Value::Integer(42));
    let e2 = e.create_block(0);
    assert_eq!(
        format!("{:?}", e.get(Register::Number(0, 0))),
        "Some(Integer(42))"
    );
    assert_eq!(
        format!("{:?}", e.get(Register::Number(0, 1))),
        "Some(Integer(17))"
    );
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 0))),
        "Some(Integer(42))"
    );
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 1))),
        "Some(Integer(17))"
    );

    e2.set(Register::Number(0, 1), Value::Integer(99));
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(0, 1))),
        "Some(Integer(99))"
    );

    let x = e2.registers();
    e2.dump_to_trace();
    assert_eq!(x.len(), 2);

    e.set(Register::Number(0, 0), Value::Integer(55));
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 0))),
        "Some(Integer(55))"
    );

    e2.set(Register::Number(0, 0), Value::Integer(100));
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(0, 0))),
        "Some(Integer(100))"
    );
}
