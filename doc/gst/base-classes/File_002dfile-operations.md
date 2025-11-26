[]{#File_002dfile-operations}

::: header
Next: [File-still
unclassified](File_002dstill-unclassified.html#File_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [File-file name
management](File_002dfile-name-management.html#File_002dfile-name-management){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#File_003a-file-operations}

#### 1.75.11 File: file operations {#file-file-operations .subsection}

[]{#index-lastAccessTime_003alastModifyTime_003a}

**lastAccessTime: accessDateTime lastModifyTime: modifyDateTime**

Set the receiver's timestamps to be accessDateTime and modifyDateTime.

[]{#index-open_003amode_003aifFail_003a}

**open: class mode: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-owner_003agroup_003a}

**owner: ownerString group: groupString**

Set the receiver's owner and group to be ownerString and groupString.

[]{#index-pathFrom_003a}

**pathFrom: dir**

Compute the relative path from the directory dirName to the receiver

[]{#index-remove}

**remove**

Remove the file with the given path name

[]{#index-renameTo_003a}

**renameTo: newFileName**

Rename the file with the given path name to newFileName

[]{#index-symlinkAs_003a}

**symlinkAs: destName**

Create destName as a symbolic link of the receiver. The appropriate
relative path is computed automatically.

[]{#index-symlinkFrom_003a}

**symlinkFrom: srcName**

Create the receiver as a symlink from path destName
