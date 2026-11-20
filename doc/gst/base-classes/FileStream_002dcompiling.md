[]{#FileStream_002dcompiling}

::: header
Next:
[FileStream-initialize-release](FileStream_002dinitialize_002drelease.html#FileStream_002dinitialize_002drelease){accesskey="n"
rel="next"}, Previous:
[FileStream-buffering](FileStream_002dbuffering.html#FileStream_002dbuffering){accesskey="p"
rel="prev"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileStream_003a-compiling}

#### 1.79.5 FileStream: compiling {#filestream-compiling .subsection}

[]{#index-segmentFrom_003ato_003a} []{#index-asString-16}

**segmentFrom: startPos to: endPos**

Answer an object that, when sent #asString, will yield the result of
sending 'copyFrom: startPos to: endPos' to the receiver
