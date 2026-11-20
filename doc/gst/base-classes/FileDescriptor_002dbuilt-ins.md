[]{#FileDescriptor_002dbuilt-ins}

::: header
Next: [FileDescriptor-class type
methods](FileDescriptor_002dclass-type-methods.html#FileDescriptor_002dclass-type-methods){accesskey="n"
rel="next"}, Previous: [FileDescriptor-binary
I/O](FileDescriptor_002dbinary-I_002fO.html#FileDescriptor_002dbinary-I_002fO){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor_003a-built-ins}

#### 1.76.7 FileDescriptor: built ins {#filedescriptor-built-ins .subsection}

[]{#index-fileIn}

**fileIn**

File in the contents of the receiver. During a file in operation, global
variables (starting with an uppercase letter) that are not declared
don't yield an 'unknown variable' error. Instead, they are defined as
nil in the 'Undeclared' dictionary (a global variable residing in
Smalltalk). As soon as you add the variable to a namespace (for example
by creating a class) the Association will be removed from Undeclared and
reused in the namespace, so that the old references will automagically
point to the new value.

[]{#index-fileOp_003a}

**fileOp: ioFuncIndex**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003aifFail_003a}

**fileOp: ioFuncIndex ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003a}

**fileOp: ioFuncIndex with: arg1**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003awith_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003awith_003awith_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003awith_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3 ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

[]{#index-fileOp_003awith_003awith_003awith_003awith_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3 with: arg4**

Private - Used to limit the number of primitives used by FileStreams

[]{#index-fileOp_003awith_003awith_003awith_003awith_003aifFail_003a}

**fileOp: ioFuncIndex with: arg1 with: arg2 with: arg3 with: arg4
ifFail: aBlock**

Private - Used to limit the number of primitives used by FileStreams.

------------------------------------------------------------------------

::: header
Next: [FileDescriptor-class type
methods](FileDescriptor_002dclass-type-methods.html#FileDescriptor_002dclass-type-methods){accesskey="n"
rel="next"}, Previous: [FileDescriptor-binary
I/O](FileDescriptor_002dbinary-I_002fO.html#FileDescriptor_002dbinary-I_002fO){accesskey="p"
rel="prev"}, Up:
[FileDescriptor](FileDescriptor.html#FileDescriptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
