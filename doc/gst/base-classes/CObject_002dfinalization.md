[]{#CObject_002dfinalization}

::: header
Next: [CObject-pointer-like
behavior](CObject_002dpointer_002dlike-behavior.html#CObject_002dpointer_002dlike-behavior){accesskey="n"
rel="next"}, Previous:
[CObject-conversion](CObject_002dconversion.html#CObject_002dconversion){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject_003a-finalization}

#### 1.36.9 CObject: finalization {#cobject-finalization .subsection}

[]{#index-finalize} []{#index-addToBeFinalized-3} []{#index-free-2}

**finalize**

To make the VM call this, use #addToBeFinalized. It frees automatically
any memory pointed to by the CObject. It is not automatically enabled
because big trouble hits you if you use #free and the receiver doesn't
point to the base of a malloc-ed area.
