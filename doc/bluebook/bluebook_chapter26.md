---
Author: Dwight Hughes
GENERATOR: Mozilla/4.5 \[en\] (WinNT; I) \[Netscape\]
title: Blue Book Chapter 26
---

[]{#top_of_26}\[[TOC](bluebook_imp_toc.html)\] 26
\[[27](bluebook_chapter27.html)\] \[[28](bluebook_chapter28.html)\]
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]\

------------------------------------------------------------------------

26\
The Implementation

------------------------------------------------------------------------

**[The Compiler](#TheCompiler26)**

*[Compiled Methods](#compiled_methods26)*

*[The Bytecodes](#TheBytecodes26)*

**[The Interpreter](#TheInterpreter26)**

*[Contexts](#Contexts26)*

*[Block Contexts](#BlockContexts26)*

*[Messages](#Messages26)*

*[Primitive Methods](#PrimitiveMethods26)*

**[The Object Memory](#TheObjectMemory26)**

**[The Hardware](#TheHardware26)**

------------------------------------------------------------------------

Two major components of the Smalltalk-80 system can be distinguished:
the *virtual image* and the *virtual machine.*

1.  The *virtual image* consists of all of the objects in the system.
2.  The *virtual machine* consists of the hardware devices and machine
    language (or microcode) routines that give dynamics to the objects
    in the virtual image.

The system implementer\'s task is to create a virtual machine. A virtual
image can then be loaded into the virtual machine and the Smalltalk-80
system becomes the interactive entity described in earlier chapters.\
    The overview of the Smalltalk-80 implementation given in this
chapter is organized in a top-down fashion, starting with the source
methods written by programmers. These methods are translated by a
*compiler* into sequences of eight-bit instructions called *bytecodes*.
The compiler and bytecodes are the subject of this chapter\'s first
section. The bytecodes produced by the compiler are instructions for an
*interpreter*, which is described in the second section. Below the
interpreter in the implementation is an *object memory* that stores the
objects that make up the virtual image. The object memory is described
in the third section of this chapter. At the bottom of any
implementation is the *hardware*. The fourth and final section of this
chapter discusses the hardware required to implement the interpreter and
object memory. [Chapters 27 - 30](bluebook_imp_toc.html) give a detailed
specification of the virtual machine\'s interpreter and object memory.

------------------------------------------------------------------------

[]{#TheCompiler26}**The Compiler**

------------------------------------------------------------------------

Source methods written by programmers are represented in the
Smalltalk-80 system as instances of `String`. The `Strings` contain
sequences of characters that conform to the syntax introduced in the
first part of this book. For example, the following source method might
describe how instances of class `Rectangle` respond to the unary message
`center`. The `center` message is used to find the `Point` equidistant
from a `Rectangle's` four sides.

**center**

\^origin + corner / 2

Source methods are translated by the system\'s *compiler* into sequences
of instructions for a stack-oriented interpreter. The instructions are
eight-bit numbers called *bytecodes*. For example, the bytecodes
corresponding to the source method shown above are,

`0, 1, 176, 119, 185, 124`

Since a bytecode\'s value gives us little indication of its meaning to
the interpreter, this chapter will accompany lists of bytecodes with
comments about their functions. Any part of a bytecode\'s comment that
depends on the context of the method in which it appears will be
parenthesized. The unparenthesized part of the comment describes its
general function. For example, the bytecode 0 always instructs the
interpreter to push the value of the receiver\'s first instance variable
on to its stack. The fact that the variable is named `origin` depends on
the fact that this method is used by `Rectangles `, so `origin` is
parenthesized. The commented form of the bytecodes for `Rectangle`
`center` is shown below.\
 

  ----------------------- ----------------------- -----------------------
  **Rectangle center**                            

                          **`0`**                 push the value of the
                                                  receiver\'s first
                                                  instance variable
                                                  (`origin`) onto the
                                                  stack

                          **`1`**                 push the value of the
                                                  receiver\'s second
                                                  instance variable
                                                  (`corner`) onto the
                                                  stack

                          **`176`**               send a binary message
                                                  with the selector `+`

                          **`119`**               push the `SmallInteger`
                                                  `2` onto the stack

                          **`185`**               send a binary message
                                                  with the selector `/`

                          **`124`**               return the object on
                                                  top of the stack as the
                                                  value of the message
                                                  (`center`)\
                                                       
  ----------------------- ----------------------- -----------------------

The stack mentioned in some of the bytecodes is used for several
purposes. In this method, it is used to hold the receiver, arguments,
and results of the two messages that are sent. The stack is also used as
the source of the result to be returned from the center method. The
stack is maintained by the interpreter and will be described in greater
detail in the next section. A description of all the types of bytecodes
will appear at the end of this section.\
    A programmer does not interact directly with the compiler. When a
new source method is added to a class (`Rectangle` in this example), the
class asks the compiler for an instance of `CompiledMethod `containing
the bytecode translation of the source method. The class provides the
compiler with some necessary information not given in the source method,
including the names of the receiver\'s instance variables and the
dictionaries containing accessible shared variables (global, class, and
pool variables). The compiler translates the source text into a
`CompiledMethod `and the class stores the method in its message
dictionary. For example, the `CompiledMethod` shown above is stored in
`Rectangle's` message dictionary associated with the selector `center`.\
    Another example of the bytecodes compiled from a source method
illustrates the use of a store bytecode. The message `extent:` to a
`Rectangle` changes the receiver\'s width and height to be equal to the
x and y coordinates of the argument (a `Point`). The receiver\'s upper
left corner (`origin`) is kept the same and the lower right corner
(`corner`) is moved.

**extent: newExtent**

corner := origin + newExtent

  ----------------------- ----------------------- -----------------------
  **Rectangle extent: **                          

                          **`0`**                 push the value of the
                                                  receiver\'s first
                                                  instance variable
                                                  (`origin`) onto the
                                                  stack 

                          **`16`**                push the argument
                                                  (`newExtent`) onto the
                                                  stack 

                          **`176`**               send a binary message
                                                  with the selector `+ `

                          **`97`**                pop the top object off
                                                  of the stack and store
                                                  it in the receiver\'s
                                                  second instance
                                                  variable (`corner`) 

                          **`120`**               return the receiver as
                                                  the value of the
                                                  message (`extent:`)\
                                                       
  ----------------------- ----------------------- -----------------------

The forms of source methods and compiled bytecodes are different in
several respects. The variable names in a source method are converted
into instructions to push objects on the stack, the selectors are
converted into instructions to send messages, and the uparrow is
converted into an instruction to return a result. The order of the
corresponding components is also different in a source method and
compiled bytecodes. Despite these differences in form, the source method
and compiled bytecodes describe the same actions.

[]{#compiled_methods26}***Compiled Methods***\

------------------------------------------------------------------------

The compiler creates an instance of `CompiledMethod` to hold the
bytecode translation of a source method. In addition to the bytecodes
themselves, a `CompiledMethod` contains a set of objects called its
literal frame. The literal frame contains any objects that could not be
referred to directly by bytecodes. All of the objects in
`Rectangle center` and `Rectangle` `extent:` were referred to directly
by bytecodes, so the `CompiledMethods` for these methods do not need
literal frames. As an example of a `CompiledMethod` with a literal
frame, consider the method for `Rectangle` `intersects:`. The
`intersects:` message inquires whether one `Rectangle` (the receiver)
overlaps another `Rectangle` (the argument).

**intersects: aRectangle**

\^(origin max: aRectangle origin) \< (corner min: aRectangle corner)

The four message selectors, `max:`, `origin`, `min:`, and `corner` are
not in the set that can be directly referenced by bytecodes. These
selectors are included in the `CompiledMethod's` literal frame and the
send bytecodes refer to the selectors by their position in the literal
frame. A `CompiledMethod's` literal frame will be shown after its
bytecodes.\
 

  ----------------- ----------------- ----------------- -----------------
  **Rectangle                                           
  intersects:**                                         

                    **`0`**           push the value of 
                                      the receiver\'s   
                                      first instance    
                                      variable          
                                      (`origin`) onto   
                                      the stack         

                    **`16`**          push the argument 
                                      (`aRectangle`)    

                    **`209`**         send a unary      
                                      message with the  
                                      selector in the   
                                      second literal    
                                      frame location    
                                      (`origin`)        

                    **`224`**         send a single     
                                      argument message  
                                      with the selector 
                                      in the first      
                                      literal frame     
                                      location          
                                      (`max:`)          

                    **`1`**           push the value of 
                                      the receiver\'s   
                                      second instance   
                                      variable          
                                      (`corner`) onto   
                                      the stack         

                    **`16`**          push the argument 
                                      (`aRectangle`)    
                                      onto the stack    

                    **`211`**         send a unary      
                                      message with the  
                                      selector in the   
                                      fourth literal    
                                      frame location    
                                      (`corner`)        

                    **`226`**         send a single     
                                      argument message  
                                      with the selector 
                                      in the third      
                                      literal frame     
                                      location          
                                      (`min:`)          

                    **`178`**         send a binary     
                                      message with the  
                                      selector `<`      

                    **`124`**         return the object 
                                      on top of the     
                                      stack as the      
                                      value of the      
                                      message           
                                      (`intersects:`)   

                                      **literal         
                                      frame**\          
                                      `#max:`\          
                                      `#origin`\        
                                      `#min:`\          
                                      `#corner`\        
                                      `    `            
  ----------------- ----------------- ----------------- -----------------

The categories of objects that can be referred to directly by bytecodes
are:

-   the receiver and arguments of the invoking message
-   the values of the receiver\'s instance variables
-   the values of any temporary variables required by the method
-   seven special constants (`true`, `false`, `nil`, `-1`, `0`, `1`, and
    `2`)
-   32 special message selectors

The 32 special message selectors are listed below.\
 

  -------------- -------------- -------------- -------------- --------------
                 `+`            `-`            `<`            `>`

                 `<=`           `>=`           `=`            `~=`

                 `*`            `/`            `\`            `@`

                 `bitShift:`    `\\`           `bitAnd:`      `bitOr:`

                 `(at:)`        `(at:put:)`    `(size)`       `(next)`

                 `(nextPut:)`   `(atEnd)`      `==`           `class`

                 `blockCopy:`   `value`        `value:`       `(do:)`

                 `(new)`        `(new:)`\      `(x)`          `(y)`
                                `    `                        
  -------------- -------------- -------------- -------------- --------------

The selectors in parentheses may be replaced with other selectors by
modifying the compiler and recompiling all methods in the system. The
other selectors are built into the virtual machine.\
    Any objects referred to in a `CompiledMethod's` bytecodes that do
not fall into one of the categories above must appear in its literal
frame. The objects ordinarily contained in a literal frame are

-   shared variables (global, class, and pool)
-   most literal constants (numbers, characters, strings, arrays, and
    symbols)
-   most message selectors (those that are not special)

Objects of these three types may be intermixed in the literal frame. If
an object in the literal frame is referenced twice in the same method,
it need only appear in the literal frame once. The two bytecodes that
refer to the object will refer to the same location in the literal
frame.\
    Two types of object that were referred to above, temporary variables
and shared variables, have not been used in the example methods. The
following example method for `Rectangle` `merge:` uses both types. The
`merge:` message is used to find a `Rectangle` that includes the areas
in both the receiver and the argument.

**merge: aRectangle**

\| minPoint maxPoint \|

minPoint := origin min: aRectangle origin.

maxPoint := corner max: aRectangle corner.

\^Rectangle origin: minPoint corner: maxPoint

When a `CompiledMethod` uses temporary variables (`maxPoint` and
`minPoint` in this example), the number required is specified in the
first line of its printed form. When a `CompiledMethod` uses a shared
variable (`Rectangle` in this example) an instance of `Association `is
included in its literal frame. All `CompiledMethods` that refer to a
particular shared variable\'s name include the same `Association` in
their literal frames.\
 

  ----------------- ----------------- --------------------- -----------------
  **Rectangle                                               
  merge:** requires                                         
  2 temporary                                               
  variables                                                 

                    **`0`**           push the value of the 
                                      receiver\'s first     
                                      instance variable     
                                      (`origin`) onto the   
                                      stack                 

                    **`16`**          push the contents of  
                                      the first temporary   
                                      frame location (the   
                                      argument              
                                      `aRectangle`) onto    
                                      the stack             

                    **`209`**         send a unary message  
                                      with the selector in  
                                      the second literal    
                                      frame location        
                                      (`origin`)            

                    **`224`**         send the single       
                                      argument message with 
                                      the selector in the   
                                      first literal frame   
                                      location (`min:`)     

                    **`105`**         pop the top object    
                                      off of the stack and  
                                      stare in the second   
                                      temporary frame       
                                      location              
                                      (`minPoint`)          

                    **`1`**           push the value of the 
                                      receiver\'s second    
                                      instance variable     
                                      (`corner`) onto the   
                                      stack                 

                    **`16`**          push the contents of  
                                      the first temporary   
                                      frame location (the   
                                      argument              
                                      `aRectangle`) onto    
                                      the stack             

                    **`211`**         send a unary message  
                                      with the selector in  
                                      the fourth literal    
                                      frame location        
                                      (`corner`)            

                    **`226`**         send a single         
                                      argument message with 
                                      the selector in the   
                                      third literal frame   
                                      location (`max:`)     

                    **`106`**         pop the top object    
                                      off of the stack and  
                                      store it in the third 
                                      temporary frame       
                                      location (`maxPoint`) 

                    **`69`**          push the value of the 
                                      shared variable in    
                                      the sixth literal     
                                      frame location        
                                      (`Rectangle`) onto    
                                      the stack             

                    **`17`**          push the contents of  
                                      the second temporary  
                                      frame location        
                                      (`minPoint`) onto the 
                                      stack                 

                    **`18`**          push the contents of  
                                      the third temporary   
                                      frame location        
                                      (`maxPoint`) onto the 
                                      stack                 

                    **`244`**         send the two argument 
                                      message with the      
                                      selector in the fifth 
                                      literal frame         
                                      location              
                                      (`origin:corner:`)    

                    **`124`**         return the object on  
                                      top of the stack as   
                                      the value of the      
                                      message (`merge:`)    

                                      **literal frame**\    
                                      #min: \               
                                      #origin \             
                                      #max: \               
                                      #corner \             
                                      #origin:corner: \     
                                      Association:          
                                      #Rectangle -\>        
                                      Rectangle \           
                                                            
  ----------------- ----------------- --------------------- -----------------

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Temporary
Variables* ** Temporary variables are created for a particular execution
of a `CompiledMethod` and cease to exist when the execution is complete.
The `CompiledMethod` indicates to the interpreter how many temporary
variables will be required. The arguments of the invoking message and
the values of the temporary variables are stored together in the
temporary frame. The arguments are stored first and the temporary
variable values immediately after. They are accessed by the same type of
bytecode (whose comments refer to a temporary frame location). Since
`merge:` takes a single argument, its two temporary variables use the
second and third locations in the temporary frame. The compiler enforces
the fact that the values of the argument names cannot be changed by
never issuing a store bytecode referring to the part of the temporary
frame inhabited by the arguments.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Shared
Variables *** Shared variables are found in dictionaries.

-   *global variables* in a dictionary whose names can appear in any
    method
-   *class variables* in a dictionary whose names can only appear in the
    methods of a single class and its subclasses
-   *pool variables* in a dictionary whose names can appear in the
    methods of several classes

Shared variables are the individual associations that make up these
dictionaries. The system represents associations in general, and shared
variables in particular, with instances of `Association`. When the
compiler encounters the name of a shared variable in a source method,
the `Association` with the same name is included in the
`CompiledMethod's `literal frame. The bytecodes that access shared
variables indicate the location of an `Association` in the literal
frame. The actual value of the variable is stored in an instance
variable of the `Association`. In the `CompiledMethod` for
`Rectangle merge:` shown above, class `Rectangle` is referenced by
including the `Association` from the global dictionary whose name is the
symbol `#Rectangle` and whose value is the class `Rectangle`.

[]{#TheBytecodes26}***The Bytecodes***

------------------------------------------------------------------------

The interpreter understands 256 bytecode instructions that fall into
five categories: pushes, stores, sends, returns, and jumps. This section
gives a general description of each type of bytecode without going into
detail about which bytecode represents which instruction. [Chapter
28](bluebook_chapter28.html) describes the exact meaning of each
bytecode. Since more than 256 instructions for the interpreter are
needed, some of the bytecodes take extensions. An extension is one or
two bytes following the bytecode that further specify the instruction.
An extension is not an instruction on its own, it is only a part of an
instruction.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Push
Bytecodes***   A push bytecode indicates the source of an object to be
added to the top of the interpreter\'s stack. The sources include

-   the receiver of the message that invoked the `CompiledMethod`
-   the instance variables of the receiver
-   the temporary frame (the arguments of the message and the temporary
    variables)
-   the literal frame of the `CompiledMethod`
-   the top of the stack (i.e., this bytecode duplicates the top of
    stack)

Examples of most of the types of push bytecode have been included in the
examples. The bytecode that duplicates the top of the stack is used to
implement cascaded messages.\
    Two different types of push bytecode use the literal frame as their
source. One is used to push literal constants and the other to push the
value of shared variables. Literal constants are stored directly in the
literal frame, but the values of shared variables are stored in an
`Association` that is pointed to by the literal frame. The following
example method uses one shared variable and one literal constant.

**incrementIndex**

\^Index := Index + 4

  ------------------ ----------------- -------------------- -----------------
  **ExampleClass                                            
  incrementIndex**                                          

                     **`64`**          push the value of    
                                       the shared variable  
                                       in the first literal 
                                       frame location       
                                       (`Index`) onto the   
                                       stack                

                     **`33`**          push the constant in 
                                       the second literal   
                                       frame location (`4`) 
                                       onto the stack       

                     **`176`**         send a binary        
                                       message with the     
                                       selector `+`         

                     **`129,192`**     store the object on  
                                       top of the stack in  
                                       the shared variable  
                                       in the first literal 
                                       frame location       
                                       (`Index`)            

                     **`124`**         return the object on 
                                       top of the stack as  
                                       the value of the     
                                       message              
                                       (`incrementIndex`)   

                                       **literal frame**\   
                                       Association: #Index  
                                       -\> 260 \            
                                       4\                   
                                                            
  ------------------ ----------------- -------------------- -----------------

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Store
Bytecodes***   The bytecodes compiled from an assignment expression end
with a store bytecode. The bytecodes before the store bytecode compute
the new value of a variable and leave it on top of the stack. A store
bytecode indicates the variable whose value should be changed. The
variables that can be changed are

-   the instance variables of the receiver
-   temporary variables
-   shared variables

Some of the store bytecodes remove the object to be stored from the
stack, and others leave the object on top of the stack, after storing
it.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Send
Bytecodes***   A send bytecode specifies the selector of a message to be
sent and how many arguments it should have. The receiver and arguments
of the message are taken off the interpreter\'s stack, the receiver from
below the arguments. By the time the bytecode following the send is
executed, the message\'s result will have replaced its receiver and
arguments on the top of the stack. The details of sending messages and
returning results is the subject of the next sections of this chapter. A
set of 32 send bytecodes refer directly to the special selectors listed
earlier. The other send bytecodes refer to their selectors in the
literal frame.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Return
Bytecodes ***   When a return bytecode is encountered, the
`CompiledMethod` in which it was found has been completely executed.
Therefore a value is returned for the message that invoked that
`CompiledMethod`. The value is usually found on top of the stack. Four
special return bytecodes return the message receiver (`self`), `true`,
`false`, and `nil`.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Jump
Bytecodes ***   Ordinarily, the interpreter executes the bytecodes
sequentially in the order they appear in a `CompiledMethod`. The jump
bytecodes indicate that the next bytecode to execute is not the one
following the jump. There are two varieties of jump, *unconditional* and
*conditional*. The unconditional jumps transfer control whenever they
are encountered. The conditional jumps will only transfer control if the
top of the stack is a specified value. Some of the conditional jumps
transfer if the top object on the stack is `true` and others if it is
`false`. The jump bytecodes are used to implement efficient control
structures.\
    The control structures that are so optimized by the compiler are
conditional selection messages to Booleans (`ifTrue:`, `ifFalse: `, and
`ifTrue:ifFalse:`), some of the logical operation messages to Booleans
(`and:` and `or:`), and the conditional repetition messages to blocks
(`whileTrue:` and `whileFalse:`). The jump bytecodes indicate the next
bytecode to be executed relative to the position of the jump. In other
words, they tell the interpreter how many bytecodes to skip. The
following method for `Rectangle` `includesPoint:` uses a conditional
jump.

**includesPoint: aPoint**

origin \<= aPoint

ifTrue: \[\^aPoint \< corner\]

ifFalse: \[\^false\]

  ------------------ ----------------- --------------------- -----------------
  **Rectangle                                                
  includesPoint:**                                           

                     **`0`**           push the value of the 
                                       receiver\'s first     
                                       instance variable     
                                       (`origin`) onto the   
                                       stack                 

                     **`16`**          push the contents of  
                                       the first temporary   
                                       frame location (the   
                                       argument `aPoint`)    
                                       onto the stack        

                     **`180`**         send a binary message 
                                       with the selector     
                                       `<=`                  

                     **`155`**         jump ahead 4          
                                       bytecodes if the      
                                       object on top of the  
                                       stack is `false`      

                     **`16`**          push the contents of  
                                       the first temporary   
                                       frame location (the   
                                       argument `aPoint`)    
                                       onto the stack        

                     **`1`**           push the value of the 
                                       receiver\'s second    
                                       instance variable     
                                       (`corner`) onto the   
                                       stack                 

                     **`178`**         send a binary message 
                                       with the selector `<` 

                     **`124`**         return the object on  
                                       top of the stack as   
                                       the value of the      
                                       message (             
                                       `includesPoint:`)     

                     **`122`**         return `false` as the 
                                       value of the message  
                                       (`includesPoint:`)\   
                                                             
  ------------------ ----------------- --------------------- -----------------

------------------------------------------------------------------------

[]{#TheInterpreter26}**The Interpreter**

------------------------------------------------------------------------

The Smalltalk-80 interpreter executes the bytecode instructions found in
`CompiledMethods`. The interpreter uses five pieces of information and
repeatedly performs a three-step cycle.

**The State of the Interpreter**

1.  The `CompiledMethod` whose bytecodes are being executed.
2.  The location of the next bytecode to be executed in that
    `CompiledMethod`. This is the interpreter\'s *instruction pointer*.
3.  The receiver and arguments of the message that invoked the
    `CompiledMethod`.
4.  Any temporary variables needed by the `CompiledMethod`.
5.  A stack.

The execution of most bytecodes involves the interpreter\'s stack. Push
bytecodes tell where to find objects to add to the stack. Store
bytecodes tell where to put objects found on the stack. Send bytecodes
remove the receiver and arguments of messages from the stack. When the
result of a message is computed, it is pushed onto the stack.

**The Cycle of the Interpreter**

1.  Fetch the bytecode from the `CompiledMethod` indicated by the
    instruction pointer.
2.  Increment the instruction pointer.
3.  Perform the function specified by the bytecode.

As an example of the interpreter\'s function, we will trace its
execution of the `CompiledMethod` for `Rectangle` `center`. The state of
the interpreter will be displayed after each of its cycles. The
instruction pointer will be indicated by an arrow pointing at the next
bytecode in the `CompiledMethod` to be executed.\
 

  ---------------------------------------- ----------------------- -----------------------
  ![](right_arrow_black.gif){height="17"   **`0`**                 push the value of the
  width="24"}                                                      receiver\'s first
                                                                   instance variable
                                                                   (`origin`) onto the
                                                                   stack\
                                                                        

  ---------------------------------------- ----------------------- -----------------------

The receiver, arguments, temporary variables, and objects on the stack
will be shown as normally printed (their responses to `printString`).
For example, if a message is sent to a `Rectangle`, the receiver will be
shown as\
 

  ----------------- ----------------- ----------------- -----------------
                                      **Receiver**      100@100 corner:
                                                        200@200\
                                                             

  ----------------- ----------------- ----------------- -----------------

At the start of execution, the stack is empty and the instruction
pointer indicates the first bytecode in the `CompiledMethod`. This
`CompiledMethod` does not require temporaries and the invoking message
did not have arguments, so these two categories are also empty.\

------------------------------------------------------------------------

  ---------------------------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------
  **Method for Rectangle center**                                                                                                                                           
  ![](right_arrow_black.gif){height="17" width="24" align="ABSBOTTOM"}   **`0`**     push the value of the receiver\'s first instance variable (`origin`) onto the stack    
                                                                         **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                                         **`176`**   send a binary message with the selector `+`                                            
                                                                         **`119`**   push the `SmallInteger` `2 `onto the stack                                             
                                                                         **`185`**   send a binary message with the selector `/`                                            
                                                                         **`124`**   return the object on top of the stack as the value of the message (`center`)           
                                                                                     **Receiver**                                                                           100@100 corner: 200@200 
                                                                                     **Arguments**                                                                          
                                                                                     **Temporary Variables **                                                               
                                                                                     **Stack**                                                                              
  ---------------------------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------

------------------------------------------------------------------------

Following one cycle of the interpreter, the instruction pointer has been
advanced and the value of the receiver\'s first instance variable has
been copied onto the stack.\

------------------------------------------------------------------------

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------
  **Method for Rectangle center**                                                                                                                         
                                                       **`0`**     push the value of the receiver\'s first instance variable (`origin`) onto the stack    
  ![](right_arrow_black.gif){height="17" width="24"}   **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                       **`176`**   send a binary message with the selector `+`                                            
                                                       **`119`**   push the `SmallInteger` `2 `onto the stack                                             
                                                       **`185`**   send a binary message with the selector `/`                                            
                                                       **`124`**   return the object on top of the stack as the value of the message (`center`)           
                                                                   **Receiver**                                                                           100@100 corner: 200@200 
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              100@100
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------

------------------------------------------------------------------------

The interpreter\'s second cycle has an effect similar to the first. The
top of the stack is shown toward the bottom of the page. This
corresponds to the commonly used convention that memory locations are
shown with addresses increasing toward the bottom of the page.\

------------------------------------------------------------------------

  ---------------------------------------- ----------------- ----------------- -----------------
  **Method for Rectangle center**                                              

                                           **`0`**           push the value of 
                                                             the receiver\'s   
                                                             first instance    
                                                             variable          
                                                             (`origin`) onto   
                                                             the stack         

                                           **`1`**           push the value of 
                                                             the receiver\'s   
                                                             second instance   
                                                             variable          
                                                             (`corner`) onto   
                                                             the stack         

  ![](right_arrow_black.gif){height="17"   **`176`**         send a binary     
  width="24"}                                                message with the  
                                                             selector `+`      

                                           **`119`**         push the          
                                                             `SmallInteger`    
                                                             `2 `onto the      
                                                             stack             

                                           **`185`**         send a binary     
                                                             message with the  
                                                             selector `/`      

                                           **`124`**         return the object 
                                                             on top of the     
                                                             stack as the      
                                                             value of the      
                                                             message           
                                                             (`center`)        

                                                             **Receiver**      100@100 corner:
                                                                               200@200 

                                                             **Arguments**     

                                                             **Temporary       
                                                             Variables **      

                                                             **Stack**         100@100 \
                                                                               200@200
  ---------------------------------------- ----------------- ----------------- -----------------

------------------------------------------------------------------------

The interpreter\'s third cycle encounters a send bytecode. It removes
two objects from the stack and uses them as the receiver and argument of
a message with selector `+` The procedure for sending the message will
not be described in detail here. For the moment, it is only necessary to
know that eventually the result of the `+` message will be pushed onto
the stack. Sending messages will be described in later sections.\

------------------------------------------------------------------------

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------
  **Method for Rectangle center**                                                                                                                         
                                                       **`0`**     push the value of the receiver\'s first instance variable (`origin`) onto the stack    
                                                       **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                       **`176`**   send a binary message with the selector `+`                                            
  ![](right_arrow_black.gif){height="17" width="24"}   **`119`**   push the `SmallInteger` `2 `onto the stack                                             
                                                       **`185`**   send a binary message with the selector `/`                                            
                                                       **`124`**   return the object on top of the stack as the value of the message (`center`)           
                                                                   **Receiver**                                                                           100@100 corner: 200@200 
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              300@300
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------

------------------------------------------------------------------------

The interpreter\'s next cycle pushes the constant `2` onto the stack.\

------------------------------------------------------------------------

  ---------------------------------------- ----------------- ----------------- -----------------
  **Method for Rectangle center**                                              

                                           **`0`**           push the value of 
                                                             the receiver\'s   
                                                             first instance    
                                                             variable          
                                                             (`origin`) onto   
                                                             the stack         

                                           **`1`**           push the value of 
                                                             the receiver\'s   
                                                             second instance   
                                                             variable          
                                                             (`corner`) onto   
                                                             the stack         

                                           **`176`**         send a binary     
                                                             message with the  
                                                             selector `+`      

                                           **`119`**         push the          
                                                             `SmallInteger`    
                                                             `2 `onto the      
                                                             stack             

  ![](right_arrow_black.gif){height="17"   **`185`**         send a binary     
  width="24"}                                                message with the  
                                                             selector `/`      

                                           **`124`**         return the object 
                                                             on top of the     
                                                             stack as the      
                                                             value of the      
                                                             message           
                                                             (`center`)        

                                                             **Receiver**      100@100 corner:
                                                                               200@200 

                                                             **Arguments**     

                                                             **Temporary       
                                                             Variables **      

                                                             **Stack**         300@300 \
                                                                               2
  ---------------------------------------- ----------------- ----------------- -----------------

------------------------------------------------------------------------

The interpreter\'s next cycle sends another message whose result
replaces its receiver and arguments on the stack.\

------------------------------------------------------------------------

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------
  **Method for Rectangle center**                                                                                                                         
                                                       **`0`**     push the value of the receiver\'s first instance variable (`origin`) onto the stack    
                                                       **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                       **`176`**   send a binary message with the selector `+`                                            
                                                       **`119`**   push the `SmallInteger` `2 `onto the stack                                             
                                                       **`185`**   send a binary message with the selector `/`                                            
  ![](right_arrow_black.gif){height="17" width="24"}   **`124`**   return the object on top of the stack as the value of the message (`center`)           
                                                                   **Receiver**                                                                           100@100 corner: 200@200 
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              150@150
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- --------------------------

------------------------------------------------------------------------

The final bytecode returns a result to the `center` message. The result
is found on the stack (`150@150`). It is clear by this point that a
return bytecode must involve pushing the result onto another stack. The
details of returning a value to a message will be described after the
description of sending a message.

[]{#Contexts26}***Contexts***\

------------------------------------------------------------------------

Push, store, and jump bytecodes require only small changes to the state
of the interpreter. Objects may be moved to or from the stack, and the
instruction pointer is always changed; but most of the state remains the
same. Send and return bytecodes may require much larger changes to the
interpreter\'s state. When a message is sent, all five parts of the
interpreter\'s state may have to be changed in order to execute a
different `CompiledMethod` in response to this new message. The
interpreter\'s old state must be remembered because the bytecodes after
the send must be executed after the value of the message is returned.\
    The interpreter saves its state in objects called *contexts*. There
will be many contexts in the system at any one time. The context that
represents the current state of the interpreter is called the *active
context*. When a send bytecode in the active context\'s `CompiledMethod`
requires a new `CompiledMethod` to be executed, the active context
becomes *suspended* and a new context is created and made active. The
suspended context retains the state associated with the original
`CompiledMethod` until that context becomes active again. A context must
remember the context that it suspended so that the suspended context can
be resumed when a result is returned. The suspended context is called
the new context\'s *sender*.\
    The form used to show the interpreter\'s state in the last section
will be used to show contexts as well. The active context will be
indicated by the word **Active** in its top delimiter. Suspended
contexts will say **Suspended**. For example, consider a context
representing the execution of the `CompiledMethod` for `Rectangle`
`rightCenter `with a receiver of `100@100` `corner:` `200@200`. The
source method for `Rectangle` `rightCenter` is

**rightCenter**

\^self right @ self center y

The interpreter\'s state following execution of the first bytecode is
shown below. The sender is some other context in the system.\
 

  ---------------------------------------- ----------------- ----------------- -----------------
  **Active**                                                                   

  **Method for Rectangle rightCenter**                                         

                                           **`112`**         push the receiver 
                                                             (`self`) onto the 
                                                             stack             

  ![](right_arrow_black.gif){height="17"   **`208`**         send a unary      
  width="24"}                                                message with the  
                                                             selector in the   
                                                             first literal     
                                                             (`right`)         

                                           **`112`**         push the receiver 
                                                             (`self`) onto the 
                                                             stack             

                                           **`209`**         send the unary    
                                                             message with the  
                                                             selector in the   
                                                             second literal    
                                                             (`center`)        

                                           **`207`**         send the unary    
                                                             message with the  
                                                             selector `y`      

                                           **`187`**         send the binary   
                                                             message with the  
                                                             selector `@`      

                                           **`124`**         return the object 
                                                             on top of the     
                                                             stack as the      
                                                             value of the      
                                                             message           
                                                             (`rightCenter`)   

                                                             **literal         
                                                             frame**\          
                                                             `#right`\         
                                                             `#center`         

                                                             **Receiver**      100@100 corner:
                                                                               200@200

                                                             **Arguments**     

                                                             **Temporary       
                                                             Variables **      

                                                             **Stack**         100@100 corner:
                                                                               200@200

                                                             **Sender**        
  ---------------------------------------- ----------------- ----------------- -----------------

------------------------------------------------------------------------

After the next bytecode is executed, that context will be suspended. The
object pushed by the first bytecode has been removed to be used as the
receiver of a new context, which becomes active. The new active context
is shown above the suspended context.\
 

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------
  **Active**                                                                                                                                              
  **Method for Rectangle right**                                                                                                                          
  ![](right_arrow_black.gif){height="17" width="24"}   **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                       **`206`**   send a unary message with the selector `x`                                             
                                                       **`124`**   return the object on top of the stack as the value of the message (`right`)            
                                                                   **Receiver**                                                                           100@100 corner: 200@200
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              
                                                                   **Sender**                                                                             
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------

------------------------------------------------------------------------

  ---------------------------------------- ----------------- ------------------ -----------------
  **Suspended**                                                                 

  **Method for Rectangle rightCenter**                                          

                                           **`112`**         push the receiver  
                                                             (`self`) onto the  
                                                             stack              

                                           **`208`**         send a unary       
                                                             message with the   
                                                             selector in the    
                                                             first literal (    
                                                             `right`)           

  ![](right_arrow_black.gif){height="17"   **`112`**         push the receiver  
  width="24"}                                                (`self`) onto the  
                                                             stack              

                                           **`209`**         send the unary     
                                                             message with the   
                                                             selector in the    
                                                             second literal     
                                                             (`center`)         

                                           **`207`**         send the unary     
                                                             message with the   
                                                             selector `y `      

                                           **`187`**         send the binary    
                                                             message with the   
                                                             selector `@`       

                                           **`124`**         return the object  
                                                             on top of the      
                                                             stack as the value 
                                                             of the message     
                                                             (`rightCenter`)    

                                                             **literal frame**\ 
                                                             `#right `\         
                                                             `#center`          

                                                             **Receiver**       100@100 corner:
                                                                                200@200 

                                                             **Arguments**      

                                                             **Temporary        
                                                             Variables **       

                                                             **Stack**          

                                                             **Sender**         
  ---------------------------------------- ----------------- ------------------ -----------------

------------------------------------------------------------------------

The next cycle of the interpreter advances the new context instead of
the previous one.\
 

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------
  **Active**                                                                                                                                              
  **Method for Rectangle right**                                                                                                                          
                                                       **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
  ![](right_arrow_black.gif){height="17" width="24"}   **`206`**   send a unary message with the selector `x`                                             
                                                       **`124`**   return the object on top of the stack as the value of the message (`right`)            
                                                                   **Receiver**                                                                           100@100 corner: 200@200
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              200@200
                                                                   **Sender**                                                                             
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------

------------------------------------------------------------------------

  ---------------------------------------- ----------------- ------------------ -----------------
  **Suspended**                                                                 

  **Method for Rectangle rightCenter**                                          

                                           **`112`**         push the receiver  
                                                             (`self`) onto the  
                                                             stack              

                                           **`208`**         send a unary       
                                                             message with the   
                                                             selector in the    
                                                             first literal      
                                                             (`right`)          

  ![](right_arrow_black.gif){height="17"   **`112`**         push the receiver  
  width="24"}                                                (`self`) onto the  
                                                             stack              

                                           **`209`**         send the unary     
                                                             message with the   
                                                             selector in the    
                                                             second literal     
                                                             (`center`)         

                                           **`207`**         send the unary     
                                                             message with the   
                                                             selector `y `      

                                           **`187`**         send the binary    
                                                             message with the   
                                                             selector `@`       

                                           **`124`**         return the object  
                                                             on top of the      
                                                             stack as the value 
                                                             of the message     
                                                             (`rightCenter`)    

                                                             **literal frame**\ 
                                                             `#right `\         
                                                             `#center`          

                                                             **Receiver**       100@100 corner:
                                                                                200@200 

                                                             **Arguments**      

                                                             **Temporary        
                                                             Variables **       

                                                             **Stack**          

                                                             **Sender**         
  ---------------------------------------- ----------------- ------------------ -----------------

------------------------------------------------------------------------

In the next cycle, another message is sent, perhaps creating another
context. Instead of following the response of this new message (`x`), we
will skip to the point that this context returns a value (to `right`).
When the result of `x` has been returned, the new context looks like
this:\
 

  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------
  **Active**                                                                                                                                              
  **Method for Rectangle right**                                                                                                                          
                                                       **`1`**     push the value of the receiver\'s second instance variable (`corner`) onto the stack   
                                                       **`206`**   send a unary message with the selector `x`                                             
  ![](right_arrow_black.gif){height="17" width="24"}   **`124`**   return the object on top of the stack as the value of the message (`right`)            
                                                                   **Receiver**                                                                           100@100 corner: 200@200
                                                                   **Arguments**                                                                          
                                                                   **Temporary Variables **                                                               
                                                                   **Stack**                                                                              200
                                                                   **Sender**                                                                             
  ---------------------------------------------------- ----------- -------------------------------------------------------------------------------------- -------------------------

------------------------------------------------------------------------

  ---------------------------------------- ----------------- ------------------ -----------------
  **Suspended**                                                                 

  **Method for Rectangle rightCenter**                                          

                                           **`112`**         push the receiver  
                                                             (`self`) onto the  
                                                             stack              

                                           **`208`**         send a unary       
                                                             message with the   
                                                             selector in the    
                                                             first literal      
                                                             (`right`)          

  ![](right_arrow_black.gif){height="17"   **`112`**         push the receiver  
  width="24"}                                                (`self`) onto the  
                                                             stack              

                                           **`209`**         send the unary     
                                                             message with the   
                                                             selector in the    
                                                             second literal     
                                                             (`center`)         

                                           **`207`**         send the unary     
                                                             message with the   
                                                             selector `y `      

                                           **`187`**         send the binary    
                                                             message with the   
                                                             selector `@`       

                                           **`124`**         return the object  
                                                             on top of the      
                                                             stack as the value 
                                                             of the message     
                                                             (`rightCenter`)    

                                                             **literal frame**\ 
                                                             `#right`\          
                                                             `#center`          

                                                             **Receiver**       100@100 corner:
                                                                                200@200 

                                                             **Arguments**      

                                                             **Temporary        
                                                             Variables **       

                                                             **Stack**          

                                                             **Sender**         
  ---------------------------------------- ----------------- ------------------ -----------------

------------------------------------------------------------------------

The next bytecode returns the value on the top of the active context\'s
stack (`200`) as the value of the message that created the context
(`right`). The active context\'s sender becomes the active context again
and the returned value is pushed on its stack.\
 

  ---------------------------------------- ----------------- ------------------ -----------------
  **Active**                                                                    

  **Method for Rectangle rightCenter**                                          

                                           **`112`**         push the receiver  
                                                             (`self`) onto the  
                                                             stack              

                                           **`208`**         send a unary       
                                                             message with the   
                                                             selector in the    
                                                             first literal      
                                                             (`right`)          

  ![](right_arrow_black.gif){height="17"   **`112`**         push the receiver  
  width="24"}                                                (`self`) onto the  
                                                             stack              

                                           **`209`**         send the unary     
                                                             message with the   
                                                             selector in the    
                                                             second literal     
                                                             (`center`)         

                                           **`207`**         send the unary     
                                                             message with the   
                                                             selector `y `      

                                           **`187`**         send the binary    
                                                             message with the   
                                                             selector `@`       

                                           **`124`**         return the object  
                                                             on top of the      
                                                             stack as the value 
                                                             of the message     
                                                             (`rightCenter`)    

                                                             **literal frame**\ 
                                                             `#right`\          
                                                             `#center`          

                                                             **Receiver**       100@100 corner:
                                                                                200@200 

                                                             **Arguments**      

                                                             **Temporary        
                                                             Variables **       

                                                             **Stack**          200

                                                             **Sender**         
  ---------------------------------------- ----------------- ------------------ -----------------

------------------------------------------------------------------------

[]{#BlockContexts26}***Block Contexts***\

------------------------------------------------------------------------

The contexts illustrated in the last section are represented in the
system by instances of `MethodContext`. A `MethodContext` represents the
execution of a `CompiledMethod` in response to a message. There is
another type of context in the system, which is represented by instances
of `BlockContext`. A `BlockContext` represents a block in a source
method that is not part of an optimized control structure. The
compilation of the optimized control structures was described in the
earlier section on jump bytecodes. The bytecodes compiled from a
nonoptimized control structure are illustrated by the following
hypothetical method in `Collection`. This method returns a collection of
the classes of the receiver\'s elements.

**classes**

\^self collect: \[:element \| element class\]

  ----------------- ----------------- ----------------- -----------------
  **Collection                                          
  classes**                                             
  requires 1                                            
  temporary                                             
  variable                                              

                    **`112`**         push the receiver 
                                      (`self`) onto the 
                                      stack             

                    **`137`**         push the active   
                                      context           
                                      (`thisContext`)   
                                      onto the stack    

                    **`118`**         push the          
                                      `SmallInteger`    
                                      `1` onto the      
                                      stack             

                    **`200`**         send a single     
                                      argument message  
                                      with the selector 
                                      `blockCopy:`      

                    **`164,4`**       jump around the   
                                      next 4 bytes      

                    **`104`**         pop the top       
                                      object off of the 
                                      stack and store   
                                      in the first      
                                      temporary frame   
                                      location          
                                      (`element`)       

                    **`16`**          push the contents 
                                      of the first      
                                      temporary frame   
                                      location          
                                      (`element`) onto  
                                      the stack         

                    **`199`**         send a unary      
                                      message with the  
                                      selector `class`  

                    **`125`**         return the object 
                                      on top of the     
                                      stack as the      
                                      value of the      
                                      block             

                    **`224`**         send a single     
                                      argument message  
                                      with the selector 
                                      in the first      
                                      literal frame     
                                      location          
                                      (`collect:`)      

                    **`124`**         return the object 
                                      on top of the     
                                      stack as the      
                                      value of the      
                                      message           
                                      (`classes`)       

                                      **literal         
                                      frame**\          
                                      `#collect:`\      
                                      `     `           
  ----------------- ----------------- ----------------- -----------------

A new `BlockContext` is created by the `blockCopy:` message to the
active context. The bytecode that pushes the active context was not
described along with the rest of the push bytecodes since the function
of contexts had not been described at that point. The argument to
`blockCopy: `(1 in this example) indicates the number of block arguments
the block requires. The `BlockContext` shares much of the state of the
active context that creates it. The receiver, arguments, temporary
variables, `CompiledMethod`, and sender are all the same. The
`BlockContext` has its own instruction pointer and stack. Upon returning
from the `blockCopy:` message, the newly created `BlockContext` is on
the stack of the active context and the next instruction jumps around
the bytecodes that describe the actions of the block. The active context
gave the `BlockContext` an initial instruction pointer pointing to the
bytecode after this jump. The compiler always uses an extended
(two-byte) jump after a `blockCopy:` so that the `BlockContext's`
initial instruction pointer is always two more than the active
context\'s instruction pointer when it receives the `blockCopy:`
message.\
    The method for `Collection` classes creates a `BlockContext`, but
does not execute its bytecodes. When the collection receives the
`collect:` message, it will repeatedly send `value:` messages to the
`BlockContext` with the elements of the collection as arguments. A
`BlockContext` responds to `value:` by becoming the active context,
which causes its bytecodes to be executed by the interpreter. Before the
`BlockContext` becomes active, the argument to `value: `is pushed onto
the `BlockContext's` stack. The first bytecode executed by the
`BlockContext` stores this value in a temporary variable used for the
block argument.\
    A `BlockContext` can return a value in two ways. After the bytecodes
in the block have been executed, the final value on the stack is
returned as the value of the message `value` or `value: `. The block can
also return a value to the message that invoked the `CompiledMethod`
that created the `BlockContext`. This is done with the regular return
bytecodes. The hypothetical method for `Collection containsInstanceOf:`
uses both types of return from a `BlockContext`.

**containsInstanceOf: aClass**

self do: \[:element \| (element isKindOf: aClass) ifTrue: \[\^true\]\].

\^false

  ----------------------- ----------------- -------------------------- -----------------
  **Collection                                                         
  containsInstanceOf:**                                                
  requires 1 temporary                                                 
  variable                                                             

                          **`112`**         push the receiver (`self`) 
                                            onto the stack             

                          **`137`**         push the active context    
                                            (`thisContext`) onto the   
                                            stack                      

                          **`118`**         push the `SmallInteger`    
                                            `1` onto the stack         

                          **`200`**         send a single argument     
                                            message with the selector  
                                            `blockCopy:`               

                          **`164,8`**       jump around the next 8     
                                            bytes                      

                          **`105`**         pop the top object off of  
                                            the stack and store in the 
                                            second temporary frame     
                                            location (`element`)       

                          **`17`**          push the contents of the   
                                            second temporary frame     
                                            location (`element`) onto  
                                            the stack                  

                          **`16`**          push the contents of the   
                                            first temporary frame      
                                            location (`aClass`) onto   
                                            the stack                  

                          **`224`**         send a single argument     
                                            message with the selector  
                                            in the first literal frame 
                                            location (`isKindOf:`)     

                          **`152`**         pop the top object off of  
                                            the stack and jump around  
                                            1 byte if it is `false`    

                          **`121`**         return `true` as the value 
                                            of the message             
                                            (`containsInstanceOf:`)    

                          **`115`**         push `nil` onto the stack  

                          **`125`**         return the object on top   
                                            of the stack as the value  
                                            of the block               

                          **`203`**         send the single argument   
                                            message with the selector  
                                            `do:`                      

                          **`135`**         pop the top object off the 
                                            stack                      

                          **`122`**         return false as the value  
                                            of the message             
                                            (`containsInstanceOf:`)    

                                            **literal frame**\         
                                            `#isKindOf:`\              
                                            `    `                     
  ----------------------- ----------------- -------------------------- -----------------

------------------------------------------------------------------------

\
[]{#Messages26}***Messages***\

------------------------------------------------------------------------

Then a send bytecode is encountered, the interpreter finds the
`CompiledMethod` indicated by the message as follows.\
 

  ----------------------------------- -----------------------------------
  1.                                  *Find the message receiver.* The
                                      receiver is below the arguments on
                                      the stack. The number of arguments
                                      is indicated in the send bytecode.

  2.                                  *Access a message dictionary.* The
                                      original message dictionary is
                                      found in the receiver\'s class.

  3.                                  *Look up the message selector in
                                      the message dictionary.* The
                                      selector is indicated in the send
                                      bytecode.

  4.                                  *If the selector is found,* the
                                      associated `CompiledMethod`
                                      describes the response to the
                                      message.

  5.                                  *If the selector is not found,* a
                                      new message dictionary must be
                                      searched (returning to step 3). The
                                      new message dictionary will be
                                      found in the superclass of the last
                                      class whose message dictionary was
                                      searched. This cycle may be
                                      repeated several times, traveling
                                      up the superclass chain.\
                                           
  ----------------------------------- -----------------------------------

If the selector is not found in the receiver\'s class nor in any of its
superclasses, an error is reported, and execution of the bytecodes
following the send is suspended.

![](bluesquare.gif){height="20" width="20" align="TEXTTOP"}***Superclass
Sends***  A variation of the send bytecodes called *super-sends* uses a
slightly different algorithm to find the `CompiledMethod` associated
with a message. Everything is the same except for the second step, which
specifies the original message dictionary to search. When a super-send
is encountered, the following second step is substituted.\
 

  ----------------------------------- -----------------------------------
  2.                                  *Access a message dictionary.* The
                                      original message dictionary is
                                      found in the superclass of the
                                      class in which the currently
                                      executing `CompiledMethod `was
                                      found.\
                                           

  ----------------------------------- -----------------------------------

Super-send bytecodes are used when `super` is used as the receiver of a
message in a source method. The bytecode used to push the receiver will
be the same as if `self` had been used, but a super-send bytecode will
be used to describe the selector.\
    As an example of the use of a super-send, imagine a subclass of
`Rectangle` called `ShadedRectangle` that adds an instance variable
named `shade`. A `Rectangle` might respond to the message `shade:` by
producing a new `ShadedRectangle`. `ShadedRectangle` provides a new
method for the message `intersect: `, returning a `ShadedRectangle`
instead of a `Rectangle`. This method must use `super` to access its own
ability to actually compute the intersection.

**intersect: aRectangle**

\^(super intersect: aRectangle) shade: shade

  ------------------- ----------------- ----------------------------------------------------- -----------------
  **ShadedRectangle                                                                           
  intersect:**                                                                                

                      **`112`**         push the receiver (`self`) onto the stack             

                      **`16`**          push the contents of the first temporary frame        
                                        location (the argument `aRectangle`) onto the stack   

                      **`133,33`**      send to `super` a single argument message with the    
                                        selector in the second literal frame location         
                                        (`intersect:`)                                        

                      **`2`**           push the value of the receiver\'s third instance      
                                        variable (`shade`) onto the stack                     

                      **`224`**         send a single argument message with the selector in   
                                        the first literal frame location (`shade:`)           

                      **`124`**         return the object on top of the stack as the value of 
                                        the message (`intersect:`)                            

                                        **literal frame**\                                    
                                        `#shade: `\                                           
                                        `#intersect: `\                                       
                                        `Association: #ShadedRectangle -> ShadedRectangle`\   
                                        `     `                                               
  ------------------- ----------------- ----------------------------------------------------- -----------------

It is important to note that the initial class searched in response to a
super-send will be the superclass of the receiver\'s class only if the
`CompiledMethod` containing the super-send was originally found the
receiver\'s class. If the `CompiledMethod` was originally found in a
superclass of the receiver\'s class, the search will start in *that*
class\'s superclass. Since the interpreter\'s state does not include the
class in which it found each `CompiledMethod`, that information is
included in the `CompiledMethod` itself. Every `CompiledMethod` that
includes a super-send bytecode refers to the class in whose message
dictionary it is found. The last entry of the literal frame of those
`CompiledMethods` contains an association referring to the class.

[]{#PrimitiveMethods26}***Primitive Methods***\

------------------------------------------------------------------------

The interpreter\'s actions after finding a `CompiledMethod` depend on
whether or not the `CompiledMethod` indicates that a *primitive method*
may be able to respond to the message. If no primitive method is
indicated, a new `MethodContext` is created and made active as described
in previous sections. If a primitive method is indicated in the
`CompiledMethod`, the interpreter may be able to respond to the message
without actually executing the bytecodes. For example, one of the
primitive methods is associated with the `+` message to instances of
`SmallInteger`.

**+ addend**

\<primitive: 1\>

\^super + addend

  ----------------- ----------------- ----------------- -----------------
  **SmallInteger                                        
  +** associated                                        
  with primitive #1                                     

                    **`112`**         push the receiver 
                                      (`self`) onto the 
                                      stack             

                    **`16`**          push the contents 
                                      of the first      
                                      temporary frame   
                                      location (the     
                                      argument          
                                      `addend`) onto    
                                      the stack         

                    **`133,32`**      send to `super` a 
                                      single argument   
                                      message with the  
                                      selector in the   
                                      first literal     
                                      frame location    
                                      (`+`)             

                    **`124`**         return the object 
                                      on top of the     
                                      stack as the      
                                      value of the      
                                      message (`+`)     

                                      **literal         
                                      frame**\          
                                      \#`+`\            
                                      `    `            
  ----------------- ----------------- ----------------- -----------------

Even if a primitive method is indicated for a `CompiledMethod`, the
interpreter may not be able to respond successfully. For example, the
argument of the `+` message might not be another instance of
`SmallInteger `or the sum might not be representable by a
`SmallInteger`. If the interpreter cannot execute the primitive for some
reason, the primitive is said to *fail*. When a primitive fails, the
bytecodes in the `CompiledMethod `are executed as if the primitive
method had not been indicated. The method for `SmallInteger +` indicates
that the `+` method in the superclass (`Integer`) will be used if the
primitive fails.\
    There are about a hundred primitive methods in the system that
perform four types of operation. The exact function of all of the
primitives will be described in [Chapter 29](bluebook_chapter29.html).

1.  Arithmetic
2.  Storage management
3.  Control
4.  Input-output

------------------------------------------------------------------------

[]{#TheObjectMemory26}**The Object Memory**

------------------------------------------------------------------------

The object memory provides the interpreter with an interface to the
objects that make up the Smalltalk-80 virtual image. Each object is
associated with a unique identifier called its *object pointer*. The
object memory and interpreter communicate about objects with object
pointers. The size of object pointers determines the maximum number of
objects a Smalltalk-80 system can contain. This number is not fixed by
anything about the language, but the implementation described in this
book uses 16-bit object pointers, allowing 65536 objects to be
referenced. Implementation of the Smalltalk-80 system with larger object
references will require changing certain parts of the virtual machine
specification. It is not within the scope of this book to detail the
relevant changes.\
    The object memory associates each object pointer with a set of other
object pointers. Every object pointer is associated with the object
pointer of a class. If an object has instance variables, its object
pointer is also associated with the object pointers of their values. The
individual instance variables are referred to by zero-relative integer
indices. The value of an instance variable can be changed, but the class
associated with an object cannot be changed. The object memory provides
the following five fundamental functions to the interpreter.\
 

  ----------------------------------- -----------------------------------
  1\.                                 *Access* the value of an object\'s
                                      *instance variable.* The object
                                      pointer of the instance and the
                                      index of the instance variable must
                                      be supplied. The object pointer of
                                      the instance variable\'s value is
                                      returned. 

  2\.                                 *Change* the value of an object\'s
                                      *instance variable.* The object
                                      pointer of the instance and the
                                      index of the instance variable must
                                      be supplied. The object pointer of
                                      the new value must also be
                                      supplied. 

  3\.                                 *Access* an object\'s *class*. The
                                      object pointer of the instance must
                                      be supplied. The object pointer of
                                      the instance\'s class is returned. 

  4\.                                 *Create* a new *object*. The object
                                      pointer of the new object\'s class
                                      and the number of instance
                                      variables it should have must be
                                      supplied. The object pointer of the
                                      new instance is returned. 

  5\.                                 Find the *number of instance
                                      variables* an object has. The
                                      object\'s pointer must be supplied.
                                      The number of instance variables is
                                      returned.\
                                            
  ----------------------------------- -----------------------------------

There is no explicit function of the object memory to remove an object
no longer being used because these objects are reclaimed automatically.
An object is reclaimed when there are no object pointers to it from
other objects. This reclamation can be accomplished either by reference
counting or garbage collection.\
    There are two additional features of the object memory that provide
efficient representation of numerical information. The first of these
sets aside certain object pointers for instances of class
`SmallInteger `. The second allows objects to contain integer values
instead of object pointers.

![](bluesquare.gif){height="20" width="20"
align="TEXTTOP"}***Representation of Small Integers *** The instances of
class `SmallInteger` represent the integers -16384 through 16383. Each
of these instances is assigned a unique object pointer. These object
pointers all have a 1 in the low-order bit position and the two\'s
complement representation of their value in the high-order 15 bits. An
instance of `SmallInteger` needs no instance storage since both its
class and its value can be determined from its object pointer. Two
additional functions are provided by the object memory to convert back
and forth between `SmallInteger` object pointers and numerical values.\
 

  ----------------------------------- -----------------------------------
  6\.                                 Find the numerical value
                                      represented by a `SmallInteger`.
                                      The object pointer of the
                                      `SmallInteger` must be supplied.
                                      The two\'s complement value is
                                      returned. 

  7\.                                 Find the `SmallInteger`
                                      representing a numerical value. The
                                      two\'s complement value must be
                                      supplied. A `SmallInteger` object
                                      pointer is returned.\
                                            
  ----------------------------------- -----------------------------------

This representation for `SmallIntegers` implies that there can be 32768
instances of the other classes in the system. It also implies that
equality (=) and equivalence (==) will be the same for instances of
`SmallInteger`. Integers outside the range -16384 through 16383 are
represented by instances of class `LargePositiveInteger` or
`LargeNegativeInteger`. There may be several instances representing the
same value, so equality and equivalence are different.

![](bluesquare.gif){height="20" width="20"
align="TEXTTOP"}***Collections of Integer Values*** Another special
representation is included for objects representing collections of
integers. Instead of storing the object pointers of the `SmallIntegers`
representing the contents of the collection, the actual numerical values
are stored. The values in these special collections are constrained to
be positive. There are two varieties of collection, one limiting its
values to be less than 256 and the other limiting its values to be less
than 65536. The object memory provides functions analogous to the first
five listed in this section, but for objects whose contents are
numerical values instead of object pointers.\
    The distinction between objects that contain object pointers and
those that contain integer values is never visible to the Smalltalk-80
programmer. When one of these special numerical collections is accessed
by sending it a message, the object pointer of an object representing
the value is returned. The nature of these special collections is only
evident in that they may refuse to store objects that do not represent
integers within the proper range.

------------------------------------------------------------------------

[]{#TheHardware26}**The Hardware**

------------------------------------------------------------------------

The Smalltalk-80 implementation has been described as a virtual machine
to avoid unnecessary hardware dependencies. It is naturally assumed that
the hardware will include a processor and more than enough memory to
store the virtual image and the machine language routines simulating the
interpreter and object memory. The current size of the virtual image
requires at least a half megabyte of memory.\
    The size of the processor and the organization of the memory are not
actually constrained by the virtual machine specification. Since object
pointers are 16 bits, the most convenient arrangement would be a 16-bit
processor and a memory of 16-bit words. As with the processor and memory
of any system, the faster the better.\
    The other hardware requirements are imposed by the primitives that
the virtual image depends on. These input-output devices and clocks are
listed below.

1.  A bitmap display. It is most convenient if the bitmap being
    displayed can be located in the object memory, although this is not
    absolutely necessary.
2.  A pointing device.
3.  Three buttons associated with the pointing device. It is most
    convenient if these are physically located on the device.
4.  A keyboard, either decoded ASCII or undecoded ALTO.
5.  A disk. The standard Smalltalk-80 virtual image contains only a
    skeleton disk system that must be tailored to the actual disk used.
6.  A millisecond timer.
7.  A real time clock with one second resolution.

------------------------------------------------------------------------

\[[TOC](bluebook_imp_toc.html)\] \[[26](#top_of_26)\]
\[[27](bluebook_chapter27.html)\] \[[28](bluebook_chapter28.html)\]
\[[29](bluebook_chapter29.html)\] \[[30](bluebook_chapter30.html)\]
