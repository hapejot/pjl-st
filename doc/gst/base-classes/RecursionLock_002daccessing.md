[]{#RecursionLock_002daccessing}

::: header
Next: [RecursionLock-mutual
exclusion](RecursionLock_002dmutual-exclusion.html#RecursionLock_002dmutual-exclusion){accesskey="n"
rel="next"}, Previous: [RecursionLock class-instance
creation](RecursionLock-class_002dinstance-creation.html#RecursionLock-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[RecursionLock](RecursionLock.html#RecursionLock){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RecursionLock_003a-accessing}

#### 1.143.2 RecursionLock: accessing {#recursionlock-accessing .subsection}

[]{#index-isOwnerProcess}

**isOwnerProcess**

Answer whether the receiver is the owner of the lock.

[]{#index-name-12}

**name**

Answer a user-defined name for the lock.

[]{#index-name_003a-5}

**name: aString**

Set to aString the user-defined name for the lock.

[]{#index-waitingProcesses}

**waitingProcesses**

Answer the set of processes that are waiting on the semaphore.

[]{#index-wouldBlock} []{#index-wait-3}

**wouldBlock**

Answer whether sending #wait to the receiver would suspend the active
process.
