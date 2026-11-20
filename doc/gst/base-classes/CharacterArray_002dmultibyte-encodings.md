[]{#CharacterArray_002dmultibyte-encodings}

::: header
Next: [CharacterArray-still
unclassified](CharacterArray_002dstill-unclassified.html#CharacterArray_002dstill-unclassified){accesskey="n"
rel="next"}, Previous:
[CharacterArray-converting](CharacterArray_002dconverting.html#CharacterArray_002dconverting){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CharacterArray_003a-multibyte-encodings}

#### 1.29.7 CharacterArray: multibyte encodings {#characterarray-multibyte-encodings .subsection}

[]{#index-encoding}

**encoding**

Answer the encoding used by the receiver.

[]{#index-isUnicode-1}

**isUnicode**

Answer whether the receiver stores bytes (i.e. an encoded form) or
characters (if true is returned).

[]{#index-numberOfCharacters}

**numberOfCharacters**

Answer the number of Unicode characters in the receiver. This is not
implemented unless you load the I18N package.
