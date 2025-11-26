[]{#Behavior_002dtesting-the-class-hierarchy}

::: header
Next: [Behavior-testing the form of the
instances](Behavior_002dtesting-the-form-of-the-instances.html#Behavior_002dtesting-the-form-of-the-instances){accesskey="n"
rel="next"}, Previous: [Behavior-testing
functionality](Behavior_002dtesting-functionality.html#Behavior_002dtesting-functionality){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-testing-the-class-hierarchy}

#### 1.9.23 Behavior: testing the class hierarchy {#behavior-testing-the-class-hierarchy .subsection}

[]{#index-includesBehavior_003a}

**includesBehavior: aClass**

Returns true if aClass is the receiver or a superclass of the receiver.

[]{#index-inheritsFrom_003a}

**inheritsFrom: aClass**

Returns true if aClass is a superclass of the receiver

[]{#index-kindOfSubclass}

**kindOfSubclass**

Return a string indicating the type of class the receiver is

[]{#index-shape}

**shape**

Answer the symbolic shape of my instances.

[]{#index-shape_003a} []{#index-byte} []{#index-int8}
[]{#index-character} []{#index-short} []{#index-word} []{#index-ushort}
[]{#index-int} []{#index-uint} []{#index-int64} []{#index-uint64}
[]{#index-utf32} []{#index-float} []{#index-double} []{#index-pointer}
[]{#index-inherit} []{#index-inherit-1}

**shape: shape**

Give the provided shape to the receiver's instances. The shape can be
nil, or one of #byte #int8 #character #short #word #ushort #int #uint
#int64 #uint64 #utf32 #float #double or #pointer. In addition, the
special value #inherit means to use the shape of the superclass; note
however that this is a static setting, and subclasses that used #inherit
are not mutated when the superclass adopts a different shape.
