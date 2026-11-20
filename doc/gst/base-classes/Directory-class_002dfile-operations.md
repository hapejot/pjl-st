[]{#Directory-class_002dfile-operations}

::: header
Next: [Directory class-reading system
defaults](Directory-class_002dreading-system-defaults.html#Directory-class_002dreading-system-defaults){accesskey="n"
rel="next"}, Previous: [Directory class-file name
management](Directory-class_002dfile-name-management.html#Directory-class_002dfile-name-management){accesskey="p"
rel="prev"}, Up: [Directory](Directory.html#Directory){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Directory-class_003a-file-operations}

#### 1.66.2 Directory class: file operations {#directory-class-file-operations .subsection}

[]{#index-allFilesMatching_003ado_003a}
[]{#index-allFilesMatching_003ado_003a-2}

**allFilesMatching: aPattern do: aBlock**

Invoke #allFilesMatching:do: on the current working directory.

[]{#index-create_003a}

**create: dirName**

Create a directory named dirName and answer it.

[]{#index-createTemporary_003a}

**createTemporary: prefix**

Create an empty directory whose name starts with prefix and answer it.

[]{#index-working}

**working**

Answer the current working directory, not following symlinks.

[]{#index-working_003a}

**working: dirName**

Change the current working directory to dirName.
