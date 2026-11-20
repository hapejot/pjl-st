[]{#Sockets_002eAbstractSocket-class_002dtimed_002dout-operations}

::: header
Next: [Sockets.AbstractSocket class-well known
ports](Sockets_002eAbstractSocket-class_002dwell-known-ports.html#Sockets_002eAbstractSocket-class_002dwell-known-ports){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocket class-instance
creation](Sockets_002eAbstractSocket-class_002dinstance-creation.html#Sockets_002eAbstractSocket-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket-class_003a-timed_002dout-operations}

#### 6.1.3 Sockets.AbstractSocket class: timed-out operations {#sockets.abstractsocket-class-timed-out-operations .subsection}

[]{#index-checkPeriod}

**checkPeriod**

Answer the period that is to elapse between socket polls if data data is
not ready and the connection is still open (in milliseconds)

[]{#index-checkPeriod_003a}

**checkPeriod: anInteger**

Set the period that is to elapse between socket polls if data data is
not ready and the connection is still open (in milliseconds)

[]{#index-timeout}

**timeout**

Answer the period that is to elapse between the request for (yet
unavailable) data and the moment when the connection is considered dead
(in milliseconds)

[]{#index-timeout_003a}

**timeout: anInteger**

Set the period that is to elapse between the request for (yet
unavailable) data and the moment when the connection is considered dead
(in milliseconds)
