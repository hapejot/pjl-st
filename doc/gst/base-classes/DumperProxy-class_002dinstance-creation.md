[]{#DumperProxy-class_002dinstance-creation}

::: header
Next: [DumperProxy-saving and
restoring](DumperProxy_002dsaving-and-restoring.html#DumperProxy_002dsaving-and-restoring){accesskey="n"
rel="next"}, Previous: [DumperProxy
class-accessing](DumperProxy-class_002daccessing.html#DumperProxy-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[DumperProxy](DumperProxy.html#DumperProxy){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DumperProxy-class_003a-instance-creation}

#### 1.68.2 DumperProxy class: instance creation {#dumperproxy-class-instance-creation .subsection}

[]{#index-on_003a-1} []{#index-dumpTo_003a-4}

**on: anObject**

Answer a proxy to be used to save anObject. This method MUST be
overridden and anObject must NOT be stored in the object's instance
variables unless you override #dumpTo:, because that would result in an
infinite loop!
