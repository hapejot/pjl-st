[]{#PluggableProxy-class_002daccessing}

::: header
Next: [PluggableProxy-saving and
restoring](PluggableProxy_002dsaving-and-restoring.html#PluggableProxy_002dsaving-and-restoring){accesskey="n"
rel="next"}, Up:
[PluggableProxy](PluggableProxy.html#PluggableProxy){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PluggableProxy-class_003a-accessing}

#### 1.131.1 PluggableProxy class: accessing {#pluggableproxy-class-accessing .subsection}

[]{#index-on_003a-6} []{#index-binaryRepresentationObject-5}
[]{#index-preStore-2} []{#index-postStore-2}

**on: anObject**

Answer a proxy to be used to save anObject. The proxy stores a different
object obtained by sending to anObject the #binaryRepresentationObject
message (embedded between #preStore and #postStore as usual).
