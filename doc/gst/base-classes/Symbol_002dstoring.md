[]{#Symbol_002dstoring}

::: header
Next:
[Symbol-testing](Symbol_002dtesting.html#Symbol_002dtesting){accesskey="n"
rel="next"}, Previous:
[Symbol-misc](Symbol_002dmisc.html#Symbol_002dmisc){accesskey="p"
rel="prev"}, Up: [Symbol](Symbol.html#Symbol){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Symbol_003a-storing}

#### 1.159.9 Symbol: storing {#symbol-storing .subsection}

[]{#index-displayOn_003a-7} []{#index-printOn_003a-57}

**displayOn: aStream**

Print a represention of the receiver on aStream. For most objects this
is simply its #printOn: representation, but for strings and characters,
superfluous dollars or extra pairs of quotes are stripped.

[]{#index-displayString-4} []{#index-printString-6}

**displayString**

Answer a String representing the receiver. For most objects this is
simply its #printString, but for strings and characters, superfluous
dollars or extra pair of quotes are stripped.

[]{#index-printOn_003a-41}

**printOn: aStream**

Print a represention of the receiver on aStream.

[]{#index-storeLiteralOn_003a-9}

**storeLiteralOn: aStream**

Print Smalltalk code on aStream that compiles to the same symbol as the
receiver.

[]{#index-storeOn_003a-37}

**storeOn: aStream**

Print Smalltalk code on aStream that compiles to the same symbol as the
receiver.
