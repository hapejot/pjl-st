[]{#Collection_002dremoving}

::: header
Next:
[Collection-sorting](Collection_002dsorting.html#Collection_002dsorting){accesskey="n"
rel="next"}, Previous:
[Collection-printing](Collection_002dprinting.html#Collection_002dprinting){accesskey="p"
rel="prev"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection_003a-removing}

#### 1.37.12 Collection: removing {#collection-removing .subsection}

[]{#index-empty}

**empty**

Remove everything from the receiver.

[]{#index-remove_003a}

**remove: oldObject**

Remove oldObject from the receiver. If absent, fail, else answer
oldObject.

[]{#index-remove_003aifAbsent_003a-1}

**remove: oldObject ifAbsent: anExceptionBlock**

Remove oldObject from the receiver. If absent, evaluate anExceptionBlock
and answer the result, else answer oldObject.

[]{#index-removeAll_003a}

**removeAll: aCollection**

Remove each object in aCollection, answer aCollection, fail if some of
them is absent. Warning: this could leave the collection in a
semi-updated state.

[]{#index-removeAll_003aifAbsent_003a}

**removeAll: aCollection ifAbsent: aBlock**

Remove each object in aCollection, answer aCollection; if some element
is absent, pass it to aBlock.

[]{#index-removeAllSuchThat_003a}

**removeAllSuchThat: aBlock**

Remove from the receiver all objects for which aBlock returns true.
