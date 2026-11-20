[]{#HashedCollection_002dremoving}

::: header
Next: [HashedCollection-saving and
loading](HashedCollection_002dsaving-and-loading.html#HashedCollection_002dsaving-and-loading){accesskey="n"
rel="next"}, Previous:
[HashedCollection-rehashing](HashedCollection_002drehashing.html#HashedCollection_002drehashing){accesskey="p"
rel="prev"}, Up:
[HashedCollection](HashedCollection.html#HashedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#HashedCollection_003a-removing}

#### 1.88.7 HashedCollection: removing {#hashedcollection-removing .subsection}

[]{#index-remove_003aifAbsent_003a-3}

**remove: oldObject ifAbsent: anExceptionBlock**

Remove oldObject from the set. If it is found, answer oldObject.
Otherwise, evaluate anExceptionBlock and answer its value.
