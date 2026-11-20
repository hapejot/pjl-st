[]{#HashedCollection_002dsaving-and-loading}

::: header
Next:
[HashedCollection-storing](HashedCollection_002dstoring.html#HashedCollection_002dstoring){accesskey="n"
rel="next"}, Previous:
[HashedCollection-removing](HashedCollection_002dremoving.html#HashedCollection_002dremoving){accesskey="p"
rel="prev"}, Up:
[HashedCollection](HashedCollection.html#HashedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#HashedCollection_003a-saving-and-loading}

#### 1.88.8 HashedCollection: saving and loading {#hashedcollection-saving-and-loading .subsection}

[]{#index-postLoad}

**postLoad**

Called after loading an object; rehash the collection because identity
objects will most likely mutate their hashes.

[]{#index-postStore} []{#index-postLoad-6}

**postStore**

Called after an object is dumped. Do nothing -- necessary because by
default this calls #postLoad by default
