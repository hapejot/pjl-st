[]{#ExceptionSet_002denumerating}

::: header
Next: [ExceptionSet-instance
creation](ExceptionSet_002dinstance-creation.html#ExceptionSet_002dinstance-creation){accesskey="n"
rel="next"}, Previous: [ExceptionSet class-instance
creation](ExceptionSet-class_002dinstance-creation.html#ExceptionSet-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[ExceptionSet](ExceptionSet.html#ExceptionSet){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ExceptionSet_003a-enumerating}

#### 1.73.2 ExceptionSet: enumerating {#exceptionset-enumerating .subsection}

[]{#index-allExceptionsDo_003a-1}

**allExceptionsDo: aBlock**

Private - Evaluate aBlock for every exception in the receiver. Answer
the receiver

[]{#index-goodness_003a-1}

**goodness: exception**

Answer how good the receiver is at handling the given exception. A
negative value indicates that the receiver is not able to handle the
exception.

[]{#index-handles_003a-1}

**handles: exception**

Answer whether the receiver handles 'exception'.
