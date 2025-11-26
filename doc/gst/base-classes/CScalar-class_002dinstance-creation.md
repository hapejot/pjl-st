[]{#CScalar-class_002dinstance-creation}

::: header
Next:
[CScalar-accessing](CScalar_002daccessing.html#CScalar_002daccessing){accesskey="n"
rel="next"}, Up: [CScalar](CScalar.html#CScalar){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CScalar-class_003a-instance-creation}

#### 1.45.1 CScalar class: instance creation {#cscalar-class-instance-creation .subsection}

[]{#index-gcValue_003a}

**gcValue: anObject**

Answer a newly allocated CObject containing the passed value, anObject,
in garbage-collected storage.

[]{#index-type-4}

**type**

Answer a CType for the receiver---for example, CByteType if the receiver
is CByte.

[]{#index-value_003a-7} []{#index-addToBeFinalized-4}

**value: anObject**

Answer a newly allocated CObject containing the passed value, anObject.
Remember to call #addToBeFinalized if you want the CObject to be
automatically freed
