[]{#UndefinedObject_002dclass-polymorphism}

::: header
Next: [UndefinedObject-CObject
interoperability](UndefinedObject_002dCObject-interoperability.html#UndefinedObject_002dCObject-interoperability){accesskey="n"
rel="next"}, Previous: [UndefinedObject-class creation -
alternative](UndefinedObject_002dclass-creation-_002d-alternative.html#UndefinedObject_002dclass-creation-_002d-alternative){accesskey="p"
rel="prev"}, Up:
[UndefinedObject](UndefinedObject.html#UndefinedObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UndefinedObject_003a-class-polymorphism}

#### 1.201.3 UndefinedObject: class polymorphism {#undefinedobject-class-polymorphism .subsection}

[]{#index-allSubclasses-1}

**allSubclasses**

Return all the classes in the system.

[]{#index-instSize-1}

**instSize**

Answer '0'.

[]{#index-metaclassFor_003a}

**metaclassFor: classNameString**

Create a Metaclass object for the given class name. The metaclass is a
subclass of Class

[]{#index-methodDictionary-1}

**methodDictionary**

Answer 'nil'.

[]{#index-removeSubclass_003a-1}

**removeSubclass: aClass**

Ignored -- necessary to support disjoint class hierarchies

[]{#index-subclass_003a-1}

**subclass: classNameString**

Define a subclass of the receiver with the given name. If the class is
already defined, don't modify its instance or class variables but still,
if necessary, recompile everything needed.

[]{#index-subclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a-1}

**subclass: classNameString instanceVariableNames: stringInstVarNames
classVariableNames: stringOfClassVarNames poolDictionaries:
stringOfPoolNames category: categoryNameString**

Define a fixed subclass of the receiver with the given name, instance
variables, class variables, pool dictionaries and category. If the class
is already defined, if necessary, recompile everything needed.

[]{#index-variable_003asubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a-1}
[]{#index-byte-2} []{#index-int8-2} []{#index-character-2}
[]{#index-short-2} []{#index-ushort-2} []{#index-int-3}
[]{#index-uint-2} []{#index-int64-2} []{#index-uint64-2}
[]{#index-utf32-2} []{#index-float-2} []{#index-double-2}
[]{#index-pointer-2}

**variable: shape subclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a variable subclass of the receiver with the given name, shape,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.
The shape can be one of #byte #int8 #character #short #ushort #int #uint
#int64 #uint64 #utf32 #float #double or #pointer.

[]{#index-variableByteSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a-1}

**variableByteSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a byte variable subclass of the receiver with the given name,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.

[]{#index-variableSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a-1}

**variableSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a variable pointer subclass of the receiver with the given name,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.

[]{#index-variableWordSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a-1}

**variableWordSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a word variable subclass of the receiver with the given name,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.

------------------------------------------------------------------------

::: header
Next: [UndefinedObject-CObject
interoperability](UndefinedObject_002dCObject-interoperability.html#UndefinedObject_002dCObject-interoperability){accesskey="n"
rel="next"}, Previous: [UndefinedObject-class creation -
alternative](UndefinedObject_002dclass-creation-_002d-alternative.html#UndefinedObject_002dclass-creation-_002d-alternative){accesskey="p"
rel="prev"}, Up:
[UndefinedObject](UndefinedObject.html#UndefinedObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
