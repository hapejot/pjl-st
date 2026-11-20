[]{#Collection_002dsorting}

::: header
Next:
[Collection-storing](Collection_002dstoring.html#Collection_002dstoring){accesskey="n"
rel="next"}, Previous:
[Collection-removing](Collection_002dremoving.html#Collection_002dremoving){accesskey="p"
rel="prev"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection_003a-sorting}

#### 1.37.13 Collection: sorting {#collection-sorting .subsection}

[]{#index-sorted-1} []{#index-_003c_003d-13}

**sorted**

Return a sequenceable collection with the contents of the receiver
sorted according to the default sort block, which uses #\<= to compare
items.

[]{#index-sorted_003a-1}

**sorted: sortBlock**

Return a sequenceable collection with the contents of the receiver
sorted according to the given sort block, which accepts pair of items
and returns true if the first item is less than the second one.
