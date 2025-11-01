---
GENERATOR: Mozilla/4.51 \[en\] (WinNT; I) \[Netscape\]
title: Blue Book Chapter 28
---

[]{#top_of_28}\[[TOC](bluebook_imp_toc.html)\]
\[[26](bluebook_chapter26.html)\] \[[27](bluebook_chapter27.html)\] 28
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]\

------------------------------------------------------------------------

28\
Formal Specification of the Interpreter

------------------------------------------------------------------------

**[Stack Bytecodes](#StackBytecodes28)**

**[Jump Bytecodes](#JumpBytecodes28)**

**[Send Bytecodes](#SendBytecodes28)**

**[Return Bytecodes](#ReturnBytecodes28)**

------------------------------------------------------------------------

The main loop of the Smalltalk-80 interpreter fetches bytecodes from a
`CompiledMethod` sequentially and dispatches to routines that perform
the operations the bytecodes indicate. The `fetchByte` routine fetches
the byte indicated by the active context\'s instruction pointer and
increments the instruction pointer.

**fetchByte**

\| byte \|

byte := memory fetchByte: instructionPointer

  ofObject: method.

instructionPointer := instructionPointer + 1.

\^byte

Since process switches are only allowed between bytecodes, the first
action in the interpreter\'s main loop is to call a routine that
switches processes if necessary. The `checkProcessSwitch` routine will
be described with the process scheduling primitive routines in the [next
chapter](bluebook_chapter29.html). After checking for a process switch,
a bytecode is fetched (perhaps from a new process), and a dispatch is
made to the appropriate routine.

**interpret**

\[true\] whileTrue: \[self cycle\]

 

**cycle**

self checkProcessSwitch.

currentBytecode := self fetchByte.

self dispatchOnThisBytecode

The table on page 595 lists the [Smalltalk-80
bytecodes](#st80_bytecodes_table). The bytecodes are listed in ranges
that have similar function. For example, the first range includes the
bytecodes from 0 through 15 and its entry is shown below.\
 

  ------ ------------ ---------------------------------
  0-15   `0000iiii`   Push Receiver Variable \#`iiii`
  ------ ------------ ---------------------------------

    Each range of bytecodes is listed with a bit pattern and a comment
about the function of the bytecodes. The bit pattern shows the binary
representation of the bytecodes in the range. 0s and 1s are used in bit
locations that have the same value for all bytecodes in the range. Since
all numbers from 0 through 15 have four zeros in their high order bits,
these bits are shown as 0000. Lower case letters are used in bit
locations whose values vary within the range. The value of each letter
can be either 0 or 1. The letters used in the pattern can be included in
the comment to refer to the value of those bits in a specific bytecode
in the range. The comment for the first range of bytecodes indicates
that the low-order four bits of the bytecode specify the index of one of
the receiver\'s variables to be pushed on the stack.\
    The variable bits in a bit pattern are also sometimes used as a
zero-relative index into a list included in the comment. For example,
the entry\
 

  --------- ------------ -------------------------------------------------------------
  120-123   `011110ii`   Return (receiver`, true, false, nil`) \[`ii`\] From Message
  --------- ------------ -------------------------------------------------------------

specifies that the bytecode 120 returns the receiver, bytecode 121
returns `true`, bytecode 122 returns `false` and bytecode 123 returns
`nil`.\
    The entries for bytecodes that take extensions will include more
than one bit pattern. For example,\
 

  ----- --------------------- ------------------------------------------------------
  131   `10000011 jjjkkkkk`   Send Literal Selector \#`kkkkk` With `jjj` Arguments
  ----- --------------------- ------------------------------------------------------

There are four basic types of bytecode.

-   *stack bytecodes* move object pointers between the object memory and
    the active context\'s evaluation stack. These include both the push
    bytecodes and store bytecodes described in Chapter 26.
-   *jump bytecodes* change the instruction pointer of the active
    context.
-   *send bytecodes* invoke `CompiledMethods` or primitive routines.
-   *return bytecodes* terminate the execution of `CompiledMethods`.

Not all of the bytecodes of one type are contiguous, so the main
dispatch has seven branches each of which calls one of four routines
(`stackBytecode`, `jumpBytecode`, `sendBytecode`, or `returnBytecode`).
These four routines will be described in the next four subsections.

**dispatchOnThisBytecode**

(currentBytecode between: 0 and: 119) ifTrue: \[\^self stackBytecode\].

(currentBytecode between: 120 and: 127) ifTrue: \[\^self
returnBytecode\].

(currentBytecode between: 128 and: 130) ifTrue: \[\^self
stackBytecode\].

(currentBytecode between: 131 and: 134) ifTrue: \[\^self sendBytecode\].

(currentBytecode between: 135 and: 137) ifTrue: \[\^self
stackBytecode\].

(currentBytecode between: 144 and: 175) ifTrue: \[\^self jumpBytecode\].

(currentBytecode between: 176 and: 255) ifTrue: \[\^self sendBytecode\]

The bytecodes 176-191 refer to Arithmetic Messages. These are

`+, -, <, >, <=, >=, =, ~=, *, /, \\, @, bitShift:, //, bitAnd:, bitOr:`

The bytecodes 192-207 refer to Special Messages. These are

`at:*, at:put:*, size*, next*, nextPut:*, atEnd*, ==, class, blockCopy:, value, value:, do:*, new*, new:*, x*, y*`

Selectors indicated with an asterisk (\*) can be changed by compiler
modification.\
[]{#st80_bytecodes_table}

+-----------------------+-----------------------+-----------------------+
| **The Smalltalk-80    |                       |                       |
| Bytecodes**           |                       |                       |
+-----------------------+-----------------------+-----------------------+
| **Range**             | **Bits**              | **Function**          |
|                       |                       |                       |
| -------------------   | -------------------   | -------------------   |
+-----------------------+-----------------------+-----------------------+
| `0-15`                | `0000iiii`            | Push Receiver         |
|                       |                       | Variable `#iiii`      |
+-----------------------+-----------------------+-----------------------+
| `16-31`               | `0001iiii`            | Push Temporary        |
|                       |                       | Location `#iiii`      |
+-----------------------+-----------------------+-----------------------+
| `32-63`               | `001iiiii`            | Push Literal Constant |
|                       |                       | `#iiiii`              |
+-----------------------+-----------------------+-----------------------+
| `64-95`               | `010iiiii`            | Push Literal Variable |
|                       |                       | `#iiiii`              |
+-----------------------+-----------------------+-----------------------+
| `96-103`              | `01100iii`            | Pop and Store         |
|                       |                       | Receiver Variable     |
|                       |                       | `#iii`                |
+-----------------------+-----------------------+-----------------------+
| `104-111`             | `01101iii`            | Pop and Store         |
|                       |                       | Temporary Location    |
|                       |                       | `#iii`                |
+-----------------------+-----------------------+-----------------------+
| `112-119`             | `01110iii`            | Push (receiver,       |
|                       |                       | `true`, `false`,      |
|                       |                       | `nil`, `-1`, `0`,     |
|                       |                       | `1`, `2`) \[`iii`\]   |
+-----------------------+-----------------------+-----------------------+
| `120-123`             | `011110ii`            | Return (receiver,     |
|                       |                       | `true`, `false`,      |
|                       |                       | `nil`) \[`ii`\] From  |
|                       |                       | Message               |
+-----------------------+-----------------------+-----------------------+
| `124-125`             | `0111110i`            | Return Stack Top From |
|                       |                       | (Message, Block)      |
|                       |                       | \[`i`\]               |
+-----------------------+-----------------------+-----------------------+
| `126-127`             | `0111111i`            | unused                |
+-----------------------+-----------------------+-----------------------+
| `128`                 | `10000000 jjkkkkkk`   | Push (Receiver        |
|                       |                       | Variable, Temporary   |
|                       |                       | Location, Literal     |
|                       |                       | Constant, Literal     |
|                       |                       | Variable) \[`jj`\]    |
|                       |                       | `#kkkkkk`             |
+-----------------------+-----------------------+-----------------------+
| `129`                 | `10000001 jjkkkkkk`   | Store (Receiver       |
|                       |                       | Variable, Temporary   |
|                       |                       | Location, Illegal,    |
|                       |                       | Literal Variable)     |
|                       |                       | \[`jj`\] `#kkkkkk`    |
+-----------------------+-----------------------+-----------------------+
| `130`                 | `10000010 jjkkkkkk`   | Pop and Store         |
|                       |                       | (Receiver Variable,   |
|                       |                       | Temporary Location,   |
|                       |                       | Illegal, Literal      |
|                       |                       | Variable) \[`jj`\]    |
|                       |                       | `#kkkkkk`             |
+-----------------------+-----------------------+-----------------------+
| `131`                 | `10000011 jjjkkkkk`   | Send Literal Selector |
|                       |                       | `#kkkkk` With `jjj`   |
|                       |                       | Arguments             |
+-----------------------+-----------------------+-----------------------+
| `132`                 | `100001               | Send Literal Selector |
|                       | 00 jjjjjjjj kkkkkkkk` | `#kkkkkkkk` With      |
|                       |                       | `jjjjjjjj` Arguments  |
+-----------------------+-----------------------+-----------------------+
| `133`                 | `10000101 jjjkkkkk`   | Send Literal Selector |
|                       |                       | `#kkkkk` To           |
|                       |                       | Superclass With `jjj` |
|                       |                       | Arguments             |
+-----------------------+-----------------------+-----------------------+
| `134`                 | `100001               | Send Literal Selector |
|                       | 10 jjjjjjjj kkkkkkkk` | `#kkkkkkkk` To        |
|                       |                       | Superclass With       |
|                       |                       | `jjjjjjjj` Arguments  |
+-----------------------+-----------------------+-----------------------+
| `135`                 | `10000111`            | Pop Stack Top         |
+-----------------------+-----------------------+-----------------------+
| `136`                 | `10001000`            | Duplicate Stack Top   |
+-----------------------+-----------------------+-----------------------+
| `137`                 | `10001001`            | Push Active Context   |
+-----------------------+-----------------------+-----------------------+
| `138-143`             |                       | unused                |
+-----------------------+-----------------------+-----------------------+
| `144-151`             | `10010iii`            | Jump `iii` `+` `1`    |
|                       |                       | (i.e., 1 through 8)   |
+-----------------------+-----------------------+-----------------------+
| `152-159`             | `10011iii`            | Pop and Jump 0n False |
|                       |                       | `iii` `+1` (i.e., 1   |
|                       |                       | through 8)            |
+-----------------------+-----------------------+-----------------------+
| `160-167`             | `10100iii jjjjjjjj`   | Jump(`iii` `-` `4`)   |
|                       |                       | `*256+jjjjjjjj`       |
+-----------------------+-----------------------+-----------------------+
| `168-171`             | `101010ii jjjjjjjj`   | Pop and Jump On True  |
|                       |                       | `ii` `*256+jjjjjjjj`  |
+-----------------------+-----------------------+-----------------------+
| `172-175`             | `101011ii jjjjjjjj`   | Pop and Jump On False |
|                       |                       | `ii` `*256+jjjjjjjj`  |
+-----------------------+-----------------------+-----------------------+
| `176-191`             | `1011iiii`            | Send Arithmetic       |
|                       |                       | Message `#iiii`       |
+-----------------------+-----------------------+-----------------------+
| `192-207`             | `1100iiii`            | Send Special Message  |
|                       |                       | `#iiii`               |
+-----------------------+-----------------------+-----------------------+
| `208-223`             | `1101iiii`            | Send Literal Selector |
|                       |                       | `#iiii` With `No`     |
|                       |                       | Arguments             |
+-----------------------+-----------------------+-----------------------+
| `224-239`             | `1110iiii`            | Send Literal Selector |
|                       |                       | `#iiii` With `1`      |
|                       |                       | Argument              |
+-----------------------+-----------------------+-----------------------+
| `240-255`             | `1111iiii`            | Send Literal Selector |
|                       |                       | `#iiii` With `2`      |
| -------------------   | -------------------   | Arguments             |
|                       |                       |                       |
|                       |                       | -------------------   |
+-----------------------+-----------------------+-----------------------+

------------------------------------------------------------------------

[]{#StackBytecodes28}**Stack Bytecodes**

------------------------------------------------------------------------

The stack bytecodes all perform simple operations on the active
context\'s evaluation stack.

-   107 bytecodes *push* an object pointer on the stack
    -   99 push an object pointer found in the object memory
    -   7 push a constant object pointer
    -   1 pushes the interpreter\'s active context register
        (`activeContext`)
-   18 bytecodes *store* an object pointer found on the stack into the
    object memory
    -   17 of these also remove it from the stack
    -   1 leaves it on the stack
-   1 bytecode *removes* an object pointer from the stack without
    storing it anywhere.

The routines used to manipulate the stack were described in the section
of the previous chapter on contexts (`push:`, `popStack`, `pop:`). The
`stackBytecode` routine dispatches to the appropriate routine for the
current bytecode.

**stackBytecode**

(currentBytecode between: 0 and: 15)

ifTrue: \[\^self pushReceiverVariableBytecode\].

(currentBytecode between: 16 and; 31)

ifTrue: \[\^self pushTemporaryVariableBytecode\].

(currentBytecode between: 32 and: 63)

ifTrue: \[\^self pushLiteralConstantBytecode\].

(currentBytecode between: 64 and: 95)

ifTrue: \[\^self pushLiteralVariableBytecode\].

(currentBytecode between: 96 and: 103)

ifTrue: \[\^self storeAndPopReceiverVariableBytecode\].

(currentBytecode between: 104 and: 111)

ifTrue: \[\^self storeAndPopTemporaryVariableBytecode\].

currentBytecode = 112

ifTrue: \[\^self pushReceiverBytecode\].

(currentBytecode between: 113 and: 119)

ifTrue: \[\^self pushConstantBytecode\].

currentBytecode = 128

ifTrue: \[\^self extendedPushBytecode\].

currentBytecode = 129

ifTrue: \[\^self extendedStoreBytecode\].

currentBytecode = 130

ifTrue: \[\^self extendedStoreAndPopBytecode\].

currentBytecode = 135

ifTrue: \[\^self popStackBytecode\].

currentBytecode = 136

ifTrue: \[\^self duplicateTopBytecode\].

currentBytecode = 137

ifTrue: \[\^self pushActiveContextBytecode\]

There are single byte instructions that push the first 16 instance
variables of the receiver and the first 16 temporary frame locations.
Recall that the temporary frame includes the arguments and the temporary
variables.

**pushReceiverVariableBytecode**

\| fieldIndex \|

fieldIndex := self extractBits: 12 to: 15

      of: currentBytecode.

self pushReceiverVariable: fieldIndex

**pushReceiverVariable: fieldIndex**

self push: (memory fetchPointer: fieldIndex

         ofObject: receiver)

**pushTemporaryVariableBytecode**

\| fieldIndex \|

fieldIndex := self extractBits: 12 to: 15

      of: currentBytecode.

self pushTemporaryVariable: fieldIndex

**pushTemporaryVariable:** temporaryIndex

self push: (self temporary: temporaryIndex)

There are also single byte instructions that reference the first 32
locations in the literal frame of the active context\'s method. The
contents of one of these locations can be pushed with
`pushLiteralConstantBytecode`. The contents of the value field of an
`Association` stored in one of these locations can be pushed with
`pushLiteralVariableBytecode`.

**pushLiteralConstantBytecode**

\| fieldIndex \|

fieldIndex := self extractBits: 11 to: 15

      of: currentBytecode.

self pushLiteralConstant: fieldIndex

**pushLiteralConstant: literalIndex**

self push: (self literal: literalIndex)

**pushLiteralVariableBytecode**

\| fieldIndex \|

fieldIndex := self extractBits: 11 to: 15

      of: currentBytecode.

self pushLiteralVariable: fieldIndex

**pushLiteralVariable:** literalIndex

\| association \|

association := self literal: literalIndex.

self push: (memory fetchPointer: ValueIndex

       ofObject: association)

`Associations` are objects with two fields, one for a name and one for a
value. They are used to implement shared variables (global variables,
class variables, and pool variables). The following routine initializes
the index used to fetch the value field of `Associations`.

**initializeAssociationIndex**

ValueIndex := 1

The extended push bytecode can perform any of the four operations
described above (receiver variable, temporary frame location, literal
constant, or literal variable). However, instead of a limit of 16 or 32
variables accessible, it can access up to 64 instance variables,
temporary locations, literal constants, or literal variables. The
extended push bytecode is followed by a byte whose high order two bits
determine which type of push is being done and whose low order six bits
determine the offset to use.

**extendedPushBytecode**

\| descriptor variableType variableIndex \|

descriptor := self fetchByte.

variableType := self extractBits: 8 to: 9

of: descriptor.

variableIndex := self extractBits: 10 to: 15

 of: descriptor.

variableType = 0 ifTrue: \[\^self pushReceiverVariable: variableIndex\].

variableType = 1 ifTrue: \[\^self pushTemporaryVariable:
variableIndex\].

variableType = 2 ifTrue: \[\^self pushLiteralConstant: variableIndex\].

variableType = 3 ifTrue: \[\^self pushLiteralVariable: variableIndex\]

The `pushReceiverBytecode` routine pushes a pointer to the active
context\'s receiver. This corresponds to the use of `self` or `super` in
a Smalltalk method.

**pushReceiverBytecode**

\^self push: receiver

The `duplicateTopBytecode` routine pushes another copy of the object
pointer on the top of the stack.

**duplicateTopBytecode**

\^self push: self stackTop

The `pushConstantBytecode` routine pushes one of seven constant object
pointers (`true`, `false`, `nil`, `-1`, `0`, `1`, or `2`).

**pushConstantBytecode**

currentBytecode = 113 ifTrue: \[\^self push: TruePointer\].

currentBytecode = 114 ifTrue: \[\^self push: FalsePointer\].

currentBytecode = 115 ifTrue: \[\^self push: NilPointer\].

currentBytecode = 116 ifTrue: \[\^self push: MinusOnePointer\].

currentBytecode = 117 ifTrue: \[\^self push: ZeroPointer\].

currentBytecode = 118 ifTrue: \[\^self push: OnePointer\].

currentBytecode = 119 ifTrue: \[\^self push: TwoPointer\]

The `pushActiveContextBytecode` routine pushes a pointer to the active
context itself. This corresponds to the use of `thisContext` in a
Smalltalk method.

**pushActiveContextBytecode**

\^self push: activeContext

The store bytecodes transfer references in the opposite direction from
the push bytecodes; from the top of the stack to the receiver\'s
instance variables, the temporary frame, or the literal frame. There are
single byte versions for storing into the first eight variables of the
receiver or the temporary frame and then popping the stack.

**storeAndPopReceiverVariableBytecode**

\| variableIndex \|

variableIndex := self extractBits: 13 to: 15

 of: currentBytecode.

memory storePointer: variableIndex

   ofObject: receiver

   withValue: self popStack

**storeAndPopTemporaryVariableBytecode**

\| variableIndex \|

variableIndex := self extractBits: 13 to: 15

 of: currentBytecode.

memory storePointer: variableIndex + TempFrameStart

   ofObject: homeContext

   withValue: self popStack

Stores into variables other than those accessible by the single byte
versions are accomplished by two extended store bytecodes. One pops the
stack after storing and the other does not. Both extended stores take a
following byte of the same form as the extended push. It is illegal,
however, to follow an extended store with a byte of the form `10xxxxxx`
since this would mean changing the value of a literal constant.

**extendedStoreAndPopBytecode**

self extendedStoreBytecode.

self popStackBytecode

**extendedStoreBytecode**

\| descriptor variableType variableIndex association \|

descriptor := self fetchByte.

variableType := self extractBits: 8 to: 9

of: descriptor.

variableIndex := self extractBits: 10 to: 15

 of: descriptor.

variableType = 0 ifTrue:

\[\^memory storePointer: variableIndex

      ofObject: receiver

      withValue: self stackTop\].

variableType = 1 ifTrue:

\[\^memory storePointer: variableIndex + TempFrameStart

      ofObject: homeContext

      withValue: self stackTop\].

variableType=2  ifTrue:

\[\^self error: \'illegal store\'\].

variableType=3  ifTrue:

\[association := self literal: variableIndex.

\^memory storePointer: ValueIndex

    ofObject: association

    withValue: self stackTop\]

The last stack bytecode removes the top object pointer from the stack
without doing anything else with it.

**popStackBytecode**

self popStack

------------------------------------------------------------------------

[]{#JumpBytecodes28}**Jump Bytecodes**

------------------------------------------------------------------------

The jump bytecodes change the active context\'s instruction pointer by a
specified amount. Unconditional jumps change the instruction pointer
whenever they are encountered. Conditional jumps only change the
instruction pointer if the object pointer on the top of the stack is a
specified Boolean object (either `true` or `false`). Both unconditional
and conditional jumps have a short (single-byte) and a long (two-byte)
form.

**jumpBytecode**

(currentBytecode between: 144 and: 151)

ifTrue: \[\^self shortUnconditionalJump\].

(currentBytecode between: 152 and: 159)

ifTrue: \[\^self shortConditionalJump\].

(currentBytecode between: 160 and: 167)

ifTrue: \[\^self longUnconditionalJump\].

(currentBytecode between: 168 and: 175)

ifTrue: \[\^self longConditionalJump\]

The jump bytecodes use the `jump:` routine to actually change the
bytecode index.

**jump: offset**

instructionPointer := instructionPointer + offset

The eight short unconditional jumps advance the instruction pointer by 1
through 8.

**shortUnconditionalJump**

\| offset \|

offset := self extractBits: 13 to: 15

of: currentBytecode.

self jump: offset + 1

The eight long unconditional jumps are followed by another byte. The low
order three bits of the jump bytecode provide the high order three bits
of an 11-bit twos complement displacement to be added to the instruction
pointer. The byte following the jump provides the low order eight bits
of the displacement. So long unconditional jumps can jump up to 1023
forward and 1024 back.

**longUnconditionalJump**

\| offset \|

offset := self extractBits 13 to: 15

of: currentBytecode.

self jump: offset - 4 \* 256 + self fetchByte

The conditional jumps use the `jumpIf:by:` routine to test the top of
the stack and decide whether to perform the jump. The top of stack is
discarded after it is tested.

**jumpIf: condition by: offset**

\| boolean \|

boolean := self popStack.

boolean = condition

ifTrue: \[self jump: offset\]

ifFalse: \[(boolean = TruePointer) \| (boolean = FalsePointer)

ifFalse: \[self unPop: 1.

self sendMustBeBoolean\]\]

The conditional jumps are used in the compiled form of messages to
booleans (e.g., `ifTrue:` and `whileFalse:`). If the top of the stack at
the time of a conditional jump is not `true` or `false` it is an error
since an object other than a boolean has been sent a message that only
booleans understand. Instead of sending `doesNotUnderstand:`, the
interpreter sends `mustBeBoolean` to it.

**sendMustBeBoolean**

self sendSelector: MustBeBooleanSelector

      argumentCount: 0

The `sendSelector:argumentCount:` routine is described in the next
section on send bytecodes.\
        The eight short conditional jumps advance the instruction
pointer by 1 through 8 if the top of the stack is `false`.

**shortConditionalJump**

\| offset \|

offset := self extractBits: 13 to: 15

of: currentBytecode.

self jumpIf: FalsePointer

       by: offset + 1

So, there are three possible outcomes to a short conditional jump:

-   If the top of the stack is `false`, the jump is taken.
-   If the top of the stack is `true`, the jump is not taken.
-   If the top of the stack is neither, `mustBeBoolean` is sent to it.

Half of the long conditional jumps perform the jump if the top of the
stack is `false` while the other half perform the jump if it is `true`.
The low order two bits of the bytecode become the high order two bits of
a 10-bit unsigned displacement. The byte following the jump provides the
low order eight bits of the displacement. So long conditional jumps can
jump up to 1023 forward.

**longConditionalJump**

\| offset \|

offset := self extractBits: 14 to: 15

of: currentBytecode.

offset := offset \* 256 + self fetchByte.

(currentBytecode between: 168 and: 171)

ifTrue: \[\^self jumpIf: TruePointer by: offset\].

(currentBytecode between: 172 and: 176)

ifTrue: \[\^self jumpIf: FalsePointer by: offset\]

------------------------------------------------------------------------

[]{#SendBytecodes28}**Send Bytecodes**

------------------------------------------------------------------------

The send bytecodes cause a message to be sent. The object pointers for
the receiver and the arguments of the message are found on the active
context\'s evaluation stack. The send bytecode determines the selector
of the message and how many arguments to take from the stack. The number
of arguments is also indicated in the `CompiledMethod` invoked by the
message. The compiler guarantees that this information is redundant
except when a `CompiledMethod` is reached by a `perform:` message, in
which case it is checked to make sure the `CompiledMethod` takes the
right number of arguments. The `perform:` messages will be discussed in
the next chapter in a section on control primitives.\
    The selectors of most messages are found in the literal frame of the
`CompiledMethod`. The literal-selector bytecodes and the extended-send
bytecodes specify the argument count of the message and the index of the
selector in the literal frame. The 32 special-selector bytecodes specify
the offset of the selector and argument count in an `Array` in the
object memory. This `Array` is shared by all `CompiledMethods` in the
system.

**sendBytecode**

(currentBytecode between: 131 and: 134)

ifTrue: \[\^self extendedSendBytecode\].

(currentBytecode between: 176 and: 207)

ifTrue: \[\^self sendSpecialSelectorBytecode\].

(currentBytecode between: 208 and: 255)

ifTrue: \[\^self sendLiteralSelectorBytecode\]

The literal-selector bytecodes are single bytes that can specify 0, 1,
or 2 arguments and a selector in any one of the first 16 locations of
the literal frame. Both the selector index and the argument count are
encoded in the bits of the bytecode.

**sendLiteralSelectorBytecode**

\| selector \|

selector := self literal: (self extractBits: 12 to: 15

 of: currentBytecode).

self sendSelector: selector

      argumentCount: (self extractBits: 10 to: 11

      of: currentBytecode) - 1

Most of the send bytecodes call the `sendSelector:argumentCount:`
routine after determining the appropriate selector and argument count.
This routine sets up the variables `messageSelector` and
`argumentCount`, which are available to the other routines in the
interpreter that will lookup the message and perhaps activate a method.

**sendSelector: selector argumentCount: count**

\| newReceiver \|

messageSelector := selector.

argumentCount := count.

newReceiver := self stackValue: argumentCount.

self sendSelectorToClass: (memory fetchClassOf: newReceiver)

**sendSelectorToClass:** classPointer

self findNewMethodInClass: classPointer.

self executeNewMethod

The interpreter uses a method cache to reduce the number of dictionary
lookups necessary to find `CompiledMethods` associated with selectors.
The method cache may be omitted by substituting a call on
`lookupMethodInClass:` for the call on `findNewMethodInClass:` in
`sendSelectorToClass:` above. The `lookupMethodInClass:` routine is
described in the previous chapter in a section on classes. The cache may
be implemented in various ways. The following routine uses four
sequential locations in an `Array` for each entry. The four locations
store the selector, class, `CompiledMethod`, and primitive index for the
entry. This routine does not allow for reprobes.

**findNewMethodInClass: class**

\| hash \|

hash := (((messageSelector bitAnd: class) bitAnd: 16rFF) bitShift: 2) +
1.

((methodCache at: hash) = messageSelector

      and: \[(methodCache at: hash + 1) = class\])

ifTrue: \[newMethod := methodCache at: hash + 2.

primitiveIndex := methodCache at: hash + 3\]

ifFalse: \[self lookupMethodInClass: class.

methodCache at: hash put: messageSelector.

methodCache at: hash + 1 put: class.

methodCache at: hash + 2 put: newMethod.

methodCache at: hash + 3 put: primitiveIndex\]

The method cache is initialized with the following routine.

**initializeMethodCache**

methodCacheSize := 1024.

methodCache := Array new: methodCacheSize

The `executeNewMethod` routine calls a primitive routine if one is
associated with the `CompiledMethod`. The `primitiveResponse` routine
returns `false` if no primitive is indicated or the primitive routine is
unable to produce a result. In that case, the `CompiledMethod` is
activated. Primitive routines and the `primitiveResponse` routine will
be described in the [next chapter](bluebook_chapter29.html).

**executeNewMethod**

self primitiveResponse

ifFalse: \[self activateNewMethod\]

The routine that activates a method creates a `MethodContext` and
transfers the receiver and arguments from the currently active
context\'s stack to the new context\'s stack, It then makes this new
context be the interpreter\'s active context.

**activateNewMethod**

\| contextSize newContext newReceiver \|

(self largeContextFlagOf: newMethod) = 1

ifTrue: \[contextSize := 32 + TempFrameStart\]

ifFalse: \[contextSize := 12 + TempFrameStart\].

newContext := memory instantiateClass: ClassMethodContextPointer

  withPointers: contextSize.

memory storePointer: SenderIndex

   ofObject: newContext

   withValue: activeContext.

self storeInstructionPointerValue: (self
initialInstructionPointerOfMethod: newMethod)

       inContext: newContext.

self storeStackPointerValue: (self temporaryCountof: newMethod)

       inContext: newContext.

memory storePointer: MethodIndex

   ofObject: newContext

   withValue: newMethod.

self transfer: argumentCount + 1

      fromIndex: stackPointer - argumentCount

      ofObject: activeContext

      toIndex: receiverIndex

      ofObject: newContext.

self pop: argumentCount + 1.

self newActiveContext: newContext

There are four extended-send bytecodes. The first two have the same
effect as the literal-selector bytecodes except that the selector index
and argument count are found in one or two following bytes instead of in
the bytecode itself. The other two extended-send bytecodes are used for
superclass messages.

**extendedSendBytecode**

currentBytecode = 131 ifTrue: \[\^self singleExtendedSendBytecode\].

currentBytecode = 132 ifTrue: \[\^self doubleExtendedSendBytecode\].

currentBytecode = 133 ifTrue: \[\^self singleExtendedSuperBytecode\].

currentBytecode = 134 ifTrue: \[\^self doubleExtendedSuperBytecode\]

The first form of extended send is followed by a single byte specifying
the number of arguments in its high order three bits and selector index
in the low order five bits.

**singleExtendedSendBytecode**

\| descriptor selectorIndex \|

descriptor := self fetchByte.

selectorIndex := self extractBits: 11 to: 15

 of: descriptor.

self sendSelector: (self literal: selectorIndex)

      argumentCount: (self extractBits: 8 to: 10

        of: descriptor)

The second form of extended send bytecode is followed by two bytes; the
first is the number of arguments and the second is the index of the
selector in the literal frame.

**doubleExtendedSendBytecode**

\| count selector \|

count := self fetchByte.

selector := self literal: self fetchByte.

self sendSelector: selector

      argumentCount: count

When the compiler encounters a message to super in a symbolic method, it
uses the bytecode that pushes self for the receiver, but it uses an
extended-super bytecode to indicate the selector instead of a regular
send bytecode. The two extended-super bytecodes are similar to the two
extended-send bytecodes. The first is followed by a single byte and the
second by two bytes that are interpreted exactly as for the
extended-send bytecodes. The only difference in what these bytecodes do
is that they start the message lookup in the superclass of the class in
which the current `CompiledMethod` was found. Note that this is not
necessarily the immediate superclass of self. Specifically, it will not
be the immediate superclass of self if the `CompiledMethod` containing
the extended-super bytecode was found in a superclass of self
originally. All `CompiledMethods` that contain extended-super bytecodes
have the class in which they are found as their last literal variable.

**singleExtendedSuperBytecode**

\| descriptor selectorIndex methodClass \|

descriptor := self fetchByte.

argumentCount := self extractBits: 8 to: 10

  of: descriptor.

selectorIndex := self extractBits: 11 to: 15

 of: descriptor.

messageSelector := self literal: selectorIndex.

methodClass := self methodClassOf: method.

self sendSelectorToClass: (self superclassOf: methodClass)

**doubleExtendedSuperBytecode**

\| methodClass \|

argumentCount := self fetchByte.

messageSelector := self literal: self fetchByte.

methodClass := self methodClassOf: method.

self sendSelectorToClass: (self superclassOf: methodClass)

The set of special selectors can be used in a message without being
included in the literal frame. An `Array` in the object memory contains
the object pointers of the selectors in alternating locations. The
argument count for each selector is stored in the location following the
selector\'s object pointer. The `specialSelectorPrimitiveResponse`
routine will be described in the next chapter.

**sendSpecialSelectorBytecode**

\| selectorIndex selector count \|

self specialSelectorPrimitiveResponse

ifFalse: \[selectorIndex := (currentBytecode - 176) \* 2.

selector := memory fetchPointer: selectorIndex

        ofObject: SpecialSelectorsPointer.

count := self fetchInteger: selectorIndex + 1

ofObject: SpecialSelectorsPointer.

self sendSelector: selector

      argumentCount: count\]

------------------------------------------------------------------------

[]{#ReturnBytecodes28}**Return Bytecodes**

------------------------------------------------------------------------

There are six bytecodes that return control and a value from a context;
five return the value of a message (invoked explicitly by \"\^\" or
implicitly at the end of a method) and the other one returns the value
of a block (invoked implicitly at the end of a block). The distinction
between the two types of return is that the former returns to the sender
of the home context while the latter returns to the caller of the active
context. The values returned from the five return bytecodes are: the
receiver (`self`), `true`, `false`, `nil`, or the top of the stack. The
last return bytecode returns the top of the stack as the value of a
block.

**returnBytecode**

currentBytecode = 120

ifTrue: \[\^self returnValue: receiver to: self sender\].

currentBytecode = 121

ifTrue: \[\^self returnValue: TruePointer to: self sender\].

currentBytecode = 122

ifTrue: \[\^self returnValue: FalsePointer to: self sender\].

currentBytecode = 123

ifTrue: \[\^self returnValue: NilPointer to: self sender\].

currentBytecode = 124

ifTrue: \[\^self returnValue: self popStack to: self sender\].

currentBytecode = 125

ifTrue: \[\^self returnValue: self popStack to: self caller\]

The simple way to return a value to a context would be to simply make it
the active context and push the value on its stack.

**simpleReturnValue: resultPointer to: contextPointer**

self newActiveContext: contextPointer.

self push: resultPointer

However, there are three situations in which this routine is too simple
to work correctly. If the sender of the active context were `nil`; this
routine would store a `nil` in the interpreter\'s active context
pointer, bringing the system to an unpleasant halt. In order to prevent
this, the actual `returnValue:to:` routine first checks to see if the
sender is `nil`. The interpreter also prevents returns to a context that
has already been returned from. It does this by storing `nil` in the
instruction pointer of the active context on return and checking for a
`nil` instruction pointer of the context being returned to. Both of
these situations can arise since contexts are objects and can be
manipulated by user programs as well as by the interpreter. If either
situation arises, the interpreter sends a message to the active context
informing it of the problem. The third situation will arise in systems
that automatically deallocate objects based on their reference counts.
The active context may be deallocated as it is returning. It, in turn,
may contain the only reference to the result being returned. In this
case, the result will be deallocated before it can be pushed on the new
context\'s stack. Because of these considerations, the `returnValue:`
routine must be somewhat more complicated.

**returnValue: resultPointer to: contextPointer**

\| sendersIP \|

contextPointer = NilPointer

ifTrue: \[self push: activeContext.

self push: resultPointer.

\^self sendSelector: CannotReturnSelector

        argumentCount: 1\].

sendersIP := memory fetchPointer: InstructionPointerIndex

  ofObject: contextPointer.

sendersIP = NilPointer

ifTrue: \[self push: activeContext.

self push: resultPointer.

\^self sendSelector: CannotReturnSelector

        argumentCount: 1\].

memory increaseReferencesTo: resultPointer.

self returnToActiveContext: contextPointer.

self push: resultPointer.

memory decreaseReferencesTo: resultPointer

This routine prevents the deallocation of the result being returned by
raising the reference count until it is pushed on the new stack. It
could also have pushed the result before switching active contexts. The
`returnToActiveContext:` routine is basically the same as the
`newActiveContext:` routine except that instead of restoring any cached
fields of the context being returned from, it stores `nil` into the
sender and instruction pointer fields.

**returnToActiveContext: aContext**

memory increaseReferencesTo: aContext.

self nilContextFields.

memory decreaseReferencesTo: activeContext.

activeContext := aContext.

self fetchContextRegisters

**nilContextFields**

memory storePointer: SenderIndex

   ofObject: activeContext

   withValue: NilPointer.

memory storePointer: InstructionPointerIndex

   ofObject: activeContext

   withValue: NilPointer

Due to the nature of `BlockContexts`, this implementation of the return
bytecodes will create circular structures. Implementations of the object
memory that rely exclusively on reference counting to reclaim unused
storage will not properly deallocate the objects that make up these
circular structures. To avoid this problem, the following additional
mechanism should be included. If the active context is a `BlockContext`
and the context being returned to (a `Context`) is on the sender chain
of the active context, the sender pointers of the intervening contexts
on the sender chain should be set to `nil`.\

------------------------------------------------------------------------

\[[TOC](bluebook_imp_toc.html)\] \[[26](bluebook_chapter26.html)\]
\[[27](bluebook_chapter27.html)\] \[[28](#top_of_28)\]
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]
