[]{#VersionableObjectProxy-class_002dsaving-and-restoring}

::: header
Next: [VersionableObjectProxy-saving and
restoring](VersionableObjectProxy_002dsaving-and-restoring.html#VersionableObjectProxy_002dsaving-and-restoring){accesskey="n"
rel="next"}, Up:
[VersionableObjectProxy](VersionableObjectProxy.html#VersionableObjectProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VersionableObjectProxy-class_003a-saving-and-restoring}

#### 1.207.1 VersionableObjectProxy class: saving and restoring {#versionableobjectproxy-class-saving-and-restoring .subsection}

[]{#index-loadFrom_003a-3} []{#index-binaryRepresentationVersion-1}
[]{#index-convertFromVersion_003awithFixedVariables_003ainstanceVariables_003afor_003a}
[]{#index-nonVersionedInstSize-1}

**loadFrom: anObjectDumper**

Retrieve the object. If the version number doesn't match the
#binaryRepresentationVersion answered by the class, call the class'
#convertFromVersion:withFixedVariables:instanceVariables:for: method.
The stored version number will be the first parameter to that method (or
nil if the stored object did not employ a VersionableObjectProxy), the
remaining parameters will be respectively the fixed instance variables,
the indexed instance variables (or nil if the class is fixed), and the
ObjectDumper itself. If no VersionableObjectProxy, the class is sent
#nonVersionedInstSize to retrieve the number of fixed instance variables
stored for the non-versioned object.
