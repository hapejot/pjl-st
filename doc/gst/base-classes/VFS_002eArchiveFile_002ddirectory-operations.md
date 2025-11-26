[]{#VFS_002eArchiveFile_002ddirectory-operations}

::: header
Next:
[VFS.ArchiveFile-querying](VFS_002eArchiveFile_002dquerying.html#VFS_002eArchiveFile_002dquerying){accesskey="n"
rel="next"}, Previous: [VFS.ArchiveFile-ArchiveMember
protocol](VFS_002eArchiveFile_002dArchiveMember-protocol.html#VFS_002eArchiveFile_002dArchiveMember-protocol){accesskey="p"
rel="prev"}, Up:
[VFS.ArchiveFile](VFS_002eArchiveFile.html#VFS_002eArchiveFile){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveFile_003a-directory-operations}

#### 1.208.2 VFS.ArchiveFile: directory operations {#vfs.archivefile-directory-operations .subsection}

[]{#index-at_003a-22}

**at: aName**

Answer a FilePath for a file named 'aName' residing in the directory
represented by the receiver.

[]{#index-nameAt_003a-1}

**nameAt: aString**

Answer a FilePath for a file named 'aName' residing in the directory
represented by the receiver.

[]{#index-namesDo_003a-2}

**namesDo: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing its name.

[]{#index-release-2}

**release**

Release the resources used by the receiver that don't survive when
reloading a snapshot.
