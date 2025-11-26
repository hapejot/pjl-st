[]{#VFS_002eArchiveMember_002ddirectory-operations}

::: header
Next: [VFS.ArchiveMember-file
operations](VFS_002eArchiveMember_002dfile-operations.html#VFS_002eArchiveMember_002dfile-operations){accesskey="n"
rel="next"}, Previous:
[VFS.ArchiveMember-delegation](VFS_002eArchiveMember_002ddelegation.html#VFS_002eArchiveMember_002ddelegation){accesskey="p"
rel="prev"}, Up:
[VFS.ArchiveMember](VFS_002eArchiveMember.html#VFS_002eArchiveMember){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveMember_003a-directory-operations}

#### 1.209.4 VFS.ArchiveMember: directory operations {#vfs.archivemember-directory-operations .subsection}

[]{#index-at_003a-23}

**at: aName**

Answer a FilePath for a file named 'aName' residing in the directory
represented by the receiver.

[]{#index-createDirectory_003a}

**createDirectory: dirName**

Create a subdirectory of the receiver, naming it dirName.

[]{#index-namesDo_003a-3}

**namesDo: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing its name.
