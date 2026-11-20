[]{#CCallable-class_002dinstance-creation}

::: header
Next:
[CCallable-accessing](CCallable_002daccessing.html#CCallable_002daccessing){accesskey="n"
rel="next"}, Up: [CCallable](CCallable.html#CCallable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CCallable-class_003a-instance-creation}

#### 1.21.1 CCallable class: instance creation {#ccallable-class-instance-creation .subsection}

[]{#index-for_003areturning_003awithArgs_003a}

**for: aCObject returning: returnTypeSymbol withArgs: argsArray**

Answer a CFunctionDescriptor with the given address, return type and
arguments. The address will be reset to NULL upon image save (and it's
the user's task to figure out a way to reinitialize it!)
