[]{#CString_002daccessing}

::: header
Previous: [CString class-instance
creation](CString-class_002dinstance-creation.html#CString-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [CString](CString.html#CString){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CString_003a-accessing}

#### 1.49.3 CString: accessing {#cstring-accessing .subsection}

[]{#index-cObjStoredType-22} []{#index-at_003aput_003a-28}

**cObjStoredType**

Private - Provide a conversion from a CObject to a Smalltalk object to
be stored by #at:put:

[]{#index-value-8}

**value**

Answer the value the receiver is pointing to. The exact returned value
depends on the receiver's class

[]{#index-value_003a-10}

**value: aValue**

Set the receiver to point to the value, aValue. The exact meaning of
aValue depends on the receiver's class
