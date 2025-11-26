[]{#Sockets_002eDatagramSocket}

::: header
Next:
[Sockets.DatagramSocketImpl](Sockets_002eDatagramSocketImpl.html#Sockets_002eDatagramSocketImpl){accesskey="n"
rel="next"}, Previous:
[Sockets.Datagram](Sockets_002eDatagram.html#Sockets_002eDatagram){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagramSocket-1}

### 6.6 Sockets.DatagramSocket {#sockets.datagramsocket .section}

[]{#index-Sockets_002eDatagramSocket}

**Defined in namespace Sockets**\
**Superclass: Sockets.AbstractSocket**\
**Category: Sockets-Streams**

:   This class models a connectionless datagram socket that sends
    individual packets of data across the network. In the TCP/IP world,
    this means UDP. Datagram packets do not have guaranteed delivery, or
    any guarantee about the order the data will be received on the
    remote host.

    This class uses an underlying socket implementation object which is
    a subclass of DatagramSocketImpl. This is less necessary for
    datagram sockets than for stream sockets (except for hiding some
    methods in FileDescriptor that are not relevant to sockets), but it
    is done for cleanliness and symmetry.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.DatagramSocket class-accessing](Sockets_002eDatagramSocket-class_002daccessing.html#Sockets_002eDatagramSocket-class_002daccessing){accesskey="1"}:                                (class)
  • [Sockets.DatagramSocket class-initialization](Sockets_002eDatagramSocket-class_002dinitialization.html#Sockets_002eDatagramSocket-class_002dinitialization){accesskey="2"}:                 (class)
  • [Sockets.DatagramSocket class-instance creation](Sockets_002eDatagramSocket-class_002dinstance-creation.html#Sockets_002eDatagramSocket-class_002dinstance-creation){accesskey="3"}:        (class)
  • [Sockets.DatagramSocket-accessing](Sockets_002eDatagramSocket_002daccessing.html#Sockets_002eDatagramSocket_002daccessing){accesskey="4"}:                                                  (instance)
  • [Sockets.DatagramSocket-direct operations](Sockets_002eDatagramSocket_002ddirect-operations.html#Sockets_002eDatagramSocket_002ddirect-operations){accesskey="5"}:                          (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
