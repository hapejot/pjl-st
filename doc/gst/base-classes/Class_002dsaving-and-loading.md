[]{#Class_002dsaving-and-loading}

::: header
Next:
[Class-security](Class_002dsecurity.html#Class_002dsecurity){accesskey="n"
rel="next"}, Previous:
[Class-printing](Class_002dprinting.html#Class_002dprinting){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Class_003a-saving-and-loading}

#### 1.31.8 Class: saving and loading {#class-saving-and-loading .subsection}

[]{#index-binaryRepresentationVersion}

**binaryRepresentationVersion**

Answer a number \>= 0 which represents the current version of the
object's representation. The default implementation answers zero.

[]{#index-convertFromVersion_003awithFixedVariables_003aindexedVariables_003afor_003a}

**convertFromVersion: version withFixedVariables: fixed
indexedVariables: indexed for: anObjectDumper**

This method is called if a VersionableObjectProxy is attached to a
class. It receives the version number that was stored for the object (or
nil if the object did not use a VersionableObjectProxy), the fixed
instance variables, the indexed instance variables, and the ObjectDumper
that has read the object. The default implementation ignores the version
and simply fills in an instance of the receiver with the given fixed and
indexed instance variables (nil if the class instances are of fixed
size). If instance variables were removed from the class, extras are
ignored; if the class is now fixed and used to be indexed, indexed is
not used.

[]{#index-nonVersionedInstSize}

**nonVersionedInstSize**

Answer the number of instance variables that the class used to have when
objects were stored without using a VersionableObjectProxy. The default
implementation answers the current instSize.
