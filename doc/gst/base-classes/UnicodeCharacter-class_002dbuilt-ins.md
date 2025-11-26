[]{#UnicodeCharacter-class_002dbuilt-ins}

::: header
Next: [UnicodeCharacter-coercion
methods](UnicodeCharacter_002dcoercion-methods.html#UnicodeCharacter_002dcoercion-methods){accesskey="n"
rel="next"}, Up:
[UnicodeCharacter](UnicodeCharacter.html#UnicodeCharacter){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#UnicodeCharacter-class_003a-built-ins}

#### 1.202.1 UnicodeCharacter class: built ins {#unicodecharacter-class-built-ins .subsection}

[]{#index-value_003a-19} []{#index-value_003a-27}
[]{#index-codePoint_003a-1}

**value: anInteger**

Returns the character object, possibly a Character, corresponding to
anInteger. Error if anInteger is not an integer, or not in 0..16r10FFFF.

This is only a primitive for speed. UnicodeCharacter's #value: method is
equivalent to #codePoint: (which is the same for Character and
UnicodeCharacter).
