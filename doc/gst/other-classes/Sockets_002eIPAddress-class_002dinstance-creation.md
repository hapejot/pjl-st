[]{#Sockets_002eIPAddress-class_002dinstance-creation}

::: header
Next:
[Sockets.IPAddress-accessing](Sockets_002eIPAddress_002daccessing.html#Sockets_002eIPAddress_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.IPAddress
class-initialization](Sockets_002eIPAddress-class_002dinitialization.html#Sockets_002eIPAddress-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.IPAddress](Sockets_002eIPAddress.html#Sockets_002eIPAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eIPAddress-class_003a-instance-creation}

#### 6.12.4 Sockets.IPAddress class: instance creation {#sockets.ipaddress-class-instance-creation .subsection}

[]{#index-fromArray_003a-1}

**fromArray: parts**

Answer a new IPAddress from an array of numbers; the numbers are to be
thought as the dot-separated numbers in the standard numbers-and-dots
notation for IPv4 addresses.

[]{#index-fromBytes_003a-1}

**fromBytes: aByteArray**

Answer a new IPAddress from a ByteArray containing the bytes in the same
order as the digit form: 131.175.6.2 would be represented as #\[131 175
6 2\].

[]{#index-fromSockAddr_003aport_003a-1}

**fromSockAddr: aByteArray port: portAdaptor**

Private - Answer a new IPAddress from a ByteArray containing a C
sockaddr_in structure. The portAdaptor's value is changed to contain the
port that the structure refers to.

[]{#index-fromString_003a-6} []{#index-loopbackHost-1}

**fromString: aString**

Answer a new IPAddress from a String containing the requested address in
digit form. Hexadecimal forms are not allowed.

An Internet host address is a number containing four bytes of data.
These are divided into two parts, a network number and a local network
address number within that network. The network number consists of the
first one, two or three bytes; the rest of the bytes are the local
address.

Network numbers are registered with the Network Information Center
(NIC), and are divided into three classes--A, B, and C. The local
network address numbers of individual machines are registered with the
administrator of the particular network.

Class A networks have single-byte numbers in the range 0 to 127. There
are only a small number of Class A networks, but they can each support a
very large number of hosts (several millions). Medium-sized Class B
networks have two-byte network numbers, with the first byte in the range
128 to 191; they support several thousands of host, but are almost
exhausted. Class C networks are the smallest and the most commonly
available; they have three-byte network numbers, with the first byte in
the range 192-223. Class D (multicast, 224.0.0.0 to 239.255.255.255) and
E (research, 240.0.0.0 to 255.255.255.255) also have three-byte network
numbers.

Thus, the first 1, 2, or 3 bytes of an Internet address specifies a
network. The remaining bytes of the Internet address specify the address
within that network. The Class A network 0 is reserved for broadcast to
all networks. In addition, the host number 0 within each network is
reserved for broadcast to all hosts in that network. The Class A network
127 is reserved for loopback; you can always use the Internet address
'127.0.0.1' to refer to the host machine (this is answered by the
#loopbackHost class method).

Since a single machine can be a member of multiple networks, it can have
multiple Internet host addresses. However, there is never supposed to be
more than one machine with the same host address.

There are four forms of the standard numbers-and-dots notation for
Internet addresses: a.b.c.d specifies all four bytes of the address
individually; a.b.c interprets as a 2-byte quantity, which is useful for
specifying host addresses in a Class B network with network address
number a.b; a.b intrprets the last part of the address as a 3-byte
quantity, which is useful for specifying host addresses in a Class A
network with network address number a.

If only one part is given, this corresponds directly to the host address
number.

[]{#index-new-15}

**new**

This method should not be called for instances of this class.

[]{#index-with_003awith_003awith_003awith_003a}

**with: b1 with: b2 with: b3 with: b4**

Answer a new IPAddress whose bytes (from most-significant to
least-significant) are in the parameters.

------------------------------------------------------------------------

::: header
Next:
[Sockets.IPAddress-accessing](Sockets_002eIPAddress_002daccessing.html#Sockets_002eIPAddress_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.IPAddress
class-initialization](Sockets_002eIPAddress-class_002dinitialization.html#Sockets_002eIPAddress-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.IPAddress](Sockets_002eIPAddress.html#Sockets_002eIPAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
