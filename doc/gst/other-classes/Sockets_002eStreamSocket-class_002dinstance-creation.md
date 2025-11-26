[]{#Sockets_002eStreamSocket-class_002dinstance-creation}

::: header
Next:
[Sockets.StreamSocket-accessing](Sockets_002eStreamSocket_002daccessing.html#Sockets_002eStreamSocket_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.StreamSocket
class-initialize](Sockets_002eStreamSocket-class_002dinitialize.html#Sockets_002eStreamSocket-class_002dinitialize){accesskey="p"
rel="prev"}, Up:
[Sockets.StreamSocket](Sockets_002eStreamSocket.html#Sockets_002eStreamSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eStreamSocket-class_003a-instance-creation}

#### 6.22.3 Sockets.StreamSocket class: instance creation {#sockets.streamsocket-class-instance-creation .subsection}

[]{#index-remote_003aport_003a}

**remote: ipAddressOrString port: remotePort**

Create a new socket and connect to the given host (passed as a String to
be resolved or as a SocketAddress), and to the given port.

[]{#index-remote_003aport_003alocal_003aport_003a-1}

**remote: ipAddressOrString port: remotePort local: ipAddress port:
localPort**

Create a new socket and connect to the given host (passed as a String to
be resolved or as a SocketAddress), and to the given remotePort. Then
bind it to the local address passed in ipAddress, on the localPort port;
if the former is nil, any local address will do, and if the latter is 0,
any local port will do.
