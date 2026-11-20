[]{#Dictionary_002daccessing}

::: header
Next: [Dictionary-awful ST-80 compatibility
hacks](Dictionary_002dawful-ST_002d80-compatibility-hacks.html#Dictionary_002dawful-ST_002d80-compatibility-hacks){accesskey="n"
rel="next"}, Previous: [Dictionary class-instance
creation](Dictionary-class_002dinstance-creation.html#Dictionary-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Dictionary_003a-accessing}

#### 1.64.2 Dictionary: accessing {#dictionary-accessing .subsection}

[]{#index-add_003a-3}

**add: newObject**

Add the newObject association to the receiver

[]{#index-addAll_003a-1}

**addAll: aCollection**

Adds all the elements of 'aCollection' to the receiver, answer
aCollection

[]{#index-associationAt_003a}

**associationAt: key**

Answer the key/value Association for the given key. Fail if the key is
not found

[]{#index-associationAt_003aifAbsent_003a}

**associationAt: key ifAbsent: aBlock**

Answer the key/value Association for the given key. Evaluate aBlock
(answering the result) if the key is not found

[]{#index-associations}

**associations**

Returns the content of a Dictionary as a Set of Associations.

[]{#index-at_003a-3}

**at: key**

Answer the value associated to the given key. Fail if the key is not
found

[]{#index-at_003aifAbsent_003a-2}

**at: key ifAbsent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003aifAbsentPut_003a}

**at: aKey ifAbsentPut: aBlock**

Answer the value associated to the given key. If the key is not found,
evaluate aBlock and associate the result to aKey before returning.

[]{#index-at_003aifPresent_003a}

**at: aKey ifPresent: aBlock**

If aKey is absent, answer nil. Else, evaluate aBlock passing the
associated value and answer the result of the invocation

[]{#index-at_003aput_003a-4}

**at: key put: value**

Store value as associated to the given key

[]{#index-atAll_003a-1}

**atAll: keyCollection**

Answer a Dictionary that only includes the given keys. Fail if any of
them is not found

[]{#index-keyAtValue_003a}

**keyAtValue: value**

Answer the key associated to the given value, or nil if the value is not
found

[]{#index-keyAtValue_003aifAbsent_003a}

**keyAtValue: value ifAbsent: exceptionBlock**

Answer the key associated to the given value. Evaluate exceptionBlock
(answering the result) if the value is not found. IMPORTANT: == is used
to compare values

[]{#index-keys}

**keys**

Answer a kind of Set containing the keys of the receiver

[]{#index-values-1}

**values**

Answer an Array containing the values of the receiver

------------------------------------------------------------------------

::: header
Next: [Dictionary-awful ST-80 compatibility
hacks](Dictionary_002dawful-ST_002d80-compatibility-hacks.html#Dictionary_002dawful-ST_002d80-compatibility-hacks){accesskey="n"
rel="next"}, Previous: [Dictionary class-instance
creation](Dictionary-class_002dinstance-creation.html#Dictionary-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
