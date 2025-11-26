[]{#Class_002dpragmas}

::: header
Next:
[Class-printing](Class_002dprinting.html#Class_002dprinting){accesskey="n"
rel="next"}, Previous: [Class-instance creation -
alternative](Class_002dinstance-creation-_002d-alternative.html#Class_002dinstance-creation-_002d-alternative){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Class_003a-pragmas}

#### 1.31.6 Class: pragmas {#class-pragmas .subsection}

[]{#index-pragmaHandlerFor_003a}

**pragmaHandlerFor: aSymbol**

Answer the (possibly inherited) registered handler for pragma aSymbol,
or nil if not found.

[]{#index-registerHandler_003aforPragma_003a}

**registerHandler: aBlock forPragma: pragma**

While compiling methods, on every encounter of the pragma with the given
name, call aBlock with the CompiledMethod and an array of pragma
argument values.
