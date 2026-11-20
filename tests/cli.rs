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
