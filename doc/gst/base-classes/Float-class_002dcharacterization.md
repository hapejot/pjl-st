[]{#Float-class_002dcharacterization}

::: header
Next:
[Float-arithmetic](Float_002darithmetic.html#Float_002darithmetic){accesskey="n"
rel="next"}, Previous: [Float class-byte-order
dependancies](Float-class_002dbyte_002dorder-dependancies.html#Float-class_002dbyte_002dorder-dependancies){accesskey="p"
rel="prev"}, Up: [Float](Float.html#Float){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Float-class_003a-characterization}

#### 1.80.2 Float class: characterization {#float-class-characterization .subsection}

[]{#index-denormalized}

**denormalized**

Answer whether instances of the receiver can be in denormalized form.

[]{#index-e}

**e**

Returns the value of e. Hope is that it is precise enough

[]{#index-epsilon}

**epsilon**

Return the smallest Float x for which is 1 + x \~= 1

[]{#index-fmin}

**fmin**

Return the smallest Float that is \> 0.

[]{#index-fminDenormalized}

**fminDenormalized**

Return the smallest Float that is \> 0 if denormalized values are
supported, else return 0.

[]{#index-ln10}

**ln10**

Returns the value of ln 10. Hope is that it is precise enough

[]{#index-log10Base2}

**log10Base2**

Returns the value of log2 10. Hope is that it is precise enough

[]{#index-pi}

**pi**

Returns the value of pi. Hope is that it is precise enough

[]{#index-radix}

**radix**

Answer the base in which computations between instances of the receiver
are made. This should be 2 on about every known computer, so GNU
Smalltalk always answers 2.
