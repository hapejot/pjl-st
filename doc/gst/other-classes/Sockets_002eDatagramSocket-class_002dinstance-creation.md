[]{#Sockets_002eDatagramSocket-class_002dinstance-creation}

::: header
Next:
[Sockets.DatagramSocket-accessing](Sockets_002eDatagramSocket_002daccessing.html#Sockets_002eDatagramSocket_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.DatagramSocket
class-initialization](Sockets_002eDatagramSocket-class_002dinitialization.html#Sockets_002eDatagramSocket-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.DatagramSocket](Sockets_002eDatagramSocket.html#Sockets_002eDatagramSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagramSocket-class_003a-instance-creation}

#### 6.6.3 Sockets.DatagramSocket class: instance creation {#sockets.datagramsocket-class-instance-creation .subsection}

[]{#index-local_003aport_003a}

**local: ipAddressOrString port: remotePort**

Create a new socket and bind it to the given host (passed as a String to
be resolved or as an IPAddress), on the given port.

[]{#index-new-13}

**new**

Answer a new datagram socket (by default an UDP socket), without a
specified local address and port.

[]{#index-port_003a-1}

**port: localPort**

Create a new socket and bind it to the local host on the given port.

[]{#index-remote_003aport_003alocal_003aport_003a}

**remote: ipAddressOrString port: remotePort local: ipAddress port:
localPort**

Create a new socket and bind it to the given host (passed as a String to
be resolved or as a SocketAddress), and to the given remotePort. The
default destination for the datagrams will be ipAddressOrString (if not
nil), on the remotePort port.
