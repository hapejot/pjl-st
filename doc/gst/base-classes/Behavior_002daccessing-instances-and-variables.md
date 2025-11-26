[]{#Behavior_002daccessing-instances-and-variables}

::: header
Next: [Behavior-accessing the method
dictionary](Behavior_002daccessing-the-method-dictionary.html#Behavior_002daccessing-the-method-dictionary){accesskey="n"
rel="next"}, Previous: [Behavior-accessing class
hierarchy](Behavior_002daccessing-class-hierarchy.html#Behavior_002daccessing-class-hierarchy){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-accessing-instances-and-variables}

#### 1.9.2 Behavior: accessing instances and variables {#behavior-accessing-instances-and-variables .subsection}

[]{#index-allClassVarNames}

**allClassVarNames**

Return all the class variables understood by the receiver

[]{#index-allInstVarNames}

**allInstVarNames**

Answer the names of every instance variables the receiver contained in
the receiver's instances

[]{#index-allInstances}

**allInstances**

Returns a set of all instances of the receiver

[]{#index-allSharedPoolDictionaries}

**allSharedPoolDictionaries**

Return the shared pools defined by the class and any of its
superclasses, in the correct search order.

[]{#index-allSharedPools}

**allSharedPools**

Return the names of the shared pools defined by the class and any of its
superclasses, in the correct search order.

[]{#index-classPool}

**classPool**

Answer the class pool dictionary. Since Behavior does not support
classes with class variables, we answer an empty one; adding variables
to it results in an error.

[]{#index-classVarNames}

**classVarNames**

Answer all the class variables for instances of the receiver

[]{#index-indexOfInstVar_003a}

**indexOfInstVar: aString**

Answer the index of aString in the fixed instance variables of the
instances of the receiver, or 0 if the variable is missing.

[]{#index-indexOfInstVar_003aifAbsent_003a}

**indexOfInstVar: aString ifAbsent: aBlock**

Answer the index of aString in the fixed instance variables of the
instances of the receiver, or 0 if the variable is missing.

[]{#index-instVarNames}

**instVarNames**

Answer an Array containing the instance variables defined by the
receiver

[]{#index-instanceCount}

**instanceCount**

Return a count of all the instances of the receiver

[]{#index-sharedPools}

**sharedPools**

Return the names of the shared pools defined by the class

[]{#index-subclassInstVarNames}

**subclassInstVarNames**

Answer the names of the instance variables the receiver inherited from
its superclass

------------------------------------------------------------------------

::: header
Next: [Behavior-accessing the method
dictionary](Behavior_002daccessing-the-method-dictionary.html#Behavior_002daccessing-the-method-dictionary){accesskey="n"
rel="next"}, Previous: [Behavior-accessing class
hierarchy](Behavior_002daccessing-class-hierarchy.html#Behavior_002daccessing-class-hierarchy){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
