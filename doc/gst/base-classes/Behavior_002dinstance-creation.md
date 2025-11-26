[]{#Behavior_002dinstance-creation}

::: header
Next: [Behavior-instance
variables](Behavior_002dinstance-variables.html#Behavior_002dinstance-variables){accesskey="n"
rel="next"}, Previous:
[Behavior-evaluating](Behavior_002devaluating.html#Behavior_002devaluating){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-instance-creation}

#### 1.9.13 Behavior: instance creation {#behavior-instance-creation .subsection}

[]{#index-newInFixedSpace} []{#index-new-38}

**newInFixedSpace**

Create a new instance of a class without indexed instance variables. The
instance is guaranteed not to move across garbage collections. If a
subclass overrides #new, the changes will apply to this method too.

[]{#index-newInFixedSpace_003a} []{#index-new_003a-13}

**newInFixedSpace: numInstanceVariables**

Create a new instance of a class with indexed instance variables. The
instance has numInstanceVariables indexed instance variables. The
instance is guaranteed not to move across garbage collections. If a
subclass overrides #new:, the changes will apply to this method too.
