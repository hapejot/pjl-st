[]{#Collection-class_002dinstance-creation}

::: header
Next: [Collection class-multibyte
encodings](Collection-class_002dmultibyte-encodings.html#Collection-class_002dmultibyte-encodings){accesskey="n"
rel="next"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection-class_003a-instance-creation}

#### 1.37.1 Collection class: instance creation {#collection-class-instance-creation .subsection}

[]{#index-from_003a-2}

**from: anArray**

Convert anArray to an instance of the receiver. anArray is structured
such that the instance can be conveniently and fully specified using
brace-syntax, possibly by imposing some additional structure on anArray.

[]{#index-join_003a-1}

**join: aCollection**

Answer a collection formed by treating each element in aCollection as a
'withAll:' argument collection to be added to a new instance.

[]{#index-with_003a-1}

**with: anObject**

Answer a collection whose only element is anObject

[]{#index-with_003awith_003a-1}

**with: firstObject with: secondObject**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003a-1}

**with: firstObject with: secondObject with: thirdObject**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003awith_003a-1}

**with: firstObject with: secondObject with: thirdObject with:
fourthObject**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003awith_003awith_003a-1}

**with: firstObject with: secondObject with: thirdObject with:
fourthObject with: fifthObject**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-withAll_003a-1}

**withAll: aCollection**

Answer a collection whose elements are all those in aCollection
