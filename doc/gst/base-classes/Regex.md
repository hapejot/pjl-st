[]{#Regex}

::: header
Next: [RegexResults](RegexResults.html#RegexResults){accesskey="n"
rel="next"}, Previous:
[RecursionLock](RecursionLock.html#RecursionLock){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Regex-1}

### 1.144 Regex {#regex .section}

[]{#index-Regex}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Collections-Text**

:   A Regex is a read-only string for which the regular expression
    matcher can cache a compiled representation, thus speeding up
    matching. Regex objects are constructed automatically by methods
    that expect to match many times the same regular expression, but can
    also be constructed explicitly sending #asRegex to a String or
    Symbol.

    Creation of Regex objects inside a loop is of course slower than
    creating them outside the loop, but special care is taken so that
    the same Regex object is used whenever possible (when converting
    Strings to Regex, the cache is sought for an equivalent, already
    constructed Regex).

  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Regex class-instance creation](Regex-class_002dinstance-creation.html#Regex-class_002dinstance-creation){accesskey="1"}:        (class)
  • [Regex-basic](Regex_002dbasic.html#Regex_002dbasic){accesskey="2"}:                                                              (instance)
  • [Regex-conversion](Regex_002dconversion.html#Regex_002dconversion){accesskey="3"}:                                               (instance)
  • [Regex-printing](Regex_002dprinting.html#Regex_002dprinting){accesskey="4"}:                                                     (instance)
  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
