[]{#ContextPart_002dbuilt-ins}

::: header
Next:
[ContextPart-copying](ContextPart_002dcopying.html#ContextPart_002dcopying){accesskey="n"
rel="next"}, Previous:
[ContextPart-accessing](ContextPart_002daccessing.html#ContextPart_002daccessing){accesskey="p"
rel="prev"}, Up:
[ContextPart](ContextPart.html#ContextPart){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ContextPart_003a-built-ins}

#### 1.41.4 ContextPart: built ins {#contextpart-built-ins .subsection}

[]{#index-continue_003a} []{#index-ensure_003a-6}
[]{#index-ifCurtailed_003a-1}

**continue: anObject**

Resume execution from the receiver, faking that the context on top of it
in the execution chain has returned anObject. The receiver must belong
to the same process as the executing context, otherwise the results are
not predictable. All #ensure: (and possibly #ifCurtailed:) blocks
between the currently executing context and the receiver are evaluated
(which is not what would happen if you directly bashed at the parent
context of thisContext).
