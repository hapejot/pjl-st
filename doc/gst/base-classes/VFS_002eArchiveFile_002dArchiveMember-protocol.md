[]{#VFS_002eArchiveFile_002dArchiveMember-protocol}

::: header
Next: [VFS.ArchiveFile-directory
operations](VFS_002eArchiveFile_002ddirectory-operations.html#VFS_002eArchiveFile_002ddirectory-operations){accesskey="n"
rel="next"}, Up:
[VFS.ArchiveFile](VFS_002eArchiveFile.html#VFS_002eArchiveFile){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveFile_003a-ArchiveMember-protocol}

#### 1.208.1 VFS.ArchiveFile: ArchiveMember protocol {#vfs.archivefile-archivemember-protocol .subsection}

[]{#index-fillMember_003a}
[]{#index-size_003astCtime_003astMtime_003astAtime_003aisDirectory_003a}

**fillMember: anArchiveMember**

Extract the information on anArchiveMember. Answer false if it actually
does not exist in the archive; otherwise, answer true after having told
anArchiveMember about them by sending
#size:stCtime:stMtime:stAtime:isDirectory: to it.

[]{#index-member_003ado_003a}

**member: anArchiveMember do: aBlock**

Evaluate aBlock once for each file in the directory represented by
anArchiveMember, passing its name.

[]{#index-member_003amode_003a}

**member: anArchiveMember mode: bits**

Set the permission bits for the file in anArchiveMember.

[]{#index-refresh-3}

**refresh**

Extract the directory listing from the archive

[]{#index-removeMember_003a}

**removeMember: anArchiveMember**

Remove the member represented by anArchiveMember.

[]{#index-updateMember_003a}

**updateMember: anArchiveMember**

Update the member represented by anArchiveMember by copying the file
into which it was extracted back to the archive.
