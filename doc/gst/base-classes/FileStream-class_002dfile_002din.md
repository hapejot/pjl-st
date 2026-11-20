[]{#FileStream-class_002dfile_002din}

::: header
Next: [FileStream class-standard
streams](FileStream-class_002dstandard-streams.html#FileStream-class_002dstandard-streams){accesskey="n"
rel="next"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileStream-class_003a-file_002din}

#### 1.79.1 FileStream class: file-in {#filestream-class-file-in .subsection}

[]{#index-fileIn_003a}

**fileIn: aFileName**

File in the aFileName file. During a file in operation, global variables
(starting with an uppercase letter) that are not declared yet don't
yield an 'unknown variable' error. Instead, they are defined as nil in
the 'Undeclared' dictionary (a global variable residing in Smalltalk).
As soon as you add the variable to a namespace (for example by creating
a class) the Association will be removed from Undeclared and reused in
the namespace, so that the old references will automagically point to
the new value.

[]{#index-fileIn_003aifMissing_003a}

**fileIn: aFileName ifMissing: aSymbol**

Conditionally do a file in, only if the key (often a class) specified by
'aSymbol' is not present in the Smalltalk system dictionary already.
During a file in operation, global variables (starting with an uppercase
letter) that are not declared don't yield an 'unknown variable' error.
Instead, they are defined as nil in the 'Undeclared' dictionary (a
global variable residing in Smalltalk). As soon as you add the variable
to a namespace (for example by creating a class) the Association will be
removed from Undeclared and reused in the namespace, so that the old
references will automagically point to the new value.

[]{#index-fileIn_003aifTrue_003a}

**fileIn: aFileName ifTrue: aBoolean**

Conditionally do a file in, only if the supplied boolean is true. During
a file in operation, global variables (starting with an uppercase
letter) that are not declared don't yield an 'unknown variable' error.
Instead, they are defined as nil in the 'Undeclared' dictionary (a
global variable residing in Smalltalk). As soon as you add the variable
to a namespace (for example by creating a class) the Association will be
removed from Undeclared and reused in the namespace, so that the old
references will automagically point to the new value.

[]{#index-fileIn_003aline_003afrom_003aat_003a}

**fileIn: aFileName line: lineInteger from: realFileName at: aCharPos**

File in the aFileName file giving errors such as if it was loaded from
the given line, file name and starting position (instead of 1).

[]{#index-generateMakefileOnto_003a}

**generateMakefileOnto: aStream**

Generate a make file for the file-ins since record was last set to true.
Store it on aStream

[]{#index-initialize-11}

**initialize**

Private - Initialize the receiver's class variables

[]{#index-record_003a} []{#index-generateMakefileOnto_003a-1}

**record: recordFlag**

Set whether Smalltalk should record information about nested file-ins.
When recording is enabled, use #generateMakefileOnto: to automatically
generate a valid makefile for the intervening file-ins.

[]{#index-require_003a}

**require: assoc**

Conditionally do a file in from the value of assoc, only if the key of
assoc is not present in the Smalltalk system dictionary already. During
a file in operation, global variables (starting with an uppercase
letter) that are not declared don't yield an 'unknown variable' error.
Instead, they are defined as nil in the 'Undeclared' dictionary (a
global variable residing in Smalltalk). As soon as you add the variable
to a namespace (for example by creating a class) the Association will be
removed from Undeclared and reused in the namespace, so that the old
references will automagically point to the new value.

[]{#index-verbose_003a}

**verbose: verboseFlag**

Set whether Smalltalk should output debugging messages when filing in

------------------------------------------------------------------------

::: header
Next: [FileStream class-standard
streams](FileStream-class_002dstandard-streams.html#FileStream-class_002dstandard-streams){accesskey="n"
rel="next"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
