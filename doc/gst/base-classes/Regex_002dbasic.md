[]{#Regex_002dbasic}

::: header
Next:
[Regex-conversion](Regex_002dconversion.html#Regex_002dconversion){accesskey="n"
rel="next"}, Previous: [Regex class-instance
creation](Regex-class_002dinstance-creation.html#Regex-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Regex](Regex.html#Regex){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Regex_003a-basic}

#### 1.144.2 Regex: basic {#regex-basic .subsection}

[]{#index-at_003aput_003a-18}

**at: anIndex put: anObject**

Fail. Regex objects are read-only.

[]{#index-copy-4}

**copy**

Answer the receiver; instances of Regex are identity objects because
their only purpose is to ease caching, and we obtain better caching if
we avoid copying Regex objects
