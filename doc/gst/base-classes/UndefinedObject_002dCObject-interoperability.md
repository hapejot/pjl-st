[]{#UndefinedObject_002dCObject-interoperability}

::: header
Next: [UndefinedObject-dependents
access](UndefinedObject_002ddependents-access.html#UndefinedObject_002ddependents-access){accesskey="n"
rel="next"}, Previous: [UndefinedObject-class
polymorphism](UndefinedObject_002dclass-polymorphism.html#UndefinedObject_002dclass-polymorphism){accesskey="p"
rel="prev"}, Up:
[UndefinedObject](UndefinedObject.html#UndefinedObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UndefinedObject_003a-CObject-interoperability}

#### 1.201.4 UndefinedObject: CObject interoperability {#undefinedobject-cobject-interoperability .subsection}

[]{#index-free-1}

**free**

Do nothing, a NULL pointer can be safely freed.

[]{#index-narrow-1}

**narrow**

Return the receiver: a NULL pointer is always nil, whatever its type.
