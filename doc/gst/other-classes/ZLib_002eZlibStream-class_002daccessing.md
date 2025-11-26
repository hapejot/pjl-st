[]{#ZLib_002eZlibStream-class_002daccessing}

::: header
Next: [ZLib.ZlibStream class-instance
creation](ZLib_002eZlibStream-class_002dinstance-creation.html#ZLib_002eZlibStream-class_002dinstance-creation){accesskey="n"
rel="next"}, Up:
[ZLib.ZlibStream](ZLib_002eZlibStream.html#ZLib_002eZlibStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eZlibStream-class_003a-accessing}

#### 7.12.1 ZLib.ZlibStream class: accessing {#zlib.zlibstream-class-accessing .subsection}

[]{#index-bufferSize-2}

**bufferSize**

Answer the size of the output buffers that are passed to zlib. Each zlib
stream uses a buffer of this size.

[]{#index-bufferSize_003a-2}

**bufferSize: anInteger**

Set the size of the output buffers that are passed to zlib. Each zlib
stream uses a buffer of this size.

[]{#index-defaultCompressionLevel}

**defaultCompressionLevel**

Return the default compression level used by deflating streams.

[]{#index-defaultCompressionLevel_003a}

**defaultCompressionLevel: anInteger**

Set the default compression level used by deflating streams. It should
be a number between 1 and 9.
