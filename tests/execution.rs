use std::sync::{Arc, Mutex};

use st::{
    compiler::compile_method,
    memory::Value,
    parser::topdown::parse_eval,
    vm::{
        VirtualMachine,
        block::{CompiledBlock, CompiledMethod},
        execution::{Execution, ExecutionLimits},
        instruction::Instruction,
        register::Register,
    },
};

#[test]
fn instruction_limit_counts_each_instruction_and_resets_per_execution() {
    let vm = VirtualMachine::new();
    let method = CompiledMethod::new(vec![Instruction::Nop, Instruction::Nop], vec![], 0);
    for limit in [0, 1] {
        let error = vm
            .execute_method_with_limits(
                method.clone(),
                ExecutionLimits {
                    max_instructions: Some(limit),
                    ..Default::default()
                },
            )
            .unwrap_err();
        assert!(
            error.to_string().contains("Instruction limit exceeded"),
            "{error}"
        );
        assert!(error.to_string().contains("Smalltalk stack"), "{error}");
        assert!(error.source().is_some_and(|cause| matches!(
            cause.downcast_ref::<st::vm::error::RuntimeError>(),
            Some(st::vm::error::RuntimeError::InstructionLimitExceeded)
        )));
    }
    for _ in 0..2 {
        assert!(
            vm.execute_method_with_limits(
                method.clone(),
                ExecutionLimits {
                    max_instructions: Some(2),
                    ..Default::default()
                },
            )
            .is_ok()
        );
    }
    assert!(vm.execute_method(method).is_ok());
}

#[test]
fn test_execution() {
    let vm = VirtualMachine::new();
    let cm = CompiledMethod::new(vec![], vec![], 0);
    let e = Execution::new(&vm, Value::Integer(17), vec![], cm, 0);
    e.set(Register::Number(0, 0), Value::Integer(42)).unwrap();
    let e2 = e.create_block(1, 0).unwrap();
    assert_eq!(
        format!("{:?}", e.get(Register::Number(0, 0)).unwrap()),
        "Some(Integer(42))"
    );
    assert_eq!(
        format!("{:?}", e.get(Register::Number(0, 1)).unwrap()),
        "Some(Integer(17))"
    );
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 0)).unwrap()),
        "Some(Integer(42))"
    );
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 1)).unwrap()),
        "Some(Integer(17))"
    );

    e2.set(Register::Number(0, 1), Value::Integer(99)).unwrap();
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(0, 1)).unwrap()),
        "Some(Integer(99))"
    );

    let x = e2.registers().unwrap();
    e2.dump_to_trace().unwrap();
    assert_eq!(x.len(), 2);

    e.set(Register::Number(0, 0), Value::Integer(55)).unwrap();
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(1, 0)).unwrap()),
        "Some(Integer(55))"
    );

    e2.set(Register::Number(0, 0), Value::Integer(100)).unwrap();
    assert_eq!(
        format!("{:?}", e2.get(Register::Number(0, 0)).unwrap()),
        "Some(Integer(100))"
    );
}

fn send_array_message(
    vm: &VirtualMachine,
    array: &Arc<Mutex<Vec<Value>>>,
    selector: &'static str,
    args: Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    send_message(vm, Value::Array(array.clone()), selector, args)
}

fn send_message(
    vm: &VirtualMachine,
    receiver: Value,
    selector: &'static str,
    args: Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let registers = (0..args.len())
        .map(|index| Register::Number(0, index + 2))
        .collect();
    let method = CompiledMethod::new(
        vec![Instruction::CallMethod {
            dst: Register::Result,
            receiver: Register::Receiver,
            args: registers,
            selector,
        }],
        vec![],
        args.len(),
    );
    Execution::new(vm, receiver, args, method, 0).execute()
}

#[test]
fn malformed_host_symbols_are_rejected_on_dispatch() {
    let vm = VirtualMachine::new();
    for (symbol, expected) in [
        ("missing-prefix", "start with #"),
        ("#😀", "outside the BMP"),
    ] {
        let error = send_message(&vm, Value::Symbol(symbol), "class", vec![]).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn compiled_methods_validate_arguments_before_running_or_trying_primitives() {
    let vm = VirtualMachine::new();
    vm.define_class("ArityParent", Some("Object"), vec!["state"])
        .unwrap();
    vm.define_class("ArityChild", Some("ArityParent"), vec![])
        .unwrap();
    vm.define_method(
        "ArityParent",
        "state:",
        vec!["value"],
        "state := value. ^state",
    )
    .unwrap();
    vm.define_method(
        "ArityParent",
        "sum:with:",
        vec!["left", "right"],
        "| temp | temp := left + right. ^temp",
    )
    .unwrap();
    vm.define_method(
        "ArityParent",
        "classWith:",
        vec!["unused"],
        "<primitive: VMpr_Object_class> ^99",
    )
    .unwrap();
    let Value::Class(class) = vm.get_class("ArityChild").unwrap() else {
        panic!("expected class")
    };
    let object = st::memory::SmalltalkObject::new(class);
    object.set_instance_var(0, Value::Integer(5)).unwrap();
    let receiver = Value::Object(object.clone());

    for (selector, args, expected) in [
        ("state:", vec![], "expects 1 argument(s), but received 0"),
        (
            "state:",
            vec![Value::Integer(1), Value::Integer(2)],
            "expects 1 argument(s), but received 2",
        ),
        (
            "sum:with:",
            vec![Value::Integer(1)],
            "expects 2 argument(s), but received 1",
        ),
        (
            "classWith:",
            vec![],
            "expects 1 argument(s), but received 0",
        ),
    ] {
        let error = send_message(&vm, receiver.clone(), selector, args)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(&format!("ArityParent>>{selector}")),
            "{error}"
        );
        assert!(error.contains(expected), "{error}");
        assert!(error.contains(&format!("send #{selector}")), "{error}");
    }
    assert!(matches!(object.get_instance_var(0), Ok(Value::Integer(5))));
    assert!(matches!(
        send_message(&vm, receiver.clone(), "state:", vec![Value::Integer(7)]),
        Ok(Value::Integer(7))
    ));
    assert!(matches!(
        send_message(
            &vm,
            receiver.clone(),
            "sum:with:",
            vec![Value::Integer(2), Value::Integer(3)]
        ),
        Ok(Value::Integer(5))
    ));
    assert!(matches!(
        send_message(&vm, receiver, "classWith:", vec![Value::Integer(1)]),
        Ok(Value::Integer(99))
    ));
}

#[test]
fn direct_execution_checks_declared_parameters_not_allocated_registers() {
    let vm = VirtualMachine::new();
    let method = compile_method(
        vec![],
        vec!["input"],
        &parse_eval("| temp | temp := input. ^temp").unwrap(),
    )
    .unwrap();
    let missing = Execution::new(&vm, Value::Nil, vec![], method.clone(), 0);
    missing
        .set(Register::Number(0, 2), Value::Integer(8))
        .unwrap();
    let error = missing.execute().unwrap_err().to_string();
    assert!(
        error.contains("expects 1 argument(s), but received 0"),
        "{error}"
    );
    let error = Execution::new(
        &vm,
        Value::Nil,
        vec![Value::Integer(1), Value::Integer(2)],
        method.clone(),
        0,
    )
    .execute()
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("expects 1 argument(s), but received 2"),
        "{error}"
    );
    assert!(matches!(
        Execution::new(&vm, Value::Nil, vec![Value::Integer(42)], method.clone(), 0).execute(),
        Ok(Value::Integer(42))
    ));
    assert!(
        vm.execute_method(method)
            .unwrap_err()
            .to_string()
            .contains("expects 1 argument(s)")
    );
}

#[test]
fn native_methods_check_selector_arity_before_block_invocation() {
    let vm = VirtualMachine::new();
    let method = CompiledMethod::new(
        vec![],
        vec![CompiledBlock {
            instructions: vec![Instruction::Move {
                dst: Register::Number(0, 0),
                src: Register::Number(0, 1),
            }],
            parameter_count: 1,
            depth: 1,
        }],
        0,
    );
    let home = Execution::new(&vm, Value::Nil, vec![], method, 0);
    let block = Value::Execution(home.create_block(1, 0).unwrap());
    let error = send_message(&vm, block.clone(), "value", vec![Value::Integer(7)])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Execution>>value expects 0 argument(s), but received 1"),
        "{error}"
    );
    assert!(matches!(
        send_message(&vm, block, "value:", vec![Value::Integer(7)]),
        Ok(Value::Integer(7))
    ));
    let error = send_message(&vm, Value::Integer(1), "+", vec![])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Integer>>+ expects 1 argument(s), but received 0"),
        "{error}"
    );
    vm.define_class("Array", Some("Object"), vec![]).unwrap();
    let array = Value::Array(Arc::new(Mutex::new(vec![Value::Nil])));
    let error = send_message(&vm, array, "at:put:", vec![Value::Integer(1)])
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Array>>at:put: expects 2 argument(s), but received 1"),
        "{error}"
    );
}

#[test]
fn array_messages_report_poisoning() {
    let vm = VirtualMachine::new();
    vm.define_class("Array", Some("Object"), vec![]).unwrap();
    let array = Arc::new(Mutex::new(vec![Value::Integer(42)]));
    let messages = [
        ("size", vec![]),
        ("at:", vec![Value::Integer(1)]),
        ("at:put:", vec![Value::Integer(1), Value::Integer(7)]),
    ];

    assert!(matches!(
        send_array_message(&vm, &array, "size", vec![]),
        Ok(Value::Integer(1))
    ));
    assert!(matches!(
        send_array_message(&vm, &array, "at:", vec![Value::Integer(1)]),
        Ok(Value::Integer(42))
    ));
    assert!(matches!(
        send_array_message(
            &vm,
            &array,
            "at:put:",
            vec![Value::Integer(1), Value::Integer(7)]
        ),
        Ok(Value::Integer(7))
    ));
    assert!(matches!(
        send_array_message(&vm, &array, "at:", vec![Value::Integer(1)]),
        Ok(Value::Integer(7))
    ));

    let other = array.clone();
    assert!(
        std::thread::spawn(move || {
            let _guard = other.lock().unwrap();
            panic!("poison array lock");
        })
        .join()
        .is_err()
    );
    for (selector, args) in &messages {
        let error = send_array_message(&vm, &array, selector, args.clone())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("Array lock:") && error.contains("poisoned"),
            "{error}"
        );
    }
}

#[test]
fn malformed_bytecode_returns_errors() {
    let vm = VirtualMachine::new();
    for (instruction, expected) in [
        (
            Instruction::CreateBlock {
                dst: Register::Result,
                prog_id: 7,
            },
            "Block 7 not found",
        ),
        (
            Instruction::LoadImm {
                dst: Register::Receiver,
                value: Value::Integer(1),
            },
            "Invalid destination register",
        ),
        (
            Instruction::LoadImm {
                dst: Register::Number(0, usize::MAX),
                value: Value::Integer(1),
            },
            "Cannot allocate destination register",
        ),
        (
            Instruction::LoadImm {
                dst: Register::Number(1, 0),
                value: Value::Integer(1),
            },
            "Invalid destination register",
        ),
        (
            Instruction::CreateArray {
                dst: Register::Result,
                elements: vec![Register::Number(1, 0)],
            },
            "Register r1-0 not assigned",
        ),
        (
            Instruction::CreateString {
                dst: Register::Result,
                value: "😀".to_owned(),
            },
            "outside the BMP",
        ),
    ] {
        let method = CompiledMethod::new(vec![instruction], vec![], 0);
        let error = vm.execute_method(method).unwrap_err().to_string();
        assert!(error.contains(expected), "{error}");
        assert!(error.contains("Smalltalk stack"), "{error}");
    }

    let execution = Execution::new(
        &vm,
        Value::Nil,
        vec![],
        CompiledMethod::new(vec![], vec![], 0),
        0,
    );
    let block = execution.create_block(1, 7).unwrap();
    assert!(
        block
            .execute()
            .unwrap_err()
            .to_string()
            .contains("Block 7 not found")
    );
    assert!(
        block
            .instructions()
            .unwrap_err()
            .to_string()
            .contains("Block 7 not found")
    );

    let Value::Class(class) = vm.get_class("Object").unwrap() else {
        panic!("expected class")
    };
    let method = CompiledMethod::new(
        vec![
            Instruction::LoadImm {
                dst: Register::Number(0, 2),
                value: Value::Integer(1),
            },
            Instruction::StoreInstanceVar {
                src: Register::Number(0, 2),
                index: usize::MAX,
            },
        ],
        vec![],
        0,
    );
    let execution = Execution::new(
        &vm,
        Value::Object(st::memory::SmalltalkObject::new(class)),
        vec![],
        method,
        0,
    );
    assert!(
        execution
            .execute()
            .unwrap_err()
            .to_string()
            .contains("Cannot allocate instance variable")
    );
}
