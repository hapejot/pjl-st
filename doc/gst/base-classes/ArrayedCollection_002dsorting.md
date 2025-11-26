[]{#ArrayedCollection_002dsorting}

::: header
Next:
[ArrayedCollection-storing](ArrayedCollection_002dstoring.html#ArrayedCollection_002dstoring){accesskey="n"
rel="next"}, Previous: [ArrayedCollection-enumerating the elements of a
collection](ArrayedCollection_002denumerating-the-elements-of-a-collection.html#ArrayedCollection_002denumerating-the-elements-of-a-collection){accesskey="p"
rel="prev"}, Up:
[ArrayedCollection](ArrayedCollection.html#ArrayedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ArrayedCollection_003a-sorting}

#### 1.5.7 ArrayedCollection: sorting {#arrayedcollection-sorting .subsection}

[]{#index-sorted} []{#index-_003c_003d-12}

**sorted**

Return a copy of the receiver sorted according to the default sort
block, which uses #\<= to compare items.

[]{#index-sorted_003a}

**sorted: sortBlock**

Return a copy of the receiver sorted according to the given sort block,
which accepts pair of items and returns true if the first item is less
than the second one.
