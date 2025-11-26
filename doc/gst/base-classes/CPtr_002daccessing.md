[]{#CPtr_002daccessing}

::: header
Up: [CPtr](CPtr.html#CPtr){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CPtr_003a-accessing}

#### 1.43.1 CPtr: accessing {#cptr-accessing .subsection}

[]{#index-alignof-18}

**alignof**

Answer the receiver's required aligment

[]{#index-sizeof-18}

**sizeof**

Answer the receiver's size

[]{#index-value-6}

**value**

Answer the address of the location pointed to by the receiver.

[]{#index-value_003a-6}

**value: anObject**

Set the address of the location pointed to by the receiver to anObject,
which can be either an Integer or a CObject. if anObject is an Integer,
it is interpreted as a 32-bit or 64-bit address. If it is a CObject, its
address is stored.
