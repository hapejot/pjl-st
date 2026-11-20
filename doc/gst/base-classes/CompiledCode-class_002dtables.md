[]{#CompiledCode-class_002dtables}

::: header
Next:
[CompiledCode-accessing](CompiledCode_002daccessing.html#CompiledCode_002daccessing){accesskey="n"
rel="next"}, Previous: [CompiledCode class-instance
creation](CompiledCode-class_002dinstance-creation.html#CompiledCode-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledCode-class_003a-tables}

#### 1.39.3 CompiledCode class: tables {#compiledcode-class-tables .subsection}

[]{#index-bytecodeInfoTable}

**bytecodeInfoTable**

Return a ByteArray which defines some properties of the bytecodes. For
each bytecode, 4 bytes are reserved. The fourth byte is a flag byte: bit
7 means that the argument is a line number to be used in creating the
bytecode-\>line number map.

The first three have a meaning only for those bytecodes that represent a
combination of operations: the combination can be BC1 ARG BC2 OPERAND if
the fourth byte's bit 0 = 0 or BC1 OPERAND BC2 ARG if the fourth byte's
bit 0 = 1

where BC1 is the first byte, BC2 is the second, ARG is the third and
OPERAND is the bytecode argument as it appears in the bytecode stream.

[]{#index-specialSelectors}

**specialSelectors**

Answer an array of message names that don't need to be in literals to be
sent in a method. Their position here reflects their integer code in
bytecode.

[]{#index-specialSelectorsNumArgs} []{#index-specialSelectors-1}

**specialSelectorsNumArgs**

Answer a harmoniously-indexed array of arities for the messages answered
by #specialSelectors.
