[]{#CObject}

::: header
Next: [Collection](Collection.html#Collection){accesskey="n"
rel="next"}, Previous:
[CLongLong](CLongLong.html#CLongLong){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject-1}

### 1.36 CObject {#cobject .section}

[]{#index-CObject}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-C interface**

:   I am not part of the standard Smalltalk kernel class hierarchy. My
    instances contain values that are not interpreted by the Smalltalk
    system; they frequently hold \"pointers\" to data outside of the
    Smalltalk environment. The C callout mechanism allows my instances
    to be transformed into their corresponding C values for use in
    external routines.

  -------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [CObject class-conversion](CObject-class_002dconversion.html#CObject-class_002dconversion){accesskey="1"}:                                      (class)
  • [CObject class-instance creation](CObject-class_002dinstance-creation.html#CObject-class_002dinstance-creation){accesskey="2"}:                 (class)
  • [CObject class-primitive allocation](CObject-class_002dprimitive-allocation.html#CObject-class_002dprimitive-allocation){accesskey="3"}:        (class)
  • [CObject class-subclass creation](CObject-class_002dsubclass-creation.html#CObject-class_002dsubclass-creation){accesskey="4"}:                 (class)
  • [CObject-accessing](CObject_002daccessing.html#CObject_002daccessing){accesskey="5"}:                                                           (instance)
  • [CObject-basic](CObject_002dbasic.html#CObject_002dbasic){accesskey="6"}:                                                                       (instance)
  • [CObject-C data access](CObject_002dC-data-access.html#CObject_002dC-data-access){accesskey="7"}:                                               (instance)
  • [CObject-conversion](CObject_002dconversion.html#CObject_002dconversion){accesskey="8"}:                                                        (instance)
  • [CObject-finalization](CObject_002dfinalization.html#CObject_002dfinalization){accesskey="9"}:                                                  (instance)
  • [CObject-pointer-like behavior](CObject_002dpointer_002dlike-behavior.html#CObject_002dpointer_002dlike-behavior):                              (instance)
  • [CObject-testing](CObject_002dtesting.html#CObject_002dtesting):                                                                                (instance)
  • [CObject-testing functionality](CObject_002dtesting-functionality.html#CObject_002dtesting-functionality):                                      (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
