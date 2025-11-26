[]{#VFS_002eZipFile_002dmembers}

::: header
Up: [VFS.ZipFile](VFS_002eZipFile.html#VFS_002eZipFile){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VFS_002eZipFile_003a-members}

#### 1.213.1 VFS.ZipFile: members {#vfs.zipfile-members .subsection}

[]{#index-centralDirectoryRangeIn_003a}

**centralDirectoryRangeIn: f**

Not commented.

[]{#index-createDirectory_003a-1}

**createDirectory: dirName**

Create a subdirectory of the receiver, naming it dirName.

[]{#index-extractMember_003ainto_003a-1}

**extractMember: anArchiveMember into: temp**

Extract the contents of anArchiveMember into a file that resides on
disk, and answer the name of the file.

[]{#index-fileData}

**fileData**

Extract the directory listing from the archive

[]{#index-member_003amode_003a-1}

**member: anArchiveMember mode: bits**

Set the permission bits for the file in anArchiveMember.

[]{#index-removeMember_003a-1}

**removeMember: anArchiveMember**

Remove the member represented by anArchiveMember.

[]{#index-updateMember_003a-1}

**updateMember: anArchiveMember**

Update the member represented by anArchiveMember by copying the file
into which it was extracted back to the archive.
