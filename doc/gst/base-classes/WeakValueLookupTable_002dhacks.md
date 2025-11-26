[]{#WeakValueLookupTable_002dhacks}

::: header
Next:
[WeakValueLookupTable-rehashing](WeakValueLookupTable_002drehashing.html#WeakValueLookupTable_002drehashing){accesskey="n"
rel="next"}, Previous: [WeakValueLookupTable
class-hacks](WeakValueLookupTable-class_002dhacks.html#WeakValueLookupTable-class_002dhacks){accesskey="p"
rel="prev"}, Up:
[WeakValueLookupTable](WeakValueLookupTable.html#WeakValueLookupTable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakValueLookupTable_003a-hacks}

#### 1.221.2 WeakValueLookupTable: hacks {#weakvaluelookuptable-hacks .subsection}

[]{#index-at_003aifAbsent_003a-9}

**at: key ifAbsent: aBlock**

Answer the value associated to the given key, or the result of
evaluating aBlock if the key is not found

[]{#index-at_003aifPresent_003a-4}

**at: key ifPresent: aBlock**

If aKey is absent, answer nil. Else, evaluate aBlock passing the
associated value and answer the result of the invocation

[]{#index-includesKey_003a-3}

**includesKey: key**

Answer whether the receiver contains the given key.
