[]{#NullProxy}

::: header
Next:
[NullValueHolder](NullValueHolder.html#NullValueHolder){accesskey="n"
rel="next"}, Previous:
[Notification](Notification.html#Notification){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#NullProxy-1}

### 1.120 NullProxy {#nullproxy .section}

[]{#index-NullProxy}

**Defined in namespace Smalltalk**\
**Superclass: AlternativeObjectProxy**\
**Category: Streams-Files**

:   I am a proxy that does no special processing on the object to be
    saved. I can be used to disable proxies for particular subclasses.
    My subclasses add to the stored information, but share the fact that
    the format is about the same as that of #dump: without a proxy.

  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [NullProxy class-instance creation](NullProxy-class_002dinstance-creation.html#NullProxy-class_002dinstance-creation){accesskey="1"}:        (class)
  • [NullProxy-accessing](NullProxy_002daccessing.html#NullProxy_002daccessing){accesskey="2"}:                                                  (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
