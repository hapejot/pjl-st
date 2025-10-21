use st::{init_tracing, parser::topdown::parse_eval};
use tracing::trace;

#[test]
fn test_grammar() {
    init_tracing();
    trace!("Starting test_grammar");
    let src = "[:x | x + 1]";
    match parse_eval(src) {
    Err(e) => {
        println!("{}", e);
        panic!();
    }
    Ok(result) => println!("{:#?}", result),
}
}

