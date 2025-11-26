[]{#Debugger_002dstepping-commands}

::: header
Previous: [Debugger-inferior process
properties](Debugger_002dinferior-process-properties.html#Debugger_002dinferior-process-properties){accesskey="p"
rel="prev"}, Up: [Debugger](Debugger.html#Debugger){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Debugger_003a-stepping-commands}

#### 4.1.5 Debugger: stepping commands {#debugger-stepping-commands .subsection}

[]{#index-continue}

**continue**

Terminate the controlling process and continue execution of the traced
process.

[]{#index-finish}

**finish**

Run to the next return.

[]{#index-finish_003a}

**finish: aContext**

Run up until aContext returns.

[]{#index-next-1}

**next**

Run to the end of the current line in the inferior process, skipping
over message sends.

[]{#index-slowFinish}

**slowFinish**

Run in single-step mode up to the next return.

[]{#index-slowFinish_003a}

**slowFinish: aContext**

Run in single-step mode until aContext returns.

[]{#index-step}

**step**

Run to the end of the current line in the inferior process or to the
next message send.

[]{#index-stepBytecode}

**stepBytecode**

Run a single bytecode in the inferior process.

[]{#index-stopInferior}

**stopInferior**

Suspend the inferior process and raise a DebuggerReentered notification
in the controlling process.

[]{#index-stopInferior_003a}

**stopInferior: anObject**

Suspend the inferior process and raise a DebuggerReentered notification
in the controlling process with anObject as the exception's message.
