# Runtime architecture

## Execution path

`Interpreter::eval` parses source into an AST, compiles it into a
`CompiledMethod`, and executes its bytecode. The AST is a compiler input, not
a separate execution engine. Loading class definitions also parses and
compiles method bodies; it does not execute them as scripts.

## Module responsibilities

- [Interpreter](../src/interpreter.rs): embedding facade, bootstrap from
  files, source-loading exclusion, and evaluation entry points.
- [Parser](../src/parser/topdown.rs) and [compiler](../src/compiler/mod.rs):
  syntax, AST, register allocation, and bytecode generation.
- [VM](../src/vm/mod.rs): shared VM state and class registration.
  [Bootstrap](../src/vm/bootstrap.rs) initializes built-in classes;
  [source](../src/vm/source.rs) loads definitions; and
  [primitives](../src/vm/primitives.rs), [numbers](../src/vm/numbers.rs),
  [characters](../src/vm/characters.rs), [byte arrays](../src/vm/byte_arrays.rs),
  [strings](../src/vm/strings.rs),
  and [symbols](../src/vm/symbols.rs)
  implement native methods.
- [Execution](../src/vm/execution.rs): frames, instruction loop, debugger
  callbacks, execution limits, and error stacks.
  Its internal modules handle [instructions](../src/vm/execution/instructions.rs),
  [message dispatch](../src/vm/execution/dispatch.rs),
  [register access](../src/vm/execution/registers.rs), and
  [blocks](../src/vm/execution/blocks.rs).
- [Memory](../src/memory/mod.rs): runtime values, objects, and classes;
  [strings](../src/memory/string.rs) stores mutable BMP strings; and
  [formatting](../src/memory/format.rs) handles their display.

## Invariants

- Each block call creates a fresh activation. A non-local return requires an
  active home method in the same execution.
- Instruction budgets are shared with called Smalltalk methods and blocks;
  call-depth limits count active Smalltalk frames. They are not a security
  sandbox.
- `Interpreter` allows concurrent evaluations but excludes source loading
  during evaluation. Low-level VM and execution APIs do not enforce this
  exclusion.
- Object, class, string, array, and execution state use synchronized access; a
  sequence of accesses is not automatically atomic. Value formatting is
  non-blocking and may display an unavailable placeholder.

For the embedding contract and limitations, see [Embedding in Rust](embedding.md).
