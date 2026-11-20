[]{#Sockets-package}

::: header
Next: [ZLib package](ZLib-package.html#ZLib-package){accesskey="n"
rel="next"}, Previous: [Iconv/I18N
packages](Iconv_002fI18N-packages.html#Iconv_002fI18N-packages){accesskey="p"
rel="prev"}, Up: [Top](index.html#Top){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Network-programming-with-Sockets}

## 6 Network programming with Sockets {#network-programming-with-sockets .chapter}

+-----------------------+-----------------------+-----------------------+
| ``` menu-comment      |                       |                       |
| Alphabetic list:      |                       |                       |
| ```                   |                       |                       |
+:======================+=======================+:======================+
| •                     |                       |                       |
| [Sockets.Abstrac      |                       |                       |
| tSocket](Sockets_002e |                       |                       |
| AbstractSocket.html#S |                       |                       |
| ockets_002eAbstractSo |                       |                       |
| cket){accesskey="1"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Socket               |                       |                       |
| s.AbstractSocketImpl] |                       |                       |
| (Sockets_002eAbstract |                       |                       |
| SocketImpl.html#Socke |                       |                       |
| ts_002eAbstractSocket |                       |                       |
| Impl){accesskey="2"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.CAddrInfoS   |                       |                       |
| truct](Sockets_002eCA |                       |                       |
| ddrInfoStruct.html#So |                       |                       |
| ckets_002eCAddrInfoSt |                       |                       |
| ruct){accesskey="3"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Socket               |                       |                       |
| s.CSockAddrIn6Struct] |                       |                       |
| (Sockets_002eCSockAdd |                       |                       |
| rIn6Struct.html#Socke |                       |                       |
| ts_002eCSockAddrIn6St |                       |                       |
| ruct){accesskey="4"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Datagram](   |                       |                       |
| Sockets_002eDatagram. |                       |                       |
| html#Sockets_002eData |                       |                       |
| gram){accesskey="5"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Datagra      |                       |                       |
| mSocket](Sockets_002e |                       |                       |
| DatagramSocket.html#S |                       |                       |
| ockets_002eDatagramSo |                       |                       |
| cket){accesskey="6"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Socket               |                       |                       |
| s.DatagramSocketImpl] |                       |                       |
| (Sockets_002eDatagram |                       |                       |
| SocketImpl.html#Socke |                       |                       |
| ts_002eDatagramSocket |                       |                       |
| Impl){accesskey="7"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Socket               |                       |                       |
| s.DummyStream](Socket |                       |                       |
| s_002eDummyStream.htm |                       |                       |
| l#Sockets_002eDummySt |                       |                       |
| ream){accesskey="8"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.ICMP6Socke   |                       |                       |
| tImpl](Sockets_002eIC |                       |                       |
| MP6SocketImpl.html#So |                       |                       |
| ckets_002eICMP6Socket |                       |                       |
| Impl){accesskey="9"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [                     |                       |                       |
| Sockets.ICMPSocketImp |                       |                       |
| l](Sockets_002eICMPSo |                       |                       |
| cketImpl.html#Sockets |                       |                       |
| _002eICMPSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.I            |                       |                       |
| P6Address](Sockets_00 |                       |                       |
| 2eIP6Address.html#Soc |                       |                       |
| kets_002eIP6Address): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Socket               |                       |                       |
| s.IPAddress](Sockets_ |                       |                       |
| 002eIPAddress.html#So |                       |                       |
| ckets_002eIPAddress): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Soc                  |                       |                       |
| kets.MulticastSocket] |                       |                       |
| (Sockets_002eMulticas |                       |                       |
| tSocket.html#Sockets_ |                       |                       |
| 002eMulticastSocket): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Multica      |                       |                       |
| stSocketImpl](Sockets |                       |                       |
| _002eMulticastSocketI |                       |                       |
| mpl.html#Sockets_002e |                       |                       |
| MulticastSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.OOBSocketI   |                       |                       |
| mpl](Sockets_002eOOBS |                       |                       |
| ocketImpl.html#Socket |                       |                       |
| s_002eOOBSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.RawSocketI   |                       |                       |
| mpl](Sockets_002eRawS |                       |                       |
| ocketImpl.html#Socket |                       |                       |
| s_002eRawSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.R            |                       |                       |
| eadBuffer](Sockets_00 |                       |                       |
| 2eReadBuffer.html#Soc |                       |                       |
| kets_002eReadBuffer): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.ServerS      |                       |                       |
| ocket](Sockets_002eSe |                       |                       |
| rverSocket.html#Socke |                       |                       |
| ts_002eServerSocket): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Socket](So   |                       |                       |
| ckets_002eSocket.html |                       |                       |
| #Sockets_002eSocket): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.SocketAddr   |                       |                       |
| ess](Sockets_002eSock |                       |                       |
| etAddress.html#Socket |                       |                       |
| s_002eSocketAddress): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.S            |                       |                       |
| ocketImpl](Sockets_00 |                       |                       |
| 2eSocketImpl.html#Soc |                       |                       |
| kets_002eSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.StreamS      |                       |                       |
| ocket](Sockets_002eSt |                       |                       |
| reamSocket.html#Socke |                       |                       |
| ts_002eStreamSocket): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.TCPSocketI   |                       |                       |
| mpl](Sockets_002eTCPS |                       |                       |
| ocketImpl.html#Socket |                       |                       |
| s_002eTCPSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.UDPSocketI   |                       |                       |
| mpl](Sockets_002eUDPS |                       |                       |
| ocketImpl.html#Socket |                       |                       |
| s_002eUDPSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Unix         |                       |                       |
| Address](Sockets_002e |                       |                       |
| UnixAddress.html#Sock |                       |                       |
| ets_002eUnixAddress): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Soc                  |                       |                       |
| kets.UnixDatagramSock |                       |                       |
| etImpl](Sockets_002eU |                       |                       |
| nixDatagramSocketImpl |                       |                       |
| .html#Sockets_002eUni |                       |                       |
| xDatagramSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [                     |                       |                       |
| Sockets.UnixSocketImp |                       |                       |
| l](Sockets_002eUnixSo |                       |                       |
| cketImpl.html#Sockets |                       |                       |
| _002eUnixSocketImpl): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [Sockets.Writ         |                       |                       |
| eBuffer](Sockets_002e |                       |                       |
| WriteBuffer.html#Sock |                       |                       |
| ets_002eWriteBuffer): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| ``` menu-comment      |                       |                       |
|                       |                       |                       |
|                       |                       |                       |
| Classe                |                       |                       |
| s documented in this  |                       |                       |
| manual are boldfaced. |                       |                       |
| ```                   |                       |                       |
|                       |                       |                       |
| ::: display           |                       |                       |
| ``` display           |                       |                       |
| Object                |                       |                       |
|   CObject             |                       |                       |
|     CCompound         |                       |                       |
|       CStruct         |                       |                       |
|         So            |                       |                       |
| ckets.CAddrInfoStruct |                       |                       |
|         Socke         |                       |                       |
| ts.CSockAddrIn6Struct |                       |                       |
|   Iterable            |                       |                       |
|     Stream            |                       |                       |
|       FileDescriptor  |                       |                       |
|         Socke         |                       |                       |
| ts.AbstractSocketImpl |                       |                       |
|           Socke       |                       |                       |
| ts.DatagramSocketImpl |                       |                       |
|             Socket    |                       |                       |
| s.MulticastSocketImpl |                       |                       |
|                       |                       |                       |
| Sockets.UDPSocketImpl |                       |                       |
|                       |                       |                       |
| Sockets.OOBSocketImpl |                       |                       |
|                       |                       |                       |
| Sockets.RawSocketImpl |                       |                       |
|               So      |                       |                       |
| ckets.ICMP6SocketImpl |                       |                       |
|               S       |                       |                       |
| ockets.ICMPSocketImpl |                       |                       |
|             Sockets.U |                       |                       |
| nixDatagramSocketImpl |                       |                       |
|                       |                       |                       |
|    Sockets.SocketImpl |                       |                       |
|                       |                       |                       |
| Sockets.TCPSocketImpl |                       |                       |
|             S         |                       |                       |
| ockets.UnixSocketImpl |                       |                       |
|                       |                       |                       |
|    PositionableStream |                       |                       |
|         ReadStream    |                       |                       |
|                       |                       |                       |
|    Sockets.ReadBuffer |                       |                       |
|         WriteStream   |                       |                       |
|                       |                       |                       |
|   Sockets.WriteBuffer |                       |                       |
|       S               |                       |                       |
| ockets.AbstractSocket |                       |                       |
|         S             |                       |                       |
| ockets.DatagramSocket |                       |                       |
|           So          |                       |                       |
| ckets.MulticastSocket |                       |                       |
|                       |                       |                       |
|  Sockets.ServerSocket |                       |                       |
|                       |                       |                       |
|  Sockets.StreamSocket |                       |                       |
|                       |                       |                       |
|        Sockets.Socket |                       |                       |
|                       |                       |                       |
|   Sockets.DummyStream |                       |                       |
|   Sockets.Datagram    |                       |                       |
|                       |                       |                       |
| Sockets.SocketAddress |                       |                       |
|                       |                       |                       |
|    Sockets.IP6Address |                       |                       |
|     Sockets.IPAddress |                       |                       |
|                       |                       |                       |
|   Sockets.UnixAddress |                       |                       |
| ```                   |                       |                       |
| :::                   |                       |                       |
+-----------------------+-----------------------+-----------------------+

------------------------------------------------------------------------

::: header
Next: [ZLib package](ZLib-package.html#ZLib-package){accesskey="n"
rel="next"}, Previous: [Iconv/I18N
packages](Iconv_002fI18N-packages.html#Iconv_002fI18N-packages){accesskey="p"
rel="prev"}, Up: [Top](index.html#Top){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
