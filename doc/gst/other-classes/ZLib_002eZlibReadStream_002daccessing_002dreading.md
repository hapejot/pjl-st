[]{#ZLib_002eZlibReadStream_002daccessing_002dreading}

::: header
Next:
[ZLib.ZlibReadStream-streaming](ZLib_002eZlibReadStream_002dstreaming.html#ZLib_002eZlibReadStream_002dstreaming){accesskey="n"
rel="next"}, Up:
[ZLib.ZlibReadStream](ZLib_002eZlibReadStream.html#ZLib_002eZlibReadStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eZlibReadStream_003a-accessing_002dreading}

#### 7.11.1 ZLib.ZlibReadStream: accessing-reading {#zlib.zlibreadstream-accessing-reading .subsection}

[]{#index-nextAvailable_003ainto_003astartingAt_003a-2}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Place up to anInteger objects from the receiver into aCollection,
starting from position pos and stopping if no more data is available.

[]{#index-nextAvailable_003aputAllOn_003a-2}

**nextAvailable: anInteger putAllOn: aStream**

Copy up to anInteger objects from the receiver to aStream, stopping if
no more data is available.
