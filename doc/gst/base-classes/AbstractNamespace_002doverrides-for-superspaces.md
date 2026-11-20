[]{#AbstractNamespace_002doverrides-for-superspaces}

::: header
Next:
[AbstractNamespace-printing](AbstractNamespace_002dprinting.html#AbstractNamespace_002dprinting){accesskey="n"
rel="next"}, Previous: [AbstractNamespace-namespace
hierarchy](AbstractNamespace_002dnamespace-hierarchy.html#AbstractNamespace_002dnamespace-hierarchy){accesskey="p"
rel="prev"}, Up:
[AbstractNamespace](AbstractNamespace.html#AbstractNamespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#AbstractNamespace_003a-overrides-for-superspaces}

#### 1.1.6 AbstractNamespace: overrides for superspaces {#abstractnamespace-overrides-for-superspaces .subsection}

[]{#index-inheritedKeys}

**inheritedKeys**

Answer a Set of all the keys in the receiver and its superspaces

[]{#index-set_003ato_003a}

**set: key to: newValue**

Assign newValue to the variable named as specified by 'key'. This method
won't define a new variable; instead if the key is not found it will
search in superspaces and raising an error if the variable cannot be
found in any of the superspaces. Answer newValue.

[]{#index-set_003ato_003aifAbsent_003a}

**set: key to: newValue ifAbsent: aBlock**

Assign newValue to the variable named as specified by 'key'. This method
won't define a new variable; instead if the key is not found it will
search in superspaces and evaluate aBlock if it is not found. Answer
newValue.

[]{#index-values}

**values**

Answer a Bag containing the values of the receiver
