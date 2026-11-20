[]{#UndefinedObject_002dtesting}

::: header
Previous:
[UndefinedObject-storing](UndefinedObject_002dstoring.html#UndefinedObject_002dstoring){accesskey="p"
rel="prev"}, Up:
[UndefinedObject](UndefinedObject.html#UndefinedObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UndefinedObject_003a-testing}

#### 1.201.10 UndefinedObject: testing {#undefinedobject-testing .subsection}

[]{#index-ifNil_003a-1}

**ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else answer nil

[]{#index-ifNil_003aifNotNil_003a-1}

**ifNil: nilBlock ifNotNil: notNilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate notNilBlock,
passing the receiver.

[]{#index-ifNotNil_003a-1}

**ifNotNil: notNilBlock**

Evaluate notNilBlock if the receiver is not nil, passing the receiver.
Else answer nil

[]{#index-ifNotNil_003aifNil_003a-1}

**ifNotNil: notNilBlock ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate notNilBlock,
passing the receiver.

[]{#index-isNil-1}

**isNil**

Answer whether the receiver is the undefined object nil. Always answer
true.

[]{#index-isNull-1}

**isNull**

Answer whether the receiver represents a NULL C pointer. Always answer
true.

[]{#index-notNil-1}

**notNil**

Answer whether the receiver is not the undefined object nil. Always
answer false.
