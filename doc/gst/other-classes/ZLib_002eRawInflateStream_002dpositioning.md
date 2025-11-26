[]{#ZLib_002eRawInflateStream_002dpositioning}

::: header
Up:
[ZLib.RawInflateStream](ZLib_002eRawInflateStream.html#ZLib_002eRawInflateStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eRawInflateStream_003a-positioning}

#### 7.9.1 ZLib.RawInflateStream: positioning {#zlib.rawinflatestream-positioning .subsection}

[]{#index-copyFrom_003ato_003a} []{#index-position-3}

**copyFrom: start to: end**

Answer the data on which the receiver is streaming, from the start-th
item to the end-th. Note that this method is 0-based, unlike the one in
Collection, because a Stream's #position method returns 0-based values.
Notice that this class can only provide the illusion of random access,
by appropriately rewinding the input stream or skipping compressed data.

[]{#index-isPositionable} []{#index-skip_003a-1}

**isPositionable**

Answer true if the stream supports moving backwards with #skip:.

[]{#index-position_003a-1}

**position: anInteger**

Set the current position in the stream to anInteger. Notice that this
class can only provide the illusion of random access, by appropriately
rewinding the input stream or skipping compressed data.

[]{#index-reset}

**reset**

Reset the stream to the beginning of the compressed data.

[]{#index-skip_003a}

**skip: anInteger**

Move the current position by anInteger places, either forwards or
backwards.
