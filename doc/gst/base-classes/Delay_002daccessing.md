[]{#Delay_002daccessing}

::: header
Next:
[Delay-comparing](Delay_002dcomparing.html#Delay_002dcomparing){accesskey="n"
rel="next"}, Previous: [Delay class-timer
process](Delay-class_002dtimer-process.html#Delay-class_002dtimer-process){accesskey="p"
rel="prev"}, Up: [Delay](Delay.html#Delay){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Delay_003a-accessing}

#### 1.62.4 Delay: accessing {#delay-accessing .subsection}

[]{#index-asAbsolute}

**asAbsolute**

Answer a delay that waits until the current delay's resumptionTime, or
delayDuration milliseconds from now if that would be nil. May answer the
receiver if it is already waiting until an absolute time.

[]{#index-delayDuration}

**delayDuration**

Answer the time I have left to wait, in milliseconds.

[]{#index-isAbsolute-1}

**isAbsolute**

Answer whether the receiver waits until an absolute time on the
millisecond clock.

[]{#index-resumptionTime}

**resumptionTime**

Answer 'resumptionTime'.
