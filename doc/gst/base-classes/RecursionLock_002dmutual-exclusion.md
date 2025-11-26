[]{#RecursionLock_002dmutual-exclusion}

::: header
Next:
[RecursionLock-printing](RecursionLock_002dprinting.html#RecursionLock_002dprinting){accesskey="n"
rel="next"}, Previous:
[RecursionLock-accessing](RecursionLock_002daccessing.html#RecursionLock_002daccessing){accesskey="p"
rel="prev"}, Up:
[RecursionLock](RecursionLock.html#RecursionLock){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RecursionLock_003a-mutual-exclusion}

#### 1.143.3 RecursionLock: mutual exclusion {#recursionlock-mutual-exclusion .subsection}

[]{#index-critical_003a}

**critical: aBlock**

Wait for the receiver to be free, execute aBlock and signal the receiver
again. Return the result of evaluating aBlock.
