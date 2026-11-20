[]{#BlockClosure}

::: header
Next: [BlockContext](BlockContext.html#BlockContext){accesskey="n"
rel="next"}, Previous:
[BindingDictionary](BindingDictionary.html#BindingDictionary){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure-1}

### 1.11 BlockClosure {#blockclosure .section}

[]{#index-BlockClosure}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Implementation**

:   I am a factotum class. My instances represent Smalltalk blocks,
    portions of executeable code that have access to the environment
    that they were declared in, take parameters, and can be passed
    around as objects to be executed by methods outside the current
    class. Block closures are sent a message to compute their value and
    create a new execution context; this property can be used in the
    construction of control flow methods. They also provide some methods
    that are used in the creation of Processes from blocks.

  -------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [BlockClosure class-instance creation](BlockClosure-class_002dinstance-creation.html#BlockClosure-class_002dinstance-creation){accesskey="1"}:        (class)
  • [BlockClosure class-testing](BlockClosure-class_002dtesting.html#BlockClosure-class_002dtesting){accesskey="2"}:                                      (class)
  • [BlockClosure-accessing](BlockClosure_002daccessing.html#BlockClosure_002daccessing){accesskey="3"}:                                                  (instance)
  • [BlockClosure-built ins](BlockClosure_002dbuilt-ins.html#BlockClosure_002dbuilt-ins){accesskey="4"}:                                                  (instance)
  • [BlockClosure-control structures](BlockClosure_002dcontrol-structures.html#BlockClosure_002dcontrol-structures){accesskey="5"}:                       (instance)
  • [BlockClosure-exception handling](BlockClosure_002dexception-handling.html#BlockClosure_002dexception-handling){accesskey="6"}:                       (instance)
  • [BlockClosure-multiple process](BlockClosure_002dmultiple-process.html#BlockClosure_002dmultiple-process){accesskey="7"}:                             (instance)
  • [BlockClosure-overriding](BlockClosure_002doverriding.html#BlockClosure_002doverriding){accesskey="8"}:                                               (instance)
  • [BlockClosure-testing](BlockClosure_002dtesting.html#BlockClosure_002dtesting){accesskey="9"}:                                                        (instance)
  • [BlockClosure-unwind protection](BlockClosure_002dunwind-protection.html#BlockClosure_002dunwind-protection):                                         (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
