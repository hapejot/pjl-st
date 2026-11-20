[]{#VFS_002eFileWrapper_002daccessing}

::: header
Next:
[VFS.FileWrapper-basic](VFS_002eFileWrapper_002dbasic.html#VFS_002eFileWrapper_002dbasic){accesskey="n"
rel="next"}, Previous: [VFS.FileWrapper class-instance
creation](VFS_002eFileWrapper-class_002dinstance-creation.html#VFS_002eFileWrapper-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[VFS.FileWrapper](VFS_002eFileWrapper.html#VFS_002eFileWrapper){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eFileWrapper_003a-accessing}

#### 1.210.3 VFS.FileWrapper: accessing {#vfs.filewrapper-accessing .subsection}

[]{#index-asString-15}

**asString**

Answer the string representation of the receiver's path.

[]{#index-at_003a-24}

**at: aName**

Answer a File or Directory object as appropriate for a file named
'aName' in the directory represented by the receiver.

[]{#index-lastAccessTime_003alastModifyTime_003a-2}

**lastAccessTime: accessDateTime lastModifyTime: modifyDateTime**

Update the timestamps of the file corresponding to the receiver, to be
accessDateTime and modifyDateTime.

[]{#index-name-16}

**name**

Answer the full path to the receiver.

[]{#index-owner_003agroup_003a-2}

**owner: ownerString group: groupString**

Set the receiver's owner and group to be ownerString and groupString.

[]{#index-pathTo_003a-2}

**pathTo: destName**

Compute the relative path from the receiver to destName.
