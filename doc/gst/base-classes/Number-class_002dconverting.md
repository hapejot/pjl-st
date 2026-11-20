[]{#Number-class_002dconverting}

::: header
Next: [Number
class-testing](Number-class_002dtesting.html#Number-class_002dtesting){accesskey="n"
rel="next"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number-class_003a-converting}

#### 1.122.1 Number class: converting {#number-class-converting .subsection}

[]{#index-coerce_003a-11}

**coerce: aNumber**

Answer aNumber - whatever class it belongs to, it is good

[]{#index-readFrom_003a-3}

**readFrom: aStream**

Answer the number read from the rest of aStream, converted to an
instance of the receiver. If the receiver is number, the class of the
result is undefined -- but the result is good.

[]{#index-readFrom_003aradix_003a}

**readFrom: aStream radix: anInteger**

Answer the number read from the rest of aStream, converted to an
instance of the receiver. If the receiver is number, the class of the
result is undefined -- but the result is good.

The exponent (for example 1.2e-1) is only parsed if anInteger is 10.
