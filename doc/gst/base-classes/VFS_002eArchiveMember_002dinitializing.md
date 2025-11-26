[]{#VFS_002eArchiveMember_002dinitializing}

::: header
Next: [VFS.ArchiveMember-still
unclassified](VFS_002eArchiveMember_002dstill-unclassified.html#VFS_002eArchiveMember_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [VFS.ArchiveMember-file
operations](VFS_002eArchiveMember_002dfile-operations.html#VFS_002eArchiveMember_002dfile-operations){accesskey="p"
rel="prev"}, Up:
[VFS.ArchiveMember](VFS_002eArchiveMember.html#VFS_002eArchiveMember){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveMember_003a-initializing}

#### 1.209.6 VFS.ArchiveMember: initializing {#vfs.archivemember-initializing .subsection}

[]{#index-archive_003a}

**archive: anArchiveFile**

Set the archive of which the receiver is a member.

[]{#index-fillFrom_003a}

**fillFrom: data**

Called back by the receiver's archive when the ArchiveMember asks for
file information.

[]{#index-size_003astCtime_003astMtime_003astAtime_003amode_003a}

**size: bytes stCtime: ctime stMtime: mtime stAtime: atime mode:
modeBits**

Set the file information for the receiver.

[]{#index-size_003astMtime_003amode_003a}

**size: bytes stMtime: mtime mode: modeBits**

Set the file information for the receiver.
