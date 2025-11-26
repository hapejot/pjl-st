[]{#SingletonProxy_002dsaving-and-restoring}

::: header
Previous: [SingletonProxy class-instance
creation](SingletonProxy-class_002dinstance-creation.html#SingletonProxy-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[SingletonProxy](SingletonProxy.html#SingletonProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SingletonProxy_003a-saving-and-restoring}

#### 1.154.3 SingletonProxy: saving and restoring {#singletonproxy-saving-and-restoring .subsection}

[]{#index-object-3} []{#index-reconstructOriginalObject-3}
[]{#index-postLoad-9}

**object**

Reconstruct the object stored in the proxy and answer it; the
binaryRepresentationObject is sent the #reconstructOriginalObject
message, and the resulting object is sent the #postLoad message.
