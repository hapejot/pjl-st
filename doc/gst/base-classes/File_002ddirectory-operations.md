[]{#File_002ddirectory-operations}

::: header
Next: [File-file name
management](File_002dfile-name-management.html#File_002dfile-name-management){accesskey="n"
rel="next"}, Previous:
[File-basic](File_002dbasic.html#File_002dbasic){accesskey="p"
rel="prev"}, Up: [File](File.html#File){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#File_003a-directory-operations}

#### 1.75.9 File: directory operations {#file-directory-operations .subsection}

[]{#index-createDirectory}

**createDirectory**

Create the receiver as a directory.

[]{#index-namesDo_003a}

**namesDo: aBlock**

Evaluate aBlock once for each file in the directory represented by the
receiver, passing its name. aBlock should not return.
