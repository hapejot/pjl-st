[]{#Namespace_002doverrides-for-superspaces}

::: header
Next:
[Namespace-printing](Namespace_002dprinting.html#Namespace_002dprinting){accesskey="n"
rel="next"}, Previous: [Namespace-namespace
hierarchy](Namespace_002dnamespace-hierarchy.html#Namespace_002dnamespace-hierarchy){accesskey="p"
rel="prev"}, Up: [Namespace](Namespace.html#Namespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Namespace_003a-overrides-for-superspaces}

#### 1.116.6 Namespace: overrides for superspaces {#namespace-overrides-for-superspaces .subsection}

[]{#index-associationAt_003aifAbsent_003a-2}

**associationAt: key ifAbsent: aBlock**

Return the key/value pair associated to the variable named as specified
by 'key'. If the key is not found search will be brought on in
superspaces, finally evaluating aBlock if the variable cannot be found
in any of the superspaces.

[]{#index-associationsDo_003a-2}

**associationsDo: aBlock**

Pass each association in the namespace to aBlock

[]{#index-at_003aifAbsent_003a-4}

**at: key ifAbsent: aBlock**

Return the value associated to the variable named as specified by 'key'.
If the key is not found search will be brought on in superspaces,
finally evaluating aBlock if the variable cannot be found in any of the
superspaces.

[]{#index-at_003aifPresent_003a-2}

**at: key ifPresent: aBlock**

If aKey is absent from the receiver and all its superspaces, answer nil.
Else, evaluate aBlock passing the associated value and answer the result
of the invocation

[]{#index-do_003a-10}

**do: aBlock**

Pass each value in the namespace to aBlock

[]{#index-includesKey_003a-1}

**includesKey: key**

Answer whether the receiver or any of its superspaces contain the given
key

[]{#index-keysAndValuesDo_003a-3}

**keysAndValuesDo: aBlock**

Pass to aBlock each of the receiver's keys and values, in two separate
parameters

[]{#index-keysDo_003a-3}

**keysDo: aBlock**

Pass to aBlock each of the receiver's keys

[]{#index-set_003ato_003aifAbsent_003a-1}

**set: key to: newValue ifAbsent: aBlock**

Assign newValue to the variable named as specified by 'key'. This method
won't define a new variable; instead if the key is not found it will
search in superspaces and evaluate aBlock if it is not found. Answer
newValue.

[]{#index-size-17}

**size**

Answer the number of keys in the receiver and each of its superspaces

------------------------------------------------------------------------

::: header
Next:
[Namespace-printing](Namespace_002dprinting.html#Namespace_002dprinting){accesskey="n"
rel="next"}, Previous: [Namespace-namespace
hierarchy](Namespace_002dnamespace-hierarchy.html#Namespace_002dnamespace-hierarchy){accesskey="p"
rel="prev"}, Up: [Namespace](Namespace.html#Namespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
