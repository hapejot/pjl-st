[]{#Stream_002dtesting}

::: header
Previous: [Stream-streaming
protocol](Stream_002dstreaming-protocol.html#Stream_002dstreaming-protocol){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-testing}

#### 1.157.18 Stream: testing {#stream-testing .subsection}

[]{#index-atEnd-6}

**atEnd**

Answer whether the stream has got to an end

[]{#index-isExternalStream-2}

**isExternalStream**

Answer whether the receiver streams on a file or socket. By default,
answer false.

[]{#index-isSequenceable-2}
[]{#index-at_003a_002f_0023at_003aput_003a-2}

**isSequenceable**

Answer whether the receiver can be accessed by a numeric index with
#at:/#at:put:.

[]{#index-readStream-7}

**readStream**

As a wild guess, return the receiver. WriteStreams should override this
method.
