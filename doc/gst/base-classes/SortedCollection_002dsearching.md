[]{#SortedCollection_002dsearching}

::: header
Next:
[SortedCollection-sorting](SortedCollection_002dsorting.html#SortedCollection_002dsorting){accesskey="n"
rel="next"}, Previous: [SortedCollection-saving and
loading](SortedCollection_002dsaving-and-loading.html#SortedCollection_002dsaving-and-loading){accesskey="p"
rel="prev"}, Up:
[SortedCollection](SortedCollection.html#SortedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SortedCollection_003a-searching}

#### 1.156.8 SortedCollection: searching {#sortedcollection-searching .subsection}

[]{#index-includes_003a-7}

**includes: anObject**

Private - Answer whether the receiver includes an item which is equal to
anObject

[]{#index-indexOf_003astartingAt_003aifAbsent_003a-3}

**indexOf: anObject startingAt: index ifAbsent: aBlock**

Answer the first index \> anIndex which contains anElement. Invoke
exceptionBlock and answer its result if no item is found

[]{#index-occurrencesOf_003a-4}

**occurrencesOf: anObject**

Answer how many occurrences of anObject can be found in the receiver
