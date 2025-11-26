[]{#VFS_002eArchiveMember_002dfile-operations}

::: header
Next:
[VFS.ArchiveMember-initializing](VFS_002eArchiveMember_002dinitializing.html#VFS_002eArchiveMember_002dinitializing){accesskey="n"
rel="next"}, Previous: [VFS.ArchiveMember-directory
operations](VFS_002eArchiveMember_002ddirectory-operations.html#VFS_002eArchiveMember_002ddirectory-operations){accesskey="p"
rel="prev"}, Up:
[VFS.ArchiveMember](VFS_002eArchiveMember.html#VFS_002eArchiveMember){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveMember_003a-file-operations}

#### 1.209.5 VFS.ArchiveMember: file operations {#vfs.archivemember-file-operations .subsection}

[]{#index-open_003amode_003aifFail_003a-3}

**open: class mode: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-remove-2}

**remove**

Remove the file with the given path name

[]{#index-renameTo_003a-2}

**renameTo: newFileName**

Rename the file with the given path name oldFileName to newFileName

[]{#index-update_003a-7}

**update: aspect**

Private - Update the in-archive version of the file before closing.
