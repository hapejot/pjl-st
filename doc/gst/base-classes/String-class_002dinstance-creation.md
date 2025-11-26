[]{#String-class_002dinstance-creation}

::: header
Next: [String class-multibyte
encodings](String-class_002dmultibyte-encodings.html#String-class_002dmultibyte-encodings){accesskey="n"
rel="next"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String-class_003a-instance-creation}

#### 1.158.1 String class: instance creation {#string-class-instance-creation .subsection}

[]{#index-fromCData_003a}

**fromCData: aCObject**

Answer a String containing the bytes starting at the location pointed to
by aCObject, up to the first NUL character.

[]{#index-fromCData_003asize_003a-1}

**fromCData: aCObject size: anInteger**

Answer a String containing anInteger bytes starting at the location
pointed to by aCObject
