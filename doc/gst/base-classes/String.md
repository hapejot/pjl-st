[]{#String}

::: header
Next: [Symbol](Symbol.html#Symbol){accesskey="n" rel="next"}, Previous:
[Stream](Stream.html#Stream){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String-1}

### 1.158 String {#string .section}

[]{#index-String}

**Defined in namespace Smalltalk**\
**Superclass: CharacterArray**\
**Category: Collections-Text**

:   My instances represent 8-bit character strings. Being a very common
    case, they are particularly optimized.

    Note that, if you care about multilingualization, you should treat
    String only as an encoded representation of a UnicodeString. The
    I18N package adds more Unicode-friendliness to the system so that
    encoding and decoding is performed automatically in more cases. In
    that case, String represents a case when the encoding is either
    unknown, irrelevant, or assumed to be the system default.

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [String class-instance creation](String-class_002dinstance-creation.html#String-class_002dinstance-creation){accesskey="1"}:              (class)
  • [String class-multibyte encodings](String-class_002dmultibyte-encodings.html#String-class_002dmultibyte-encodings){accesskey="2"}:        (class)
  • [String-accessing](String_002daccessing.html#String_002daccessing){accesskey="3"}:                                                        (instance)
  • [String-basic](String_002dbasic.html#String_002dbasic){accesskey="4"}:                                                                    (instance)
  • [String-built ins](String_002dbuilt-ins.html#String_002dbuilt-ins){accesskey="5"}:                                                        (instance)
  • [String-CObject](String_002dCObject.html#String_002dCObject){accesskey="6"}:                                                              (instance)
  • [String-converting](String_002dconverting.html#String_002dconverting){accesskey="7"}:                                                     (instance)
  • [String-filesystem](String_002dfilesystem.html#String_002dfilesystem){accesskey="8"}:                                                     (instance)
  • [String-printing](String_002dprinting.html#String_002dprinting){accesskey="9"}:                                                           (instance)
  • [String-regex](String_002dregex.html#String_002dregex):                                                                                   (instance)
  • [String-still unclassified](String_002dstill-unclassified.html#String_002dstill-unclassified):                                            (instance)
  • [String-testing functionality](String_002dtesting-functionality.html#String_002dtesting-functionality):                                   (instance)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
