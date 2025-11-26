[]{#CompiledMethod_002dcompiling}

::: header
Next:
[CompiledMethod-invoking](CompiledMethod_002dinvoking.html#CompiledMethod_002dinvoking){accesskey="n"
rel="next"}, Previous: [CompiledMethod-c
call-outs](CompiledMethod_002dc-call_002douts.html#CompiledMethod_002dc-call_002douts){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledMethod_003a-compiling}

#### 1.40.8 CompiledMethod: compiling {#compiledmethod-compiling .subsection}

[]{#index-methodFormattedSourceString}

**methodFormattedSourceString**

Answer the method source code as a string, formatted using the
RBFormatter. Requires package Parser.

[]{#index-methodParseNode}

**methodParseNode**

Answer the parse tree for the receiver, or nil if there is an error.
Requires package Parser.

[]{#index-parserClass-1}

**parserClass**

Answer a parser class, similar to Behavior\>\>parserClass, that can
parse my source code. Requires package Parser.

[]{#index-recompile}

**recompile**

Recompile the method in the scope of the class where it leaves.

[]{#index-recompileNotifying_003a} []{#index-error_003a-8}

**recompileNotifying: aNotifier**

Recompile the method in the scope of the class where it leaves,
notifying errors to aNotifier by sending it #error:.
