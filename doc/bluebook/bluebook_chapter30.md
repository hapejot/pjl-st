---
GENERATOR: Mozilla/4.5 \[en\] (WinNT; I) \[Netscape\]
title: Blue Book Chapter 30
---

[]{#top_of_30}\[[TOC](bluebook_imp_toc.html)\]
\[[26](bluebook_chapter26.html)\] \[[27](bluebook_chapter27.html)\]
\[[28](bluebook_chapter28.html)\] \[[29](bluebook_chapter29.html)\] 30\

------------------------------------------------------------------------

30\
Formal Specification of the Object Memory

------------------------------------------------------------------------

**[Heap Storage](#HeapStorage30)**

*[Compaction](#Compaction30)*

**[The Object Table](#TheObjectTable30)**

*[Object Pointers](#ObjectPointers30)*

*[Object Table Entries](#ObjectTableEntries30)*

*[Unallocated Space](#UnallocatedSpace30)*

**[Allocation and Deallocation](#AllocationAndDeallocation30)**

*[An Allocation Algorithm](#AnAllocationAlgorithm30)*

*[A Deallocation Algorithm](#ADeallocationAlgorithm30)*

*[A Compaction Algorithm](#ACompactionAlgorithm30)*

**[Garbage Collection](#GarbageCollection30)**

*[A Simple Reference-counting
Collector](#ASimpleReferenceCountingCollector30)*

*[A Space-efficient Reference-counting
Collector](#ASpaceEfficientReferenceCountingCollector30)*

*[A Marking Collector](#AMarkingCollector30)*

**[Nonpointer Objects](#NonpointerObjects30)**

*[CompiledMethods](#CompiledMethods30)*

**[Interface to the Bytecode
Interpreter](#InterfaceToTheBytecodeInterpreter30)**

------------------------------------------------------------------------

The two major components of any Smalltalk-80 implementation are the
bytecode interpreter and the object memory. Chapters
[28](bluebook_chapter28.html) and [29](bluebook_chapter29.html)
described an implementation of the bytecode interpreter. This chapter
describes an implementation of the object memory. The function of the
object memory is to create, store, and destroy objects, and to provide
access to their fields.\
    Memory-management systems fall into two major categories,
*real-memory* implementations and *virtual-memory* implementations. In a
real-memory implementation, all the objects in the environment reside in
primary memory that is directly addressable by the program. In a
virtual-memory implementation, objects reside in more than one level of
a memory hierarchy and must be shuffled among the various levels during
execution. This chapter describes the design of `RealObjectMemory`, an
object memory for a real-memory Smalltalk-80.\
    Although Smalltalk can be implemented on computers of any word size,
this presentation will be simplified by several assumptions in the
standard algorithms. The routines of `RealObjectMemory` assume

-   that there are eight bits in a byte,
-   that there are two bytes in a word,
-   that the more significant byte of a word precedes the less
    significant byte, and
-   that the target computer is word addressed and word indexed.

Moreover, the routines assume that the address space is partitioned into
16 or fewer *segments* of 64K (65,536) words apiece. The standard
algorithms can be systematically changed to adapt them to hardware with
different properties. The routines of `RealObjectMemory` deal almost
exclusively with 16-bit integers, as would a machine-language
implementation.\
    To access locations in the address space of the host machine,
machine language implementations use load and store instructions. In
`RealObjectMemory`, the load and store instructions are symbolized by
messages to an instance of `RealWordMemory` whose name is `wordMemory`.
The protocol of `RealWordMemory` is shown below

`RealWordMemory` instance protocol

**segment: s word: w**

Return word `w` of segment `s`.

**segment: s word: w put: value**

Store `value` into word `w` of segment `s`; return `value`.

**segment: s word: w byte: byteNumber**

Return byte `byteNumber` of word `w` of segment `s`.

**segment: s word: w byte: byteNumber put: value**

Store `value` into byte `byteNumber` of word `w` of segment `s`; return
`value`.

**segment: s word: w bits: firstBitIndex to: lastBitIndex**

Return bits `firstBitIndex` to `lastBitIndex` of word `w` of segment
`s`.

**segment: s word: w bits: firstBitIndex to: lastBitIndex put: value**

Store `value` into bits `firstBitIndex` to `lastBitIndex` of word `w` of
segment `s`; return `value`

When it is necessary to distinguish the two bytes of a word, the left
(more significant) byte will be referred to with the index 0 and the
right (less significant) byte with the index 1. The most significant bit
in a word will be referred to with the index 0 and the least significant
with the index 15. Note that `self` is an instance of
class` RealObjectMemory` in all routines of this chapter.\
    The most important thing about any implementation of the object
memory is that it conform to the functional specification of the object
memory interface given in [Chapter 27](bluebook_chapter27.html). This
chapter describes a range of possible implementations of that interface.
In particular, simple versions of some routines are presented early in
the chapter and refined versions are presented later as the need for
those refinements becomes clear. These preliminary versions will be
flagged by including the comment, *\"\*\*Preliminary Version\*\*\"*, on
the first line of the routine.

------------------------------------------------------------------------

[]{#HeapStorage30}**Heap Storage**

------------------------------------------------------------------------

In a real-memory implementation of Smalltalk, all objects are stored in
an area called the *heap*. A new object is created by obtaining space to
store its fields in a contiguous series of words in the heap. An object
is destroyed by releasing the heap space it occupied. The format of an
allocated object in the heap is shown in [Figure 30.1](#Figure_30.1).
The actual data of the object are preceded by a two-word *header*. The
size field of the header indicates the number of words of heap that the
object occupies, including the header. It is an unsigned 16-bit number,
and can range from 2 up to 65,535.\
[]{#Figure_30.1}

  -----------------------------------------------
  ![](figure30_1.gif){height="249" width="253"}
  **Figure 30.1**
  -----------------------------------------------

When memory is segmented, it is usually convenient for a Smalltalk-80
implementation to divide the heap into *heap segments*, each in a
different memory segment. As stated earlier, the routines in this
chapter assume that the target computer is segmented into address spaces
of 65,536 words.\
 

  ----------------------------------- ----------------------------------------------
  `HeapSegmentCount`                  The number of heap segments used in the
                                      implementation.

  `FirstHeapSegment`                  The index of the first memory segment used to
                                      store the heap.

  `LastHeapSegment`                   The index of the last memory segment used to
                                      store the heap
                                      (`FirstHeapSegment + HeapSegmentCount - 1`).

  `HeapSpaceStop`                     The address of the last location used in each
                                      heap segment.

  `HeaderSize`                        The number of words in an object header (2).\
                                             
  ----------------------------------- ----------------------------------------------

  : **Heap Related Constants**

[]{#Compaction30}***Compaction***\

------------------------------------------------------------------------

Suppose for a moment that an object once allocated never changes its
location in the heap. To allocate a new object, a space between existing
objects must be found that is large enough to hold the new object. After
a while, the memory \"fragments\" or \"checkerboards.\" That is, an
allocation request is bound to arrive for an amount of space smaller
than the total available memory but larger than any of the disjoint
pieces ([Figure 30.2a](#Figure_30.2)). This can occur even if there is a
large amount of available space and a relatively small allocation
request.\
    Fragmentation cannot be tolerated in an interactive system that is
expected to preserve a dynamic environment for hundreds of hours or more
without reinitialization. Therefore when memory fragments, it must be
*compacted*. Memory is compacted by moving all objects that are still in
use towards one end of the heap, squeezing out all the free space
between them and leaving one large unallocated block at the other end
(see [Figure 30.2b](#Figure_30.2)).\
    Each heap segment is compacted separately. Even on a
linearly-addressed machine it is preferable to segment a large heap to
reduce the duration of each compaction.\
[]{#Figure_30.2}

  -----------------------------------------------
  ![](figure30_2.gif){height="420" width="308"}
  **Figure 30.2**
  -----------------------------------------------

------------------------------------------------------------------------

[]{#TheObjectTable30}**The Object Table**

------------------------------------------------------------------------

When an object is moved during compaction, all pointers to its heap
memory must be updated. If many other objects contain pointers directly
to the old location, then it is time-consuming on a sequential computer
to find and update those references to point to the new location.
Therefore to make the pointer update inexpensive, only one pointer to an
object\'s heap memory is allowed. That pointer is stored in a table
called the *object table*. All references to an object must be
indirected through the object table. Thus, the object pointers found in
Smalltalk objects are really indices into the object table, in which
pointers into the heap are in turn found (see [Figure
30.3](#Figure_30.3)).\
[]{#Figure_30.3}

  -----------------------------------------------
  ![](figure30_3.gif){height="285" width="495"}
  **Figure 30.3**
  -----------------------------------------------

    Indirection through the object table provides another benefit. The
number of objects of average size Z addressable by an n-bit pointer is
on the order of 2^n^ instead of 2^n^/Z. In our experience, objects
average 10 words in size (Z\~10), so a significant gain in address space
can be realized by indirection.\
    Throughout the object table, abandoned entries can occur that are
not associated with any space on the heap. These entries are called
*free entries* and their object pointers are called *free pointers*. It
is easy to recycle a free entry, because all object table entries are
the same size. Compaction of the object table is difficult and generally
unnecessary, so it is not supported.\
    Although the heap is segmented, the object table is stored in a
single segment so that an object pointer can be 16 bits and thus fit in
one word. Consequently, the number of objects that can be addressed in
real memory is limited to the number of object table entries that can
fit in one segment. A common arrangement is for each object table entry
to occupy two words and for the entire table to occupy 64K words or
less, yielding a maximum capacity of 32K objects.

[]{#ObjectPointers30}***Object Pointers***\

------------------------------------------------------------------------

An object pointer occupies 16 bits, apportioned as in [Figure
30.4](#Figure_30.4).\
[]{#Figure_30.4}

  ----------------------------------------------
  ![](figure30_4.gif){height="99" width="384"}
  **Figure 30.4**
  ----------------------------------------------

When the low-order bit of the object pointer is 0, the first 15 bits are
an index into the object table. Up to 2^15^ (32K) objects can be
addressed. When the low-order bit of the object pointer is 1, the first
15 bits are an immediate signed integer, and no additional space in the
object table or the heap is utilized. The benefit of giving special
treatment to integers in the range ±2^14^ is that they come and go with
high frequency during arithmetic and many other operations. The cost of
their efficient representation is the number of tests the interpreter
must perform to distinguish object pointers of small integers from
object pointers of other objects.\
    The `isIntegerObject:` routine tests the low order bit of
`objectPointer` to determine whether the rest of the pointer is an
immediate integer value rather than an object table index.

**isIntegerObject: objectPointer**

\^(objectPointer bitAnd: 1) = 1

Every other object-access routine requires that its object pointer
argument really be an object table index. The `cantBeIntegerObject:`
routine is used to trap erroneous calls, If the hardware, the bytecode
interpreter, and the object memory manager are bug free, then this error
condition is never encountered.

**cantBeIntegerObject: objectPointer**

(self isIntegerObject: objectPointer)

ifTrue: \[Sensor notify: \'A small integer has no object table entry\'\]

[]{#ObjectTableEntries30}***Object Table Entries***\

------------------------------------------------------------------------

The format of an object table entry is shown in [Figure
30.5](#Figure_30.5). If the free entry bit is on, then the entry is
free. If the free entry bit is off, then the four segment bits select a
heap segment and the 16 location bits locate the beginning of the space
in that segment that is owned by the object table entry. The count
field, the odd length bit (O), and the pointer fields bit will be
explained later in the chapter.\
[]{#Figure_30.5}

  ----------------------------------------------
  ![](figure30_5.gif){height="97" width="383"}
  **Figure 30.5**
  ----------------------------------------------

  ----------------------------------- -----------------------------------
  `ObjectTableSegment`                The number of the memory segment
                                      containing the object table.

  `ObjectTableStart`                  The location in
                                      `ObjectTableSegment` of the base of
                                      the object table.

  `ObjectTableSize`                   The number of words in the object
                                      table (an even number \<= 64K).

  `HugeSize`                          The smallest number that is too
                                      large to represent in an eight-bit
                                      count field; that is, 256.

  `NilPointer`                        The object table index of the
                                      object `nil.`\
                                      `       `
  ----------------------------------- -----------------------------------

  : **Object Table Related Constants**

The following set of routines accesses the first word of object table
entries in four different ways: loading the whole word, storing the
whole word, loading a bit field, and storing a bit field. These routines
in turn utilize routines of `wordMemory`, an instance of
`RealWordMemory`. They assume that `objectPointer` is expressed as an
even word offset relative to `ObjectTableStart`, the base of the object
table in segment `ObjectTableSegment`. Note that `ot` is an abbreviation
for \"object table.\"

**ot: objectPointer**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer

**ot: objectPointer put: value**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer

put: value

**ot: objectPointer bits: firstBitIndex to: lastBitIndex**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer

bits: firstBitIndex

to: lastBitIndex

**ot: objectPointer bits: firstBitIndex to: lastBitIndex put: value**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer

bits: firstBitIndex

to: lastBitIndex

put: value

The following 12 object-access subroutines load or store the various
fields of the object table entry of `objectPointer`.

**countBitsOf: objectPointer**

\^self ot: objectPointer bits: 0 to: 7

**countBitsOf: objectPointer put: value**

\^self ot: objectPointer bits: 0 to: 7 put: value

**oddBitOf: objectPointer**

\^self ot: objectPointer bits: 8 to: 8

**oddBitOf: objectPointer put: value**

\^self ot: objectPointer bits: 8 to: 8 put: value

**pointerBitOf: objectPointer**

\^self ot: objectPointer bits: 9 to: 9

**pointerBitOf: objectPointer put: value**

\^self ot: objectPointer bits: 9 to: 9 put: value

**freeBitOf: objectPointer**

\^self ot: objectPointer bits: 10 to: 10

**freeBitOf: objectPointer put: value**

\^self ot: objectPointer bits: 10 to: 10 put: value

**segmentBitsOf: objectPointer**

\^self ot: objectPointer bits: 12 to: 15

**segmentBitsOf: objectPointer put: value**

\^self ot: objectPointer bits: 12 to: 15 put: value

**locationBitsOf: objectPointer**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer + 1

**locationBitsOf: objectPointer put: value**

self cantBeIntegerObject: objectPointer.

\^wordMemory segment: ObjectTableSegment

word: ObjectTableStart + objectPointer + 1

put: value

For objects that occupy a chunk of heap storage (those whose free bit is
0), the following four object-access subroutines load or store words or
bytes from the chunk.

**heapChunkOf: objectPointer word: offset**

\^wordMemory segment: (self segmentBitsOf: objectPointer)

word: ((self locationBitsOf: objectPointer) + offset)

**heapChunkOf: objectPointer word: offset put: value**

\^wordMemory segment: (self segmentBitsOf: objectPointer)

word: ((self locationBitsOf: objectPointer) + offset)

put: value

**heapChunkOf: objectPointer byte: offset**

\^wordMemory segment: (self segmentBitsOf: objectPointer)

word: ((self locationBitsOf: objectPointer) + (offset // 2))

byte: (offset \\\\ 2)

**heapChunkOf: objectPointer byte: offset put: value**

\^wordMemory segment: (self segmentBitsOf: objectPointer)

word: ((self locationBitsOf: objectPointer) + (offset // 2))

byte: (offset \\\\ 2)

put: value

The next four object-access subroutines are more specialized in that
they load or store words of the object header.

**sizeBitsOf: objectPointer**

\^self heapChunkOf: objectPointer word: 0

**sizeBitsOf: objectPointer put: value**

\^self heapChunkOf: objectPointer word: 0 put: value

**classBitsOf: objectPointer**

\^self heapChunkOf: objectPointer word: 1

**classBitsOf: objectPointer put: value**

\^self heapChunkOf: objectPointer word: 1 put: value

The remaining two object-access subroutines are functionally identical
to `sizeBitsOf:` in the versions shown below. Later in this chapter,
refinements to the object-memory manager will require new versions of
both of these subroutines that will return something different from the
object size in certain cases. For that reason, these methods are marked
\"preliminary.\"

**lastPointerOf: objectPointer** *\"\*\*Preliminary Version\*\*\"*

\^self sizeBitsOf: objectPointer

**spaceOccupiedBy: objectPointer** *\"\*\*Preliminary Version\*\*\"*

\^self sizeBitsOf: objectPointer

[]{#UnallocatedSpace30}***Unallocated Space***\

------------------------------------------------------------------------

All free entries in the object table are kept on a linked list headed at
the location named freePointerList. The link from one free entry to the
next is an object pointer in its location field (see [Figure
30.6](#Figure_30.6)).\
[]{#Figure_30.6}

  -----------------------------------------------
  ![](figure30_6.gif){height="275" width="460"}
  **Figure 30.6**
  -----------------------------------------------

Unallocated space in the heap is grouped into *free chunks* (contiguous
blocks) of assorted sizes and each of those free chunks is assigned an
object table entry. Free chunks are linked together on lists, each
containing chunks of the same size. The link from one free chunk to the
next is in its class field ([Figure 30.7](#Figure_30.7)). To keep the
table of list heads small, all free chunks bigger than 20 words are
linked onto a single list.\
 

  ----------------------------------- -----------------------------------
  `FreePointerList`                   The location of the head of the
                                      linked list of free object table
                                      entries.

  `BigSize`                           The smallest size of chunk that is
                                      not stored on a list whose chunks
                                      are the same size. (The index of
                                      the last free chunk list).

  `FirstFreeChunkList`                The location of the head of the
                                      linked list of free chunks of size
                                      zero. Lists for chunks of larger
                                      sizes are stored in contiguous
                                      locations following
                                      `FirstFreeChunkList`. Note that the
                                      lists at `FirstFreeChunkList` and
                                      `FirstFreeChunkList` `+` `1` will
                                      always be empty since all chunks
                                      are at least two words long.

  `LastFreeChunkList`                 The location of the head of the
                                      linked list of free chunks of size
                                      `BigSize` or larger.

  `NonPointer`                        Any sixteen-bit value that cannot
                                      be an object table index, e.g.,
                                      2^16^ - 1.\
                                              
  ----------------------------------- -----------------------------------

  : **Free Space Related Constants**

A separate set of free chunk lists is maintained for each heap segment,
but only one free pointer list is maintained for the object table. Note
that the object table entry associated with a \"free chunk\" is not a
\"free entry.\" It is not on the free pointer list, and its free entry
bit is not set. The way a free chunk is distinguished from an allocated
chunk is by setting the count field of the object table entry to zero
for a free chunk and to nonzero for an allocated chunk.\
[]{#Figure_30.7}

  -----------------------------------------------
  ![](figure30_7.gif){height="434" width="529"}
  **Figure 30.7**
  -----------------------------------------------

The following four routines manage the free pointer list headed at
`FreePointerList` in segment `ObjectTableSegment`. The first two
routines simply load and store the list head.

**headOfFreePointerList**

\^wordMemory segment: ObjectTableSegment

word: FreePointerList

**headOfFreePointerListPut: objectPointer**

\^wordMemory segment: ObjectTableSegment

word: FreePointerList

put: objectPointer

The routine `toFreePointerListAdd:` adds a free entry to the head of the
list.

**toFreePointerListAdd: objectPointer**

self locationBitsOf: objectPointer

put: (self headOfFreePointerList).

self headOfFreePointerListPut: objectPointer

The routine `removeFromFreePointerList` removes the first entry from the
list and returns it; if the list was empty, it returns `nil`. The
distinguished value `NonPointer` signifies the end of a linked list. A
good value for `NonPointer` is 2^16^ - 1, a value that is easily
detected on most computers and that cannot be confused with an actual
object table entry address because it is an odd number.

**removeFromFreePointerList**

\| objectPointer \|

objectPointer := self headOfFreePointerList.

objectPointer = NonPointer ifTrue: \[\^nil\].

self headOfFreePointerListPut: (self locationBitsOf: objectPointer).

\^objectPointer

The following routines manage the free-chunk lists headed at
`FirstFreeChunkList + 2` through `LastFreeChunkList` of each heap
segment. Their behavior is exactly analogous to that of the routines
above. The first three routines work in the segment specified or implied
by their second parameter. The fourth routine works in the segment
specified by the register `currentSegment`.

**headOfFreeChunkList: size inSegment: segment**

\^wordMemory segment: segment

word: FirstFreeChunkList + size

**headOfFreeChunkList: size inSegment: segment put: objectPointer**

\^wordMemory segment: segment

word: FirstFreeChunkList + size

put: objectPointer

**toFreeChunkList: size add: objectPointer**

\| segment \|

segment := self segmentBitsOf: objectPointer.

self classBitsOf: objectPointer

put: (self headOfFreeChunkList: size

inSegment: segment).

self headOfFreeChunkList: size

inSegment: segment

put: objectPointer

**removeFromFreeChunkList: size**

\| objectPointer secondChunk \|

objectPointer := self headOfFreeChunkList: size

inSegment: currentSegment.

objectPointer = NonPointer ifTrue: \[\^nil\].

secondChunk := self classBitsOf: objectPointer.

self headOfFreeChunkList: size

inSegment: currentSegment

put: secondChunk.

\^objectPointer

The routine `resetFreeChunkList:inSegment:` resets the specified
free-chunk list to an empty list.

**resetFreeChunkList: size inSegment: segment**

self headOfFreeChunkList: size

inSegment: segment

put: NonPointer

------------------------------------------------------------------------

[]{#AllocationAndDeallocation30}**Allocation and Deallocation**

------------------------------------------------------------------------

To allocate an object, an entry is obtained from the object table and
sufficient space for the header and data is obtained from some heap
segment. The heap segment in which space is found is called the *current
segment*. It becomes the first segment in which to look for space to
allocate the next object. The only register required by the object
memory holds the index of the current segment.\
 

  ----------------------------------- -----------------------------------
  `currentSegment`                    The index of the heap segment
                                      currently being used for
                                      allocation.\
                                               

  ----------------------------------- -----------------------------------

  : **Registers of the Object Memory**

To allocate a \"large\" object requiring `n` words of heap space
(`n >= BigSize`), the list beginning at `LastFreeChunkList` in the
current segment is searched for a free chunk whose size is either `n`
words or at least` n + headerSize `words. If the free chunk found is
larger than `n` words, it is *subdivided* and only `n` of the words are
used to satisfy the allocation request.\
    To allocate a \"small\" object requiring `n` words of heap space
(`headerSize <= n < BigSize`), the list beginning
at` freeChunkLists + n `is searched. If the list is nonempty, its first
free chunk is removed and used for the new object. If the list is empty,
the above algorithm for \"large\" objects is used.\
    If no chunk of sufficient size is found in the current segment, then
the next segment is made current and the search continues there. The new
current segment is compacted first to improve the chances of finding
sufficient space. In a compacted segment, all the allocated objects are
at one end and the (presumably large) space at the other end is all in
one large chunk, the sole member of the list `LastFreeChunkLists`. If
enough space is not found in any segment, execution is halted.\
    When an object is deallocated, its space is recycled on the list of
free chunks of the appropriate size. However, to simplify the
presentation in this chapter, the standard algorithms leave the unused
part of any subdivided chunk on the list of big free chunks even if that
part is small in size.

[]{#AnAllocationAlgorithm30}***An Allocation Algorithm***\

------------------------------------------------------------------------

The` allocate:class:` routine is presented below as an example of a
simple allocation routine. It takes as parameters the size of the
desired chunk (in words, including header) and the class of the object
that chunk will represent. The actual allocation routine takes several
other parameters and so the` allocate:class:` routine will be flagged as
preliminary. A more complete routine,` allocate:extra:class:`, is
presented in a later section and the actual routine used in the
implementation,` allocate:odd:pointer:extra:class:`, is presented after
that.

**allocate: size class: classPointer** *\"\*\*Preliminary Version\*\*\"*

\| objectPointer \|

objectPointer := self allocateChunk: size. *\"actually allocate\"*

self classBitsOf: objectPointer put: classPointer. *\"fill in class\"*

*\"initialize all fields to the object table index of the object nil\"*

(headerSize to: size - 1) do:

\[:i \| self heapChunkOf: objectPointer word: i put: NilPointer\].

self sizeBitsOf: objectPointer put: size.

*\"return the new object\'s pointer\"*

\^objectPointer

The routine `allocateChunk:` either succeeds in its allocation task, or
it reports an unrecoverable error. It uses a
subroutine,` attemptToAllocateChunk:`, that either completes the job or
returns `nil` if no space can be found.

**allocateChunk: size** *\"\*\*Preliminary Version\*\*\"*

\| objectPointer \|

objectPointer := self attemptToAllocateChunk: size.

objectPointer isNil ifFalse: \[\^objectPointer\].

\^self error: \'Out of memory\'

The `attemptToAllocateChunk:` routine first tries to allocate in
`currentSegment`, the segment currently targeted for allocations. It
does so using the subroutine` attemptToAllocateChunkInCurrentSegment:`.
If the subroutine fails (returns `nil`), then the routine compacts the
next segment and retries the allocation there. This procedure continues
until the original segment has been compacted and searched. If no space
can be found anywhere, the routine returns `nil`. Note that it uses
implementation-dependent constants: `HeapSegmentCount`,
`FirstHeapSegment`, and `LastHeapSegment`.

**attemptToAllocateChunk: size**

\| objectPointer \|

objectPointer := self attemptToAllocateChunkInCurrentSegment: size.

objectPointer isNil ifFalse: \[\^objectPointer\].

1 to: HeapSegmentCount do:

\[:i \|

currentSegment := currentSegment + 1.

currentSegment \> LastHeapSegment

ifTrue: \[currentSegment := FirstHeapSegment\].

self compactCurrentSegment.

objectPointer := self attemptToAllocateChunkInCurrentSegment: size.

objectPointer isNil ifFalse: \[\^objectPointer\]\].

\^nil

The `attemptToAllocateChunkInCurrentSegment:` routine searches the
current heap segment\'s free-chunk lists for the first chunk that is the
right size or that can be subdivided to yield a chunk of the right size.
Because most objects are smaller than `BigSize` and most allocation
requests can be satisfied by recycling deallocated objects of the
desired size, most allocations execute only the first four lines of the
routine.

**attemptToAllocateChunkInCurrentSegment: size**

\| objectPointer predecessor next availableSize excessSize newPointer \|

objectPointer := nil.

size \< BigSize

ifTrue: \[objectPointer := self removeFromFreeChunkList: size\].

objectPointer notNil

ifTrue: \[\^objectPointer\]. *\"small chunk of exact size handy so use
it\"*

predecessor := NonPointer. *\"remember predecessor of chunk under
consideration\"*

objectPointer := self headOfFreeChunkList: BigSize

    inSegment: currentSegment.

*\"the search loop stops when the end of the linked list is
encountered\"*

\[objectPointer = NonPointer\] whileFalse:

\[availableSize := self sizeBitsOf: objectPointer.

availableSize = size

ifTrue: *\"exact fit \-- remove from free chunk list and return\"*

\[next := self classBitsOf: objectPointer. *\"the link to the next
chunk\"*

predecessor = NonPointer

ifTrue: *\"it was the head of the list; make the next item the head\"*

\[self headOfFreeChunkList: BigSize

inSegment: currentSegment

put: next\]

ifFalse: *\"it was between two chunks; link them together\"*

\[self classBitsOf: predecessor

put: next\].

\^objectPointer\].

*\"this chunk was either too big or too small; inspect the amount of
variance\"*

excessSize := availableSize - size.

excessSize \>= HeaderSize

ifTrue: *\"can be broken into two usable parts: return the second
part\"*

\[*\"obtain an object table entry for the second part\"*

newPointer := self obtainPointer: size

location: (self locationBitsOf: objectPointer) + excessSize.

newPointer isNil ifTrue: \[\^nil\].

*\"correct the size of the first part (which remains on the free
list)\"*

self sizeBitsOf: objectPointer put: excessSize.

\^newPointer\]

ifFalse: *\"not big enough to use; try the next chunk on the list\"*

\[predecessor := objectPointer.

objectPointer := self classBitsOf: objectPointer\]\].

\^nil *\"the end of the linked list was reached and no fit was found\"*

The subroutine `obtainPointer:location:` used by the above routine
obtains a free object table entry, zeroes its free entry bit as well as
the rest of the first word of the entry, points the entry at the
specified location, and sets the size field of the header to the
specified size.

**obtainPointer: size location: location**

\| objectPointer \|

objectPointer := self removeFromFreePointerList.

objectPointer isNil ifTrue: \[\^nil\].

self ot: objectPointer put: 0.

self segmentBitsOf: objectPointer put. currentSegment.

self locationBitsOf: objectPointer put: location.

self sizeBitsOf: objectPointer put: size.

\^objectPointer

[]{#ADeallocationAlgorithm30}***A Deallocation Algorithm***\

------------------------------------------------------------------------

It is much simpler to deallocate an object than to allocate one. The
chunk is recycled on a free-chunk list. The following routine expects
the count field to have been reset to zero by a higher-level routine.

**deallocate: objectPointer** *\"\*\*Preliminary Version\*\*\"*

\| space \|

space := self spaceOccupiedBy: objectPointer.

self toFreeChunkList: (space min: BigSize)

add: objectPointer

Note that this routine computes the space occupied by the object using
`spaceOccupiedBy:` instead of `sizeBitsOf:`. The reason will become
clear later in the chapter when `spaceOccupiedBy:` is redefined.

[]{#ACompactionAlgorithm30}***A Compaction Algorithm***\

------------------------------------------------------------------------

The `compactCurrentSegment` routine invoked above by
`attemptToAllocateChunk:` sweeps through a heap segment, massing all
allocated objects together and updating their object table entries. For
the benefit of subsequent allocation, it also links the object table
entries reclaimed from the free chunk lists onto the free pointer list
and creates a single free chunk from the unused portion of the heap
segment. The algorithm for `compactCurrentSegment` will be presented
shortly, after some preparatory discussion.\
    After a heap segment is compacted a number of times, relatively
permanent objects sift to the bottom of the segment and most allocation
and deallocation activity occurs nearer to the top. The segment consists
of a densely packed region of allocated chunks, followed by a region of
both allocated and free chunks. During compaction, chunks in the densely
packed region never move, because there is no space beneath them to
eliminate. Therefore, the compacter expends effort only on chunks above
the first free chunk, whose location is referred to as `lowWaterMark`.\
    The `abandonFreeChunksInSegment:` routine computes `lowWaterMark`.
It also finds all deallocated chunks, recycles their object table
entries onto the list of free pointers using the subroutine
`releasePointer:`, and changes their class fields to the distinguished
value `NonPointer`. During the subsequent sweep, when the compacter
encounters objects so marked it can recognize them as deallocated
chunks.

**abandonFreeChunksInSegment: segment**

\| lowWaterMark objectPointer nextPointer \|

lowWaterMark := HeapSpaceStop. *\"first assume that no chunk is free\"*

HeaderSize to: BigSize do: *\"for each free-chunk list\"*

\[:size \|

objectPointer := self headOfFreeChunkList: size

inSegment: segment.

\[objectPointer = NonPointer\] whileFalse:

\[lowWaterMark := lowWaterMark min: (self locationBitsOf:
objectPointer).

nextPointer := self classBitsOf: objectPointer. *\"link to next free
chunk\"*

self classBitsOf: objectPointer put: NonPointer. *\"distinguish for
sweep\"*

self releasePointer: objectPointer. *\"add entry to free-pointer list\"*

objectPointer := nextPointer\].

self resetFreeChunkList: size inSegment: segment\].

\^lowWaterMark

**releasePointer: objectPointer**

self freeBitOf: objectPointer put: 1.

self toFreePointerListAdd: objectPointer

A heap segment is compacted by sweeping through it from bottom to top.
Each allocated object is moved as far down in the segment as possible
without overwriting other allocated objects. For each object moved, the
corresponding object table entry is found and its location field is
updated to point to the new location of the object.\
    It is by no means trivial to find the object table entry of an
object encountered during a sweep of the heap segment. The
representation of the object in the heap does not include a pointer back
to the object table entry. To avoid the cost of such a backpointer for
every object or making the compacter search the object table after every
object is moved, a trick called \"reversing pointers\" is employed.
During compaction, instead of the usual arrangement in which the object
table entry points to the header in the heap, the header points
temporarily to the object table entry.\
    Pointers are reversed before starting to sweep through a heap
segment. The object table is scanned to find every in-use entry whose
segment field refers to the segment being compacted and whose location
field is above `lowWaterMark`. Each such entry points to the header of
an object that is to be moved ([Figure 30.8a](#Figure_30.8)). The
pointer is then reversed, i.e., the object\'s own object pointer is
stored in the first word of its header. This causes the header to point
to the object table entry. By doing this, the size field of the header
is overwritten. To prevent losing the size, it is saved in the second
word of the object table entry ([Figure 30.8b](#Figure_30.8)). By doing
that, the location field is overwritten, but that is of no consequence,
because the compacter recomputes the object\'s heap location after the
move.\
[]{#Figure_30.8}

  -----------------------------------------------
  ![](figure30_8.gif){height="304" width="420"}
  **Figure 30.8**
  -----------------------------------------------

**reverseHeapPointersAbove: lowWaterMark**

\| size \|

0 to: ObjectTableSize - 2 by: 2 do:

\[:objectPointer \|

(self freeBitOf: objectPointer) = 0

ifTrue: *\"the Object Table entry is in use\"*

\[(self segmentBitsOf: objectPointer) = currentSegment

ifTrue: *\"the object is in this segment\"*

\[(self locationBitsOf: objectPointer) \< lowWaterMark

ifFalse: *\"the object will be swept\"*

\[size := self sizeBitsOf: objectPointer. *\"rescue the size\"*

self sizeBitsOf: objectPointer

put: objectPointer. *\"reverse the pointer\"*

self locationBitsOf: objectPointer

put: size *\"save the size\"* \]\]\]\]

After all preparations for compaction are complete, the current heap
segment is swept using the `sweepCurrentSegmentFrom`: routine. It
maintains two pointers into the segment, `si` (source index) and `di`
(destination index). The pointer `si` points to the header of an object
currently being considered for retention or elimination. The pointer
`di` points to the location where that object will be moved if retained.

**sweepCurrentSegmentFrom: lowWaterMark**

\| si di objectPointer size space \|

si := di := lowWaterMark.

\[si \< HeapSpaceStop\] whileTrue: *\"for each object, si\"*

\[(wordMemory segment: currentSegment word: si + 1) = NonPointer

ifTrue: *\"unallocated, so skip it\"*

\[size := wordMemory segment: currentSegment word: si.

si := si + size\]

ifFalse: *\"allocated, so keep it, but move it to compact storage\"*

\[objectPointer := wordMemory segment: currentSegment word: si.

size := self locationBitsOf: objectPointer. *\"the reversed size\"*

self locationBitsOf: objectPointer

put: di. *\"point object table at new location\"*

self sizeBitsOf: objectPointer

put: size. *\"restore the size to its proper place\"*

si := si + 1. *\"skip the size\"*

di := di + 1. *\"skip the size\"*

2 to: (self spaceOccupiedBy: objectPointer) do:

*\"move the rest of the object\"*

\[:i \|

wordMemory segment: currentSegment

word: di

put: (wordMemory segment: currentSegment

word: si).

si := si + 1.

di := di + 1\]\]\].

\^di

Note that while pointers are reversed, it is impossible to access the
heap memory of an object from its object table entry. Therefore the
Smalltalk interpreter cannot run during compaction.

The `compactCurrentSegment` routine invokes the above routines in the
proper order and then creates the single free chunk at the top of the
heap segment.

**compactCurrentSegment**

\| lowWaterMark bigSpace \|

lowWaterMark := self abandonFreeChunksInSegment: currentSegment.

lowWaterMark \< HeapSpaceStop

ifTrue: \[self reverseHeapPointersAbove: lowWaterMark.

bigSpace := self sweepCurrentSegmentFrom: lowWaterMark.

self deallocate: (self obtainPointer: (HeapSpaceStop + 1 - bigSpace)

location: bigSpace)\]

If there are no free chunks within the segment when this routine is
invoked, then it does not move any objects.

------------------------------------------------------------------------

[]{#GarbageCollection30}**Garbage Collection**

------------------------------------------------------------------------

In Smalltalk, a new object is allocated explicitly (e.g., when the
message `new` is sent to a class) but there is no explicit language
construct that causes an object to be deallocated. Such a construct
would be unsafe, because it could be used to deallocate an object even
though \"dangling\" references to it still existed in other objects. An
environment containing dangling references would be inconsistent and
would be likely to exhibit unintended behavior and to suffer
unrecoverable errors.\
    Most noninteractive programming systems require explicit
deallocation. The burden of avoiding dangling references is placed on
the programmer. If a dangling reference arises, the programmer is
expected to find the bug that created it, fix that bug, and restart the
program. In an interactive environment like Smalltalk (as well as most
LISP and APL systems), to require a restart because of a common bug
would be unacceptable, since it could require the user to redo a
potentially large amount of work.\
    Because there is no explicit deallocation in Smalltalk, the memory
manager must identify objects that have become *inaccessible* and
deallocate them automatically. This task is traditionally known as
*garbage collection*. As compared with explicit deallocation, garbage
collection entails a large performance penalty. The penalty is incurred
because the computer must manage deallocation at execution time instead
of relying on the programmer to have done so during coding time.
However, the cost is well worth the reliability it adds to an
interactive system.\
    There are two traditional approaches to identifying inaccessible
objects in an object memory: *marking* and *reference counting*. A
marking garbage collector performs an exhaustive search of memory for
accessible objects and marks them all. Then it scans memory in search of
objects that are unmarked and thus inaccessible and deallocates them. A
reference-counting garbage collector maintains a count of how many
references there are to each object from other objects. When the count
of references to some object reaches zero, that object is known to be
inaccessible, and the space it occupies can be reclaimed.\
    Reference-counting systems do not deal properly with so-called
\"cyclic structures.\" Such a structure is said to occur when an object
references itself directly or when an object references itself
indirectly via other objects that reference it. In a reference-counting
system, when a cyclic structure becomes inaccessible to the program, it
will have nonzero reference counts due to the intrastructure references.
Therefore the memory manager doesn\'t recognize that the structure
should be deallocated, and the objects that constitute the structure are
not deallocated. These inaccessible objects waste space; but, unlike
dangling references, they do not cause inconsistency in the
environment.\
    Both reference counting and marking involve performance penalties on
conventional computers. In a reference-counting system, the frequently
performed operation of storing a reference to an object involves
overhead for reference-count maintenance, so programs run significantly
more slowly. In a marking garbage collector, an extensive search of
memory must be performed whenever space is entirely depleted. Therefore,
program execution is subject to relatively lengthy interruptions that
can be quite annoying in an interactive system. Both approaches incur
space overhead. In a reference-counting system, space must be provided
to store reference counts. In a marking system, extra space must be
allotted in the heap to allow garbage to accumulate between collections.
Otherwise, collections occur too frequently.\
    The approach to garbage collection that should be used in a
particular implementation of Smalltalk depends in part on the capacity
of the hardware. If a relatively small amount of memory (e.g., 100
kilobytes) is available, a reference counting system is intolerable,
because it can waste precious space by leaving inaccessible cyclic
structures allocated. On the other hand, a marking collector is quite
acceptable in these circumstances, in spite of the interruption that
occurs when it is invoked, because when memory is small, the duration of
the interruption can be so brief as to be imperceptible. If an abundant
supply of memory (e.g., two megabytes) is available, the time it takes
to mark all accessible objects can be so long as to be intolerable. On
the other hand, there is enough space available that a moderate number
of inaccessible objects can be tolerated.\
    The contrast between the two approaches is accentuated in a large
virtual-memory system. Marking is even more costly because so much time
is spent in random accesses to secondary memory. Reference counting is
even less costly because unreclaimed cyclic structures simply migrate to
secondary memory where wasted space is less bothersome. When memory is
abundant, a reference-counting garbage collector is appropriate.
However, Smalltalk programmers should take precautions to avoid the
accumulation of an excessive number of inaccessible cyclic structures,
otherwise even a large memory will be depleted. To break a cyclic
structure before it becomes inaccessible, the program can change any
pointer that participates in the cycle to `nil`.\
    The two approaches to garbage collection can be combined. References
can be counted during normal operation and marking collections performed
periodically to reclaim inaccessible cyclic structures. A combined
approach is suitable for all but the smallest real-memory
implementations. If a small-to-medium-size memory is available, a
marking collection can be performed whenever compaction fails to recover
enough space. If an abundant memory is available, marking collections
can be performed nightly or at other convenient intervals.

[]{#ASimpleReferenceCountingCollector30}***A Simple Reference-counting
Collector***\

------------------------------------------------------------------------

In the reference-counting collector described in this chapter, the
reference count of an object is recorded in the count field of its
object table entry. If an object pointer is an immediate integer, it is
its own only reference, so its reference count is not recorded
explicitly. Reference counts are updated during store operations. When
an object pointer referencing object ***P*** is stored into a location
that formerly contained an object pointer referencing object ***Q***,
the count field of ***P*** is incremented and the count field of ***Q***
is decremented. Because the count field of an object table entry has
only eight bits, it is possible for an incremented count to overflow. To
facilitate overflow detection on most computers, the high order bit of
the count field serves as an overflow bit. Once the count field reaches
128, it remains at that value and it will not increase or decrease. The
algorithm for incrementing a reference count is

**countUp: objectPointer**

\| count \|

(self isIntegerObject: objectPointer)

ifFalse: \[count := (self countBitsOf: objectPointer) + 1.

count \< 129 ifTrue: \[self countBitsOf: objectPointer put: count\]\].

\^objectPointer

If the decremented reference count of an object reaches zero, then that
object is deallocated. Before doing so, the count field of every object
referenced from that object is decremented, because once the object is
deallocated it will no longer reference those other objects. Note that
this procedure recurs if any of the latter counts reach zero. A
recursive procedure that can traverse the original object plus all the
objects it references is expressed below as the routine
`forAllObjectsAccessibleFrom:suchThat:do:`. This routine takes two
procedural arguments represented by blocks, a `predicate` that
decrements a count and tests for zero and an `action` that deallocates
an object. Between evaluating the `predicate` and the `action`, the
procedure\'s subroutine,
`forAllOtherObjectsAccessibleFrom:suchThat:do:`, recursively processes
every pointer in the object. The procedure is allowed to alter the count
as a side effect, so the `action` argument must restore the count to
zero in preparation for deallocation.

**countDown: rootObjectPointer**

\| count \|

(self isIntegerObject: rootObjectPointer)

ifTrue: \[\^rootObjectPointer\]

ifFalse: *\"this is a pointer, so decrement its reference count\"*

\[\^self forAllObjectsAccessibleFrom: rootObjectPointer

         suchThat: *\"the predicate decrements the count and tests for
zero\"*

\[:objectPointer \|

count := (self countBitsOf: objectPointer) - 1.

count \< 127

ifTrue: \[self countBitsOf: objectPointer

put: count\].

count = 0\]

          do: *\"the action zeroes the count and deallocates the
object\"*

\[:objectPointer \|

self countBitsOf: objectPointer put: 0.

self deallocate: objectPointer\]\]

The traversal routine shown below first tests the predicate on the
supplied object. It then invokes a subroutine that (1) recursively
processes all objects referenced from within the supplied object that
satisfy `predicate`, and (2) performs `action` on the supplied object.

**forAllObjectsAccessibleFrom: objectPointer suchThat: predicate do:
action**

(predicate value: objectPointer)

ifTrue: \[\^self forAllOtherObjectsAccessibleFrom: objectPointer

  suchThat: predicate

  do: action\]

**forAllOtherObjectsAccessibleFrom: objectPointer suchThat: predicate
do: action**

\| next \|

1 to: (self lastPointerOf: objectPointer) - 1 do:

\[:offset \|

next := self heapChunkOf: objectPointer word: offset.

((self isIntegerObject: next) == false and: \[predicate value: next\])

ifTrue: *\"it\'s a non-immediate object and it should be processed\"*

\[self forAllOtherObjectsAccessibleFrom: next

        suchThat: predicate

        do: action\]\].

*\"all pointers have been followed; now perform the action\"*

action value: objectPointer.

\^objectPointer

[]{#ASpaceEfficientReferenceCountingCollector30}***A Space-efficient
Reference-counting Collector***\

------------------------------------------------------------------------

The traversal algorithm outlined above is recursive and, therefore, must
use a stack in its execution. To guard against stack overflow, the depth
of the stack must be greater than the longest chain of pointers in
memory. This requirement is difficult to satisfy when memory space is
limited. To guarantee that enough space is available, the pointer chain
itself can be used as the stack. If object ***A*** references object
***B*** from ***A***\'s ***i^th^*** field, and object ***B*** references
object ***C*** from ***B***\'s ***j^th^*** field, and object ***C***
references another object from ***C\'***s ***k^th^*** field, and so on,
the pointer chain can be represented as
***A.i**-\>**B.j**-\>**C.k**-\>**\...*** ([Figure 30.9a](#Figure_30.9)).
To use the pointer chain as a stack for the recursion of the traversal
algorithm, the chain is temporarily reversed to
***\...**-\>**C.k**-\>**B.j**-\>**A.i*** so that each field in the chain
points to its predecessor instead of to its successor ([Figure
30.9b](#Figure_30.9)).\
    Each step of the traversal algorithm\'s recursion must be completed
by \"popping the stack.\" After processing any object in the chain
(e.g., ***C***), its predecessor (e.g., ***B***) is found by following
the reversed pointer chain. The algorithm also needs to know which field
of the predecessor was being worked on. To maintain this information,
the algorithm must be changed at the earlier stage where it left ***B***
to process ***C***. At that stage, the index of the field, ***j***, is
copied into the count field of the object table entry of ***B***. The
count can be overwritten because the object is being deallocated. But if
the size of ***B*** exceeds 255 words, then the count field will not be
large enough to store every field index. Instead, the allocator is
revised to over-allocate by one word any object that is HugeSize (256)
words or more and to reserve that extra word for use by the traversal
algorithm to store *offset*.\
[]{#Figure_30.9}

  -----------------------------------------------
  ![](figure30_9.gif){height="298" width="496"}
  **Figure 30.9**
  -----------------------------------------------

To accommodate over-allocation, the allocation routine is revised to
accept an additional argument, `extraWord`, that is either 0 or 1. It is
also necessary for the allocator to increment the reference count of the
new object\'s class before storing the class into the header of the new
object. (In fact, this must be accomplished even earlier, before calling
`allocateChunk:`, to assure that the class is not deallocated
accidentally by some side effect of that subroutine.)

**allocate: size extra: extraWord class: classPointer**
*\"\*\*Preliminary Version\*\*\"*

\| objectPointer \|

self countUp: classPointer. *\"increment the reference count of the
class\"*

objectPointer := self allocateChunk: size + extraWord. *\"allocate
enough\"*

self classBitsOf: objectPointer put: classPointer.

HeaderSize to: size - 1 do:

\[:i \| self heapChunkOf: objectPointer word: i put: NilPointer\].

*\"the next statement to correct the SIZE need only be executed if
extraWord \> 0\"*

self sizeBitsOf: objectPointer put: size.

\^objectPointer

The actual heap space occupied by an object with at least `HugeSize`
fields is one greater than that stated in its size field, because of the
extra word allocated. Therefore, the `spaceOccupiedBy:` routine must be
changed to account for the difference.

**spaceOccupiedBy: objectPointer** *\"\*\*Preliminary Version\*\*\"*

\| size \|

size := self sizeBitsOf: objectPointer.

size \< HugeSize

ifTrue: \[\^size\]

ifFalse: \[\^size + 1\]

The deallocation algorithm must also be revised because deallocated
objects have no provision for an extra word not counted in the size
field.

**deallocate: objectPointer**

\| space \|

space := self spaceOccupiedBy: objectPointer.

self sizeBitsOf: objectPointer put: space.

self toFreeChunkList: (space min: BigSize) add: objectPointer

The following routine implements the space-efficient traversal
algorithm, with ***A***, ***B***, and ***C*** of the above example
represented by the variables `prior`, `current`, and `next`. To simplify
the loop test, the method scans the fields of each chunk in reverse
order. Thus the class field is processed last.\
    Note that the last statement of the method restores the pointer
chain to get ***B.j*** again pointing to ***C*** instead of to ***A***.
It is easy to do so when returning to ***B*** from processing ***C***,
because object pointer of ***C*** can simply be stored in the
***j^th^*** field of ***B***. One might think that step unnecessary,
because ***B*** is being deallocated. However, the same traversal
algorithm can be used by a marking collector in which ***B*** is not
being deallocated.

**forAllOtherObjectsAccessibleFrom: objectPointer suchThat: predicate
do: action**

\| prior current offset size next \|

*\"compute prior, current, offset, and size to begin processing
objectPointer\"*

prior := NonPointer.

current := objectPointer.

offset := size := self lastPointerOf: objectPointer.

\[true\] whileTrue: *\"for all pointers in all objects traversed\"*

     \[(offset := offset - 1) \> 0 *\"decrement the field index\"*

ifTrue: *\"the class field hasn\'t been passed yet\"*

     \[next := self heapChunkOf: current word: offset. *\"one of the
pointers\"*

     ((self isIntegerObject: next) == false and: \[predicate value:
next\])

ifTrue: *\"it\'s a non-immediate object and it should be processed\"*

\[*\"reverse the pointer chain\"*

self heapChunkOf: current word: offset put: prior.

*\"save the offset either in the count field or in the extra word\"*

size \< HugeSize

ifTrue: \[self countBitsOf: current put: offset\]

ifFalse: \[self heapChunkOf: current word: size + 1 put: offset\].

*\"compute prior, current, offset, and size to begin processing next\"*

prior := current.

current := next.

offset := size := self lastPointerOf: current\]\]

ifFalse:

\[*\" all pointers have been followed; now perform the action\"*

action value: current.

*\"did we get here from another object?\"*

prior = NonPointer

ifTrue: *\"this was the root object, so we are done\"*

\[\^objectPointer\].

*\"restore next, current, and size to resume processing prior\"*

next := current.

current := prior.

size := self lastPointerOf: current.

*\"restore offset either from the count field or from the extra word\"*

size \< HugeSize

ifTrue: \[offset := self countBitsOf: current\]

ifFalse: \[offset := self heapChunkOf: current word: size + 1\].

*\"restore prior from the reversed pointer chain\"*

prior := self heapChunkOf: current word: offset.

*\"restore (unreverse) the pointer chain\"*

self heapChunkOf: current word: offset put: next\]\]

The machine-language implementation can deal with the procedural
arguments either by passing a pair of subroutine addresses to be called
indirectly or by expanding the subroutines in line. If the hardware has
enough registers, it is possible to keep the variables `next`,
`current`, `prior`, `size`, and `offset` in registers for additional
speed of execution.

[]{#AMarkingCollector30}***A Marking Collector***\

------------------------------------------------------------------------

The job of the marking garbage collector is to mark all accessible
objects so that the remaining inaccessible objects can be identified and
added to the lists of free chunks. Accessible objects can be found most
easily by a recursive search from the \"roots of the world,\" namely,
the interpreter\'s stacks and the table of global variables (the
`Dictionary` named `Smalltalk`).\
    The following algorithm is performed on each root object. In the
object table entry of the object, set the count field to 1 to mean
\"marked.\" Apply the algorithm of this paragraph to each unmarked
object referenced by the object.\
    Note that the above *marking algorithm* is inherently recursive. In
its implementation, the same traversal routine used for reference
counting can be used, in either the simple or the space-efficient
version. Before marking begins, the count fields of all objects are
reset to 0 to mean \"unmarked.\" After marking ends, all unmarked
objects are deallocated and the reference counts of all marked objects
are recomputed. The routine that performs all the necessary steps is
called `reclaimInaccessibleObjects`.

**reclaimInaccessibleObjects**

self zeroReferenceCounts.

self markAccessibleObjects.

self rectifyCountsAndDeallocateGarbage

The subroutine that sets the count fields of all objects to 0 is called
`zeroReferenceCounts`. It is superfluous to zero the count field of a
free chunk or of a free entry. Nevertheless, the following version
zeroes the count field of every entry, because on most computers, it
takes less time to zero the first byte of an entry than it takes to test
the status of that entry.

**zeroReferenceCounts**

0 to: ObjectTableSize - 2 by: 2 do:

\[:objectPointer \|

self countBitsOf: objectPointer put: 0\]

The subroutine `markAccessibleObjects` invokes the marking algorithm
`markObjectsAccessibleFrom:` for every object in the list
`rootObjectPointers`. Typically, the list `rootObjectPointers` includes
the object pointer of the current process and the object pointer of the
global variable dictionary, from which all other accessible objects are
referenced directly or indirectly.

**markAccessibleObjects**

rootObjectPointers do:

\[:rootObjectPointer \|

self markObjectsAccessibleFrom: rootObjectPointer\]

The marking algorithm `markObjectsAccessibleFrom:` calls the same
traversal routine as the reference-counting collector did. Its
`predicate` succeeds for unmarked objects and it marks them with a count
of 1 as a side effect. Its `action` restores the count field to 1
because the space-efficient version of the traversal routine could have
changed that field to any nonzero value as a side effect.

**markObjectsAccessibleFrom: rootObjectPointer**

\| unmarked \|

\^self forAllObjectsAccessibleFrom: rootObjectPointer

         suchThat: *\"the predicate tests for an unmarked object and
marks it\"*

\[:objectPointer \|

unmarked := (self countBitsOf: objectPointer) = 0.

unmarked ifTrue: \[self countBitsOf: objectPointer put: 1\].

unmarked\]

         do: *\"the action restores the mark to count = 1\"*

\[:objectPointer \|

self countBitsOf: objectPointer put: 1\]

After the marking algorithm has been executed, every non-free object
table entry is examined using the subroutine
`rectifyCountsAndDeallocateGarbage`. If the entry is unmarked, then the
entry and its heap chunk are added to the appropriate free lists. If the
entry is marked, then the count is decremented by one to unmark it, and
the counts of all objects that it references directly are incremented.
Note that when a marked object is processed, its count may exceed 1
because objects previously processed may have referenced it. That is why
it is unmarked by subtraction instead of by setting its count to 0.\
    During the examination of object table entries, chunks that were
already free before the marking collection began will be encountered.
The count field of an already-free chunk is zero just like an unmarked
object, so it will be added to a free-chunk list. Doing so would cause a
problem if the chunk were already on a free-chunk list. Therefore before
the scan begins, all heads of free-chunk lists are reset.\
    As a final step, the reference count of each root object is
incremented to assure that it is not deallocated accidentally.

**rectifyCountsAndDeallocateGarbage**

\| count \|

*\"reset heads of free-chunk lists\"*

FirstHeapSegment to: LastHeapSegment do: *\"for every segment\"*

\[:segment \|

HeaderSize to: BigSize do: *\"for every free chunk list\"*

\[:size \| *\"reset the list head\"*

self resetFreeChunkList: size inSegment: segment\]\].

*\"rectify counts, and deallocate garbage\"*

0 to: ObjectTableSize - 2 by: 2 do: *\"for every object table entry\"*

\[:objectPointer \|

(self freeBitOf: objectPointer) = 0 ifTrue: *\"if it is not a free
entry\"*

\[(count := self countBitsOf: objectPointer) = 0

ifTrue: *\"it is unmarked, so deallocate it\"*

\[self deallocate: objectPointer\]

ifFalse: *\"it is marked, so rectify reference counts\"*

\[count \< 128 ifTrue: *\"subtract 1 to compensate for the mark\"*

\[self countBitsOf: objectPointer put: count - 1\].

1 to: (self lastPointerOf: objectPointer) - 1 do:

\[:offset \| *\"increment the reference count of each pointer\"*

self countUp: (self heapChunkOf: objectPointer word: offset)\]\]\]\].

*\"be sure the root objects don\'t disappear\"*

rootObjectPointers do:

\[:rootObjectPointer \| self countUp: rootObjectPointer\].

self countBitsOf: NilPointer put: 128

The `allocateChunk:` routine can now be revised so that it attempts a
marking collection if compaction of all segments has failed to yield
enough space to satisfy an allocation request.

**allocateChunk: size**

\| objectPointer \|

objectPointer := self attemptToAllocateChunk: size.

objectPointer isNil ifFalse: \[\^objectPointer\].

self reclaimInaccessibleObjects. *\"garbage collect and try again\"*

objectPointer := self attemptToAllocateChunk: size.

objectPointer isNil ifFalse: \[\^objectPointer\].

self outOfMemoryError *\"give up\"*

------------------------------------------------------------------------

[]{#NonpointerObjects30}**Nonpointer Objects**

------------------------------------------------------------------------

The object format presented in this chapter is not particularly space
efficient, but since its uniformity makes the system software small and
simple, the inefficiency can generally be forgiven. There are two
classes of object for which the inefficiency is intolerable, namely,
character strings and bytecoded methods. There are usually many strings
and methods in memory, and when stored one character or one bytecode per
word, they are quite wasteful of space.\
    To store such objects more efficiently, an alternate memory format
is used in which the data part of an object contains 8-bit or 16-bit
values that are interpreted as unsigned integers rather than as object
pointers. Such objects are distinguished by the setting of the
pointer-fields bit of the object table entry: when that bit is 1, the
data consist of object pointers; when that bit is 0, the data consist of
positive 8- or 16-bit integers. When there are an odd number of bytes of
data in a nonpointer object, the final byte of the last word is 0 (a
slight waste of space), and the odd-length bit of the object table
entry, which is normally 0, is set to 1. To support nonpointer objects,
the allocator needs two additional parameters, `pointerBit` and
`oddBit`. In the case of a nonpointer object (`pointerBit` `=` `0`), the
default initial value of the elements is `0` instead of `nil`. The final
version of the allocation routine is shown below.

**allocate: size odd: oddBit pointer: pointerBit extra: extraWord class:
classPointer**

\| objectPointer default \|

self countUp: classPointer.

objectPointer := self allocateChunk: size + extraWord.

self oddBitOf: objectPointer put: oddBit.

self pointerBitOf: objectPointer put: pointerBit.

self classBitsOf: objectPointer put: classPointer.

default := pointerBit = 0 ifTrue: \[0\] ifFalse: \[NilPointer\].

HeaderSize to: size - 1 do:

\[:i \| self heapChunkOf: objectPointer word: i put: default\].

self sizeBitsOf: objectPointer put: size.

\^objectPointer

The garbage-collecting traversal routines need only process the class
field of each nonpointer object, because the data contain no pointers.
To make this happen, the routine `lastPointerOf:` is changed as follows:

**lastPointerOf: objectPointer** *\"\*\*Preliminary Version\*\*\"*

(self pointerBitOf: objectPointer) = 0

ifTrue: \[\^HeaderSize\]

ifFalse: \[\^self sizeBitsOf: objectPointer\]

The value of `lastPointerOf:` is never as large as 256 for a nonpointer
object, so a nonpointer object never needs to be over-allocated.
Therefore, `spaceOccupiedBy:` is revised again as follows:

**spaceOccupiedBy: objectPointer**

\| size \|

size := self sizeBitsOf: objectPointer.

(size \< HugeSize or: \[(self pointerBitOf: objectPointer) = 0\])

ifTrue: \[\^size\]

ifFalse: \[\^size + 1\]

[]{#CompiledMethods30}***CompiledMethods***\

------------------------------------------------------------------------

A `CompiledMethod` is an anomaly for the memory manager because its data
are a mixture of 16-bit pointers and 8-bit unsigned integers. The only
change needed to support` CompiledMethods` is to add to `lastPointerOf:`
a computation similar to that in the bytecode interpreter\'s routine
`bytecodeIndexOf:`. `MethodClass` is the object table index of
`CompiledMethod`.

**lastPointerOf: objectPointer**

\| methodHeader \|

(self pointerBitOf: objectPointer) = 0

ifTrue:

\[(self classBitsOf: objectPointer) = MethodClass

ifTrue: \[methodHeader := self heapChunkOf: objectPointer

word: HeaderSize.

\^HeaderSize + 1 + ((methodHeader bitAnd: 126) bitShift: -1)\]

ifFalse: \[\^HeaderSize\]\]

ifFalse:

\[\^self sizeBitsOf: objectPointer\]

------------------------------------------------------------------------

[]{#InterfaceToTheBytecodeInterpreter30}**Interface to the Bytecode
Interpreter**

------------------------------------------------------------------------

The final step in the implementation of the object memory is to provide
the interface routines required by the interpreter. Note that
`fetchClassOf:` `objectPointer` returns `IntegerClass` (the object table
index of `SmallInteger`) if its argument is an immediate integer.

*object pointer access*

**fetchPointer: fieldIndex ofObject: objectPointer**

\^self heapChunkOf: objectPointer word: HeaderSize + fieldIndex

**storePointer: fieldIndex ofObject: objectPointer withValue:
valuePointer**

\| chunkIndex \|

chunkIndex := HeaderSize + fieldIndex.

self countUp: valuePointer.

self countDown: (self heapChunkOf: objectPointer word: chunkIndex).

\^self heapChunkOf: objectPointer word: chunkIndex put: valuePointer

*word access*

**fetchWord: wordIndex ofObject: objectPointer**

\^self heapChunkOf: objectPointer word: HeaderSize + wordIndex

**storeWord: wordIndex ofObject: objectPointer withValue: valueWord**

\^self heapChunkOf: objectPointer word: HeaderSize + wordIndex put:
valueWord

*byte access*

**fetchByte: byteIndex ofObject: objectPointer**

\^self heapChunkOf: objectPointer byte: (HeaderSize \* 2 + byteIndex)

**storeByte: byteIndex ofObject: objectPointer withValue: valueByte**

\^self heapChunkOf: objectPointer byte: (HeaderSize \* 2 + byteIndex)
put: valueByte

*reference counting*

**increaseReferencesTo: objectPointer**

self countUp: objectPointer

**decreaseReferencesTo: objectPointer**

self countDown: objectPointer

*class pointer access*

**fetchClassOf: objectPointer**

(self isIntegerObject: objectPointer)

ifTrue: \[\^IntegerClass\]

ifFalse: \[\^self classBitsOf: objectPointer\]

*length access*

**fetchWordLengthOf: objectPointer**

\^(self sizeBitsOf: objectPointer) - HeaderSize

**fetchByteLengthOf: objectPointer**

\^(self fetchWordLengthOf: objectPointer) \* 2 - (self oddBitOf:
objectPointer)

*object creation*

**instantiateClass: classPointer withPointers: length**

\| size extra \|

size := HeaderSize + length.

extra := size \< HugeSize ifTrue: \[0\] ifFalse: \[1\].

\^self allocate: size odd: 0 pointer: 1 extra: extra class: classPointer

**instantiateClass: classPointer withWords: length**

\| size \|

size := HeaderSize + length.

\^self allocate: size odd: 0 pointer: 0 extra: 0 class: classPointer

**instantiateClass: classPointer withBytes: length**

\| size \|

size := HeaderSize + ((length + 1) / 2).

\^self allocate: size odd: length \\\\ 2 pointer: 0 extra: 0 class:
classPointer

*instance enumeration*

**initialInstanceOf: classPointer**

0 to: ObjectTableSize - 2 by: 2 do:

\[:pointer \|

(self freeBitOf: pointer) = 0

ifTrue: \[(self fetchClassOf: pointer) = classPointer

ifTrue: \[\^pointer\]\]\].

\^NilPointer

**instanceAfter: objectPointer**

\| classPointer \|

classPointer := self fetchClassOf: objectPointer.

objectPointer to: ObjectTableSize - 2 by: 2 do:

\[:pointer \|

(self freeBitOf: pointer) = 0

ifTrue: \[(self fetchClassOf: pointer) = classPointer

ifTrue: \[\^pointer\]\]\].

\^NilPointer

*pointer swapping*

**swapPointersOf: firstPointer and: secondPointer**

\| firstSegment firstLocation firstPointer firstOdd \|

firstSegment := self segmentBitsOf: firstPointer.

firstLocation := self locationBitsOf: firstPointer.

firstPointer := self pointerBitOf: firstPointer.

firstOdd := self oddBitOf: firstPointer.

self segmentBitsOf: firstPointer put: (self segmentBitsOf:
secondPointer).

self locationBitsOf: firstPointer put: (self locationBitsOf: second
Pointer).

self pointerBitOf: firstPointer put: (self pointerBitOf: secondPointer).

self oddBitOf: firstPointer put: (self oddBitOf: secondPointer).

self segmentBitsOf: secondPointer put: firstSegment.

self locationBitsOf: secondPointer put: firstLocation.

self pointerBitOf: secondPointer put: firstPointer.

self oddBitOf: secondPointer put: firstOdd

*integer access*

**integerValueOf: objectPointer**

\^objectPointer / 2

**integerObjectOf: value**

\^(value bitShift: 1) + 1

**isIntegerObject: objectPointer**

\^(objectPointer bitAnd: 1) = 1

**isIntegerValue: valueWord**

\^valueWord \<= -16384 and: \[valueWord \> 16834\]

------------------------------------------------------------------------

\[[TOC](bluebook_imp_toc.html)\] \[[26](bluebook_chapter26.html)\]
\[[27](bluebook_chapter27.html)\] \[[28](bluebook_chapter28.html)\]
\[[29](bluebook_chapter29.html)\] \[[30](#top_of_30)\]
