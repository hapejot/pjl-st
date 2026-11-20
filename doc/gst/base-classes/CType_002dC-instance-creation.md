[]{#CType_002dC-instance-creation}

::: header
Next:
[CType-storing](CType_002dstoring.html#CType_002dstoring){accesskey="n"
rel="next"}, Previous:
[CType-basic](CType_002dbasic.html#CType_002dbasic){accesskey="p"
rel="prev"}, Up: [CType](CType.html#CType){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CType_003a-C-instance-creation}

#### 1.52.5 CType: C instance creation {#ctype-c-instance-creation .subsection}

[]{#index-address_003a-2}

**address: cObjOrInt**

Create a new CObject with the type (class) identified by the receiver,
pointing to the given address (identified by an Integer or CObject).

[]{#index-gcNew-1}

**gcNew**

Allocate a new CObject with the type (class) identified by the receiver.
The object is movable in memory, but on the other hand it is
garbage-collected automatically.

[]{#index-gcNew_003a-1}

**gcNew: anInteger**

Allocate a new CObject with room for anInteger C object of the type
(class) identified by the receiver. The object is movable in memory, but
on the other hand it is garbage-collected automatically.

[]{#index-new-5}

**new**

Allocate a new CObject with the type (class) identified by the receiver.
It is the caller's responsibility to free the memory allocated for it.
