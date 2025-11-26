[]{#ProcessEnvironment_002daccessing}

::: header
Next: [ProcessEnvironment-dictionary
removing](ProcessEnvironment_002ddictionary-removing.html#ProcessEnvironment_002ddictionary-removing){accesskey="n"
rel="next"}, Previous: [ProcessEnvironment
class-singleton](ProcessEnvironment-class_002dsingleton.html#ProcessEnvironment-class_002dsingleton){accesskey="p"
rel="prev"}, Up:
[ProcessEnvironment](ProcessEnvironment.html#ProcessEnvironment){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessEnvironment_003a-accessing}

#### 1.135.3 ProcessEnvironment: accessing {#processenvironment-accessing .subsection}

[]{#index-add_003a-9}

**add: newObject**

Add the newObject association to the receiver

[]{#index-associationAt_003a-1}

**associationAt: key**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-associationAt_003aifAbsent_003a-3}

**associationAt: key ifAbsent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003a-17}

**at: key**

Answer the value associated to the given key. Return nil if the key is
not found

[]{#index-at_003aifAbsent_003a-5}

**at: key ifAbsent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003aifAbsentPut_003a-1}

**at: key ifAbsentPut: aBlock**

Answer the value associated to the given key, setting it to the result
of evaluating aBlock if the key is not found.

[]{#index-at_003aifPresent_003a-3}

**at: key ifPresent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003aput_003a-17}

**at: key put: value**

Store value as associated to the given key

[]{#index-keys-2}

**keys**

Answer a kind of Set containing the keys of the receiver
