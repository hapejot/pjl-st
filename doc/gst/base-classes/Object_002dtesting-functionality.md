[]{#Object_002dtesting-functionality}

::: header
Next: [Object-VM
callbacks](Object_002dVM-callbacks.html#Object_002dVM-callbacks){accesskey="n"
rel="next"}, Previous: [Object-syntax
shortcuts](Object_002dsyntax-shortcuts.html#Object_002dsyntax-shortcuts){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-testing-functionality}

#### 1.123.18 Object: testing functionality {#object-testing-functionality .subsection}

[]{#index-ifNil_003a}

**ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else answer self

[]{#index-ifNil_003aifNotNil_003a}

**ifNil: nilBlock ifNotNil: notNilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate notNilBlock,
passing the receiver.

[]{#index-ifNotNil_003a}

**ifNotNil: notNilBlock**

Evaluate notNilBlock if the receiver is not nil, passing the receiver.
Else answer nil.

[]{#index-ifNotNil_003aifNil_003a}

**ifNotNil: notNilBlock ifNil: nilBlock**

Evaluate nilBlock if the receiver is nil, else evaluate notNilBlock,
passing the receiver.

[]{#index-isArray-1}

**isArray**

Answer 'false'.

[]{#index-isBehavior-1}

**isBehavior**

Answer 'false'.

[]{#index-isCObject-1}

**isCObject**

Answer 'false'.

[]{#index-isCharacter-1}

**isCharacter**

Answer 'false'.

[]{#index-isCharacterArray-1}

**isCharacterArray**

Answer 'false'.

[]{#index-isClass-1}

**isClass**

Answer 'false'.

[]{#index-isFloat-1}

**isFloat**

Answer 'false'.

[]{#index-isInteger-1}

**isInteger**

Answer 'false'.

[]{#index-isKindOf_003a}

**isKindOf: aClass**

Answer whether the receiver's class is aClass or a subclass of aClass

[]{#index-isMemberOf_003a}

**isMemberOf: aClass**

Returns true if the receiver is an instance of the class 'aClass'

[]{#index-isMeta}

**isMeta**

Same as isMetaclass

[]{#index-isMetaClass}

**isMetaClass**

Same as isMetaclass

[]{#index-isMetaclass-1}

**isMetaclass**

Answer 'false'.

[]{#index-isNamespace-1}

**isNamespace**

Answer 'false'.

[]{#index-isNil}

**isNil**

Answer whether the receiver is nil

[]{#index-isNumber-1}

**isNumber**

Answer 'false'.

[]{#index-isSmallInteger}

**isSmallInteger**

Answer 'false'.

[]{#index-isString}

**isString**

Answer 'false'.

[]{#index-isSymbol}

**isSymbol**

Answer 'false'.

[]{#index-notNil}

**notNil**

Answer whether the receiver is not nil

[]{#index-respondsTo_003a}

**respondsTo: aSymbol**

Returns true if the receiver understands the given selector

------------------------------------------------------------------------

::: header
Next: [Object-VM
callbacks](Object_002dVM-callbacks.html#Object_002dVM-callbacks){accesskey="n"
rel="next"}, Previous: [Object-syntax
shortcuts](Object_002dsyntax-shortcuts.html#Object_002dsyntax-shortcuts){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
