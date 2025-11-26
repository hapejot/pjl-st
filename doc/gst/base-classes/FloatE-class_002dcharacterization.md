[]{#FloatE-class_002dcharacterization}

::: header
Next: [FloatE
class-converting](FloatE-class_002dconverting.html#FloatE-class_002dconverting){accesskey="n"
rel="next"}, Previous: [FloatE class-byte-order
dependencies](FloatE-class_002dbyte_002dorder-dependencies.html#FloatE-class_002dbyte_002dorder-dependencies){accesskey="p"
rel="prev"}, Up: [FloatE](FloatE.html#FloatE){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FloatE-class_003a-characterization}

#### 1.82.3 FloatE class: characterization {#floate-class-characterization .subsection}

[]{#index-decimalDigits-1}

**decimalDigits**

Return the number of decimal digits of precision for a FloatE.
Technically, if P is the precision for the representation, then the
decimal precision Q is the maximum number of decimal digits such that
any floating point number with Q base 10 digits can be rounded to a
floating point number with P base 2 digits and back again, without
change to the Q decimal digits.

[]{#index-e-1}

**e**

Returns the value of e. Hope is that it is precise enough

[]{#index-emax-1}

**emax**

Return the maximum allowable exponent for a FloatE that is finite.

[]{#index-emin-1}

**emin**

Return the maximum allowable exponent for a FloatE that is finite.

[]{#index-fmax-1}

**fmax**

Return the largest normalized FloatE that is not infinite.

[]{#index-fminNormalized-1}

**fminNormalized**

Return the smallest normalized FloatE that is \> 0

[]{#index-infinity-1}

**infinity**

Return a FloatE that represents positive infinity.

[]{#index-ln10-1}

**ln10**

Returns the value of ln 10. Hope is that it is precise enough

[]{#index-log10Base2-1}

**log10Base2**

Returns the value of log2 10. Hope is that it is precise enough

[]{#index-nan-1}

**nan**

Return a FloatE that represents a mathematically indeterminate value
(e.g. Inf - Inf, Inf / Inf).

[]{#index-negativeInfinity-1}

**negativeInfinity**

Return a FloatE that represents negative infinity.

[]{#index-pi-1}

**pi**

Returns the value of pi. Hope is that it is precise enough

[]{#index-precision-1}

**precision**

Answer the number of bits in the mantissa. 1 + (2\^-precision) = 1

------------------------------------------------------------------------

::: header
Next: [FloatE
class-converting](FloatE-class_002dconverting.html#FloatE-class_002dconverting){accesskey="n"
rel="next"}, Previous: [FloatE class-byte-order
dependencies](FloatE-class_002dbyte_002dorder-dependencies.html#FloatE-class_002dbyte_002dorder-dependencies){accesskey="p"
rel="prev"}, Up: [FloatE](FloatE.html#FloatE){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
