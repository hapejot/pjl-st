[]{#WriteStream-class_002dinstance-creation}

::: header
Next:
[WriteStream-accessing-writing](WriteStream_002daccessing_002dwriting.html#WriteStream_002daccessing_002dwriting){accesskey="n"
rel="next"}, Up:
[WriteStream](WriteStream.html#WriteStream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WriteStream-class_003a-instance-creation}

#### 1.223.1 WriteStream class: instance creation {#writestream-class-instance-creation .subsection}

[]{#index-on_003a-12}

**on: aCollection**

Answer a new instance of the receiver which streams on aCollection.
Every item of aCollection is discarded.

[]{#index-with_003a-7}

**with: aCollection**

Answer a new instance of the receiver which streams from the end of
aCollection.

[]{#index-with_003afrom_003ato_003a}

**with: aCollection from: firstIndex to: lastIndex**

Answer a new instance of the receiver which streams from the
firstIndex-th item of aCollection to the lastIndex-th. The pointer is
moved to the last item in that range.
