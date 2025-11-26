[]{#FileStream_002dbasic}

::: header
Next:
[FileStream-buffering](FileStream_002dbuffering.html#FileStream_002dbuffering){accesskey="n"
rel="next"}, Previous: [FileStream class-standard
streams](FileStream-class_002dstandard-streams.html#FileStream-class_002dstandard-streams){accesskey="p"
rel="prev"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileStream_003a-basic}

#### 1.79.3 FileStream: basic {#filestream-basic .subsection}

[]{#index-bufferStart}

**bufferStart**

Private - Answer the offset from the start of the file corresponding to
the beginning of the read buffer.

[]{#index-copyFrom_003ato_003a-3}

**copyFrom: from to: to**

Answer the contents of the file between the two given positions

[]{#index-next-1}

**next**

Return the next character in the file, or nil at eof

[]{#index-nextPut_003a-1}

**nextPut: aCharacter**

Store aCharacter on the file

[]{#index-peek-1}

**peek**

Return the next character in the file, or nil at eof. Don't advance the
file pointer.

[]{#index-position-1}

**position**

Answer the zero-based position from the start of the file

[]{#index-position_003a-1}

**position: n**

Set the file pointer to the zero-based position n

[]{#index-size-8}

**size**

Return the current size of the file, in bytes

[]{#index-truncate-1}

**truncate**

Truncate the file at the current position
