[]{#CompiledBlock-class_002dinstance-creation}

::: header
Next:
[CompiledBlock-accessing](CompiledBlock_002daccessing.html#CompiledBlock_002daccessing){accesskey="n"
rel="next"}, Up:
[CompiledBlock](CompiledBlock.html#CompiledBlock){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledBlock-class_003a-instance-creation}

#### 1.38.1 CompiledBlock class: instance creation {#compiledblock-class-instance-creation .subsection}

[]{#index-new_003aheader_003amethod_003a}

**new: numBytecodes header: anInteger method: outerMethod**

Answer a new instance of the receiver with room for the given number of
bytecodes and the given header.

[]{#index-numArgs_003anumTemps_003abytecodes_003adepth_003aliterals_003a-1}

**numArgs: args numTemps: temps bytecodes: bytecodes depth: depth
literals: literalArray**

Answer an (almost) full fledged CompiledBlock. To make it complete, you
must either set the new object's 'method' variable, or put it into a
BlockClosure and put the BlockClosure into a CompiledMethod's literals.
The clean-ness of the block is automatically computed.
