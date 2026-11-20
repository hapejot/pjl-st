[]{#ArrayedCollection_002denumerating-the-elements-of-a-collection}

::: header
Next:
[ArrayedCollection-sorting](ArrayedCollection_002dsorting.html#ArrayedCollection_002dsorting){accesskey="n"
rel="next"}, Previous: [ArrayedCollection-copying
Collections](ArrayedCollection_002dcopying-Collections.html#ArrayedCollection_002dcopying-Collections){accesskey="p"
rel="prev"}, Up:
[ArrayedCollection](ArrayedCollection.html#ArrayedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ArrayedCollection_003a-enumerating-the-elements-of-a-collection}

#### 1.5.6 ArrayedCollection: enumerating the elements of a collection {#arrayedcollection-enumerating-the-elements-of-a-collection .subsection}

[]{#index-collect_003a}

**collect: aBlock**

Answer a new instance of an ArrayedCollection containing all the results
of evaluating aBlock passing each of the receiver's elements

[]{#index-reject_003a}

**reject: aBlock**

Answer a new instance of an ArrayedCollection containing all the
elements in the receiver which, when passed to aBlock, answer false

[]{#index-select_003a}

**select: aBlock**

Answer a new instance of an ArrayedCollection containing all the
elements in the receiver which, when passed to aBlock, answer true

[]{#index-with_003acollect_003a}

**with: aSequenceableCollection collect: aBlock**

Evaluate aBlock for each pair of elements took respectively from the
receiver and from aSequenceableCollection; answer a collection of the
same kind of the receiver, made with the block's return values. Fail if
the receiver has not the same size as aSequenceableCollection.
