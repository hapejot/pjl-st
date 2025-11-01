use st::{init_tracing, parser::topdown::parse_eval, vm::VirtualMachine};
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

#[test]
fn class_variables() {
    let mut vm = Box::leak(Box::new(VirtualMachine::new()));
    let src = r#"
nil subclass: Object [
    
    <comment: 'I am the root of the Smalltalk class system. 
All classes in the system are subclasses of me.'>
    <category: 'Language-Implementation'>

    Dependencies := nil.
    FinalizableObjects := nil.
    FinalSemaphore := nil.

    Object class >> update: aspect [
	"Do any global tasks for the ObjectMemory events."

	<category: 'initialization'>
	aspect == #returnFromSnapshot ifFalse: [^self].
	ContextPart checkPresenceOfJIT.
	FinalizableObjects := nil
    ] ]
"#;
    init_tracing();
    let r = vm.compile_src(src);
}
