[]{#SequenceableCollection_002dcopying-SequenceableCollections}

::: header
Next:
[SequenceableCollection-enumerating](SequenceableCollection_002denumerating.html#SequenceableCollection_002denumerating){accesskey="n"
rel="next"}, Previous:
[SequenceableCollection-concatenating](SequenceableCollection_002dconcatenating.html#SequenceableCollection_002dconcatenating){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-copying-SequenceableCollections}

#### 1.151.5 SequenceableCollection: copying SequenceableCollections {#sequenceablecollection-copying-sequenceablecollections .subsection}

[]{#index-copyAfter_003a}

**copyAfter: anObject**

Answer a new collection holding all the elements of the receiver after
the first occurrence of anObject, up to the last.

[]{#index-copyAfterLast_003a}

**copyAfterLast: anObject**

Answer a new collection holding all the elements of the receiver after
the last occurrence of anObject, up to the last.

[]{#index-copyFrom_003a}

**copyFrom: start**

Answer a new collection containing all the items in the receiver from
the start-th.

[]{#index-copyFrom_003ato_003a-7}

**copyFrom: start to: stop**

Answer a new collection containing all the items in the receiver from
the start-th and to the stop-th

[]{#index-copyReplaceAll_003awith_003a-1}

**copyReplaceAll: oldSubCollection with: newSubCollection**

Answer a new collection in which all the sequences matching
oldSubCollection are replaced with newSubCollection

[]{#index-copyReplaceFrom_003ato_003awith_003a-1}

**copyReplaceFrom: start to: stop with: replacementCollection**

Answer a new collection of the same class as the receiver that contains
the same elements as the receiver, in the same order, except for
elements from index 'start' to index 'stop'.

If start \< stop, these are replaced by the contents of the
replacementCollection. Instead, If start = (stop + 1), like in
'copyReplaceFrom: 4 to: 3 with: anArray', then every element of the
receiver will be present in the answered copy; the operation will be an
append if stop is equal to the size of the receiver or, if it is not, an
insert before index 'start'.

[]{#index-copyReplaceFrom_003ato_003awithObject_003a-1}

**copyReplaceFrom: start to: stop withObject: anObject**

Answer a new collection of the same class as the receiver that contains
the same elements as the receiver, in the same order, except for
elements from index 'start' to index 'stop'.

If start \< stop, these are replaced by stop-start+1 copies of anObject.
Instead, If start = (stop + 1), then every element of the receiver will
be present in the answered copy; the operation will be an append if stop
is equal to the size of the receiver or, if it is not, an insert before
index 'start'.

[]{#index-copyUpTo_003a}

**copyUpTo: anObject**

Answer a new collection holding all the elements of the receiver from
the first up to the first occurrence of anObject, excluded.

[]{#index-copyUpToLast_003a}

**copyUpToLast: anObject**

Answer a new collection holding all the elements of the receiver from
the first up to the last occurrence of anObject, excluded.

[]{#index-copyWithFirst_003a}

**copyWithFirst: anObject**

Answer a new collection holding all the elements of the receiver after
the first occurrence of anObject, up to the last.

------------------------------------------------------------------------

::: header
Next:
[SequenceableCollection-enumerating](SequenceableCollection_002denumerating.html#SequenceableCollection_002denumerating){accesskey="n"
rel="next"}, Previous:
[SequenceableCollection-concatenating](SequenceableCollection_002dconcatenating.html#SequenceableCollection_002dconcatenating){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
