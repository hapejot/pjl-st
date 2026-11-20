[]{#CType-class_002dC-instance-creation}

::: header
Next: [CType
class-initialization](CType-class_002dinitialization.html#CType-class_002dinitialization){accesskey="n"
rel="next"}, Up: [CType](CType.html#CType){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CType-class_003a-C-instance-creation}

#### 1.52.1 CType class: C instance creation {#ctype-class-c-instance-creation .subsection}

[]{#index-cObjectBinding_003a}

**cObjectBinding: aCObjectSubclassBinding**

Create a new CType for the given subclass of CObject

[]{#index-cObjectType_003a}

**cObjectType: aCObjectSubclass**

Create a new CType for the given subclass of CObject

[]{#index-computeAggregateType_003a} []{#index-array} []{#index-int-2}
[]{#index-ptr}

**computeAggregateType: type**

Private - Called by from: for pointers/arrays. Format of type: (#array
#int 3) or (#ptr #{FooStruct})

[]{#index-from_003a-4}

**from: type**

Private - Pass the size, alignment, and description of CType for aBlock,
given the field description in 'type' (the second element of each pair).
