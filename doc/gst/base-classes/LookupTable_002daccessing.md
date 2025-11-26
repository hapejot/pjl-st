[]{#LookupTable_002daccessing}

::: header
Next:
[LookupTable-enumerating](LookupTable_002denumerating.html#LookupTable_002denumerating){accesskey="n"
rel="next"}, Previous: [LookupTable class-instance
creation](LookupTable-class_002dinstance-creation.html#LookupTable-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[LookupTable](LookupTable.html#LookupTable){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LookupTable_003a-accessing}

#### 1.106.2 LookupTable: accessing {#lookuptable-accessing .subsection}

[]{#index-add_003a-6}

**add: anAssociation**

Add the anAssociation key to the receiver

[]{#index-associationAt_003aifAbsent_003a-1}

**associationAt: key ifAbsent: aBlock**

Answer the key/value Association for the given key. Evaluate aBlock
(answering the result) if the key is not found

[]{#index-at_003aifAbsent_003a-3}

**at: key ifAbsent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003aifPresent_003a-1}

**at: aKey ifPresent: aBlock**

If aKey is absent, answer nil. Else, evaluate aBlock passing the
associated value and answer the result of the invocation

[]{#index-at_003aput_003a-11}

**at: key put: value**

Store value as associated to the given key
