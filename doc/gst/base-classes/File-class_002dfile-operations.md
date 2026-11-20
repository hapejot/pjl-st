[]{#File-class_002dfile-operations}

::: header
Next: [File
class-initialization](File-class_002dinitialization.html#File-class_002dinitialization){accesskey="n"
rel="next"}, Previous: [File class-C
functions](File-class_002dC-functions.html#File-class_002dC-functions){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#File-class_003a-file-operations}

#### 1.75.2 File class: file operations {#file-class-file-operations .subsection}

[]{#index-checkError}

**checkError**

Return whether an error had been reported or not. If there had been one,
raise an exception too

[]{#index-checkError_003a}

**checkError: errno**

The error with the C code 'errno' has been reported. If errno \>= 1,
raise an exception

[]{#index-remove_003a-2}

**remove: fileName**

Remove the file with the given path name

[]{#index-rename_003ato_003a}

**rename: oldFileName to: newFileName**

Rename the file with the given path name oldFileName to newFileName

[]{#index-symlink_003aas_003a}

**symlink: srcName as: destName**

Create a symlink for the srcName file with the given path name

[]{#index-symlink_003afrom_003a}

**symlink: destName from: srcName**

Create a symlink named destName file from the given path (relative to
destName)

[]{#index-touch_003a}

**touch: fileName**

Update the timestamp of the file with the given path name.
