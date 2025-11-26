[]{#Sockets_002eIPAddress_002daccessing}

::: header
Next:
[Sockets.IPAddress-printing](Sockets_002eIPAddress_002dprinting.html#Sockets_002eIPAddress_002dprinting){accesskey="n"
rel="next"}, Previous: [Sockets.IPAddress class-instance
creation](Sockets_002eIPAddress-class_002dinstance-creation.html#Sockets_002eIPAddress-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.IPAddress](Sockets_002eIPAddress.html#Sockets_002eIPAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eIPAddress_003a-accessing}

#### 6.12.5 Sockets.IPAddress: accessing {#sockets.ipaddress-accessing .subsection}

[]{#index-addressClass-1} []{#index-fromString_003a-7}

**addressClass**

Answer the 'address class' of the receiver (see IPAddress
class\>\>#fromString:)

[]{#index-asByteArray-1}

**asByteArray**

Answer a read-only ByteArray of size four containing the receiver's
bytes in network order (big-endian)

[]{#index-host}

**host**

Answer an host number for the receiver; this is given by the last three
bytes for class A addresses, by the last two bytes for class B
addresses, else by the last byte.

[]{#index-isMulticast-1}

**isMulticast**

Answer whether the receiver reprensents an address reserved for
multicast datagram connections

[]{#index-network}

**network**

Answer a network number for the receiver; this is given by the first
three bytes for class C/D/E addresses, by the first two bytes for class
B addresses, else by the first byte.

[]{#index-subnet}

**subnet**

Answer an host number for the receiver; this is 0 for class A addresses,
while it is given by the last byte of the network number for class
B/C/D/E addresses.
