[]{#Sockets_002eAbstractSocket_002dstream-protocol}

::: header
Next:
[Sockets.AbstractSocket-testing](Sockets_002eAbstractSocket_002dtesting.html#Sockets_002eAbstractSocket_002dtesting){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocket-socket
options](Sockets_002eAbstractSocket_002dsocket-options.html#Sockets_002eAbstractSocket_002dsocket-options){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket_003a-stream-protocol}

#### 6.1.8 Sockets.AbstractSocket: stream protocol {#sockets.abstractsocket-stream-protocol .subsection}

[]{#index-atEnd-2}

**atEnd**

By default, answer whether the connection is still open.

[]{#index-next-3}

**next**

Read another character from the socket, failing if the connection is
dead.

[]{#index-next_003aputAll_003astartingAt_003a}

**next: n putAll: aCollection startingAt: pos**

Write 'char' to the socket, failing if the connection is dead. The
SIGPIPE signal is automatically caught and ignored by the system.

[]{#index-nextPut_003a-2}

**nextPut: char**

Write 'char' to the socket, failing if the connection is dead. The
SIGPIPE signal is automatically caught and ignored by the system.
