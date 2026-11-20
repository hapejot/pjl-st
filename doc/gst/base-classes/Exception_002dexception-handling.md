[]{#Exception_002dexception-handling}

::: header
Next: [Exception-exception
signaling](Exception_002dexception-signaling.html#Exception_002dexception-signaling){accesskey="n"
rel="next"}, Previous: [Exception-exception
description](Exception_002dexception-description.html#Exception_002dexception-description){accesskey="p"
rel="prev"}, Up: [Exception](Exception.html#Exception){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Exception_003a-exception-handling}

#### 1.72.10 Exception: exception handling {#exception-exception-handling .subsection}

[]{#index-context} []{#index-on_003ado_003a-3}

**context**

Return the execution context for the #on:do: snippet

[]{#index-isNested}

**isNested**

Answer whether the current exception handler is within the scope of
another handler for the same exception.

[]{#index-outer} []{#index-outer-1} []{#index-outer-2} []{#index-pass-1}

**outer**

Raise the exception that instantiated the receiver, passing the same
parameters. If the receiver is resumable and the evaluated exception
action resumes then the result returned from #outer will be the
resumption value of the evaluated exception action. If the receiver is
not resumable or if the exception action does not resume then this
message will not return, and #outer will be equivalent to #pass.

[]{#index-pass} []{#index-outer-3}

**pass**

Yield control to the enclosing exception action for the receiver.
Similar to #outer, but control does not return to the currently active
exception handler.

[]{#index-resignalAs_003a}

**resignalAs: replacementException**

Reinstate all handlers and execute the handler for
'replacementException'; control does not return to the currently active
exception handler. The new Signal object that is created has the same
contents as the receiver (this might or not be correct -- if it isn't
you can use an idiom such as 'sig retryUsing: \[ replacementException
signal \])

[]{#index-resume}

**resume**

If the exception is resumable, resume the execution of the block that
raised the exception; the method that was used to signal the exception
will answer the receiver. Use this method IF AND ONLY IF you know who
caused the exception and if it is possible to resume it in that
particular case

[]{#index-resume_003a}

**resume: anObject**

If the exception is resumable, resume the execution of the block that
raised the exception; the method that was used to signal the exception
will answer anObject. Use this method IF AND ONLY IF you know who caused
the exception and if it is possible to resume it in that particular case

[]{#index-retry} []{#index-on_003ado_003a-4}

**retry**

Re-execute the receiver of the #on:do: message. All handlers are
reinstated: watch out, this can easily cause an infinite loop.

[]{#index-retryUsing_003a} []{#index-signal-3}

**retryUsing: aBlock**

Execute aBlock reinstating all handlers, and return its result from the
#signal method.

[]{#index-return} []{#index-on_003ado_003a-5}

**return**

Exit the #on:do: snippet, answering nil to its caller.

[]{#index-return_003a} []{#index-on_003ado_003a-6}

**return: anObject**

Exit the #on:do: snippet, answering anObject to its caller.

------------------------------------------------------------------------

::: header
Next: [Exception-exception
signaling](Exception_002dexception-signaling.html#Exception_002dexception-signaling){accesskey="n"
rel="next"}, Previous: [Exception-exception
description](Exception_002dexception-description.html#Exception_002dexception-description){accesskey="p"
rel="prev"}, Up: [Exception](Exception.html#Exception){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
