[]{#LargeInteger_002dbuilt_002dins}

::: header
Next:
[LargeInteger-coercion](LargeInteger_002dcoercion.html#LargeInteger_002dcoercion){accesskey="n"
rel="next"}, Previous: [LargeInteger-bit
operations](LargeInteger_002dbit-operations.html#LargeInteger_002dbit-operations){accesskey="p"
rel="prev"}, Up:
[LargeInteger](LargeInteger.html#LargeInteger){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LargeInteger_003a-built_002dins}

#### 1.98.4 LargeInteger: built-ins {#largeinteger-built-ins .subsection}

[]{#index-at_003a-9}

**at: anIndex**

Answer the anIndex-th byte in the receiver's representation

[]{#index-at_003aput_003a-8}

**at: anIndex put: aNumber**

Set the anIndex-th byte in the receiver's representation

[]{#index-digitAt_003a-1}

**digitAt: anIndex**

Answer the index-th base-256 digit of the receiver (byte), expressed in
two's complement

[]{#index-digitAt_003aput_003a}

**digitAt: anIndex put: aNumber**

Set the anIndex-th base-256 digit in the receiver's representation

[]{#index-digitLength}

**digitLength**

Answer the number of base-256 digits in the receiver

[]{#index-hash-22}

**hash**

Answer an hash value for the receiver

[]{#index-primReplaceFrom_003ato_003awith_003astartingAt_003a}

**primReplaceFrom: start to: stop with: replacementString startingAt:
replaceStart**

Private - Replace the characters from start to stop with new characters
contained in replacementString (which, actually, can be any variable
byte class), starting at the replaceStart location of replacementString

[]{#index-size-12}

**size**

Answer the number of indexed instance variable in the receiver
