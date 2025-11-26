[]{#PluggableAdaptor-class_002dcreating-instances}

::: header
Next:
[PluggableAdaptor-accessing](PluggableAdaptor_002daccessing.html#PluggableAdaptor_002daccessing){accesskey="n"
rel="next"}, Up:
[PluggableAdaptor](PluggableAdaptor.html#PluggableAdaptor){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PluggableAdaptor-class_003a-creating-instances}

#### 1.130.1 PluggableAdaptor class: creating instances {#pluggableadaptor-class-creating-instances .subsection}

[]{#index-getBlock_003aputBlock_003a} []{#index-value-25}
[]{#index-value_003a-23}

**getBlock: getBlock putBlock: putBlock**

Answer a PluggableAdaptor using the given blocks to implement #value and
#value:

[]{#index-on_003aaspect_003a} []{#index-value-26}
[]{#index-value_003a-24}

**on: anObject aspect: aSymbol**

Answer a PluggableAdaptor using anObject's aSymbol message to implement
#value, and anObject's aSymbol: message (aSymbol followed by a colon) to
implement #value:

[]{#index-on_003agetSelector_003aputSelector_003a} []{#index-value-27}
[]{#index-value_003a-25}

**on: anObject getSelector: getSelector putSelector: putSelector**

Answer a PluggableAdaptor using anObject's getSelector message to
implement #value, and anObject's putSelector message to implement
#value:

[]{#index-on_003aindex_003a} []{#index-at_003a-26}
[]{#index-at_003aput_003a-29} []{#index-value-28}
[]{#index-value_003a-26} []{#index-at_003a-27}
[]{#index-at_003aput_003a-30}

**on: anObject index: anIndex**

Answer a PluggableAdaptor using anObject's #at: and #at:put: message to
implement #value and #value:; the first parameter of #at: and #at:put:
is anIndex

[]{#index-on_003akey_003a} []{#index-on_003aindex_003a-1}

**on: aDictionary key: aKey**

Same as #on:index:. Provided for clarity and completeness.
