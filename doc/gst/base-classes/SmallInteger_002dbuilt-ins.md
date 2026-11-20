[]{#SmallInteger_002dbuilt-ins}

::: header
Next:
[SmallInteger-builtins](SmallInteger_002dbuiltins.html#SmallInteger_002dbuiltins){accesskey="n"
rel="next"}, Previous: [SmallInteger-bit
arithmetic](SmallInteger_002dbit-arithmetic.html#SmallInteger_002dbit-arithmetic){accesskey="p"
rel="prev"}, Up:
[SmallInteger](SmallInteger.html#SmallInteger){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SmallInteger_003a-built-ins}

#### 1.155.4 SmallInteger: built ins {#smallinteger-built-ins .subsection}

[]{#index-_002a-11}

**\* arg**

Multiply the receiver and arg and answer another Number

[]{#index-_002b-16}

**+ arg**

Sum the receiver and arg and answer another Number

[]{#index-_002d-16}

**- arg**

Subtract arg from the receiver and answer another Number

[]{#index-_002f-11}

**/ arg**

Divide the receiver by arg and answer another Integer or Fraction

[]{#index-_002f_002f-6}

**// arg**

Dividing receiver by arg (with truncation towards -infinity) and answer
the result

[]{#index-_003c-14}

**\< arg**

Answer whether the receiver is less than arg

[]{#index-_003c_003d-11}

**\<= arg**

Answer whether the receiver is less than or equal to arg

[]{#index-_003d-39}

**= arg**

Answer whether the receiver is equal to arg

[]{#index-_003d_003d-1}

**== arg**

Answer whether the receiver is the same object as arg

[]{#index-_003e-11}

**\> arg**

Answer whether the receiver is greater than arg

[]{#index-_003e_003d-11}

**\>= arg**

Answer whether the receiver is greater than or equal to arg

[]{#index-_005c_005c-5}

**\\\\ arg**

Calculate the remainder of dividing receiver by arg (with truncation
towards -infinity) and answer it

[]{#index-asFloatD-8}

**asFloatD**

Convert the receiver to a FloatD, answer the result

[]{#index-asFloatE-8}

**asFloatE**

Convert the receiver to a FloatE, answer the result

[]{#index-asFloatQ-8}

**asFloatQ**

Convert the receiver to a FloatQ, answer the result

[]{#index-asObject-1}

**asObject**

Answer the object whose index is in the receiver, nil if there is a free
object, fail if index is out of bounds

[]{#index-asObjectNoFail-1}

**asObjectNoFail**

Answer the object whose index is in the receiver, or nil if no object is
found at that index

[]{#index-bitAnd_003a-1}

**bitAnd: arg**

Do a bitwise AND between the receiver and arg, answer the result

[]{#index-bitOr_003a-1}

**bitOr: arg**

Do a bitwise OR between the receiver and arg, answer the result

[]{#index-bitShift_003a-1}

**bitShift: arg**

Shift the receiver by arg places to the left if arg \> 0, by arg places
to the right if arg \< 0, answer another Number

[]{#index-bitXor_003a-1}

**bitXor: arg**

Do a bitwise XOR between the receiver and arg, answer the result

[]{#index-divExact_003a-1}

**divExact: arg**

Dividing receiver by arg assuming that the remainder is zero, and answer
the result

[]{#index-nextValidOop}

**nextValidOop**

Answer the index of the first non-free OOP after the receiver. This is
used internally; it is placed here to avoid polluting Object.

[]{#index-quo_003a-3}

**quo: arg**

Dividing receiver by arg (with truncation towards zero) and answer the
result

[]{#index-_007e_003d-6}

**\~= arg**

Answer whether the receiver is not equal to arg

[]{#index-_007e_007e-1}

**\~\~ arg**

Answer whether the receiver is not the same object as arg

------------------------------------------------------------------------

::: header
Next:
[SmallInteger-builtins](SmallInteger_002dbuiltins.html#SmallInteger_002dbuiltins){accesskey="n"
rel="next"}, Previous: [SmallInteger-bit
arithmetic](SmallInteger_002dbit-arithmetic.html#SmallInteger_002dbit-arithmetic){accesskey="p"
rel="prev"}, Up:
[SmallInteger](SmallInteger.html#SmallInteger){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
