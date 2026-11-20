[]{#FileDescriptor-class_002dinstance-creation}

::: header
Next: [FileDescriptor class-still
unclassified](FileDescriptor-class_002dstill-unclassified.html#FileDescriptor-class_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [FileDescriptor
class-initialization](FileDescriptor-class_002dinitialization.html#FileDescriptor-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor-class_003a-instance-creation}

#### 1.76.2 FileDescriptor class: instance creation {#filedescriptor-class-instance-creation .subsection}

[]{#index-append}

**append**

Open for writing. The file is created if it does not exist. The stream
is positioned at the end of the file.

[]{#index-create}

**create**

Open for reading and writing. The file is created if it does not exist,
otherwise it is truncated. The stream is positioned at the beginning of
the file.

[]{#index-fopen_003amode_003a} []{#index-append-1} []{#index-create-1}
[]{#index-readWrite-1} []{#index-read-1} []{#index-write-1}
[]{#index-close-3} []{#index-removeToBeFinalized-2}

**fopen: fileName mode: fileMode**

Open fileName in the required mode - answered by #append, #create,
#readWrite, #read or #write - and fail if the file cannot be opened.
Else answer a new FileStream. For mode anyway you can use any standard C
non-binary fopen mode. The file will be automatically closed upon GC if
the object is not referenced anymore, but it is better to close it as
soon as you're finished with it anyway, using #close. To keep a file
open even when no references exist anymore, send it #removeToBeFinalized

[]{#index-fopen_003amode_003aifFail_003a} []{#index-append-2}
[]{#index-create-2} []{#index-readWrite-2} []{#index-read-2}
[]{#index-write-2} []{#index-close-4} []{#index-removeToBeFinalized-3}

**fopen: fileName mode: fileMode ifFail: aBlock**

Open fileName in the required mode - answered by #append, #create,
#readWrite, #read or #write - and evaluate aBlock if the file cannot be
opened. Else answer a new FileStream. For mode anyway you can use any
The file will be automatically closed upon GC if the object is not
referenced anymore, but it is better to close it as soon as you're
finished with it anyway, using #close. To keep a file open even when no
references exist anymore, send it #removeToBeFinalized

[]{#index-on_003a-2}

**on: fd**

Open a FileDescriptor on the given file descriptor. Read-write access is
assumed.

[]{#index-open_003a} []{#index-close-5} []{#index-removeToBeFinalized-4}

**open: fileName**

Open fileName in read-write mode - fail if the file cannot be opened.
Else answer a new FileStream. The file will be automatically closed upon
GC if the object is not referenced anymore, but you should close it with
#close anyway. To keep a file open, send it #removeToBeFinalized

[]{#index-open_003amode_003aifFail_003a-1} []{#index-append-3}
[]{#index-create-3} []{#index-readWrite-3} []{#index-read-3}
[]{#index-write-3} []{#index-close-6} []{#index-removeToBeFinalized-5}

**open: fileName mode: fileMode ifFail: aBlock**

Open fileName in the required mode - answered by #append, #create,
#readWrite, #read or #write - and evaluate aBlock if the file cannot be
opened. Else answer a new instance of the receiver. For mode anyway you
can use any standard C non-binary fopen mode. fileName can be a 'virtual
filesystem' path, including URLs and '#' suffixes that are inspected by
the virtual filesystem layers and replaced with tasks such as
un-gzipping a file or extracting a file from an archive.

The file will be automatically closed upon GC if the object is not
referenced anymore, but it is better to close it as soon as you're
finished with it anyway, using #close. To keep a file open even when no
references exist anymore, send it #removeToBeFinalized

[]{#index-openTemporaryFile_003a}

**openTemporaryFile: baseName**

Open for writing a file whose name starts with baseName, followed by six
random alphanumeric characters. The file is created with mode read/write
and permissions 0666 or 0600 on most recent operating systems (beware,
the former behavior might constitute a security problem). The file is
opened with the O_EXCL flag, guaranteeing that when the method returns
successfully we are the only user.

[]{#index-popen_003adir_003a} []{#index-read-4} []{#index-write-4}

**popen: commandName dir: direction**

Open a pipe on the given command and fail if the file cannot be opened.
Else answer a new FileStream. The pipe will not be automatically closed
upon GC, even if the object is not referenced anymore, because when you
close a pipe you have to wait for the associated process to terminate.
direction is returned by #read or #write ('r' or 'w') and is interpreted
from the point of view of Smalltalk: reading means Smalltalk reads the
standard output of the command, writing means Smalltalk writes the
standard input of the command. The other channel (stdin when reading,
stdout when writing) is the same as GST's, unless commandName alters it.

[]{#index-popen_003adir_003aifFail_003a}

**popen: commandName dir: direction ifFail: aBlock**

Open a pipe on the given command and evaluate aBlock file cannot be
opened. Else answer a new FileStream. The pipe will not be automatically
closed upon GC, even if the object is not referenced anymore, because
when you close a pipe you have to wait for the associated process to
terminate. direction is interpreted from the point of view of Smalltalk:
reading means that Smalltalk reads the standard output of the command,
writing means that Smalltalk writes the standard input of the command

[]{#index-read}

**read**

Open text file for reading. The stream is positioned at the beginning of
the file.

[]{#index-readWrite}

**readWrite**

Open for reading and writing. The stream is positioned at the beginning
of the file.

[]{#index-write}

**write**

Truncate file to zero length or create text file for writing. The stream
is positioned at the beginning of the file.

------------------------------------------------------------------------

::: header
Next: [FileDescriptor class-still
unclassified](FileDescriptor-class_002dstill-unclassified.html#FileDescriptor-class_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [FileDescriptor
class-initialization](FileDescriptor-class_002dinitialization.html#FileDescriptor-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
