[]{#Metaclass}

::: header
Next: [MethodContext](MethodContext.html#MethodContext){accesskey="n"
rel="next"}, Previous:
[MessageNotUnderstood](MessageNotUnderstood.html#MessageNotUnderstood){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Metaclass-1}

### 1.112 Metaclass {#metaclass .section}

[]{#index-Metaclass}

**Defined in namespace Smalltalk**\
**Superclass: ClassDescription**\
**Category: Language-Implementation**

:   I am the root of the class hierarchy. My instances are metaclasses,
    one for each real class. My instances have a single instance, which
    they hold onto, which is the class that they are the metaclass of. I
    provide methods for creation of actual class objects from metaclass
    object, and the creation of metaclass objects, which are my
    instances. If this is confusing to you, it should be\...the
    Smalltalk metaclass system is strange and complex.

  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Metaclass class-instance creation](Metaclass-class_002dinstance-creation.html#Metaclass-class_002dinstance-creation){accesskey="1"}:        (class)
  • [Metaclass-accessing](Metaclass_002daccessing.html#Metaclass_002daccessing){accesskey="2"}:                                                  (instance)
  • [Metaclass-basic](Metaclass_002dbasic.html#Metaclass_002dbasic){accesskey="3"}:                                                              (instance)
  • [Metaclass-compiling methods](Metaclass_002dcompiling-methods.html#Metaclass_002dcompiling-methods){accesskey="4"}:                          (instance)
  • [Metaclass-delegation](Metaclass_002ddelegation.html#Metaclass_002ddelegation){accesskey="5"}:                                               (instance)
  • [Metaclass-filing](Metaclass_002dfiling.html#Metaclass_002dfiling){accesskey="6"}:                                                           (instance)
  • [Metaclass-printing](Metaclass_002dprinting.html#Metaclass_002dprinting){accesskey="7"}:                                                     (instance)
  • [Metaclass-testing functionality](Metaclass_002dtesting-functionality.html#Metaclass_002dtesting-functionality){accesskey="8"}:              (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
