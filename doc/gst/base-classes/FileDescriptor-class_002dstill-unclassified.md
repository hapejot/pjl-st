[]{#FileDescriptor-class_002dstill-unclassified}

::: header
Next:
[FileDescriptor-accessing](FileDescriptor_002daccessing.html#FileDescriptor_002daccessing){accesskey="n"
rel="next"}, Previous: [FileDescriptor class-instance
creation](FileDescriptor-class_002dinstance-creation.html#FileDescriptor-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor-class_003a-still-unclassified}

#### 1.76.3 FileDescriptor class: still unclassified {#filedescriptor-class-still-unclassified .subsection}

[]{#index-open_003amode_003a} []{#index-append-4} []{#index-create-4}
[]{#index-readWrite-4} []{#index-read-5} []{#index-write-5}
[]{#index-close-7} []{#index-removeToBeFinalized-6}

**open: fileName mode: fileMode**

Open fileName in the required mode - answered by #append, #create,
#readWrite, #read or #write - and fail if the file cannot be opened.
Else answer a new FileStream. For mode anyway you can use any standard C
non-binary fopen mode. fileName can be a 'virtual filesystem' path,
including URLs and '#' suffixes that are inspected by the virtual
filesystem layers and replaced with tasks such as un-gzipping a file or
extracting a file from an archive.

The file will be automatically closed upon GC if the object is not
referenced anymore, but it is better to close it as soon as you're
finished with it anyway, using #close. To keep a file open even when no
references exist anymore, send it #removeToBeFinalized
