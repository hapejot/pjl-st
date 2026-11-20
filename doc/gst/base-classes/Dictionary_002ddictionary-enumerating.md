[]{#Dictionary_002ddictionary-enumerating}

::: header
Next: [Dictionary-dictionary
removing](Dictionary_002ddictionary-removing.html#Dictionary_002ddictionary-removing){accesskey="n"
rel="next"}, Previous:
[Dictionary-compilation](Dictionary_002dcompilation.html#Dictionary_002dcompilation){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Dictionary_003a-dictionary-enumerating}

#### 1.64.5 Dictionary: dictionary enumerating {#dictionary-dictionary-enumerating .subsection}

[]{#index-associationsDo_003a}

**associationsDo: aBlock**

Pass each association in the dictionary to aBlock

[]{#index-collect_003a-2}

**collect: aBlock**

Answer a new dictionary where the keys are the same and the values are
obtained by passing each value to aBlock and collecting the return
values

[]{#index-do_003a-1}

**do: aBlock**

Pass each value in the dictionary to aBlock

[]{#index-keysAndValuesDo_003a}

**keysAndValuesDo: aBlock**

Pass each key/value pair in the dictionary as two distinct parameters to
aBlock

[]{#index-keysDo_003a}

**keysDo: aBlock**

Pass each key in the dictionary to aBlock

[]{#index-reject_003a-2}

**reject: aBlock**

Answer a new dictionary containing the key/value pairs for which aBlock
returns false. aBlock only receives the value part of the pairs.

[]{#index-select_003a-2}

**select: aBlock**

Answer a new dictionary containing the key/value pairs for which aBlock
returns true. aBlock only receives the value part of the pairs.
