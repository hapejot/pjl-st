[]{#CompiledCode_002dtesting-accesses}

::: header
Next:
[CompiledCode-translation](CompiledCode_002dtranslation.html#CompiledCode_002dtranslation){accesskey="n"
rel="next"}, Previous:
[CompiledCode-security](CompiledCode_002dsecurity.html#CompiledCode_002dsecurity){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledCode_003a-testing-accesses}

#### 1.39.11 CompiledCode: testing accesses {#compiledcode-testing-accesses .subsection}

[]{#index-accesses_003a}

**accesses: instVarIndex**

Answer whether the receiver accesses the instance variable with the
given index

[]{#index-assigns_003a}

**assigns: instVarIndex**

Answer whether the receiver writes to the instance variable with the
given index

[]{#index-containsLiteral_003a}

**containsLiteral: anObject**

Answer if the receiver contains a literal which is equal to anObject.

[]{#index-hasBytecode_003abetween_003aand_003a}

**hasBytecode: byte between: firstIndex and: lastIndex**

Answer whether the receiver includes the 'byte' bytecode in any of the
indices between firstIndex and lastIndex.

[]{#index-jumpDestinationAt_003aforward_003a}

**jumpDestinationAt: anIndex forward: aBoolean**

Answer where the jump at bytecode index 'anIndex' lands

[]{#index-reads_003a}

**reads: instVarIndex**

Answer whether the receiver reads the instance variable with the given
index

[]{#index-refersTo_003a}

**refersTo: anObject**

Answer whether the receiver refers to the given object

[]{#index-sendsToSuper}

**sendsToSuper**

Answer whether the receiver includes a send to super.

[]{#index-sourceCodeMap-1}

**sourceCodeMap**

Answer an array which maps bytecode indices to source code line numbers.
0 values represent invalid instruction pointer indices.
