[]{#Sockets_002eSocketImpl_002dsocket-operations}

::: header
Previous:
[Sockets.SocketImpl-abstract](Sockets_002eSocketImpl_002dabstract.html#Sockets_002eSocketImpl_002dabstract){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketImpl](Sockets_002eSocketImpl.html#Sockets_002eSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketImpl_003a-socket-operations}

#### 6.21.3 Sockets.SocketImpl: socket operations {#sockets.socketimpl-socket-operations .subsection}

[]{#index-connectTo_003aport_003a-1}

**connectTo: ipAddress port: port**

Try to connect the socket represented by the receiver to the given
remote machine.

[]{#index-getPeerName}

**getPeerName**

Retrieve a ByteArray containing a sockaddr_in struct for the remote
endpoint of the socket.
