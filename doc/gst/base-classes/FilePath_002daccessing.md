[]{#FilePath_002daccessing}

::: header
Next:
[FilePath-converting](FilePath_002dconverting.html#FilePath_002dconverting){accesskey="n"
rel="next"}, Previous: [FilePath class-still
unclassified](FilePath-class_002dstill-unclassified.html#FilePath-class_002dstill-unclassified){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath_003a-accessing}

#### 1.77.3 FilePath: accessing {#filepath-accessing .subsection}

[]{#index-at_003a-5}

**at: aName**

Answer a File or Directory object as appropriate for a file named
'aName' in the directory represented by the receiver.

[]{#index-creationTime-1}

**creationTime**

Answer the creation time of the file identified by the receiver. On some
operating systems, this could actually be the last change time (the
'last change time' has to do with permissions, ownership and the like).

[]{#index-group_003a}

**group: aString**

Set the group of the file identified by the receiver to be aString.

[]{#index-includes_003a-3}

**includes: aName**

Answer whether a file named 'aName' exists in the directory represented
by the receiver.

[]{#index-lastAccessTime-1}

**lastAccessTime**

Answer the last access time of the file identified by the receiver

[]{#index-lastAccessTime_003a}

**lastAccessTime: aDateTime**

Update the last access time of the file corresponding to the receiver,
to be aDateTime.

[]{#index-lastAccessTime_003alastModifyTime_003a-1}

**lastAccessTime: accessDateTime lastModifyTime: modifyDateTime**

Update the timestamps of the file corresponding to the receiver, to be
accessDateTime and modifyDateTime.

[]{#index-lastChangeTime-1}

**lastChangeTime**

Answer the last change time of the file identified by the receiver (the
'last change time' has to do with permissions, ownership and the like).
On some operating systems, this could actually be the file creation
time.

[]{#index-lastModifyTime-1}

**lastModifyTime**

Answer the last modify time of the file identified by the receiver (the
'last modify time' has to do with the actual file contents).

[]{#index-lastModifyTime_003a}

**lastModifyTime: aDateTime**

Update the last modification timestamp of the file corresponding to the
receiver, to be aDateTime.

[]{#index-mode-1}

**mode**

Answer the permission bits for the file identified by the receiver

[]{#index-mode_003a-1}

**mode: anInteger**

Set the permission bits for the file identified by the receiver to be
anInteger.

[]{#index-owner_003a}

**owner: aString**

Set the owner of the file identified by the receiver to be aString.

[]{#index-owner_003agroup_003a-1}

**owner: ownerString group: groupString**

Set the owner and group of the file identified by the receiver to be
aString.

[]{#index-pathTo_003a-1}

**pathTo: destName**

Compute the relative path from the receiver to destName.

[]{#index-refresh-1}

**refresh**

Refresh the statistics for the receiver

[]{#index-size-6}

**size**

Answer the size of the file identified by the receiver

------------------------------------------------------------------------

::: header
Next:
[FilePath-converting](FilePath_002dconverting.html#FilePath_002dconverting){accesskey="n"
rel="next"}, Previous: [FilePath class-still
unclassified](FilePath-class_002dstill-unclassified.html#FilePath-class_002dstill-unclassified){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
