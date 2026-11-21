# Embedding in Rust

For an overview of the internal modules, see [Runtime architecture](architecture.md).

## Interpreter benchmark

Run the single-threaded benchmark in the optimized Cargo bench profile:

```text
cargo bench --offline --bench interpreter
cargo bench --offline --bench interpreter -- --samples 10 --iterations 5 --warmup 2
```

By default it takes five samples of three runs each, after one warmup run per
measurement. `--samples` and `--iterations` must be positive; `--warmup` may
be zero. Omit `--offline` if dependencies are not already available locally.

The benchmark reports the median of the sample-average durations and the
corresponding complete runs per second, not individual bytecode instructions
or dictionary lookups. It measures `Interpreter::new` separately, then parsing,
compilation of an already parsed expression, execution of precompiled bytecode,
and `Interpreter::eval` including parsing, compilation, and execution.
The bytecode measurement uses a separately bootstrapped low-level VM. Both
execution paths check their results on every run outside the timed interval.

Example output (times vary by machine):

```text
integer loop (1000)
  parse                                    0.608160 ms        1644.3 runs/s
  compile (parsed input)                   0.001020 ms      980392.2 runs/s
  execute (compiled bytecode)              1.623800 ms         615.8 runs/s
  Interpreter::eval (end-to-end)           2.270080 ms         440.5 runs/s
```

Workloads cover 1,000 integer loop iterations, block calls and method sends;
a dictionary populated with 100 entries and read 1,000 times; and array
construction followed by `collect:` and `inject:` over 100 elements. Workload
setup is included in execution and evaluation times. Interpreter bootstrap
reads the kernel from disk and can be affected by filesystem caching. Compare
results only on the same machine under comparable conditions; warmup, timer
resolution, OS scheduling, and build settings affect measurements. No debugger
or tracing subscriber is enabled.

### Comparison with native Rust, Python and Node.js

The equivalent native Rust workloads can be measured without external runtimes:

```text
cargo bench --offline --bench interpreter -- --native --samples 7 --iterations 5 --warmup 30
```

They use the same inputs and expected results, including collection
construction. Rust uses `i64`, `HashMap` and `Vec`; the compiler can inline
the closure and method calls and optimize the surrounding code. `black_box`
is applied to loop values and the collected vector to discourage elimination
of the workload; it also affects optimization and adds some measurement
overhead. This is native, statically typed Rust, not a prediction of the speed
of a Smalltalk JIT. Because individual native runs can be shorter than the
timer resolution, each native timer interval batches 100 runs; warmups are
also batches of 100. Result storage and loop overhead are included, and each
result is checked outside the timed interval.

To include the Rust workloads along with Python and Node.js, put `python`
(Python 3) and `node` on PATH and run:

```text
cargo bench --offline --bench interpreter -- --compare --samples 7 --iterations 5 --warmup 30
```

The comparison exits with an error if either runtime or a workload fails. It
reports their versions and executable paths. Python and JavaScript perform the
same five tasks, with the same loop counts and expected results, and use the
same sample, iteration and warmup counts as Smalltalk. Each result is checked
outside the timed interval. Compare their workload times with **execute
(compiled bytecode)**, not `Interpreter::eval`: the Python and JavaScript
programs are loaded before timing. Python uses a `dict` and Node.js a `Map`;
their arrays, numeric types, library implementations and method dispatch are
not identical to Smalltalk's. Node.js may optimize code with its JIT, and warmup
can significantly affect the result. The smallest durations are also sensitive
to per-run timer overhead and scheduling.

The separate process-start measurement launches each runtime, evaluates a
trivial expression yielding `1`, captures its output, and waits for exit.
Smalltalk loads its kernel and evaluates `^1`; Python and Node.js execute
`print(1)` and `console.log(1)`. It includes process creation, output and exit,
runtime initialization and compilation as applicable, and may include a
platform launcher. It does not measure a common library bootstrap. Filesystem
caches and other activity affect these numbers; they are not cold-start
guarantees. The separate `Interpreter::new` measurement above excludes process
startup and exit.

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
Symbols are distinct `Value::Symbol` values.
The low-level parser distinguishes `SmalltalkNode::Array`, `ByteArray`, and
`DynamicArray`.

Arrays support `do:`, `collect:`, `select:`, `reject:`, and `inject:into:`. Enumeration
evaluates blocks in order and propagates block errors and non-local returns.
`String` and `Symbol` also support `select:`, `reject:`, and `collect:`; `collect:` requires
the block to answer characters. Both operations return a fresh `String`.
With block receivers and bodies, `whileTrue:` evaluates without growing the call
stack on each iteration; block instructions still count towards execution limits.

`Dictionary new` and `new:` create dictionaries with `at:`, `at:put:`,
`at:ifAbsent:`, `includesKey:`, `removeKey:`, `removeKey:ifAbsent:`, `keys`,
`values`, `size`, `do:`, and `keysAndValuesDo:`. `keys` and `values` return
fresh arrays.
Keys are compared using Smalltalk `=`. Numbers and symbols use hash buckets;
other keys, including mutable strings and user-defined objects, retain linear
lookup without requiring a custom `hash`. Mixed lookups also check these
fallback keys in insertion order. Mutating a key between operations remains
supported; changing keys or the dictionary during enumeration is not.
Redefining built-in numeric or symbol equality disables bucket lookup.
Dictionary operations comprising multiple accesses are not atomic across
concurrent evaluations. Smalltalk dictionaries are exposed as `Value::Object`;
`Value::Dictionary` is used for VM globals.

Integer, Float, String, and Symbol implement value-based `hash`; equal integer
and float values have the same hash. Strings may change their hash when mutated.

`OrderedCollection` is a resizable sequence supporting indexed access, `add:`,
`addFirst:`, `addLast:`, `removeFirst`, `removeLast`, `copy`, and `asArray`.
Its inherited enumeration methods produce ordered collections.

`ReadStream`, `WriteStream`, and `ReadWriteStream` can be created with `on:`
for arrays, strings, byte arrays, or ordered collections. Reads support `next`
and `peek` (both return nil at the end); writes support `nextPut:` and
`nextPutAll:`. Streams expose `contents`, zero-based `position`,
`position:`, `reset`, and `atEnd`. A `WriteStream` starts empty, while a
`ReadWriteStream` initially contains the supplied collection. Writes update
the supplied collection while there is space; growth of fixed-size collections
switches the stream to a new backing collection. `contents` returns a copy.

## Fullscreen terminal

`Terminal with: [:screen | ...]` explicitly opens a fullscreen session on the
process's stdin and stdout. Both must be terminals. The block receives
`Terminal` and its result is returned. Normal completion, a non-local return,
and runtime errors restore the alternate screen, colors, cursor visibility and
any raw mode enabled by the session. Raw mode already enabled through
crossterm remains enabled.
On Windows, the session also restores the original console input and output
modes.
Restoration errors are reported. Abrupt process termination cannot be cleaned
up. The session assumes exclusive control of the terminal: the previous
alternate-screen, colors, and cursor visibility cannot be queried or restored
exactly, and unrelated host terminal I/O can interfere.

Only one session or terminal-debugger interaction may use the terminal at a
time, including across interpreters. Conflicts fail rather than wait; do not
run the fullscreen debugger while a terminal session is active. A terminal
operation outside its own `with:` block fails. The application is responsible
for quitting its input loop; raw-mode Ctrl-C is input rather than guaranteed
process termination. Instruction limits do not interrupt a blocking read.

The following messages are available on the block argument:

* `size` answers an array `{columns. rows}`.
* `clear`, `moveToColumn:row:` (zero-based), `write:` (String or Character),
  `flush`, `hideCursor`, `showCursor`, `foreground:`, `background:`, and
  `resetColors` control the display. Writes are immediately flushed.
* Colors are symbols: `#reset`, `#black`, `#darkGrey`, `#red`, `#darkRed`,
  `#green`, `#darkGreen`, `#yellow`, `#darkYellow`, `#blue`, `#darkBlue`,
  `#magenta`, `#darkMagenta`, `#cyan`, `#darkCyan`, `#white`, and `#grey`.
* `nextEvent` blocks; `pollEvent: milliseconds` waits for at most the given
  non-negative timeout and answers nil if no event is available.
* A key event is `{#key. key. modifiers. kind}`. The key is a Character, a
  function-key number, or a symbol such as `#enter`, `#escape`, `#up`, `#down`,
  `#left`, `#right`, `#tab`, `#backspace`, `#delete`, `#home`, `#end`,
  `#pageUp`, `#pageDown`, or `#insert`. Modifiers are an array of symbols
  `#shift`, `#control`, `#alt`, `#super`, `#hyper`, or `#meta`; kind is
  `#press`, `#repeat`, or `#release`. Event availability varies by terminal.
* A resize event is `{#resize. columns. rows}`. Mouse support is not enabled;
  other unsupported event and key types report an error.

Run the interactive example with `cargo run -- examples\terminal.st`.
For an ISPF-style example with editable option and dataset fields, run
`cargo run -- examples\ispf.st`. Use Tab to switch fields, Backspace to erase,
Enter to run an option, and F3 or Escape to exit. The example requires at
least 72 columns and 20 rows; on smaller terminals it waits for a resize or
exit. Fields accept ASCII letters, digits, spaces, `.`, `_`, `-`, `/`, `#`,
`$`, and `@`.
Its dataset actions are demonstrations and do not access real files.

Integer and Float support arithmetic and comparisons with each other. Integer
arithmetic remains checked, and integer division retains truncation toward
zero. Mixed arithmetic converts the integer operand to `f64`, which can lose
precision for large integers; comparisons avoid that conversion loss. Division
by zero returns an error for both types. Float values cannot be constructed
with `Float basicNew`.

Strings are mutable, 16-bit BMP character sequences. `size` counts characters,
`at:` uses 1-based indexing, and `at:put:` accepts a character; characters
outside the BMP are rejected. `String new:` initializes its slots to NUL.
String literals are fresh on each execution. Literal arrays containing strings
are rebuilt along the paths to mutable strings or byte arrays; other literal
arrays retain their existing shared behavior. String operations include `size`,
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
string is false. Concatenating strings or symbols with `,` returns a fresh
String. Quoted symbol literals such as `#'hello world'` support escaped
quotes and BMP characters. Interned symbol text, including
text created at runtime, remains allocated for the process lifetime.

`ByteArray` stores mutable bytes in the range 0-255. `new:` initializes bytes
to zero, `at:` and `at:put:` use 1-based indices, and literals such as
`#[0 255]` are fresh on each execution. It also supports `size`, `copy`,
`do:`, `collect:`, and `select:`. Rust hosts receive `Value::ByteArray` with
an `Arc<Mutex<Vec<u8>>>`; release its guard before calling the interpreter.

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
