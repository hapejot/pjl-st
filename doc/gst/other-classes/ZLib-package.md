[]{#ZLib-package}

::: header
Next: [XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="n"
rel="next"}, Previous: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="p"
rel="prev"}, Up: [Top](index.html#Top){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Compressing-and-decompressing-data-with-ZLib}

## 7 Compressing and decompressing data with ZLib {#compressing-and-decompressing-data-with-zlib .chapter}

+-----------------------+-----------------------+-----------------------+
| ``` menu-comment      |                       |                       |
| Alphabetic list:      |                       |                       |
| ```                   |                       |                       |
+:======================+=======================+:======================+
| •                     |                       |                       |
| [ZLi                  |                       |                       |
| b.DeflateStream](ZLib |                       |                       |
| _002eDeflateStream.ht |                       |                       |
| ml#ZLib_002eDeflateSt |                       |                       |
| ream){accesskey="1"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.DeflateWriteS   |                       |                       |
| tream](ZLib_002eDefla |                       |                       |
| teWriteStream.html#ZL |                       |                       |
| ib_002eDeflateWriteSt |                       |                       |
| ream){accesskey="2"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.GZipDeflat      |                       |                       |
| eStream](ZLib_002eGZi |                       |                       |
| pDeflateStream.html#Z |                       |                       |
| Lib_002eGZipDeflateSt |                       |                       |
| ream){accesskey="3"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.GZip            |                       |                       |
| DeflateWriteStream](Z |                       |                       |
| Lib_002eGZipDeflateWr |                       |                       |
| iteStream.html#ZLib_0 |                       |                       |
| 02eGZipDeflateWriteSt |                       |                       |
| ream){accesskey="4"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.GZipInflat      |                       |                       |
| eStream](ZLib_002eGZi |                       |                       |
| pInflateStream.html#Z |                       |                       |
| Lib_002eGZipInflateSt |                       |                       |
| ream){accesskey="5"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLi                  |                       |                       |
| b.InflateStream](ZLib |                       |                       |
| _002eInflateStream.ht |                       |                       |
| ml#ZLib_002eInflateSt |                       |                       |
| ream){accesskey="6"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.RawDefl         |                       |                       |
| ateStream](ZLib_002eR |                       |                       |
| awDeflateStream.html# |                       |                       |
| ZLib_002eRawDeflateSt |                       |                       |
| ream){accesskey="7"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.R               |                       |                       |
| awDeflateWriteStream] |                       |                       |
| (ZLib_002eRawDeflateW |                       |                       |
| riteStream.html#ZLib_ |                       |                       |
| 002eRawDeflateWriteSt |                       |                       |
| ream){accesskey="8"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.RawInfl         |                       |                       |
| ateStream](ZLib_002eR |                       |                       |
| awInflateStream.html# |                       |                       |
| ZLib_002eRawInflateSt |                       |                       |
| ream){accesskey="9"}: |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.ZlibError](ZL   |                       |                       |
| ib_002eZlibError.html |                       |                       |
| #ZLib_002eZlibError): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.ZlibRea         |                       |                       |
| dStream](ZLib_002eZli |                       |                       |
| bReadStream.html#ZLib |                       |                       |
| _002eZlibReadStream): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [                     |                       |                       |
| ZLib.ZlibStream](ZLib |                       |                       |
| _002eZlibStream.html# |                       |                       |
| ZLib_002eZlibStream): |                       |                       |
+-----------------------+-----------------------+-----------------------+
| •                     |                       |                       |
| [ZLib.ZlibWriteS      |                       |                       |
| tream](ZLib_002eZlibW |                       |                       |
| riteStream.html#ZLib_ |                       |                       |
| 002eZlibWriteStream): |                       |                       |
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
|   Exception           |                       |                       |
|     Error             |                       |                       |
|       ZLib.ZlibError  |                       |                       |
|   Iterable            |                       |                       |
|     Stream            |                       |                       |
|       ZLib.ZlibStream |                       |                       |
|                       |                       |                       |
|   ZLib.ZlibReadStream |                       |                       |
|                       |                       |                       |
| ZLib.RawDeflateStream |                       |                       |
|                       |                       |                       |
|    ZLib.DeflateStream |                       |                       |
|             Z         |                       |                       |
| Lib.GZipDeflateStream |                       |                       |
|                       |                       |                       |
| ZLib.RawInflateStream |                       |                       |
|             Z         |                       |                       |
| Lib.GZipInflateStream |                       |                       |
|                       |                       |                       |
|    ZLib.InflateStream |                       |                       |
|                       |                       |                       |
|  ZLib.ZlibWriteStream |                       |                       |
|           ZLib.       |                       |                       |
| RawDeflateWriteStream |                       |                       |
|             ZL        |                       |                       |
| ib.DeflateWriteStream |                       |                       |
|             ZLib.G    |                       |                       |
| ZipDeflateWriteStream |                       |                       |
| ```                   |                       |                       |
| :::                   |                       |                       |
+-----------------------+-----------------------+-----------------------+
