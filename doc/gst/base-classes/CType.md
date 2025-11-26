[]{#CType}

::: header
Next: [CUChar](CUChar.html#CUChar){accesskey="n" rel="next"}, Previous:
[CStruct](CStruct.html#CStruct){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CType-1}

### 1.52 CType {#ctype .section}

[]{#index-CType}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-C interface**

:   I am not part of the standard Smalltalk kernel class hierarchy. I
    contain type information used by subclasses of CObject, which
    represents external C data items.

    My only instance variable, cObjectType, is used to hold onto the
    CObject subclass that gets created for a given CType. Used primarily
    in the C part of the interpreter because internally it cannot
    execute methods to get values, so it has a simple way to access
    instance variable which holds the desired subclass.

    My subclasses have instances which represent the actual data types;
    for the scalar types, there is only one instance created of each,
    but for the aggregate types, there is at least one instance per base
    type and/or number of elements.

  ----------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [CType class-C instance creation](CType-class_002dC-instance-creation.html#CType-class_002dC-instance-creation){accesskey="1"}:        (class)
  • [CType class-initialization](CType-class_002dinitialization.html#CType-class_002dinitialization){accesskey="2"}:                       (class)
  • [CType-accessing](CType_002daccessing.html#CType_002daccessing){accesskey="3"}:                                                        (instance)
  • [CType-basic](CType_002dbasic.html#CType_002dbasic){accesskey="4"}:                                                                    (instance)
  • [CType-C instance creation](CType_002dC-instance-creation.html#CType_002dC-instance-creation){accesskey="5"}:                          (instance)
  • [CType-storing](CType_002dstoring.html#CType_002dstoring){accesskey="6"}:                                                              (instance)
  ----------------------------------------------------------------------------------------------------------------------------------- ---- ------------
