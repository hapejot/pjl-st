[]{#SequenceableCollection_002dsorting}

::: header
Next: [SequenceableCollection-still
unclassified](SequenceableCollection_002dstill-unclassified.html#SequenceableCollection_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [SequenceableCollection-replacing
items](SequenceableCollection_002dreplacing-items.html#SequenceableCollection_002dreplacing-items){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-sorting}

#### 1.151.9 SequenceableCollection: sorting {#sequenceablecollection-sorting .subsection}

[]{#index-sort} []{#index-_003c_003d-14}

**sort**

Sort the contents of the receiver according to the default sort block,
which uses #\<= to compare items.

[]{#index-sort_003a}

**sort: sortBlock**

Sort the contents of the receiver according to the given sort block,
which accepts pair of items and returns true if the first item is less
than the second one.

[]{#index-sorted-2} []{#index-_003c_003d-15}

**sorted**

Return a copy of the receiver sorted according to the default sort
block, which uses #\<= to compare items.

[]{#index-sorted_003a-2}

**sorted: sortBlock**

Return a copy of the receiver sorted according to the given sort block,
which accepts pair of items and returns true if the first item is less
than the second one.
