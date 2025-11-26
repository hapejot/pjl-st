[]{#Sockets_002eReadBuffer-class_002dinstance-creation}

::: header
Next:
[Sockets.ReadBuffer-accessing-reading](Sockets_002eReadBuffer_002daccessing_002dreading.html#Sockets_002eReadBuffer_002daccessing_002dreading){accesskey="n"
rel="next"}, Up:
[Sockets.ReadBuffer](Sockets_002eReadBuffer.html#Sockets_002eReadBuffer){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eReadBuffer-class_003a-instance-creation}

#### 6.17.1 Sockets.ReadBuffer class: instance creation {#sockets.readbuffer-class-instance-creation .subsection}

[]{#index-on_003a-3}

**on: aCollection**

Answer a Stream that uses aCollection as a buffer. You should ensure
that the fillBlock is set before the first operation, because the buffer
will report that the data has ended until you set the fillBlock.
