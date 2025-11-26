[]{#Iterable_002diteration}

::: header
Next:
[Iterable-streaming](Iterable_002dstreaming.html#Iterable_002dstreaming){accesskey="n"
rel="next"}, Previous:
[Iterable-enumeration](Iterable_002denumeration.html#Iterable_002denumeration){accesskey="p"
rel="prev"}, Up: [Iterable](Iterable.html#Iterable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Iterable_003a-iteration}

#### 1.94.3 Iterable: iteration {#iterable-iteration .subsection}

[]{#index-ifNil_003aifNotNilDo_003a}

**ifNil: nilBlock ifNotNilDo: iterableBlock**

Evaluate nilBlock if the receiver is nil, else evaluate iterableBlock
with each element of the receiver (which should be an Iterable).

[]{#index-ifNotNilDo_003a}

**ifNotNilDo: iterableBlock**

Evaluate iterableBlock with each element of the receiver (which should
be an Iterable) if not nil. Else answer nil

[]{#index-ifNotNilDo_003aifNil_003a}

**ifNotNilDo: iterableBlock ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate iterableBlock,
passing each element of the receiver (which should be an Iterable).
