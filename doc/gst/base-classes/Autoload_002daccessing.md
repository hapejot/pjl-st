[]{#Autoload_002daccessing}

::: header
Previous: [Autoload class-instance
creation](Autoload-class_002dinstance-creation.html#Autoload-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Autoload](Autoload.html#Autoload){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Autoload_003a-accessing}

#### 1.7.2 Autoload: accessing {#autoload-accessing .subsection}

[]{#index-class}

**class**

We need it to access the metaclass instance, because that's what will
load the file.

[]{#index-doesNotUnderstand_003a}

**doesNotUnderstand: aMessage**

Load the class and resend the message to it
