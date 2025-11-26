[]{#ZLib_002eDeflateStream-class_002dinstance-creation}

::: header
Up:
[ZLib.DeflateStream](ZLib_002eDeflateStream.html#ZLib_002eDeflateStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eDeflateStream-class_003a-instance-creation}

#### 7.1.1 ZLib.DeflateStream class: instance creation {#zlib.deflatestream-class-instance-creation .subsection}

[]{#index-compressingTo_003a} []{#index-nextPut_003a-7}

**compressingTo: aStream**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream.

[]{#index-compressingTo_003alevel_003a} []{#index-nextPut_003a-8}

**compressingTo: aStream level: level**

Answer a stream that receives data via #nextPut: and compresses it onto
aStream with the given compression level.
