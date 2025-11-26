[]{#ByteArray_002dbuilt-ins}

::: header
Next:
[ByteArray-CObject](ByteArray_002dCObject.html#ByteArray_002dCObject){accesskey="n"
rel="next"}, Previous:
[ByteArray-basic](ByteArray_002dbasic.html#ByteArray_002dbasic){accesskey="p"
rel="prev"}, Up: [ByteArray](ByteArray.html#ByteArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ByteArray_003a-built-ins}

#### 1.14.3 ByteArray: built ins {#bytearray-built-ins .subsection}

[]{#index-asCData_003a}

**asCData: aCType**

Allocate memory with malloc for a copy of the receiver, and return it
converted to a CObject with the given type

[]{#index-at_003aifAbsent_003a-1}

**at: anIndex ifAbsent: aBlock**

Answer the index-th indexed instance variable of the receiver

[]{#index-byteAt_003a}

**byteAt: index**

Answer the index-th indexed instance variable of the receiver

[]{#index-byteAt_003aput_003a}

**byteAt: index put: value**

Store the 'value' byte in the index-th indexed instance variable of the
receiver

[]{#index-hash-3}

**hash**

Answer an hash value for the receiver

[]{#index-replaceFrom_003ato_003awith_003astartingAt_003a-1}

**replaceFrom: start to: stop with: aByteArray startingAt:
replaceStart**

Replace the characters from start to stop with the bytes contained in
aByteArray (which, actually, can be any variable byte class), starting
at the replaceStart location of aByteArray

[]{#index-replaceFrom_003ato_003awithString_003astartingAt_003a}

**replaceFrom: start to: stop withString: aString startingAt:
replaceStart**

Replace the characters from start to stop with the ASCII codes contained
in aString (which, actually, can be any variable byte class), starting
at the replaceStart location of aString
