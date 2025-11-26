[]{#String_002dbuilt-ins}

::: header
Next:
[String-CObject](String_002dCObject.html#String_002dCObject){accesskey="n"
rel="next"}, Previous:
[String-basic](String_002dbasic.html#String_002dbasic){accesskey="p"
rel="prev"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String_003a-built-ins}

#### 1.158.5 String: built ins {#string-built-ins .subsection}

[]{#index-asCData_003a-1}

**asCData: aCType**

Allocate memory with malloc for a NULL-terminated copy of the receiver,
and return a pointer to it as a CObject of the given type.

[]{#index-at_003a-21}

**at: anIndex**

Answer the index-th indexed instance variable of the receiver

[]{#index-at_003aifAbsent_003a-7}

**at: anIndex ifAbsent: aBlock**

Answer the index-th indexed instance variable of the receiver

[]{#index-at_003aput_003a-22}

**at: anIndex put: value**

Store value in the index-th indexed instance variable of the receiver

[]{#index-basicAt_003a-2}

**basicAt: anIndex**

Answer the index-th indexed instance variable of the receiver. This
method must not be overridden, override at: instead

[]{#index-basicAt_003aput_003a-2}

**basicAt: anIndex put: value**

Store value in the index-th indexed instance variable of the receiver
This method must not be overridden, override at:put: instead

[]{#index-hash-34}

**hash**

Answer an hash value for the receiver

[]{#index-replaceFrom_003ato_003awith_003astartingAt_003a-3}

**replaceFrom: start to: stop with: aString startingAt: replaceStart**

Replace the characters from start to stop with new characters whose
ASCII codes are contained in aString, starting at the replaceStart
location of aString

[]{#index-replaceFrom_003ato_003awithByteArray_003astartingAt_003a}

**replaceFrom: start to: stop withByteArray: byteArray startingAt:
replaceStart**

Replace the characters from start to stop with new characters whose
ASCII codes are contained in byteArray, starting at the replaceStart
location of byteArray

[]{#index-similarityTo_003a}

**similarityTo: aString**

Answer a number that denotes the similarity between aString and the
receiver. 0 indicates equality, negative numbers indicate some
difference. Implemented as a primitive for speed.

[]{#index-size-24}

**size**

Answer the size of the receiver
