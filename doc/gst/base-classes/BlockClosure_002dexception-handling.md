[]{#BlockClosure_002dexception-handling}

::: header
Next: [BlockClosure-multiple
process](BlockClosure_002dmultiple-process.html#BlockClosure_002dmultiple-process){accesskey="n"
rel="next"}, Previous: [BlockClosure-control
structures](BlockClosure_002dcontrol-structures.html#BlockClosure_002dcontrol-structures){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure_003a-exception-handling}

#### 1.11.6 BlockClosure: exception handling {#blockclosure-exception-handling .subsection}

[]{#index-ifError_003a} []{#index-error_003a-7}

**ifError: aBlock**

Evaluate the receiver; when #error: is called, pass to aBlock the
receiver and the parameter, and answer the result of evaluating aBlock.
If another exception is raised, it is passed to an outer handler; if no
exception is raised, the result of evaluating the receiver is returned.

[]{#index-on_003ado_003a} []{#index-return_003a-1}

**on: anException do: aBlock**

Evaluate the receiver; when anException is signaled, evaluate aBlock
passing a Signal describing the exception. Answer either the result of
evaluating the receiver or the parameter of a Signal\>\>#return:

[]{#index-on_003ado_003aon_003ado_003a} []{#index-return_003a-2}

**on: e1 do: b1 on: e2 do: b2**

Evaluate the receiver; when e1 or e2 are signaled, evaluate respectively
b1 or b2, passing a Signal describing the exception. Answer either the
result of evaluating the receiver or the argument of a
Signal\>\>#return:

[]{#index-on_003ado_003aon_003ado_003aon_003ado_003a}
[]{#index-return_003a-3}

**on: e1 do: b1 on: e2 do: b2 on: e3 do: b3**

Evaluate the receiver; when e1, e2 or e3 are signaled, evaluate
respectively b1, b2 or b3, passing a Signal describing the exception.
Answer either the result of evaluating the receiver or the parameter of
a Signal\>\>#return:

[]{#index-on_003ado_003aon_003ado_003aon_003ado_003aon_003ado_003a}
[]{#index-return_003a-4}

**on: e1 do: b1 on: e2 do: b2 on: e3 do: b3 on: e4 do: b4**

Evaluate the receiver; when e1, e2, e3 or e4 are signaled, evaluate
respectively b1, b2, b3 or b4, passing a Signal describing the
exception. Answer either the result of evaluating the receiver or the
parameter of a Signal\>\>#return:

[]{#index-on_003ado_003aon_003ado_003aon_003ado_003aon_003ado_003aon_003ado_003a}
[]{#index-return_003a-5}

**on: e1 do: b1 on: e2 do: b2 on: e3 do: b3 on: e4 do: b4 on: e5 do:
b5**

Evaluate the receiver; when e1, e2, e3, e4 or e5 are signaled, evaluate
respectively b1, b2, b3, b4 or b5, passing a Signal describing the
exception. Answer either the result of evaluating the receiver or the
parameter of a Signal\>\>#return:

------------------------------------------------------------------------

::: header
Next: [BlockClosure-multiple
process](BlockClosure_002dmultiple-process.html#BlockClosure_002dmultiple-process){accesskey="n"
rel="next"}, Previous: [BlockClosure-control
structures](BlockClosure_002dcontrol-structures.html#BlockClosure_002dcontrol-structures){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
