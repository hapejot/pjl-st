[]{#ObjectDumper}

::: header
Next: [ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="n"
rel="next"}, Previous: [Object](Object.html#Object){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectDumper-1}

### 1.124 ObjectDumper {#objectdumper .section}

[]{#index-ObjectDumper}

**Defined in namespace Smalltalk**\
**Superclass: Stream**\
**Category: Streams-Files**

:   I'm not part of a normal Smalltalk system, but most Smalltalks
    provide a similar feature: that is, support for storing objects in a
    binary format; there are many advantages in using me instead of
    #storeOn: and the Smalltalk compiler.

    The data is stored in a very compact format, which has the side
    effect of making loading much faster when compared with compiling
    the Smalltalk code prepared by #storeOn:. In addition, my instances
    support circular references between objects, while #storeOn:
    supports it only if you know of such references at design time and
    you override #storeOn: to deal with them

  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [ObjectDumper class-establishing proxy classes](ObjectDumper-class_002destablishing-proxy-classes.html#ObjectDumper-class_002destablishing-proxy-classes){accesskey="1"}:        (class)
  • [ObjectDumper class-instance creation](ObjectDumper-class_002dinstance-creation.html#ObjectDumper-class_002dinstance-creation){accesskey="2"}:                                   (class)
  • [ObjectDumper class-shortcuts](ObjectDumper-class_002dshortcuts.html#ObjectDumper-class_002dshortcuts){accesskey="3"}:                                                           (class)
  • [ObjectDumper class-testing](ObjectDumper-class_002dtesting.html#ObjectDumper-class_002dtesting){accesskey="4"}:                                                                 (class)
  • [ObjectDumper-accessing](ObjectDumper_002daccessing.html#ObjectDumper_002daccessing){accesskey="5"}:                                                                             (instance)
  • [ObjectDumper-loading/dumping objects](ObjectDumper_002dloading_002fdumping-objects.html#ObjectDumper_002dloading_002fdumping-objects){accesskey="6"}:                           (instance)
  • [ObjectDumper-stream interface](ObjectDumper_002dstream-interface.html#ObjectDumper_002dstream-interface){accesskey="7"}:                                                        (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
