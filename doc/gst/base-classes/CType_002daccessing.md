[]{#CType_002daccessing}

::: header
Next: [CType-basic](CType_002dbasic.html#CType_002dbasic){accesskey="n"
rel="next"}, Previous: [CType
class-initialization](CType-class_002dinitialization.html#CType-class_002dinitialization){accesskey="p"
rel="prev"}, Up: [CType](CType.html#CType){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CType_003a-accessing}

#### 1.52.3 CType: accessing {#ctype-accessing .subsection}

[]{#index-alignof-23}

**alignof**

Answer the size of the receiver's instances

[]{#index-arrayType_003a}

**arrayType: size**

Answer a CArrayCType which represents an array with the given size of
CObjects whose type is in turn represented by the receiver

[]{#index-cObjectType}

**cObjectType**

Answer the CObject subclass whose instance is created when new is sent
to the receiver

[]{#index-new_003a-3}

**new: anInteger**

Allocate a new CObject with room for anInteger C objects of the type
(class) identified by the receiver. It is the caller's responsibility to
free the memory allocated for it.

[]{#index-ptrType}

**ptrType**

Answer a CPtrCType which represents a pointer to CObjects whose type is
in turn represented by the receiver

[]{#index-sizeof-23}

**sizeof**

Answer the size of the receiver's instances

[]{#index-valueType-1}

**valueType**

valueType is used as a means to communicate to the interpreter the
underlying type of the data. For anything but scalars, it's just 'self'
