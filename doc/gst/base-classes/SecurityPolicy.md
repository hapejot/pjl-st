[]{#SecurityPolicy}

::: header
Next: [Semaphore](Semaphore.html#Semaphore){accesskey="n" rel="next"},
Previous:
[ScaledDecimal](ScaledDecimal.html#ScaledDecimal){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SecurityPolicy-1}

### 1.149 SecurityPolicy {#securitypolicy .section}

[]{#index-SecurityPolicy}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Security**

:   I am the class that represents which operations that could harm the
    system's security are allowed or denied to a particular class. If a
    class does not have a policy, it is allowed everything if it is
    trusted, and denied everything if it is untrusted

  -------------------------------------------------------------------------------------------------------------- ---- ------------
  • [SecurityPolicy-modifying](SecurityPolicy_002dmodifying.html#SecurityPolicy_002dmodifying){accesskey="1"}:        (instance)
  • [SecurityPolicy-querying](SecurityPolicy_002dquerying.html#SecurityPolicy_002dquerying){accesskey="2"}:           (instance)
  -------------------------------------------------------------------------------------------------------------- ---- ------------
