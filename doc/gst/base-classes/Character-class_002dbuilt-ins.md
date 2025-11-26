[]{#Character-class_002dbuilt-ins}

::: header
Next: [Character
class-constants](Character-class_002dconstants.html#Character-class_002dconstants){accesskey="n"
rel="next"}, Up: [Character](Character.html#Character){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Character-class_003a-built-ins}

#### 1.28.1 Character class: built ins {#character-class-built-ins .subsection}

[]{#index-asciiValue_003a}

**asciiValue: anInteger**

Returns the character object corresponding to anInteger. Error if
anInteger is not an integer, or not in 0..127.

[]{#index-codePoint_003a}

**codePoint: anInteger**

Returns the character object, possibly an UnicodeCharacter,
corresponding to anInteger. Error if anInteger is not an integer, or not
in 0..16r10FFFF.

[]{#index-value_003a-4}

**value: anInteger**

Returns the character object corresponding to anInteger. Error if
anInteger is not an integer, or not in 0..255.
