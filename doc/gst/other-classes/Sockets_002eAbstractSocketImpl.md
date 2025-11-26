[]{#Sockets_002eAbstractSocketImpl}

::: header
Next:
[Sockets.CAddrInfoStruct](Sockets_002eCAddrInfoStruct.html#Sockets_002eCAddrInfoStruct){accesskey="n"
rel="next"}, Previous:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocketImpl-1}

### 6.2 Sockets.AbstractSocketImpl {#sockets.abstractsocketimpl .section}

[]{#index-Sockets_002eAbstractSocketImpl}

**Defined in namespace Sockets**\
**Superclass: FileDescriptor**\
**Category: Sockets-Protocols**

:   This abstract class serves as the parent class for socket
    implementations. The implementation class serves an intermediary to
    routines that perform the actual socket operations. It hides the
    buffering and blocking behavior of the Socket classes.

    A default implementation is provided by each address family, but
    this can be changed by class methods on SocketAddress sublcasses.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.AbstractSocketImpl class-abstract](Sockets_002eAbstractSocketImpl-class_002dabstract.html#Sockets_002eAbstractSocketImpl-class_002dabstract){accesskey="1"}:                                   (class)
  • [Sockets.AbstractSocketImpl class-C call-outs](Sockets_002eAbstractSocketImpl-class_002dC-call_002douts.html#Sockets_002eAbstractSocketImpl-class_002dC-call_002douts){accesskey="2"}:                  (class)
  • [Sockets.AbstractSocketImpl class-C constants](Sockets_002eAbstractSocketImpl-class_002dC-constants.html#Sockets_002eAbstractSocketImpl-class_002dC-constants){accesskey="3"}:                          (class)
  • [Sockets.AbstractSocketImpl class-socket creation](Sockets_002eAbstractSocketImpl-class_002dsocket-creation.html#Sockets_002eAbstractSocketImpl-class_002dsocket-creation){accesskey="4"}:              (class)
  • [Sockets.AbstractSocketImpl-accessing](Sockets_002eAbstractSocketImpl_002daccessing.html#Sockets_002eAbstractSocketImpl_002daccessing){accesskey="5"}:                                                  (instance)
  • [Sockets.AbstractSocketImpl-asynchronous operations](Sockets_002eAbstractSocketImpl_002dasynchronous-operations.html#Sockets_002eAbstractSocketImpl_002dasynchronous-operations){accesskey="6"}:        (instance)
  • [Sockets.AbstractSocketImpl-C call-outs](Sockets_002eAbstractSocketImpl_002dC-call_002douts.html#Sockets_002eAbstractSocketImpl_002dC-call_002douts){accesskey="7"}:                                    (instance)
  • [Sockets.AbstractSocketImpl-C constants](Sockets_002eAbstractSocketImpl_002dC-constants.html#Sockets_002eAbstractSocketImpl_002dC-constants){accesskey="8"}:                                            (instance)
  • [Sockets.AbstractSocketImpl-socket operations](Sockets_002eAbstractSocketImpl_002dsocket-operations.html#Sockets_002eAbstractSocketImpl_002dsocket-operations){accesskey="9"}:                          (instance)
  • [Sockets.AbstractSocketImpl-socket options](Sockets_002eAbstractSocketImpl_002dsocket-options.html#Sockets_002eAbstractSocketImpl_002dsocket-options):                                                  (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
