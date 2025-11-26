[]{#BlockClosure_002dunwind-protection}

::: header
Previous:
[BlockClosure-testing](BlockClosure_002dtesting.html#BlockClosure_002dtesting){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure_003a-unwind-protection}

#### 1.11.10 BlockClosure: unwind protection {#blockclosure-unwind-protection .subsection}

[]{#index-ensure_003a}

**ensure: aBlock**

Evaluate the receiver; when any exception is signaled exit returning the
result of evaluating aBlock; if no exception is raised, return the
result of evaluating aBlock when the receiver has ended

[]{#index-ifCurtailed_003a}

**ifCurtailed: aBlock**

Evaluate the receiver; if its execution triggers an unwind which
truncates the execution of the block ('curtails' the block), evaluate
aBlock. The three cases which can curtail the execution of the receiver
are: a non-local return in the receiver, a non-local return in a block
evaluated by the receiver which returns past the receiver itself, and an
exception raised and not resumed during the execution of the receiver.

[]{#index-valueWithUnwind} []{#index-valueWithUnwind-1}
[]{#index-ensure_003a-1} []{#index-on_003ado_003a-2}

**valueWithUnwind**

Evaluate the receiver. Any errors caused by the block will cause a
backtrace, but execution will continue in the method that sent
#valueWithUnwind, after that call. Example: \[ 1 / 0 \] valueWithUnwind.
'unwind works!' printNl.

Important: this method is public, but it is intended to be used in very
special cases (as a rule of thumb, use it only when the corresponding C
code uses the \_gst_prepare_execution_environment and
\_gst_finish_execution_environment functions). You should usually rely
on #ensure: and #on:do:.
