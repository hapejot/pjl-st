[]{#VFS_002eTmpFileArchiveMember_002ddirectory-operations}

::: header
Next:
[VFS.TmpFileArchiveMember-finalization](VFS_002eTmpFileArchiveMember_002dfinalization.html#VFS_002eTmpFileArchiveMember_002dfinalization){accesskey="n"
rel="next"}, Up:
[VFS.TmpFileArchiveMember](VFS_002eTmpFileArchiveMember.html#VFS_002eTmpFileArchiveMember){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eTmpFileArchiveMember_003a-directory-operations}

#### 1.212.1 VFS.TmpFileArchiveMember: directory operations {#vfs.tmpfilearchivemember-directory-operations .subsection}

[]{#index-file-3}

**file**

Answer the real file name which holds the file contents, or nil if it
does not apply.

[]{#index-open_003amode_003aifFail_003a-6}

**open: class mode: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)
