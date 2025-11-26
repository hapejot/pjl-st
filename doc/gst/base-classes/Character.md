[]{#Character}

::: header
Next: [CharacterArray](CharacterArray.html#CharacterArray){accesskey="n"
rel="next"}, Previous:
[CFunctionDescriptor](CFunctionDescriptor.html#CFunctionDescriptor){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Character-1}

### 1.28 Character {#character .section}

[]{#index-Character}

**Defined in namespace Smalltalk**\
**Superclass: Magnitude**\
**Category: Language-Data types**

:   My instances represent the 256 characters of the character set. I
    provide messages to translate between integers and character
    objects, and provide names for some of the common unprintable
    characters.

    Character is always used (mostly for performance reasons) when
    referring to characters whose code point is between 0 and 127. Above
    127, instead, more care is needed: Character refers to bytes that
    are used as part of encoding of a character, while UnicodeCharacter
    refers to the character itself.

  -------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Character class-built ins](Character-class_002dbuilt-ins.html#Character-class_002dbuilt-ins){accesskey="1"}:                                                           (class)
  • [Character class-constants](Character-class_002dconstants.html#Character-class_002dconstants){accesskey="2"}:                                                           (class)
  • [Character class-initializing lookup tables](Character-class_002dinitializing-lookup-tables.html#Character-class_002dinitializing-lookup-tables){accesskey="3"}:        (class)
  • [Character class-instance creation](Character-class_002dinstance-creation.html#Character-class_002dinstance-creation){accesskey="4"}:                                   (class)
  • [Character class-testing](Character-class_002dtesting.html#Character-class_002dtesting){accesskey="5"}:                                                                 (class)
  • [Character-built ins](Character_002dbuilt-ins.html#Character_002dbuilt-ins){accesskey="6"}:                                                                             (instance)
  • [Character-coercion methods](Character_002dcoercion-methods.html#Character_002dcoercion-methods){accesskey="7"}:                                                        (instance)
  • [Character-comparing](Character_002dcomparing.html#Character_002dcomparing){accesskey="8"}:                                                                             (instance)
  • [Character-converting](Character_002dconverting.html#Character_002dconverting){accesskey="9"}:                                                                          (instance)
  • [Character-printing](Character_002dprinting.html#Character_002dprinting):                                                                                               (instance)
  • [Character-storing](Character_002dstoring.html#Character_002dstoring):                                                                                                  (instance)
  • [Character-testing](Character_002dtesting.html#Character_002dtesting):                                                                                                  (instance)
  • [Character-testing functionality](Character_002dtesting-functionality.html#Character_002dtesting-functionality):                                                        (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
