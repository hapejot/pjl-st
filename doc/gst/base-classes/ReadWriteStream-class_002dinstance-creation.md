[]{#ReadWriteStream-class_002dinstance-creation}

::: header
Next:
[ReadWriteStream-positioning](ReadWriteStream_002dpositioning.html#ReadWriteStream_002dpositioning){accesskey="n"
rel="next"}, Up:
[ReadWriteStream](ReadWriteStream.html#ReadWriteStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ReadWriteStream-class_003a-instance-creation}

#### 1.141.1 ReadWriteStream class: instance creation {#readwritestream-class-instance-creation .subsection}

[]{#index-on_003a-9}

**on: aCollection**

Answer a new stream working on aCollection from its start. The stream
starts at the front of aCollection.

[]{#index-on_003afrom_003ato_003a-2}

**on: aCollection from: firstIndex to: lastIndex**

Answer an instance of the receiver streaming from the firstIndex-th item
of aCollection to the lastIndex-th

[]{#index-with_003a-2}

**with: aCollection**

Answer a new instance of the receiver which streams from the end of
aCollection.
