[]{#WeakArray_002dconversion}

::: header
Next:
[WeakArray-loading](WeakArray_002dloading.html#WeakArray_002dloading){accesskey="n"
rel="next"}, Previous:
[WeakArray-accessing](WeakArray_002daccessing.html#WeakArray_002daccessing){accesskey="p"
rel="prev"}, Up: [WeakArray](WeakArray.html#WeakArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakArray_003a-conversion}

#### 1.215.3 WeakArray: conversion {#weakarray-conversion .subsection}

[]{#index-asArray-2}

**asArray**

Answer a non-weak version of the receiver

[]{#index-deepCopy-11}

**deepCopy**

Returns a deep copy of the receiver (the instance variables are copies
of the receiver's instance variables)

[]{#index-shallowCopy-8}

**shallowCopy**

Returns a shallow copy of the receiver (the instance variables are not
copied)

[]{#index-species-7} []{#index-copyEmpty_003a-4}

**species**

Answer Array; this method is used in the #copyEmpty: message, which in
turn is used by all collection-returning methods (collect:, select:,
reject:, etc.).
