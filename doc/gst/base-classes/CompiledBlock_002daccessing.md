[]{#CompiledBlock_002daccessing}

::: header
Next:
[CompiledBlock-basic](CompiledBlock_002dbasic.html#CompiledBlock_002dbasic){accesskey="n"
rel="next"}, Previous: [CompiledBlock class-instance
creation](CompiledBlock-class_002dinstance-creation.html#CompiledBlock-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[CompiledBlock](CompiledBlock.html#CompiledBlock){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledBlock_003a-accessing}

#### 1.38.2 CompiledBlock: accessing {#compiledblock-accessing .subsection}

[]{#index-flags}

**flags**

Answer the 'cleanness' of the block. 0 = clean; 1 = access to receiver
variables and/or self; 2-30 = access to variables that are 1-29 contexts
away; 31 = return from method or push thisContext

[]{#index-method-1}

**method**

Answer the CompiledMethod in which the receiver lies

[]{#index-methodClass}

**methodClass**

Answer the class in which the receiver is installed.

[]{#index-methodClass_003a}

**methodClass: methodClass**

Set the receiver's class instance variable

[]{#index-numArgs-1}

**numArgs**

Answer the number of arguments passed to the receiver

[]{#index-numLiterals}

**numLiterals**

Answer the number of literals for the receiver

[]{#index-numTemps-1}

**numTemps**

Answer the number of temporary variables used by the receiver

[]{#index-selector}

**selector**

Answer the selector through which the method is called

[]{#index-selector_003a}

**selector: aSymbol**

Set the selector through which the method is called

[]{#index-sourceCodeLinesDelta}

**sourceCodeLinesDelta**

Answer the delta from the numbers in LINE_NUMBER bytecodes to source
code line numbers.

[]{#index-sourceCodeMap}

**sourceCodeMap**

Answer an array which maps bytecode indices to source code line numbers.
0 values represent invalid instruction pointer indices.

[]{#index-stackDepth-1}

**stackDepth**

Answer the number of stack slots needed for the receiver
