[]{#ObjectMemory_002dderived-information}

::: header
Previous:
[ObjectMemory-builtins](ObjectMemory_002dbuiltins.html#ObjectMemory_002dbuiltins){accesskey="p"
rel="prev"}, Up:
[ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectMemory_003a-derived-information}

#### 1.125.7 ObjectMemory: derived information {#objectmemory-derived-information .subsection}

[]{#index-scavengesBeforeTenuring}

**scavengesBeforeTenuring**

Answer the number of scavenges that an object must on average survive
before being promoted to oldspace; this is however only an estimate
because objects that are reachable from oldspace have a higher
probability to be tenured soon, while objects that are only reachable
from thisContext have a lower probability to be tenured. Anyway, if this
number falls below 2-3 you should definitely increment the size of eden
and/or of survivor space, because you are tenuring too often and relying
too much on global garbage collection to keep your heap clean; do the
measurement on a restarted image, so that the extra tenuring incurred
when creating long-lived objects such as classes or methods is not
considered.
