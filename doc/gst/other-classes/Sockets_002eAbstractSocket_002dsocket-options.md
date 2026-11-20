[]{#Sockets_002eAbstractSocket_002dsocket-options}

::: header
Next: [Sockets.AbstractSocket-stream
protocol](Sockets_002eAbstractSocket_002dstream-protocol.html#Sockets_002eAbstractSocket_002dstream-protocol){accesskey="n"
rel="next"}, Previous:
[Sockets.AbstractSocket-printing](Sockets_002eAbstractSocket_002dprinting.html#Sockets_002eAbstractSocket_002dprinting){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket_003a-socket-options}

#### 6.1.7 Sockets.AbstractSocket: socket options {#sockets.abstractsocket-socket-options .subsection}

[]{#index-soLinger}

**soLinger**

Answer the number of seconds that the socket is allowed to wait if it
promises reliable delivery but has unacknowledged/untransmitted packets
when it is closed, or nil if those packets are left to their destiny or
discarded.

[]{#index-soLinger_003a}

**soLinger: linger**

Set the number of seconds that the socket is allowed to wait if it
promises reliable delivery but has unacknowledged/untransmitted packets
when it is closed.

[]{#index-soLingerOff}

**soLingerOff**

Specify that, even if the socket promises reliable delivery, any packets
that are unacknowledged/untransmitted when it is closed are to be left
to their destiny or discarded.

[]{#index-species-2}

**species**

Answer 'String'.
