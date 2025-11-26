[]{#Sockets_002eDatagramSocket-class_002daccessing}

::: header
Next: [Sockets.DatagramSocket
class-initialization](Sockets_002eDatagramSocket-class_002dinitialization.html#Sockets_002eDatagramSocket-class_002dinitialization){accesskey="n"
rel="next"}, Up:
[Sockets.DatagramSocket](Sockets_002eDatagramSocket.html#Sockets_002eDatagramSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagramSocket-class_003a-accessing}

#### 6.6.1 Sockets.DatagramSocket class: accessing {#sockets.datagramsocket-class-accessing .subsection}

[]{#index-defaultBufferSize}

**defaultBufferSize**

Answer the default maximum size for input datagrams.

[]{#index-defaultBufferSize_003a}

**defaultBufferSize: size**

Set the default maximum size for input datagrams.

[]{#index-defaultImplementationClassFor_003a-1}

**defaultImplementationClassFor: aSocketAddressClass**

Answer the default implementation class. Depending on the subclass, this
might be the default stream socket implementation class of the given
address class, or rather its default datagram socket implementation
class.
