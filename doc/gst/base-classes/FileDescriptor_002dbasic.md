[]{#FileDescriptor_002dbasic}

::: header
Next: [FileDescriptor-binary
I/O](FileDescriptor_002dbinary-I_002fO.html#FileDescriptor_002dbinary-I_002fO){accesskey="n"
rel="next"}, Previous:
[FileDescriptor-accessing](FileDescriptor_002daccessing.html#FileDescriptor_002daccessing){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor_003a-basic}

#### 1.76.5 FileDescriptor: basic {#filedescriptor-basic .subsection}

[]{#index-checkError-1} []{#index-checkError-2}

**checkError**

Perform error checking. By default, we call File class\>\>#checkError.

[]{#index-close}

**close**

Close the file

[]{#index-contents}

**contents**

Answer the whole contents of the file

[]{#index-copyFrom_003ato_003a-1}

**copyFrom: from to: to**

Answer the contents of the file between the two given positions

[]{#index-finalize-1}

**finalize**

Close the file if it is still open by the time the object becomes
garbage.

[]{#index-invalidate}

**invalidate**

Invalidate a file descriptor

[]{#index-next}

**next**

Return the next character in the file, or nil at eof

[]{#index-nextByte}

**nextByte**

Return the next byte in the file, or nil at eof

[]{#index-nextPut_003a}

**nextPut: aCharacter**

Store aCharacter on the file

[]{#index-nextPutByte_003a}

**nextPutByte: anInteger**

Store the byte, anInteger, on the file

[]{#index-nextPutByteArray_003a}

**nextPutByteArray: aByteArray**

Store aByteArray on the file

[]{#index-peek}

**peek**

Returns the next element of the stream without moving the pointer.
Returns nil when at end of stream.

[]{#index-peekFor_003a}

**peekFor: anObject**

Returns whether the next element of the stream is equal to anObject,
without moving the pointer if it is not.

[]{#index-position}

**position**

Answer the zero-based position from the start of the file

[]{#index-position_003a}

**position: n**

Set the file pointer to the zero-based position n

[]{#index-reset}

**reset**

Reset the stream to its beginning

[]{#index-shutdown}

**shutdown**

Close the transmission side of a full-duplex connection. This is useful
on read-write pipes.

[]{#index-size-5}

**size**

Return the current size of the file, in bytes

[]{#index-truncate}

**truncate**

Truncate the file at the current position

------------------------------------------------------------------------

::: header
Next: [FileDescriptor-binary
I/O](FileDescriptor_002dbinary-I_002fO.html#FileDescriptor_002dbinary-I_002fO){accesskey="n"
rel="next"}, Previous:
[FileDescriptor-accessing](FileDescriptor_002daccessing.html#FileDescriptor_002daccessing){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
