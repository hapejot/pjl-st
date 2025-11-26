[]{#Delay_002ddelaying}

::: header
Next:
[Delay-initialization](Delay_002dinitialization.html#Delay_002dinitialization){accesskey="n"
rel="next"}, Previous:
[Delay-copying](Delay_002dcopying.html#Delay_002dcopying){accesskey="p"
rel="prev"}, Up: [Delay](Delay.html#Delay){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Delay_003a-delaying}

#### 1.62.7 Delay: delaying {#delay-delaying .subsection}

[]{#index-timedWaitOn_003a}

**timedWaitOn: aSemaphore**

Schedule this Delay and wait on it. The current process will be
suspended for the amount of time specified when this Delay was created,
or until aSemaphore is signaled.

[]{#index-wait}

**wait**

Schedule this Delay and wait on it. The current process will be
suspended for the amount of time specified when this Delay was created.
