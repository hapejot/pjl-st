[]{#Sockets_002eServerSocket}

::: header
Next:
[Sockets.Socket](Sockets_002eSocket.html#Sockets_002eSocket){accesskey="n"
rel="next"}, Previous:
[Sockets.ReadBuffer](Sockets_002eReadBuffer.html#Sockets_002eReadBuffer){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eServerSocket-1}

### 6.18 Sockets.ServerSocket {#sockets.serversocket .section}

[]{#index-Sockets_002eServerSocket}

**Defined in namespace Sockets**\
**Superclass: Sockets.AbstractSocket**\
**Category: Sockets-Streams**

:   This class models server side sockets. The basic model is that the
    server socket is created and bound to some well known port. It then
    listens for and accepts connections. At that point the client and
    server sockets are ready to communicate with one another utilizing
    whatever application layer protocol they desire.

    As with the other AbstractSocket subclasses, most instance methods
    of this class simply redirect their calls to an implementation
    class.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.ServerSocket class-accessing](Sockets_002eServerSocket-class_002daccessing.html#Sockets_002eServerSocket-class_002daccessing){accesskey="1"}:                                (class)
  • [Sockets.ServerSocket class-instance creation](Sockets_002eServerSocket-class_002dinstance-creation.html#Sockets_002eServerSocket-class_002dinstance-creation){accesskey="2"}:        (class)
  • [Sockets.ServerSocket-accessing](Sockets_002eServerSocket_002daccessing.html#Sockets_002eServerSocket_002daccessing){accesskey="3"}:                                                  (instance)
  • [Sockets.ServerSocket-initializing](Sockets_002eServerSocket_002dinitializing.html#Sockets_002eServerSocket_002dinitializing){accesskey="4"}:                                         (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
