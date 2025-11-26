[]{#Number_002dtesting}

::: header
Next: [Number-truncation and round
off](Number_002dtruncation-and-round-off.html#Number_002dtruncation-and-round-off){accesskey="n"
rel="next"}, Previous: [Number-shortcuts and
iterators](Number_002dshortcuts-and-iterators.html#Number_002dshortcuts-and-iterators){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-testing}

#### 1.122.13 Number: testing {#number-testing .subsection}

[]{#index-closeTo_003a}

**closeTo: num**

Answer whether the receiver can be considered sufficiently close to num
(this is done by checking equality if num is not a number, and by
checking with 0.01% tolerance if num is a number).

[]{#index-even-1}

**even**

Returns true if self is divisible by 2

[]{#index-isExact-2} []{#index-subclassResponsibility-2}

**isExact**

Answer whether the receiver performs exact arithmetic. Most numeric
classes do (in fact the only exceptions is Float and its descendants),
so the default is to answer true rather than calling
#subclassResponsibility.

[]{#index-isFinite-1} []{#index-subclassResponsibility-3}

**isFinite**

Answer whether the receiver represents a finite quantity. Most numeric
classes are for finite quantities, so the default is to answer true
rather than calling #subclassResponsibility.

[]{#index-isInfinite-1} []{#index-subclassResponsibility-4}

**isInfinite**

Answer whether the receiver represents an infinite quantity. Most
numeric classes are for finite quantities, so the default is to answer
false rather than calling #subclassResponsibility.

[]{#index-isNaN-1} []{#index-subclassResponsibility-5}

**isNaN**

Answer whether the receiver is a Not-A-Number. Most numeric classes
don't handle nans, so the default is to answer false rather than calling
#subclassResponsibility.

[]{#index-isNumber}

**isNumber**

Answer 'true'.

[]{#index-isRational-2}

**isRational**

Answer whether the receiver is rational - false by default

[]{#index-negative-4}

**negative**

Answer whether the receiver is \< 0

[]{#index-odd-1}

**odd**

Returns true if self is not divisible by 2

[]{#index-positive-4}

**positive**

Answer whether the receiver is \>= 0

[]{#index-sign-4}

**sign**

Returns the sign of the receiver.

[]{#index-strictlyPositive-4}

**strictlyPositive**

Answer whether the receiver is \> 0

------------------------------------------------------------------------

::: header
Next: [Number-truncation and round
off](Number_002dtruncation-and-round-off.html#Number_002dtruncation-and-round-off){accesskey="n"
rel="next"}, Previous: [Number-shortcuts and
iterators](Number_002dshortcuts-and-iterators.html#Number_002dshortcuts-and-iterators){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
