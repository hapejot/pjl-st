[]{#VFS_002eArchiveMember_002daccessing}

::: header
Next:
[VFS.ArchiveMember-basic](VFS_002eArchiveMember_002dbasic.html#VFS_002eArchiveMember_002dbasic){accesskey="n"
rel="next"}, Up:
[VFS.ArchiveMember](VFS_002eArchiveMember.html#VFS_002eArchiveMember){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eArchiveMember_003a-accessing}

#### 1.209.1 VFS.ArchiveMember: accessing {#vfs.archivemember-accessing .subsection}

[]{#index-archive}

**archive**

Answer the archive of which the receiver is a member.

[]{#index-asString-14} []{#index-name-18}

**asString**

Answer the name of the file identified by the receiver as answered by
File\>\>#name.

[]{#index-creationTime-2}

**creationTime**

Answer the creation time of the file identified by the receiver. On some
operating systems, this could actually be the last change time (the
'last change time' has to do with permissions, ownership and the like).

[]{#index-lastAccessTime-2}

**lastAccessTime**

Answer the last access time of the file identified by the receiver

[]{#index-lastChangeTime-2}

**lastChangeTime**

Answer the last change time of the file identified by the receiver (the
'last change time' has to do with permissions, ownership and the like).
On some operating systems, this could actually be the file creation
time.

[]{#index-lastModifyTime-2}

**lastModifyTime**

Answer the last modify time of the file identified by the receiver (the
'last modify time' has to do with the actual file contents).

[]{#index-name-15}

**name**

Answer the receiver's file name.

[]{#index-name_003a-7}

**name: aName**

Set the receiver's file name to aName.

[]{#index-refresh-4}

**refresh**

Refresh the statistics for the receiver

[]{#index-size-25}

**size**

Answer the size of the file identified by the receiver
