[]{#VFS_002eFileWrapper_002ddelegation}

::: header
Next:
[VFS.FileWrapper-enumerating](VFS_002eFileWrapper_002denumerating.html#VFS_002eFileWrapper_002denumerating){accesskey="n"
rel="next"}, Previous:
[VFS.FileWrapper-basic](VFS_002eFileWrapper_002dbasic.html#VFS_002eFileWrapper_002dbasic){accesskey="p"
rel="prev"}, Up:
[VFS.FileWrapper](VFS_002eFileWrapper.html#VFS_002eFileWrapper){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eFileWrapper_003a-delegation}

#### 1.210.5 VFS.FileWrapper: delegation {#vfs.filewrapper-delegation .subsection}

[]{#index-creationTime-3}

**creationTime**

Answer the creation time of the file identified by the receiver. On some
operating systems, this could actually be the last change time (the
'last change time' has to do with permissions, ownership and the like).

[]{#index-full-3}

**full**

Answer the size of the file identified by the receiver

[]{#index-isExecutable-3}

**isExecutable**

Answer whether a file with the name contained in the receiver does exist
and is executable

[]{#index-isReadable-3}

**isReadable**

Answer whether a file with the name contained in the receiver does exist
and is readable

[]{#index-isWriteable-3}

**isWriteable**

Answer whether a file with the name contained in the receiver does exist
and is writeable

[]{#index-lastAccessTime-3}

**lastAccessTime**

Answer the last access time of the file identified by the receiver

[]{#index-lastChangeTime-3}

**lastChangeTime**

Answer the last change time of the file identified by the receiver (the
'last change time' has to do with permissions, ownership and the like).
On some operating systems, this could actually be the file creation
time.

[]{#index-lastModifyTime-3}

**lastModifyTime**

Answer the last modify time of the file identified by the receiver (the
'last modify time' has to do with the actual file contents).

[]{#index-mode-3}

**mode**

Answer the permission bits for the file identified by the receiver

[]{#index-mode_003a-3}

**mode: anInteger**

Answer the permission bits for the file identified by the receiver

[]{#index-open_003amode_003aifFail_003a-4}

**open: class mode: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-remove-3}

**remove**

Remove the file with the given path name

[]{#index-size-26}

**size**

Answer the size of the file identified by the receiver

------------------------------------------------------------------------

::: header
Next:
[VFS.FileWrapper-enumerating](VFS_002eFileWrapper_002denumerating.html#VFS_002eFileWrapper_002denumerating){accesskey="n"
rel="next"}, Previous:
[VFS.FileWrapper-basic](VFS_002eFileWrapper_002dbasic.html#VFS_002eFileWrapper_002dbasic){accesskey="p"
rel="prev"}, Up:
[VFS.FileWrapper](VFS_002eFileWrapper.html#VFS_002eFileWrapper){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
