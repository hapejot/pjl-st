[]{#CCompound-class_002dinstance-creation}

::: header
Next: [CCompound class-subclass
creation](CCompound-class_002dsubclass-creation.html#CCompound-class_002dsubclass-creation){accesskey="n"
rel="next"}, Up: [CCompound](CCompound.html#CCompound){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CCompound-class_003a-instance-creation}

#### 1.24.1 CCompound class: instance creation {#ccompound-class-instance-creation .subsection}

[]{#index-gcNew}

**gcNew**

Allocate a new instance of the receiver, backed by garbage-collected
storage.

[]{#index-new-3} []{#index-addToBeFinalized-2}

**new**

Allocate a new instance of the receiver. To free the memory after GC,
remember to call #addToBeFinalized.
