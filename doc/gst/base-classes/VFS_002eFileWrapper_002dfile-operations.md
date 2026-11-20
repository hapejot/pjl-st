[]{#VFS_002eFileWrapper_002dfile-operations}

::: header
Next:
[VFS.FileWrapper-testing](VFS_002eFileWrapper_002dtesting.html#VFS_002eFileWrapper_002dtesting){accesskey="n"
rel="next"}, Previous:
[VFS.FileWrapper-enumerating](VFS_002eFileWrapper_002denumerating.html#VFS_002eFileWrapper_002denumerating){accesskey="p"
rel="prev"}, Up:
[VFS.FileWrapper](VFS_002eFileWrapper.html#VFS_002eFileWrapper){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eFileWrapper_003a-file-operations}

#### 1.210.7 VFS.FileWrapper: file operations {#vfs.filewrapper-file-operations .subsection}

[]{#index-pathFrom_003a-2}

**pathFrom: dirName**

Compute the relative path from the directory dirName to the receiver

[]{#index-renameTo_003a-3}

**renameTo: newName**

Rename the file identified by the receiver to newName

[]{#index-symlinkAs_003a-2}

**symlinkAs: destName**

Create destName as a symbolic link of the receiver. The appropriate
relative path is computed automatically.

[]{#index-symlinkFrom_003a-2}

**symlinkFrom: srcName**

Create the receiver as a symbolic link from srcName (relative to the
path of the receiver).
