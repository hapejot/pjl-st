[]{#CompiledMethod-class_002dinstance-creation}

::: header
Next: [CompiledMethod class-lean
images](CompiledMethod-class_002dlean-images.html#CompiledMethod-class_002dlean-images){accesskey="n"
rel="next"}, Previous: [CompiledMethod class-c
call-outs](CompiledMethod-class_002dc-call_002douts.html#CompiledMethod-class_002dc-call_002douts){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledMethod-class_003a-instance-creation}

#### 1.40.2 CompiledMethod class: instance creation {#compiledmethod-class-instance-creation .subsection}

[]{#index-literals_003anumArgs_003anumTemps_003aattributes_003abytecodes_003adepth_003a}

**literals: lits numArgs: numArg numTemps: numTemp attributes: attrArray
bytecodes: bytecodes depth: depth**

Answer a full fledged CompiledMethod. Construct the method header from
the parameters, and set the literals and bytecodes to the provided ones.
Also, the bytecodes are optimized and any embedded CompiledBlocks
modified to refer to these literals and to the newly created
CompiledMethod.

[]{#index-numArgs_003a}
[]{#index-valueWithReceiver_003awithArguments_003a-1}

**numArgs: args**

Create a user-defined method (one that is sent
#valueWithReceiver:withArguments: when it is invoked) with numArgs
arguments. This only makes sense when called for a subclass of
CompiledMethod.
