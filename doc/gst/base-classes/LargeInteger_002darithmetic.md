[]{#LargeInteger_002darithmetic}

::: header
Next: [LargeInteger-bit
operations](LargeInteger_002dbit-operations.html#LargeInteger_002dbit-operations){accesskey="n"
rel="next"}, Previous:
[LargeInteger-accessing](LargeInteger_002daccessing.html#LargeInteger_002daccessing){accesskey="p"
rel="prev"}, Up:
[LargeInteger](LargeInteger.html#LargeInteger){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LargeInteger_003a-arithmetic}

#### 1.98.2 LargeInteger: arithmetic {#largeinteger-arithmetic .subsection}

[]{#index-_002a-6}

**\* aNumber**

Multiply aNumber and the receiver, answer the result

[]{#index-_002b-8}

**+ aNumber**

Sum the receiver and aNumber, answer the result

[]{#index-_002d-8}

**- aNumber**

Subtract aNumber from the receiver, answer the result

[]{#index-_002f-6}

**/ aNumber**

Divide aNumber and the receiver, answer the result (an Integer or
Fraction)

[]{#index-_002f_002f-1}

**// aNumber**

Divide aNumber and the receiver, answer the result truncated towards
-infinity

[]{#index-_005c_005c-1}

**\\\\ aNumber**

Divide aNumber and the receiver, answer the remainder truncated towards
-infinity

[]{#index-divExact_003a}

**divExact: aNumber**

Dividing receiver by arg assuming that the remainder is zero, and answer
the result

[]{#index-estimatedLog-3}

**estimatedLog**

Answer an estimate of (self abs floorLog: 10)

[]{#index-negated-3}

**negated**

Answer the receiver's negated

[]{#index-quo_003a}

**quo: aNumber**

Divide aNumber and the receiver, answer the result truncated towards 0

[]{#index-rem_003a}

**rem: aNumber**

Divide aNumber and the receiver, answer the remainder truncated towards
0
