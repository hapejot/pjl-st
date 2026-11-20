[]{#Sockets_002eStreamSocket-class_002daccessing}

::: header
Next: [Sockets.StreamSocket
class-initialize](Sockets_002eStreamSocket-class_002dinitialize.html#Sockets_002eStreamSocket-class_002dinitialize){accesskey="n"
rel="next"}, Up:
[Sockets.StreamSocket](Sockets_002eStreamSocket.html#Sockets_002eStreamSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eStreamSocket-class_003a-accessing}

#### 6.22.1 Sockets.StreamSocket class: accessing {#sockets.streamsocket-class-accessing .subsection}

[]{#index-defaultImplementationClassFor_003a-3}

**defaultImplementationClassFor: aSocketAddressClass**

Answer the default implementation class. Depending on the subclass, this
might be the default stream socket implementation class of the given
address class, or rather its default datagram socket implementation
class.

[]{#index-readBufferSize}

**readBufferSize**

Answer the size of the read buffer for newly-created sockets

[]{#index-readBufferSize_003a}

**readBufferSize: anInteger**

Set the size of the read buffer for newly-created sockets
