[]{#Sockets_002eServerSocket_002daccessing}

::: header
Next:
[Sockets.ServerSocket-initializing](Sockets_002eServerSocket_002dinitializing.html#Sockets_002eServerSocket_002dinitializing){accesskey="n"
rel="next"}, Previous: [Sockets.ServerSocket class-instance
creation](Sockets_002eServerSocket-class_002dinstance-creation.html#Sockets_002eServerSocket-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.ServerSocket](Sockets_002eServerSocket.html#Sockets_002eServerSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eServerSocket_003a-accessing}

#### 6.18.3 Sockets.ServerSocket: accessing {#sockets.serversocket-accessing .subsection}

[]{#index-accept}

**accept**

Accept a new connection and create a new instance of Socket if there is
one, else answer nil.

[]{#index-accept_003a-1}

**accept: socketClass**

Accept a new connection and create a new instance of socketClass if
there is one, else answer nil. This is usually needed only to create
DatagramSockets.

[]{#index-address-3}

**address**

Answer the local address

[]{#index-port-3}

**port**

Answer the local port (the port that the passive socket is listening
on).

[]{#index-primAccept_003a}

**primAccept: socketClass**

Accept a new connection and create a new instance of Socket if there is
one, else fail.

[]{#index-waitForConnection}

**waitForConnection**

Wait for a connection to be available, and suspend the currently
executing process in the meanwhile.
