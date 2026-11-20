[]{#BLOX_002eBlox_002dcustomization}

::: header
Next: [BLOX.Blox-widget
protocol](BLOX_002eBlox_002dwidget-protocol.html#BLOX_002eBlox_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.Blox-creating
children](BLOX_002eBlox_002dcreating-children.html#BLOX_002eBlox_002dcreating-children){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox_003a-customization}

#### 1.26.8 BLOX.Blox: customization {#blox.blox-customization .subsection}

[]{#index-addChild_003a-1} []{#index-basicAddChild_003a-2}

**addChild: child**

The widget identified by child has been added to the receiver. This
method is public not because you can call it, but because it can be
useful to override it, not forgetting the call to either the superclass
implementation or #basicAddChild:, to perform some initialization on the
children just added. Answer the new child.

[]{#index-basicAddChild_003a} []{#index-addChild_003a-4}

**basicAddChild: child**

The widget identified by child has been added to the receiver. Add it to
the children collection and answer the new child. This method is public
because you can call it from #addChild:.
