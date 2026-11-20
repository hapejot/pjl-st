[]{#BLOX_002eBCanvas_002dgeometry-management}

::: header
Next: [BLOX.BCanvas-widget
protocol](BLOX_002eBCanvas_002dwidget-protocol.html#BLOX_002eBCanvas_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BCanvas-accessing](BLOX_002eBCanvas_002daccessing.html#BLOX_002eBCanvas_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvas](BLOX_002eBCanvas.html#BLOX_002eBCanvas){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBCanvas_003a-geometry-management}

#### 1.6.2 BLOX.BCanvas: geometry management {#blox.bcanvas-geometry-management .subsection}

[]{#index-addChild_003a} []{#index-basicAddChild_003a-1}

**addChild: child**

The widget identified by child has been added to the receiver. This
method is public not because you can call it, but because it can be
useful to override it, not forgetting the call to either the superclass
implementation or #basicAddChild:, to perform some initialization on the
children just added. Answer the new child.

[]{#index-child_003aheight_003a}

**child: child height: value**

Set the given child's height.

[]{#index-child_003aheightOffset_003a}

**child: child heightOffset: value**

Offset the given child's height by value pixels.

[]{#index-child_003awidth_003a}

**child: child width: value**

Set the given child's width.

[]{#index-child_003awidthOffset_003a}

**child: child widthOffset: value**

Offset the given child's width by value pixels.

[]{#index-child_003ax_003a}

**child: child x: value**

Set the given child's top-left corner's x coordinate, in pixels in the
canvas' coordinate system.

[]{#index-child_003axOffset_003a}

**child: child xOffset: value**

Offset the given child's top-left x by value pixels.

[]{#index-child_003ay_003a}

**child: child y: value**

Set the given child's top-left corner's y coordinate, in pixels in the
canvas' coordinate system.

[]{#index-child_003ayOffset_003a}

**child: child yOffset: value**

Offset the given child's top-left y by value pixels.

[]{#index-heightChild_003a}

**heightChild: child**

Answer the given child's height in pixels.

[]{#index-widthChild_003a}

**widthChild: child**

Answer the given child's width in pixels.

[]{#index-xChild_003a}

**xChild: child**

Answer the given child's top-left corner's x coordinate, in pixels in
the canvas' coordinate system.

[]{#index-yChild_003a}

**yChild: child**

Answer the given child's top-left corner's y coordinate, in pixels in
the canvas' coordinate system.

------------------------------------------------------------------------

::: header
Next: [BLOX.BCanvas-widget
protocol](BLOX_002eBCanvas_002dwidget-protocol.html#BLOX_002eBCanvas_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BCanvas-accessing](BLOX_002eBCanvas_002daccessing.html#BLOX_002eBCanvas_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvas](BLOX_002eBCanvas.html#BLOX_002eBCanvas){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
