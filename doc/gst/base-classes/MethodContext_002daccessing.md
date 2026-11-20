[]{#MethodContext_002daccessing}

::: header
Next:
[MethodContext-debugging](MethodContext_002ddebugging.html#MethodContext_002ddebugging){accesskey="n"
rel="next"}, Up:
[MethodContext](MethodContext.html#MethodContext){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#MethodContext_003a-accessing}

#### 1.113.1 MethodContext: accessing {#methodcontext-accessing .subsection}

[]{#index-home-3}

**home**

Answer the MethodContext to which the receiver refers (i.e. the receiver
itself)

[]{#index-isBlock-2}

**isBlock**

Answer whether the receiver is a block context

[]{#index-isDisabled-2} []{#index-ensure_003a-7}

**isDisabled**

Answers whether the receiver has actually ended execution and will be
skipped when doing a return. BlockContexts are removed from the chain
whenever a non-local return is done, but MethodContexts need to stay
there in case there is a non-local return from the #ensure: block.

[]{#index-isEnvironment-2}

**isEnvironment**

To create a valid execution environment for the interpreter even before
it starts, GST creates a fake context which invokes a special
"termination" method. Such a context can be used as a marker for the
current execution environment. Answer whether the receiver is that kind
of context.

[]{#index-isUnwind-2} []{#index-continue_003a-4}
[]{#index-ensure_003a-8}

**isUnwind**

Answers whether the context must continue execution even after a
non-local return (a return from the enclosing method of a block, or a
call to the #continue: method of ContextPart). Such contexts are created
only by #ensure:.

[]{#index-mark} []{#index-valueWithUnwind-2}

**mark**

To create a valid execution environment for the interpreter even before
it starts, GST creates a fake context which invokes a special
"termination" method. A similar context is created by #valueWithUnwind,
by using this method.

[]{#index-sender}

**sender**

Return the context from which the receiver was sent
