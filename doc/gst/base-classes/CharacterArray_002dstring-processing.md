[]{#CharacterArray_002dstring-processing}

::: header
Next: [CharacterArray-testing
functionality](CharacterArray_002dtesting-functionality.html#CharacterArray_002dtesting-functionality){accesskey="n"
rel="next"}, Previous: [CharacterArray-still
unclassified](CharacterArray_002dstill-unclassified.html#CharacterArray_002dstill-unclassified){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CharacterArray_003a-string-processing}

#### 1.29.9 CharacterArray: string processing {#characterarray-string-processing .subsection}

[]{#index-_0025}

**% aCollection**

Answer the receiver with special escape sequences replaced by elements
of aCollection. %n (1\<=n\<=9, A\<=n\<=Z) are replaced by the n-th
element of aCollection (A being the 10-th element and so on until the
35th). %(string) sequences are accessed as strings, which makes sense
only if aCollection is a Dictionary or LookupTable. In addition, the
special pattern %\<trueString\|falseString\>n or
%\<trueString\|falseString\>(string) is replaced with one of the two
strings depending on the element of aCollection being true or false. The
replaced elements are 'displayed' (i.e. their displayString is used).

[]{#index-bindWith_003a}

**bindWith: s1**

Answer the receiver with every %1 replaced by the displayString of s1

[]{#index-bindWith_003awith_003a}

**bindWith: s1 with: s2**

Answer the receiver with every %1 or %2 replaced by s1 or s2,
respectively. s1 and s2 are 'displayed' (i.e. their displayString is
used) upon replacement.

[]{#index-bindWith_003awith_003awith_003a}

**bindWith: s1 with: s2 with: s3**

Answer the receiver with every %1, %2 or %3 replaced by s1, s2 or s3,
respectively. s1, s2 and s3 are 'displayed' (i.e. their displayString is
used) upon replacement.

[]{#index-bindWith_003awith_003awith_003awith_003a}

**bindWith: s1 with: s2 with: s3 with: s4**

Answer the receiver with every %1, %2, %3 or %4 replaced by s1, s2, s3
or s4, respectively. s1, s2, s3 and s4 are 'displayed' (i.e. their
displayString is used) upon replacement.

[]{#index-bindWithArguments_003a}

**bindWithArguments: aCollection**

Answer the receiver with special escape sequences replaced by elements
of aCollection. %n (1\<=n\<=9, A\<=n\<=Z) are replaced by the n-th
element of aCollection (A being the 10-th element and so on until the
35th). %(string) sequences are accessed as strings, which makes sense
only if aCollection is a Dictionary or LookupTable. In addition, the
special pattern %\<trueString\|falseString\>n or
%\<trueString\|falseString\>(string) is replaced with one of the two
strings depending on the element of aCollection being true or false. The
replaced elements are 'displayed' (i.e. their displayString is used).

[]{#index-contractTo_003a}

**contractTo: smallSize**

Either return myself, or a copy shortened to smallSize characters by
inserting an ellipsis (three dots: \...)

[]{#index-lines}

**lines**

Answer an Array of Strings each representing one line in the receiver.

[]{#index-linesDo_003a}

**linesDo: aBlock**

Evaluate aBlock once for every newline delimited line in the receiver,
passing the line to the block.

[]{#index-subStrings}

**subStrings**

Answer an OrderedCollection of substrings of the receiver. A new
substring start at the start of the receiver, or after every sequence of
white space characters

[]{#index-subStrings_003a}

**subStrings: sep**

Answer an OrderedCollection of substrings of the receiver. A new
substring start at the start of the receiver, or after every occurrence
of one of the characters in sep

[]{#index-substrings}

**substrings**

Answer an OrderedCollection of substrings of the receiver. A new
substring start at the start of the receiver, or after every sequence of
white space characters. This message is preserved for backwards
compatibility; the ANSI standard mandates 'subStrings', with an
uppercase s.

[]{#index-substrings_003a}

**substrings: sep**

Answer an OrderedCollection of substrings of the receiver. A new
substring start at the start of the receiver, or after every occurrence
of one of the characters in sep. This message is preserved for backwards
compatibility; the ANSI standard mandates 'subStrings:', with an
uppercase s.

[]{#index-withShellEscapes}

**withShellEscapes**

Answer the receiver with special shell characters converted to a
backslash sequence.

------------------------------------------------------------------------

::: header
Next: [CharacterArray-testing
functionality](CharacterArray_002dtesting-functionality.html#CharacterArray_002dtesting-functionality){accesskey="n"
rel="next"}, Previous: [CharacterArray-still
unclassified](CharacterArray_002dstill-unclassified.html#CharacterArray_002dstill-unclassified){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
