[]{#Metaclass_002ddelegation}

::: header
Next:
[Metaclass-filing](Metaclass_002dfiling.html#Metaclass_002dfiling){accesskey="n"
rel="next"}, Previous: [Metaclass-compiling
methods](Metaclass_002dcompiling-methods.html#Metaclass_002dcompiling-methods){accesskey="p"
rel="prev"}, Up: [Metaclass](Metaclass.html#Metaclass){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Metaclass_003a-delegation}

#### 1.112.5 Metaclass: delegation {#metaclass-delegation .subsection}

[]{#index-addClassVarName_003a-1}

**addClassVarName: aString**

Add a class variable with the given name to the class pool dictionary

[]{#index-addSharedPool_003a-3}

**addSharedPool: aDictionary**

Add the given shared pool to the list of the class' pool dictionaries

[]{#index-allClassVarNames-2}

**allClassVarNames**

Answer the names of the variables in the receiver's class pool
dictionary and in each of the superclasses' class pool dictionaries

[]{#index-allSharedPoolDictionariesDo_003a-2}

**allSharedPoolDictionariesDo: aBlock**

Answer the shared pools visible from methods in the metaclass, in the
correct search order.

[]{#index-allSharedPools-1}

**allSharedPools**

Return the names of the shared pools defined by the class and any of its
superclasses

[]{#index-category-1}

**category**

Answer the class category

[]{#index-classPool-2}

**classPool**

Answer the class pool dictionary

[]{#index-classVarNames-2}

**classVarNames**

Answer the names of the variables in the class pool dictionary

[]{#index-comment-1}

**comment**

Answer the class comment

[]{#index-debuggerClass-2}

**debuggerClass**

Answer the debugger class that was set in the instance class

[]{#index-environment-6}

**environment**

Answer the namespace in which the receiver is implemented

[]{#index-name-8}

**name**

Answer the class name - it has none, actually

[]{#index-pragmaHandlerFor_003a-1}

**pragmaHandlerFor: aSymbol**

Answer the (possibly inherited) registered handler for pragma aSymbol,
or nil if not found.

[]{#index-removeClassVarName_003a-1}

**removeClassVarName: aString**

Removes the class variable from the class, error if not present, or
still in use.

[]{#index-removeSharedPool_003a-2}

**removeSharedPool: aDictionary**

Remove the given dictionary to the list of the class' pool dictionaries

[]{#index-sharedPools-2}

**sharedPools**

Return the names of the shared pools defined by the class

------------------------------------------------------------------------

::: header
Next:
[Metaclass-filing](Metaclass_002dfiling.html#Metaclass_002dfiling){accesskey="n"
rel="next"}, Previous: [Metaclass-compiling
methods](Metaclass_002dcompiling-methods.html#Metaclass_002dcompiling-methods){accesskey="p"
rel="prev"}, Up: [Metaclass](Metaclass.html#Metaclass){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
