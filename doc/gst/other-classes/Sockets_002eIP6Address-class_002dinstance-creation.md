[]{#Sockets_002eIP6Address-class_002dinstance-creation}

::: header
Next:
[Sockets.IP6Address-accessing](Sockets_002eIP6Address_002daccessing.html#Sockets_002eIP6Address_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.IP6Address
class-initialization](Sockets_002eIP6Address-class_002dinitialization.html#Sockets_002eIP6Address-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.IP6Address](Sockets_002eIP6Address.html#Sockets_002eIP6Address){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eIP6Address-class_003a-instance-creation}

#### 6.11.4 Sockets.IP6Address class: instance creation {#sockets.ip6address-class-instance-creation .subsection}

[]{#index-fromArray_003a}

**fromArray: parts**

Answer a new IP6Address from an array of numbers; the numbers are to be
thought as the colon-separated numbers in the standard
numbers-and-colons notation for IPv4 addresses.

[]{#index-fromBytes_003a}

**fromBytes: aByteArray**

Answer a new IP6Address from a ByteArray containing the bytes in the
same order as the digit form: 131.175.6.2 would be represented as #\[131
175 6 2\].

[]{#index-fromSockAddr_003aport_003a}

**fromSockAddr: aByteArray port: portAdaptor**

Private - Answer a new IP6Address from a ByteArray containing a C
sockaddr_in structure. The portAdaptor's value is changed to contain the
port that the structure refers to.

[]{#index-fromString_003a-5}

**fromString: aString**

Answer a new IP6Address from a String containing the requested address
in digit form.

[]{#index-new-14}

**new**

This method should not be called for instances of this class.
