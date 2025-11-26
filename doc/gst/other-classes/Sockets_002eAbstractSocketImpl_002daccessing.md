[]{#Sockets_002eAbstractSocketImpl_002daccessing}

::: header
Next: [Sockets.AbstractSocketImpl-asynchronous
operations](Sockets_002eAbstractSocketImpl_002dasynchronous-operations.html#Sockets_002eAbstractSocketImpl_002dasynchronous-operations){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocketImpl class-socket
creation](Sockets_002eAbstractSocketImpl-class_002dsocket-creation.html#Sockets_002eAbstractSocketImpl-class_002dsocket-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocketImpl_003a-accessing}

#### 6.2.5 Sockets.AbstractSocketImpl: accessing {#sockets.abstractsocketimpl-accessing .subsection}

[]{#index-connectTo_003aport_003a}

**connectTo: ipAddress port: port**

Connect the receiver to the given IP address and port. 'Connecting'
means attaching the remote endpoint of the socket.

[]{#index-localAddress-1}

**localAddress**

Answer the address of the local endpoint of the socket (even if IP is
not being used, this identifies the machine that is bound to the
socket).

[]{#index-localPort-1}

**localPort**

Answer the port of the local endpoint of the socket (even if IP is not
being used, this identifies the service or process that is bound to the
socket).

[]{#index-remoteAddress-1}

**remoteAddress**

Answer the address of the remote endpoint of the socket (even if IP is
not being used, this identifies the machine to which the socket is
connected).

[]{#index-remotePort-1}

**remotePort**

Answer the port of the remote endpoint of the socket (even if IP is not
being used, this identifies the service or process to which the socket
is connected).
