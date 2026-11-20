[]{#FloatD-class_002dcharacterization}

::: header
Next: [FloatD
class-converting](FloatD-class_002dconverting.html#FloatD-class_002dconverting){accesskey="n"
rel="next"}, Previous: [FloatD class-byte-order
dependencies](FloatD-class_002dbyte_002dorder-dependencies.html#FloatD-class_002dbyte_002dorder-dependencies){accesskey="p"
rel="prev"}, Up: [FloatD](FloatD.html#FloatD){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FloatD-class_003a-characterization}

#### 1.81.2 FloatD class: characterization {#floatd-class-characterization .subsection}

[]{#index-decimalDigits}

**decimalDigits**

Return the number of decimal digits of precision for a FloatD.
Technically, if P is the precision for the representation, then the
decimal precision Q is the maximum number of decimal digits such that
any floating point number with Q base 10 digits can be rounded to a
floating point number with P base 2 digits and back again, without
change to the Q decimal digits.

[]{#index-emax}

**emax**

Return the maximum allowable exponent for a FloatD that is finite.

[]{#index-emin}

**emin**

Return the maximum allowable exponent for a FloatD that is finite.

[]{#index-fmax}

**fmax**

Return the largest normalized FloatD that is not infinite.

[]{#index-fminNormalized}

**fminNormalized**

Return the smallest normalized FloatD that is \> 0

[]{#index-infinity}

**infinity**

Return a FloatD that represents positive infinity.

[]{#index-nan}

**nan**

Return a FloatD that represents a mathematically indeterminate value
(e.g. Inf - Inf, Inf / Inf).

[]{#index-negativeInfinity}

**negativeInfinity**

Return a FloatD that represents negative infinity.

[]{#index-precision}

**precision**

Answer the number of bits in the mantissa. 1 + (2\^-precision) = 1
