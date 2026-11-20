[]{#Fraction_002darithmetic}

::: header
Next:
[Fraction-coercing](Fraction_002dcoercing.html#Fraction_002dcoercing){accesskey="n"
rel="next"}, Previous:
[Fraction-accessing](Fraction_002daccessing.html#Fraction_002daccessing){accesskey="p"
rel="prev"}, Up: [Fraction](Fraction.html#Fraction){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Fraction_003a-arithmetic}

#### 1.84.4 Fraction: arithmetic {#fraction-arithmetic .subsection}

[]{#index-_002a-5}

**\* aNumber**

Multiply two numbers and answer the result.

[]{#index-_002b-7}

**+ aNumber**

Sum two numbers and answer the result.

[]{#index-_002d-7}

**- aNumber**

Subtract aNumber from the receiver and answer the result.

[]{#index-_002f-5}

**/ aNumber**

Divide the receiver by aNumber and answer the result.

[]{#index-_002f_002f}

**// aNumber**

Return the integer quotient of dividing the receiver by aNumber with
truncation towards negative infinity.

[]{#index-_005c_005c}

**\\\\ aNumber**

Return the remainder from dividing the receiver by aNumber, (using //).

[]{#index-estimatedLog-1}

**estimatedLog**

Answer an estimate of (self abs floorLog: 10)
