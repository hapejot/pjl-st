[]{#Behavior_002dtesting-the-form-of-the-instances}

::: header
Next: [Behavior-testing the method
dictionary](Behavior_002dtesting-the-method-dictionary.html#Behavior_002dtesting-the-method-dictionary){accesskey="n"
rel="next"}, Previous: [Behavior-testing the class
hierarchy](Behavior_002dtesting-the-class-hierarchy.html#Behavior_002dtesting-the-class-hierarchy){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-testing-the-form-of-the-instances}

#### 1.9.24 Behavior: testing the form of the instances {#behavior-testing-the-form-of-the-instances .subsection}

[]{#index-instSize}

**instSize**

Answer how many fixed instance variables are reserved to each of the
receiver's instances

[]{#index-isBits}

**isBits**

Answer whether my instances' variables are immediate, non-OOP values.

[]{#index-isFixed}

**isFixed**

Answer whether the receiver's instances have no indexed instance
variables

[]{#index-isIdentity}

**isIdentity**

Answer whether x = y implies x == y for instances of the receiver

[]{#index-isImmediate}

**isImmediate**

Answer whether, if x is an instance of the receiver, x copy == x

[]{#index-isPointers}

**isPointers**

Answer whether the instance variables of the receiver's instances are
objects

[]{#index-isVariable}

**isVariable**

Answer whether the receiver's instances have indexed instance variables
