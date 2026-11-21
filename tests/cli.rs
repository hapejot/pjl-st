use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;

use st::{
    compiler::compile_statements,
    debug::Debugger,
    parser::topdown::parse_eval,
    vm::{VirtualMachine, execution::Execution},
};

#[test]
fn cli_runs_without_debugger_when_output_is_piped() {
    let output = Command::new(env!("CARGO_BIN_EXE_st"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["--expression", "^1+2"])
        .stdin(Stdio::null())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "3");
}

#[test]
fn cli_requires_a_terminal_for_debugging() {
    let output = Command::new(env!("CARGO_BIN_EXE_st"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["--debug", "--expression", "^1+2"])
        .stdin(Stdio::null())
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--debug requires a terminal"));
}

#[test]
fn terminal_session_requires_interactive_stdio() {
    let output = Command::new(env!("CARGO_BIN_EXE_st"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["--expression", "Terminal with: [:screen | screen clear]"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Terminal requires a terminal on stdin and stdout"),
        "{stderr}"
    );
    assert!(output.stdout.is_empty());
}

#[test]
fn terminal_operations_require_a_session() {
    let interpreter = st::Interpreter::new(env!("CARGO_MANIFEST_DIR")).unwrap();
    let error = interpreter.eval("Terminal clear").unwrap_err().to_string();
    assert!(
        error.contains("requires an active Terminal with: session"),
        "{error}"
    );
}

#[test]
fn ispf_example_handles_input_and_redraws() {
    let interpreter = st::Interpreter::new(env!("CARGO_MANIFEST_DIR")).unwrap();
    interpreter
        .load_source(
            "Object subclass: ExampleTerminal [
                | step sawSmall sawStatus sawEdit sawReset sawInvalid column row optionText datasetText |
                ExampleTerminal class >> new [ ^self basicNew initialize ]
                initialize [ step := 0. sawSmall := false. sawStatus := false. sawEdit := false. sawReset := false. sawInvalid := false. ^self ]
                size [ step = 0 ifTrue: [^{8. 2}]. ^{80. 24} ]
                hideCursor [ ^self ]
                showCursor [ ^self ]
                background: color [ ^self ]
                foreground: color [ ^self ]
                clear [ ^self ]
                moveToColumn: x row: y [ column := x. row := y. ^self ]
                write: text [
                    (text = 'Terminal') ifTrue: [sawSmall := true].
                    (text = 'Anzeigen: AC (Demo)') ifTrue: [sawStatus := true].
                    (text = 'Bearbeiten: AC (Demo)') ifTrue: [sawEdit := true].
                    (text = 'Eingaben geloescht.') ifTrue: [sawReset := true].
                    (text = 'Ungueltige Option. Bitte 1, 2 oder 3 eingeben.') ifTrue: [sawInvalid := true].
                    (column = 16 and: [row = 2]) ifTrue: [optionText := text].
                    (column = 16 and: [row = 9]) ifTrue: [datasetText := text].
                    ^self
                ]
                nextEvent [
                    step := step + 1.
                    step = 1 ifTrue: [sawSmall ifFalse: [self error: 'missing small screen message']. ^{#resize. 80. 24}].
                    step = 2 ifTrue: [^{#key. #enter. #(). #press}].
                    step = 3 ifTrue: [sawInvalid ifFalse: [self error: 'missing invalid option message']. ^{#key. $1. #(). #press}].
                    step = 4 ifTrue: [^{#key. $1. #(). #release}].
                    step = 5 ifTrue: [^{#key. #tab. #(). #press}].
                    step = 6 ifTrue: [^{#key. $A. #(). #press}].
                    step = 7 ifTrue: [^{#key. $X. #(#control). #press}].
                    step = 8 ifTrue: [^{#key. $B. #(). #press}].
                    step = 9 ifTrue: [^{#key. #backspace. #(). #press}].
                    step = 10 ifTrue: [^{#key. $C. #(). #press}].
                    step = 11 ifTrue: [^{#key. #enter. #(). #press}].
                    step = 12 ifTrue: [sawStatus ifFalse: [self error: 'missing output status']. ^{#key. #tab. #(). #press}].
                    step = 13 ifTrue: [^{#key. #backspace. #(). #press}].
                    step = 14 ifTrue: [^{#key. $2. #(). #press}].
                    step = 15 ifTrue: [^{#key. #enter. #(). #press}].
                    step = 16 ifTrue: [sawEdit ifFalse: [self error: 'missing edit status']. ^{#key. #backspace. #(). #press}].
                    step = 17 ifTrue: [^{#key. $3. #(). #press}].
                    step = 18 ifTrue: [^{#key. #enter. #(). #press}].
                    step = 19 ifTrue: [
                        sawReset ifFalse: [self error: 'missing reset status'].
                        (optionText = '') ifFalse: [self error: 'option was not reset'].
                        (datasetText = '') ifFalse: [self error: 'dataset was not reset'].
                        ^{#key. 3. #(). #press}
                    ].
                    self error: 'unexpected example event'
                ]
            ]
            Terminal extend [
                Terminal class >> with: block [ ^block value: ExampleTerminal new ]
            ]",
        )
        .unwrap();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("ispf.st");
    let source = std::fs::read_to_string(path).unwrap();
    let result = interpreter.eval(&source);
    assert!(matches!(result, Ok(st::memory::Value::Nil)), "{result:?}");
}

#[test]
fn cli_reports_a_missing_project_root_without_panicking() {
    let output = Command::new(env!("CARGO_BIN_EXE_st"))
        .current_dir(std::env::temp_dir())
        .env_remove("CARGO_MANIFEST_DIR")
        .args(["--expression", "^1"])
        .stdin(Stdio::null())
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Could not find project root"), "{stderr}");
    assert!(!stderr.contains("panicked"), "{stderr}");
}

struct FailingDebugger {
    before: bool,
}

impl Debugger for FailingDebugger {
    fn before_execute(&self, _: Execution) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.before {
            Err("debugger stopped before execution".into())
        } else {
            Ok(())
        }
    }

    fn after_execute(&self, _: Execution) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("debugger stopped after execution".into())
    }
}

#[test]
fn debugger_failures_are_returned_to_the_caller() {
    for (before, expected) in [
        (true, "debugger stopped before execution"),
        (false, "debugger stopped after execution"),
    ] {
        let vm = VirtualMachine::new();
        vm.set_debugger(Arc::new(FailingDebugger { before }))
            .unwrap();
        let method = compile_statements(&parse_eval("^1+2").unwrap()).unwrap();
        let error = vm.execute_method(method).unwrap_err().to_string();
        assert!(error.contains(expected), "{error}");
        assert!(error.contains("Smalltalk stack"), "{error}");
    }
}
