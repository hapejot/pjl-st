[]{#Integer_002dprinting}

::: header
Next:
[Integer-storing](Integer_002dstoring.html#Integer_002dstoring){accesskey="n"
rel="next"}, Previous: [Integer-math
methods](Integer_002dmath-methods.html#Integer_002dmath-methods){accesskey="p"
rel="prev"}, Up: [Integer](Integer.html#Integer){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Integer_003a-printing}

#### 1.92.9 Integer: printing {#integer-printing .subsection}

[]{#index-displayOn_003a-2}

**displayOn: aStream**

Print on aStream the base 10 representation of the receiver

[]{#index-displayString}

**displayString**

Return the base 10 representation of the receiver

[]{#index-isLiteralObject-5}

**isLiteralObject**

Answer whether the receiver is expressible as a Smalltalk literal.

[]{#index-printOn_003a-23}

**printOn: aStream**

Print on aStream the base 10 representation of the receiver

[]{#index-printOn_003abase_003a}

**printOn: aStream base: b**

Print on aStream the base b representation of the receiver

[]{#index-printOn_003apaddedWith_003ato_003a}

**printOn: aStream paddedWith: padding to: size**

Print on aStream the base 10 representation of the receiver, padded if
necessary to size characters with copies of padding.

[]{#index-printOn_003apaddedWith_003ato_003abase_003a}

**printOn: aStream paddedWith: padding to: size base: baseInteger**

Print on aStream the base b representation of the receiver, padded if
necessary to size characters with copies of padding.

[]{#index-printPaddedWith_003ato_003a}

**printPaddedWith: padding to: size**

Return the base baseInteger representation of the receiver, padded if
necessary to size characters with copies of padding.

[]{#index-printPaddedWith_003ato_003abase_003a}

**printPaddedWith: padding to: size base: baseInteger**

Return the base baseInteger representation of the receiver, padded if
necessary to size characters with copies of padding.

[]{#index-printString}

**printString**

Return the base 10 representation of the receiver

[]{#index-printString_003a}

**printString: baseInteger**

Return the base baseInteger representation of the receiver

[]{#index-printStringRadix_003a}

**printStringRadix: baseInteger**

Return the base baseInteger representation of the receiver, with BBr in
front of it

[]{#index-radix_003a} []{#index-printStringRadix_003a-1}

**radix: baseInteger**

Return the base baseInteger representation of the receiver, with BBr in
front of it. This method is deprecated, use #printStringRadix: instead.

[]{#index-storeLiteralOn_003a-5}

**storeLiteralOn: aStream**

Store on aStream some Smalltalk code which compiles to the receiver

[]{#index-storeOn_003abase_003a}

**storeOn: aStream base: b**

Print on aStream Smalltalk code compiling to the receiver, represented
in base b

------------------------------------------------------------------------

::: header
Next:
[Integer-storing](Integer_002dstoring.html#Integer_002dstoring){accesskey="n"
rel="next"}, Previous: [Integer-math
methods](Integer_002dmath-methods.html#Integer_002dmath-methods){accesskey="p"
rel="prev"}, Up: [Integer](Integer.html#Integer){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
