# Embedding in Rust

For an overview of the internal modules, see [Runtime architecture](architecture.md).

The host provides a resource root containing `boot.yaml` and the `smalltalk`
directory. These files must be distributed with the application; they are not
embedded in the Rust binary. An absolute path avoids depending on the working
directory.

```rust,no_run
use st::{Interpreter, memory::Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interpreter = Interpreter::new(r"C:\my-smalltalk")?;
    let result = interpreter.eval("^1 + 2")?;
    if let Value::Integer(value) = result {
        println!("{value}");
    }
    Ok(())
}
```

`Interpreter::with_boot_file` accepts separate paths for a YAML manifest and
the directory containing its package subdirectories. `load_source` adds class
and method definitions; they remain available to subsequent calls to `eval`
on the same interpreter. Separate interpreters have separate VM state.
Instances include distinct slots for variables declared by their class and all
superclasses. Methods in a subclass can access inherited instance variables
directly. Reusing an instance-variable name in the same class or an ancestor
is rejected.
`InterpreterError` distinguishes bootstrap, source, parse, compile and
runtime errors. Runtime errors retain the Smalltalk stack.
`load_source` is not transactional: definitions loaded before an error remain
in the interpreter. Dynamic arrays (`{ expression. expression }`) evaluate
their elements in order and create a fresh array on each execution. Literal
arrays (`#(...)`) support numbers, strings, characters, symbols, booleans, nil
and nested literal arrays; unsupported elements produce a runtime error.
Symbols are distinct `Value::Symbol` values. `#[...]` is not yet a distinct
byte-array type.
The low-level parser distinguishes `SmalltalkNode::Array`, `ByteArray`, and
`DynamicArray`.

Strings are mutable, 16-bit BMP character sequences. `size` counts characters,
`at:` uses 1-based indexing, and `at:put:` accepts a character; characters
outside the BMP are rejected. `String new:` initializes its slots to NUL.
String literals are fresh on each execution. Literal arrays containing strings
are rebuilt along the paths to those strings; string-free literal arrays
retain their existing shared behavior. String operations include `size`,
`at:`, `at:put:`, `=`, `,`, `asString`, `asSymbol`, `copy`, and `isString`.

Characters are immediate values and support `class`, `=`, `==`, `isCharacter`,
and `asString`. Character literals and Rust `Value::Character` can represent
Unicode characters outside the BMP, but `asString` rejects those characters
because strings are limited to the BMP. `asString` creates a fresh mutable
string. `Character basicNew` is not supported.

Symbols are immutable, interned BMP character sequences. They support `class`,
`size`, `basicSize`, `at:`, `basicAt:`, `=`, `==`, `asSymbol`, `asString`,
`copy`, `isString`, and `isSymbol`; mutation via `at:put:` or `basicAt:put:`
fails. `asString` creates a fresh mutable string, and `String>>asSymbol` or
`Symbol class>>intern:` interns a snapshot of its input. Equality with a
string is false. Quoted symbol literals are not yet supported; use
`'text' asSymbol` for arbitrary symbol text. Interned symbol text, including
text created at runtime, remains allocated for the process lifetime.

The public Rust representation has changed: `Value::String` contains a
`SmalltalkString`, while symbols use `Value::Symbol` with a leading `#`
(not part of the symbol text). Host-created symbols must use that prefix and
contain only BMP characters. Use `Value::try_from`
to construct a string and `SmalltalkString::to_rust_string` to read it.
These operations return errors for invalid input or poisoned locks.

The library does not install a tracing subscriber or start a debugger. The
optional `set_debugger` accepts an `Arc<dyn Debugger>`; dropping the interpreter
releases its VM and debugger ownership. An escaped block cannot execute after
the VM is dropped.

The implementation is still experimental: some language paths may panic, and
interned strings and reference cycles among Smalltalk objects or closures can
remain allocated after an interpreter is dropped. Scripts are not sandboxed;
do not treat this API as isolation for untrusted code.

An `Interpreter` is `Send + Sync` and can be shared through `Arc`. Calls to
`eval` and `eval_with_limits` can run concurrently against the same VM and
shared values. Individual object-slot and array accesses are synchronized;
sequences of operations, such as a read followed by an increment and write,
are not atomic. A block cannot perform a non-local return into a different
execution, even when its home method is still active there.

`load_source` is exclusive with evaluation and other source loads. A conflicting
operation returns `InterpreterError::Busy` without waiting; retry it after the
other operation finishes. A panic during source loading poisons this state and
subsequent loads and evaluations return `InterpreterError::StatePoisoned`.
Debugger callbacks may run concurrently and must synchronize their own state;
they may evaluate recursively or replace the debugger, but cannot load source
while an evaluation is active. Each execution frame snapshots the debugger
when it starts.

The low-level VM and `Execution` APIs do not enforce the interpreter's
source-loading exclusion. Do not execute the same low-level activation
concurrently. `Value::Array` exposes its mutex: release any host-held array
guard before calling back into the interpreter. `InterpreterError` is
`Send + Sync`, so a worker can return the original result directly:

```rust,no_run
use std::sync::Arc;
use st::Interpreter;

let interpreter = Arc::new(Interpreter::new(r"C:\my-smalltalk")?);
let worker = std::thread::spawn(move || interpreter.eval("^1 + 2"));
let result = worker.join().expect("worker panicked")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Error sources and Smalltalk stacks remain available after the join.
Low-level parser and VM errors and debugger/native callback signatures now
use `Box<dyn Error + Send + Sync>`. Existing implementations and callers
using boxed errors may need to update their signatures accordingly.

Low-level `Execution` accessors, `SmalltalkObject` accessors, and
`SmalltalkClass` lookup and name accessors return `Result`; callers must handle
invalid bytecode, allocation, or lock errors. These signatures have changed.
`VirtualMachine::try_new` reports initialization errors; the compatibility
constructor `VirtualMachine::new` panics if initialization fails.
`Interpreter` uses the fallible constructor.
Object, class, string, array, execution and debugger access wait for ordinary lock
contention and return errors for poisoned locks. Value formatting remains
non-blocking and can show an unavailable placeholder during contention.
Compiler-generated registers are separate from named variables, including when
constructing an AST directly through the low-level compiler API.
Compiled methods validate their declared argument count before execution,
including primitive attempts and direct `Execution::execute` calls. Native
method sends validate the argument count implied by their selector. Low-level
callers constructing a `CompiledMethod` must supply the actual parameter count.

For trusted scripts, `eval_with_limits` can bound VM instruction count and
active Smalltalk call depth per evaluation:

```rust,no_run
use st::{Interpreter, vm::execution::ExecutionLimits};

let interpreter = Interpreter::new(r"C:\my-smalltalk")?;
let value = interpreter.eval_with_limits(
    "^1 + 2",
    ExecutionLimits { max_instructions: Some(10_000), max_call_depth: Some(100) },
)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Each bytecode instruction consumes one unit before debugger callbacks and
execution. The budget is shared with called Smalltalk methods and blocks,
including escaped blocks, and resets for each evaluation. The top-level
execution has depth one; each active Smalltalk method or block adds one. Native
calls themselves do not add a frame. `None` means unlimited, and `eval` retains
its unlimited behavior. Limit failures have a `RuntimeError` cause and include
a Smalltalk stack. Side effects completed before a failure are not rolled back.

These are not security or wall-clock limits: they do not bound parsing,
compilation, memory, or work performed by native code or debugger callbacks.
Excessively high depth limits may still exhaust the Rust stack. Low-level calls
to `Execution::execute` are not covered by an `eval_with_limits` invocation.
