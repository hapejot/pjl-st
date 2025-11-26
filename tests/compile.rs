use st::{
    compiler::{VariableAllocation, compile_statements},
    init_tracing,
    parser::topdown::parse_eval,
    time_and_trace,
    vm::{VirtualMachine, register::Register},
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
            init_tracing();
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
            init_tracing();
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
            init_tracing();
            compile_statements(&result).unwrap();
        }
    }
}

#[test]
fn test_statements() {
    let src = "|a b| a := 10. b := 20. a + b";
    init_tracing();
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
    init_tracing();
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
    init_tracing();
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
    init_tracing();
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
fn read_big_file() {
    let vm = Box::leak(Box::new(VirtualMachine::new()));

    // time_and_trace("Load Matnitude", || {
    //     vm.compile_file("kernel", "Magnitude.st")
    // }).unwrap();
    init_tracing();
    time_and_trace("Load Number", || vm.compile_file("kernel", "Temp.st")).unwrap();
    let m = vm.get_static_method("Number", "readFrom:radix:").unwrap();
    m.dump_to_trace();
}

fn load_prerequisites(vm: &VirtualMachine) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

#[test]
fn read_builtins() {
    let vm = Box::leak(Box::new(VirtualMachine::new()));
    match load_prerequisites(&vm) {
        Ok(_) => {
            init_tracing();
            time_and_trace("Load Builtins", || vm.compile_file("kernel", "Builtins.st")).unwrap();
        }
        Err(e) => panic!("failed loading file. [{e}]"),
    };
}
