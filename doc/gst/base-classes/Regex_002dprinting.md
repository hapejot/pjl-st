[]{#Regex_002dprinting}

::: header
Previous:
[Regex-conversion](Regex_002dconversion.html#Regex_002dconversion){accesskey="p"
rel="prev"}, Up: [Regex](Regex.html#Regex){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Regex_003a-printing}

#### 1.144.4 Regex: printing {#regex-printing .subsection}

[]{#index-displayOn_003a-4} []{#index-printOn_003a-53}

**displayOn: aStream**

Print a represention of the receiver on aStream. For most objects this
is simply its #printOn: representation, but for strings and characters,
superfluous dollars or extra pairs of quotes are stripped.

[]{#index-displayString-2} []{#index-printString-3}

**displayString**

Answer a String representing the receiver. For most objects this is
simply its #printString, but for strings and characters, superfluous
dollars or extra pair of quotes are stripped.

[]{#index-printOn_003a-37}

**printOn: aStream**

Print a represention of the receiver on aStream.
