[]{#Duration_002darithmetics}

::: header
Next:
[Duration-processes](Duration_002dprocesses.html#Duration_002dprocesses){accesskey="n"
rel="next"}, Previous: [Duration class-instance creation (non
ANSI)](Duration-class_002dinstance-creation-_0028non-ANSI_0029.html#Duration-class_002dinstance-creation-_0028non-ANSI_0029){accesskey="p"
rel="prev"}, Up: [Duration](Duration.html#Duration){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Duration_003a-arithmetics}

#### 1.69.3 Duration: arithmetics {#duration-arithmetics .subsection}

[]{#index-_002a-1}

**\* factor**

Answer a Duration that is 'factor' times longer than the receiver

[]{#index-_002b-3}

**+ aDuration**

Answer a Duration that is the sum of the receiver and aDuration's
lengths.

[]{#index-_002d-3}

**- aDuration**

Answer a Duration that is the difference of the receiver and aDuration's
lengths.

[]{#index-_002f}

**/ factorOrDuration**

If the parameter is a Duration, answer the ratio between the receiver
and factorOrDuration. Else divide the receiver by factorOrDuration (a
Number) and answer a new Duration that is correspondingly shorter.

[]{#index-abs}

**abs**

Answer a Duration that is as long as the receiver, but always in the
future.

[]{#index-days}

**days**

Answer the number of days in the receiver

[]{#index-isZero}

**isZero**

Answer whether the receiver correspond to a duration of zero seconds.

[]{#index-negated}

**negated**

Answer a Duration that is as long as the receiver, but with past and
future exchanged.

[]{#index-negative}

**negative**

Answer whether the receiver is in the past.

[]{#index-positive}

**positive**

Answer whether the receiver is a zero-second duration or is in the
future.

[]{#index-printOn_003a-17}

**printOn: aStream**

Print a represention of the receiver on aStream.
