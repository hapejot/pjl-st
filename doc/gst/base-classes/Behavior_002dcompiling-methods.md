[]{#Behavior_002dcompiling-methods}

::: header
Next: [Behavior-creating a class
hierarchy](Behavior_002dcreating-a-class-hierarchy.html#Behavior_002dcreating-a-class-hierarchy){accesskey="n"
rel="next"}, Previous:
[Behavior-compiling](Behavior_002dcompiling.html#Behavior_002dcompiling){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-compiling-methods}

#### 1.9.9 Behavior: compiling methods {#behavior-compiling-methods .subsection}

[]{#index-methodsFor_003a}

**methodsFor: aCategoryString**

Calling this method prepares the parser to receive methods to be
compiled and installed in the receiver's method dictionary. The methods
are put in the category identified by the parameter.

[]{#index-poolResolution}

**poolResolution**

Answer a PoolResolution class to be used for resolving pool variables
while compiling methods on this class.
