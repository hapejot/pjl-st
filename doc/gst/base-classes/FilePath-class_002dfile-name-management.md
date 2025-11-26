[]{#FilePath-class_002dfile-name-management}

::: header
Next: [FilePath class-still
unclassified](FilePath-class_002dstill-unclassified.html#FilePath-class_002dstill-unclassified){accesskey="n"
rel="next"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath-class_003a-file-name-management}

#### 1.77.1 FilePath class: file name management {#filepath-class-file-name-management .subsection}

[]{#index-append_003ato_003a-1}

**append: fileName to: directory**

Answer the name of a file named 'fileName' which resides in a directory
named 'directory'.

[]{#index-extensionFor_003a}

**extensionFor: aString**

Answer the extension of a file named 'aString'. Note: the extension
includes an initial dot.

[]{#index-fullNameFor_003a}

**fullNameFor: aString**

Answer the full path to a file called 'aString', resolving the '.' and
'..' directory entries, and answer the result. '/..' is the same as '/'.

[]{#index-pathFor_003a}

**pathFor: aString**

Determine the path of the name of a file called 'aString', and answer
the result. With the exception of the root directory, the final slash is
stripped.

[]{#index-pathFor_003aifNone_003a}

**pathFor: aString ifNone: aBlock**

Determine the path of the name of a file called 'aString', and answer
the result. With the exception of the root directory, the final slash is
stripped. If there is no path, evaluate aBlock and return the result.

[]{#index-pathFrom_003ato_003a}

**pathFrom: srcName to: destName**

Answer the relative path to destName when the current directory is
srcName's directory.

[]{#index-stripExtensionFrom_003a}

**stripExtensionFrom: aString**

Remove the extension from the name of a file called 'aString', and
answer the result.

[]{#index-stripFileNameFor_003a}

**stripFileNameFor: aString**

Determine the path of the name of a file called 'aString', and answer
the result as a directory name including the final slash.

[]{#index-stripPathFrom_003a}

**stripPathFrom: aString**

Remove the path from the name of a file called 'aString', and answer the
file name plus extension.
