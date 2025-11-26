[]{#PositionableStream}

::: header
Next: [Process](Process.html#Process){accesskey="n" rel="next"},
Previous: [Point](Point.html#Point){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PositionableStream-1}

### 1.133 PositionableStream {#positionablestream .section}

[]{#index-PositionableStream}

**Defined in namespace Smalltalk**\
**Superclass: Stream**\
**Category: Streams-Collections**

:   My instances represent streams where explicit positioning is
    permitted. Thus, my streams act in a manner to normal disk files:
    you can read or write sequentially, but also position the file to a
    particular place whenever you choose. Generally, you'll want to use
    ReadStream, WriteStream or ReadWriteStream instead of me to create
    and use streams.

  -------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [PositionableStream class-instance creation](PositionableStream-class_002dinstance-creation.html#PositionableStream-class_002dinstance-creation){accesskey="1"}:        (class)
  • [PositionableStream-accessing-reading](PositionableStream_002daccessing_002dreading.html#PositionableStream_002daccessing_002dreading){accesskey="2"}:                  (instance)
  • [PositionableStream-class type methods](PositionableStream_002dclass-type-methods.html#PositionableStream_002dclass-type-methods){accesskey="3"}:                       (instance)
  • [PositionableStream-compiling](PositionableStream_002dcompiling.html#PositionableStream_002dcompiling){accesskey="4"}:                                                  (instance)
  • [PositionableStream-positioning](PositionableStream_002dpositioning.html#PositionableStream_002dpositioning){accesskey="5"}:                                            (instance)
  • [PositionableStream-still unclassified](PositionableStream_002dstill-unclassified.html#PositionableStream_002dstill-unclassified){accesskey="6"}:                       (instance)
  • [PositionableStream-testing](PositionableStream_002dtesting.html#PositionableStream_002dtesting){accesskey="7"}:                                                        (instance)
  • [PositionableStream-truncating](PositionableStream_002dtruncating.html#PositionableStream_002dtruncating){accesskey="8"}:                                               (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
