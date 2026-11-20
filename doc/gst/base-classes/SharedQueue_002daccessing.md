[]{#SharedQueue_002daccessing}

::: header
Previous: [SharedQueue class-instance
creation](SharedQueue-class_002dinstance-creation.html#SharedQueue-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[SharedQueue](SharedQueue.html#SharedQueue){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SharedQueue_003a-accessing}

#### 1.153.2 SharedQueue: accessing {#sharedqueue-accessing .subsection}

[]{#index-isEmpty-6}

**isEmpty**

Answer whether there is an object on the queue

[]{#index-next-7}

**next**

Wait for an object to be on the queue, then remove it and answer it

[]{#index-nextPut_003a-4}

**nextPut: value**

Put value on the queue and answer it

[]{#index-peek-4} []{#index-next-12}

**peek**

Wait for an object to be on the queue if necessary, then answer the same
object that #next would answer without removing it.
