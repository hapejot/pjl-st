[]{#Sockets_002eSocket_002dstream-protocol}

::: header
Previous: [Sockets.Socket class-well known
ports](Sockets_002eSocket-class_002dwell-known-ports.html#Sockets_002eSocket-class_002dwell-known-ports){accesskey="p"
rel="prev"}, Up:
[Sockets.Socket](Sockets_002eSocket.html#Sockets_002eSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocket_003a-stream-protocol}

#### 6.19.4 Sockets.Socket: stream protocol {#sockets.socket-stream-protocol .subsection}

[]{#index-canWrite-1}

**canWrite**

Answer whether more data is available in the socket's read buffer or
from the operating system.

[]{#index-ensureWriteable-1}

**ensureWriteable**

Answer whether more data is available in the socket's read buffer or
from the operating system.

[]{#index-flush-5}

**flush**

Flush the write buffer to the operating system

[]{#index-next_003aputAll_003astartingAt_003a-1}

**next: n putAll: aCollection startingAt: pos**

Write aString to the socket; this acts as a bit-bucket when the socket
is closed. This might yield control to other Smalltalk Processes.

[]{#index-nextPut_003a-5}

**nextPut: char**

Write a character to the socket; this acts as a bit-bucket when the
socket is closed. This might yield control to other Smalltalk Processes.

[]{#index-writeBufferSize_003a-1}

**writeBufferSize: size**

Create a new write buffer of the given size, flushing the old one is
needed. This might yield control to other Smalltalk Processes.
