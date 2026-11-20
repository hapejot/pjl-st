mod support;

use st::{
    compiler::{VariableAllocation, compile_method, compile_statements},
    memory::Value,
    parser::topdown::{SmalltalkNode, parse_eval},
    vm::{VirtualMachine, execution::Execution, register::Register},
};

#[test]
fn test_block() {
    let src = "[:x | |y| x + 1] value: 1";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            compile_statements(&result).unwrap();
        }
    }
}

#[test]
fn test_simple_add_compile() {
    let src = "1+2";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            compile_statements(&result).unwrap();
        }
    }
}

#[test]
fn test_complex_expression() {
    let src = "1+2*3-4/5";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            compile_statements(&result).unwrap();
        }
    }
}

#[test]
fn test_statements() {
    let src = "|a b| a := 10. b := 20. a + b";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            let m = compile_statements(&result).unwrap();
            m.dump_to_trace();
        }
    }
}

#[test]
fn test_statements_with_block() {
    let src = "|a b x| a := 10. b := 20. x := 0. ^[:a| x := a + b]";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            let m = compile_statements(&result).unwrap();
            m.dump_to_trace();
        }
    }
}

#[test]
fn test_statements_with_nested_blocks_and_vars() {
    let src = r#"self withAllSuperclassesDo: [:behavior | | classSpaces |
	    aBlock value: behavior classPool.

	    "Extract the spaces of this class from superclassSpaces into
	     classSpaces..."
            superclassSpaces isNil
		ifTrue: [classSpaces := IdentitySet new.
                         (env ifNil: [behavior environment])
                             withAllSuperspacesDo: [:each | classSpaces add: each]]
		ifFalse: [classSpaces := superclassSpaces].]
"#;
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            let m = compile_statements(&result).unwrap();
            assert_eq!(m.blocks().len(), 5);
            m.dump_to_trace();
        }
    }
}

#[test]
fn test_statements_with_nested_blocks() {
    let src = r#"  | factorial |
        factorial := [:n| n = 1 ifTrue: [1] ifFalse: [(factorial value: n - 1) * n]].
        (1 to: 10) collect: factorial"#;
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            let m = compile_statements(&result).unwrap();
            assert_eq!(m.blocks().len(), 3);
            m.dump_to_trace();
        }
    }
}

#[test]
fn test_var_allocation() {
    let mut va = VariableAllocation::new();
    assert!(va.add("@result").is_ok());
    assert_eq!(va.get("@result"), Some(Register::Number(0, 0)));
    assert_eq!(va.get("@result"), Some(Register::Result));

    assert!(va.add("self").is_ok());
    assert_eq!(va.get("self"), Some(Register::Number(0, 1)));
    assert_eq!(va.get("self"), Some(Register::Receiver));

    assert!(va.add("b").is_ok());
    assert_eq!(va.get("b"), Some(Register::Number(0, 2)));

    assert!(va.add("@result").is_err()); // duplicate
}

#[test]
fn test_nested_var_allocation() {
    let mut va = VariableAllocation::new();
    assert!(va.add("a").is_ok());
    assert_eq!(va.get("a"), Some(Register::Number(0, 0)));

    {
        let mut inner_va = va.create_child().unwrap();
        assert!(inner_va.add("b").is_ok());
        assert_eq!(inner_va.get("b"), Some(Register::Number(0, 0)));
        assert_eq!(inner_va.get("a"), Some(Register::Number(1, 0)));
        assert_eq!(inner_va.get("c"), None);
        let x = inner_va.add("a");
        inner_va.dump_to_trace();
        assert_eq!(x, Ok(1));

        assert_eq!(inner_va.get("a"), Some(Register::Number(0, 1)));
    }

    assert_eq!(va.get("b"), None);
    assert_eq!(va.get("c"), None);
}

#[test]
fn internal_registers_do_not_collide_with_method_or_block_parameters() {
    let vm = VirtualMachine::new();
    let method_body = SmalltalkNode::Return(Box::new(SmalltalkNode::MessageInvoke {
        receiver: Box::new(SmalltalkNode::Identifier("@r3")),
        messages: vec![SmalltalkNode::Message {
            selector: "+",
            arguments: vec![SmalltalkNode::Value(Value::Integer(2))],
        }],
    }));
    let method = compile_method(vec![], vec!["@r3"], &method_body).unwrap();
    let result = Execution::new(&vm, Value::Nil, vec![Value::Integer(40)], method, 0)
        .execute()
        .unwrap();
    assert!(matches!(result, Value::Integer(42)));

    let block_call = SmalltalkNode::MessageInvoke {
        receiver: Box::new(SmalltalkNode::Block {
            parameters: vec!["@r0"],
            temps: vec![],
            body: Box::new(SmalltalkNode::Identifier("@r0")),
        }),
        messages: vec![SmalltalkNode::Message {
            selector: "value:",
            arguments: vec![SmalltalkNode::Value(Value::Integer(42))],
        }],
    };
    let method = compile_statements(&block_call).unwrap();
    assert!(matches!(vm.execute_method(method), Ok(Value::Integer(42))));
}

#[test]
fn internal_registers_are_not_visible_as_variables() {
    let vm = VirtualMachine::new();
    let node = SmalltalkNode::Return(Box::new(SmalltalkNode::MessageInvoke {
        receiver: Box::new(SmalltalkNode::Value(Value::Integer(1))),
        messages: vec![SmalltalkNode::Message {
            selector: "+",
            arguments: vec![SmalltalkNode::Identifier("@r2")],
        }],
    }));
    let method = compile_statements(&node).unwrap();
    let error = vm.execute_method(method).unwrap_err().to_string();
    assert!(error.contains("Global name @r2 not defined"), "{error}");
}

#[test]
fn read_big_file() {
    let vm = support::vm_with_sources(&[("kernel", "Magnitude.st"), ("kernel", "Number.st")]);
    let m = vm.get_static_method("Number", "readFrom:radix:").unwrap();
    m.dump_to_trace();

    if let st::memory::Value::Class(m) = vm.get_class("Magnitude").unwrap() {
        assert!(m.method_names().unwrap().contains(&"<="));
        assert!(m.lookup_method("<=").unwrap().is_some());
    }

    let c = vm.get_class("Number").unwrap();
    assert!(Execution::lookup_method(c, "<=").is_ok());
}
