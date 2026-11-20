[]{#Class_002dinstance-creation}

::: header
Next: [Class-instance creation -
alternative](Class_002dinstance-creation-_002d-alternative.html#Class_002dinstance-creation-_002d-alternative){accesskey="n"
rel="next"}, Previous:
[Class-filing](Class_002dfiling.html#Class_002dfiling){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Class_003a-instance-creation}

#### 1.31.4 Class: instance creation {#class-instance-creation .subsection}

[]{#index-extend}

**extend**

Redefine a version of the receiver in the current namespace. Note: this
method can bite you in various ways when sent to system classes; read
the section on namespaces in the manual for some examples of the
problems you can encounter.

[]{#index-inheritShape}

**inheritShape**

Answer whether subclasses will have by default the same shape as this
class. The default is false.

[]{#index-subclass_003a}

**subclass: classNameString**

Define a subclass of the receiver with the given name. If the class is
already defined, don't modify its instance or class variables but still,
if necessary, recompile everything needed.

[]{#index-subclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**subclass: classNameString instanceVariableNames: stringInstVarNames
classVariableNames: stringOfClassVarNames poolDictionaries:
stringOfPoolNames category: categoryNameString**

Define a fixed subclass of the receiver with the given name, instance
variables, class variables, pool dictionaries and category. If the class
is already defined, if necessary, recompile everything needed.

[]{#index-variable_003asubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a}
[]{#index-byte-1} []{#index-int8-1} []{#index-character-1}
[]{#index-short-1} []{#index-ushort-1} []{#index-int-1}
[]{#index-uint-1} []{#index-int64-1} []{#index-uint64-1}
[]{#index-utf32-1} []{#index-float-1} []{#index-double-1}
[]{#index-pointer-1}

**variable: shape subclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a variable subclass of the receiver with the given name, shape,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.
The shape can be one of #byte #int8 #character #short #ushort #int #uint
#int64 #uint64 #utf32 #float #double or #pointer.

[]{#index-variableByteSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**variableByteSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a byte variable subclass of the receiver with the given name,
instance variables (must be "), class variables, pool dictionaries and
category. If the class is already defined, if necessary, recompile
everything needed.

[]{#index-variableSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**variableSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a variable pointer subclass of the receiver with the given name,
instance variables, class variables, pool dictionaries and category. If
the class is already defined, if necessary, recompile everything needed.

[]{#index-variableWordSubclass_003ainstanceVariableNames_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**variableWordSubclass: classNameString instanceVariableNames:
stringInstVarNames classVariableNames: stringOfClassVarNames
poolDictionaries: stringOfPoolNames category: categoryNameString**

Define a word variable subclass of the receiver with the given name,
instance variables (must be "), class variables, pool dictionaries and
category. If the class is already defined, if necessary, recompile
everything needed.

------------------------------------------------------------------------

::: header
Next: [Class-instance creation -
alternative](Class_002dinstance-creation-_002d-alternative.html#Class_002dinstance-creation-_002d-alternative){accesskey="n"
rel="next"}, Previous:
[Class-filing](Class_002dfiling.html#Class_002dfiling){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
