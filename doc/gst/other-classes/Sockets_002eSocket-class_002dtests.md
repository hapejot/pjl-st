[]{#Sockets_002eSocket-class_002dtests}

::: header
Next: [Sockets.Socket class-well known
ports](Sockets_002eSocket-class_002dwell-known-ports.html#Sockets_002eSocket-class_002dwell-known-ports){accesskey="n"
rel="next"}, Previous: [Sockets.Socket
class-accessing](Sockets_002eSocket-class_002daccessing.html#Sockets_002eSocket-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[Sockets.Socket](Sockets_002eSocket.html#Sockets_002eSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocket-class_003a-tests}

#### 6.19.2 Sockets.Socket class: tests {#sockets.socket-class-tests .subsection}

[]{#index-datagramLoopbackTest}

**datagramLoopbackTest**

Send data from one datagram socket to another on the local machine.
Tests most of the socket primitives and works with different processes.

[]{#index-datagramLoopbackTestOn_003a}

**datagramLoopbackTestOn: addressClass**

Send data from one datagram socket to another on the local machine.
Tests most of the socket primitives and works with different processes.

[]{#index-loopbackTest}

**loopbackTest**

Send data from one socket to another on the local machine. Tests most of
the socket primitives.

[]{#index-loopbackTest_003a}

**loopbackTest: bufferSizes**

Send data from one socket to another on the local machine. Tests most of
the socket primitives. The parameter is the size of the input and output
buffer sizes.

[]{#index-loopbackTest_003aaddressClass_003a}

**loopbackTest: bufferSizes addressClass: addressClass**

Send data from one socket to another on the local machine. Tests most of
the socket primitives. The parameters are the size of the input and
output buffer sizes, and the address class (family) to use.

[]{#index-loopbackTestOn_003a}

**loopbackTestOn: addressClass**

Send data from one socket to another on the local machine. Tests most of
the socket primitives. The parameter is the address class (family) to
use.

[]{#index-microTest}

**microTest**

Extremely small test (try to receive SMTP header)

[]{#index-producerConsumerTest}

**producerConsumerTest**

Send data from one datagram socket to another on the local machine.
Tests most of the socket primitives and works with different processes.

[]{#index-producerConsumerTestOn_003a}

**producerConsumerTestOn: addressClass**

Send data from one socket to another on the local machine. Tests most of
the socket primitives and works with different processes.

[]{#index-sendTest}

**sendTest**

Send data to the 'discard' socket of localhost.

[]{#index-sendTest_003a}

**sendTest: host**

Send data to the 'discard' socket of the given host. Tests the speed of
one-way data transfers across the network to the given host. Note that
many hosts do not run a discard server.

[]{#index-testPort2For_003a}

**testPort2For: anAddressClass**

Not commented.

[]{#index-testPortFor_003a}

**testPortFor: anAddressClass**

Not commented.

[]{#index-tweakedLoopbackTest}

**tweakedLoopbackTest**

Send data from one socket to another on the local machine, trying to
avoid buffering overhead. Tests most of the socket primitives.
Comparison of the results of loopbackTest and tweakedLoopbackTest should
give a measure of the overhead of buffering when sending/receiving large
quantities of data.

------------------------------------------------------------------------

::: header
Next: [Sockets.Socket class-well known
ports](Sockets_002eSocket-class_002dwell-known-ports.html#Sockets_002eSocket-class_002dwell-known-ports){accesskey="n"
rel="next"}, Previous: [Sockets.Socket
class-accessing](Sockets_002eSocket-class_002daccessing.html#Sockets_002eSocket-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[Sockets.Socket](Sockets_002eSocket.html#Sockets_002eSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
