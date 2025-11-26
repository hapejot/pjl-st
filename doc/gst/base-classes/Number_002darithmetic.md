[]{#Number_002darithmetic}

::: header
Next:
[Number-coercion](Number_002dcoercion.html#Number_002dcoercion){accesskey="n"
rel="next"}, Previous: [Number
class-testing](Number-class_002dtesting.html#Number-class_002dtesting){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-arithmetic}

#### 1.122.3 Number: arithmetic {#number-arithmetic .subsection}

[]{#index-_002a-8}

**\* aNumber**

Subtract the receiver and aNumber, answer the result

[]{#index-_002b-12}

**+ aNumber**

Sum the receiver and aNumber, answer the result

[]{#index-_002d-12}

**- aNumber**

Subtract aNumber from the receiver, answer the result

[]{#index-_002f-8}

**/ aNumber**

Divide the receiver by aNumber, answer the result (no loss of
precision). Raise a ZeroDivide exception or return a valid (possibly
infinite) continuation value if aNumber is zero.

[]{#index-_002f_002f-3}

**// aNumber**

Return the integer quotient of dividing the receiver by aNumber with
truncation towards negative infinity. Raise a ZeroDivide exception if
aNumber is zero

[]{#index-_005c_005c-3}

**\\\\ aNumber**

Return the remainder of dividing the receiver by aNumber with truncation
towards negative infinity. Raise a ZeroDivide exception if aNumber is
zero

[]{#index-quo_003a-2}

**quo: aNumber**

Return the integer quotient of dividing the receiver by aNumber with
truncation towards zero. Raise a ZeroDivide exception if aNumber is zero

[]{#index-reciprocal-1}

**reciprocal**

Return the reciprocal of the receiver

[]{#index-rem_003a-2}

**rem: aNumber**

Return the remainder of dividing the receiver by aNumber with truncation
towards zero. Raise a ZeroDivide exception if aNumber is zero
