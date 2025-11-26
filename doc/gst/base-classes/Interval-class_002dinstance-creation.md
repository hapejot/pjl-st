[]{#Interval-class_002dinstance-creation}

::: header
Next:
[Interval-basic](Interval_002dbasic.html#Interval_002dbasic){accesskey="n"
rel="next"}, Up: [Interval](Interval.html#Interval){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Interval-class_003a-instance-creation}

#### 1.93.1 Interval class: instance creation {#interval-class-instance-creation .subsection}

[]{#index-from_003ato_003a}

**from: startInteger to: stopInteger**

Answer an Interval going from startInteger to the stopInteger, with a
step of 1

[]{#index-from_003ato_003aby_003a}

**from: startInteger to: stopInteger by: stepInteger**

Answer an Interval going from startInteger to the stopInteger, with a
step of stepInteger

[]{#index-withAll_003a-3}

**withAll: aCollection**

Answer an Interval containing the same elements as aCollection. Fail if
it is not possible to create one.
