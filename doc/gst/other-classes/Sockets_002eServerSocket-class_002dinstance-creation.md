[]{#Sockets_002eServerSocket-class_002dinstance-creation}

::: header
Next:
[Sockets.ServerSocket-accessing](Sockets_002eServerSocket_002daccessing.html#Sockets_002eServerSocket_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.ServerSocket
class-accessing](Sockets_002eServerSocket-class_002daccessing.html#Sockets_002eServerSocket-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[Sockets.ServerSocket](Sockets_002eServerSocket.html#Sockets_002eServerSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eServerSocket-class_003a-instance-creation}

#### 6.18.2 Sockets.ServerSocket class: instance creation {#sockets.serversocket-class-instance-creation .subsection}

[]{#index-defaultQueueSize} []{#index-accept-1}

**defaultQueueSize**

Answer the default length of the queue for pending connections. When the
queue fills, new clients attempting to connect fail until the server has
sent #accept to accept a connection from the queue.

[]{#index-port_003a-2}

**port: anInteger**

Answer a new ServerSocket serving on any local address, on the given
port, with a pending connections queue of the default length.

[]{#index-port_003abindTo_003a}

**port: anInteger bindTo: ipAddress**

Answer a new ServerSocket serving on the given address and port, with a
pending connections queue of the default length.

[]{#index-port_003aqueueSize_003a}

**port: anInteger queueSize: backlog**

Answer a new ServerSocket serving on any local address, on the given
port, with a pending connections queue of the given length.

[]{#index-port_003aqueueSize_003abindTo_003a}

**port: anInteger queueSize: backlog bindTo: ipAddress**

Answer a new ServerSocket serving on the given address and port, and
with a pending connections queue of the given length.

[]{#index-queueSize_003a}

**queueSize: backlog**

Answer a new ServerSocket serving on any local address and port, with a
pending connections queue of the given length.

[]{#index-queueSize_003abindTo_003a}

**queueSize: backlog bindTo: ipAddress**

Answer a new ServerSocket serving on the given local address, and on any
port, with a pending connections queue of the given length.
