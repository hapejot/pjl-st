[]{#FilePath_002dfile-name-management}

::: header
Next: [FilePath-file
operations](FilePath_002dfile-operations.html#FilePath_002dfile-operations){accesskey="n"
rel="next"}, Previous:
[FilePath-enumerating](FilePath_002denumerating.html#FilePath_002denumerating){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath_003a-file-name-management}

#### 1.77.8 FilePath: file name management {#filepath-file-name-management .subsection}

[]{#index-directory}

**directory**

Answer the Directory object for the receiver's path

[]{#index-extension}

**extension**

Answer the extension of the receiver

[]{#index-full-1}

**full**

Answer the full name of the receiver, resolving the '.' and '..'
directory entries, and answer the result. Answer nil if the name is
invalid (such as '/usr/../../badname')

[]{#index-fullName} []{#index-name-17}

**fullName**

Answer a String with the full path to the receiver (same as #name; it is
useless to override this method).

[]{#index-name-7} []{#index-fullName-1}

**name**

Answer String with the full path to the receiver (same as #fullName).

[]{#index-parent}

**parent**

Answer the Directory object for the receiver's path

[]{#index-path-1}

**path**

Answer the path (if any) of the receiver

[]{#index-stripExtension}

**stripExtension**

Answer the path (if any) and file name of the receiver

[]{#index-stripFileName}

**stripFileName**

Answer the path of the receiver, always including a directory name
(possibly '.') and the final directory separator

[]{#index-stripPath}

**stripPath**

Answer the file name and extension (if any) of the receiver
