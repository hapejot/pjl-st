[]{#Stream_002dbuilt-ins}

::: header
Next: [Stream-character
writing](Stream_002dcharacter-writing.html#Stream_002dcharacter-writing){accesskey="n"
rel="next"}, Previous:
[Stream-buffering](Stream_002dbuffering.html#Stream_002dbuffering){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-built-ins}

#### 1.157.5 Stream: built ins {#stream-built-ins .subsection}

[]{#index-fileIn-2}

**fileIn**

File in the contents of the receiver. During a file in operation, global
variables (starting with an uppercase letter) that are not declared
don't yield an 'unknown variable' error. Instead, they are defined as
nil in the 'Undeclared' dictionary (a global variable residing in
Smalltalk). As soon as you add the variable to a namespace (for example
by creating a class) the Association will be removed from Undeclared and
reused in the namespace, so that the old references will automagically
point to the new value.

[]{#index-fileInLine_003afile_003aat_003a} []{#index-line}
[]{#index-fileIn-3}

**fileInLine: lineNum file: aFile at: charPosInt**

Private - Much like a preprocessor #line directive; it is used
internally by #fileIn, and explicitly by the Emacs Smalltalk mode.

[]{#index-fileInLine_003afileName_003aat_003a} []{#index-line-1}
[]{#index-fileIn-4}

**fileInLine: lineNum fileName: aString at: charPosInt**

Private - Much like a preprocessor #line directive; it is used
internally by #fileIn, and explicitly by the Emacs Smalltalk mode.
