[]{#FileSegment-class_002dbasic}

::: header
Next: [FileSegment
class-installing](FileSegment-class_002dinstalling.html#FileSegment-class_002dinstalling){accesskey="n"
rel="next"}, Up:
[FileSegment](FileSegment.html#FileSegment){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileSegment-class_003a-basic}

#### 1.78.1 FileSegment class: basic {#filesegment-class-basic .subsection}

[]{#index-on_003astartingAt_003afor_003a}

**on: aFile startingAt: startPos for: sizeInteger**

Create a new FileSegment referring to the contents of the given file,
from the startPos-th byte and for sizeInteger bytes. Note that
FileSegments should always be created with full paths because relative
paths are interpreted to be relative to the kernel directory.
