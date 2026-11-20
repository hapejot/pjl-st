[]{#Sockets_002eAbstractSocket_002daccessing}

::: header
Next:
[Sockets.AbstractSocket-printing](Sockets_002eAbstractSocket_002dprinting.html#Sockets_002eAbstractSocket_002dprinting){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocket class-well known
ports](Sockets_002eAbstractSocket-class_002dwell-known-ports.html#Sockets_002eAbstractSocket-class_002dwell-known-ports){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket_003a-accessing}

#### 6.1.5 Sockets.AbstractSocket: accessing {#sockets.abstractsocket-accessing .subsection}

[]{#index-address}

**address**

Answer an IP address that is of common interest (this can be either the
local or the remote address, according to the definition in the
subclass).

[]{#index-available} []{#index-canRead-3}

**available**

Answer whether there is data available on the socket. Same as #canRead,
present for backwards compatibility.

[]{#index-canRead}

**canRead**

Answer whether there is data available on the socket.

[]{#index-canWrite}

**canWrite**

Answer whether there is free space in the socket's write buffer.

[]{#index-close-1}

**close**

Close the socket represented by the receiver.

[]{#index-flush-4}

**flush**

Flush any buffers used by the receiver.

[]{#index-isOpen}

**isOpen**

Answer whether the connection between the receiver and the remote
endpoint is still alive.

[]{#index-isPeerAlive}

**isPeerAlive**

Answer whether the connection with the peer remote machine is still
valid.

[]{#index-localAddress}

**localAddress**

Answer the local IP address of the socket.

[]{#index-localPort}

**localPort**

Answer the local IP port of the socket.

[]{#index-port}

**port**

Answer an IP port that is of common interest (this can be the port for
either the local or remote endpoint, according to the definitions in the
subclass

[]{#index-remoteAddress}

**remoteAddress**

Answer the IP address of the socket's remote endpoint.

[]{#index-remotePort}

**remotePort**

Answer the IP port of the socket's remote endpoint.
