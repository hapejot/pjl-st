[]{#PluggableProxy_002dsaving-and-restoring}

::: header
Previous: [PluggableProxy
class-accessing](PluggableProxy-class_002daccessing.html#PluggableProxy-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[PluggableProxy](PluggableProxy.html#PluggableProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PluggableProxy_003a-saving-and-restoring}

#### 1.131.2 PluggableProxy: saving and restoring {#pluggableproxy-saving-and-restoring .subsection}

[]{#index-object-2} []{#index-reconstructOriginalObject-2}
[]{#index-postLoad-8}

**object**

Reconstruct the object stored in the proxy and answer it; the
binaryRepresentationObject is sent the #reconstructOriginalObject
message, and the resulting object is sent the #postLoad message.
