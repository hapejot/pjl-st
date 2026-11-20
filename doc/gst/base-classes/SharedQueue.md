[]{#SharedQueue}

::: header
Next: [SingletonProxy](SingletonProxy.html#SingletonProxy){accesskey="n"
rel="next"}, Previous: [Set](Set.html#Set){accesskey="p" rel="prev"},
Up: [Base classes](Base-classes.html#Base-classes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SharedQueue-1}

### 1.153 SharedQueue {#sharedqueue .section}

[]{#index-SharedQueue}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Processes**

:   My instances provide a guaranteed safe mechanism to allow for
    communication between processes. All access to the underlying data
    structures is controlled with critical sections so that things
    proceed smoothly.

  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [SharedQueue class-instance creation](SharedQueue-class_002dinstance-creation.html#SharedQueue-class_002dinstance-creation){accesskey="1"}:        (class)
  • [SharedQueue-accessing](SharedQueue_002daccessing.html#SharedQueue_002daccessing){accesskey="2"}:                                                  (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
