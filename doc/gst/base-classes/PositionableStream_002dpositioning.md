[]{#PositionableStream_002dpositioning}

::: header
Next: [PositionableStream-still
unclassified](PositionableStream_002dstill-unclassified.html#PositionableStream_002dstill-unclassified){accesskey="n"
rel="next"}, Previous:
[PositionableStream-compiling](PositionableStream_002dcompiling.html#PositionableStream_002dcompiling){accesskey="p"
rel="prev"}, Up:
[PositionableStream](PositionableStream.html#PositionableStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PositionableStream_003a-positioning}

#### 1.133.5 PositionableStream: positioning {#positionablestream-positioning .subsection}

[]{#index-basicPosition_003a}

**basicPosition: anInteger**

Move the stream pointer to the anInteger-th object

[]{#index-isPositionable-1} []{#index-skip_003a-4}

**isPositionable**

Answer true if the stream supports moving backwards with #skip:.

[]{#index-position-2}

**position**

Answer the current value of the stream pointer

[]{#index-position_003a-2}

**position: anInteger**

Move the stream pointer to the anInteger-th object

[]{#index-reset-1}

**reset**

Move the stream back to its first element. For write-only streams, the
stream is truncated there.

[]{#index-setToEnd-1}

**setToEnd**

Move the current position to the end of the stream.

[]{#index-size-20}

**size**

Answer the size of data on which we are streaming.

[]{#index-skip_003a-1}

**skip: anInteger**

Move the current position by anInteger places, either forwards or
backwards.
