[]{#SequenceableCollection_002denumerating}

::: header
Next:
[SequenceableCollection-manipulation](SequenceableCollection_002dmanipulation.html#SequenceableCollection_002dmanipulation){accesskey="n"
rel="next"}, Previous: [SequenceableCollection-copying
SequenceableCollections](SequenceableCollection_002dcopying-SequenceableCollections.html#SequenceableCollection_002dcopying-SequenceableCollections){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-enumerating}

#### 1.151.6 SequenceableCollection: enumerating {#sequenceablecollection-enumerating .subsection}

[]{#index-anyOne-1}

**anyOne**

Answer an unspecified element of the collection.

[]{#index-do_003a-13}

**do: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection

[]{#index-do_003aseparatedBy_003a-1}

**do: aBlock separatedBy: sepBlock**

Evaluate aBlock for all the elements in the sequenceable collection.
Between each element, evaluate sepBlock without parameters.

[]{#index-doWithIndex_003a} []{#index-keysAndValuesDo_003a-5}

**doWithIndex: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection,
passing the index of each element as the second parameter. This method
is mantained for backwards compatibility and is not mandated by the ANSI
standard; use #keysAndValuesDo:

[]{#index-findFirst_003a}

**findFirst: aBlock**

Returns the index of the first element of the sequenceable collection
for which aBlock returns true, or 0 if none

[]{#index-findLast_003a}

**findLast: aBlock**

Returns the index of the last element of the sequenceable collection for
which aBlock returns true, or 0 if none does

[]{#index-fold_003a-1}

**fold: binaryBlock**

First, pass to binaryBlock the first and second elements of the
receiver; for each subsequent element, pass the result of the previous
evaluation and an element. Answer the result of the last invocation, or
the first element if the collection has size 1. Fail if the collection
is empty.

[]{#index-from_003ato_003ado_003a}

**from: startIndex to: stopIndex do: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection
whose indices are in the range index to stopIndex

[]{#index-from_003ato_003adoWithIndex_003a}
[]{#index-from_003ato_003akeysAndValuesDo_003a-1}

**from: startIndex to: stopIndex doWithIndex: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection
whose indices are in the range index to stopIndex, passing the index of
each element as the second parameter. This method is mantained for
backwards compatibility and is not mandated by the ANSI standard; use
#from:to:keysAndValuesDo:

[]{#index-from_003ato_003akeysAndValuesDo_003a}

**from: startIndex to: stopIndex keysAndValuesDo: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection
whose indices are in the range index to stopIndex, passing the index of
each element as the first parameter and the element as the second.

[]{#index-keys-3}

**keys**

Return an Interval corresponding to the valid indices in the receiver.

[]{#index-keysAndValuesDo_003a-4}

**keysAndValuesDo: aBlock**

Evaluate aBlock for all the elements in the sequenceable collection,
passing the index of each element as the first parameter and the element
as the second.

[]{#index-readStream-6}

**readStream**

Answer a ReadStream streaming on the receiver

[]{#index-readWriteStream}

**readWriteStream**

Answer a ReadWriteStream which streams on the receiver

[]{#index-reverse-2}

**reverse**

Answer the receivers' contents in reverse order

[]{#index-reverseDo_003a}

**reverseDo: aBlock**

Evaluate aBlock for all elements in the sequenceable collection, from
the last to the first.

[]{#index-with_003acollect_003a-1}

**with: aSequenceableCollection collect: aBlock**

Evaluate aBlock for each pair of elements took respectively from the
receiver and from aSequenceableCollection; answer a collection of the
same kind of the receiver, made with the block's return values. Fail if
the receiver has not the same size as aSequenceableCollection.

[]{#index-with_003ado_003a}

**with: aSequenceableCollection do: aBlock**

Evaluate aBlock for each pair of elements took respectively from the
receiver and from aSequenceableCollection. Fail if the receiver has not
the same size as aSequenceableCollection.

------------------------------------------------------------------------

::: header
Next:
[SequenceableCollection-manipulation](SequenceableCollection_002dmanipulation.html#SequenceableCollection_002dmanipulation){accesskey="n"
rel="next"}, Previous: [SequenceableCollection-copying
SequenceableCollections](SequenceableCollection_002dcopying-SequenceableCollections.html#SequenceableCollection_002dcopying-SequenceableCollections){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
