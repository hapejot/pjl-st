[]{#CScalar_002daccessing}

::: header
Previous: [CScalar class-instance
creation](CScalar-class_002dinstance-creation.html#CScalar-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [CScalar](CScalar.html#CScalar){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CScalar_003a-accessing}

#### 1.45.2 CScalar: accessing {#cscalar-accessing .subsection}

[]{#index-cObjStoredType-16} []{#index-at_003aput_003a-26}

**cObjStoredType**

Private - Provide a conversion from a CObject to a Smalltalk object to
be stored by #at:put:

[]{#index-value-7}

**value**

Answer the value the receiver is pointing to. The exact returned value
depends on the receiver's class

[]{#index-value_003a-8}

**value: aValue**

Set the receiver to point to the value, aValue. The exact meaning of
aValue depends on the receiver's class
