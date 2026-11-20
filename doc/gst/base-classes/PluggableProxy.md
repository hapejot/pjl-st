[]{#PluggableProxy}

::: header
Next: [Point](Point.html#Point){accesskey="n" rel="next"}, Previous:
[PluggableAdaptor](PluggableAdaptor.html#PluggableAdaptor){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PluggableProxy-1}

### 1.131 PluggableProxy {#pluggableproxy .section}

[]{#index-PluggableProxy}

**Defined in namespace Smalltalk**\
**Superclass: AlternativeObjectProxy**\
**Category: Streams-Files**

:   I am a proxy that stores a different object and, upon load, sends
    #reconstructOriginalObject to that object (which can be a
    DirectedMessage, in which case the message is sent). The object to
    be stored is retrieved by sending #binaryRepresentationObject to the
    object.

  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [PluggableProxy class-accessing](PluggableProxy-class_002daccessing.html#PluggableProxy-class_002daccessing){accesskey="1"}:                       (class)
  • [PluggableProxy-saving and restoring](PluggableProxy_002dsaving-and-restoring.html#PluggableProxy_002dsaving-and-restoring){accesskey="2"}:        (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
