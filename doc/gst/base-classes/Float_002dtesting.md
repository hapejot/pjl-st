[]{#Float_002dtesting}

::: header
Next: [Float-testing
functionality](Float_002dtesting-functionality.html#Float_002dtesting-functionality){accesskey="n"
rel="next"}, Previous:
[Float-storing](Float_002dstoring.html#Float_002dstoring){accesskey="p"
rel="prev"}, Up: [Float](Float.html#Float){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Float_003a-testing}

#### 1.80.15 Float: testing {#float-testing .subsection}

[]{#index-isExact}

**isExact**

Answer whether the receiver performs exact arithmetic. Floats do not.

[]{#index-isFinite}

**isFinite**

Answer whether the receiver does not represent infinity, nor a NaN

[]{#index-isInfinite}

**isInfinite**

Answer whether the receiver represents positive or negative infinity

[]{#index-isNaN}

**isNaN**

Answer whether the receiver represents a NaN

[]{#index-negative-1}

**negative**

Answer whether the receiver is negative

[]{#index-positive-1}

**positive**

Answer whether the receiver is positive. Negative zero is not positive,
so the definition is not simply \>= 0.

[]{#index-sign}

**sign**

Answer 1 if the receiver is greater than 0, -1 if less than 0, else 0.
Negative zero is the same as positive zero.

[]{#index-strictlyPositive}

**strictlyPositive**

Answer whether the receiver is \> 0
