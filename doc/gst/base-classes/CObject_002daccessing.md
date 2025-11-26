[]{#CObject_002daccessing}

::: header
Next:
[CObject-basic](CObject_002dbasic.html#CObject_002dbasic){accesskey="n"
rel="next"}, Previous: [CObject class-subclass
creation](CObject-class_002dsubclass-creation.html#CObject-class_002dsubclass-creation){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject_003a-accessing}

#### 1.36.5 CObject: accessing {#cobject-accessing .subsection}

[]{#index-address} []{#index-storage-1}

**address**

Answer the address the receiver is pointing to. The address can be
absolute if the storage is nil, or relative to the Smalltalk object in
#storage. In this case, an address of 0 corresponds to the first
instance variable.

[]{#index-address_003a-1}

**address: anInteger**

Set the receiver to point to the passed address, anInteger

[]{#index-isAbsolute}

**isAbsolute**

Answer whether the object points into a garbage-collected Smalltalk
storage, or it is an absolute address.

[]{#index-printOn_003a-8}

**printOn: aStream**

Print a representation of the receiver

[]{#index-storage}

**storage**

Answer the storage that the receiver is pointing into, or nil if the
address is absolute.

[]{#index-storage_003a}

**storage: anObject**

Change the receiver to point to the storage of anObject.

[]{#index-type_003a}

**type: aCType**

Set the receiver's type to aCType.
