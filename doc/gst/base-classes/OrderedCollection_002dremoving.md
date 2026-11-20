[]{#OrderedCollection_002dremoving}

::: header
Previous:
[OrderedCollection-enumerating](OrderedCollection_002denumerating.html#OrderedCollection_002denumerating){accesskey="p"
rel="prev"}, Up:
[OrderedCollection](OrderedCollection.html#OrderedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#OrderedCollection_003a-removing}

#### 1.126.6 OrderedCollection: removing {#orderedcollection-removing .subsection}

[]{#index-identityRemove_003a}

**identityRemove: oldObject**

Remove oldObject from the receiver. If absent, fail, else answer
oldObject.

[]{#index-identityRemove_003aifAbsent_003a}

**identityRemove: anObject ifAbsent: aBlock**

Remove anObject from the receiver. If it can't be found, answer the
result of evaluating aBlock

[]{#index-remove_003aifAbsent_003a-6}

**remove: anObject ifAbsent: aBlock**

Remove anObject from the receiver. If it can't be found, answer the
result of evaluating aBlock

[]{#index-removeAtIndex_003a}

**removeAtIndex: anIndex**

Remove the object at index anIndex from the receiver. Fail if the index
is out of bounds.

[]{#index-removeFirst-1}

**removeFirst**

Remove an object from the start of the receiver. Fail if the receiver is
empty

[]{#index-removeLast-1}

**removeLast**

Remove an object from the end of the receiver. Fail if the receiver is
empty
