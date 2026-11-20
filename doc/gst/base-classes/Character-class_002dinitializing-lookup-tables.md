[]{#Character-class_002dinitializing-lookup-tables}

::: header
Next: [Character class-instance
creation](Character-class_002dinstance-creation.html#Character-class_002dinstance-creation){accesskey="n"
rel="next"}, Previous: [Character
class-constants](Character-class_002dconstants.html#Character-class_002dconstants){accesskey="p"
rel="prev"}, Up: [Character](Character.html#Character){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Character-class_003a-initializing-lookup-tables}

#### 1.28.3 Character class: initializing lookup tables {#character-class-initializing-lookup-tables .subsection}

[]{#index-initialize}

**initialize**

Initialize the lookup table which is used to make case and digit-to-char
conversions faster. Indices in Table are ASCII values incremented by
one. Indices 1-256 classify chars (0 = nothing special, 2 = separator,
48 = digit, 55 = uppercase, 3 = lowercase), indices 257-512 map to
lowercase chars, indices 513-768 map to uppercase chars.
