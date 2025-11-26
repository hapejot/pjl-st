[]{#Collection_002denumeration}

::: header
Next:
[Collection-finalization](Collection_002dfinalization.html#Collection_002dfinalization){accesskey="n"
rel="next"}, Previous: [Collection-copying
SequenceableCollections](Collection_002dcopying-SequenceableCollections.html#Collection_002dcopying-SequenceableCollections){accesskey="p"
rel="prev"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection_003a-enumeration}

#### 1.37.9 Collection: enumeration {#collection-enumeration .subsection}

[]{#index-anyOne}

**anyOne**

Answer an unspecified element of the collection.

[]{#index-beConsistent} []{#index-do_003a-19}

**beConsistent**

This method is private, but it is quite interesting so it is documented.
It ensures that a collection is in a consistent state before attempting
to iterate on it; its presence reduces the number of overrides needed by
collections who try to amortize their execution times. The default
implementation does nothing, so it is optimized out by the virtual
machine and so it loses very little on the performance side. Note that
descendants of Collection have to call it explicitly since #do: is
abstract in Collection.

[]{#index-collect_003a-1}

**collect: aBlock**

Answer a new instance of a Collection containing all the results of
evaluating aBlock passing each of the receiver's elements

[]{#index-gather_003a} []{#index-join-1}

**gather: aBlock**

Answer a new instance of a Collection containing all the results of
evaluating aBlock, joined together. aBlock should return collections.
The result is the same kind as the first collection, returned by aBlock
(as for #join).

[]{#index-readStream}

**readStream**

Answer a stream that gives elements of the receiver

[]{#index-reject_003a-1}

**reject: aBlock**

Answer a new instance of a Collection containing all the elements in the
receiver which, when passed to aBlock, don't answer true

[]{#index-select_003a-1}

**select: aBlock**

Answer a new instance of a Collection containing all the elements in the
receiver which, when passed to aBlock, answer true
