[]{#LargePositiveInteger_002dprimitive-operations}

::: header
Previous: [LargePositiveInteger-numeric
testing](LargePositiveInteger_002dnumeric-testing.html#LargePositiveInteger_002dnumeric-testing){accesskey="p"
rel="prev"}, Up:
[LargePositiveInteger](LargePositiveInteger.html#LargePositiveInteger){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LargePositiveInteger_003a-primitive-operations}

#### 1.100.5 LargePositiveInteger: primitive operations {#largepositiveinteger-primitive-operations .subsection}

[]{#index-divide_003ausing_003a}

**divide: aNumber using: aBlock**

Private - Divide the receiver by aNumber (unsigned division). Evaluate
aBlock passing the result ByteArray, the remainder ByteArray, and
whether the division had a remainder

[]{#index-isSmall}

**isSmall**

Private - Answer whether the receiver is small enough to employ simple
scalar algorithms for division and multiplication

[]{#index-multiply_003a}

**multiply: aNumber**

Private - Multiply the receiver by aNumber (unsigned multiply)
