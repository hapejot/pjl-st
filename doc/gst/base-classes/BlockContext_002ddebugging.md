[]{#BlockContext_002ddebugging}

::: header
Next:
[BlockContext-printing](BlockContext_002dprinting.html#BlockContext_002dprinting){accesskey="n"
rel="next"}, Previous:
[BlockContext-accessing](BlockContext_002daccessing.html#BlockContext_002daccessing){accesskey="p"
rel="prev"}, Up:
[BlockContext](BlockContext.html#BlockContext){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockContext_003a-debugging}

#### 1.12.2 BlockContext: debugging {#blockcontext-debugging .subsection}

[]{#index-isInternalExceptionHandlingContext}

**isInternalExceptionHandlingContext**

Answer whether the receiver is a context that should be hidden to the
user when presenting a backtrace. Such contexts are never blocks, but
check the rest of the chain.
