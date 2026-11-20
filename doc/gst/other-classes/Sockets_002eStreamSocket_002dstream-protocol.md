[]{#Sockets_002eStreamSocket_002dstream-protocol}

::: header
Previous:
[Sockets.StreamSocket-printing](Sockets_002eStreamSocket_002dprinting.html#Sockets_002eStreamSocket_002dprinting){accesskey="p"
rel="prev"}, Up:
[Sockets.StreamSocket](Sockets_002eStreamSocket.html#Sockets_002eStreamSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eStreamSocket_003a-stream-protocol}

#### 6.22.8 Sockets.StreamSocket: stream protocol {#sockets.streamsocket-stream-protocol .subsection}

[]{#index-atEnd-4}

**atEnd**

Answer whether more data is available on the socket

[]{#index-availableBytes-1}

**availableBytes**

Answer how many bytes are available in the socket's read buffer or from
the operating system.

[]{#index-bufferContents-1}

**bufferContents**

Answer the current contents of the read buffer

[]{#index-canRead-2}

**canRead**

Answer whether more data is available in the socket's read buffer or
from the operating system.

[]{#index-close-3}

**close**

Flush and close the socket.

[]{#index-fill-1}

**fill**

Fill the read buffer with data read from the socket

[]{#index-isPeerAlive-1}

**isPeerAlive**

Answer whether the connection with the peer remote machine is still
valid.

[]{#index-next-6}

**next**

Read a byte from the socket. This might yield control to other Smalltalk
Processes.

[]{#index-peek-2}

**peek**

Read a byte from the socket, without advancing the buffer; answer nil if
no more data is available. This might yield control to other Smalltalk
Processes.

[]{#index-peekFor_003a}

**peekFor: anObject**

Read a byte from the socket, advancing the buffer only if it matches
anObject; answer whether they did match or not. This might yield control
to other Smalltalk Processes.

[]{#index-readBufferSize_003a-1}

**readBufferSize: size**

Create a new read buffer of the given size (which is only possible
before the first read or if the current buffer is empty).
