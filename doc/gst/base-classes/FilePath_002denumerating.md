[]{#FilePath_002denumerating}

::: header
Next: [FilePath-file name
management](FilePath_002dfile-name-management.html#FilePath_002dfile-name-management){accesskey="n"
rel="next"}, Previous: [FilePath-directory
operations](FilePath_002ddirectory-operations.html#FilePath_002ddirectory-operations){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath_003a-enumerating}

#### 1.77.7 FilePath: enumerating {#filepath-enumerating .subsection}

[]{#index-allFilesMatching_003ado_003a-1} []{#index-match_003a-1}

**allFilesMatching: aPattern do: aBlock**

Evaluate aBlock on the File objects that match aPattern (according to
String\>\>#match:) in the directory named by the receiver. Recursively
descend into directories.

[]{#index-directories}

**directories**

Answer an Array with Directory objects for the subdirectories of the
directory represented by the receiver.

[]{#index-do_003a-2}

**do: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing a FilePath object (or a subclass) to it. It depends on
the subclass whether iteration will include the '.' and '..' directory
entries.

[]{#index-entries}

**entries**

Answer an Array with File or Directory objects for the contents of the
directory represented by the receiver.

[]{#index-entryNames}

**entryNames**

Answer an Array with the names of the files in the directory represented
by the receiver.

[]{#index-files}

**files**

Answer an Array with File objects for the contents of the directory
represented by the receiver.

[]{#index-filesMatching_003a}

**filesMatching: aPattern**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing a File or Directory object to aBlock. Returns the
\*names\* of the files for which aBlock returns true.

[]{#index-filesMatching_003ado_003a} []{#index-match_003a-2}

**filesMatching: aPattern do: block**

Evaluate block on the File objects that match aPattern (according to
String\>\>#match:) in the directory named by the receiver.

[]{#index-namesDo_003a-1}

**namesDo: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing its name. It depends on the subclass whether iteration
will include the '.' and '..' directory entries.

[]{#index-namesMatching_003ado_003a} []{#index-match_003a-3}

**namesMatching: aPattern do: block**

Evaluate block on the file names that match aPattern (according to
String\>\>#match:) in the directory named by the receiver.

[]{#index-reject_003a-3}

**reject: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing a File or Directory object to aBlock. Returns the
\*names\* of the files for which aBlock returns true.

[]{#index-select_003a-3}

**select: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing a File or Directory object to aBlock. Returns the
\*names\* of the files for which aBlock returns true.

------------------------------------------------------------------------

::: header
Next: [FilePath-file name
management](FilePath_002dfile-name-management.html#FilePath_002dfile-name-management){accesskey="n"
rel="next"}, Previous: [FilePath-directory
operations](FilePath_002ddirectory-operations.html#FilePath_002ddirectory-operations){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
