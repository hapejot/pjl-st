[]{#CompiledCode_002daccessing}

::: header
Next:
[CompiledCode-basic](CompiledCode_002dbasic.html#CompiledCode_002dbasic){accesskey="n"
rel="next"}, Previous: [CompiledCode
class-tables](CompiledCode-class_002dtables.html#CompiledCode-class_002dtables){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledCode_003a-accessing}

#### 1.39.4 CompiledCode: accessing {#compiledcode-accessing .subsection}

[]{#index-at_003aput_003a-2}

**at: anIndex put: aBytecode**

Store aBytecode as the anIndex-th bytecode

[]{#index-blockAt_003a}

**blockAt: anIndex**

Answer the CompiledBlock attached to the anIndex-th literal, assuming
that the literal is a CompiledBlock or a BlockClosure.

[]{#index-bytecodeAt_003a}

**bytecodeAt: anIndex**

Answer the anIndex-th bytecode

[]{#index-bytecodeAt_003aput_003a}

**bytecodeAt: anIndex put: aBytecode**

Store aBytecode as the anIndex-th bytecode

[]{#index-flags-1}

**flags**

Private - Answer the optimization flags for the receiver

[]{#index-isAnnotated}

**isAnnotated**

Answer 'false'.

[]{#index-literalAt_003a}

**literalAt: anIndex**

Answer the anIndex-th literal

[]{#index-literalAt_003aput_003a}

**literalAt: anInteger put: aValue**

Store aValue as the anIndex-th literal

[]{#index-literals}

**literals**

Answer the literals referenced by my code or any CompiledCode instances
I own.

[]{#index-method-2}

**method**

Answer the parent method for the receiver, or self if it is a method.

[]{#index-methodClass-1}

**methodClass**

Answer the class in which the receiver is installed.

[]{#index-methodClass_003a-1}

**methodClass: methodClass**

Set the receiver's class instance variable

[]{#index-numArgs-2}

**numArgs**

Answer the number of arguments for the receiver

[]{#index-numLiterals-1}

**numLiterals**

Answer the number of literals for the receiver

[]{#index-numTemps-2}

**numTemps**

Answer the number of temporaries for the receiver

[]{#index-primitive}

**primitive**

Answer the primitive called by the receiver

[]{#index-selector-1}

**selector**

Answer the selector through which the method is called

[]{#index-selector_003a-1}

**selector: aSymbol**

Set the selector through which the method is called

[]{#index-sourceCodeLinesDelta-1}

**sourceCodeLinesDelta**

Answer the delta from the numbers in LINE_NUMBER bytecodes to source
code line numbers.

[]{#index-stackDepth-2}

**stackDepth**

Answer the number of stack slots needed for the receiver

------------------------------------------------------------------------

::: header
Next:
[CompiledCode-basic](CompiledCode_002dbasic.html#CompiledCode_002dbasic){accesskey="n"
rel="next"}, Previous: [CompiledCode
class-tables](CompiledCode-class_002dtables.html#CompiledCode-class_002dtables){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
