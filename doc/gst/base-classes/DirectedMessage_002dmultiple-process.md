[]{#DirectedMessage_002dmultiple-process}

::: header
Next: [DirectedMessage-saving and
loading](DirectedMessage_002dsaving-and-loading.html#DirectedMessage_002dsaving-and-loading){accesskey="n"
rel="next"}, Previous:
[DirectedMessage-basic](DirectedMessage_002dbasic.html#DirectedMessage_002dbasic){accesskey="p"
rel="prev"}, Up:
[DirectedMessage](DirectedMessage.html#DirectedMessage){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DirectedMessage_003a-multiple-process}

#### 1.65.4 DirectedMessage: multiple process {#directedmessage-multiple-process .subsection}

[]{#index-fork-1}

**fork**

Create a new process executing the receiver and start it

[]{#index-forkAt_003a-1}

**forkAt: priority**

Create a new process executing the receiver with given priority and
start it

[]{#index-newProcess-1}

**newProcess**

Create a new process executing the receiver in suspended state. The
priority is the same as for the calling process. The receiver must not
contain returns
