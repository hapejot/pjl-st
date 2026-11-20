[]{#Object_002dfinalization}

::: header
Next:
[Object-introspection](Object_002dintrospection.html#Object_002dintrospection){accesskey="n"
rel="next"}, Previous: [Object-error
raising](Object_002derror-raising.html#Object_002derror-raising){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-finalization}

#### 1.123.11 Object: finalization {#object-finalization .subsection}

[]{#index-addToBeFinalized-1} []{#index-finalize-4}

**addToBeFinalized**

Arrange things so that #finalize is sent to the object when the garbage
collector finds out there are only weak references to it.

[]{#index-finalize-2}

**finalize**

Do nothing by default

[]{#index-mourn-2}

**mourn**

This method is sent by the VM to weak and ephemeron objects when one of
their fields is found out to be garbage collectable (this means, for
weak objects, that there are no references to it from non-weak objects,
and for ephemeron objects, that the only paths to the first instance
variable pass through other instance variables of the same ephemeron).
The default behavior is to do nothing.

[]{#index-removeToBeFinalized-1} []{#index-finalize-5}

**removeToBeFinalized**

Unregister the object, so that #finalize is no longer sent to the object
when the garbage collector finds out there are only weak references to
it.
