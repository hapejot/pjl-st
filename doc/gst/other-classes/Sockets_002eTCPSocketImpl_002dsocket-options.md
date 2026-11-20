[]{#Sockets_002eTCPSocketImpl_002dsocket-options}

::: header
Previous: [Sockets.TCPSocketImpl class-C
constants](Sockets_002eTCPSocketImpl-class_002dC-constants.html#Sockets_002eTCPSocketImpl-class_002dC-constants){accesskey="p"
rel="prev"}, Up:
[Sockets.TCPSocketImpl](Sockets_002eTCPSocketImpl.html#Sockets_002eTCPSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eTCPSocketImpl_003a-socket-options}

#### 6.23.2 Sockets.TCPSocketImpl: socket options {#sockets.tcpsocketimpl-socket-options .subsection}

[]{#index-valueWithoutBuffering_003a-1}

**valueWithoutBuffering: aBlock**

Evaluate aBlock, ensuring that any data that it writes to the socket is
sent immediately to the network.
