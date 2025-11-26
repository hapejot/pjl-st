[]{#ArrayedCollection_002dbasic}

::: header
Next: [ArrayedCollection-built
ins](ArrayedCollection_002dbuilt-ins.html#ArrayedCollection_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [ArrayedCollection class-instance
creation](ArrayedCollection-class_002dinstance-creation.html#ArrayedCollection-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[ArrayedCollection](ArrayedCollection.html#ArrayedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ArrayedCollection_003a-basic}

#### 1.5.2 ArrayedCollection: basic {#arrayedcollection-basic .subsection}

[]{#index-_002c}

**, aSequenceableCollection**

Answer a new instance of an ArrayedCollection containing all the
elements in the receiver, followed by all the elements in
aSequenceableCollection

[]{#index-add_003a}

**add: value**

This method should not be called for instances of this class.

[]{#index-atAll_003a} []{#index-collect_003a-7}

**atAll: keyCollection**

Answer a collection of the same kind returned by #collect:, that only
includes the values at the given indices. Fail if any of the values in
keyCollection is out of bounds for the receiver.

[]{#index-copyFrom_003ato_003a}

**copyFrom: start to: stop**

Answer a new collection containing all the items in the receiver from
the start-th and to the stop-th

[]{#index-copyWith_003a}

**copyWith: anElement**

Answer a new instance of an ArrayedCollection containing all the
elements in the receiver, followed by the single item anElement

[]{#index-copyWithout_003a}

**copyWithout: oldElement**

Answer a copy of the receiver to which all occurrences of oldElement are
removed
