[]{#Number_002dretrying}

::: header
Next: [Number-shortcuts and
iterators](Number_002dshortcuts-and-iterators.html#Number_002dshortcuts-and-iterators){accesskey="n"
rel="next"}, Previous: [Number-point
creation](Number_002dpoint-creation.html#Number_002dpoint-creation){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-retrying}

#### 1.122.11 Number: retrying {#number-retrying .subsection}

[]{#index-retry_003acoercing_003a} []{#index-_003d-46}
[]{#index-_007e_003d-7}

**retry: aSymbol coercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling aSymbol. aSymbol is
supposed not to be #= or #\~= (since those don't fail if aNumber is not
a Number).

[]{#index-retryDifferenceCoercing_003a} []{#index-_002d-17}

**retryDifferenceCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #-.

[]{#index-retryDivisionCoercing_003a} []{#index-_002f-13}

**retryDivisionCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #/.

[]{#index-retryEqualityCoercing_003a} []{#index-_003d-47}

**retryEqualityCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #=.

[]{#index-retryError}

**retryError**

Raise an error---a retrying method was called with two arguments having
the same generality.

[]{#index-retryInequalityCoercing_003a} []{#index-_007e_003d-8}

**retryInequalityCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #\~=.

[]{#index-retryMultiplicationCoercing_003a} []{#index-_002a-13}

**retryMultiplicationCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #\*.

[]{#index-retryRelationalOp_003acoercing_003a}

**retryRelationalOp: aSymbol coercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling aSymbol (\<, \<=,
\>, \>=).

[]{#index-retrySumCoercing_003a} []{#index-_002b-17}

**retrySumCoercing: aNumber**

Coerce to the other number's class the one number between the receiver
and aNumber which has the lowest, and retry calling #+.

------------------------------------------------------------------------

::: header
Next: [Number-shortcuts and
iterators](Number_002dshortcuts-and-iterators.html#Number_002dshortcuts-and-iterators){accesskey="n"
rel="next"}, Previous: [Number-point
creation](Number_002dpoint-creation.html#Number_002dpoint-creation){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
