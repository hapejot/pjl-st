[]{#BlockContext}

::: header
Next: [Boolean](Boolean.html#Boolean){accesskey="n" rel="next"},
Previous: [BlockClosure](BlockClosure.html#BlockClosure){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockContext-1}

### 1.12 BlockContext {#blockcontext .section}

[]{#index-BlockContext}

**Defined in namespace Smalltalk**\
**Superclass: ContextPart**\
**Category: Language-Implementation**

:   My instances represent executing Smalltalk blocks, which are
    portions of executeable code that have access to the environment
    that they were declared in, take parameters, and result from
    BlockClosure objects created to be executed by methods outside the
    current class. Block contexts are created by messages sent to
    compute a closure's value. They contain a stack and also provide
    some methods that can be used in inspection or debugging.

  -------------------------------------------------------------------------------------------------------- ---- ------------
  • [BlockContext-accessing](BlockContext_002daccessing.html#BlockContext_002daccessing){accesskey="1"}:        (instance)
  • [BlockContext-debugging](BlockContext_002ddebugging.html#BlockContext_002ddebugging){accesskey="2"}:        (instance)
  • [BlockContext-printing](BlockContext_002dprinting.html#BlockContext_002dprinting){accesskey="3"}:           (instance)
  -------------------------------------------------------------------------------------------------------- ---- ------------
