[]{#CharacterArray_002dconverting}

::: header
Next: [CharacterArray-multibyte
encodings](CharacterArray_002dmultibyte-encodings.html#CharacterArray_002dmultibyte-encodings){accesskey="n"
rel="next"}, Previous:
[CharacterArray-comparing](CharacterArray_002dcomparing.html#CharacterArray_002dcomparing){accesskey="p"
rel="prev"}, Up:
[CharacterArray](CharacterArray.html#CharacterArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CharacterArray_003a-converting}

#### 1.29.6 CharacterArray: converting {#characterarray-converting .subsection}

[]{#index-asByteArray}

**asByteArray**

Return the receiver, converted to a ByteArray of ASCII values

[]{#index-asClassPoolKey}

**asClassPoolKey**

Return the receiver, ready to be put in a class pool dictionary

[]{#index-asGlobalKey}

**asGlobalKey**

Return the receiver, ready to be put in the Smalltalk dictionary

[]{#index-asInteger-1}

**asInteger**

Parse an Integer number from the receiver until the input character is
invalid and answer the result at this point

[]{#index-asLowercase-1}

**asLowercase**

Returns a copy of self as a lowercase CharacterArray

[]{#index-asNumber}

**asNumber**

Parse a Number from the receiver until the input character is invalid
and answer the result at this point

[]{#index-asPoolKey}

**asPoolKey**

Return the receiver, ready to be put in a pool dictionary

[]{#index-asString-3}

**asString**

But I already am a String! Really!

[]{#index-asSymbol-1}

**asSymbol**

Returns the symbol corresponding to the CharacterArray

[]{#index-asUnicodeString-2}

**asUnicodeString**

Answer a UnicodeString whose character's codes are the receiver's
contents This is not implemented unless you load the I18N package.

[]{#index-asUppercase-1}

**asUppercase**

Returns a copy of self as an uppercase CharacterArray

[]{#index-fileName}

**fileName**

But I don't HAVE a file name!

[]{#index-filePos}

**filePos**

But I don't HAVE a file position!

[]{#index-isNumeric}

**isNumeric**

Answer whether the receiver denotes a number

[]{#index-trimSeparators}

**trimSeparators**

Return a copy of the reciever without any spaces on front or back. The
implementation is protected against the 'all blanks' case.
