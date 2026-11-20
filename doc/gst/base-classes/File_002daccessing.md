[]{#File_002daccessing}

::: header
Next: [File-basic](File_002dbasic.html#File_002dbasic){accesskey="n"
rel="next"}, Previous: [File
class-testing](File-class_002dtesting.html#File-class_002dtesting){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#File_003a-accessing}

#### 1.75.7 File: accessing {#file-accessing .subsection}

[]{#index-asString-5}

**asString**

Answer the name of the file identified by the receiver

[]{#index-at_003a-4}

**at: aString**

Answer a File or Directory object as appropriate for a file named
'aName' in the directory represented by the receiver.

[]{#index-creationTime}

**creationTime**

Answer the creation time of the file identified by the receiver. On some
operating systems, this could actually be the last change time (the
'last change time' has to do with permissions, ownership and the like).

[]{#index-isDirectory}

**isDirectory**

Answer whether the file is a directory.

[]{#index-isSocket}

**isSocket**

Answer whether the file is an AF_UNIX socket.

[]{#index-isSymbolicLink}

**isSymbolicLink**

Answer whether the file is a symbolic link.

[]{#index-lastAccessTime}

**lastAccessTime**

Answer the last access time of the file identified by the receiver

[]{#index-lastChangeTime}

**lastChangeTime**

Answer the last change time of the file identified by the receiver (the
'last change time' has to do with permissions, ownership and the like).
On some operating systems, this could actually be the file creation
time.

[]{#index-lastModifyTime}

**lastModifyTime**

Answer the last modify time of the file identified by the receiver (the
'last modify time' has to do with the actual file contents).

[]{#index-mode}

**mode**

Answer the permission bits for the file identified by the receiver

[]{#index-mode_003a}

**mode: anInteger**

Set the permission bits for the file identified by the receiver to be
anInteger.

[]{#index-name-5}

**name**

Answer the name of the file identified by the receiver

[]{#index-pathTo_003a}

**pathTo: destName**

Compute the relative path from the receiver to destName.

[]{#index-refresh}

**refresh**

Refresh the statistics for the receiver

[]{#index-size-4}

**size**

Answer the size of the file identified by the receiver

------------------------------------------------------------------------

::: header
Next: [File-basic](File_002dbasic.html#File_002dbasic){accesskey="n"
rel="next"}, Previous: [File
class-testing](File-class_002dtesting.html#File-class_002dtesting){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
