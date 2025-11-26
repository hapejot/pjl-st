[]{#ContextPart_002daccessing}

::: header
Next: [ContextPart-built
ins](ContextPart_002dbuilt-ins.html#ContextPart_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [ContextPart class-exception
handling](ContextPart-class_002dexception-handling.html#ContextPart-class_002dexception-handling){accesskey="p"
rel="prev"}, Up:
[ContextPart](ContextPart.html#ContextPart){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ContextPart_003a-accessing}

#### 1.41.3 ContextPart: accessing {#contextpart-accessing .subsection}

[]{#index-at_003a-1}

**at: index**

Answer the index-th slot in the receiver. Any read access from (self
size + 1) to (self basicSize) will give nil.

[]{#index-at_003aput_003a-3}

**at: index put: anObject**

Answer the index-th slot in the receiver. Any write access from (self
size + 1) to (self basicSize) will give an error unless nil is being
written. This is because doing such an access first, and then updating
sp, is racy: the garbage collector may trigger in the middle and move
anObject, but the slot in the context won't be adjusted.

[]{#index-client}

**client**

Answer the client of this context, that is, the object that sent the
message that created this context. Fail if the receiver has no parent

[]{#index-currentFileName}

**currentFileName**

Answer the name of the file where the method source code is

[]{#index-environment-4}

**environment**

To create a valid execution environment for the interpreter even before
it starts, GST creates a fake context whose selector is nil and which
can be used as a marker for the current execution environment. This
method answers that context. For processes, it answers the process block
itself

[]{#index-home-1}

**home**

Answer the MethodContext to which the receiver refers

[]{#index-initialIP-1}

**initialIP**

Answer the value of the instruction pointer when execution starts in the
current context

[]{#index-ip}

**ip**

Answer the current instruction pointer into the receiver

[]{#index-ip_003a}

**ip: newIP**

Set the instruction pointer for the receiver

[]{#index-isBlock-1}

**isBlock**

Answer whether the receiver is a block context

[]{#index-isDisabled-1} []{#index-continue_003a-2}
[]{#index-ensure_003a-4}

**isDisabled**

Answers whether the context is skipped when doing a return. Contexts are
marked as disabled whenever a non-local return is done (either by
returning from the enclosing method of a block, or with the #continue:
method of ContextPart) and there are unwind contexts such as those
created by #ensure:. All non-unwind contexts are then marked as
disabled.

[]{#index-isEnvironment-1}

**isEnvironment**

To create a valid execution environment for the interpreter even before
it starts, GST creates a fake context which invokes a special
"termination" method. Such a context can be used as a marker for the
current execution environment. Answer whether the receiver is that kind
of context.

[]{#index-isProcess} []{#index-newProcess-2}

**isProcess**

Answer whether the receiver represents a process context, i.e. a context
created by BlockClosure\>\>#newProcess. Such a context can be recognized
because it has no parent but its flags are different from those of the
contexts created by the VM's prepareExecutionEnvironment function.

[]{#index-isUnwind-1} []{#index-continue_003a-3}
[]{#index-ensure_003a-5}

**isUnwind**

Answers whether the context must continue execution even after a
non-local return (a return from the enclosing method of a block, or a
call to the #continue: method of ContextPart). Such contexts are created
by #ensure:.

[]{#index-method-4}

**method**

Return the CompiledMethod being executed

[]{#index-methodClass-3}

**methodClass**

Return the class in which the CompiledMethod being executed is defined

[]{#index-numArgs-4}

**numArgs**

Answer the number of arguments passed to the receiver

[]{#index-numTemps-4}

**numTemps**

Answer the number of temporaries used by the receiver

[]{#index-parentContext}

**parentContext**

Answer the context that called the receiver

[]{#index-parentContext_003a}

**parentContext: aContext**

Set the context to which the receiver will return

[]{#index-push_003a}

**push: anObject**

Push an object on the receiver's stack.

[]{#index-receiver-1}

**receiver**

Return the receiver (self) for the method being executed

[]{#index-selector-3}

**selector**

Return the selector for the method being executed

[]{#index-size-3}

**size**

Answer the number of valid fields for the receiver. Any read access from
(self size + 1) to (self basicSize) will give nil.

[]{#index-sp}

**sp**

Answer the current stack pointer into the receiver

[]{#index-sp_003a}

**sp: newSP**

Set the stack pointer for the receiver.

[]{#index-validSize}

**validSize**

Answer how many elements in the receiver should be inspected

------------------------------------------------------------------------

::: header
Next: [ContextPart-built
ins](ContextPart_002dbuilt-ins.html#ContextPart_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [ContextPart class-exception
handling](ContextPart-class_002dexception-handling.html#ContextPart-class_002dexception-handling){accesskey="p"
rel="prev"}, Up:
[ContextPart](ContextPart.html#ContextPart){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
