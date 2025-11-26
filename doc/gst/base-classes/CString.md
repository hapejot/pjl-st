[]{#CString}

::: header
Next: [CStringCType](CStringCType.html#CStringCType){accesskey="n"
rel="next"}, Previous:
[CSmalltalk](CSmalltalk.html#CSmalltalk){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CString-1}

### 1.49 CString {#cstring .section}

[]{#index-CString}

**Defined in namespace Smalltalk**\
**Superclass: CPtr**\
**Category: Language-C interface**

:   Technically, CString is really a pointer to CChar. However, it can
    be very useful as a distinct datatype because it is a separate
    datatype in Smalltalk, so we allow developers to express their
    semantics more precisely by using a more descriptive type.

    Note that like CChar is a pointer to char, CString is actually a
    \*pointer\* to string: a char \*\* in C terms. If you need to take a
    String out of a char \*, use CChar\>\>#asString.

    In general, I behave like a cross between an array of characters and
    a pointer to a character. I provide the protocol for both data
    types. My #value method returns a Smalltalk String, as you would
    expect for a scalar datatype.

  ----------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [CString class-accessing](CString-class_002daccessing.html#CString-class_002daccessing){accesskey="1"}:                                (class)
  • [CString class-instance creation](CString-class_002dinstance-creation.html#CString-class_002dinstance-creation){accesskey="2"}:        (class)
  • [CString-accessing](CString_002daccessing.html#CString_002daccessing){accesskey="3"}:                                                  (instance)
  ----------------------------------------------------------------------------------------------------------------------------------- ---- ------------
