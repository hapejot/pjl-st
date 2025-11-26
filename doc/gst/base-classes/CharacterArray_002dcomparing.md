[]{#CharacterArray_002dcomparing}

::: header
Next:
[CharacterArray-converting](CharacterArray_002dconverting.html#CharacterArray_002dconverting){accesskey="n"
rel="next"}, Previous: [CharacterArray-built
ins](CharacterArray_002dbuilt-ins.html#CharacterArray_002dbuilt-ins){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CharacterArray_003a-comparing}

#### 1.29.5 CharacterArray: comparing {#characterarray-comparing .subsection}

[]{#index-_003c-1}

**\< aCharacterArray**

Return true if the receiver is less than aCharacterArray, ignoring case
differences.

[]{#index-_003c_003d-1}

**\<= aCharacterArray**

Returns true if the receiver is less than or equal to aCharacterArray,
ignoring case differences. If is receiver is an initial substring of
aCharacterArray, it is considered to be less than aCharacterArray.

[]{#index-_003d-6}

**= aString**

Answer whether the receiver's items match those in aCollection

[]{#index-_003e-1}

**\> aCharacterArray**

Return true if the receiver is greater than aCharacterArray, ignoring
case differences.

[]{#index-_003e_003d-1}

**\>= aCharacterArray**

Returns true if the receiver is greater than or equal to
aCharacterArray, ignoring case differences. If is aCharacterArray is an
initial substring of the receiver, it is considered to be less than the
receiver.

[]{#index-indexOf_003amatchCase_003astartingAt_003a}

**indexOf: aCharacterArray matchCase: aBoolean startingAt: anIndex**

Answer an Interval of indices in the receiver which match the
aCharacterArray pattern. \# in aCharacterArray means 'match any
character', \* in aCharacterArray means 'match any sequence of
characters'. The first item of the returned interval is \>= anIndex. If
aBoolean is false, the search is case-insensitive, else it is
case-sensitive. If no Interval matches the pattern, answer nil.

[]{#index-match_003a}

**match: aCharacterArray**

Answer whether aCharacterArray matches the pattern contained in the
receiver. \# in the receiver means 'match any character', \* in receiver
means 'match any sequence of characters'.

[]{#index-match_003aignoreCase_003a}

**match: aCharacterArray ignoreCase: aBoolean**

Answer whether aCharacterArray matches the pattern contained in the
receiver. \# in the receiver means 'match any character', \* in receiver
means 'match any sequence of characters'. The case of alphabetic
characters is ignored if aBoolean is true.

[]{#index-sameAs_003a}

**sameAs: aCharacterArray**

Returns true if the receiver is the same CharacterArray as
aCharacterArray, ignoring case differences.

------------------------------------------------------------------------

::: header
Next:
[CharacterArray-converting](CharacterArray_002dconverting.html#CharacterArray_002dconverting){accesskey="n"
rel="next"}, Previous: [CharacterArray-built
ins](CharacterArray_002dbuilt-ins.html#CharacterArray_002dbuilt-ins){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
