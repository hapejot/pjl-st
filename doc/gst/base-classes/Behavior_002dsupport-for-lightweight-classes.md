[]{#Behavior_002dsupport-for-lightweight-classes}

::: header
Next: [Behavior-testing
functionality](Behavior_002dtesting-functionality.html#Behavior_002dtesting-functionality){accesskey="n"
rel="next"}, Previous: [Behavior-still
unclassified](Behavior_002dstill-unclassified.html#Behavior_002dstill-unclassified){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-support-for-lightweight-classes}

#### 1.9.21 Behavior: support for lightweight classes {#behavior-support-for-lightweight-classes .subsection}

[]{#index-article}

**article**

Answer an article ('a' or 'an') which is ok for the receiver's name

[]{#index-asClass}

**asClass**

Answer the first superclass that is a full-fledged Class object

[]{#index-environment-1}

**environment**

Answer the namespace that this class belongs to - the same as the
superclass, since Behavior does not support namespaces yet.

[]{#index-name-1}

**name**

Answer the class name; this prints to the name of the superclass
enclosed in braces. This class name is used, for example, to print the
receiver.

[]{#index-nameIn_003a-1}

**nameIn: aNamespace**

Answer the class name when the class is referenced from aNamespace - a
dummy one, since Behavior does not support names.

[]{#index-printOn_003ain_003a}

**printOn: aStream in: aNamespace**

Answer the class name when the class is referenced from aNamespace - a
dummy one, since Behavior does not support names.

[]{#index-securityPolicy}

**securityPolicy**

Not commented.

[]{#index-securityPolicy_003a}

**securityPolicy: aSecurityPolicy**

This method should not be called for instances of this class.
