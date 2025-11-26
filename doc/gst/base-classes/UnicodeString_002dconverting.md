[]{#UnicodeString_002dconverting}

::: header
Next: [UnicodeString-multibyte
encodings](UnicodeString_002dmultibyte-encodings.html#UnicodeString_002dmultibyte-encodings){accesskey="n"
rel="next"}, Previous:
[UnicodeString-built-ins](UnicodeString_002dbuilt_002dins.html#UnicodeString_002dbuilt_002dins){accesskey="p"
rel="prev"}, Up:
[UnicodeString](UnicodeString.html#UnicodeString){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UnicodeString_003a-converting}

#### 1.203.5 UnicodeString: converting {#unicodestring-converting .subsection}

[]{#index-asString-13}

**asString**

Returns the string corresponding to the receiver. Without the Iconv
package, unrecognized Unicode characters become \$? characters. When it
is loaded, an appropriate single- or multi-byte encoding could be used.

[]{#index-asSymbol-4}

**asSymbol**

Returns the symbol corresponding to the receiver

[]{#index-asUnicodeString-4}

**asUnicodeString**

But I already am a UnicodeString! Really!

[]{#index-displayOn_003a-8}

**displayOn: aStream**

Print a representation of the receiver on aStream

[]{#index-printOn_003a-47}

**printOn: aStream**

Print a representation of the receiver on aStream
