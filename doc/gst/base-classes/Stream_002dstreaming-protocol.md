[]{#Stream_002dstreaming-protocol}

::: header
Next:
[Stream-testing](Stream_002dtesting.html#Stream_002dtesting){accesskey="n"
rel="next"}, Previous:
[Stream-storing](Stream_002dstoring.html#Stream_002dstoring){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-streaming-protocol}

#### 1.157.17 Stream: streaming protocol {#stream-streaming-protocol .subsection}

[]{#index-nextAvailablePutAllOn_003a}

**nextAvailablePutAllOn: aStream**

Copy to aStream a more-or-less arbitrary amount of data. When used on
files, this does at most one I/O operation. For other kinds of stream,
the definition may vary. This method is used to do stream-to-stream
copies.
