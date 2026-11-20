[]{#Message}

::: header
Next:
[MessageNotUnderstood](MessageNotUnderstood.html#MessageNotUnderstood){accesskey="n"
rel="next"}, Previous: [Memory](Memory.html#Memory){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Message-1}

### 1.110 Message {#message .section}

[]{#index-Message}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Implementation**

:   I represent a message send. My instances are created to hold a
    message that has failed, so that error reporting methods can examine
    the sender and arguments, but also to represent method attributes
    (like \<primitive: 111\> since their syntax is isomorphic to that of
    a message send.

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Message class-creating instances](Message-class_002dcreating-instances.html#Message-class_002dcreating-instances){accesskey="1"}:        (class)
  • [Message-accessing](Message_002daccessing.html#Message_002daccessing){accesskey="2"}:                                                     (instance)
  • [Message-basic](Message_002dbasic.html#Message_002dbasic){accesskey="3"}:                                                                 (instance)
  • [Message-printing](Message_002dprinting.html#Message_002dprinting){accesskey="4"}:                                                        (instance)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
