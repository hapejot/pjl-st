[]{#FilePath_002ddecoration}

::: header
Next: [FilePath-directory
operations](FilePath_002ddirectory-operations.html#FilePath_002ddirectory-operations){accesskey="n"
rel="next"}, Previous:
[FilePath-converting](FilePath_002dconverting.html#FilePath_002dconverting){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath_003a-decoration}

#### 1.77.5 FilePath: decoration {#filepath-decoration .subsection}

[]{#index-all}

**all**

Return a decorator of the receiver that will provide recursive descent
into directories for iteration methods. Furthermore, iteration on the
returned wrapper will not include '.' or '..' directory entries, and
will include the receiver (directly, not via '.').
