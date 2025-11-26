[]{#Behavior_002dmethod-dictionary}

::: header
Next: [Behavior-parsing class
declarations](Behavior_002dparsing-class-declarations.html#Behavior_002dparsing-class-declarations){accesskey="n"
rel="next"}, Previous: [Behavior-instance
variables](Behavior_002dinstance-variables.html#Behavior_002dinstance-variables){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-method-dictionary}

#### 1.9.15 Behavior: method dictionary {#behavior-method-dictionary .subsection}

[]{#index-addSelector_003awithMethod_003a}

**addSelector: selector withMethod: compiledMethod**

Add the given compiledMethod to the method dictionary, giving it the
passed selector. Answer compiledMethod

[]{#index-compile_003a}

**compile: code**

Compile method source. If there are parsing errors, answer nil. Else,
return a CompiledMethod result of compilation

[]{#index-compile_003aifError_003a}

**compile: code ifError: block**

Compile method source. If there are parsing errors, invoke exception
block, 'block' passing file name, line number and error. Return a
CompiledMethod result of compilation

[]{#index-compile_003anotifying_003a} []{#index-error_003a-2}

**compile: code notifying: requestor**

Compile method source. If there are parsing errors, send #error: to the
requestor object, else return a CompiledMethod result of compilation

[]{#index-compileAll}

**compileAll**

Recompile all selectors in the receiver. Ignore errors.

[]{#index-compileAll_003a} []{#index-error_003a-3}

**compileAll: aNotifier**

Recompile all selectors in the receiver. Notify aNotifier by sending
#error: messages if something goes wrong.

[]{#index-compileAllSubclasses} []{#index-error_003a-4}

**compileAllSubclasses**

Recompile all selector of all subclasses. Notify aNotifier by sending
#error: messages if something goes wrong.

[]{#index-compileAllSubclasses_003a} []{#index-error_003a-5}

**compileAllSubclasses: aNotifier**

Recompile all selector of all subclasses. Notify aNotifier by sending
#error: messages if something goes wrong.

[]{#index-createGetMethod_003a}

**createGetMethod: what**

Create a method accessing the variable 'what'.

[]{#index-createGetMethod_003adefault_003a}

**createGetMethod: what default: value**

Create a method accessing the variable 'what', with a default value of
'value', using lazy initialization

[]{#index-createSetMethod_003a}

**createSetMethod: what**

Create a method which sets the variable 'what'.

[]{#index-decompile_003a}

**decompile: selector**

Decompile the bytecodes for the given selector.

[]{#index-defineAsyncCFunc_003awithSelectorArgs_003aargs_003a}

**defineAsyncCFunc: cFuncNameString withSelectorArgs: selectorAndArgs
args: argsArray**

Please lookup the part on the C interface in the manual. This method is
deprecated, you should use the asyncCCall:args: attribute.

[]{#index-defineCFunc_003awithSelectorArgs_003areturning_003aargs_003a}

**defineCFunc: cFuncNameString withSelectorArgs: selectorAndArgs
returning: returnTypeSymbol args: argsArray**

Please lookup the part on the C interface in the manual. This method is
deprecated, you should use the cCall:returning:args: attribute.

[]{#index-edit_003a}

**edit: selector**

Open Emacs to edit the method with the passed selector, then compile it

[]{#index-methodDictionary}

**methodDictionary**

Answer the receiver's method dictionary. Don't modify the method
dictionary unless you exactly know what you're doing

[]{#index-methodDictionary_003a}

**methodDictionary: aDictionary**

Set the receiver's method dictionary to aDictionary

[]{#index-recompile_003a}

**recompile: selector**

Recompile the given selector, answer nil if something goes wrong or the
new CompiledMethod if everything's ok.

[]{#index-recompile_003anotifying_003a} []{#index-error_003a-6}

**recompile: selector notifying: aNotifier**

Recompile the given selector. If there are parsing errors, send #error:
to the aNotifier object, else return a CompiledMethod result of
compilation

[]{#index-removeSelector_003a}

**removeSelector: selector**

Remove the given selector from the method dictionary, answer the
CompiledMethod attached to that selector

[]{#index-removeSelector_003aifAbsent_003a}

**removeSelector: selector ifAbsent: aBlock**

Remove the given selector from the method dictionary, answer the
CompiledMethod attached to that selector. If the selector cannot be
found, answer the result of evaluating aBlock.

[]{#index-selectorsAndMethodsDo_003a}

**selectorsAndMethodsDo: aBlock**

Evaluate aBlock, passing for each evaluation a selector that's defined
in the receiver and the corresponding method.

------------------------------------------------------------------------

::: header
Next: [Behavior-parsing class
declarations](Behavior_002dparsing-class-declarations.html#Behavior_002dparsing-class-declarations){accesskey="n"
rel="next"}, Previous: [Behavior-instance
variables](Behavior_002dinstance-variables.html#Behavior_002dinstance-variables){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
