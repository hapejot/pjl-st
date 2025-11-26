[]{#Sockets_002eWriteBuffer_002dbuffer-handling}

::: header
Next:
[Sockets.WriteBuffer-testing](Sockets_002eWriteBuffer_002dtesting.html#Sockets_002eWriteBuffer_002dtesting){accesskey="n"
rel="next"}, Previous:
[Sockets.WriteBuffer-accessing-writing](Sockets_002eWriteBuffer_002daccessing_002dwriting.html#Sockets_002eWriteBuffer_002daccessing_002dwriting){accesskey="p"
rel="prev"}, Up:
[Sockets.WriteBuffer](Sockets_002eWriteBuffer.html#Sockets_002eWriteBuffer){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eWriteBuffer_003a-buffer-handling}

#### 6.28.2 Sockets.WriteBuffer: buffer handling {#sockets.writebuffer-buffer-handling .subsection}

[]{#index-close-6}

**close**

Not commented.

[]{#index-flush-7}

**flush**

Evaluate the flushing block and reset the stream

[]{#index-flushBlock_003a}

**flushBlock: block**

Set which block will be used to flush the buffer. The block will be
evaluated with a collection and an Integer n as parameters, and will
have to write the first n elements of the collection.
