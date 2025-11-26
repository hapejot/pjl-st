[]{#VFS_002eArchiveFile}

::: header
Next:
[VFS.ArchiveMember](VFS_002eArchiveMember.html#VFS_002eArchiveMember){accesskey="n"
rel="next"}, Previous:
[VersionableObjectProxy](VersionableObjectProxy.html#VersionableObjectProxy){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveFile-1}

### 1.208 VFS.ArchiveFile {#vfs.archivefile .section}

[]{#index-VFS_002eArchiveFile}

**Defined in namespace Smalltalk.VFS**\
**Superclass: VFS.FileWrapper**\
**Category: Streams-Files**

:   ArchiveFile handles virtual filesystems that have a directory
    structure of their own. The directories and files in the archive are
    instances of ArchiveMember, but the functionality resides entirely
    in ArchiveFile because the members will still ask the archive to get
    directory information on them, to extract them to a real file, and
    so on.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [VFS.ArchiveFile-ArchiveMember protocol](VFS_002eArchiveFile_002dArchiveMember-protocol.html#VFS_002eArchiveFile_002dArchiveMember-protocol){accesskey="1"}:                             (instance)
  • [VFS.ArchiveFile-directory operations](VFS_002eArchiveFile_002ddirectory-operations.html#VFS_002eArchiveFile_002ddirectory-operations){accesskey="2"}:                                   (instance)
  • [VFS.ArchiveFile-querying](VFS_002eArchiveFile_002dquerying.html#VFS_002eArchiveFile_002dquerying){accesskey="3"}:                                                                       (instance)
  • [VFS.ArchiveFile-still unclassified](VFS_002eArchiveFile_002dstill-unclassified.html#VFS_002eArchiveFile_002dstill-unclassified){accesskey="4"}:                                         (instance)
  • [VFS.ArchiveFile-TmpFileArchiveMember protocol](VFS_002eArchiveFile_002dTmpFileArchiveMember-protocol.html#VFS_002eArchiveFile_002dTmpFileArchiveMember-protocol){accesskey="5"}:        (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
