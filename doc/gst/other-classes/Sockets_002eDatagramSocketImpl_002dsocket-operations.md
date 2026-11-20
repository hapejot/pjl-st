[]{#Sockets_002eDatagramSocketImpl_002dsocket-operations}

::: header
Previous: [Sockets.DatagramSocketImpl-C
constants](Sockets_002eDatagramSocketImpl_002dC-constants.html#Sockets_002eDatagramSocketImpl_002dC-constants){accesskey="p"
rel="prev"}, Up:
[Sockets.DatagramSocketImpl](Sockets_002eDatagramSocketImpl.html#Sockets_002eDatagramSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagramSocketImpl_003a-socket-operations}

#### 6.7.4 Sockets.DatagramSocketImpl: socket operations {#sockets.datagramsocketimpl-socket-operations .subsection}

[]{#index-next-5}

**next**

Retrieve a datagram from the receiver, answer a new object of the
receiver's datagram class.

[]{#index-nextPut_003a-4}

**nextPut: aDatagram**

Send aDatagram on the socket

[]{#index-peek-1}

**peek**

Peek for a datagram on the receiver, answer a new object of the
receiver's datagram class.

[]{#index-peek_003a-1}

**peek: aDatagram**

Peek for a datagram on the receiver, answer aDatagram modified to
contain information on the newly received datagram.

[]{#index-receive_003a-1}

**receive: aDatagram**

Retrieve a datagram from the receiver, answer aDatagram modified to
contain information on the newly received datagram.

[]{#index-receive_003adatagram_003a}

**receive: flags datagram: aDatagram**

Receive a new datagram into 'datagram', with the given flags, and answer
'datagram' itself; this is an abstract method. The flags can be zero to
receive the datagram, or 'self msgPeek' to only peek for it without
removing it from the queue.

[]{#index-send_003ato_003aport_003a}

**send: aDatagram to: theReceiver port: port**

Send aDatagram on the socket to the given receiver and port
