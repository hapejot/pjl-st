[]{#Generator}

::: header
Next: [Getopt](Getopt.html#Getopt){accesskey="n" rel="next"}, Previous:
[Fraction](Fraction.html#Fraction){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Generator-1}

### 1.85 Generator {#generator .section}

[]{#index-Generator}

**Defined in namespace Smalltalk**\
**Superclass: Stream**\
**Category: Streams-Generators**

:   A Generator object provides a way to use blocks to define a Stream
    of many return values. The return values are computed one at a time,
    as needed, and hence need not even be finite.

    A generator block is converted to a Generator with \"Generator on:
    \[\...\]\". The Generator itself is passed to the block, and as soon
    as a message like #next, #peek, #atEnd or #peekFor: is sent to the
    generator, execution of the block starts/resumes and goes on until
    the generator's #yield: method is called: then the argument of
    #yield: will be the Generator's next element. If the block goes on
    to the end without calling #yield:, the Generator will produce no
    more elements and #atEnd will return true.

    You could achieve the effect of generators manually by writing your
    own class and storing all the local variables of the generator as
    instance variables. For example, returning a list of integers could
    be done by setting a variable to 0, and having the #next method
    increment it and return it. However, for a moderately complicated
    generator, writing a corresponding class would be much messier (and
    might lead to code duplication or inefficiency if you want to
    support #peek, #peekFor: and/or #atEnd): in general, providing a
    #do:-like interface is easy, but not providing a Stream-like one
    (think binary trees).

    The idea of generators comes from other programming languages, in
    particular this interface looks much like Scheme streams and Python
    generators. But Python in turn mutuated the idea for example from
    Icon, where the idea of generators is central. In Icon, every
    expression and function call behaves like a generator, and if a
    statement manages scalars, it automatically uses up all the results
    that the corresponding generator provides; on the other hand, Icon
    does not represent generators as first-class objects like Python and
    Smalltalk do.

  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Generator class-instance creation](Generator-class_002dinstance-creation.html#Generator-class_002dinstance-creation){accesskey="1"}:        (class)
  • [Generator-stream protocol](Generator_002dstream-protocol.html#Generator_002dstream-protocol){accesskey="2"}:                                (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------- ---- ------------

------------------------------------------------------------------------

::: header
Next: [Getopt](Getopt.html#Getopt){accesskey="n" rel="next"}, Previous:
[Fraction](Fraction.html#Fraction){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
