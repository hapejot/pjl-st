[]{#CObject_002dC-data-access}

::: header
Next:
[CObject-conversion](CObject_002dconversion.html#CObject_002dconversion){accesskey="n"
rel="next"}, Previous:
[CObject-basic](CObject_002dbasic.html#CObject_002dbasic){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject_003a-C-data-access}

#### 1.36.7 CObject: C data access {#cobject-c-data-access .subsection}

[]{#index-at_003aput_003atype_003a}

**at: byteOffset put: aValue type: aType**

Store aValue as data of the given type from byteOffset bytes after the
pointer stored in the receiver

[]{#index-at_003atype_003a}

**at: byteOffset type: aType**

Answer some data of the given type from byteOffset bytes after the
pointer stored in the receiver

[]{#index-free}

**free**

Free the receiver's pointer and set it to null. Big trouble hits you if
the receiver doesn't point to the base of a malloc-ed area.
