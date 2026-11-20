[]{#Object_002dprinting}

::: header
Next: [Object-relational
operators](Object_002drelational-operators.html#Object_002drelational-operators){accesskey="n"
rel="next"}, Previous:
[Object-introspection](Object_002dintrospection.html#Object_002dintrospection){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-printing}

#### 1.123.13 Object: printing {#object-printing .subsection}

[]{#index-basicPrintNl}

**basicPrintNl**

Print a basic representation of the receiver, followed by a new line.

[]{#index-basicPrintOn_003a}

**basicPrintOn: aStream**

Print a represention of the receiver on aStream

[]{#index-display} []{#index-print-1}

**display**

Print a represention of the receiver on the Transcript (stdout the GUI
is not active). For most objects this is simply its #print
representation, but for strings and characters, superfluous dollars or
extra pair of quotes are stripped.

[]{#index-displayNl} []{#index-printNl-1}

**displayNl**

Print a represention of the receiver, then put a new line on the
Transcript (stdout the GUI is not active). For most objects this is
simply its #printNl representation, but for strings and characters,
superfluous dollars or extra pair of quotes are stripped.

[]{#index-displayOn_003a-3} []{#index-printOn_003a-52}

**displayOn: aStream**

Print a represention of the receiver on aStream. For most objects this
is simply its #printOn: representation, but for strings and characters,
superfluous dollars or extra pair of quotes are stripped.

[]{#index-displayString-1} []{#index-printString-2}

**displayString**

Answer a String representing the receiver. For most objects this is
simply its #printString, but for strings and characters, superfluous
dollars or extra pair of quotes are stripped.

[]{#index-print}

**print**

Print a represention of the receiver on the Transcript (stdout the GUI
is not active)

[]{#index-printNl}

**printNl**

Print a represention of the receiver on stdout, put a new line the
Transcript (stdout the GUI is not active)

[]{#index-printOn_003a-30}

**printOn: aStream**

Print a represention of the receiver on aStream

[]{#index-printString-1}

**printString**

Answer a String representing the receiver

------------------------------------------------------------------------

::: header
Next: [Object-relational
operators](Object_002drelational-operators.html#Object_002drelational-operators){accesskey="n"
rel="next"}, Previous:
[Object-introspection](Object_002dintrospection.html#Object_002dintrospection){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
