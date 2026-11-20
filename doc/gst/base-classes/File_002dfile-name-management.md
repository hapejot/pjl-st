[]{#File_002dfile-name-management}

::: header
Next: [File-file
operations](File_002dfile-operations.html#File_002dfile-operations){accesskey="n"
rel="next"}, Previous: [File-directory
operations](File_002ddirectory-operations.html#File_002ddirectory-operations){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#File_003a-file-name-management}

#### 1.75.10 File: file name management {#file-file-name-management .subsection}

[]{#index-full}

**full**

Answer the full name of the receiver, resolving the '.' and '..'
directory entries, and answer the result. Answer nil if the name is
invalid (such as '/usr/../../badname')
