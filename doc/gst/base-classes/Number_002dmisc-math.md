[]{#Number_002dmisc-math}

::: header
Next: [Number-point
creation](Number_002dpoint-creation.html#Number_002dpoint-creation){accesskey="n"
rel="next"}, Previous: [Number-error
raising](Number_002derror-raising.html#Number_002derror-raising){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-misc-math}

#### 1.122.9 Number: misc math {#number-misc-math .subsection}

[]{#index-abs-3}

**abs**

Answer the absolute value of the receiver

[]{#index-arcCos-1}

**arcCos**

Answer the arc cosine of the receiver

[]{#index-arcCosh}

**arcCosh**

Answer the hyperbolic arc-cosine of the receiver.

[]{#index-arcSin-1}

**arcSin**

Answer the arc sine of the receiver

[]{#index-arcSinh}

**arcSinh**

Answer the hyperbolic arc-sine of the receiver.

[]{#index-arcTan-1}

**arcTan**

Answer the arc tangent of the receiver

[]{#index-arcTan_003a}

**arcTan: x**

Answer the angle (measured counterclockwise) between (x, self) and a ray
starting in (0, 0) and moving towards (1, 0) - i.e. 3 o'clock

[]{#index-arcTanh}

**arcTanh**

Answer the hyperbolic arc-tangent of the receiver.

[]{#index-ceilingLog_003a-2}

**ceilingLog: radix**

Answer (self log: radix) ceiling. Optimized to answer an integer.

[]{#index-cos-1}

**cos**

Answer the cosine of the receiver

[]{#index-cosh}

**cosh**

Answer the hyperbolic cosine of the receiver.

[]{#index-estimatedLog-4}

**estimatedLog**

Answer an estimate of (self abs floorLog: 10). This method should be
overridden by subclasses, but Number's implementation does not raise
errors - simply, it gives a correct result, so it is slow.

[]{#index-exp-1}

**exp**

Answer e raised to the receiver

[]{#index-floorLog_003a-2}

**floorLog: radix**

Answer (self log: radix) floor. Optimized to answer an integer.

[]{#index-ln-1}

**ln**

Answer log base e of the receiver

[]{#index-log-1}

**log**

Answer log base 10 of the receiver

[]{#index-log_003a-1}

**log: aNumber**

Answer log base aNumber of the receiver

[]{#index-negated-4}

**negated**

Answer the negated of the receiver

[]{#index-positiveDifference_003a}

**positiveDifference: aNumber**

Answer the positive difference of the receiver and aNumber, that is
self - aNumber if it is positive, 0 otherwise.

[]{#index-raisedTo_003a-1}

**raisedTo: aNumber**

Return self raised to aNumber power

[]{#index-raisedToInteger_003a-3}

**raisedToInteger: anInteger**

Return self raised to the anInteger-th power

[]{#index-sin-1}

**sin**

Answer the sine of the receiver

[]{#index-sinh}

**sinh**

Answer the hyperbolic sine of the receiver.

[]{#index-sqrt-2}

**sqrt**

Answer the square root of the receiver

[]{#index-squared-1}

**squared**

Answer the square of the receiver

[]{#index-tan-1}

**tan**

Answer the tangent of the receiver

[]{#index-tanh}

**tanh**

Answer the hyperbolic tangent of the receiver.

[]{#index-withSignOf_003a-1}

**withSignOf: aNumber**

Answer the receiver, with its sign possibly changed to match that of
aNumber.

------------------------------------------------------------------------

::: header
Next: [Number-point
creation](Number_002dpoint-creation.html#Number_002dpoint-creation){accesskey="n"
rel="next"}, Previous: [Number-error
raising](Number_002derror-raising.html#Number_002derror-raising){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
