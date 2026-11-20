[]{#FileDescriptor}

::: header
Next: [FilePath](FilePath.html#FilePath){accesskey="n" rel="next"},
Previous: [File](File.html#File){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileDescriptor-1}

### 1.76 FileDescriptor {#filedescriptor .section}

[]{#index-FileDescriptor}

**Defined in namespace Smalltalk**\
**Superclass: Stream**\
**Category: Streams-Files**

:   My instances are what conventional programmers think of as files. My
    instance creation methods accept the name of a disk file (or any
    named file object, such as /dev/rmt0 on UNIX or MTA0: on VMS). In
    addition, they accept a virtual filesystem path like
    'configure.gz#ugz' which can be used to transparently extract or
    decompress files from archives, or do arbitrary processing on the
    files.

  ----------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [FileDescriptor class-initialization](FileDescriptor-class_002dinitialization.html#FileDescriptor-class_002dinitialization){accesskey="1"}:                    (class)
  • [FileDescriptor class-instance creation](FileDescriptor-class_002dinstance-creation.html#FileDescriptor-class_002dinstance-creation){accesskey="2"}:           (class)
  • [FileDescriptor class-still unclassified](FileDescriptor-class_002dstill-unclassified.html#FileDescriptor-class_002dstill-unclassified){accesskey="3"}:        (class)
  • [FileDescriptor-accessing](FileDescriptor_002daccessing.html#FileDescriptor_002daccessing){accesskey="4"}:                                                     (instance)
  • [FileDescriptor-basic](FileDescriptor_002dbasic.html#FileDescriptor_002dbasic){accesskey="5"}:                                                                 (instance)
  • [FileDescriptor-binary I/O](FileDescriptor_002dbinary-I_002fO.html#FileDescriptor_002dbinary-I_002fO){accesskey="6"}:                                          (instance)
  • [FileDescriptor-built ins](FileDescriptor_002dbuilt-ins.html#FileDescriptor_002dbuilt-ins){accesskey="7"}:                                                     (instance)
  • [FileDescriptor-class type methods](FileDescriptor_002dclass-type-methods.html#FileDescriptor_002dclass-type-methods){accesskey="8"}:                          (instance)
  • [FileDescriptor-initialize-release](FileDescriptor_002dinitialize_002drelease.html#FileDescriptor_002dinitialize_002drelease){accesskey="9"}:                  (instance)
  • [FileDescriptor-low-level access](FileDescriptor_002dlow_002dlevel-access.html#FileDescriptor_002dlow_002dlevel-access):                                       (instance)
  • [FileDescriptor-overriding inherited methods](FileDescriptor_002doverriding-inherited-methods.html#FileDescriptor_002doverriding-inherited-methods):           (instance)
  • [FileDescriptor-polymorphism](FileDescriptor_002dpolymorphism.html#FileDescriptor_002dpolymorphism):                                                           (instance)
  • [FileDescriptor-positioning](FileDescriptor_002dpositioning.html#FileDescriptor_002dpositioning):                                                              (instance)
  • [FileDescriptor-printing](FileDescriptor_002dprinting.html#FileDescriptor_002dprinting):                                                                       (instance)
  • [FileDescriptor-testing](FileDescriptor_002dtesting.html#FileDescriptor_002dtesting):                                                                          (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
