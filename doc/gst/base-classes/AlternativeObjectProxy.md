[]{#AlternativeObjectProxy}

::: header
Next:
[ArithmeticError](ArithmeticError.html#ArithmeticError){accesskey="n"
rel="next"}, Previous:
[AbstractNamespace](AbstractNamespace.html#AbstractNamespace){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#AlternativeObjectProxy-1}

### 1.2 AlternativeObjectProxy {#alternativeobjectproxy .section}

[]{#index-AlternativeObjectProxy}

**Defined in namespace Smalltalk**\
**Superclass: DumperProxy**\
**Category: Streams-Files**

:   I am a proxy that uses the same ObjectDumper to store an object
    which is not the object to be dumped, but from which the dumped
    object can be reconstructed. I am an abstract class, using me would
    result in infinite loops because by default I try to store the same
    object again and again. See the method comments for more information

  -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [AlternativeObjectProxy class-instance creation](AlternativeObjectProxy-class_002dinstance-creation.html#AlternativeObjectProxy-class_002dinstance-creation){accesskey="1"}:        (class)
  • [AlternativeObjectProxy-accessing](AlternativeObjectProxy_002daccessing.html#AlternativeObjectProxy_002daccessing){accesskey="2"}:                                                  (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
