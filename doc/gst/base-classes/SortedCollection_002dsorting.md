[]{#SortedCollection_002dsorting}

::: header
Previous:
[SortedCollection-searching](SortedCollection_002dsearching.html#SortedCollection_002dsearching){accesskey="p"
rel="prev"}, Up:
[SortedCollection](SortedCollection.html#SortedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SortedCollection_003a-sorting}

#### 1.156.9 SortedCollection: sorting {#sortedcollection-sorting .subsection}

[]{#index-sort-1}

**sort**

Sort the contents of the receiver according to the given sort block,
which accepts pair of items and returns true if the first item is less
than the second one. Fails if the collections's sort block is not the
same as the default sort block.

[]{#index-sort_003a-1}

**sort: sortBlock**

Sort the contents of the receiver according to the given sort block,
which accepts pair of items and returns true if the first item is less
than the second one. Fails if the sort block is not the same as the
collection's sort block.
