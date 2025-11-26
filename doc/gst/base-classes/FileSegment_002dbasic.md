[]{#FileSegment_002dbasic}

::: header
Next:
[FileSegment-equality](FileSegment_002dequality.html#FileSegment_002dequality){accesskey="n"
rel="next"}, Previous: [FileSegment
class-installing](FileSegment-class_002dinstalling.html#FileSegment-class_002dinstalling){accesskey="p"
rel="prev"}, Up:
[FileSegment](FileSegment.html#FileSegment){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileSegment_003a-basic}

#### 1.78.3 FileSegment: basic {#filesegment-basic .subsection}

[]{#index-asString-7}

**asString**

Answer a String containing the required segment of the file

[]{#index-copyFrom_003ato_003a-2}

**copyFrom: from to: to**

Answer a String containing the given subsegment of the file. As for
streams, from and to are 0-based.

[]{#index-file-1}

**file**

Answer the File object for the file containing the segment

[]{#index-fileName-1}

**fileName**

Answer the name of the file containing the segment

[]{#index-filePos-1}

**filePos**

Answer the position in the file where the segment starts

[]{#index-relocateFrom_003amap_003a}

**relocateFrom: startPath map: map**

If the path starts with startPath, remove that part of the path. map is
a Dictionary that is used so that equal filenames stay equal, without
increasing the amount of memory that the image uses.

[]{#index-size-7}

**size**

Answer the length of the segment

[]{#index-withFileDo_003a}

**withFileDo: aBlock**

Evaluate aBlock passing it the FileStream in which the segment
identified by the receiver is stored
