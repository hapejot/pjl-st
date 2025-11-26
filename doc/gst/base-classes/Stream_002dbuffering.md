[]{#Stream_002dbuffering}

::: header
Next: [Stream-built
ins](Stream_002dbuilt-ins.html#Stream_002dbuilt-ins){accesskey="n"
rel="next"}, Previous:
[Stream-basic](Stream_002dbasic.html#Stream_002dbasic){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-buffering}

#### 1.157.4 Stream: buffering {#stream-buffering .subsection}

[]{#index-next_003ainto_003astartingAt_003a}

**next: anInteger into: answer startingAt: pos**

Read up to anInteger bytes from the stream and store them into answer.
Return the number of bytes that were read, raising an exception if we
could not read the full amount of data.

[]{#index-next_003aputAllOn_003a}

**next: anInteger putAllOn: aStream**

Read up to anInteger bytes from the stream and store them into aStream.
Return the number of bytes that were read, raising an exception if we
could not read the full amount of data.
