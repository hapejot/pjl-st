[]{#Metaclass_002dbasic}

::: header
Next: [Metaclass-compiling
methods](Metaclass_002dcompiling-methods.html#Metaclass_002dcompiling-methods){accesskey="n"
rel="next"}, Previous:
[Metaclass-accessing](Metaclass_002daccessing.html#Metaclass_002daccessing){accesskey="p"
rel="prev"}, Up: [Metaclass](Metaclass.html#Metaclass){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Metaclass_003a-basic}

#### 1.112.3 Metaclass: basic {#metaclass-basic .subsection}

[]{#index-name_003aenvironment_003asubclassOf_003a}

**name: className environment: aNamespace subclassOf: theSuperclass**

Private - create a full featured class and install it, or change the
superclass or shape of an existing one; instance variable names, class
variable names and pool dictionaries are left untouched.

[]{#index-name_003aenvironment_003asubclassOf_003ainstanceVariableArray_003ashape_003aclassPool_003apoolDictionaries_003acategory_003a}

**name: className environment: aNamespace subclassOf: newSuperclass
instanceVariableArray: variableArray shape: shape classPool:
classVarDict poolDictionaries: sharedPoolNames category: categoryName**

Private - create a full featured class and install it, or change an
existing one

[]{#index-name_003aenvironment_003asubclassOf_003ainstanceVariableNames_003ashape_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**name: newName environment: aNamespace subclassOf: theSuperclass
instanceVariableNames: stringOfInstVarNames shape: shape
classVariableNames: stringOfClassVarNames poolDictionaries:
stringOfPoolNames category: categoryName**

Private - parse the instance and class variables, and the pool
dictionaries, then create the class.

[]{#index-newMeta_003aenvironment_003asubclassOf_003ainstanceVariableArray_003ashape_003aclassPool_003apoolDictionaries_003acategory_003a}

**newMeta: className environment: aNamespace subclassOf: theSuperclass
instanceVariableArray: arrayOfInstVarNames shape: shape classPool:
classVarDict poolDictionaries: sharedPoolNames category: categoryName**

Private - create a full featured class and install it
