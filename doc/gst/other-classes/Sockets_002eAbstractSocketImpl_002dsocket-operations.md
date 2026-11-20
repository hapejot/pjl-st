[]{#Sockets_002eAbstractSocketImpl_002dsocket-operations}

::: header
Next: [Sockets.AbstractSocketImpl-socket
options](Sockets_002eAbstractSocketImpl_002dsocket-options.html#Sockets_002eAbstractSocketImpl_002dsocket-options){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocketImpl-C
constants](Sockets_002eAbstractSocketImpl_002dC-constants.html#Sockets_002eAbstractSocketImpl_002dC-constants){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocketImpl_003a-socket-operations}

#### 6.2.9 Sockets.AbstractSocketImpl: socket operations {#sockets.abstractsocketimpl-socket-operations .subsection}

[]{#index-accept_003a}

**accept: implementationClass**

Accept a connection on the receiver, and create a new instance of
implementationClass that will deal with the newly created active server
socket.

[]{#index-bindTo_003aport_003a}

**bindTo: ipAddress port: port**

Bind the receiver to the given IP address and port. 'Binding' means
attaching the local endpoint of the socket.

[]{#index-fileOp_003a}

**fileOp: ioFuncIndex**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003aifFail_003a}

**fileOp: ioFuncIndex ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003a}

**fileOp: ioFuncIndex with: arg1**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003awith_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003awith_003awith_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003awith_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-getSockName}

**getSockName**

Retrieve a ByteArray containing a sockaddr_in struct for the local
endpoint of the socket.

[]{#index-listen_003a}

**listen: backlog**

Make the receiver a passive server socket with a pending connections
queue of the given size.

------------------------------------------------------------------------

::: header
Next: [Sockets.AbstractSocketImpl-socket
options](Sockets_002eAbstractSocketImpl_002dsocket-options.html#Sockets_002eAbstractSocketImpl_002dsocket-options){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocketImpl-C
constants](Sockets_002eAbstractSocketImpl_002dC-constants.html#Sockets_002eAbstractSocketImpl_002dC-constants){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
