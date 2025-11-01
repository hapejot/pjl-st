use st::{
    compiler::{VariableAllocation, compile_method},
    init_tracing,
    parser::topdown::parse_eval,
};
use tracing::trace;

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
            compile_method(vec![], vec![], &result).unwrap();
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
            compile_method(vec![], vec![], &result).unwrap();
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
            compile_method(vec![], vec![], &result).unwrap();
        }
    }
}

#[test]
fn test_statements() {
    let src = "|a b| a := 10. b := 20. ^a + b";
    init_tracing();
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            compile_method(vec![], vec![], &result).unwrap();
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
            let m = compile_method(vec![], vec![], &result).unwrap();
            trace!("Compiled method: {:#?}", m);
        }
    }
}

#[test]
fn test_var_allocation() {
    let mut va = VariableAllocation::new();
    assert!(va.add("@result").is_ok());
    assert_eq!(va.get("@result"), Some(0));

    assert!(va.add("a").is_ok());
    assert_eq!(va.get("a"), Some(1));

    assert!(va.add("b").is_ok());
    assert_eq!(va.get("b"), Some(2));

    assert!(va.add("@result").is_err()); // duplicate
}

#[test]
fn test_nested_var_allocation() {
    let mut va = VariableAllocation::new();
    assert!(va.add("a").is_ok());
    assert_eq!(va.get("a"), Some(0));

    {
        let mut inner_va = va.create_child().unwrap();
        assert!(inner_va.add("b").is_ok());
        assert_eq!(inner_va.get("b"), Some(1));
        assert_eq!(inner_va.get("a"), Some(0));
        assert_eq!(inner_va.get("c"), None);
        assert_eq!(inner_va.add("a"), Ok(2));

        assert_eq!(inner_va.get("a"), Some(2));
    }

    assert_eq!(va.get("b"), None);
    assert_eq!(va.get("c"), None);
}
