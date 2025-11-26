[]{#CArrayCType-class_002dinstance-creation}

::: header
Next:
[CArrayCType-accessing](CArrayCType_002daccessing.html#CArrayCType_002daccessing){accesskey="n"
rel="next"}, Up:
[CArrayCType](CArrayCType.html#CArrayCType){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CArrayCType-class_003a-instance-creation}

#### 1.18.1 CArrayCType class: instance creation {#carrayctype-class-instance-creation .subsection}

[]{#index-elementType_003a}

**elementType: aCType**

This method should not be called for instances of this class.

[]{#index-elementType_003anumberOfElements_003a}

**elementType: aCType numberOfElements: anInteger**

Answer a new instance of CPtrCType that maps an array whose elements are
of the given CType, and whose size is exactly anInteger elements (of
course, anInteger only matters for allocation, not for access, since no
out-of-bounds protection is provided for C objects).

[]{#index-from_003a-1}

**from: type**

Private - Called by CType\>\>from: for arrays
