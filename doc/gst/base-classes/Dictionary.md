[]{#Dictionary}

::: header
Next:
[DirectedMessage](DirectedMessage.html#DirectedMessage){accesskey="n"
rel="next"}, Previous:
[DelayedAdaptor](DelayedAdaptor.html#DelayedAdaptor){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Dictionary-1}

### 1.64 Dictionary {#dictionary .section}

[]{#index-Dictionary}

**Defined in namespace Smalltalk**\
**Superclass: HashedCollection**\
**Category: Collections-Keyed**

:   I implement a dictionary, which is an object that is indexed by
    unique objects (typcially instances of Symbol), and associates
    another object with that index. I use the equality operator = to
    determine equality of indices.

    In almost all places where you would use a plain Dictionary, a
    LookupTable would be more efficient; see LookupTable's comment
    before you use it. I do have a couple of special features that are
    useful in certain special cases.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Dictionary class-instance creation](Dictionary-class_002dinstance-creation.html#Dictionary-class_002dinstance-creation){accesskey="1"}:                                        (class)
  • [Dictionary-accessing](Dictionary_002daccessing.html#Dictionary_002daccessing){accesskey="2"}:                                                                                  (instance)
  • [Dictionary-awful ST-80 compatibility hacks](Dictionary_002dawful-ST_002d80-compatibility-hacks.html#Dictionary_002dawful-ST_002d80-compatibility-hacks){accesskey="3"}:        (instance)
  • [Dictionary-compilation](Dictionary_002dcompilation.html#Dictionary_002dcompilation){accesskey="4"}:                                                                            (instance)
  • [Dictionary-dictionary enumerating](Dictionary_002ddictionary-enumerating.html#Dictionary_002ddictionary-enumerating){accesskey="5"}:                                           (instance)
  • [Dictionary-dictionary removing](Dictionary_002ddictionary-removing.html#Dictionary_002ddictionary-removing){accesskey="6"}:                                                    (instance)
  • [Dictionary-dictionary testing](Dictionary_002ddictionary-testing.html#Dictionary_002ddictionary-testing){accesskey="7"}:                                                       (instance)
  • [Dictionary-namespace protocol](Dictionary_002dnamespace-protocol.html#Dictionary_002dnamespace-protocol){accesskey="8"}:                                                       (instance)
  • [Dictionary-printing](Dictionary_002dprinting.html#Dictionary_002dprinting){accesskey="9"}:                                                                                     (instance)
  • [Dictionary-rehashing](Dictionary_002drehashing.html#Dictionary_002drehashing):                                                                                                 (instance)
  • [Dictionary-removing](Dictionary_002dremoving.html#Dictionary_002dremoving):                                                                                                    (instance)
  • [Dictionary-storing](Dictionary_002dstoring.html#Dictionary_002dstoring):                                                                                                       (instance)
  • [Dictionary-testing](Dictionary_002dtesting.html#Dictionary_002dtesting):                                                                                                       (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
