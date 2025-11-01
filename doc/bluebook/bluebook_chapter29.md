---
GENERATOR: Mozilla/4.5 \[en\] (WinNT; I) \[Netscape\]
title: Blue Book Chapter 29
---

[]{#top_of_29}\[[TOC](bluebook_imp_toc.html)\]
\[[26](bluebook_chapter26.html)\] \[[27](bluebook_chapter27.html)\]
\[[28](bluebook_chapter28.html)\] 29 \[[30](bluebook_chapter30.html)\]\

------------------------------------------------------------------------

29\
Formal Specification of the Primitive Methods 

------------------------------------------------------------------------

**[Arithmetic Primitives](#ArithmeticPrimitives29)**

**[Array and Stream Primitives](#ArrayAndStreamPrimitives29)**

**[Storage Management Primitives](#StorageManagementPrimitives29)**

**[Control Primitives](#ControlPrimitives29)**

**[Input/Output Primitives](#InputOutputPrimitives29)**

**[System Primitives](#SystemPrimitives29)**

------------------------------------------------------------------------

When a message is sent, the interpreter usually responds by executing a
Smalltalk `CompiledMethod`. This involves creating a new `MethodContext`
for that `CompiledMethod` and executing its bytecodes until a return
bytecode is encountered. Some messages, however, may be responded to
*primitively*. A primitive response is performed directly by the
interpreter without creating a new context or executing any other
bytecodes. Each primitive response the interpreter can make is described
by a *primitive routine*. A primitive routine removes the message
receiver and arguments from the stack and replaces them with the
appropriate result. Some primitive routines have other effects on the
object memory or on some hardware devices. After a primitive response is
completed, the interpreter proceeds with interpretation of the bytecode
after the send bytecode that caused the primitive to be executed.\
    At any point in its execution, a primitive routine may determine
that a primitive response cannot be made. This may, for example, be due
to a message argument of the wrong class. This is called *primitive
failure*. When a primitive fails, the Smalltalk method associated with
the selector and receiver\'s class will be executed as if the primitive
method did not exist.\
    The table below shows the class-selector pairs associated with each
primitive routine. Some of these class-selector pairs have not appeared
earlier in this book since they are part of the class\'s private
protocol. Some of the primitive routines must meet their specification
in order for the system to function properly. Other primitive routines
are optional; the system will simply perform less efficiently if they
always fail. The optional primitives are marked with an asterisk. The
Smalltalk methods associated with optional primitive routines must do
everything the primitive does. The Smalltalk methods associated with
required primitive routines need only handle the cases for which the
primitive fails.

------------------------------------------------------------------------

  ----------------------------------- --------------------------------------------------------------------------------
  **Primitive Index**                 **Class-Selector Pairs **

  `1`                                 `SmallInteger +`

  `2`                                 `SmallInteger -`

  `3`                                 `SmallInteger <`

  `4`                                 `SmallInteger >`

  `5*`                                `SmallInteger <=`

  `6*`                                `SmallInteger >=`

  `7`                                 `SmallInteger =`

  `8*`                                `SmallInteger ~=`

  `9`                                 `SmallInteger *`

  `10*`                               `SmallInteger / `

  `11*`                               `SmallInteger \\`

  `12*`                               `SmallInteger //`

  `13`                                `SmallInteger quo:`

  `14`                                `SmallInteger bitAnd:`

  `15`                                `SmallInteger bitOr:`

  `16`                                `SmallInteger bitXor:`

  `17`                                `SmallInteger bitShift:`

  `18*`                               `Number @`

  `19`                                 

  `20`                                 

  `21*`                               `Integer + `\
                                      `LargePositiveInteger + `

  `22*`                               `Integer - `\
                                      `LargePositiveInteger - `

  `23*`                               `Integer < `\
                                      `LargePositiveInteger < `

  `24*`                               `Integer > `\
                                      `LargePositiveInteger > `

  `25*`                               `Integer <= `\
                                      `LargePositiveInteger <= `

  `26`                                `Integer >= `\
                                      `LargePositiveInteger >= `

  `27*`                               `Integer = `\
                                      `LargePositiveInteger = `

  `28*`                               `Integer ~= `\
                                      `LargePositiveInteger ~= `

  `29*`                               `Integer * `\
                                      `LargePositiveInteger * `

  `30*`                               `Integer / `\
                                      `LargePositiveInteger / `

  `31*`                               `Integer \\ `\
                                      `LargePositiveInteger \\ `

  `32*`                               `Integer // `\
                                      `LargePositiveInteger // `

  `33*`                               `Integer quo: `\
                                      `LargePositiveInteger quo: `

  `34*`                               `Integer bitAnd: `\
                                      `LargePositiveInteger bitAnd: `

  `35*`                               `Integer bitOr: `\
                                      `LargePositiveInteger bitOr: `

  `36*`                               `Integer bitXor: `\
                                      `LargePositiveInteger bitXor:`

  `37*`                               `Integer bitShift: `\
                                      `LargePositiveInteger bitShift: `

  `38`                                 

  `39`                                 

  `40`                                `SmallInteger asFloat `

  `41`                                `Float + `

  `42`                                `Float - `

  `43`                                `Float < `

  `44`                                `Float >`

  `45*`                               `Float <= `

  `46*`                               `Float >= `

  `47`                                `Float = `

  `48*`                               `Float ~= `

  `49`                                `Float *`

  `50`                                `Float /`

  `51`                                `Float truncated `

  `52*`                               `Float fractionPart `

  `53*`                               `Float exponent `

  `54*`                               `Float timesTwoPower:`

  `55`                                 

  `56`                                 

  `57`                                 

  `58`                                 

  `59`                                 

  `60`                                `LargeNegativeInteger digitAt: `\
                                      `LargePositiveInteger digitAt: `\
                                      `Object at: `\
                                      `Object basicAt: `

  `61`                                `LargeNegativeInteger digitAt:put: `\
                                      `LargePositiveInteger digitAt:put: `\
                                      `Object basicAt:put: `\
                                      `Object at:put: `

  `62`                                `ArrayedCollection size `\
                                      `LargeNegativeInteger digitLength `\
                                      `LargePositiveInteger digitLength `\
                                      `Object basicSize `\
                                      `Object size `\
                                      `String size `

  `63`                                `String at: `\
                                      `String basicAt: `

  `64`                                `String basicAt:put: `\
                                      `String at:put: `

  `65*`                               `ReadStream next `\
                                      `ReadWriteStream next `

  `66*`                               `WriteStream nextPut: `

  `67*`                               `PositionableStream atEnd `

  `68`                                `CompiledMethod objectAt: `

  `69`                                `CompiledMethod objectAt:put: `

  `70`                                `Behavior basicNew `\
                                      `Behavior new `\
                                      `Interval class new `

  `71`                                `Behavior new: `\
                                      `Behavior basicNew: `

  `72`                                `Object become: `

  `73`                                `Object instVarAt: `

  `74`                                `Object instVarAt:put: `

  `75`                                `Object asOop `\
                                      `Object hash `\
                                      `Symbol hash `

  `76`                                `SmallInteger asObject `\
                                      `SmallInteger asObjectNoFail `

  `77`                                `Behavior someInstance `

  `78`                                `Object nextInstance `

  `79`                                `CompiledMethod class newMethod:header: `

  `80*`                               `ContextPart blockCopy: `

  `81`                                `BlockContext value:value:value: `\
                                      `BlockContext value `\
                                      `BlockContext value: `\
                                      `BlockContext value:value: `

  `82`                                `BlockContext valueWithArguments: `

  `83*`                               `Object perform:with:with:with: `\
                                      `Object perform:with: `\
                                      `Object perform:with:with: `\
                                      `Object perform: `

  `84`                                `Object perform:withArguments: `

  `85`                                `Semaphore signal `

  `86`                                `Semaphore wait `

  `87`                                `Process resume `

  `88`                                `Process suspend `

  `89`                                `Behavior flushCache `

  `90*`                               `InputSensor primMousePt `\
                                      `InputState primMousePt `

  `91`                                `InputState primCursorLocPut: `\
                                      `InputState primCursorLocPutAgain: `

  `92`                                `Cursor class cursorLink: `

  `93`                                `InputState primInputSemaphore: `

  `94`                                `InputState primSampleInterval: `

  `95`                                `InputState primInputWord `

  `96`                                `BitBlt copyBitsAgain `\
                                      `BitBlt copyBits `

  `97`                                `SystemDictionary snapshotPrimitive `

  `98`                                `Time class secondClockInto: `

  `99`                                `Time class millisecondClockInto: `

  `100`                               `ProcessorScheduler signal:atMilliseconds:`

  `101`                               `Cursor beCursor `

  `102`                               `DisplayScreen beDisplay `

  `103*`                              `CharacterScanner scanCharactersFrom:to:in:rightX:stopConditions:displaying: `

  `104*`                              `BitBlt drawLoopX:Y: `

  `105*`                              `ByteArray primReplaceFrom:to:with:startingAt: `\
                                      `ByteArray replaceFrom:to:withString:startingAt: `\
                                      `String replaceFrom:to:withByteArray:startingAt: `\
                                      `String primReplaceFrom:to:with;startingAt: `

  `106`                               ` `

  `107`                                

  `108`                                

  `109`                                

  `110`                               `Character = `\
                                      `Object == `

  `111`                               `Object class `

  `112`                               `SystemDictionary coreLeft `

  `113`                               `SystemDictionary quitPrimitive `

  `114`                               `SystemDictionary exitToDebugger `

  `115`                               `SystemDictionary oopsLeft `

  `116`                               `SystemDictionary signal:atOopsLeft:wordsLeft: `

  `117`                               ` `

  `118`                                

  `119`                                

  `120`                                

  `121`                                

  `122`                                

  `123`                                

  `124`                                

  `125`                                

  `126`                                

  `127`                                
  ----------------------------------- --------------------------------------------------------------------------------

  : **The Smalltalk Primitives**

------------------------------------------------------------------------

\
 \
    An example of a primitive method is the response of instances of
`SmallInteger` to messages with selector `+`. If the argument is also an
instance of `SmallInteger`, and the sum of the values of the receiver
and argument is in the range that can be represented by `SmallInteger`,
then the primitive method will remove the receiver and argument from the
stack and replace them with an instance of `SmallInteger` whose value is
the sum. If the argument is not a `SmallInteger` or the sum is out of
range, the primitive will fail and the Smalltalk method associated with
the selector `+` in `SmallInteger` will be executed.\
    The control structures used in the specification of the interpreter
given in this book and the control structures used in a machine language
implementation of the interpreter will probably use different mechanisms
when primitive routines fail to complete. When a failure condition
arises, a machine language primitive routine can decide not to return to
its caller and simply jump to the appropriate place in the interpreter
(usually the place that activates a `CompiledMethod`). However, since
the formal specification is written in Smalltalk, all routines *must*
return to their senders and `Interpreter` must keep track of primitive
success or failure independently of the routine call structure. Part of
the book specification, therefore, is a register called `success` that
is initialized to `true` when a primitive is started and may be set to
`false` if the routine fails. The following two routines set and test
the state of the primitive `success` register.

**success: successValue**

success := successValue & success

**success**

\^success

The following routines set the state of the `success` flag in the two
common cases of initialization before a primitive routine runs and
discovery by a primitive routine that it cannot complete.

**initPrimitive**

success := true

**primitiveFail**

success := false

Many of the primitives manipulate integer quantities, so the interpreter
includes several routines that perform common functions. The
`popInteger` routine is used when a primitive expects a `SmallInteger`
on the top of the stack. If it is a `SmallInteger`, its value is
returned; if not, a primitive failure is signaled.

**popInteger**

\| integerPointer \|

integerPointer := self popStack.

self success: (memory isIntegerObject: integerPointer).

self success

ifTrue: \[\^memory integerValueOf: integerPointer\]

Recall that the `fetchInteger:ofObject:` routine signaled a primitive
failure if the indicated field did not contain a `SmallInteger`. The
`pushInteger:` routine converts a value to a `SmallInteger` and pushes
it on the stack.

**pushInteger: integerValue**

self push: (memory integerObjectOf: integerValue)

Since the largest indexable collections may have 65534 indexable
elements, and `SmallIntegers` can only represent values up to 16383,
primitive routines that deal with indices or sizes must be able to
manipulate `LargePositiveIntegers`. The following two routines convert
back and forth between 16-bit unsigned values and object pointers to
`SmallIntegers` or `LargePositiveIntegers`.

**positive16BitIntegerFor: integerValue**

\| newLargeInteger \|

(self extractBits: 0 to: 1 of: integerValue) = 0

ifTrue: \[\^memory integerObjectOf: integerValue\].

newLargeInteger := memory instantiateClass:
ClassLargePositiveIntegerPointer

withBytes: 2.

memory storeByte: 0

  ofObject: newLargeInteger

  withValue: (self lowByteOf: integerValue).

memory storeByte: 1

  ofObject: newLargeInteger

  withValue: (self highByteOf: integerValue).

\^newLargeInteger

**positive16BitValueOf: integerPointer**

\| value \|

(memory isIntegerObject: integerPointer)

ifTrue: \[\^memory integerValueOf: integerPointer\].

(memory fetchClassOf: integerPointer) = ClassLargePositiveIntegerPointer

ifFalse: \[\^self primitiveFail\].

(memory fetchByteLengthOf: integerPointer) = 2

ifFalse: \[\^self primitiveFail\].

value := memory fetchByte: 1 ofObject: integerPointer.

value := value \* 256 + (memory fetchByte: 0 ofObject: integerPointer).

\^value

There are three ways that a primitive routine can be reached in the
process of interpreting a send-message bytecode.

1.  Some primitive routines are associated with send-special-selector
    bytecodes for certain classes of receiver. These can be reached
    without a message lookup.
2.  The two most common primitive routines (returning `self` or an
    instance variable) can be indicated in the flag value of the header
    a `CompiledMethod`. These are only found after a message lookup has
    produced a `CompiledMethod`, but only the header need be examined.
3.  Most primitive routines are indicated by a number in the header
    extension of a `CompiledMethod`. These are also found after a
    message lookup.

The first path to a primitive routine was represented by the call on
`specialSelectorPrimitiveResponse` in the `sendSpecialSelectorBytecode`
routine. The `specialSelectorPrimitiveResponse` routine selects an
appropriate primitive routine and returns `true` if a primitive response
was successfully made and `false` otherwise. Recall that the
`sendSpecialSelectorBytecode` routine looks up the special selector if
`specialSelectorPrimitiveResponse` returns `false`.

**specialSelectorPrimitiveResponse**

self initPrimitive.

(currentBytecode between: 176 and: 191)

ifTrue: \[self arithmeticSelectorPrimitive\].

(currentBytecode between: 192 and: 207)

ifTrue: \[self commonSelectorPrimitive\].

\^self success

A primitive routine will be accessed by a special arithmetic selector
only if the receiver is a `SmallInteger`. The actual primitive routines
will be described in the section on arithmetic primitives.

**arithmeticSelectorPrimitive**

self success: (memory isIntegerObject: (self stackValue: 1)).

self success

ifTrue: \[currentBytecode = 176 ifTrue: \[\^self primitiveAdd\].

currentBytecode = 177 ifTrue: \[\^self primitiveSubtract\].

currentBytecode = 178 ifTrue: \[\^self primitiveLessThan\].

currentBytecode = 179 ifTrue: \[\^self primitiveGreaterThan\].

currentBytecode = 180 ifTrue: \[\^self primitiveLessOrEqual\].

currentBytecode = 181 ifTrue: \[\^self primitiveGreaterOrEqual\].

currentBytecode = 182 ifTrue: \[\^self primitiveEqual\].

currentBytecode = 183 ifTrue: \[\^self primitiveNotEqual\].

currentBytecode = 184 ifTrue: \[\^self primitiveMultiply\].

currentBytecode = 185 ifTrue: \[\^self primitiveDivide\].

currentBytecode = 186 ifTrue: \[\^self primitiveMod\].

currentBytecode = 187 ifTrue: \[\^self primitiveMakePoint\].

currentBytecode = 188 ifTrue: \[\^self primitiveBitShift\].

currentBytecode = 189 ifTrue: \[\^self primitiveDiv\].

currentBytecode = 190 ifTrue: \[\^self primitiveBitAnd\].

currentBytecode = 191 ifTrue: \[\^self primitiveBitOr\]\]

Only five of the non-arithmetic special selectors invoke primitives
without a message lookup (`==`, `class`, `blockCopy:`, `value`, and
`value:`). The primitive routine for `==` is found in the section on
system primitives and the routine for `class` in storage management
primitives. They are both invoked for any class of receiver. The
routines for `blockCopy:`, `value`, and `value:` are found in the
section on control primitives. The routine for `blockCopy:` will be
invoked if the receiver is a `MethodContext` or a `BlockContext`. The
routines for `value` and `value:` will only be invoked if the receiver
is a `BlockContext`.

**commonSelectorPrimitive**

\| receiverClass \|

argumentCount := self fetchInteger: (currentBytecode - 176) \* 2 + 1

    ofObject: SpecialSelectorsPointer.

receiverClass := memory fetchClassOf: (self stackValue: argumentCount).

currentBytecode = 198 ifTrue: \[\^self primitiveEquivalent\].

currentBytecode = 199 ifTrue: \[\^self primitiveClass\].

currentBytecode = 200

ifTrue: \[self success: (receiverClass = ClassMethodContextPointer)

\| (receiverClass = ClassBlockContextPointer).

\^self success ifTrue: \[self primitiveBlockCopy\]\].

(currentBytecode = 201) \| (currentBytecode = 202)

ifTrue: \[self success: receiverClass = ClassBlockContextPointer.

\^self success ifTrue: \[self primitiveValue\]\].

self primitiveFail

The second and third paths to primitive routines listed above are taken
after a `CompiledMethod` for a message has been found. The presence of a
primitive is detected by the `primitiveResponse` routine called in
`executeNewMethod`. The `primitiveResponse` routine is similar to the
`specialSelectorPrimitiveResponse` routine in that it returns `true` if
a primitive response was successfully made and `false` otherwise. Recall
that the `executeNewMethod` routine activates the `CompiledMethod` has
been looked up if `primitiveResponse` returns `false`.

**primitiveResponse**

\| flagValue thisReceiver offset \|

primitiveIndex = 0

ifTrue: \[flagValue := self flagValueOf: newMethod.

flagValue = 5

ifTrue: \[self quickReturnSelf.

\^true\].

flagValue = 6

ifTrue: \[self quickInstanceLoad.

\^true\].

\^false\]

ifFalse: \[self initPrimitive.

self dispatchPrimitives.

\^self success\]

Flag values of 5 and 6 reach the two most commonly found primitives, a
simple return of `self` and a simple return of one of the receiver\'s
instance variables. Returning `self` is a no-op as far as the
interpreter is concerned since `self's` object pointer occupies the same
place on the stack as the message receiver that it should occupy as the
message response.

**quickReturnSelf**

Returning an instance variable of the receiver is almost as easy.

**quickInstanceLoad**

\| thisReceiver fieldIndex \|

thisReceiver := self popStack.

fieldIndex := self fieldIndexOf: newMethod.

self push: (memory fetchPointer: fieldIndex

ofObject: thisReceiver)

The six types of primitives in the formal specification deal with
arithmetic, subscripting and streaming, storage management, control
structures, input/output, and general system access. These correspond to
six ranges of primitive indices. A range of primitive indices has been
reserved for implementation-private primitive routines. They may be
assigned any meaning, but cannot be depended upon from interpreter to
interpreter. Since these are not part of the specification, they cannot
be described here.

**dispatchPrimitives**

primitiveIndex \< 60 ifTrue: \[\^self dispatchArithmeticPrimitives\].

primitiveIndex \< 68 ifTrue: \[\^self
dispatchSubscriptAndStreamPrimitives\].

primitiveIndex \< 80 ifTrue: \[\^self
dispatchStorageManagementPrimitives\].

primitiveIndex \< 90 ifTrue: \[\^self dispatchControlPrimitives\].

primitiveIndex \< 110 ifTrue: \[\^self dispatchInputOutputPrimitives\].

primitiveIndex \< 128 ifTrue: \[\^self dispatchSystemPrimitives\].

primitiveIndex \< 256 ifTrue: \[\^self dispatchPrivatePrimitives\]

------------------------------------------------------------------------

[]{#ArithmeticPrimitives29}**Arithmetic Primitives**

------------------------------------------------------------------------

There are three sets of arithmetic primitive routines, one for
`SmallIntegers`, one for large integers (`LargePositiveIntegers` and
`LargeNegativeIntegers`), and one for `Floats`. The primitives for
`SmallIntegers` and `Floats` must be implemented, the primitives for
large integers are optional.

**dispatchArithmeticPrimitives**

primitiveIndex \< 20 ifTrue: \[\^self dispatchIntegerPrimitives\].

primitiveIndex \< 40 ifTrue: \[\^self dispatchLargeIntegerPrimitives\].

primitiveIndex \< 60 ifTrue: \[\^self dispatchFloatPrimitives\]

The first set of arithmetic primitive routines all pop a receiver and
argument off the stack and fail if they are not both `SmallIntegers`.
The routines then push on the stack either the integral result of a
computation or the Boolean result of a comparison. The routines that
produce an integral result fail if the value cannot be represented as a
`SmallInteger`.

**dispatchIntegerPrimitives**

primitiveIndex = 1 ifTrue: \[\^self primitiveAdd\].

primitiveIndex = 2 ifTrue: \[\^self primitiveSubtract\].

primitiveIndex = 3 ifTrue: \[\^self primitiveLessThan\].

primitiveIndex = 4 ifTrue: \[\^self primitiveGreaterThan\].

primitiveIndex = 5 ifTrue: \[\^self primitiveLessOrEqual\].

primitiveIndex = 6 ifTrue: \[\^self primitiveGreaterOrEqual\].

primitiveIndex = 7 ifTrue: \[\^self primitiveEqual\].

primitiveIndex = 8 ifTrue: \[\^self primitiveNotEqual\].

primitiveIndex = 9 ifTrue: \[\^self primitiveMultiply\].

primitiveIndex = 10 ifTrue: \[\^self primitiveDivide\].

primitiveIndex = 11 ifTrue: \[\^self primitiveMod\].

primitiveIndex = 12 ifTrue: \[\^self primitiveDiv\].

primitiveIndex = 13 ifTrue: \[\^self primitiveQuo\].

primitiveIndex = 14 ifTrue: \[\^self primitiveBitAnd\].

primitiveIndex = 15 ifTrue: \[\^self primitiveBitOr\].

primitiveIndex = 16 ifTrue: \[\^self primitiveBitXor\].

primitiveIndex = 17 ifTrue: \[\^self primitiveBitShift\].

primitiveIndex = 18 ifTrue: \[\^self primitiveMakePoint\]

The `primitiveAdd`, `primitiveSubtract`, and `primitiveMultiply`
routines are all identical except for the arithmetic operation used, so
only `primitiveAdd` routine will be shown here.

**primitiveAdd**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success

ifTrue: \[integerResult := integerReceiver + integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

The primitive routine for division (associated with the selector `/`) is
different than the other three arithmetic primitives since it only
produces a result if the division is exact, otherwise the primitive
fails. This primitive, and the next three that have to do with rounding
division, all fail if their argument is 0.

**primitiveDivide**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success: integerArgument \~= 0.

self success: integerReceiver \\\\ integerArgument = 0.

self success

ifTrue: \[integerResult := integerReceiver // integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self push: (memory integerObjectOf: integerResult)\]

ifFalse: \[self unPop: 2\]

The primitive routine for the modulo function (associated with the
selector `\\`) gives the remainder of a division where the quotient is
always rounded down (toward negative infinity).

**primitiveMod**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success: integerArgument \~= 0.

self success

ifTrue: \[integerResult := integerReceiver \\\\ integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

There are two primitive routines for rounding division (associated with
the selectors `//` and `quo:`). The result of `//` is always rounded
down (toward negative infinity).

**primitiveDiv**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success: integerArgument \~= 0.

self success

ifTrue: \[integerResult := integerReceiver // integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

The result of `quo:` is truncated (rounded toward zero).

**primitiveQuo**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success: integerArgument \~= 0.

self success

ifTrue: \[integerResult := integerReceiver quo: integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

The `primitiveEqual`, `primitiveNotEqual`, `primitiveLessThan`,
`primitiveLessOrEqual`, `primitiveGreaterThan`, and
`primitiveGreaterOrEqual` routines are all identical except for the
comparison operation used, so only the `primitiveEqual` routine will be
shown here.

**primitiveEqual**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success

ifTrue: \[integerReceiver = integerArgument

ifTrue: \[self push: TruePointer\]

ifFalse: \[self push: FalsePointer\]\]

ifFalse: \[self unPop: 2\]

The `primitiveBitAnd`, `primitiveBitOr`, and `primitiveBitXor` routines
perform logical operations on the two\'s-complement binary
representations of `SmallInteger` values. They are identical except for
the logical operation used, so only the `primitiveBitAnd` routine will
be shown here.

**primitiveBitAnd**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success

ifTrue: \[integerResult := integerReceiver bitAnd: integerArgument\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

The primitive routine for shifting (associated with the selector
`bitShift:`) returns a `SmallInteger` whose value represented in
two\'s-complement is the receiver\'s value represented in
two\'s-complement shifted left by the number of bits indicated by the
argument. Negative arguments shift right. Zeros are shifted in from the
right in left shifts. The sign bit is extended in right shifts. This
primitive fails if the correct result cannot be represented as a
`SmallInteger`.

**primitiveBitShift**

\| integerReceiver integerArgument integerResult \|

integerArgument := self popInteger.

integerReceiver := self popInteger.

self success

ifTrue: \[integerResult := integerReceiver bitShift: integerArgument.

self success: (memory isIntegerValue: integerResult)\].

self success

ifTrue: \[self pushInteger: integerResult\]

ifFalse: \[self unPop: 2\]

The primitive routine associated with the selector `@` returns a new
`Point` whose `x` value is the receiver and whose `y` value is the
argument.

**primitiveMakePoint**

\| integerReceiver integerArgument pointResult \|

integerArgument := self popStack.

integerReceiver := self popStack.

self success: (memory isIntegerValue: integerReceiver).

self success: (memory isIntegerValue: integerArgument).

self success

ifTrue: \[pointResult := memory instantiateClass: ClassPointPointer

       withPointers: ClassPointSize.

memory storePointer: XIndex

ofObject: pointResult

withValue: integerReceiver.

memory storePointer: YIndex

ofObject: pointResult

withValue: integerArgument.

self push: pointResult\]

ifFalse: \[self unPop: 2\]

**initializePointIndices**

XIndex := 0.

YIndex := 1.

ClassPointSize := 2

The primitive indices 21 to 37 are the same as primitives 1 to 17 except
that they perform their operations on large integers (instances of
`LargePositiveInteger` and `LargeNegativeInteger`). There are adequate
Smalltalk implementations for all of these operations so the primitive
routines are optional and will not be specified in this chapter. To
implement them, the corresponding Smalltalk methods should be translated
into machine language routines.

**dispatchLargeIntegerPrimitives**

self primitiveFail

Instances of `Float` are represented in IEEE single-precision (32-bit)
format. This format represents a floating point quantity as a number
between one and two, a power of two, and a sign. A `Float` is a
word-size, nonpointer object. The most significant bit of the first
field indicates the sign of the number (1 means negative). The next
eight most significant bits of the first field are the 8-bit exponent of
two biased by 127 (0 means an exponent of -127, 128 means an exponent of
1, and so on). The seven least significant bits of the first field are
the seven most significant bits of the fractional part of the number
between one and two. The fractional part is 23 bits long and its 16
least significant bits are the contents of the second field of the
`Float`. So a `Float` whose fields are

**SEEEEEEE EFFFFFFF FFFFFFFF FFFFFFFF**

represents the value

-1**^S^** \* 2^**E**-127^ \* 1.**F**

0 is represented as both fields = 0. The floating point primitives fail
if the argument is not an instance of `Float` or if the result cannot be
represented as a `Float`. This specification of the Smalltalk-80 virtual
machine does not specifically include the parts of the IEEE standard
other than the representation of floating point numbers. The
implementation of routines that perform the necessary operations on
floating point values is left to the implementer.\
    The `primitiveAsFloat` routine converts its `SmallInteger` receiver
into a `Float`. The routines for primitives 41 through 50 perform the
same operations as 1 through 10 or 21 through 30, except that they
operate on `Floats`. The `primitiveTruncated` routine returns a
`SmallInteger` equal to the value of the receiver without any fractional
part. It fails if its truncated value cannot be represented as a
`SmallInteger`. The `primitiveFractionalPart` returns the difference
between the receiver and its truncated value. The `primitiveExponent`
routine returns the exponent of the receiver and the
`primitiveTimesTwoPower` routine increases the exponent by an amount
specified by the argument.

**dispatchFloatPrimitives**

primitiveIndex = 40 ifTrue: \[\^self primitiveAsFloat\].

primitiveIndex = 41 ifTrue: \[\^self primitiveFloatAdd\].

primitiveIndex = 42 ifTrue: \[\^self primitiveFloatSubtract\].

primitiveIndex = 43 ifTrue: \[\^self primitiveFloatLessThan\].

primitiveIndex = 44 ifTrue: \[\^self primitiveFloatGreaterThan\].

primitiveIndex = 45 ifTrue: \[\^self primitiveFloatLessOrEqual\].

primitiveIndex = 46 ifTrue: \[\^self primitiveFloatGreaterOrEqual\].

primitiveIndex = 47 ifTrue: \[\^self primitiveFloatEqual\].

primitiveIndex = 48 ifTrue: \[\^self primitiveFloatNotEqual\].

primitiveIndex = 49 ifTrue: \[\^self primitiveFloatMultiply\].

primitiveIndex = 50 ifTrue: \[\^self primitiveFloatDivide\].

primitiveIndex = 51 ifTrue: \[\^self primitiveTruncated\].

primitiveIndex = 52 ifTrue: \[\^self primitiveFractionalPart\].

primitiveIndex = 53 ifTrue: \[\^self primitiveExponent\].

primitiveIndex = 54 ifTrue: \[\^self primitiveTimesTwoPower\]

------------------------------------------------------------------------

[]{#ArrayAndStreamPrimitives29}**Array and Stream Primitives**

------------------------------------------------------------------------

The second set of primitive routines are for manipulating the indexable
fields of objects both directly, by subscripting, and indirectly, by
streaming. These routines make use of the 16-bit positive integer
routines, since the limit on indexable fields is 65534.

**dispatchSubscriptAndStreamPrimitives**

primitiveIndex = 60 ifTrue: \[\^self primitiveAt\].

primitiveIndex = 61 ifTrue: \[\^self primitiveAtPut\].

primitiveIndex = 62 ifTrue: \[\^self primitiveSize\].

primitiveIndex = 63 ifTrue: \[\^self primitiveStringAt\].

primitiveIndex = 64 ifTrue: \[\^self primitiveStringAtPut\].

primitiveIndex = 65 ifTrue: \[\^self primitiveNext\].

primitiveIndex = 66 ifTrue: \[\^self primitiveNextPut\].

primitiveIndex = 67 ifTrue: \[\^self primitiveAtEnd\]

The following routines are used to check the bounds on subscripting
operations and to perform the subscripting accesses. They determine
whether the object being indexed contains pointers, 16-bit integer
values, or 8-bit integer values, in its indexable fields. The
`checkIndexableBoundsOf:in:` routine takes a one-relative index and
determines whether it is a legal subscript of an object. It must take
into account any fixed fields.

**checkIndexableBoundsOf: index in: array**

\| class \|

class := memory fetchClassOf: array.

self success: index \>= 1.

self success: index + (self fixedFieldsOf: class) \<= (self lengthOf:
array)

**lengthOf: array**

(self isWords: (memory fetchClassOf: array))

ifTrue: \[\^memory fetchWordLengthOf: array\]

ifFalse: \[\^memory fetchByteLengthOf: array\]

The `subscript:with:` and `subscript:with:storing:` routines assume that
the number of fixed fields has been added into the index, so they use it
as a one-relative index into the object as a whole.

**subscript: array with: index**

\| class value \|

class := memory fetchClassOf: array.

(self isWords: class)

ifTrue: \[(self isPointers: class)

ifTrue: \[\^memory fetchPointer: index - 1

      ofObject: array\]

ifFalse: \[value := memory fetchWord: index - 1

       ofObject: array.

\^self positive16BitIntegerFor: value\]\]

ifFalse: \[value := memory fetchByte: index - 1

       ofObject: array.

\^memory integerObjectOf: value\]

**subscript: array with: index storing: value**

\| class \|

class := memory fetchClassOf: array.

(self isWords: class)

ifTrue: \[(self isPointers: class)

ifTrue: \[\^memory storePointer: index - 1

       ofObject: array

       withValue: value\]

ifFalse: \[self success: (memory isIntegerObject: value).

self success ifTrue:

\[\^memory storeWord: index - 1

     ofObject: array

     withValue: (self positive16BitValueOf: value)\]\]\]

ifFalse: \[self success: (memory isIntegerObject: value).

self success ifTrue:

\[\^memory storeByte: index - 1

     ofObject: array

     withValue: (self lowByteOf: (memory integerValueOf: value))\]\]

The `primitiveAt` and `primitiveAtPut` routines simply fetch or store
one of the indexable fields of the receiver. They fail if the index is
not a `SmallInteger` or if it is out of bounds.

**primitiveAt**

\| index array arrayClass result \|

index := self positive16BitValueOf: self popStack.

array := self popStack.

arrayClass := memory fetchClassOf: array.

self checkIndexableBoundsOf: index in: array.

self success

ifTrue: \[index := index + (self fixedFieldsOf: arrayClass).

result := self subscript: array with: index\].

self success

ifTrue: \[self push: result\]

ifFalse: \[self unPop: 2\]

The `primitiveAtPut` routine also fails if the receiver is not a pointer
type and the second argument is not an 8-bit (for byte-indexable
objects) or 16-bit (for word-indexable objects) positive integer. The
primitive routine returns the stored value as its value.

**primitiveAtPut**

\| array index arrayClass value result \|

value := self popStack.

index := self positive16BitValueOf: self popStack.

array := self popStack.

arrayClass := memory fetchClassOf: array.

self checkIndexableBoundsOf: index in: array.

self success

ifTrue: \[index := index + (self fixedFieldsOf: arrayClass).

self subscript: array with: index storing: value\].

self success

ifTrue: \[self push: value\]

ifFalse: \[self unPop: 3\]

The `primitiveSize` routine returns the number of indexable fields the
receiver has (i.e., the largest legal subscript).

**primitiveSize**

\| array class length \|

array := self popStack.

class := memory fetchClassOf: array.

length := self positive16BitIntegerFor: (self lengthOf: array) - (self
fixedFieldsOf: class).

self success

ifTrue: \[self push: length\]

ifFalse: \[self unPop: 1\]

The `primitiveStringAt` and `primitiveStringAtPut` routines are special
responses to the `at:` and `at:put:` messages by instances of `String`.
A `String` actually stores 8-bit numbers in byte-indexable fields, but
it communicates through the `at:` and `at:put:` messages with instances
of `Character`. A `Character` has a single instance variable that holds
a `SmallInteger`. The value of the `SmallInteger` returned from the
`at:` message is then stored in the indicated field of the `String`. The
`primitiveStringAt` routine always returns the same instance of
`Character` for any particular value. It gets the `Characters` from an
`Array` in the object memory that has a guaranteed object pointer called
`CharacterTablePointer`.

**primitiveStringAt**

\| index array ascii character \|

index := self positive16BitValueOf: self popStack.

array := self popStack.

self checkIndexableBoundsOf: index in: array.

self success

ifTrue: \[ascii := memory integerValueOf: (self subscript: array with:
index).

character := memory fetchPointer: ascii

 ofObject: CharacterTablePointer\].

self success

ifTrue:\[self push: character\]

ifFalse: \[self unPop: 2\]

**initializeCharacterIndex**

CharacterValueIndex := 0

The `primitiveStringAtPut` routine stores the value of a `Character` in
one of the receiver\'s indexable bytes. It fails if the second argument
of the `at:put:` message is not a `Character`.

**primitiveStringAtPut**

\| index array ascii character \|

character := self popStack.

index := self positive16BitValueOf: self popStack.

array := self popStack.

self checkIndexableBoundsOf: index in: array.

self success: (memory fetchClassOf: character) = ClassCharacterPointer.

self success

ifTrue: \[ascii := memory fetchPointer: CharacterValueIndex

      ofObject: character.

self subscript: array with: index storing: ascii\].

self success

ifTrue: \[self push: character\]

ifFalse: \[self unPop: 3\]

The `primitiveNext`, `primitiveNextPut`, and `primitiveAtEnd` routines
are optional primitive versions of the Smalltalk code for the `next`,
`nextPut:`, and `atEnd` messages to streams. The `primitiveNext` and
`primitiveNextPut` routines only work if the object being streamed is an
`Array` or a `String`.

**initializeStreamIndices**

StreamArrayIndex := 0.

StreamIndexIndex := 1.

StreamReadLimitIndex := 2.

StreamWriteLimitIndex := 3

**primitiveNext**

\| stream index limit array arrayClass result ascii \|

stream := self popStack.

array := memory fetchPointer: StreamArrayIndex

    ofObject: stream.

arrayClass := memory fetchClassOf: array.

index := self fetchInteger: StreamIndexIndex

ofObject: stream.

limit := self fetchInteger: StreamReadLimitIndex

         ofObject: stream.

self success: index \< limit.

self success: (arrayClass \~= ClassArrayPointer) \| (arrayClass =
ClassStringPointer).

self checkIndexableBoundsOf: index + 1

in: array.

self success

ifTrue: \[index := index + 1.

result := self subscript: array

with: index\].

self success

ifTrue: \[self storeInteger: StreamIndexIndex

ofObject: stream

withValue: index\].

self success

ifTrue: \[arrayClass = ClassArrayPointer

ifTrue: \[self push: result\]

ifFalse: \[ascii := memory integerValueOf: result.

self push: (memory fetchPointer: ascii

ofObject: CharacterTablePointer)\]\]

ifFalse: \[self unPop: 1\]

**primitiveNextPut**

\| value stream index limit array arrayClass result ascii \|

value := self popStack.

stream := self popStack.

array := memory fetchPointer: StreamArrayIndex

     ofObject: stream.

arrayClass := memory fetchClassOf: array.

index := self fetchInteger: StreamIndexIndex

ofObject: stream.

limit := self fetchInteger: StreamWriteLimitIndex

        ofObject: stream.

self success: index \< limit.

self success: (arrayClass = ClassArrayPointer) \| (arrayClass =
ClassStringPointer).

self checkIndexableBoundsOf: index + 1

in: array.

self success

ifTrue: \[index := index + 1.

arrayClass = ClassArrayPointer

ifTrue: \[self subscript: array

with: index

storing: value\]

ifFalse: \[ascii := memory fetchPointer: CharacterValueIndex

        ofObject: value.

self subscript: array

with: index

storing: ascii\]\].

self success

ifTrue: \[self storeInteger: StreamIndexIndex

ofObject: stream

withValue: index\].

self success

ifTrue: \[self push: value\]

ifFalse: \[self unPop: 2\]

**primitiveAtEnd**

\| stream array arrayClass index limit \|

stream := self popStack.

array := memory fetchPointer: StreamArrayIndex

     ofObject: stream.

arrayClass := memory fetchClassOf: array.

index := self fetchInteger: StreamIndexIndex

ofObject: stream.

limit := self fetchInteger: StreamReadLimitIndex

        ofObject: stream.

self success: (arrayClass = ClassArrayPointer) \| (arrayClass =
ClassStringPointer).

self success

ifTrue: \[(index \>= limit)

ifTrue: \[self push: TruePointer\]

ifFalse: \[self push: FalsePointer\]\]

ifFalse: \[self unPop: 1\]

------------------------------------------------------------------------

[]{#StorageManagementPrimitives29}**Storage Management Primitives**

------------------------------------------------------------------------

The storage management primitive routines manipulate the representation
of objects. They include primitives for manipulating object pointers,
accessing fields, creating new instances of a class, and enumerating the
instances of a class.

**dispatchStorageManagementPrimitives**

primitiveIndex = 68 ifTrue: \[\^self primitiveObjectAt\].

primitiveIndex = 69 ifTrue: \[\^self primitiveObjectAtPut\].

primitiveIndex = 70 ifTrue: \[\^self primitiveNew\].

primitiveIndex = 71 ifTrue: \[\^self primitiveNewWithArg\].

primitiveIndex = 72 ifTrue: \[\^self primitiveBecome\].

primitiveIndex = 73 ifTrue: \[\^self primitiveInstVarAt\].

primitiveIndex = 74 ifTrue: \[\^self primitiveInstVarAtPut\].

primitiveIndex = 75 ifTrue: \[\^self primitiveAsOop\].

primitiveIndex = 76 ifTrue: \[\^self primitiveAsObject\].

primitiveIndex = 77 ifTrue: \[\^self primitiveSomeInstance\].

primitiveIndex = 78 ifTrue: \[\^self primitiveNextInstance\].

primitiveIndex = 79 ifTrue: \[\^self primitiveNewMethod\]

The `primitiveObjectAt` and `primitiveObjectAtPut` routines are
associated with the `objectAt:` and `objectAt:put:` messages in
`CompiledMethod`. They provide access to the object pointer fields of
the receiver (the method header and the literals) from Smalltalk. The
header is accessed with an index of 1 and the literals are accessed with
indices 2 through the number of literals plus 1. These messages are used
primarily by the compiler.

**primitiveObjectAt**

\| thisReceiver index \|

index := self popInteger.

thisReceiver := self popStack.

self success: index \> 0.

self success: index \<= (self objectPointerCountOf: thisReceiver).

self success

ifTrue: \[self push: (memory fetchPointer: index - 1

ofObject: thisReceiver)\]

ifFalse: \[self unPop: 2\]

**primitiveObjectAtPut**

\| thisReceiver index newValue \|

newValue := self popStack.

index := self popInteger.

thisReceiver := self popStack.

self success: index \> 0.

self success: index \<= (self objectPointerCountOf: thisReceiver).

self success

ifTrue: \[memory storePointer: index - 1

      ofObject: thisReceiver

      withValue: newValue.

self push: newValue\]

ifFalse: \[self unPop: 3\]

The `primitiveNew` routine creates a new instance of the receiver (a
class) without indexable fields. The primitive fails if the class is
indexable.

**primitiveNew**

\| class size \|

class := self popStack.

size := self fixedFieldsOf: class.

self success: (self isIndexable: class) == false.

self success

ifTrue: \[(self isPointers: class)

ifTrue: \[self push: (memory instantiateClass: class

withPointers: size)\]

ifFalse: \[self push: (memory instantiateClass: class

 withWords: size)\]\]

ifFalse: \[self unPop: 1\]

The `primitiveNewWithArg` routine creates a new instance of the receiver
(a class) with the number of indexable fields specified by the integer
argument. The primitive fails if the class is not indexable.

**primitiveNewWithArg**

\| size class \|

size := self positive16BitValueOf: self popStack.

class := self popStack.

self success: (self isIndexable: class).

self success

ifTrue: \[size := size + (self fixedFieldsOf: class).

(self isPointers: class)

ifTrue: \[self push: (memory instantiateClass: class

withPointers: size)\]

ifFalse: \[(self isWords: class)

ifTrue: \[self push: (memory instantiateClass: class

withWords: size)\]

ifFalse: \[self push: (memory instantiateClass: class

withBytes: size)\]\]\]

ifFalse: \[self unPop: 2\]

The `primitiveBecome` routine swaps the instance pointers of the
receiver and argument. This means that all objects that used to point to
the receiver now point to the argument and vice versa.

**primitiveBecome**

\| thisReceiver otherPointer \|

otherPointer := self popStack.

thisReceiver := self popStack.

self success: (memory isIntegerObject: otherPointer) not.

self success: (memory isIntegerObject: thisReceiver) not.

self success

ifTrue: \[memory swapPointersOf: thisReceiver and: otherPointer.

self push: thisReceiver\]

ifFalse: \[self unPop: 2\]

The `primitiveInstVarAt` and `primitiveInstVarAtPut` routines are
associated with the `instVarAt:` and `instVarAt:put:` messages in
`Object`. They are similar to `primitiveAt` and `primitiveAtPut` except
that the numbering of fields starts with the fixed fields (corresponding
to named instance variables) instead of with the indexable fields. The
indexable fields are numbered starting with one more than the number of
fixed fields. These routines need a different routine to check the
bounds of the subscript.

**checkInstanceVariableBoundsOf: index in: object**

\| class \|

class := memory fetchClassOf: object.

self success: index \>= 1.

self success: index \<= (self lengthOf: object)

**primitiveInstVarAt**

\| this Receiver index value \|

index := self popInteger.

thisReceiver := self popStack.

self checkInstanceVariableBoundsOf: index

in: thisReceiver.

self success

ifTrue: \[value := self subscript: thisReceiver

with: index\].

self success

ifTrue: \[self push: value\]

ifFalse: \[self unPop: 2\]

**primitiveInstVarAtPut**

\| thisReceiver index newValue realValue \|

newValue := self popStack.

index := self popInteger.

thisReceiver := self popStack.

self checkInstanceVariableBoundsOf: index

in: thisReceiver.

self success

ifTrue: \[ self subscript: thisReceiver

with: index

storing: newValue\].

self success

ifTrue: \[self push: newValue\]

ifFalse: \[self unPop: 3\]

The `primitiveAsOop` routine produces a `SmallInteger` whose value is
half of the receiver\'s object pointer (interpreting object pointers as
16-bit signed quantities). The primitive only works for
non-`SmallInteger` receivers. Since non-`SmallInteger` object pointers
are even, no information in the object pointer is lost. Because of the
encoding of `SmallIntegers`, the halving operation can be performed by
setting the least significant bit of the receiver\'s object pointer.

**primitiveAsOop**

\| thisReceiver \|

thisReceiver := self popStack.

self success: (memory isIntegerObject: thisReceiver) == false.

self success

ifTrue: \[self push: (thisReceiver bitOr: 1)\]

ifFalse: \[self unPop: 1\]

The `primitiveAsObject` routine performs the inverse operation of
`primitiveAsOop`. It only works for `SmallInteger` receivers (it is
associated with the `asObject` message in `SmallInteger`). It produces
the object pointer that is twice the receiver\'s value. The primitive
fails if there is no object for that pointer.

**primitiveAsObject**

\| thisReceiver newOop \|

thisReceiver := self popStack.

newOop := thisReceiver bitAnd: 16rFFFE.

self success: (memory isIntegerObject: thisReceiver).

self success

ifTrue: \[self push: newOop\]

ifFalse: \[self unPop: 1\]

The `primitiveSomeInstance` and `primitiveNextInstance` routines allow
for the enumeration of the instances of a class. They rely on the
ability of the object memory to define an ordering on object pointers,
to find the first instance of a class in that ordering, and, given an
object pointer, to find the next instance of the same class.

**primitiveSomeInstance**

\| class \|

class := self popStack.

(memory instancesOf: class)

ifTrue: \[self push: (memory initialInstanceOf: class)\]

ifFalse: \[self primitiveFail\]

**primitiveNextInstance**

\| object \|

object := self popStack.

(memory isLastInstance: object)

ifTrue: \[self primitiveFail\]

ifFalse: \[self push: (memory instanceAfter: object)\]

The `primitiveNewMethod` routine is associated with the
`newMethod:header:` message in `CompiledMethod` class. Instances of
`CompiledMethod` are created with a special message. Since the part of a
`CompiledMethod` that contains pointers instead of bytes is indicated in
the header, all `CompiledMethods` must have a valid header. Therefore,
`CompiledMethods` are created with a message (`newMethod:header:`) that
takes the number of bytes as the first argument and the header as the
second argument. The header, in turn, indicates the number of pointer
fields.

**primitiveNewMethod**

\| header bytecodeCount class size \|

header := self popStack.

bytecodeCount := self popInteger.

class := self popStack.

size := (self literalCountOfHeader: header) + 1 \* 2 + bytecodeCount.

self push: (memory instantiateClass: class

       withBytes: size)

------------------------------------------------------------------------

[]{#ControlPrimitives29}**Control Primitives**

------------------------------------------------------------------------

The control primitives provide the control structures not provided by
the bytecodes. They include support for the behavior of `BlockContexts`,
`Processes`, and `Semaphores`. They also provide for messages with
parameterized selectors.

**dispatchControlPrimitives**

primitiveIndex = 80 ifTrue: \[\^self primitiveBlockCopy\].

primitiveIndex = 81 ifTrue: \[\^self primitiveValue\].

primitiveIndex = 82 ifTrue: \[\^self primitiveValueWithArgs\].

primitiveIndex = 83 ifTrue: \[\^self primitivePerform\].

primitiveIndex = 84 ifTrue: \[\^self primitivePerformWithArgs\].

primitiveIndex = 85 ifTrue: \[\^self primitiveSignal\].

primitiveIndex = 86 ifTrue: \[\^self primitiveWait\].

primitiveIndex = 87 ifTrue: \[\^self primitiveResume\].

primitiveIndex = 88 ifTrue: \[\^self primitiveSuspend\].

primitiveIndex = 89 ifTrue: \[\^self primitiveFlushCache\]

The `primitiveBlockCopy` routine is associated with the `blockCopy:`
message in both `BlockContext` and `MethodContext`. This message is only
produced by the compiler. The number of block arguments the new
`BlockContext` takes is passed as the argument. The `primitiveBlockCopy`
routine creates a new instance of `BlockContext`. If the receiver is a
`MethodContext`, it becomes the new `BlockContext's` home context. If
the receiver is a `BlockContext`, its home context is used for the new
`BlockContext's` home context.

**primitiveBlockCopy**

\| context methodContext blockArgumentCount newContext initialIP
contextSize \|

blockArgumentCount := self popStack.

context := self popStack.

(self isBlockContext: context)

ifTrue: \[methodContext := memory fetchPointer: HomeIndex

ofObject: context\]

ifFalse: \[methodContext := context\].

contextSize := memory fetchWordLengthOf: methodContext.

newContext := memory instantiateClass: ClassBlockContextPointer

  withPointers: contextSize.

initialIP := memory integerObjectOf: instructionPointer + 3.

memory storePointer: initialIPIndex

 ofObject: newContext

 withValue: initialIP.

memory storePointer: InstructionPointerIndex

 ofObject: newContext

 withValue: initialIP.

self storeStackPointerValue: 0

       inContext: newContext.

memory storePointer: BlockArgumentCountIndex

 ofObject: newContext

 withValue: blockArgumentCount.

memory storePointer: HomeIndex

 ofObject: newContext

 withValue: methodContext.

self push: newContext

The `primitiveValue` routine is associated with all \"`value`\" messages
in `BlockContext` (`value`, `value:`, `value:value:`, and so on). It
checks that the receiver takes the same number of block arguments that
the \"`value`\" message did and then transfers them from the active
context\'s stack to the receiver\'s stack. The primitive fails if the
number of arguments do not match. The `primitiveValue` routine also
stores the active context in the receiver\'s caller field and
initializes the receiver\'s instruction pointer and stack pointer. After
the receiver has been initialized, it becomes the active context.

**primitiveValue**

\| blockContext blockArgumentCount initialIP \|

blockContext := self stackValue: argumentCount.

blockArgumentCount := self argumentCountOfBlock: blockContext.

self success: argumentCount = blockArgumentCount.

self success

ifTrue: \[self transfer: argumentCount

fromIndex: stackPointer - argumentCount + 1

ofObject: activeContext

toIndex: TempFrameStart

ofObject: blockContext.

self pops argumentCount + 1.

initialIP := memory fetchPointer: initialIPIndex

ofObject: blockContext.

memory storePointer: InstructionPointerIndex

 ofObject: blockContext

 withValue: initialIP.

self storeStackPointerValue: argumentCount

       inContext: blockContext.

memory storePointer: CallerIndex

 ofObject: blockContext

 withValue: activeContext.

self newActiveContext: blockContext\]

The `primitiveValueWithArgs` routine is associated with the
`valueWithArguments:` messages in `BlockContext`. It is basically the
same as the `primitiveValue` routine except that the block arguments
come in a single `Array` argument to the `valueWithArguments:` message
instead of as multiple arguments to the \"`value`\" message.

**primitiveValueWithArgs**

\| argumentArray blockContext blockArgumentCount

arrayClass arrayArgumentCount initialIP \|

argumentArray := self popStack.

blockContext := self popStack.

blockArgumentCount := self argumentCountOfBlock: blockContext.

arrayClass := memory fetchClassOf: argumentArray.

self success: (arrayClass = ClassArrayPointer).

self success

ifTrue: \[arrayArgumentCount := memory fetchWordLengthOf: argumentArray.

self success: arrayArgumentCount = blockArgumentCount\].

self success

ifTrue: \[self transfer: arrayArgumentCount

fromIndex: 0

ofObject: argumentArray

toIndex: TempFrameStart

ofObject: blockContext.

initialIP := memory fetchPointer: initialIPIndex

        ofObject: blockContext.

memory storePointer: InstructionPointerIndex

 ofObject: blockContext

 withValue: initialIP.

self storeStackPointerValue: arrayArgumentCount

       inContext: blockContext.

memory storePointer: CallerIndex

 ofObject: blockContext

 withValue: activeContext.

self newActiveContext: blockContext\]

ifFalse: \[self unPop: 2\]

The `primitivePerform` routine is associated with all \"`perform`\"
messages in `Object` (`perform:`, `perform:with:`, `perform:with:with:`,
and so on). It is equivalent to sending a message to the receiver whose
selector is the first argument of and whose arguments are the remaining
arguments. It is, therefore, similar to the
`sendSelector:argumentCount:` routine except that it must get rid of the
selector from the stack before calling `executeNewMethod` and it must
check that the `CompiledMethod` it finds takes one less argument that
the \"`perform`\" message did. The primitive fails if the number of
arguments does not match.

**primitivePerform**

\| performSelector newReceiver selectorIndex \|

performSelector := messageSelector.

messageSelector := self stackValue: argumentCount - 1.

newReceiver := self stackValue: argumentCount.

self lookupMethodInClass: (memory fetchClassOf: newReceiver).

self success: (self argumentCountOf: newMethod) = (argumentCount - 1).

self success

ifTrue: \[selectorIndex := stackPointer - argumentCount + 1.

self transfer: argumentCount - 1

       fromIndex: selectorIndex + 1

       ofObject: activeContext

       toIndex: selectorIndex

       ofObject: activeContext.

self pop: 1.

argumentCount := argumentCount - 1.

self executeNewMethod\]

ifFalse: \[messageSelector := performSelector\]

The `primitivePerformWithArgs` routine is associated with the
`performWithArguments:` messages in `Object`. It is basically the same
as the `primitivePerform` routine except that the message arguments come
in a single `Array` argument to the `performWithArguments:` message
instead of as multiple arguments to the \"`perform`\" message.

**primitivePerformWithArgs**

\| thisReceiver performSelector argumentArray arrayClass arraySize index
\|

argumentArray := self popStack.

arraySize := memory fetchWordLengthOf: argumentArray.

arrayClass := memory fetchClassOf: argumentArray.

self success: (stackPointer + arraySize)

\< (memory fetchWordLengthOf: activeContext).

self success: (arrayClass = ClassArrayPointer)

self success

ifTrue: \[performSelector := messageSelector.

messageSelector := self popStack.

thisReceiver := self stackTop.

argumentCount := arraySize.

index := 1.

\[index \<= argumentCount\]

whileTrue: \[self push: (memory fetchPointer: index - 1

    ofObject: argumentArray).

index := index + 1\].

self lookupMethodInClass: (memory fetchClassOf: thisReceiver).

self success: (self argumentCountOf: newMethod) = argumentCount.

self success

ifTrue: \[self executeNewMethod\]

ifFalse: \[self pop: argumentCount.

self push: messageSelector.

self push: argumentArray.

argumentCount := 2.

messageSelector := performSelector\]\]

ifFalse: \[self unPop: 1\]

The next four primitive routines (for primitive indices 85 through 88)
are used for communication and scheduling of independent processes. The
following routine initializes the indices used to access `Processes`,
`ProcessorSchedulers`, and `Semaphores`.

**initializeSchedulerIndices**

*\"Class ProcessorScheduler\"*

ProcessListsIndex := 0.

ActiveProcessIndex := 1.

*\"Class LinkedList\"*

FirstLinkIndex := 0.

LastLinkIndex := 1.

*\"Class Semaphore\"*

ExcessSignalsIndex := 2.

*\"Class Link\"*

NextLinkIndex := 0.

*\"Class Process\"*

SuspendedContextIndex := 1.

PriorityIndex := 2.

MyListIndex := 3

Process switching must be synchronized with the execution of bytecodes.
This is done using the following four interpreter registers and the four
routines: `checkProcessSwitch`, `asynchronousSignal:`,
`synchronousSignal:`, and `transferTo:`.\
 

  ----------------------------------- -----------------------------------
  `newProcessWaiting`                 The `newProcessWaiting` register
                                      will be `true` if a process switch
                                      is called for and `false`
                                      otherwise.

  `newProcess`                        If `newProcessWaiting` is `true`
                                      then the `newProcess` register will
                                      point to the `Process` to be
                                      transferred to. 

  `semaphoreList`                     The `semaphoreList` register points
                                      to an `Array` used by the
                                      interpreter to buffer `Semaphores`
                                      that should be signaled. This is an
                                      `Array` in `Interpreter`, not in
                                      the object memory. It will be a
                                      table in a machine-language
                                      interpreter. 

  `semaphoreIndex`                    The `semaphoreIndex` register holds
                                      the index of the last `Semaphore`
                                      in the `semaphoreList` buffer.\
                                       
  ----------------------------------- -----------------------------------

  : **Process-related Registers of the Interpreter**

The `asynchronousSignal:` routine adds a Semaphore to the buffer.

**asynchronousSignal: aSemaphore**

semaphoreIndex := semaphoreIndex + 1.

semaphoreList at: semaphoreIndex put: aSemaphore

The `Semaphores` will actually be signaled in the `checkProcessSwitch`
routine which calls the `synchronousSignal:` routine once for each
`Semaphore` in the buffer. If a `Process` is waiting for the
`Semaphore`, the `synchronousSignal:` routine resumes it. If no
`Process` is waiting, the `synchronousSignal:` routine increments the
`Semaphore's` count of excess signals. The `isEmptyList:`, `resume:`,
and `removeFirstLinkOfList:` routines are described later in this
section.

**synchronousSignal: aSemaphore**

\| excessSignals \|

(self isEmptyList: aSemaphore)

ifTrue: \[excessSignals := self fetchInteger: ExcessSignalsIndex

   ofObject: aSemaphore.

self storeInteger ExcessSignalsIndex

       ofObject: aSemaphore

       withValue: excessSignals + 1\]

ifFalse: \[self resume: (self removeFirstLinkOfList: aSemaphore)\]

The `transferTo:` routine is used whenever the need to switch processes
is detected. It sets the `newProcessWaiting` and `newProcess` registers.

**transferTo: aProcess**

newProcessWaiting := true.

newProcess := aProcess

The `checkProcessSwitch` routine is called before each bytecode fetch
(in the interpret routine) and performs the actual process switch if one
has been called for. It stores the active context pointer in the old
`Process`, stores the new `Process` in the `ProcessorScheduler's` active
process field, and loads the new active context out of that `Process`.

**checkProcessSwitch**

\| activeProcess \|

\[semaphoreIndex \> 0\]

whileTrue: \[self synchronousSignal: (semaphoreList at: semaphoreIndex).

semaphoreIndex := semaphoreIndex - 1\].

newProcessWaiting

ifTrue: \[newProcessWaiting := false.

activeProcess := self activeProcess.

memory storePointer: SuspendedContextIndex

 ofObject: activeProcess

 withValue: activeContext.

memory storePointer: ActiveProcessIndex

 ofObject: self schedulerPointer

 withValue: newProcess.

self newActiveContext: (memory fetchPointer: SuspendedContextIndex

      ofObject: newProcess)\]

Any routines desiring to know what the active process will be must take
into account the `newProcessWaiting` and `newProcess` registers.
Therefore, they use the following routine.

**activeProcess**

newProcessWaiting

ifTrue: \[\^newProcess\]

ifFalse: \[\^memory fetchPointer: ActiveProcessIndex

      ofObject: self schedulerPointer\]

The instance of `ProcessorScheduler` responsible for scheduling the
actual processor needs to be known globally so that the primitives will
know where to resume and suspend `Processes`. This `ProcessorScheduler`
is bound to the name `Processor` in the Smalltalk global dictionary. The
association corresponding to `Processor` has a guaranteed object
pointer, so the appropriate `ProcessorScheduler` can be found.

**schedulerPointer**

\^memory fetchPointer: ValueIndex

    ofObject: SchedulerAssociationPointer

When Smalltalk is started up, the initial active context is found
through the scheduler\'s active `Process`.

**firstContext**

newProcessWaiting := false.

\^memory fetchPointer: SuspendedContextIndex

    ofObject: self activeProcess

If the object memory automatically deallocates objects on the basis of
reference counting, special consideration must be given to reference
counting in the process scheduling routines. During the execution of
some of these routines, there will be times at which there are no
references to some object from the object memory (e.g., after a
`Process` has been removed from a `Semaphore` but before it has been
placed on one of the `ProcessorScheduler's` `LinkedLists`). If the
object memory uses garbage collection, it simply must avoid doing a
collection in the middle of a primitive routine. The routines listed
here ignore the reference-counting problem in the interest of clarity.
Implementations using reference-counting will have to modify these
routines in order to prevent premature deallocation of objects.\
        The following three routines are used to manipulate
`LinkedLists`.

**removeFirstLinkOfList: aLinkedList**

\| firstLink lastLink nextLink \|

firstLink := memory fetchPointer: FirstLinkIndex

ofObject: aLinkedList.

lastLink := memory fetchPointer: LastLinkIndex

ofObject: aLinkedList.

lastLink = firstLink

ifTrue: \[memory storePointer: FirstLinkIndex

   ofObject: aLinkedList

   withValue: NilPointer.

memory storePointer: LastLinkIndex

 ofObject: aLinkedList

 withValue: NilPointer\]

ifFalse: \[nextLink := memory fetchPointer: NextLinkIndex

 ofObject: firstLink.

memory storePointer: FirstLinkIndex

 ofObject: aLinkedList

 withValue: nextLink\].

memory storePointer: NextLinkIndex

 ofObject: firstLink

 withValue: NilPointer.

\^firstLink

**addLastLink: aLink toList: aLinkedList**

\| lastLink \|

(self isEmptyList: aLinkedList)

ifTrue: \[memory storePointer: FirstLinkIndex

    ofObject: aLinkedList

    withValue: aLink\]

ifFalse: \[lastLink := memory fetchPointer: LastLinkIndex

ofObject: aLinkedList.

memory storePointer: NextLinkIndex

 ofObject: lastLink

 withValue: aLink\].

memory storePointer: LastLinkIndex

 ofObject: aLinkedList

 withValue: aLink.

memory storePointer: MyListIndex

 ofObject: aLink

 withValue: aLinkedList

**isEmptyList: aLinkedList**

\^(memory fetchPointer: FirstLinkIndex

    ofObject: aLinkedList) = NilPointer

These three `LinkedList` routines are used, in turn, to implement the
following two routines that remove links from or add links to the
`ProcessorScheduler'sLinkedLists` of quiescent `Processes`.

**wakeHighestPriority**

\| priority processLists processList \|

processLists := memory fetchPointer: ProcessListsIndex

    ofObject: self schedulerPointer.

priority := memory fetchWordLengthOf: processLists.

\[processList := memory fetchPointer: priority - 1

    ofObject: processLists.

self isEmptyList: processList\] whileTrue: \[priority := priority - 1\].

\^self removeFirstLinkOfList: processList

**sleep: aProcess**

\| priority processLists processList \|

priority := self fetchInteger: PriorityIndex

   ofObject: aProcess.

processLists := memory fetchPointer: ProcessListsIndex

    ofObject: self schedulerPointer.

processList := memory fetchPointer: priority - 1

   ofObject: processLists.

self addLastLink: aProcess

       toList: processList

These two routines are used, in turn, to implement the following two
routines that actually suspend and resume `Processes`.

**suspendActive**

self transferTo: self wakeHighestPriority

**resume: aProcess**

\| activeProcess activePriority newPriority \|

activeProcess := self activeProcess.

activePriority := self fetchInteger: PriorityIndex

 ofObject: activeProcess.

newPriority := self fetchInteger: PriorityIndex

        ofObject: aProcess.

newPriority \> activePriority

ifTrue: \[self sleep: activeProcess.

self transferTo: aProcess\]

ifFalse: \[self sleep: aProcess\]

The `primitiveSignal` routine is associated with the signal message in
`Semaphore`. Since it is called in the process of interpreting a
bytecode, it can use the `synchronousSignal:` routine. Any other
signaling of `Semaphores` by the interpreter (for example, for timeouts
and keystrokes) must use the `asynchronousSignal:` routine.

**primitiveSignal**

self synchronousSignal: self stackTop

The `primitiveWait` routine is associated with the wait message in
`Semaphore`. If the receiver has an excess signal count greater than 0,
`primitiveWait` routine decrements the count. If the excess signal count
is 0, the `primitiveWait` suspends the active `Process` and adds it to
the receiver\'s list of `Processes`.

**primitiveWait**

\| thisReceiver excessSignals \|

thisReceiver := self stackTop.

excessSignals := self fetchInteger: ExcessSignalsIndex ofObject:
thisReceiver.

excessSignals \> 0

ifTrue: \[self storeInteger: ExcessSignalsIndex

         ofObject: thisReceiver

         withValue: excessSignals - 1\]

ifFalse: \[self addLastLink: self activeProcess

          toList: thisReceiver.

self suspendActive\]

The `primitiveResume` routine is associated with the resume message in
`Process`. It simply calls the `resume:` routine with the receiver as
argument.

**primitiveResume**

self resume: self stackTop

The `primitiveSuspend` routine is associated with the suspend message in
`Process`. The `primitiveSuspend` routine suspends the receiver if it is
the active `Process`. If the receiver is not the active `Process`, the
primitive fails.

**primitiveSuspend**

self success: self stackTop = self activeProcess.

self success

ifTrue: \[self popStack.

self push: NilPointer.

self suspendActive\]

The `primitiveFlushCache` routine removes the contents of the method
cache. Implementations that do not use a method cache can treat this as
a no-op.

**primitiveFlushCache**

self initializeMethodCache

------------------------------------------------------------------------

[]{#InputOutputPrimitives29}**Input/Output Primitives**

------------------------------------------------------------------------

The input/output primitive routines provide Smalltalk with access to the
state of the hardware devices. Since the implementation of these
routines will be dependent on the structure of the implementing machine,
no routines will be given, just a specification of the behavior of the
primitives.

**dispatchInputOutputPrimitives**

primitiveIndex = 90 ifTrue: \[\^self primitiveMousePoint\].

primitiveIndex = 91 ifTrue: \[\^self primitiveCursorLocPut\].

primitiveIndex = 92 ifTrue: \[\^self primitiveCursorLink\].

primitiveIndex = 93 ifTrue: \[\^self primitiveInputSemaphore\].

primitiveIndex = 94 ifTrue: \[\^self primitiveSamplelnterval\].

primitiveIndex = 95 ifTrue: \[\^self primitiveInputWord\].

primitiveIndex = 96 ifTrue: \[\^self primitiveCopyBits\].

primitiveIndex = 97 ifTrue: \[\^self primitiveSnapshot\].

primitiveIndex = 98 ifTrue: \[\^self primitiveTimeWordsInto\].

primitiveIndex = 99 ifTrue: \[\^self primitiveTickWordsInto\].

primitiveIndex = 100 ifTrue: \[\^self primitiveSignalAtTick\].

primitiveIndex = 101 ifTrue: \[\^self primitiveBeCursor\].

primitiveIndex = 102 ifTrue: \[\^self primitiveBeDisplay\].

primitiveIndex = 103 ifTrue: \[\^self primitiveScanCharacters\].

primitiveIndex = 104 ifTrue: \[\^self primitiveDrawLoop\].

primitiveIndex = 105 ifTrue: \[\^self primitiveStringReplace\]

Four of the primitive routines are used to detect actions by the user.
The two types of user action the system can detect are changing the
state of a bi-state device and moving the pointing device. The bi-state
devices are the keys on the keyboard, three buttons associated with the
pointing device and an optional five-paddle keyset. The buttons
associated with the pointing device may or may not actually be on the
physical pointing device. Three of the four input primitive routines
(`primitiveInputSemaphore`, `primitiveInputWord`, and
`primitiveSampleInterval`) provide an *active* or *event-initiated*
mechanism to detect either state change or movement. The other primitive
routine (`primitiveMousePoint`) provides a *passive* or *polling*
mechanism to detect pointing device location.\
    The event-initiated mechanism provides a buffered stream of 16-bit
words that encode changes to the bi-state devices or the pointing device
location. Each time a word is placed in the buffer, a `Semaphore` is
signaled (using the `asynchronousSignal:` routine). The `Semaphore` to
signal is initialized by the `primitiveInputSemaphore` routine. This
routine is associated with the `primInputSemaphore:` message in
`InputState` and the argument of the message becomes the `Semaphore` to
be signaled. The `primitiveInputWord` routine (associated with the
`primInputWord` message in `InputState`) returns the next word in the
buffer, removing it from the buffer. Since the `Semaphore` is signaled
once for every word in the buffer, the Smalltalk process emptying the
buffer should send the `Semaphore` a wait message before sending each
`primInputWord` message. There are six types of 16-bit word placed in
the buffer. Two types specify the time of an event, two types specify
state change of a bi-state device, and two types specify pointing device
movement. The type of the word is stored in its high order four bits.
The low order 12-bits are referred to as the *parameter*.\
    The six type codes have the following meanings.\
 

  ----------------------------------- -----------------------------------
  **type code**                       **meaning**

  0                                   Delta time (the parameter is the
                                      number of milliseconds since the
                                      last event of any type) 

  1                                   X location of the pointing device

  2                                   Y location of the pointing device

  3                                   Bi-state device turned on (the
                                      parameter indicates which device)

  4                                   Bi-state device turned off (the
                                      parameter indicates which device)

  5                                   Absolute time (the parameter is
                                      ignored, the next two words in the
                                      buffer contain a 32-bit unsigned
                                      number that is the absolute value
                                      of the millisecond clock)\
                                       
  ----------------------------------- -----------------------------------

Whenever a device state changes or the pointing device moves, a time
word is put into the buffer. A type 0 word will be used if the number of
milliseconds since the last event can be represented in 12 bits.
Otherwise, a type 5 event is used followed by two words representing the
absolute time. Note that the Semaphore will be signaled 3 times in the
latter case. Following the time word(s) will be one or more words of
type 1 through 4. Type 1 and 2 words will be generated whenever the
pointing device moves at all. It should be remembered that Smalltalk
uses a left-hand coordinate system to talk about the screen. The origin
is the upper left corner of the screen, the x dimension increases toward
the right, and the y dimension increases toward the bottom. The minimum
time span between these events can be set by the
`primitiveSampleInterval` routine which is associated with the
`primSamplelnterval:` message in `InputState`. The argument to
`primSamplelnterval:` specifies the number of milliseconds between
movement events if the pointing device is moving constantly.\
    Type 3 and 4 words use the low-order eight bits of the parameter to
specify which device changed state. The numbering scheme is set up to
work with both decoded and undecoded keyboards. An undecoded keyboard is
made up of independent keys with independent down and up transitions. A
decoded keyboard consists of some independent keys and some \"meta\"
keys (shift and escape) that cannot be detected on their own, but that
change the value of the other keys. The keys on a decoded keyboard only
indicate their down transition, not their up transition. On an undecoded
keyboard, the standard keys produce parameters that are the ASCII code
of the character on the keytop *without* shift or control information
(i.e., the key with \"A\" on it produces the ASCII for \"a\" and the key
with \"2\" and \"@\" on it produces the ASCII for \"2\"). The other
standard keys produce the following parameters.\
 

  ----------------------------------- -----------------------------------
  **key**                             **parameter**

  backspace                           8

  tab                                 9

  line feed                           10

  return                              13

  escape                              27

  space                               32

  delete                              127\
                                       
  ----------------------------------- -----------------------------------

For an undecoded keyboard, the meta keys have the following parameters.\
 

  ----------------------------------- -----------------------------------
  **key**                             **parameter**

  left shift                          136

  right shift                         137

  control                             138

  alpha-lock                          139\
                                       
  ----------------------------------- -----------------------------------

For a decoded keyboard, the full shifted and \"controlled\" ASCII should
be used as a parameter and successive type 3 and 4 words should be
produced for each keystroke.\
    The remaining bi-state devices have the following parameters.\
 

  ----------------------------------- -----------------------------------
  **key**                             **parameter**

  left or top \"pointing device\"     130
  button                              

  center \"pointing device\" button   129

  right or bottom \"pointing device\" 128
  button                              

  keyset paddles right to left        131 through 135\
                                       
  ----------------------------------- -----------------------------------

The `primitiveMousePoint` routine allows the location of the pointing
device to be polled. It allocates a new `Point` and stores the location
of the pointing device in its `x` and `y` fields.\
    The display screen is a rectangular set of pixels that can each be
one of two colors. The colors of the pixels are determined by the
individual bits in a specially designated instance of `DisplayScreen`.
`DisplayScreen` is a subclass of `Form`. The instance of `DisplayScreen`
that should be used to update the screen is designated by sending it the
message `beDisplay`. This message invokes the `primitiveBeDisplay`
primitive routine. The screen will be updated from the last recipient of
`beDisplay` approximately 60 times a second.\
    Every time the screen is updated, a *cursor* is ORed into its
pixels. The cursor image is determined by a specially designated
instance of `Cursor`. `Cursor` is a subclass of `Form` whose instances
always have both width and height of 16. The instance of `Cursor` that
should be ORed into the screen is designated by sending it the message
`beCursor`. This message invokes the `primitiveBeCursor` primitive
routine.\
    The location at which the cursor image should appear is called the
*cursor location*. The cursor location may be linked to the location of
the pointing device or the two locations may be independent. Whether or
not the two locations are linked is determined by sending a message to
class `Cursor` with the selector `cursorLink:` and either `true` or
`false` as the argument. If the argument is `true`, then the two
locations will be the same; if it is `false`, they are independent. The
`cursorLink:` message in `Cursor's` metaclass invokes the
`primitiveCursorLink` primitive routine. The cursor can be moved in two
ways. If the cursor and pointing device have been linked, then moving
the pointing device moves the cursor. The cursor can also be moved by
sending the message `primCursorLocPut:` to an instance of `InputState`.
This message takes a `Point` as an argument and invokes the
`primitiveCursorLocPut` primitive routine. This routine moves the cursor
to the location specified by the argument. If the cursor and pointing
device are linked, the `primitiveCursorLocPut` routine also changes the
location indicated by the pointing device.\
    The `primitiveCopyBits` routine is associated with the `copyBits`
message in `BitBlt` and performs an operation on a bitmap specified by
the receiver. This routine is described in Chapter 18.\
    The `primitiveSnapshot` routine writes the current state of the
object memory on a file of the same format as the Smalltalk-80 release
file. This file can be resumed in exactly the same way that the release
file was originally started. Note that the pointer of the active context
at the time of the primitive call must be stored in the active `Process`
on the file.\
    The `primitiveTimeWordsInto` and `primitiveTickWordsInto` routines
are associated with the `timeWordsInto:` and `tickWordsInto:` messages
in `Sensor`. Both of these messages take a byte indexable object of at
least four bytes as an argument. The `primitiveTimeWordsInto` routine
stores the number of seconds since the midnight previous to January 1,
1901 as an unsigned 32-bit integer into the first four bytes of the
argument. The `primitiveTickWordsInto` routine stores the number of
ticks of the millisecond clock (since it last was reset or rolled over)
as an unsigned 32-bit integer into the first four bytes of the
argument.\
    The `primitiveSignalAtTick` routine is associated with the
`signal:atTick:` messages in `ProcessorScheduler`. This message takes a
`Semaphore` as the first argument and a byte indexable object of at
least four bytes as the second argument. The first four bytes of the
second argument are interpreted as an unsigned 32-bit integer of the
type stored by the `primitiveTickWordsInto` routine. The interpreter
should signal the `Semaphore` argument when the millisecond clock
reaches the value specified by the second argument. If the specified
time has passed, the `Semaphore` is signaled immediately. This primitive
signals the last `Semaphore` to be passed to it. If a new call is made
on it before the last timer value has been reached, the last `Semaphore`
will not be signaled. If the first argument is not a `Semaphore`, any
currently waiting `Semaphore` will be forgotten.\
    The `primitiveScanCharacters` routine is an optional primitive
associated with the
`scanCharactersFrom:to:in:rightX:stopConditions:displaying:` message in
`CharacterScanner`. If the function of the Smalltalk method is
duplicated in the primitive routine, text display will go faster. The
`primitiveDrawLoop` routine is similarly an optional primitive
associated with the `drawLoopX:Y:` message in `BitBlt`. If the function
of the Smalltalk method is duplicated in the primitive routine, drawing
lines will go faster.

------------------------------------------------------------------------

[]{#SystemPrimitives29}**System Primitives**

------------------------------------------------------------------------

The seven final primitives are grouped together as system primitives.

**dispatchSystemPrimitives**

primitiveIndex = 110 ifTrue: \[\^self primitiveEquivalent\].

primitiveIndex = 111 ifTrue: \[\^self primitiveClass\].

primitiveIndex = 112 ifTrue: \[\^self primitiveCoreLeft\].

primitiveIndex = 113 ifTrue: \[\^self primitiveQuit\].

primitiveIndex = 114 ifTrue: \[\^self primitiveExitToDebugger\].

primitiveIndex = 115 ifTrue: \[\^self primitiveOopsLeft\].

primitiveIndex = 116 ifTrue: \[\^self
primitiveSignalAtOopsLeftWordsLeft\]

The `primitiveEquivalent` routine is associated with the `=` message in
`Object`. It returns `true` if the receiver and argument are the same
object (have the same object pointer) and `false` otherwise.

**primitiveEquivalent**

\| thisObject otherObject \|

otherObject := self popStack.

thisObject := self popStack.

thisObject = otherObject

ifTrue: \[self push: TruePointer\]

ifFalse: \[self push: FalsePointer\]

The `primitiveClass` routine is associated with the class message in
`Object`. It returns the object pointer of the receiver\'s class.

**primitiveClass**

\| instance \|

instance := self popStack.

self push: (memory fetchClassOf: instance)

The `primitiveCoreLeft` routine returns the number of unallocated words
in the object space. The `primitiveQuit` routine exits to another
operating system for the host machine, if one exists. The
`primitiveExitToDebugger` routine calls the machine language debugger,
if one exists.\

------------------------------------------------------------------------

\[[TOC](bluebook_imp_toc.html)\] \[[26](bluebook_chapter26.html)\]
\[[27](bluebook_chapter27.html)\] \[[28](bluebook_chapter28.html)\]
\[[29](#top_of_29)\] \[[30](bluebook_chapter30.html)\]
