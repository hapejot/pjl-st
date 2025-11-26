[]{#Memory-class_002daccessing}

::: header
Up: [Memory](Memory.html#Memory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Memory-class_003a-accessing}

#### 1.109.1 Memory class: accessing {#memory-class-accessing .subsection}

[]{#index-at_003a-14}

**at: anAddress**

Access the Smalltalk object (OOP) at the given address.

[]{#index-at_003aput_003a-13}

**at: anAddress put: aValue**

Store a pointer (OOP) to the Smalltalk object identified by 'value' at
the given address.

[]{#index-bigEndian}

**bigEndian**

Answer whether we're running on a big- or little-endian system.

[]{#index-charAt_003a-1}

**charAt: anAddress**

Access the C char at the given address. The value is returned as a
Smalltalk Character.

[]{#index-charAt_003aput_003a-1}

**charAt: anAddress put: aValue**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given address, using sizeof(char) bytes - i.e. 1
byte.

[]{#index-deref_003a}

**deref: anAddress**

Access the C int pointed by the given address

[]{#index-doubleAt_003a-1}

**doubleAt: anAddress**

Access the C double at the given address.

[]{#index-doubleAt_003aput_003a-1}

**doubleAt: anAddress put: aValue**

Store the Smalltalk Float object identified by 'value', at the given
address, writing it like a C double.

[]{#index-floatAt_003a-1}

**floatAt: anAddress**

Access the C float at the given address.

[]{#index-floatAt_003aput_003a-1}

**floatAt: anAddress put: aValue**

Store the Smalltalk Float object identified by 'value', at the given
address, writing it like a C float.

[]{#index-intAt_003a-1}

**intAt: anAddress**

Access the C int at the given address.

[]{#index-intAt_003aput_003a-1}

**intAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(int) bytes.

[]{#index-longAt_003a-1}

**longAt: anAddress**

Access the C long int at the given address.

[]{#index-longAt_003aput_003a-1}

**longAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(long) bytes.

[]{#index-longDoubleAt_003a-1}

**longDoubleAt: anAddress**

Access the C long double at the given address.

[]{#index-longDoubleAt_003aput_003a-1}

**longDoubleAt: anAddress put: aValue**

Store the Smalltalk Float object identified by 'value', at the given
address, writing it like a C long double.

[]{#index-shortAt_003a-1}

**shortAt: anAddress**

Access the C short int at the given address.

[]{#index-shortAt_003aput_003a-1}

**shortAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(short) bytes.

[]{#index-stringAt_003a-1}

**stringAt: anAddress**

Access the string pointed by the C 'char \*' at the given given address.

[]{#index-stringAt_003aput_003a-1}

**stringAt: anAddress put: aValue**

Store the Smalltalk String object identified by 'value', at the given
address in memory, writing it like a \*FRESHLY ALLOCATED\* C string. It
is the caller's responsibility to free it if necessary.

[]{#index-ucharAt_003aput_003a-1}

**ucharAt: anAddress put: aValue**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given address, using sizeof(char) bytes - i.e. 1
byte.

[]{#index-uintAt_003aput_003a-1}

**uintAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(int) bytes.

[]{#index-ulongAt_003aput_003a-1}

**ulongAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(long) bytes.

[]{#index-unsignedCharAt_003a-1}

**unsignedCharAt: anAddress**

Access the C unsigned char at the given address. The value is returned
as a Smalltalk Character.

[]{#index-unsignedCharAt_003aput_003a-1}

**unsignedCharAt: anAddress put: aValue**

Store as a C char the Smalltalk Character or Integer object identified
by 'value', at the given address, using sizeof(char) bytes - i.e. 1
byte.

[]{#index-unsignedIntAt_003a-1}

**unsignedIntAt: anAddress**

Access the C unsigned int at the given address.

[]{#index-unsignedIntAt_003aput_003a-1}

**unsignedIntAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(int) bytes.

[]{#index-unsignedLongAt_003a-1}

**unsignedLongAt: anAddress**

Access the C unsigned long int at the given address.

[]{#index-unsignedLongAt_003aput_003a-1}

**unsignedLongAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(long) bytes.

[]{#index-unsignedShortAt_003a-1}

**unsignedShortAt: anAddress**

Access the C unsigned short int at the given address.

[]{#index-unsignedShortAt_003aput_003a-1}

**unsignedShortAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(short) bytes.

[]{#index-ushortAt_003aput_003a-1}

**ushortAt: anAddress put: aValue**

Store the Smalltalk Integer object identified by 'value', at the given
address, using sizeof(short) bytes.

------------------------------------------------------------------------

::: header
Up: [Memory](Memory.html#Memory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
