[]{#Memory}

::: header
Next: [Message](Message.html#Message){accesskey="n" rel="next"},
Previous:
[MappedCollection](MappedCollection.html#MappedCollection){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Memory-1}

### 1.109 Memory {#memory .section}

[]{#index-Memory}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Implementation**

:   I provide access to actual machine addresses of OOPs and objects. I
    have no instances; you send messages to my class to map between an
    object and the address of its OOP or object. In addition I provide
    direct memory access with different C types (ints, chars, OOPs,
    floats,\...).

  -------------------------------------------------------------------------------------------------------- ---- ---------
  • [Memory class-accessing](Memory-class_002daccessing.html#Memory-class_002daccessing){accesskey="1"}:        (class)
  -------------------------------------------------------------------------------------------------------- ---- ---------
