[]{#FloatQ-class_002dcharacterization}

::: header
Next: [FloatQ
class-converting](FloatQ-class_002dconverting.html#FloatQ-class_002dconverting){accesskey="n"
rel="next"}, Previous: [FloatQ class-byte-order
dependancies](FloatQ-class_002dbyte_002dorder-dependancies.html#FloatQ-class_002dbyte_002dorder-dependancies){accesskey="p"
rel="prev"}, Up: [FloatQ](FloatQ.html#FloatQ){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FloatQ-class_003a-characterization}

#### 1.83.2 FloatQ class: characterization {#floatq-class-characterization .subsection}

[]{#index-decimalDigits-2}

**decimalDigits**

Return the number of decimal digits of precision for a FloatQ.
Technically, if P is the precision for the representation, then the
decimal precision Q is the maximum number of decimal digits such that
any floating point number with Q base 10 digits can be rounded to a
floating point number with P base 2 digits and back again, without
change to the Q decimal digits.

[]{#index-e-2}

**e**

Returns the value of e. Hope is that it is precise enough

[]{#index-emax-2}

**emax**

Return the maximum allowable exponent for a FloatQ that is finite.

[]{#index-emin-2}

**emin**

Return the maximum allowable exponent for a FloatQ that is finite.

[]{#index-fmax-2}

**fmax**

Return the largest normalized FloatQ that is not infinite.

[]{#index-fminNormalized-2}

**fminNormalized**

Return the smallest normalized FloatQ that is \> 0

[]{#index-infinity-2}

**infinity**

Return a FloatQ that represents positive infinity.

[]{#index-ln10-2}

**ln10**

Returns the value of ln 10. Hope is that it is precise enough

[]{#index-log10Base2-2}

**log10Base2**

Returns the value of log2 10. Hope is that it is precise enough

[]{#index-nan-2}

**nan**

Return a FloatQ that represents a mathematically indeterminate value
(e.g. Inf - Inf, Inf / Inf).

[]{#index-negativeInfinity-2}

**negativeInfinity**

Return a FloatQ that represents negative infinity.

[]{#index-pi-2}

**pi**

Returns the value of pi. Hope is that it is precise enough

[]{#index-precision-2}

**precision**

Answer the number of bits in the mantissa. 1 + (2\^-precision) = 1

------------------------------------------------------------------------

::: header
Next: [FloatQ
class-converting](FloatQ-class_002dconverting.html#FloatQ-class_002dconverting){accesskey="n"
rel="next"}, Previous: [FloatQ class-byte-order
dependancies](FloatQ-class_002dbyte_002dorder-dependancies.html#FloatQ-class_002dbyte_002dorder-dependancies){accesskey="p"
rel="prev"}, Up: [FloatQ](FloatQ.html#FloatQ){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
