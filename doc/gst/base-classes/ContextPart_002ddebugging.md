[]{#ContextPart_002ddebugging}

::: header
Next:
[ContextPart-enumerating](ContextPart_002denumerating.html#ContextPart_002denumerating){accesskey="n"
rel="next"}, Previous:
[ContextPart-copying](ContextPart_002dcopying.html#ContextPart_002dcopying){accesskey="p"
rel="prev"}, Up:
[ContextPart](ContextPart.html#ContextPart){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ContextPart_003a-debugging}

#### 1.41.6 ContextPart: debugging {#contextpart-debugging .subsection}

[]{#index-currentLine}

**currentLine**

Answer the 1-based number of the line that is pointed to by the
receiver's instruction pointer. The DebugTools package caches
information, thus making the implementation faster.

[]{#index-currentLineInFile}

**currentLineInFile**

Answer the 1-based number of the line that is pointed to by the
receiver's instruction pointer, relative to the method's file. The
implementation is slow unless the DebugTools package is loaded.

[]{#index-debugger}

**debugger**

Answer the debugger that is attached to the given context. It is always
nil unless the DebugTools package is loaded.

[]{#index-debuggerClass-1}

**debuggerClass**

Answer which debugger should be used to debug the current context chain.
The class with the highest debugging priority is picked among those
mentioned in the chain.

[]{#index-isInternalExceptionHandlingContext-1}

**isInternalExceptionHandlingContext**

Answer whether the receiver is a context that should be hidden to the
user when presenting a backtrace.
