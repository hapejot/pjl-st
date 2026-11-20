[]{#Autoload-class_002dinstance-creation}

::: header
Next:
[Autoload-accessing](Autoload_002daccessing.html#Autoload_002daccessing){accesskey="n"
rel="next"}, Up: [Autoload](Autoload.html#Autoload){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Autoload-class_003a-instance-creation}

#### 1.7.1 Autoload class: instance creation {#autoload-class-instance-creation .subsection}

[]{#index-class_003afrom_003a}

**class: nameSymbol from: fileNameString**

Make Smalltalk automatically load the class named nameSymbol from
fileNameString when needed

[]{#index-class_003ain_003afrom_003a}

**class: nameSymbol in: aNamespace from: fileNameString**

Make Smalltalk automatically load the class named nameSymbol and
residing in aNamespace from fileNameString when needed

[]{#index-class_003ain_003aloader_003a} []{#index-autoload}

**class: nameSymbol in: aNamespace loader: anObject**

Make Smalltalk automatically load the class named nameSymbol and
residing in aNamespace. When the class is needed, anObject will be sent
#autoload. By default, instances of FilePath and Package can be used.

[]{#index-class_003aloader_003a} []{#index-autoload-1}

**class: nameSymbol loader: anObject**

Make Smalltalk automatically load the class named nameSymbol. When the
class is needed, anObject will be sent #autoload. By default, instances
of FilePath and Package can be used.
