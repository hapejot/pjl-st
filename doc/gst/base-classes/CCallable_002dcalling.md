[]{#CCallable_002dcalling}

::: header
Next:
[CCallable-restoring](CCallable_002drestoring.html#CCallable_002drestoring){accesskey="n"
rel="next"}, Previous:
[CCallable-accessing](CCallable_002daccessing.html#CCallable_002daccessing){accesskey="p"
rel="prev"}, Up: [CCallable](CCallable.html#CCallable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CCallable_003a-calling}

#### 1.21.3 CCallable: calling {#ccallable-calling .subsection}

[]{#index-asyncCall} []{#index-self} []{#index-selfSmalltalk}

**asyncCall**

Perform the call-out for the function represented by the receiver. The
arguments (and the receiver if one of the arguments has type #self or
#selfSmalltalk) are taken from the parent context. Asynchronous
call-outs don't return a value, but if the function calls back into
Smalltalk the process that started the call-out is not suspended.

[]{#index-asyncCallNoRetryFrom_003a} []{#index-self-1}
[]{#index-selfSmalltalk-1} []{#index-asyncCallFrom_003a}

**asyncCallNoRetryFrom: aContext**

Perform the call-out for the function represented by the receiver. The
arguments (and the receiver if one of the arguments has type #self or
#selfSmalltalk) are taken from the base of the stack of aContext.
Asynchronous call-outs don't return a value, but if the function calls
back into Smalltalk the process that started the call-out is not
suspended. Unlike #asyncCallFrom:, this method does not attempt to find
functions in shared objects.

[]{#index-callInto_003a} []{#index-self-2} []{#index-selfSmalltalk-2}

**callInto: aValueHolder**

Perform the call-out for the function represented by the receiver. The
arguments (and the receiver if one of the arguments has type #self or
#selfSmalltalk) are taken from the parent context, and the the result is
stored into aValueHolder. aValueHolder is also returned.

[]{#index-callNoRetryFrom_003ainto_003a} []{#index-self-3}
[]{#index-selfSmalltalk-3} []{#index-callFrom_003ainto_003a}

**callNoRetryFrom: aContext into: aValueHolder**

Perform the call-out for the function represented by the receiver. The
arguments (and the receiver if one of the arguments has type #self or
#selfSmalltalk) are taken from the base of the stack of aContext, and
the result is stored into aValueHolder. aValueHolder is also returned.
Unlike #callFrom:into:, this method does not attempt to find functions
in shared objects.

------------------------------------------------------------------------

::: header
Next:
[CCallable-restoring](CCallable_002drestoring.html#CCallable_002drestoring){accesskey="n"
rel="next"}, Previous:
[CCallable-accessing](CCallable_002daccessing.html#CCallable_002daccessing){accesskey="p"
rel="prev"}, Up: [CCallable](CCallable.html#CCallable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
