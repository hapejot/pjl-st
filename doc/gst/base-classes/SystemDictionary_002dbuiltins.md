[]{#SystemDictionary_002dbuiltins}

::: header
Next: [SystemDictionary-c
call-outs](SystemDictionary_002dc-call_002douts.html#SystemDictionary_002dc-call_002douts){accesskey="n"
rel="next"}, Previous:
[SystemDictionary-basic](SystemDictionary_002dbasic.html#SystemDictionary_002dbasic){accesskey="p"
rel="prev"}, Up:
[SystemDictionary](SystemDictionary.html#SystemDictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SystemDictionary_003a-builtins}

#### 1.161.3 SystemDictionary: builtins {#systemdictionary-builtins .subsection}

[]{#index-basicBacktrace}

**basicBacktrace**

Prints the method invocation stack backtrace, as an aid to debugging

[]{#index-byteCodeCounter}

**byteCodeCounter**

Answer the number of bytecodes executed by the VM

[]{#index-debug}

**debug**

This methods provides a way to break in the VM code. Set a breakpoint in
\_gst_debug and call this method near the point where you think the bug
happens.

[]{#index-declarationTrace}

**declarationTrace**

Answer whether compiled bytecodes are printed on stdout

[]{#index-declarationTrace_003a}

**declarationTrace: aBoolean**

Set whether compiled bytecodes are printed on stdout

[]{#index-executionTrace}

**executionTrace**

Answer whether executed bytecodes are printed on stdout

[]{#index-executionTrace_003a}

**executionTrace: aBoolean**

Set whether executed bytecodes are printed on stdout

[]{#index-getTraceFlag_003a}

**getTraceFlag: anIndex**

Private - Returns a boolean value which is one of the interpreter's
tracing flags

[]{#index-setTraceFlag_003ato_003a}

**setTraceFlag: anIndex to: aBoolean**

Private - Sets the value of one of the interpreter's tracing flags
(indicated by 'anIndex') to the value aBoolean.

[]{#index-verboseTrace}

**verboseTrace**

Answer whether execution tracing prints the object on the stack top

[]{#index-verboseTrace_003a}

**verboseTrace: aBoolean**

Set whether execution tracing prints the object on the stack top
