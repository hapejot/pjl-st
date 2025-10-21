use st::{compiler::compile, init_tracing, parser::topdown::parse_eval};

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
            compile(&result);
        }
    }
}

#[test]
fn test_simple_add() {
    let src = "1+2";
    match parse_eval(src) {
        Err(e) => {
            println!("{}", e);
            panic!();
        }
        Ok(result) => {
            init_tracing();
            compile(&result);
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
            compile(&result);
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
            compile(&result);
        }
    }
}
