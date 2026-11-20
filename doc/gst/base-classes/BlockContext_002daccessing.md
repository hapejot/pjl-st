[]{#BlockContext_002daccessing}

::: header
Next:
[BlockContext-debugging](BlockContext_002ddebugging.html#BlockContext_002ddebugging){accesskey="n"
rel="next"}, Up:
[BlockContext](BlockContext.html#BlockContext){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockContext_003a-accessing}

#### 1.12.1 BlockContext: accessing {#blockcontext-accessing .subsection}

[]{#index-caller}

**caller**

Answer the context that called the receiver

[]{#index-home}

**home**

Answer the MethodContext to which the receiver refers, or nil if it has
been optimized away

[]{#index-isBlock}

**isBlock**

Answer whether the receiver is a block context

[]{#index-isDisabled} []{#index-ensure_003a-2}

**isDisabled**

Answers false, because contexts that are skipped when doing a return are
always MethodContexts. BlockContexts are removed from the chain whenever
a non-local return is done, while MethodContexts need to stay there in
case there is a non-local return from the #ensure: block.

[]{#index-isEnvironment}

**isEnvironment**

To create a valid execution environment for the interpreter even before
it starts, GST creates a fake context whose selector is nil and which
can be used as a marker for the current execution environment. Answer
whether the receiver is that kind of context (always false, since those
contexts are always MethodContexts).

[]{#index-isUnwind} []{#index-continue_003a-1} []{#index-ensure_003a-3}

**isUnwind**

Answers whether the context must continue execution even after a
non-local return (a return from the enclosing method of a block, or a
call to the #continue: method of ContextPart). Such contexts are created
only by #ensure: and are always MethodContexts.

[]{#index-nthOuterContext_003a}

**nthOuterContext: n**

Answer the n-th outer block/method context for the receiver

[]{#index-outerContext-1}

**outerContext**

Answer the outer block/method context for the receiver
