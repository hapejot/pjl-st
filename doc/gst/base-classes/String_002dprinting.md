[]{#String_002dprinting}

::: header
Next:
[String-regex](String_002dregex.html#String_002dregex){accesskey="n"
rel="next"}, Previous:
[String-filesystem](String_002dfilesystem.html#String_002dfilesystem){accesskey="p"
rel="prev"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String_003a-printing}

#### 1.158.9 String: printing {#string-printing .subsection}

[]{#index-displayOn_003a-6} []{#index-printOn_003a-54}

**displayOn: aStream**

Print a representation of the receiver on aStream. Unlike #printOn:,
this method strips extra quotes.

[]{#index-displayString-3} []{#index-printString-5}

**displayString**

Answer a String representing the receiver. For most objects this is
simply its #printString, but for CharacterArrays and characters,
superfluous dollars or extra pair of quotes are stripped.

[]{#index-isLiteralObject-7}

**isLiteralObject**

Answer whether the receiver is expressible as a Smalltalk literal.

[]{#index-printOn_003a-40}

**printOn: aStream**

Print a representation of the receiver on aStream

[]{#index-storeLiteralOn_003a-8}

**storeLiteralOn: aStream**

Store a Smalltalk literal compiling to the receiver on aStream

[]{#index-storeOn_003a-36}

**storeOn: aStream**

Store Smalltalk code compiling to the receiver on aStream
