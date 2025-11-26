[]{#WeakArray_002daccessing}

::: header
Next:
[WeakArray-conversion](WeakArray_002dconversion.html#WeakArray_002dconversion){accesskey="n"
rel="next"}, Previous: [WeakArray class-instance
creation](WeakArray-class_002dinstance-creation.html#WeakArray-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [WeakArray](WeakArray.html#WeakArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakArray_003a-accessing}

#### 1.215.2 WeakArray: accessing {#weakarray-accessing .subsection}

[]{#index-aliveObjectsDo_003a}

**aliveObjectsDo: aBlock**

Evaluate aBlock for all the elements in the array, excluding the garbage
collected ones. Note: a finalized object stays alive until the next
collection (the collector has no means to see whether it was
resuscitated by the finalizer), so an object being alive does not mean
that it is usable.

[]{#index-at_003a-25}

**at: index**

Answer the index-th item of the receiver, or nil if it has been garbage
collected.

[]{#index-at_003aput_003a-23}

**at: index put: object**

Store the value associated to the given index; plus, store in nilValues
whether the object is nil. nil objects whose associated item of
nilValues is 1 were touched by the garbage collector.

[]{#index-atAll_003aput_003a-1}

**atAll: indices put: object**

Put object at every index contained in the indices collection

[]{#index-atAllPut_003a-1}

**atAllPut: object**

Put object at every index in the receiver

[]{#index-clearGCFlag_003a}

**clearGCFlag: index**

Clear the 'object has been garbage collected' flag for the item at the
given index

[]{#index-do_003a-16}

**do: aBlock**

Evaluate aBlock for all the elements in the array, including the garbage
collected ones (pass nil for those).

[]{#index-isAlive_003a}

**isAlive: index**

Answer whether the item at the given index is still alive or has been
garbage collected. Note: a finalized object stays alive until the next
collection (the collector has no means to see whether it was
resuscitated by the finalizer), so an object being alive does not mean
that it is usable.

[]{#index-size-27}

**size**

Answer the number of items in the receiver

------------------------------------------------------------------------

::: header
Next:
[WeakArray-conversion](WeakArray_002dconversion.html#WeakArray_002dconversion){accesskey="n"
rel="next"}, Previous: [WeakArray class-instance
creation](WeakArray-class_002dinstance-creation.html#WeakArray-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [WeakArray](WeakArray.html#WeakArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
