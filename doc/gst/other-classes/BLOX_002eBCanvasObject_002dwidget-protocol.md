[]{#BLOX_002eBCanvasObject_002dwidget-protocol}

::: header
Previous:
[BLOX.BCanvasObject-accessing](BLOX_002eBCanvasObject_002daccessing.html#BLOX_002eBCanvasObject_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvasObject](BLOX_002eBCanvasObject.html#BLOX_002eBCanvasObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBCanvasObject_003a-widget-protocol}

#### 1.7.3 BLOX.BCanvasObject: widget protocol {#blox.bcanvasobject-widget-protocol .subsection}

[]{#index-create}

**create**

If the object has not been created yet and has been initialized
correctly, insert it for real in the parent canvas

[]{#index-created}

**created**

Answer whether the object is just a placeholder or has already been
inserted for real in the parent canvas

[]{#index-lower}

**lower**

Move the item to the lowest position in the display list. Child widgets
always obscure other item types, and the stacking order of window items
is determined by sending methods to the widget object directly.

[]{#index-raise}

**raise**

Move the item to the highest position in the display list. Child widgets
always obscure other item types, and the stacking order of window items
is determined by sending methods to the widget object directly.

[]{#index-redraw}

**redraw**

Force the object to be displayed in the parent canvas, creating it if it
has not been inserted for real in the parent, and refresh its position
if it has changed.

[]{#index-remove}

**remove**

Remove the object from the canvas

[]{#index-show}

**show**

Ensure that the object is visible in the center of the canvas, scrolling
it if necessary.
