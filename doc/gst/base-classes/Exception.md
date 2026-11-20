[]{#Exception}

::: header
Next: [ExceptionSet](ExceptionSet.html#ExceptionSet){accesskey="n"
rel="next"}, Previous: [Error](Error.html#Error){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Exception-1}

### 1.72 Exception {#exception .section}

[]{#index-Exception}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Exceptions**

:   My instances describe an exception that has happened, and are passed
    to exception handlers. Classes describe the kind of exception.

    Apart from containing information on the generated exception, my
    instances contain methods that allow you to resume execution, leave
    the #on:do:\... block, and pass the exception to an handler with a
    lower priority.

  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Exception class-comparison](Exception-class_002dcomparison.html#Exception-class_002dcomparison){accesskey="1"}:                                                                                         (class)
  • [Exception class-creating ExceptionCollections](Exception-class_002dcreating-ExceptionCollections.html#Exception-class_002dcreating-ExceptionCollections){accesskey="2"}:                                (class)
  • [Exception class-instance creation](Exception-class_002dinstance-creation.html#Exception-class_002dinstance-creation){accesskey="3"}:                                                                    (class)
  • [Exception class-interoperability with TrappableEvents](Exception-class_002dinteroperability-with-TrappableEvents.html#Exception-class_002dinteroperability-with-TrappableEvents){accesskey="4"}:        (class)
  • [Exception-accessing](Exception_002daccessing.html#Exception_002daccessing){accesskey="5"}:                                                                                                              (instance)
  • [Exception-built ins](Exception_002dbuilt-ins.html#Exception_002dbuilt-ins){accesskey="6"}:                                                                                                              (instance)
  • [Exception-comparison](Exception_002dcomparison.html#Exception_002dcomparison){accesskey="7"}:                                                                                                           (instance)
  • [Exception-copying](Exception_002dcopying.html#Exception_002dcopying){accesskey="8"}:                                                                                                                    (instance)
  • [Exception-exception description](Exception_002dexception-description.html#Exception_002dexception-description){accesskey="9"}:                                                                          (instance)
  • [Exception-exception handling](Exception_002dexception-handling.html#Exception_002dexception-handling):                                                                                                  (instance)
  • [Exception-exception signaling](Exception_002dexception-signaling.html#Exception_002dexception-signaling):                                                                                               (instance)
  • [Exception-still unclassified](Exception_002dstill-unclassified.html#Exception_002dstill-unclassified):                                                                                                  (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
