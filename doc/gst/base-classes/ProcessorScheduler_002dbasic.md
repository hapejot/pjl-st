[]{#ProcessorScheduler_002dbasic}

::: header
Next: [ProcessorScheduler-built
ins](ProcessorScheduler_002dbuilt-ins.html#ProcessorScheduler_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [ProcessorScheduler class-instance
creation](ProcessorScheduler-class_002dinstance-creation.html#ProcessorScheduler-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessorScheduler_003a-basic}

#### 1.136.2 ProcessorScheduler: basic {#processorscheduler-basic .subsection}

[]{#index-activeDebugger}

**activeDebugger**

Answer the active process' debugger

[]{#index-activePriority}

**activePriority**

Answer the active process' priority

[]{#index-activeProcess}

**activeProcess**

Answer the active process

[]{#index-processEnvironment} []{#index-associationAt_003a-2}

**processEnvironment**

Answer another singleton object hosting thread-local variables for the
Smalltalk processes. This acts like a normal Dictionary with a couple of
differences: a) using #associationAt: will return special associations
that retrieve a thread-local value; b) requesting missing keys will
return nil, and removing them will be a nop.

[]{#index-processesAt_003a}

**processesAt: aPriority**

Answer a linked list of processes at the given priority

[]{#index-terminateActive}

**terminateActive**

Terminate the active process

[]{#index-timeSlice}

**timeSlice**

Answer the timeslice that is assigned to each Process before it is
automatically preempted by the system (in milliseconds). An answer of
zero means that preemptive multitasking is disabled. Note that the
system by default is compiled without preemptive multitasking, and that
even if it is enabled it will work only under BSD derivatives (or, in
general, systems that support ITIMER_VIRTUAL).

[]{#index-timeSlice_003a}

**timeSlice: milliSeconds**

Set the timeslice that is assigned to each Process before it is
automatically preempted by the system. Setting this to zero disables
preemptive multitasking. Note that the system by default is compiled
with preemptive multitasking disabled, and that even if it is enabled it
will surely work only under BSD derivatives (or, in general, systems
that support ITIMER_VIRTUAL).

[]{#index-yield-1}

**yield**

Let the active process yield control to other processes
