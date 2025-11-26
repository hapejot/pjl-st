[]{#Sockets_002eAbstractSocket}

::: header
Next:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="n"
rel="next"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket-1}

### 6.1 Sockets.AbstractSocket {#sockets.abstractsocket .section}

[]{#index-Sockets_002eAbstractSocket}

**Defined in namespace Sockets**\
**Superclass: Stream**\
**Category: Sockets-Streams**

:   This class models a client site socket. A socket is a TCP/IP
    endpoint for network communications conceptually similar to a file
    handle.

    This class only takes care of buffering and blocking if requested.
    It uses an underlying socket implementation object which is a
    subclass of AbstractSocketImpl. This is necessary to hide some
    methods in FileDescriptor that are not relevant to sockets, as well
    as to implement buffering independently of the implementation
    nuances required by the different address families. The address
    family class (a subclass of SocketAddress) acts as a factory for
    socket implementation objects.

  --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.AbstractSocket class-defaults](Sockets_002eAbstractSocket-class_002ddefaults.html#Sockets_002eAbstractSocket-class_002ddefaults){accesskey="1"}:                                                    (class)
  • [Sockets.AbstractSocket class-instance creation](Sockets_002eAbstractSocket-class_002dinstance-creation.html#Sockets_002eAbstractSocket-class_002dinstance-creation){accesskey="2"}:                         (class)
  • [Sockets.AbstractSocket class-timed-out operations](Sockets_002eAbstractSocket-class_002dtimed_002dout-operations.html#Sockets_002eAbstractSocket-class_002dtimed_002dout-operations){accesskey="3"}:        (class)
  • [Sockets.AbstractSocket class-well known ports](Sockets_002eAbstractSocket-class_002dwell-known-ports.html#Sockets_002eAbstractSocket-class_002dwell-known-ports){accesskey="4"}:                            (class)
  • [Sockets.AbstractSocket-accessing](Sockets_002eAbstractSocket_002daccessing.html#Sockets_002eAbstractSocket_002daccessing){accesskey="5"}:                                                                   (instance)
  • [Sockets.AbstractSocket-printing](Sockets_002eAbstractSocket_002dprinting.html#Sockets_002eAbstractSocket_002dprinting){accesskey="6"}:                                                                      (instance)
  • [Sockets.AbstractSocket-socket options](Sockets_002eAbstractSocket_002dsocket-options.html#Sockets_002eAbstractSocket_002dsocket-options){accesskey="7"}:                                                    (instance)
  • [Sockets.AbstractSocket-stream protocol](Sockets_002eAbstractSocket_002dstream-protocol.html#Sockets_002eAbstractSocket_002dstream-protocol){accesskey="8"}:                                                 (instance)
  • [Sockets.AbstractSocket-testing](Sockets_002eAbstractSocket_002dtesting.html#Sockets_002eAbstractSocket_002dtesting){accesskey="9"}:                                                                         (instance)
  --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
