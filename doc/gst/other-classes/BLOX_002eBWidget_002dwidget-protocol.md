[]{#BLOX_002eBWidget_002dwidget-protocol}

::: header
Previous: [BLOX.BWidget-geometry
management](BLOX_002eBWidget_002dgeometry-management.html#BLOX_002eBWidget_002dgeometry-management){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWidget_003a-widget-protocol}

#### 1.49.5 BLOX.BWidget: widget protocol {#blox.bwidget-widget-protocol .subsection}

[]{#index-activate}

**activate**

At any given time, one window on each display is designated as the focus
window; any key press or key release events for the display are sent to
that window. This method allows one to choose which window will have the
focus in the receiver's display

If the application currently has the input focus on the receiver's
display, this method resets the input focus for the receiver's display
to the receiver. If the application doesn't currently have the input
focus on the receiver's display, Blox will remember the receiver as the
focus for its top-level; the next time the focus arrives at the
top-level, it will be redirected to the receiver (this is because most
window managers will set the focus only to top-level windows, leaving it
up to the application to redirect the focus among the children of the
top-level).

[]{#index-activateNext} []{#index-tabStop_003a-1}

**activateNext**

Activate the next widget in the focus 'tabbing' order. The focus order
depends on the widget creation order; you can set which widgets are in
the order with the #tabStop: method.

[]{#index-activatePrevious} []{#index-tabStop_003a-2}

**activatePrevious**

Activate the previous widget in the focus 'tabbing' order. The focus
order depends on the widget creation order; you can set which widgets
are in the order with the #tabStop: method.

[]{#index-bringToTop}

**bringToTop**

Raise the receiver so that it is above all of its siblings in the
widgets' z-order; the receiver will not be obscured by any siblings and
will obscure any siblings that overlap it.

[]{#index-isActive}

**isActive**

Return whether the receiver is the window that currently owns the focus
on its display.

[]{#index-sendToBack}

**sendToBack**

Lower the receiver so that it is below all of its siblings in the
widgets' z-order; the receiver will be obscured by any siblings that
overlap it and will not obscure any siblings.

------------------------------------------------------------------------

::: header
Previous: [BLOX.BWidget-geometry
management](BLOX_002eBWidget_002dgeometry-management.html#BLOX_002eBWidget_002dgeometry-management){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
