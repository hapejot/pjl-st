[]{#Sockets_002eDatagramSocket_002daccessing}

::: header
Next: [Sockets.DatagramSocket-direct
operations](Sockets_002eDatagramSocket_002ddirect-operations.html#Sockets_002eDatagramSocket_002ddirect-operations){accesskey="n"
rel="next"}, Previous: [Sockets.DatagramSocket class-instance
creation](Sockets_002eDatagramSocket-class_002dinstance-creation.html#Sockets_002eDatagramSocket-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.DatagramSocket](Sockets_002eDatagramSocket.html#Sockets_002eDatagramSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagramSocket_003a-accessing}

#### 6.6.4 Sockets.DatagramSocket: accessing {#sockets.datagramsocket-accessing .subsection}

[]{#index-address-2}

**address**

Answer the local address.

[]{#index-bufferSize}

**bufferSize**

Answer the size of the buffer in which datagrams are stored.

[]{#index-bufferSize_003a}

**bufferSize: size**

Set the size of the buffer in which datagrams are stored.

[]{#index-datagramClass}

**datagramClass**

Answer the class used by the socket to return datagrams.

[]{#index-next-4}

**next**

Read a datagram on the socket and answer it.

[]{#index-nextPut_003a-3}

**nextPut: aDatagram**

Send the given datagram on the socket.

[]{#index-peek}

**peek**

Peek for a datagram on the socket and answer it.

[]{#index-peek_003a}

**peek: datagram**

Peek for a datagram on the socket, store it in 'datagram', and answer
the datagram itself.

[]{#index-port-2}

**port**

Answer the local port.

[]{#index-receive_003a}

**receive: datagram**

Read a datagram from the socket, store it in 'datagram', and answer the
datagram itself.
