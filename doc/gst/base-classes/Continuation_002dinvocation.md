[]{#Continuation_002dinvocation}

::: header
Previous: [Continuation class-instance
creation](Continuation-class_002dinstance-creation.html#Continuation-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Continuation](Continuation.html#Continuation){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Continuation_003a-invocation}

#### 1.42.2 Continuation: invocation {#continuation-invocation .subsection}

[]{#index-callCC}

**callCC**

Activate the original continuation, passing back in turn a continuation
for the caller. The called continuation becomes unusable, and any
attempt to reactivate it will cause an exception. This is not a
limitation, in general, because this method is used to replace a
continuation with another (see the implementation of the Generator
class).

[]{#index-oneShotValue} []{#index-value-24}

**oneShotValue**

Return nil to the original continuation, which becomes unusable.
Attempting to reactivate it will cause an exception. This is an
optimization over #value.

[]{#index-oneShotValue_003a} []{#index-value_003a-22}

**oneShotValue: v**

Return anObject to the original continuation, which becomes unusable.
Attempting to reactivate it will cause an exception. This is an
optimization over #value:.

[]{#index-value-5}

**value**

Return nil to the original continuation, copying the stack to allow
another activation.

[]{#index-value_003a-5}

**value: anObject**

Return anObject to the original continuation, copying the stack to allow
another activation.

[]{#index-valueWithArguments_003a-1}

**valueWithArguments: aCollection**

Return the sole element of aCollection to the original continuation (or
nil if aCollection is empty), copying the stack to allow another
activation
