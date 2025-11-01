---
Author: Dwight Hughes
GENERATOR: Mozilla/4.5 \[en\] (WinNT; I) \[Netscape\]
title: Blue Book Chapter 27
---

[]{#top_of_27}\[[TOC](bluebook_imp_toc.html)\]
\[[26](bluebook_chapter26.html)\] 27 \[[28](bluebook_chapter28.html)\]
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]\

------------------------------------------------------------------------

27\
Specification of the Virtual Machine

------------------------------------------------------------------------

**[Form of the Specification](#FormOfTheSpecification27)**

**[Object Memory Interface](#ObjectMemoryInterface27)**

**[Objects Used by the Interpreter](#ObjectsUsedByTheInterpreter27)**

*[Compiled Methods](#CompiledMethods27)*

*[Contexts](#Contexts27)*

*[Classes](#Classes27)*

------------------------------------------------------------------------

[Chapter 26](bluebook_chapter26.html) described the function of the
Smalltalk virtual machine, which consists of an interpreter and an
object memory. This chapter and the next three present a more formal
specification of these two parts of the virtual machine. Most
implementations of the virtual machine will be written in machine
language or microcode. However, for specification purposes, these
chapters will present an implementation of the virtual machine in
Smalltalk itself. While this is a somewhat circular proposition, every
attempt has been made to ensure that no details are hidden as a result.\
    This chapter consists of three sections. The first describes the
conventions and terminology used in the formal specification. It also
provides some warnings of possible confusion resulting from the form of
this specification. The second section describes the object memory
routines used by the interpreter. The implementation of these routines
will be described in [Chapter 30](bluebook_chapter30.html). The third
section describes the three main types of object that the interpreter
manipulates, methods, contexts, and classes. [Chapter
28](bluebook_chapter28.html) describes the bytecode set and how it is
interpreted; [Chapter 29](bluebook_chapter29.html) describes the
primitive routines.

------------------------------------------------------------------------

[]{#FormOfTheSpecification27}**Form of the Specification**

------------------------------------------------------------------------

Two class descriptions named `Interpreter` and `ObjectMemory` make up
the formal specification of the Smalltalk-80 virtual machine. The
implementation of `Interpreter `will be presented in detail in this
chapter and the following two; the implementation of `ObjectMemory` in
[Chapter 30](bluebook_chapter30.html).\
    A potential source of confusion in these chapters comes from the two
Smalltalk systems involved in the descriptions, the system containing
`Interpreter` and `ObjectMemory` and the system being interpreted.
`Interpreter` and `ObjectMemory` have methods and instance variables and
they also manipulate methods and instance variables in the system they
interpret. To minimize the confusion, we will use a different set of
terminology for each system. The methods of `Interpreter` and
`ObjectMemory` will be called *routines*; the word *method* will be
reserved for the methods being interpreted. Similarly, the instance
variables of `Interpreter` and `ObjectMemory` will be called
*registers*; the word *instance variable* will be reserved for the
instance variables of objects in the system being interpreted.\
    The arguments of the routines and the contents of the registers of
`Interpreter` and `ObjectMemory` will almost always be instances of
`Integer` (`SmallIntegers` and `LargePositiveIntegers`). This can also
be a source of confusion since there are `Integers` in the interpreted
system. The `Integers` that are arguments to routines and contents of
registers represent object pointers and numerical values of the
interpreted system. Some of these will represent the object pointers or
values of `Integers` in the interpreted system.\
    The interpreter routines in this specification will all be in the
form of Smalltalk method definitions. For example

**routineName: argumentName**

\| temporaryVariable \|

temporaryVariable := self anotherRoutine: argumentName.

\^temporaryVariable - 1

    The routines in the specification will contain five types of
expression.\
 

+-----------------------------------+-----------------------------------+
| 1.                                | *Calls on other routines of the   |
|                                   | interpreter.* Since both the      |
|                                   | invocation and definition of the  |
|                                   | routine are in `Interpreter`,     |
|                                   | they will appear as messages to   |
|                                   | `self`.                           |
+-----------------------------------+-----------------------------------+
|                                   | -   self **headerOf:** newMethod  |
|                                   | -   self                          |
|                                   |                                   |
|                                   | **storeInstructionPointerValue:** |
|                                   |     value **inContext:**          |
|                                   |     contextPointer                |
+-----------------------------------+-----------------------------------+
| 2.                                | *Calls on routines of the object  |
|                                   | memory.* An `Interpreter` uses    |
|                                   | the name `memory` to refer to its |
|                                   | object memory, so these calls     |
|                                   | will appear as messages to        |
|                                   | `memory`.                         |
+-----------------------------------+-----------------------------------+
|                                   | -   memory **fetchClassOf:**      |
|                                   |     newMethod                     |
|                                   | -   memory **storePointer:**      |
|                                   |     senderindex **ofObject:**     |
|                                   |     contextPointer **withValue:** |
|                                   |     activeContext                 |
+-----------------------------------+-----------------------------------+
| 3.                                | *Arithmetic operations on object  |
|                                   | pointers and numerical values.*   |
|                                   | Arithmetic operations will be     |
|                                   | represented by standard Smalltalk |
|                                   | arithmetic expressions, so they   |
|                                   | will appear as messages to the    |
|                                   | numbers themselves.               |
+-----------------------------------+-----------------------------------+
|                                   | -   receiverValue **+**           |
|                                   |     argumentValue                 |
|                                   | -   selectorPointer **bitShift:** |
|                                   |     -1                            |
+-----------------------------------+-----------------------------------+
| 4.                                | *Array accesses.* Certain tables  |
|                                   | maintained by the interpreter are |
|                                   | represented in the formal         |
|                                   | specification by `Arrays`. Access |
|                                   | to these will appear as `at:` and |
|                                   | `at:put:` messages to the         |
|                                   | `Arrays`.                         |
+-----------------------------------+-----------------------------------+
|                                   | -   methodCache **at:** hash      |
|                                   | -   semaphoreList **at:**         |
|                                   |     semaphoreIndex **put:**       |
|                                   |     semaphorePointer              |
+-----------------------------------+-----------------------------------+
| 5.                                | *Conditional control structures.* |
|                                   | The control structures of the     |
|                                   | virtual machine will be           |
|                                   | represented by standard Smalltalk |
|                                   | conditional control structures.   |
|                                   | Conditional selections will       |
|                                   | appear as messages to Booleans.   |
|                                   | Conditional repetitions will      |
|                                   | appear as messages to blocks.     |
+-----------------------------------+-----------------------------------+
|                                   | -   index \< length **ifTrue:**   |
|                                   |     \[ \... \]                    |
|                                   | -   sizeFlag = 1 **ifTrue:** \[   |
|                                   |     \... \] **ifFalse:** \[ \...  |
|                                   |     \]                            |
|                                   | -   \[currentClass \~=            |
|                                   |     NilPointer\] **whileTrue:**   |
|                                   |     \[ \... \]                    |
+-----------------------------------+-----------------------------------+
|                                   |                                   |
+-----------------------------------+-----------------------------------+

    The definition of `Interpreter` describes the function of the
Smallta1k-80 bytecode interpreter; however, the form of a machine
language implementation of the interpreter may be very different,
particularly in the control structures it uses. The dispatch to the
appropriate routine to execute a bytecode is an example of something a
machine language interpreter might do differently. To find the right
routine to execute, a machine language interpreter would probably do
some kind of address arithmetic to calculate where to jump; whereas, as
we will see, `Interpreter` does a series of conditionals and routine
calls. In a machine language implementation, the routines that execute
each bytecode would simply jump back to the beginning of the bytecode
fetch routine when they were finished, instead of returning through the
routine call structure.\
    Another difference between `Interpreter` and a machine language
implementation is the degree of optimization of the code. For the sake
of clarity, the routines specified in this chapter have not been
optimized. For example, to perform a task, `Interpreter` may fetch a
pointer from the object memory several times in different routines, when
a more optimized interpreter might save the value in a register for
later use. Many of the routines in the formal specification will not be
subroutines in a machine language implementation, but will be written
in-line instead.

------------------------------------------------------------------------

[]{#ObjectMemoryInterface27}**Object Memory Interface**

------------------------------------------------------------------------

[Chapter 26](bluebook_chapter26.html) gave an informal description of
the object memory. Since the routines of `Interpreter` need to interact
with the object memory, we need its formal functional specification.
This will be presented as the protocol specification of class
`ObjectMemory`. [Chapter 30](bluebook_chapter30.html) will describe one
way to implement this protocol specification.\
    The object memory associates a 16-bit object pointer with

1.  the object pointer of a class-describing object and
2.  a set of 8- or 16-bit fields that contain object pointers or
    numerical values.

The interface to the object memory uses zero-relative integer indices to
indicate an object\'s fields. Instances of `Integer` are used for both
object pointers and field indices in the interface between the
interpreter and object memory.\
        The protocol of `ObjectMemory` contains pairs of messages for
fetching and storing object pointers or numerical values in an object\'s
fields.

*object pointer access*

**fetchPointer: fieldIndex ofObject: objectPointer**

Return the object pointer found in the field numbered `fieldIndex` of
the object associated with `objectPointer`.

**storePointer: fieldIndex ofObject: objectPointer withValue:
valuePointer**

Store the object pointer `valuePointer` in the field numbered
`fieldIndex` of the object associated with `objectPointer`.

*word access*

**fetchWord: fieldIndex ofObject: objectPointer**

Return the 16-bit numerical value found in the field numbered
`fieldIndex` of the object associated with `objectPointer`.

**storeWord: fieldIndex ofObject: objectPointer withValue: valueWord**

Store the 16-bit numerical value `valueWord` in the field numbered
`fieldIndex` of the object associated with `objectPointer`.

*byte access*

**fetchByte: byteIndex ofObject: objectPointer**

Return the 8-bit numerical value found in the byte numbered `byteIndex`
of the object associated with `objectPointer`.

**storeByte: byteIndex ofObject: objectPointer withValue: valueByte**

Store the 8-bit numerical value `valueByte` in the byte numbered
`byteIndex` of the object associated with `objectPointer`.

    Note that `fetchPointer:ofObject:` and `fetchWord:ofObject:` will
probably be implemented in an identical fashion, since they both load a
16-bit quantity. However, the implementation of `storePointer:ofObject:`
will be different from the implementation of `storeWord:ofObject:` since
it will have to perform reference counting (see [Chapter
30](bluebook_chapter30.html)) if the object memory keeps dynamic
reference counts. We have maintained a separate interface for
`fetchPointer:ofObject:` and `fetchWord:ofObject:` for the sake of
symmetry.\
    Even though most of the maintenance of reference counts can be done
automatically in the `storePointer:ofObject:withValue:` routine, there
are some points at which the interpreter routines must directly
manipulate the reference counts. Therefore, the following two routines
are included in the object memory interface. If an object memory uses
only garbage collection to reclaim unreferenced objects, these routines
are no-ops.

*reference counting*

**increaseReferencesTo: objectPointer**

Add one to the reference count of the object whose object pointer is
`objectPointer`.

**decreaseReferencesTo: objectPointer**

Subtract one from the reference count of the object whose object pointer
is `objectPointer`.

Since every object contains the object pointer of its class description,
that pointer could be considered the contents of one of the object\'s
fields. Unlike other fields, however, an object\'s class may be fetched,
but its value may not be changed. Given the special nature of this
pointer, it was decided not to access it in the same way. Therefore,
there is a special protocol for fetching an object\'s class.

*class pointer access*

**fetchClassOf: objectPointer**

Return the object pointer of the class-describing object for the object
associated with `objectPointer`.

The length of an object might also be thought of as the contents of one
of its fields. However, it is like the class field in that it may not be
changed. There are two messages in the object memory protocol that ask
for the number of words in an object and the number of bytes in an
object. Note that we have not made a distinction between words and
pointers in this case since we assume that they both fit in exactly one
field.

*length access*

**fetchWordLengthOf: objectPointer**

Return the number of fields in the object associated with
`objectPointer`.

**fetchByteLengthOf: objectPointer**

Return the number of byte fields in the object associated with
`objectPointer`.

Another important service of the object memory is to create new objects.
The object memory must be supplied with a class and a length and will
respond with a new object pointer. Again, there are three versions for
creating objects with pointers, words, or bytes.

*object creation*

**instantiateClass: classPointer withPointers: instanceSize**

Create a new instance of the class whose object pointer is
`classPointer` with `instanceSize` fields that will contain pointers.
Return the object pointer of the new object.

**instantiateClass: classPointer withWords: instanceSize**

Create a new instance of the class whose object pointer is
`classPointer` with `instanceSize` fields that will contain 16-bit
numerical values. Return the object pointer of the new object.

**instantiateClass: classPointer withBytes: instanceByteSize**

Create a new instance of the class whose object pointer is
`classPointer` with room for `instanceByteSize` 8-bit numerical values.
Return the object pointer of the new object.

Two routines of the object memory allow the instances of a class to be
enumerated. These follow an arbitrary ordering of object pointers. Using
the numerical order of the pointers themselves is reasonable.

*instance enumeration*

**initialInstanceOf: classPointer**

Return the object pointer of the first instance of the class whose
object pointer is `classPointer` in the defined ordering (e.g., the one
with the smallest object pointer).

**instanceAfter: objectPointer**

Return the object pointer of the next instance of the same class as the
object whose object pointer is `objectPointer` in the defined ordering
(e.g., the one with the next larger object pointer).

        Another routine of the object memory allows the object pointers
of two objects to be interchanged.

*pointer Swapping*

**swapPointersOf: firstPointer and: secondPointer**

Make `firstPointer` refer to the object whose object pointer was
`secondPointer` and make `secondPointer` refer to the object whose
object pointer was `firstPointer`.

    As described in [Chapter 26](bluebook_chapter26.html), integers
between -16384 and 16383 are encoded directly as object pointers with a
1 in the low-order bit position and the appropriate 2\'s complement
value stored in the high-order 15 bits. These objects are instances of
class `SmallInteger`. A `SmallInteger's` value, which would ordinarily
be stored in a field, is actually determined from its object pointer. So
instead of storing a value into a `SmallInteger's` field, the
interpreter must request the object pointer of a `SmallInteger` with the
desired value (using the `integerObjectOf:` routine). And instead of
fetching the value from a field, it must request the value associated
with the object pointer (using the `integerValueOf:` routine). There are
also two routines that determine whether an object pointer refers to a
`SmallInteger` (`isIntegerObject:`) and whether a value is in the right
range to be represented as a `SmallInteger` (`isIntegerValue:`). The
function of the `isIntegerObject:` routine can also be performed by
requesting the class of the object and seeing if it is `SmallInteger`.

*integer access*

**integerValueOf: objectPointer**

Return the value of the instance of `SmallInteger` whose pointer is
`objectPointer`.

**integerObjectOf: value**

Return the object pointer for an instance of `SmallInteger` whose value
is `value`.

**isIntegerObject: objectPointer**

Return `true` if `objectPointer` is an instance of `SmallInteger`,
`false` if not.

**isIntegerValue: value**

Return `true` if `value` can be represented as an instance of
`SmallInteger`, `false` if not.

The interpreter provides two special routines to access fields that
contain `SmallIntegers`. The `fetchInteger:ofObject:` routine returns
the value of a `SmallInteger` whose pointer is stored in the specified
field. The check to make sure that the pointer is for a `SmallInteger`
is made for uses of this routine when non-`SmallIntegers` can be
tolerated. The `primitiveFail` routine will be described in the section
on primitive routines.

**fetchInteger: fieldIndex ofObject: objectPointer**

\| integerPointer \|

integerPointer := memory fetchPointer: fieldIndex

         ofObject: objectPointer.

(memory isIntegerObject: integerPointer)

ifTrue: \[\^memory integerValueOf: integerPointer\]

ifFalse: \[\^self primitiveFail\]

The `storeInteger:ofObject:withValue:` routine stores the pointer of the
`SmallInteger` with specified value in the specified field.

**storeInteger: fieldIndex ofObject: objectPointer withValue:
integerValue**

\| integerPointer \|

(memory isIntegerValue: integerValue)

ifTrue: \[integerPointer := memory integerObjectOf: integerValue.

memory storePointer: fieldIndex

   ofObject: objectPointer

   withValue: integerPointer\]

ifFalse: \[\^self primitiveFail\]

The interpreter also provides a routine to perform a transfer of several
pointers from one object to another. It takes the number of pointers to
transfer, and the initial field index and object pointer of the source
and destination objects as arguments.

**transfer: count fromIndex: firstFrom ofObject: fromOop toIndex:
firstTo ofObject: toOop**

\| fromIndex toIndex lastFrom oop \|

fromIndex := firstFrom.

lastFrom := firstFrom + count.

toIndex := firstTo.

\[fromIndex \< lastFrom\] whileTrue:

\[oop := memory fetchPointer: fromIndex

  ofObject: fromOop.

memory storePointer: toIndex

   ofObject: toOop

   withValue: oop.

memory storePointer: fromIndex

   ofObject: fromOop

   withValue: NilPointer.

fromIndex := fromIndex + 1.

toIndex := toIndex + 1\]

The interpreter also provides routines to extract bit fields from
numerical values. These routines refer to the high-order bit with index
0 and the low-order bit with index 15.

**extractBits: firstBitIndex to: lastBitIndex of: anInteger**

\^(anInteger bitShift: lastBitIndex - 15)

bitAnd: (2 raisedTo: lastBitIndex - firstBitIndex + 1) - 1

**highByteOf: anInteger**

\^self extractBits: 0 to: 7 of: anInteger

**lowByteOf: anInteger**

\^self extractBits 8 to: 15 of: anInteger

------------------------------------------------------------------------

[]{#ObjectsUsedByTheInterpreter27}**Objects Used by the Interpreter**

------------------------------------------------------------------------

This section describes what might be called the data structures of the
by the interpreter. Although they are objects, and therefore more than
data interpreter structures, the interpreter treats these objects as
data structures. The first two types of object correspond to data
structures found in the interpreters for most languages. *Methods*
correspond to programs, subroutines, or procedures. *Contexts*
correspond to stack frames or activation records. The final structure
described in this section, that of *classes*, is not used by the
interpreter for most languages but only by the compiler. Classes
correspond to aspects of the type declarations of some other languages.
Because of the nature of Smalltalk messages, the classes must be used by
the interpreter at runtime.\
    There are many constants included in the formal specification. They
mostly represent object pointers of known objects or field indices for
certain kinds of objects. Most of the constants will be named and a
routine that initializes them will be included as a specification of
their value. As an example, the following routines initialize the object
pointers known to the interpreter.

**initializeSmallIntegers**

*\"SmallIntegers\"*

MinusOnePointer := 65535.

ZeroPointer := 1.

OnePointer := 3.

TwoPointer := 5

**initializeGuaranteedPointers**

*\"UndefinedObject and Booleans\"*

NilPointer := 2.

FalsePointer := 4.

TruePointer := 6.

*\"Root\"*

SchedulerAssociationPointer := 8.

*\"Classes\"*

ClassStringPointer := 14.

ClassArrayPointer := 16.

ClassMethodContextPointer := 22.

ClassBlockContextPointer := 24.

ClassPointPointer := 26.

ClassLargePositiveintegerPointer := 28.

ClassMessagePointer := 32.

ClassCharacterPointer := 40.

*\"Selectors\"*

DoesNotUnderstandSelector := 42.

CannotReturnSelector := 44.

MustBeBooleanSelector := 52.

*\"Tables\"*

SpecialSelectorsPointer := 48.

CharacterTablePointer := 50

[]{#CompiledMethods27}***Compiled Methods***\

------------------------------------------------------------------------

The bytecodes executed by the interpreter are found in instances of
`CompiledMethod`. The bytecodes are stored as 8-bit values, two to a
word. In addition to the bytecodes, a `CompiledMethod` contains some
object pointers. The first of these object pointers is called the
*method header* and the rest of the object pointers make up the
method\'s *literal frame*. [Figure 27.1](#Figure_27.1) shows the
structure of a `CompiledMethod` and the following routine initializes
the indices used to access fields of `CompiledMethods`.\
[]{#Figure_27.1}

  -----------------------------------------------
  ![](figure27_1.gif){height="214" width="155"}
  **Figure 27.1**
  -----------------------------------------------

**initializeMethodIndices**

*\"Class CompiledMethod\"*

HeaderIndex := 0.

LiteralStart := 1

The header is a `SmallInteger` that encodes certain information about
the `CompiledMethod`.

**headerOf: methodPointer**

\^memory fetchPointer: HeaderIndex

    ofObject: methodPointer

The literal frame contains pointers to objects referred to by the
bytecodes. These include the selectors of messages that the method
sends, and shared variables and constants to which the method refers.

**literal: offset ofMethod: methodPointer**

\^memory fetchPointer: offset + LiteralStart

    ofObject: methodPointer

Following the header and literals of a method are the bytecodes. Methods
are the only objects in the Smalltalk system that store both object
pointers (in the header and literal frame) and numerical values (in the
bytecodes). The form of the bytecodes will be discussed in the [next
chapter](bluebook_chapter28.html).

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Method
Headers***   Since the method header is a `SmallInteger`, its value will
be encoded in its pointer. The high-order 15 bits of the pointer are
available to encode information; the low-order bit must be a one to
indicate that the pointer is for a `SmallInteger`. The header includes
four bit fields that encode information about the `CompiledMethod`.
[Figure 27.2](#Figure_27.2) shows the bit fields of a header.\
[]{#Figure_27.2}

  ----------------------------------------------
  ![](figure27_2.gif){height="90" width="379"}
  **Figure 27.2**
  ----------------------------------------------

The temporary count indicates the number of temporary variables used by
the `CompiledMethod`. This includes the number of arguments.

**temporaryCountof: methodPointer**

\^self extractBits: 3 to: 7 of: (self headerOf: methodPointer)

The large context flag indicates which of two sizes of `MethodContext`
are needed. The flag indicates whether the sum of the maximum stack
depth and the number of temporary variables needed is greater than
twelve. The smaller `MethodContexts` have room for 12 and the larger
have room for 32.

**largeContextFlagOf: methodPointer**

\^self extractBits: 8 to: 8 of: (self headerOf: methodPointer)

The literal count indicates the size of the `CompiledMethod's` literal
frame. This, in turn, indicates where the `CompiledMethod's` bytecodes
start.

**literalCountOf: methodPointer**

\^self literalCountOfHeader: (self headerOf: methodPointer)

**literalCountOfHeader: headerPointer**

\^self extractBits: 9 to: 14 of: headerPointer

The object pointer count indicates the total number of object pointers
in a `CompiledMethod`, including the header and literal frame.

**objectPointerCountOf: methodPointer**

\^(self literalCountOf: methodPointer) + LiteralStart

The following routine returns the byte index of the first bytecode of a
`CompiledMethod`.

**initialInstructionPointerOfMethod: methodPointer**

\^((self literalCountOf: methodPointer) + LiteralStart) \* 2 + 1

The flag value is used to encode the number of arguments a
`CompiledMethod` takes and whether or not it has an associated primitive
routine.

**flagValueOf: methodPointer**

\^self extractBits: 0 to: 2 of: (self headerOf: methodPointer)

The eight possible flag values have the following meanings:\
 

  -----------------------------------------------------------------------
  flag value                          meaning
  ----------------------------------- -----------------------------------
  0-4                                 no primitive and 0 to 4 arguments 

  5                                   primitive return of `self` (0
                                      arguments)

  6                                   primitive return of an instance
                                      variable (0 arguments)

  7                                   a header extension contains the
                                      number of arguments and a primitive
                                      index\
                                          
  -----------------------------------------------------------------------

Since the majority of `CompiledMethods` have four or fewer arguments and
do not have an associated primitive routine, the flag value is usually
simply the number of arguments.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Special
Primitive Methods***   Smalltalk methods that *only* return the receiver
of the message (`self`) produce `CompiledMethods` that have no literals
or bytecodes, only a header with a flag value of 5. In similar fashion,
Smalltalk methods that only return the value of one of the receiver\'s
instance variables produce `CompiledMethods` that contain only headers
with a flag value of 6. All other methods produce `CompiledMethods` with
bytecodes. When the flag value is 6, the index of the instance variable
to return is found in the header in the bit field ordinarily used to
indicate the number of temporary variables used by the `CompiledMethod`.
[Figure 27.3](#Figure_27.3) shows a `CompiledMethod` for a Smalltalk
method that only returns a receiver instance variable.\
[]{#Figure_27.3}

  ----------------------------------------------
  ![](figure27_3.gif){height="69" width="376"}
  **Figure 27.3**
  ----------------------------------------------

The following routine returns the index of the field representing the
instance variable to be returned in the case that the flag value is 6.

**fieldIndexOf: methodPointer**

\^self extractBits: 3 to: 7 of: (self headerOf: methodPointer)

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Method
Header Extensions *** If the flag value is 7, the next to last literal
is a header extension, which is another `SmallInteger`. The header
extension includes two bit fields that encode the argument count and
primitive index of the `CompiledMethod`. [Figure 27.4](#Figure_27.4)
shows the bit fields of a header extension.\
[]{#Figure_27.4}

  ----------------------------------------------
  ![](figure27_4.gif){height="70" width="375"}
  **Figure 27.4**
  ----------------------------------------------

The following routines are used to access a header extension and its bit
fields.

**headerExtensionOf: methodPointer**

\| literalCount \|

literalCount := self literalCountOf: methodPointer.

\^self literal: literalCount - 2

         ofMethod: methodPointer

**argumentCountOf: methodPointer**

\| flagValue \|

flagValue := self flagValueOf: methodPointer.

flagValue \< 5  ifTrue: \[\^flagValue\].

flagValue \< 7

ifTrue: \[\^0\]

ifFalse: \[\^self extractBits: 2 to: 6

 of: (self headerExtensionOf: methodPointer)\]

**primitiveIndexOf: methodPointer**

\| flagValue \|

flagValue := self flagValueOf: methodPointer.

flagValue = 7

ifTrue: \[\^self extractBits: 7 to: 14

of: (self headerExtensionOf: methodPointer)\]

ifFalse: \[\^0\]

Any `CompiledMethod` that sends a superclass message (i.e., a message to
`super`) or contains a header extension, will have as its last literal
an `Association` whose value is the class in whose message dictionary
the `CompiledMethod` is found. This is called the *method class* and is
accessed by the following routine.

**methodClassOf: methodPointer**

\| literalCount association \|

literalCount := self literalCountOf: methodPointer.

association := self literal: literalCount - 1

        ofMethod: methodPointer.

\^memory fetchPointer: ValueIndex

    ofObject: association

An example of a `CompiledMethod` whose literal frame contained a method
class was given in the [last chapter](bluebook_chapter26.html). The
`CompiledMethod` for the `intersect:` message to `ShadedRectangle` was
shown in the section of the last chapter called
[Messages](bluebook_chapter26.html#Messages26).

[]{#Contexts27}***Contexts***\

------------------------------------------------------------------------

The interpreter uses *contexts* to represent the state of its execution
of `CompiledMethods` and blocks. A context can be a `MethodContext` or a
`BlockContext`. A `MethodContext` represents the execution of a
`CompiledMethod` that was invoked by a message. [Figure
27.5](#Figure_27.5) shows a `MethodContext` and its `CompiledMethod`.\
[]{#Figure_27.5}

  -----------------------------------------------
  ![](figure27_5.gif){height="288" width="421"}
  **Figure 27.5**
  -----------------------------------------------

A `BlockContext` represents a block encountered in a `CompiledMethod`. A
`BlockContext` refers to the `MethodContext` whose `CompiledMethod`
contained the block it represents. This is called the
`BlockContext's`*home*. [Figure 27.6](#Figure_27.6) shows a
`BlockContext` and its home.\
    The indices used to access the fields of contexts are initialized by
the following routine.

**initializeContextIndices**

*\"Class MethodContext\"*

SenderIndex := 0.

InstructionPointerIndex := 1.

StackPointerIndex := 2.

MethodIndex := 3.

ReceiverIndex := 5.

TempFrameStart := 6.

*\"Class BlockContext\"*

CallerIndex := 0.

BlockArgumentCountIndex := 3.

InitialIPIndex := 4.

HomeIndex := 5

Both kinds of context have six fixed fields corresponding to six named
instance variables. These fixed fields are followed by some indexable
fields. The indexable fields are used to store the temporary frame
(arguments and temporary variables) followed by the contents of the
evaluation stack. The following routines are used to fetch and store the
instruction pointer and stack pointer stored in a context.\
[]{#Figure_27.6}

  -----------------------------------------------
  ![](figure27_6.gif){height="404" width="407"}
  **Figure 27.6**
  -----------------------------------------------

**instructionPointerOfContext: contextPointer**

\^self fetchInteger: InstructionPointerIndex

        ofObject: contextPointer

**storeInstructionPointerValue: value inContext: contextPointer**

self storeInteger: InstructionPointerIndex

       ofObject: contextPointer

       withValue: value

**stackPointerOfContext: contextPointer**

\^self fetchInteger: StackPointerIndex

        ofObject: contextPointer

**storeStackPointerValue: value inContext: contextPointer**

self storeInteger: StackPointerIndex

       ofObject: contextPointer

       withValue: value

A `BlockContext` stores the number of block arguments it expects in one
of its fields.

**argumentCountOfBlock: blockPointer**

\^self fetchInteger: BlockArgumentCountIndex

        ofObject: blockPointer

The context that represents the `CompiledMethod` or block currently
being executed is called the *active context*. The interpreter caches in
its registers the contents of the parts of the active context it uses
most often. These registers are:\
 

  ----------------------------------- -----------------------------------
  `activeContext`                     This is the active context itself.
                                      It is either a `MethodContext` or a
                                      `BlockContext`. 

  `homeContext`                       If the active context is a
                                      `MethodContext`, the home context
                                      is the same context. If the active
                                      context is a `BlockContext`, the
                                      home context is the contents of the
                                      home field of the active context.
                                      This will always be a
                                      `MethodContext`. 

  `method`                            This is the `CompiledMethod` that
                                      contains the bytecodes the
                                      interpreter is executing. 

  `receiver`                          This is the object that received
                                      the message that invoked the home
                                      context\'s method. 

  `instructionPointer`                This is the byte index of the next
                                      bytecode of the method to be
                                      executed. 

  `stackPointer`                      This is the index of the field of
                                      the active context containing the
                                      top of the stack.\
                                          
  ----------------------------------- -----------------------------------

  : **Context-related Registers of the Interpreter**

Whenever the active context changes (when a new `CompiledMethod` is
invoked, when a `CompiledMethod` returns or when a process switch
occurs), all of these registers must be updated using the following
routine.

**fetchContextRegisters**

(self isBlockContext: activeContext)

ifTrue: \[homeContext := memory fetchPointer: HomeIndex

       ofObject: activeContext\]

ifFalse: \[homeContext := activeContext\].

receiver := memory fetchPointer: ReceiverIndex

ofObject: homeContext.

method := memory fetchPointer: MethodIndex

         ofObject: homeContext.

instruction Pointer := (self instructionPointerOfContext:
activeContext) - 1.

stackPointer := (self stackPointerOfContext: activeContext) +
TempFrameStart - 1

Note that the receiver and method are fetched from the `homeContext` and
the `instructionPointer` and `stackPointer` are fetched from the
`activeContext`. The interpreter tells the difference between
`MethodContexts` and `BlockContexts` based on the fact that
`MethodContexts` store the method pointer (an object pointer) and
`BlockContexts` store the number of block arguments (an integer pointer)
in the same field. If this location contains an integer pointer, the
context is a `BlockContext`; otherwise, it is a `MethodContext`. The
distinction could be made on the basis of the class of the context, but
special provision would have to be made for subclasses of
`MethodContext` and `BlockContext`.

**isBlockContext: contextPointer**

\| methodOrArguments \|

methodOrArguments := memory fetchPointer: MethodIndex

     ofObject: contextPointer.

\^memory isIntegerObject: methodOrArguments

Before a new context becomes the active context, the values of the
instruction pointer and stack pointer must be stored into the active
context with the following routine.

**storeContextRegisters**

self storeInstructionPointerValue: instructionPointer + 1

       inContext: activeContext.

self storeStackPointerValue: stackPointer - TempFrameStart + 1

       inContext: activeContext

The values of the other cached registers do not change so they do not
need to be stored back into the context. The instruction pointer stored
in a context is a one-relative index to the method\'s fields because
subscripting in Smalltalk (i.e., the `at:` message) takes one-relative
indices. The memory, however, uses zero-relative indices; so the
`fetchContextRegisters` routine subtracts one to convert it to a memory
index and the `storeContextRegisters` routine adds the one back in. The
stack pointer stored in a context tells how far the top of the
evaluation stack is beyond the fixed fields of the context (i.e.. how
far after the start of the temporary frame) because subscripting in
Smalltalk takes fixed fields into account and fetches from the indexable
fields after them. The memory, however, wants an index relative to the
start of the object; so the `fetchContextRegisters` routine adds in the
offset of the start of the temporary frame (a constant) and the
`storeContextRegisters` routine subtracts the offset.\
    The following routines perform various operations on the stack of
the active context.

**push: object**

stackPointer := stackPointer + 1.

memory storePointer: stackPointer

 ofObject: activeContext

 withValue: object

**popStack**

\| stackTop \|

stackTop := memory fetchPointer: stackPointer

 ofObject: activeContext.

stackPointer := stackPointer - 1.

\^stackTop

**stackTop**

\^memory fetchPointer: stackPointer

    ofObject: activeContext

**stackValue: offset**

\^memory fetchPointer: stackPointer - offset

    ofObject: activeContext

**pop: number**

stackPointer := stackPointer - number

**unPop: number**

stackPointer := stackPointer + number

The active context register must count as a reference to the part of the
object memory that deallocates unreferenced objects. If the object
memory maintains dynamic reference counts, the routine to change active
contexts must perform the appropriate reference counting.

**newActiveContext: aContext**

self storeContextRegisters.

memory decreaseReferencesTo: activeContext.

activeContext := aContext.

memory increaseReferencesTo: activeContext.

self fetchContextRegisters

The following routines fetch fields of contexts needed by the
interpreter infrequently enough that they are not cached in registers.
The sender is the context to be returned to when a `CompiledMethod`
returns a value (either because of a \"`^`\" or at the end of the
method). Since an explicit return from within a block should return from
the `CompiledMethod` enclosing the block, the sender is fetched from the
home context.

**sender**

\^memory fetchPointer: SenderIndex

    ofObject: homeContext

The caller is the context to be returned to when a `BlockContext`
returns a value (at the end of the block).

**caller**

\^memory fetchPointer: SenderIndex

    ofObject: activeContext

Since temporaries referenced in a block are the same as those referenced
in the `CompiledMethod` enclosing the block, the temporaries are fetched
from the home context.

**temporary: offset**

\^memory fetchPointer: offset + TempFrameStart

    ofObject: homeContext

The following routine provides convenient access to the literals of the
currently executing `CompiledMethod`.

**literal: offset**

\^self literal: offset

        ofMethod: method

[]{#Classes27}***Classes***\

------------------------------------------------------------------------

The interpreter finds the appropriate `CompiledMethod` to execute in
response to a message by searching a *message dictionary*. The message
dictionary is found in the *class* of the message receiver or one of the
*superclasses* of that class. The structure of a class and its
associated message dictionary is shown in [Figure 27.7](#Figure_27.7).
In addition to the message dictionary and superclass the interpreter
uses the class\'s *instance specification* to determine its instances\'
memory requirements. The other fields of a class are used only by
Smalltalk methods and ignored by the interpreter. The following routine
initializes the indices used to access fields of classes and their
message dictionaries.\
[]{#Figure_27.7}

  -----------------------------------------------
  ![](figure27_7.gif){height="239" width="561"}
  **Figure 27.7**
  -----------------------------------------------

**initializeClassIndices**

*\"Class Class\"*

SuperclassIndex := 0.

MessageDictionaryIndex := 1.

InstanceSpecificationIndex := 2.

*\"Fields of a message dictionary\"*

MethodArrayIndex := 1.

SelectorStart := 2

The interpreter uses several registers to cache the state of the message
lookup process.\
 

  ----------------------------------- -----------------------------------
  `messageSelector`                   This is the selector of the message
                                      being sent. It is always a
                                      `Symbol`. 

  `argumentCount`                     This is the number of arguments in
                                      the message currently being sent.
                                      It indicates where the message
                                      receiver can be found on the stack
                                      since it is below the arguments. 

  `newMethod`                         This is the method associated with
                                      the `messageSelector`. 

  `primitiveIndex`                    This is the index of a primitive
                                      routine associated with `newMethod`
                                      if one exists.\
                                           
  ----------------------------------- -----------------------------------

  : **Class-related Registers of the Interpreter**

A message dictionary is an `IdentityDictionary`. `IdentityDictionary` is
a subclass of `Set` with an additional `Array` containing values
associated with the contents of the `Set`. The message selectors are
stored in the indexed instance variables inherited from `Set`. The
`CompiledMethods` are stored in an `Array` added by
`IdentityDictionary`. A `CompiledMethod` has the same index in that
`Array` that its selector has in the indexable variables of the
dictionary object itself. The index at which to store the selector and
`CompiledMethod` are computed by a hash function.\
    The selectors are instances of `Symbol`, so they may be tested for
equality by testing their object pointers for equality. Since the object
pointers of `Symbols` determine equality, the hash function may be a
function of the object pointer. Since object pointers are allocated
quasirandomly, the object pointer itself is a reasonable hash function.
The pointer shifted right one bit will produce a better hash function,
since all object pointers other than `SmallIntegers` are even.

**hash: objectPointer**

\^objectPointer bitShift: -1

The message selector lookup assumes that methods have been put into the
dictionary using the same hashing function. The hashing algorithm
reduces the original hash function modulo the number of indexable
locations in the dictionary. This gives an index in the dictionary. To
make the computation of the modulo reduction simple, message
dictionaries have an exact power of two fields. Therefore the modulo
calculation can be performed by masking off an appropriate number of
bits. If the selector is not found at the initial hash location,
successive fields are examined until the selector is found or a `nil` is
encountered. If a `nil` is encountered in the search, the selector is
not in the dictionary. If the end of the dictionary is encountered while
searching, the search wraps around and continues with the first field.\
    The following routine looks in a dictionary for a `CompiledMethod`
associated with the `Symbol` in the `messageSelector` register. If it
finds the `Symbol`, it stores the associated `CompiledMethod's` pointer
into the `newMethod` register, its primitive index into the
`primitiveIndex` register and returns `true`. If the `Symbol` is not
found in the dictionary, the routine returns `false`. Since finding a
`nil` or an appropriate `Symbol` are the only exit conditions of the
loop, the routine must check for a full dictionary (i.e., no `nils`). It
does this by keeping track of whether it has wrapped around. If the
search wraps around twice, the selector is not in the dictionary.

**lookupMethodInDictionary: dictionary**

\| length index mask wrapAround nextSelector methodArray \|

length := memory fetchWordLengthOf: dictionary.

mask := length - SelectorStart - 1.

index := (mask bitAnd: (self hash: messageSelector)) + SelectorStart.

wrapAround := false.

\[true\] whileTrue:

\[nextSelector := memory fetchPointer: index

      ofObject: dictionary.

nextSelector = NilPointer ifTrue: \[\^false\].

nextSelector = messageSelector

ifTrue: \[methodArray := memory fetchPointer: MethodArrayIndex

      ofObject: dictionary.

newMethod := memory fetchPointer: index - SelectorStart

 ofObject: methodArray.

primitiveIndex := self primitiveIndexOf: newMethod.

\^true\].

index := index + 1.

index = length

ifTrue: \[wrapAround ifTrue: \[\^false\].

wrapAround := true.

index := SelectorStart\]\]

This routine is used in the following routine to find the method a class
associates with a selector. If the selector is not found in the initial
class\'s dictionary, it is looked up in the next class on the superclass
chain. The search continues up the superclass chain until a method is
found or the superclass chain is exhausted.

**lookupMethodInClass: class**

\| currentClass dictionary \|

currentClass := class.

\[currentClass \~= NilPointer\]  whileTrue:

\[dictionary := memory fetchPointer: MessageDictionaryIndex

 ofObject: currentClass.

(self lookupMethodInDictionary: dictionary)

ifTrue: \[\^true\].

currentClass := self superclassOf: currentClass\].

messageSelector = DoesNotUnderstandSelector

ifTrue: \[self error: \'Recursive not understood error encountered\'\].

self createActualMessage.

messageSelector := DoesNotUnderstandSelector.

\^self lookupMethodInClass: class

**superclassOf: classPointer**

\^memory fetchPointer: SuperclassIndex

    ofObject: classPointer

The interpreter needs to do something out of the ordinary when a message
is sent to an object whose class and superclasses do not contain a
`CompiledMethod` associated with the message selector. In keeping with
the philosophy of Smalltalk, the interpreter sends a message. A
`CompiledMethod` for this message is guaranteed to be found. The
interpreter packages up the original message in an instance of class
`Message` and then looks for a `CompiledMethod` associated with the
selector `doesNotUnderstand:`. The `Message` becomes the single argument
for the `doesNotUnderstand:` message. The `doesNotUnderstand:` message
is defined in `Object` with a `CompiledMethod` that notifies the user.
This `CompiledMethod` can be overridden in a user-defined class to do
something else. Because of this, the `lookupMethodInClass:` routine will
always complete by storing a pointer to a `CompiledMethod` in the
`newMethod` register.

**createActualMessage**

\| argumentArray message \|

argumentArray := memory instantiateClass: ClassArrayPointer

withPointers: argumentCount.

message := memory instantiateClass: ClassMessagePointer

withPointers: self messageSize.

memory storePointer: MessageSelectorIndex

   ofObject: message

   withValue: messageSelector.

memory storePointer: MessageArgumentsIndex

   ofObject: message

   withValue: argumentArray.

self transfer: argumentCount

      fromField: stackPointer - (argumentCount - 1)

      ofObject: activeContext

      toField: 0

      ofObject: argumentArray.

self pop: argumentCount.

self push: message.

argumentCount := 1

The following routine initializes the indices used to access fields of a
`Message`.

**initializeMessageIndices**

MessageSelectorIndex := 0.

MessageArgumentsIndex := 1.

MessageSize := 2

The instance specification field of a class contains a `SmallInteger`
pointer that encodes the following four pieces of information:

1.  Whether the instances\' fields contain object pointers or numerical
    values
2.  Whether the instances\' fields are addressed in word or byte
    quantities
3.  Whether the instances have indexable fields beyond their fixed
    fields
4.  The number of fixed fields the instances have

[Figure 27.8](#Figure_27.8) shows how this information is encoded in the
instance specification.\
[]{#Figure_27.8}

  ----------------------------------------------
  ![](figure27_8.gif){height="85" width="439"}
  **Figure 27.8**
  ----------------------------------------------

The four pieces of information are not independent. If the instances\'
fields contain object pointers, they will be addressed in word
quantities. If the instances\' fields contain numerical values, they
will have indexable fields and no fixed fields.

**instanceSpecificationOf: classPointer**

\^memory fetchPointer: InstanceSpecificationIndex

    ofObject: classPointer

**isPointers: classPointer**

\| pointersFlag \|

pointersFlag := self extractBits: 0 to: 0

         of: (self instanceSpecificationOf: classPointer).

\^pointersFlag = 1

**isWords: classPointer**

\| wordsFlag \|

wordsFlag := self extractBits: 1 to: 1

      of: (self instanceSpecificationOf: classPointer)

\^wordsFlag = 1

**isIndexable: classPointer**

\| indexableFlag \|

indexableFlag := self extractBits: 2 to: 2

 of: (self instanceSpecificationOf: classPointer).

\^indexableFlag = 1

**fixedFieldsOf: classPointer**

\^self extractBits: 4 to: 14

        of: (self instanceSpecificationOf: classPointer)

Note: the instance specification of `CompiledMethod` does not accurately
reflect the structure of its instances since `CompiledMethods` are not
homogeneous. The instance specification says that the instances do not
contain pointers and are addressed by bytes. This is true of the
bytecode section of a `CompiledMethod` only. The storage manager needs
to know that `CompiledMethods` are special and actually contain some
pointers. For all other classes, the instance specification is
accurate.\

------------------------------------------------------------------------

\[[TOC](bluebook_imp_toc.html)\] \[[26](bluebook_chapter26.html)\]
\[[27](#top_of_27)\] \[[28](bluebook_chapter28.html)\]
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]
