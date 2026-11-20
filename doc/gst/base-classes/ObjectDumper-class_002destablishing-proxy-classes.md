[]{#ObjectDumper-class_002destablishing-proxy-classes}

::: header
Next: [ObjectDumper class-instance
creation](ObjectDumper-class_002dinstance-creation.html#ObjectDumper-class_002dinstance-creation){accesskey="n"
rel="next"}, Up:
[ObjectDumper](ObjectDumper.html#ObjectDumper){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectDumper-class_003a-establishing-proxy-classes}

#### 1.124.1 ObjectDumper class: establishing proxy classes {#objectdumper-class-establishing-proxy-classes .subsection}

[]{#index-disableProxyFor_003a}

**disableProxyFor: aClass**

Disable proxies for instances of aClass and its descendants

[]{#index-hasProxyFor_003a}

**hasProxyFor: aClass**

Answer whether a proxy class has been registered for instances of
aClass.

[]{#index-proxyClassFor_003a}

**proxyClassFor: anObject**

Answer the class of a valid proxy for an object, or nil if none could be
found

[]{#index-proxyFor_003a}

**proxyFor: anObject**

Answer a valid proxy for an object, or the object itself if none could
be found

[]{#index-registerProxyClass_003afor_003a}

**registerProxyClass: aProxyClass for: aClass**

Register the proxy class aProxyClass - descendent of DumperProxy - to be
used for instances of aClass and its descendants
