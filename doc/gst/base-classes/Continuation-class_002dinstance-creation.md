[]{#Continuation-class_002dinstance-creation}

::: header
Next:
[Continuation-invocation](Continuation_002dinvocation.html#Continuation_002dinvocation){accesskey="n"
rel="next"}, Up:
[Continuation](Continuation.html#Continuation){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Continuation-class_003a-instance-creation}

#### 1.42.1 Continuation class: instance creation {#continuation-class-instance-creation .subsection}

[]{#index-current}

**current**

Return a continuation.

[]{#index-currentDo_003a}

**currentDo: aBlock**

Pass a continuation to the one-argument block, aBlock and return the
result of evaluating it.

[]{#index-escapeDo_003a}

**escapeDo: aBlock**

Pass a continuation to the one-argument block, knowing that aBlock does
not fall off (either because it includes a method return, or because it
yields control to another continuation). If it does, an exception will
be signalled and the current process terminated.
