[]{#Number_002dconverting}

::: header
Next:
[Number-copying](Number_002dcopying.html#Number_002dcopying){accesskey="n"
rel="next"}, Previous:
[Number-comparing](Number_002dcomparing.html#Number_002dcomparing){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-converting}

#### 1.122.6 Number: converting {#number-converting .subsection}

[]{#index-asExactFraction-2}

**asExactFraction**

Return the receiver, converted to a Fraction retaining the exact value
of the receiver.

[]{#index-asFloat-1}

**asFloat**

Convert the receiver to an arbitrary subclass of Float

[]{#index-asFloatD-6}

**asFloatD**

This method's functionality should be implemented by subclasses of
Number

[]{#index-asFloatE-6}

**asFloatE**

This method's functionality should be implemented by subclasses of
Number

[]{#index-asFloatQ-6}

**asFloatQ**

This method's functionality should be implemented by subclasses of
Number

[]{#index-asFraction-3}

**asFraction**

This method's functionality should be implemented by subclasses of
Number

[]{#index-asNumber-1}

**asNumber**

Answer the receiver, since it is already a number

[]{#index-asRectangle}

**asRectangle**

Answer an empty rectangle whose origin is (self asPoint)

[]{#index-asScaledDecimal_003a-1}

**asScaledDecimal: n**

Answer the receiver, converted to a ScaledDecimal object.

[]{#index-asScaledDecimal_003aradix_003ascale_003a}

**asScaledDecimal: denDigits radix: base scale: n**

Answer the receiver, divided by base\^denDigits and converted to a
ScaledDecimal object.

[]{#index-asString-9} []{#index-displayString-5}

**asString**

Answer the receiver's #displayString, which should be a good enough
conversion to String for a number.

[]{#index-coerce_003a-12}

**coerce: aNumber**

Answer aNumber, converted to an integer or floating-point number.

[]{#index-degreesToRadians}

**degreesToRadians**

Convert the receiver to radians

[]{#index-generality-5}

**generality**

Answer the receiver's generality

[]{#index-radiansToDegrees}

**radiansToDegrees**

Convert the receiver from radians to degrees

[]{#index-unity-5}

**unity**

Coerce 1 to the receiver's class. The default implementation works, but
is inefficient

[]{#index-zero-6}

**zero**

Coerce 0 to the receiver's class. The default implementation works, but
is inefficient
