[]{#SingletonProxy-class_002dinstance-creation}

::: header
Next: [SingletonProxy-saving and
restoring](SingletonProxy_002dsaving-and-restoring.html#SingletonProxy_002dsaving-and-restoring){accesskey="n"
rel="next"}, Previous: [SingletonProxy
class-accessing](SingletonProxy-class_002daccessing.html#SingletonProxy-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[SingletonProxy](SingletonProxy.html#SingletonProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SingletonProxy-class_003a-instance-creation}

#### 1.154.2 SingletonProxy class: instance creation {#singletonproxy-class-instance-creation .subsection}

[]{#index-on_003a-10}

**on: anObject**

Answer a proxy to be used to save anObject. The proxy stores the class
and restores the object by looking into a dictionary of class -\>
singleton objects.
