[]{#ByteArray_002dmore-advanced-accessing}

::: header
Next:
[ByteArray-storing](ByteArray_002dstoring.html#ByteArray_002dstoring){accesskey="n"
rel="next"}, Previous:
[ByteArray-converting](ByteArray_002dconverting.html#ByteArray_002dconverting){accesskey="p"
rel="prev"}, Up: [ByteArray](ByteArray.html#ByteArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ByteArray_003a-more-advanced-accessing}

#### 1.14.6 ByteArray: more advanced accessing {#bytearray-more-advanced-accessing .subsection}

[]{#index-charAt_003a}

**charAt: index**

Access the C char at the given index in the receiver. The value is
returned as a Smalltalk Character. Indices are 1-based just like for
other Smalltalk access.

[]{#index-charAt_003aput_003a}

**charAt: index put: value**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given index in the receiver, using sizeof(char)
bytes - i.e. 1 byte. Indices are 1-based just like for other Smalltalk
access.

[]{#index-doubleAt_003a}

**doubleAt: index**

Access the C double at the given index in the receiver. Indices are
1-based just like for other Smalltalk access.

[]{#index-doubleAt_003aput_003a}

**doubleAt: index put: value**

Store the Smalltalk Float object identified by 'value', at the given
index in the receiver, writing it like a C double. Indices are 1-based
just like for other Smalltalk access.

[]{#index-floatAt_003a}

**floatAt: index**

Access the C float at the given index in the receiver. Indices are
1-based just like for other Smalltalk access.

[]{#index-floatAt_003aput_003a}

**floatAt: index put: value**

Store the Smalltalk Float object identified by 'value', at the given
index in the receiver, writing it like a C float. Indices are 1-based
just like for other Smalltalk access.

[]{#index-intAt_003a}

**intAt: index**

Access the C int at the given index in the receiver. Indices are 1-based
just like for other Smalltalk access.

[]{#index-intAt_003aput_003a}

**intAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(int) bytes. Indices are 1-based just
like for other Smalltalk access.

[]{#index-longAt_003a}

**longAt: index**

Access the C long int at the given index in the receiver. Indices are
1-based just like for other Smalltalk access.

[]{#index-longAt_003aput_003a}

**longAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(long) bytes. Indices are 1-based
just like for other Smalltalk access.

[]{#index-longDoubleAt_003a}

**longDoubleAt: index**

Access the C long double at the given index in the receiver. Indices are
1-based just like for other Smalltalk access.

[]{#index-longDoubleAt_003aput_003a}

**longDoubleAt: index put: value**

Store the Smalltalk Float object identified by 'value', at the given
index in the receiver, writing it like a C double. Indices are 1-based
just like for other Smalltalk access.

[]{#index-objectAt_003a}

**objectAt: index**

Access the Smalltalk object (OOP) at the given index in the receiver.
Indices are 1-based just like for other Smalltalk access.

[]{#index-objectAt_003aput_003a}

**objectAt: index put: value**

Store a pointer (OOP) to the Smalltalk object identified by 'value', at
the given index in the receiver. Indices are 1-based just like for other
Smalltalk access.

[]{#index-shortAt_003a}

**shortAt: index**

Access the C short int at the given index in the receiver. Indices are
1-based just like for other Smalltalk access.

[]{#index-shortAt_003aput_003a}

**shortAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(short) bytes. Indices are 1-based
just like for other Smalltalk access.

[]{#index-stringAt_003a}

**stringAt: index**

Access the string pointed by the C 'char \*' at the given index in the
receiver. Indices are 1-based just like for other Smalltalk access.

[]{#index-stringAt_003aput_003a}

**stringAt: index put: value**

Store the Smalltalk String object identified by 'value', at the given
index in the receiver, writing it like a \*FRESHLY ALLOCATED\* C string.
It is the caller's responsibility to free it if necessary. Indices are
1-based just like for other Smalltalk access.

[]{#index-ucharAt_003a}

**ucharAt: index**

Access the C unsigned char at the given index in the receiver. The value
is returned as a Smalltalk Character. Indices are 1-based just like for
other Smalltalk access.

[]{#index-ucharAt_003aput_003a}

**ucharAt: index put: value**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given index in the receiver, using sizeof(char)
bytes - i.e. 1 byte. Indices are 1-based just like for other Smalltalk
access.

[]{#index-uintAt_003a}

**uintAt: index**

Access the C unsigned int at the given index in the receiver. Indices
are 1-based just like for other Smalltalk access.

[]{#index-uintAt_003aput_003a}

**uintAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(int) bytes. Indices are 1-based just
like for other Smalltalk access.

[]{#index-ulongAt_003a}

**ulongAt: index**

Access the C unsigned long int at the given index in the receiver.
Indices are 1-based just like for other Smalltalk access.

[]{#index-ulongAt_003aput_003a}

**ulongAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(long) bytes. Indices are 1-based
just like for other Smalltalk access.

[]{#index-unsignedCharAt_003a}

**unsignedCharAt: index**

Access the C unsigned char at the given index in the receiver. The value
is returned as a Smalltalk Character. Indices are 1-based just like for
other Smalltalk access.

[]{#index-unsignedCharAt_003aput_003a}

**unsignedCharAt: index put: value**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given index in the receiver, using sizeof(char)
bytes - i.e. 1 byte. Indices are 1-based just like for other Smalltalk
access.

[]{#index-unsignedIntAt_003a}

**unsignedIntAt: index**

Access the C unsigned int at the given index in the receiver. Indices
are 1-based just like for other Smalltalk access.

[]{#index-unsignedIntAt_003aput_003a}

**unsignedIntAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(int) bytes. Indices are 1-based just
like for other Smalltalk access.

[]{#index-unsignedLongAt_003a}

**unsignedLongAt: index**

Access the C unsigned long int at the given index in the receiver.
Indices are 1-based just like for other Smalltalk access.

[]{#index-unsignedLongAt_003aput_003a}

**unsignedLongAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(long) bytes. Indices are 1-based
just like for other Smalltalk access.

[]{#index-unsignedShortAt_003a}

**unsignedShortAt: index**

Access the C unsigned short int at the given index in the receiver.
Indices are 1-based just like for other Smalltalk access.

[]{#index-unsignedShortAt_003aput_003a}

**unsignedShortAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(short) bytes. Indices are 1-based
just like for other Smalltalk access.

[]{#index-ushortAt_003a}

**ushortAt: index**

Access the C unsigned short int at the given index in the receiver.
Indices are 1-based just like for other Smalltalk access.

[]{#index-ushortAt_003aput_003a}

**ushortAt: index put: value**

Store the Smalltalk Integer object identified by 'value', at the given
index in the receiver, using sizeof(short) bytes. Indices are 1-based
just like for other Smalltalk access.

------------------------------------------------------------------------

::: header
Next:
[ByteArray-storing](ByteArray_002dstoring.html#ByteArray_002dstoring){accesskey="n"
rel="next"}, Previous:
[ByteArray-converting](ByteArray_002dconverting.html#ByteArray_002dconverting){accesskey="p"
rel="prev"}, Up: [ByteArray](ByteArray.html#ByteArray){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
