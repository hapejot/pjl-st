[]{#CObject_002dpointer_002dlike-behavior}

::: header
Next:
[CObject-testing](CObject_002dtesting.html#CObject_002dtesting){accesskey="n"
rel="next"}, Previous:
[CObject-finalization](CObject_002dfinalization.html#CObject_002dfinalization){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CObject_003a-pointer_002dlike-behavior}

#### 1.36.10 CObject: pointer-like behavior {#cobject-pointer-like-behavior .subsection}

[]{#index-_002b}

**+ anInteger**

Return another instance of the receiver's class which points at
&receiver\[anInteger\] (or, if you prefer, what 'receiver + anInteger'
does in C).

[]{#index-_002d}

**- intOrPtr**

If intOrPtr is an integer, return another instance of the receiver's
class pointing at &receiver\[-anInteger\] (or, if you prefer, what
'receiver - anInteger' does in C). If it is the same class as the
receiver, return the difference in chars, i.e. in bytes, between the two
pointed addresses (or, if you prefer, what 'receiver - anotherCharPtr'
does in C)

[]{#index-addressAt_003a}

**addressAt: anIndex**

Return a new CObject of the element type, corresponding to an object
that is anIndex places past the receiver (remember that CObjects
represent pointers and that C pointers behave like arrays). anIndex is
zero-based, just like with all other C-style accessing.

[]{#index-at_003a}

**at: anIndex**

Dereference a pointer that is anIndex places past the receiver (remember
that CObjects represent pointers and that C pointers behave like
arrays). anIndex is zero-based, just like with all other C-style
accessing.

[]{#index-at_003aput_003a-1}

**at: anIndex put: aValue**

Store anIndex places past the receiver the passed Smalltalk object or
CObject 'aValue'; if it is a CObject is dereferenced: that is, this
method is equivalent either to cobj\[anIndex\]=aValue or
cobj\[anIndex\]=\*aValue. anIndex is zero-based, just like with all
other C-style accessing.

In both cases, aValue should be of the element type or of the
corresponding Smalltalk type (that is, a String is ok for an array of
CStrings) to avoid typing problems which however will not be signaled
because C is untyped.

[]{#index-decr}

**decr**

Adjust the pointer by sizeof(dereferencedType) bytes down (i.e.
--receiver)

[]{#index-decrBy_003a}

**decrBy: anInteger**

Adjust the pointer by anInteger elements down (i.e. receiver -=
anInteger)

[]{#index-incr}

**incr**

Adjust the pointer by sizeof(dereferencedType) bytes up (i.e.
++receiver)

[]{#index-incrBy_003a}

**incrBy: anInteger**

Adjust the pointer by anInteger elements up (i.e. receiver += anInteger)

------------------------------------------------------------------------

::: header
Next:
[CObject-testing](CObject_002dtesting.html#CObject_002dtesting){accesskey="n"
rel="next"}, Previous:
[CObject-finalization](CObject_002dfinalization.html#CObject_002dfinalization){accesskey="p"
rel="prev"}, Up: [CObject](CObject.html#CObject){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
