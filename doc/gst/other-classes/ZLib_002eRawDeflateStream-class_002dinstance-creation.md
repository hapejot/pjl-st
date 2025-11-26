[]{#ZLib_002eRawDeflateStream-class_002dinstance-creation}

::: header
Up:
[ZLib.RawDeflateStream](ZLib_002eRawDeflateStream.html#ZLib_002eRawDeflateStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eRawDeflateStream-class_003a-instance-creation}

#### 7.7.1 ZLib.RawDeflateStream class: instance creation {#zlib.rawdeflatestream-class-instance-creation .subsection}

[]{#index-compressingTo_003a-2} []{#index-nextPut_003a-11}

**compressingTo: aStream**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream.

[]{#index-compressingTo_003alevel_003a-2} []{#index-nextPut_003a-12}

**compressingTo: aStream level: level**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream with the given compression level.

[]{#index-on_003a-4}

**on: aStream**

Answer a stream that compresses the data in aStream with the default
compression level.

[]{#index-on_003alevel_003a}

**on: aStream level: compressionLevel**

Answer a stream that compresses the data in aStream with the given
compression level.
