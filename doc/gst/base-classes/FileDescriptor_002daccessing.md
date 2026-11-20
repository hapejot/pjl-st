[]{#FileDescriptor_002daccessing}

::: header
Next:
[FileDescriptor-basic](FileDescriptor_002dbasic.html#FileDescriptor_002dbasic){accesskey="n"
rel="next"}, Previous: [FileDescriptor class-still
unclassified](FileDescriptor-class_002dstill-unclassified.html#FileDescriptor-class_002dstill-unclassified){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor_003a-accessing}

#### 1.76.4 FileDescriptor: accessing {#filedescriptor-accessing .subsection}

[]{#index-canRead}

**canRead**

Answer whether the file is open and we can read from it

[]{#index-canWrite}

**canWrite**

Answer whether the file is open and we can write from it

[]{#index-ensureReadable}

**ensureReadable**

If the file is open, wait until data can be read from it. The wait
allows other Processes to run.

[]{#index-ensureWriteable}

**ensureWriteable**

If the file is open, wait until we can write to it. The wait allows
other Processes to run.

[]{#index-exceptionalCondition}

**exceptionalCondition**

Answer whether the file is open and an exceptional condition (such as
presence of out of band data) has occurred on it

[]{#index-fd}

**fd**

Return the OS file descriptor of the file

[]{#index-file}

**file**

Return the name of the file

[]{#index-isOpen}

**isOpen**

Answer whether the file is still open

[]{#index-isPeerAlive}

**isPeerAlive**

Present for compatibility with sockets. For files, it answers whether
the file is still open

[]{#index-isPipe}

**isPipe**

Answer whether the file is a pipe or an actual disk file

[]{#index-name-6}

**name**

Return the name of the file

[]{#index-waitForException}

**waitForException**

If the file is open, wait until an exceptional condition (such as
presence of out of band data) has occurred on it. The wait allows other
Processes to run.
