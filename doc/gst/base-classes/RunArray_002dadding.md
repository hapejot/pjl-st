[]{#RunArray_002dadding}

::: header
Next:
[RunArray-basic](RunArray_002dbasic.html#RunArray_002dbasic){accesskey="n"
rel="next"}, Previous:
[RunArray-accessing](RunArray_002daccessing.html#RunArray_002daccessing){accesskey="p"
rel="prev"}, Up: [RunArray](RunArray.html#RunArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RunArray_003a-adding}

#### 1.147.3 RunArray: adding {#runarray-adding .subsection}

[]{#index-add_003aafterIndex_003a-1}

**add: anObject afterIndex: anIndex**

Add anObject after the element at index anIndex

[]{#index-addAll_003aafterIndex_003a-1} []{#index-do_003a-20}

**addAll: aCollection afterIndex: anIndex**

Add all the elements of aCollection after the one at index anIndex. If
aCollection is unordered, its elements could be added in an order which
is not the #do: order

[]{#index-addAllFirst_003a-1} []{#index-do_003a-21}

**addAllFirst: aCollection**

Add all the elements of aCollection at the beginning of the receiver. If
aCollection is unordered, its elements could be added in an order which
is not the #do: order

[]{#index-addAllLast_003a-1} []{#index-do_003a-22}

**addAllLast: aCollection**

Add all the elements of aCollection at the end of the receiver. If
aCollection is unordered, its elements could be added in an order which
is not the #do: order

[]{#index-addFirst_003a-2}

**addFirst: anObject**

Add anObject at the beginning of the receiver. Watch out: this operation
can cause serious performance pitfalls

[]{#index-addLast_003a-2}

**addLast: anObject**

Add anObject at the end of the receiver
