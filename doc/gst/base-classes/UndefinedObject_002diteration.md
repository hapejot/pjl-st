[]{#UndefinedObject_002diteration}

::: header
Next:
[UndefinedObject-printing](UndefinedObject_002dprinting.html#UndefinedObject_002dprinting){accesskey="n"
rel="next"}, Previous: [UndefinedObject-dependents
access](UndefinedObject_002ddependents-access.html#UndefinedObject_002ddependents-access){accesskey="p"
rel="prev"}, Up:
[UndefinedObject](UndefinedObject.html#UndefinedObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UndefinedObject_003a-iteration}

#### 1.201.6 UndefinedObject: iteration {#undefinedobject-iteration .subsection}

[]{#index-ifNil_003aifNotNilDo_003a-1}

**ifNil: nilBlock ifNotNilDo: iterableBlock**

Evaluate nilBlock if the receiver is nil, else evaluate iterableBlock
with each element of the receiver (which should be an Iterable).

[]{#index-ifNotNilDo_003a-1}

**ifNotNilDo: iterableBlock**

Evaluate iterableBlock with each element of the receiver (which should
be an Iterable) if not nil. Else answer nil

[]{#index-ifNotNilDo_003aifNil_003a-1}

**ifNotNilDo: iterableBlock ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate iterableBlock,
passing each element of the receiver (which should be an Iterable).
