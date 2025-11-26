[]{#BLOX_002eBCanvas_002dwidget-protocol}

::: header
Previous: [BLOX.BCanvas-geometry
management](BLOX_002eBCanvas_002dgeometry-management.html#BLOX_002eBCanvas_002dgeometry-management){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvas](BLOX_002eBCanvas.html#BLOX_002eBCanvas){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBCanvas_003a-widget-protocol}

#### 1.6.3 BLOX.BCanvas: widget protocol {#blox.bcanvas-widget-protocol .subsection}

[]{#index-at_003a}

**at: aPoint**

Selects the topmost item in the canvas overlapping the point given by
aPoint.

[]{#index-between_003aand_003ado_003a}

**between: origin and: corner do: aBlock**

Evaluate aBlock for each item whose bounding box intersects the
rectangle between the two Points, origin and corner. Pass the item to
the block.

[]{#index-boundingBox-1}

**boundingBox**

Answer the bounding box of all the items in the canvas

[]{#index-destroyed}

**destroyed**

The widget has been destroyed. Tell all of its items about this fact.

[]{#index-do_003a}

**do: aBlock**

Evaluate aBlock, passing each item to it.

[]{#index-empty}

**empty**

Remove all the items from the canvas, leaving it empty

[]{#index-extraSpace}

**extraSpace**

Answer the amount of space that is left as a border around the canvas
items.

[]{#index-extraSpace_003a}

**extraSpace: aPoint**

Set the amount of space that is left as a border around the canvas
items.

[]{#index-items}

**items**

Answer an Array containing all the items in the canvas

[]{#index-mapPoint_003a}

**mapPoint: aPoint**

Given aPoint, a point expressed in window coordinates, answer the
corresponding canvas coordinates that are displayed at that location.
