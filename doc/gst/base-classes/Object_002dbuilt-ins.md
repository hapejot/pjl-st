[]{#Object_002dbuilt-ins}

::: header
Next: [Object-change and
update](Object_002dchange-and-update.html#Object_002dchange-and-update){accesskey="n"
rel="next"}, Previous: [Object
class-initialization](Object-class_002dinitialization.html#Object-class_002dinitialization){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-built-ins}

#### 1.123.2 Object: built ins {#object-built-ins .subsection}

[]{#index-_003d-33}

**= arg**

Answer whether the receiver is equal to arg. The equality test is by
default the same as that for identical objects. = must not fail; answer
false if the receiver cannot be compared to arg

[]{#index-_003d_003d}

**== arg**

Answer whether the receiver is the same object as arg. This is a very
fast test and is called 'object identity'.

[]{#index-allOwners}

**allOwners**

Return an Array of Objects that point to the receiver.

[]{#index-asOop}

**asOop**

Answer the object index associated to the receiver. The object index
doesn't change when garbage collection is performed.

[]{#index-at_003a-15}

**at: anIndex**

Answer the index-th indexed instance variable of the receiver

[]{#index-at_003aput_003a-15}

**at: anIndex put: value**

Store value in the index-th indexed instance variable of the receiver

[]{#index-basicAt_003a}

**basicAt: anIndex**

Answer the index-th indexed instance variable of the receiver. This
method must not be overridden, override at: instead

[]{#index-basicAt_003aput_003a}

**basicAt: anIndex put: value**

Store value in the index-th indexed instance variable of the receiver
This method must not be overridden, override at:put: instead

[]{#index-basicPrint}

**basicPrint**

Print a basic representation of the receiver

[]{#index-basicSize}

**basicSize**

Answer the number of indexed instance variable in the receiver

[]{#index-become_003a} []{#index-become_003a-1}

**become: otherObject**

Change all references to the receiver into references to otherObject.
Depending on the implementation, references to otherObject might or
might not be transformed into the receiver (respectively, 'two-way
become' and 'one-way become'). Implementations doing one-way become
answer the receiver (so that it is not lost). Most implementations doing
two-way become answer otherObject, but this is not assured - so do
answer the receiver for consistency. GNU Smalltalk does two-way become
and answers otherObject, but this might change in future versions:
programs should not rely on the behavior and results of #become: .

[]{#index-becomeForward_003a}

**becomeForward: otherObject**

Change all references to the receiver into references to otherObject.
References to otherObject are not transformed into the receiver. Answer
the receiver so that it is not lost.

[]{#index-changeClassTo_003a}

**changeClassTo: aBehavior**

Mutate the class of the receiver to be aBehavior. Note: Tacitly assumes
that the structure is the same for the original and new class!!

[]{#index-checkIndexableBounds_003a}

**checkIndexableBounds: index**

Private - Check the reason why an access to the given indexed instance
variable failed

[]{#index-checkIndexableBounds_003aifAbsent_003a}

**checkIndexableBounds: index ifAbsent: aBlock**

Private - Check the reason why an access to the given indexed instance
variable failed. Evaluate aBlock for an invalid index.

[]{#index-checkIndexableBounds_003aput_003a}

**checkIndexableBounds: index put: object**

Private - Check the reason why a store to the given indexed instance
variable failed

[]{#index-class-1}

**class**

Answer the class to which the receiver belongs

[]{#index-halt}

**halt**

Called to enter the debugger

[]{#index-hash-28}

**hash**

Answer an hash value for the receiver. This hash value is ok for objects
that do not redefine ==.

[]{#index-identityHash}

**identityHash**

Answer an hash value for the receiver. This method must not be
overridden

[]{#index-instVarAt_003a}

**instVarAt: index**

Answer the index-th instance variable of the receiver. This method must
not be overridden.

[]{#index-instVarAt_003aput_003a}

**instVarAt: index put: value**

Store value in the index-th instance variable of the receiver. This
method must not be overridden.

[]{#index-isReadOnly}

**isReadOnly**

Answer whether the object's indexed instance variables can be written

[]{#index-isUntrusted}

**isUntrusted**

Answer whether the object is to be considered untrusted.

[]{#index-makeEphemeron} []{#index-mourn-3}

**makeEphemeron**

Make the object an 'ephemeron'. An ephemeron is marked after all other
objects, and if no references are found to the key except from the
object itself, it is sent the #mourn message.

[]{#index-makeFixed}

**makeFixed**

Avoid that the receiver moves in memory across garbage collections.

[]{#index-makeReadOnly_003a}

**makeReadOnly: aBoolean**

Set whether the object's indexed instance variables can be written

[]{#index-makeUntrusted_003a}

**makeUntrusted: aBoolean**

Set whether the object is to be considered untrusted.

[]{#index-makeWeak} []{#index-mourn-4}

**makeWeak**

Make the object a 'weak' one. When an object is only referenced by weak
objects, it is collected and the slots in the weak objects are changed
to nils by the VM; the weak object is then sent the #mourn message.

[]{#index-mark_003a} []{#index-mark_003a-1} []{#index-mark_003a-2}

**mark: aSymbol**

Private - use this method to mark code which needs to be reworked,
removed, etc. You can then find all senders of #mark: to find all marked
methods or you can look for all senders of the symbol that you sent to
#mark: to find a category of marked methods.

[]{#index-nextInstance}

**nextInstance**

Private - answer another instance of the receiver's class, or nil if the
entire object table has been walked

[]{#index-notYetImplemented}

**notYetImplemented**

Called when a method defined by a class is not yet implemented, but is
going to be

[]{#index-perform_003a}

**perform: selectorOrMessageOrMethod**

Send the unary message named selectorOrMessageOrMethod (if a Symbol) to
the receiver, or the message and arguments it identifies (if a Message
or DirectedMessage), or finally execute the method within the receiver
(if a CompiledMethod). In the last case, the method need not reside on
the hierarchy from the receiver's class to Object -- it need not reside
at all in a MethodDictionary, in fact -- but doing bad things will
compromise stability of the Smalltalk virtual machine (and don't blame
anybody but yourself).

This method should not be overridden

[]{#index-perform_003awith_003a}

**perform: selectorOrMethod with: arg1**

Send the message named selectorOrMethod (if a Symbol) to the receiver,
passing arg1 to it, or execute the method within the receiver (if a
CompiledMethod). In the latter case, the method need not reside on the
hierarchy from the receiver's class to Object -- it need not reside at
all in a MethodDictionary, in fact -- but doing bad things will
compromise stability of the Smalltalk virtual machine (and don't blame
anybody but yourself).

This method should not be overridden

[]{#index-perform_003awith_003awith_003a}

**perform: selectorOrMethod with: arg1 with: arg2**

Send the message named selectorOrMethod (if a Symbol) to the receiver,
passing arg1 and arg2 to it, or execute the method within the receiver
(if a CompiledMethod). In the latter case, the method need not reside on
the hierarchy from the receiver's class to Object -- it need not reside
at all in a MethodDictionary, in fact -- but doing bad things will
compromise stability of the Smalltalk virtual machine (and don't blame
anybody but yourself).

This method should not be overridden

[]{#index-perform_003awith_003awith_003awith_003a}

**perform: selectorOrMethod with: arg1 with: arg2 with: arg3**

Send the message named selectorOrMethod (if a Symbol) to the receiver,
passing the other arguments to it, or execute the method within the
receiver (if a CompiledMethod). In the latter case, the method need not
reside on the hierarchy from the receiver's class to Object -- it need
not reside at all in a MethodDictionary, in fact -- but doing bad things
will compromise stability of the Smalltalk virtual machine (and don't
blame anybody but yourself).

This method should not be overridden

[]{#index-perform_003awith_003awith_003awith_003awith_003a}

**perform: selectorOrMethod with: arg1 with: arg2 with: arg3 with:
arg4**

Send the message named selectorOrMethod (if a Symbol) to the receiver,
passing the other arguments to it, or execute the method within the
receiver (if a CompiledMethod). In the latter case, the method need not
reside on the hierarchy from the receiver's class to Object -- it need
not reside at all in a MethodDictionary, in fact -- but doing bad things
will compromise stability of the Smalltalk virtual machine (and don't
blame anybody but yourself).

This method should not be overridden

[]{#index-perform_003awithArguments_003a}

**perform: selectorOrMethod withArguments: argumentsArray**

Send the message named selectorOrMethod (if a Symbol) to the receiver,
passing the elements of argumentsArray as parameters, or execute the
method within the receiver (if a CompiledMethod). In the latter case,
the method need not reside on the hierarchy from the receiver's class to
Object -- it need not reside at all in a MethodDictionary, in fact --
but doing bad things will compromise stability of the Smalltalk virtual
machine (and don't blame anybody but yourself).

This method should not be overridden

[]{#index-primitiveFailed}

**primitiveFailed**

Called when a VM primitive fails

[]{#index-shallowCopy-4}

**shallowCopy**

Returns a shallow copy of the receiver (the instance variables are not
copied)

[]{#index-shouldNotImplement}

**shouldNotImplement**

Called when objects belonging to a class should not answer a selector
defined by a superclass

[]{#index-size-18}

**size**

Answer the number of indexed instance variable in the receiver

[]{#index-subclassResponsibility}

**subclassResponsibility**

Called when a method defined by a class should be overridden in a
subclass

[]{#index-tenure}

**tenure**

Move the object to oldspace.

------------------------------------------------------------------------

::: header
Next: [Object-change and
update](Object_002dchange-and-update.html#Object_002dchange-and-update){accesskey="n"
rel="next"}, Previous: [Object
class-initialization](Object-class_002dinitialization.html#Object-class_002dinitialization){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
