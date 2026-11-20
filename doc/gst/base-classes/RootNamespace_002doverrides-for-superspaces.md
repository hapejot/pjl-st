[]{#RootNamespace_002doverrides-for-superspaces}

::: header
Next:
[RootNamespace-printing](RootNamespace_002dprinting.html#RootNamespace_002dprinting){accesskey="n"
rel="next"}, Previous: [RootNamespace-namespace
hierarchy](RootNamespace_002dnamespace-hierarchy.html#RootNamespace_002dnamespace-hierarchy){accesskey="p"
rel="prev"}, Up:
[RootNamespace](RootNamespace.html#RootNamespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RootNamespace_003a-overrides-for-superspaces}

#### 1.146.3 RootNamespace: overrides for superspaces {#rootnamespace-overrides-for-superspaces .subsection}

[]{#index-inheritedKeys-2}

**inheritedKeys**

Answer a Set of all the keys in the receiver and its superspaces

[]{#index-set_003ato_003aifAbsent_003a-2}

**set: key to: newValue ifAbsent: aBlock**

Assign newValue to the variable named as specified by 'key'. This method
won't define a new variable; instead if the key is not found it will
search in superspaces and evaluate aBlock if it is not found. Answer
newValue.
