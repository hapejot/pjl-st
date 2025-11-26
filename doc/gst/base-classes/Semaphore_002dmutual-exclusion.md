[]{#Semaphore_002dmutual-exclusion}

::: header
Next:
[Semaphore-printing](Semaphore_002dprinting.html#Semaphore_002dprinting){accesskey="n"
rel="next"}, Previous:
[Semaphore-builtins](Semaphore_002dbuiltins.html#Semaphore_002dbuiltins){accesskey="p"
rel="prev"}, Up: [Semaphore](Semaphore.html#Semaphore){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Semaphore_003a-mutual-exclusion}

#### 1.150.4 Semaphore: mutual exclusion {#semaphore-mutual-exclusion .subsection}

[]{#index-critical_003a-1}

**critical: aBlock**

Wait for the receiver to be free, execute aBlock and signal the receiver
again. Return the result of evaluating aBlock.
