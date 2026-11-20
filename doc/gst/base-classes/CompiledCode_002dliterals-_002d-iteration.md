[]{#CompiledCode_002dliterals-_002d-iteration}

::: header
Next:
[CompiledCode-security](CompiledCode_002dsecurity.html#CompiledCode_002dsecurity){accesskey="n"
rel="next"}, Previous: [CompiledCode-decoding
bytecodes](CompiledCode_002ddecoding-bytecodes.html#CompiledCode_002ddecoding-bytecodes){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledCode_003a-literals-_002d-iteration}

#### 1.39.9 CompiledCode: literals - iteration {#compiledcode-literals---iteration .subsection}

[]{#index-allLiteralSymbolsDo_003a} []{#index-allLiteralsDo_003a-1}

**allLiteralSymbolsDo: aBlock**

As with #allLiteralsDo:, but only call aBlock with found Symbols.

[]{#index-allLiteralsDo_003a}

**allLiteralsDo: aBlock**

Walk my literals, descending into Arrays and Messages, invoking aBlock
with each touched object.

[]{#index-literalsDo_003a}

**literalsDo: aBlock**

Invoke aBlock with each object immediately in my list of literals.
