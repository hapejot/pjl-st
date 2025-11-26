[]{#CObject_002dconversion}

::: header
Next:
[CObject-finalization](CObject_002dfinalization.html#CObject_002dfinalization){accesskey="n"
rel="next"}, Previous: [CObject-C data
access](CObject_002dC-data-access.html#CObject_002dC-data-access){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject_003a-conversion}

#### 1.36.8 CObject: conversion {#cobject-conversion .subsection}

[]{#index-castTo_003a-1}

**castTo: aType**

Answer another CObject, pointing to the same address as the receiver,
but belonging to the aType CType.

[]{#index-narrow} []{#index-cObject}

**narrow**

This method is called on CObjects returned by a C call-out whose return
type is specified as a CType; it mostly allows one to change the class
of the returned CObject. By default it does nothing, and that's why it
is not called when #cObject is used to specify the return type.

[]{#index-type-3}

**type**

Answer a CType for the receiver
