[]{#Sockets_002eReadBuffer_002dbuffer-handling}

::: header
Previous:
[Sockets.ReadBuffer-accessing-reading](Sockets_002eReadBuffer_002daccessing_002dreading.html#Sockets_002eReadBuffer_002daccessing_002dreading){accesskey="p"
rel="prev"}, Up:
[Sockets.ReadBuffer](Sockets_002eReadBuffer.html#Sockets_002eReadBuffer){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eReadBuffer_003a-buffer-handling}

#### 6.17.3 Sockets.ReadBuffer: buffer handling {#sockets.readbuffer-buffer-handling .subsection}

[]{#index-atEnd-3}

**atEnd**

Answer whether the data stream has ended.

[]{#index-availableBytes}

**availableBytes**

Answer how many bytes are available in the buffer.

[]{#index-bufferContents}

**bufferContents**

Answer the data that is in the buffer, and empty it.

[]{#index-close-2}

**close**

Not commented.

[]{#index-fill}

**fill**

Fill the buffer with more data if it is empty, and answer true if the
fill block was able to read more data.

[]{#index-fillBlock_003a}

**fillBlock: block**

Set the block that fills the buffer. It receives a collection and the
number of bytes to fill in it, and must return the number of bytes
actually read

[]{#index-isEmpty}

**isEmpty**

Answer whether the next input operation will force a buffer fill

[]{#index-isFull}

**isFull**

Answer whether the buffer has been just filled

[]{#index-notEmpty}

**notEmpty**

Check whether the next input operation will force a buffer fill and
answer true if it will not.

[]{#index-pastEnd}

**pastEnd**

Try to fill the buffer if the data stream has ended.
