[]{#FilePath_002dfile-operations}

::: header
Next:
[FilePath-printing](FilePath_002dprinting.html#FilePath_002dprinting){accesskey="n"
rel="next"}, Previous: [FilePath-file name
management](FilePath_002dfile-name-management.html#FilePath_002dfile-name-management){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FilePath_003a-file-operations}

#### 1.77.9 FilePath: file operations {#filepath-file-operations .subsection}

[]{#index-contents-1}

**contents**

Open a read-only FileStream on the receiver, read its contents, close
the stream and answer the contents

[]{#index-fileIn-1}

**fileIn**

File in the receiver

[]{#index-open_003a-1}

**open: mode**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-open_003aifFail_003a}

**open: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods). Upon failure, evaluate aBlock.

[]{#index-open_003amode_003aifFail_003a-2}

**open: class mode: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-openDescriptor_003a}

**openDescriptor: mode**

Open the receiver in the given mode (as answered by FileStream's class
constant methods)

[]{#index-openDescriptor_003aifFail_003a}

**openDescriptor: mode ifFail: aBlock**

Open the receiver in the given mode (as answered by FileStream's class
constant methods). Upon failure, evaluate aBlock.

[]{#index-pathFrom_003a-1}

**pathFrom: dirName**

Compute the relative path from the directory dirName to the receiver

[]{#index-readStream-2}

**readStream**

Open a read-only FileStream on the receiver

[]{#index-remove-1}

**remove**

Remove the file identified by the receiver

[]{#index-renameTo_003a-1}

**renameTo: newName**

Rename the file identified by the receiver to newName

[]{#index-symlinkAs_003a-1}

**symlinkAs: destName**

Create destName as a symbolic link of the receiver. The appropriate
relative path is computed automatically.

[]{#index-symlinkFrom_003a-1}

**symlinkFrom: srcName**

Create the receiver as a symbolic link from srcName (relative to the
path of the receiver).

[]{#index-touch}

**touch**

Update the timestamp of the file corresponding to the receiver.

[]{#index-withReadStreamDo_003a}

**withReadStreamDo: aBlock**

Answer the result of invoking aBlock with a reading stream open on me,
closing it when the dynamic extent of aBlock ends.

[]{#index-withWriteStreamDo_003a}

**withWriteStreamDo: aBlock**

Answer the result of invoking aBlock with a writing stream open on me,
closing it when the dynamic extent of aBlock ends.

[]{#index-writeStream-1}

**writeStream**

Open a write-only FileStream on the receiver

------------------------------------------------------------------------

::: header
Next:
[FilePath-printing](FilePath_002dprinting.html#FilePath_002dprinting){accesskey="n"
rel="next"}, Previous: [FilePath-file name
management](FilePath_002dfile-name-management.html#FilePath_002dfile-name-management){accesskey="p"
rel="prev"}, Up: [FilePath](FilePath.html#FilePath){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
