[]{#BlockClosure_002dcontrol-structures}

::: header
Next: [BlockClosure-exception
handling](BlockClosure_002dexception-handling.html#BlockClosure_002dexception-handling){accesskey="n"
rel="next"}, Previous: [BlockClosure-built
ins](BlockClosure_002dbuilt-ins.html#BlockClosure_002dbuilt-ins){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure_003a-control-structures}

#### 1.11.5 BlockClosure: control structures {#blockclosure-control-structures .subsection}

[]{#index-repeat}

**repeat**

Evaluate the receiver 'forever' (actually until a return is executed or
the process is terminated).

[]{#index-whileFalse}

**whileFalse**

Evaluate the receiver until it returns true

[]{#index-whileFalse_003a}

**whileFalse: aBlock**

Evaluate the receiver. If it returns false, evaluate aBlock and restart

[]{#index-whileTrue}

**whileTrue**

Evaluate the receiver until it returns false

[]{#index-whileTrue_003a}

**whileTrue: aBlock**

Evaluate the receiver. If it returns true, evaluate aBlock and restart
