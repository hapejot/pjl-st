[]{#MappedCollection_002dbasic}

::: header
Previous: [MappedCollection class-instance
creation](MappedCollection-class_002dinstance-creation.html#MappedCollection-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[MappedCollection](MappedCollection.html#MappedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#MappedCollection_003a-basic}

#### 1.108.2 MappedCollection: basic {#mappedcollection-basic .subsection}

[]{#index-add_003a-7}

**add: anObject**

This method should not be called for instances of this class.

[]{#index-at_003a-13}

**at: key**

Answer the object at the given key

[]{#index-at_003aput_003a-12}

**at: key put: value**

Store value at the given key

[]{#index-atAll_003a-2}

**atAll: keyCollection**

Answer a new MappedCollection that only includes the given keys. The new
MappedCollection might use keyCollection or consecutive integers for the
keys, depending on the map's type. Fail if any of them is not found in
the map.

[]{#index-collect_003a-5}

**collect: aBlock**

Answer a Collection with the same keys as the map, where accessing a key
yields the value obtained by passing through aBlock the value accessible
from the key in the receiver. The result need not be another
MappedCollection

[]{#index-contents-2}

**contents**

Answer a bag with the receiver's values

[]{#index-copyFrom_003ato_003a-5}

**copyFrom: a to: b**

Answer a new collection containing all the items in the receiver from
the a-th to the b-th.

[]{#index-do_003a-9}

**do: aBlock**

Evaluate aBlock for each object

[]{#index-domain}

**domain**

Answer the receiver's domain

[]{#index-keys-1}

**keys**

Answer the keys that can be used to access this collection.

[]{#index-keysAndValuesDo_003a-2}

**keysAndValuesDo: aBlock**

Evaluate aBlock passing two arguments, one being a key that can be used
to access this collection, and the other one being the value.

[]{#index-keysDo_003a-2}

**keysDo: aBlock**

Evaluate aBlock on the keys that can be used to access this collection.

[]{#index-map}

**map**

Answer the receiver's map

[]{#index-reject_003a-5}

**reject: aBlock**

Answer the objects in the domain for which aBlock returns false

[]{#index-select_003a-5}

**select: aBlock**

Answer the objects in the domain for which aBlock returns true

[]{#index-size-16}

**size**

Answer the receiver's size

------------------------------------------------------------------------

::: header
Previous: [MappedCollection class-instance
creation](MappedCollection-class_002dinstance-creation.html#MappedCollection-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[MappedCollection](MappedCollection.html#MappedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
