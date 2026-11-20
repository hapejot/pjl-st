[]{#Collection_002dconverting}

::: header
Next: [Collection-copying
Collections](Collection_002dcopying-Collections.html#Collection_002dcopying-Collections){accesskey="n"
rel="next"}, Previous:
[Collection-concatenating](Collection_002dconcatenating.html#Collection_002dconcatenating){accesskey="p"
rel="prev"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection_003a-converting}

#### 1.37.6 Collection: converting {#collection-converting .subsection}

[]{#index-asArray}

**asArray**

Answer an Array containing all the elements in the receiver

[]{#index-asBag}

**asBag**

Answer a Bag containing all the elements in the receiver

[]{#index-asByteArray-1}

**asByteArray**

Answer a ByteArray containing all the elements in the receiver

[]{#index-asOrderedCollection}

**asOrderedCollection**

Answer an OrderedCollection containing all the elements in the receiver

[]{#index-asRunArray} []{#index-do_003a-18}

**asRunArray**

Answer the receiver converted to a RunArray. If the receiver is not
ordered the order of the elements in the RunArray might not be the #do:
order.

[]{#index-asSet-1}

**asSet**

Answer a Set containing all the elements in the receiver with no
duplicates

[]{#index-asSortedCollection}

**asSortedCollection**

Answer a SortedCollection containing all the elements in the receiver
with the default sort block - \[ :a :b \| a \<= b \]

[]{#index-asSortedCollection_003a}

**asSortedCollection: aBlock**

Answer a SortedCollection whose elements are the elements of the
receiver, sorted according to the sort block aBlock

[]{#index-asString-4}

**asString**

Answer a String containing all the elements in the receiver

[]{#index-asUnicodeString-3}

**asUnicodeString**

Answer a UnicodeString containing all the elements in the receiver
