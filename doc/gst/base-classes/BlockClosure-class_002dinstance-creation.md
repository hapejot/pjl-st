[]{#BlockClosure-class_002dinstance-creation}

::: header
Next: [BlockClosure
class-testing](BlockClosure-class_002dtesting.html#BlockClosure-class_002dtesting){accesskey="n"
rel="next"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure-class_003a-instance-creation}

#### 1.11.1 BlockClosure class: instance creation {#blockclosure-class-instance-creation .subsection}

[]{#index-block_003a}

**block: aCompiledBlock**

Answer a BlockClosure that activates the passed CompiledBlock.

[]{#index-block_003areceiver_003a}

**block: aCompiledBlock receiver: anObject**

Answer a BlockClosure that activates the passed CompiledBlock with the
given receiver.

[]{#index-block_003areceiver_003aouterContext_003a}

**block: aCompiledBlock receiver: anObject outerContext: aContext**

Answer a BlockClosure that activates the passed CompiledBlock with the
given receiver.

[]{#index-numArgs_003anumTemps_003abytecodes_003adepth_003aliterals_003a}

**numArgs: args numTemps: temps bytecodes: bytecodes depth: depth
literals: literalArray**

Answer a BlockClosure for a new CompiledBlock that is created using the
passed parameters. To make it work, you must put the BlockClosure into a
CompiledMethod's literals.
