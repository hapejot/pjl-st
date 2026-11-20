[]{#ContextPart_002denumerating}

::: header
Next:
[ContextPart-printing](ContextPart_002dprinting.html#ContextPart_002dprinting){accesskey="n"
rel="next"}, Previous:
[ContextPart-debugging](ContextPart_002ddebugging.html#ContextPart_002ddebugging){accesskey="p"
rel="prev"}, Up:
[ContextPart](ContextPart.html#ContextPart){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ContextPart_003a-enumerating}

#### 1.41.7 ContextPart: enumerating {#contextpart-enumerating .subsection}

[]{#index-scanBacktraceFor_003ado_003a}

**scanBacktraceFor: selectors do: aBlock**

Scan the backtrace for contexts whose selector is among those listed in
selectors; if one is found, invoke aBlock passing the context.

[]{#index-scanBacktraceForAttribute_003ado_003a}

**scanBacktraceForAttribute: selector do: aBlock**

Scan the backtrace for contexts which have the attribute selector listed
in selectors; if one is found, invoke aBlock passing the context and the
attribute.
