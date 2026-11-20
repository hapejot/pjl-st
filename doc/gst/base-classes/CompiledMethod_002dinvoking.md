[]{#CompiledMethod_002dinvoking}

::: header
Next:
[CompiledMethod-printing](CompiledMethod_002dprinting.html#CompiledMethod_002dprinting){accesskey="n"
rel="next"}, Previous:
[CompiledMethod-compiling](CompiledMethod_002dcompiling.html#CompiledMethod_002dcompiling){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledMethod_003a-invoking}

#### 1.40.9 CompiledMethod: invoking {#compiledmethod-invoking .subsection}

[]{#index-valueWithReceiver_003awithArguments_003a}
[]{#index-subclassResponsibility-1}

**valueWithReceiver: anObject withArguments: args**

Execute the method within anObject, passing the elements of the args
Array as parameters. The method need not reside on the hierarchy from
the receiver's class to Object -- it need not reside at all in a
MethodDictionary, in fact -- but doing bad things will compromise
stability of the Smalltalk virtual machine (and don't blame anybody but
yourself).

If the flags field of the method header is 6, this method instead
provides a hook from which the virtual machine can call back whenever
execution of the method is requested. In this case, invoking the method
would cause an infinite loop (the VM asks the method to run, the method
asks the VM to invoke it, and so on), so this method fails with a
#subclassResponsibility error.
