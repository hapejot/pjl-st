[]{#TextCollector}

::: header
Next: [Time](Time.html#Time){accesskey="n" rel="next"}, Previous:
[SystemExceptions.WrongMessageSent](SystemExceptions_002eWrongMessageSent.html#SystemExceptions_002eWrongMessageSent){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#TextCollector-1}

### 1.198 TextCollector {#textcollector .section}

[]{#index-TextCollector}

**Defined in namespace Smalltalk**\
**Superclass: Stream**\
**Category: Streams**

:   I am a thread-safe class that maps between standard Stream protocol
    and a single message to another object (its selector is pluggable
    and should roughly correspond to #nextPutAll:). I am, in fact, the
    class that implements the global Transcript object.

  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [TextCollector class-accessing](TextCollector-class_002daccessing.html#TextCollector-class_002daccessing){accesskey="1"}:        (class)
  • [TextCollector-accessing](TextCollector_002daccessing.html#TextCollector_002daccessing){accesskey="2"}:                          (instance)
  • [TextCollector-printing](TextCollector_002dprinting.html#TextCollector_002dprinting){accesskey="3"}:                             (instance)
  • [TextCollector-set up](TextCollector_002dset-up.html#TextCollector_002dset-up){accesskey="4"}:                                   (instance)
  • [TextCollector-storing](TextCollector_002dstoring.html#TextCollector_002dstoring){accesskey="5"}:                                (instance)
  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
