[]{#ArrayedCollection_002dcopying-Collections}

::: header
Next: [ArrayedCollection-enumerating the elements of a
collection](ArrayedCollection_002denumerating-the-elements-of-a-collection.html#ArrayedCollection_002denumerating-the-elements-of-a-collection){accesskey="n"
rel="next"}, Previous:
[ArrayedCollection-compiler](ArrayedCollection_002dcompiler.html#ArrayedCollection_002dcompiler){accesskey="p"
rel="prev"}, Up:
[ArrayedCollection](ArrayedCollection.html#ArrayedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ArrayedCollection_003a-copying-Collections}

#### 1.5.5 ArrayedCollection: copying Collections {#arrayedcollection-copying-collections .subsection}

[]{#index-copyReplaceAll_003awith_003a}

**copyReplaceAll: oldSubCollection with: newSubCollection**

Answer a new collection in which all the sequences matching
oldSubCollection are replaced with newSubCollection

[]{#index-copyReplaceFrom_003ato_003awith_003a}

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

[]{#index-copyReplaceFrom_003ato_003awithObject_003a}

**copyReplaceFrom: start to: stop withObject: anObject**

Answer a new collection of the same class as the receiver that contains
the same elements as the receiver, in the same order, except for
elements from index 'start' to index 'stop'.

If start \< stop, these are replaced by stop-start+1 copies of anObject.
Instead, If start = (stop + 1), then every element of the receiver will
be present in the answered copy; the operation will be an append if stop
is equal to the size of the receiver or, if it is not, an insert before
index 'start'.

[]{#index-reverse}

**reverse**

Answer the receivers' contents in reverse order
