[]{#VariableBinding_002dsaving-and-loading}

::: header
Next:
[VariableBinding-storing](VariableBinding_002dstoring.html#VariableBinding_002dstoring){accesskey="n"
rel="next"}, Previous:
[VariableBinding-printing](VariableBinding_002dprinting.html#VariableBinding_002dprinting){accesskey="p"
rel="prev"}, Up:
[VariableBinding](VariableBinding.html#VariableBinding){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#VariableBinding_003a-saving-and-loading}

#### 1.206.3 VariableBinding: saving and loading {#variablebinding-saving-and-loading .subsection}

[]{#index-binaryRepresentationObject-3} []{#index-at_003a-28}

**binaryRepresentationObject**

This method is implemented to allow for a PluggableProxy to be used with
VariableBindings. Answer a DirectedMessage which sends #at: to the
environment that holds the receiver.
