[]{#CString-class_002dinstance-creation}

::: header
Next:
[CString-accessing](CString_002daccessing.html#CString_002daccessing){accesskey="n"
rel="next"}, Previous: [CString
class-accessing](CString-class_002daccessing.html#CString-class_002daccessing){accesskey="p"
rel="prev"}, Up: [CString](CString.html#CString){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CString-class_003a-instance-creation}

#### 1.49.2 CString class: instance creation {#cstring-class-instance-creation .subsection}

[]{#index-type-5}

**type**

Answer a CType for the receiver---for example, CByteType if the receiver
is CByte.

[]{#index-value_003a-9} []{#index-addToBeFinalized-5}

**value: anObject**

Answer a newly allocated CObject containing the passed value, anObject.
Remember to call #addToBeFinalized if you want the CObject to be
automatically freed
