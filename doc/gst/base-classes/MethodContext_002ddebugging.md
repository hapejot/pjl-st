[]{#MethodContext_002ddebugging}

::: header
Next:
[MethodContext-printing](MethodContext_002dprinting.html#MethodContext_002dprinting){accesskey="n"
rel="next"}, Previous:
[MethodContext-accessing](MethodContext_002daccessing.html#MethodContext_002daccessing){accesskey="p"
rel="prev"}, Up:
[MethodContext](MethodContext.html#MethodContext){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#MethodContext_003a-debugging}

#### 1.113.2 MethodContext: debugging {#methodcontext-debugging .subsection}

[]{#index-isInternalExceptionHandlingContext-2}
[]{#index-exceptionHandlingInternal_003a}

**isInternalExceptionHandlingContext**

Answer whether the receiver is a context that should be hidden to the
user when presenting a backtrace. Such contexts are identified through
the #exceptionHandlingInternal: attribute: if there is such a context in
the backtrace, all those above it are marked as internal.

That is, the attribute being set to true means that the context and all
those above it are to be hidden, while the attribute being set to false
means that the contexts above it must be hidden, but not the context
itself.
