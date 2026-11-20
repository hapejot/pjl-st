[]{#Class_002daccessing-instances-and-variables}

::: header
Next:
[Class-filing](Class_002dfiling.html#Class_002dfiling){accesskey="n"
rel="next"}, Previous: [Class
class-initialize](Class-class_002dinitialize.html#Class-class_002dinitialize){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Class_003a-accessing-instances-and-variables}

#### 1.31.2 Class: accessing instances and variables {#class-accessing-instances-and-variables .subsection}

[]{#index-addClassVarName_003a}

**addClassVarName: aString**

Add a class variable with the given name to the class pool dictionary.

[]{#index-addClassVarName_003avalue_003a}

**addClassVarName: aString value: valueBlock**

Add a class variable with the given name to the class pool dictionary,
and evaluate valueBlock as its initializer.

[]{#index-addSharedPool_003a-1}

**addSharedPool: aDictionary**

Add the given shared pool to the list of the class' pool dictionaries

[]{#index-allClassVarNames-1}

**allClassVarNames**

Answer the names of the variables in the receiver's class pool
dictionary and in each of the superclasses' class pool dictionaries

[]{#index-bindingFor_003a}

**bindingFor: aString**

Answer the variable binding for the class variable with the given name

[]{#index-category}

**category**

Answer the class category

[]{#index-category_003a}

**category: aString**

Change the class category to aString

[]{#index-classPool-1}

**classPool**

Answer the class pool dictionary

[]{#index-classPragmas-1}

**classPragmas**

Return the pragmas that are written in the file-out of this class.

[]{#index-classVarNames-1}

**classVarNames**

Answer the names of the variables in the class pool dictionary

[]{#index-comment}

**comment**

Answer the class comment

[]{#index-comment_003a}

**comment: aString**

Change the class name

[]{#index-environment-3}

**environment**

Answer 'environment'.

[]{#index-environment_003a-2}

**environment: aNamespace**

Set the receiver's environment to aNamespace and recompile everything

[]{#index-initialize-2}

**initialize**

redefined in children (?)

[]{#index-initializeAsRootClass}

**initializeAsRootClass**

Perform special initialization reserved to root classes.

[]{#index-name-4}

**name**

Answer the class name

[]{#index-removeClassVarName_003a}

**removeClassVarName: aString**

Removes the class variable from the class, error if not present, or
still in use.

[]{#index-removeSharedPool_003a-1}

**removeSharedPool: aDictionary**

Remove the given dictionary to the list of the class' pool dictionaries

[]{#index-sharedPools-1}

**sharedPools**

Return the names of the shared pools defined by the class

[]{#index-superclass_003a-1}

**superclass: aClass**

Set the receiver's superclass.

------------------------------------------------------------------------

::: header
Next:
[Class-filing](Class_002dfiling.html#Class_002dfiling){accesskey="n"
rel="next"}, Previous: [Class
class-initialize](Class-class_002dinitialize.html#Class-class_002dinitialize){accesskey="p"
rel="prev"}, Up: [Class](Class.html#Class){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
