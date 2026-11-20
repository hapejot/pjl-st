[]{#HashedCollection_002dbuiltins}

::: header
Next:
[HashedCollection-copying](HashedCollection_002dcopying.html#HashedCollection_002dcopying){accesskey="n"
rel="next"}, Previous:
[HashedCollection-accessing](HashedCollection_002daccessing.html#HashedCollection_002daccessing){accesskey="p"
rel="prev"}, Up:
[HashedCollection](HashedCollection.html#HashedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#HashedCollection_003a-builtins}

#### 1.88.3 HashedCollection: builtins {#hashedcollection-builtins .subsection}

[]{#index-primAt_003a}

**primAt: anIndex**

Private - Answer the anIndex-th item of the hash table for the receiver.
Using this instead of basicAt: allows for easier changes in the
representation

[]{#index-primAt_003aput_003a}

**primAt: anIndex put: value**

Private - Store value in the anIndex-th item of the hash table for the
receiver. Using this instead of basicAt:put: allows for easier changes
in the representation

[]{#index-primSize}

**primSize**

Private - Answer the size of the hash table for the receiver. Using this
instead of basicSize allows for easier changes in the representation
