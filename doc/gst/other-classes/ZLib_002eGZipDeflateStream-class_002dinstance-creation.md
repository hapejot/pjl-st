[]{#ZLib_002eGZipDeflateStream-class_002dinstance-creation}

::: header
Up:
[ZLib.GZipDeflateStream](ZLib_002eGZipDeflateStream.html#ZLib_002eGZipDeflateStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eGZipDeflateStream-class_003a-instance-creation}

#### 7.3.1 ZLib.GZipDeflateStream class: instance creation {#zlib.gzipdeflatestream-class-instance-creation .subsection}

[]{#index-compressingTo_003a-1} []{#index-nextPut_003a-9}

**compressingTo: aStream**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream.

[]{#index-compressingTo_003alevel_003a-1} []{#index-nextPut_003a-10}

**compressingTo: aStream level: level**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream with the given compression level.
