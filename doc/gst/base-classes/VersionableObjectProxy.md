[]{#VersionableObjectProxy}

::: header
Next:
[VFS.ArchiveFile](VFS_002eArchiveFile.html#VFS_002eArchiveFile){accesskey="n"
rel="next"}, Previous:
[VariableBinding](VariableBinding.html#VariableBinding){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VersionableObjectProxy-1}

### 1.207 VersionableObjectProxy {#versionableobjectproxy .section}

[]{#index-VersionableObjectProxy}

**Defined in namespace Smalltalk**\
**Superclass: NullProxy**\
**Category: Streams-Files**

:   I am a proxy that stores additional information to allow different
    versions of an object's representations to be handled by the
    program. VersionableObjectProxies are backwards compatible, that is
    you can support versioning even if you did not use a
    VersionableObjectProxy for that class when the object was
    originarily dumped. VersionableObjectProxy does not support classes
    that changed shape across different versions. See the method
    comments for more information.

  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [VersionableObjectProxy class-saving and restoring](VersionableObjectProxy-class_002dsaving-and-restoring.html#VersionableObjectProxy-class_002dsaving-and-restoring){accesskey="1"}:        (class)
  • [VersionableObjectProxy-saving and restoring](VersionableObjectProxy_002dsaving-and-restoring.html#VersionableObjectProxy_002dsaving-and-restoring){accesskey="2"}:                          (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
