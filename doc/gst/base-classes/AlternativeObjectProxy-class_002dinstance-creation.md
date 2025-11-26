[]{#AlternativeObjectProxy-class_002dinstance-creation}

::: header
Next:
[AlternativeObjectProxy-accessing](AlternativeObjectProxy_002daccessing.html#AlternativeObjectProxy_002daccessing){accesskey="n"
rel="next"}, Up:
[AlternativeObjectProxy](AlternativeObjectProxy.html#AlternativeObjectProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#AlternativeObjectProxy-class_003a-instance-creation}

#### 1.2.1 AlternativeObjectProxy class: instance creation {#alternativeobjectproxy-class-instance-creation .subsection}

[]{#index-acceptUsageForClass_003a}

**acceptUsageForClass: aClass**

The receiver was asked to be used as a proxy for the class aClass.
Answer whether the registration is fine. By default, answer true except
if AlternativeObjectProxy itself is being used.

[]{#index-on_003a} []{#index-on_003a-13} []{#index-dumpTo_003a-3}

**on: anObject**

Answer a proxy to be used to save anObject. IMPORTANT: this method MUST
be overridden so that the overridden version sends #on: to super passing
an object that is NOT the same as anObject (alternatively, you can
override #dumpTo:, which is what NullProxy does), because that would
result in an infinite loop! This also means that AlternativeObjectProxy
must never be used directly -- only as a superclass.
