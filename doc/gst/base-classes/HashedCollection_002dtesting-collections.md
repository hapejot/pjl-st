[]{#HashedCollection_002dtesting-collections}

::: header
Previous:
[HashedCollection-storing](HashedCollection_002dstoring.html#HashedCollection_002dstoring){accesskey="p"
rel="prev"}, Up:
[HashedCollection](HashedCollection.html#HashedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#HashedCollection_003a-testing-collections}

#### 1.88.10 HashedCollection: testing collections {#hashedcollection-testing-collections .subsection}

[]{#index-_003d-25}

**= aHashedCollection**

Returns true if the two sets have the same membership, false if not

[]{#index-capacity-1}

**capacity**

Answer how many elements the receiver can hold before having to grow.

[]{#index-hash-18}

**hash**

Return the hash code for the members of the set. Since order is
unimportant, we use a commutative operator to compute the hash value.

[]{#index-includes_003a-4}

**includes: anObject**

Answer whether the receiver contains an instance of anObject.

[]{#index-isEmpty-2}

**isEmpty**

Answer whether the receiver is empty.

[]{#index-occurrencesOf_003a-3}

**occurrencesOf: anObject**

Return the number of occurrences of anObject. Since we're a set, this is
either 0 or 1. Nil is never directly in the set, so we special case it
(the result is always 1).

[]{#index-size-9}

**size**

Answer the receiver's size
