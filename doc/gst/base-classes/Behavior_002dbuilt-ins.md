[]{#Behavior_002dbuilt-ins}

::: header
Next:
[Behavior-builtin](Behavior_002dbuiltin.html#Behavior_002dbuiltin){accesskey="n"
rel="next"}, Previous: [Behavior-accessing the method
dictionary](Behavior_002daccessing-the-method-dictionary.html#Behavior_002daccessing-the-method-dictionary){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-built-ins}

#### 1.9.4 Behavior: built ins {#behavior-built-ins .subsection}

[]{#index-basicNewInFixedSpace} []{#index-basicNew-1}

**basicNewInFixedSpace**

Create a new instance of a class with no indexed instance variables. The
instance is guaranteed not to move across garbage collections. Like
#basicNew, this method should not be overridden.

[]{#index-basicNewInFixedSpace_003a} []{#index-basicNew_003a-1}

**basicNewInFixedSpace: numInstanceVariables**

Create a new instance of a class with indexed instance variables. The
instance has numInstanceVariables indexed instance variables. The
instance is guaranteed not to move across garbage collections. Like
#basicNew:, this method should not be overridden.

[]{#index-flushCache}

**flushCache**

Invalidate the method cache kept by the virtual machine. This message
should not need to be called by user programs.

[]{#index-methodsFor_003aifTrue_003a}

**methodsFor: category ifTrue: condition**

Compile the following code inside the receiver, with the given category,
if condition is true; else ignore it

[]{#index-primCompile_003a} []{#index-compile_003a-1}

**primCompile: code**

Compile the code, a string or readable stream, with no category. Fail if
the code does not obey Smalltalk syntax. Answer the generated
CompiledMethod if it does.

Do not send this in user code; use #compile: or related methods instead.

[]{#index-primCompile_003aifError_003a} []{#index-primCompile_003a-1}
[]{#index-compile_003aifError_003a-1}

**primCompile: code ifError: aBlock**

As with #primCompile:, but evaluate aBlock (passing the file name, line
number and description of the error) if the code does not obey Smalltalk
syntax.

Do not send this in user code; use #compile:ifError: or related methods
instead.

[]{#index-someInstance}

**someInstance**

Private - Answer the first instance of the receiver in the object table
