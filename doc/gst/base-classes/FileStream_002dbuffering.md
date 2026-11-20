[]{#FileStream_002dbuffering}

::: header
Next:
[FileStream-compiling](FileStream_002dcompiling.html#FileStream_002dcompiling){accesskey="n"
rel="next"}, Previous:
[FileStream-basic](FileStream_002dbasic.html#FileStream_002dbasic){accesskey="p"
rel="prev"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileStream_003a-buffering}

#### 1.79.4 FileStream: buffering {#filestream-buffering .subsection}

[]{#index-bufferSize}

**bufferSize**

Answer the file's current buffer

[]{#index-bufferSize_003a}

**bufferSize: bufSize**

Flush the file and set the buffer's size to bufSize

[]{#index-clean}

**clean**

Synchronize the file descriptor's state with the object's state.

[]{#index-fill}

**fill**

Private - Fill the input buffer

[]{#index-flush}

**flush**

Flush the output buffer.

[]{#index-newBuffer}

**newBuffer**

Private - Answer a String to be used as the receiver's buffer

[]{#index-next_003abufferAll_003astartingAt_003a}

**next: n bufferAll: aCollection startingAt: pos**

Private - Assuming that the buffer has space for n characters, store n
characters of aCollection in the buffer, starting from the pos-th.

[]{#index-nextAvailable_003ainto_003astartingAt_003a-1}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Read up to anInteger bytes from the stream and store them into
aCollection. Return the number of bytes read.

[]{#index-nextAvailable_003aputAllOn_003a}

**nextAvailable: anInteger putAllOn: aStream**

Copy up to anInteger bytes from the stream into aStream. Return the
number of bytes read.

[]{#index-pendingWrite}

**pendingWrite**

Answer whether the output buffer is full.
