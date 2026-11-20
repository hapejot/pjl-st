[]{#BLOX_002eBWidget_002dgeometry-management}

::: header
Next: [BLOX.BWidget-widget
protocol](BLOX_002eBWidget_002dwidget-protocol.html#BLOX_002eBWidget_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BWidget-customization](BLOX_002eBWidget_002dcustomization.html#BLOX_002eBWidget_002dcustomization){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWidget_003a-geometry-management}

#### 1.49.4 BLOX.BWidget: geometry management {#blox.bwidget-geometry-management .subsection}

[]{#index-boundingBox-4}

**boundingBox**

Answer a Rectangle containing the bounding box of the receiver

[]{#index-boundingBox_003a}

**boundingBox: rect**

Set the bounding box of the receiver to rect (a Rectangle).

[]{#index-child_003aheight_003a-3} []{#index-height-2}
[]{#index-height_003a-3}

**child: child height: value**

Set the given child's height to value. The default implementation of
this method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #height method. You should not use this method,
which is automatically called by the child's #height: method, but you
might want to override it. The child's property slots whose name ends
with 'Geom' are reserved for this method. This method should never fail
-- if it doesn't apply to the kind of geometry management that the
receiver does, just do nothing.

[]{#index-child_003aheightOffset_003a-3} []{#index-height-3}
[]{#index-heightOffset_003a-2} []{#index-heightOffset_003a-3}

**child: child heightOffset: value**

Adjust the given child's height by a fixed amount of value pixel. This
is meaningful for the default implementation, using 'rubber-sheet'
geometry management as explained in the comment to BWidget's #height and
#heightOffset: methods. You should not use this method, which is
automatically called by the child's #heightOffset: method, but you might
want to override it. if it doesn't apply to the kind of geometry
management that the receiver does, just add value to the current height
of the widget.

[]{#index-child_003astretch_003a}

**child: child stretch: aBoolean**

This method is only used when on the path from the receiver to its
toplevel there is a BContainer. It decides whether child is among the
widgets that are stretched to fill the entire width of the BContainer;
if this has not been set for this widget, it is propagated along the
widget hierarchy.

[]{#index-child_003awidth_003a-3} []{#index-width-5}
[]{#index-width_003a-6}

**child: child width: value**

Set the given child's width to value. The default implementation of this
method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #width method. You should not use this method,
which is automatically called by the child's #width: method, but you
might want to override it. The child's property slots whose name ends
with 'Geom' are reserved for this method. This method should never fail
-- if it doesn't apply to the kind of geometry management that the
receiver does, just do nothing.

[]{#index-child_003awidthOffset_003a-3} []{#index-width-6}
[]{#index-widthOffset_003a-2} []{#index-widthOffset_003a-3}

**child: child widthOffset: value**

Adjust the given child's width by a fixed amount of value pixel. This is
meaningful for the default implementation, using 'rubber-sheet' geometry
management as explained in the comment to BWidget's #width and
#widthOffset: methods. You should not use this method, which is
automatically called by the child's #widthOffset: method, but you might
want to override it. if it doesn't apply to the kind of geometry
management that the receiver does, just add value to the current width
of the widget.

[]{#index-child_003ax_003a-3} []{#index-x-2} []{#index-x_003a-2}

**child: child x: value**

Set the given child's x to value. The default implementation of this
method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #x method. You should not use this method, which is
automatically called by the child's #x: method, but you might want to
override it. The child's property slots whose name ends with 'Geom' are
reserved for this method. This method should never fail -- if it doesn't
apply to the kind of geometry management that the receiver does, just do
nothing.

[]{#index-child_003axOffset_003a-3} []{#index-x-3}
[]{#index-xOffset_003a-2} []{#index-xOffset_003a-3}

**child: child xOffset: value**

Adjust the given child's x by a fixed amount of value pixel. This is
meaningful for the default implementation, using 'rubber-sheet' geometry
management as explained in the comment to BWidget's #x and #xOffset:
methods. You should not use this method, which is automatically called
by the child's #xOffset: method, but you might want to override it. if
it doesn't apply to the kind of geometry management that the receiver
does, just add value to the current x of the widget.

[]{#index-child_003ay_003a-3} []{#index-y-2} []{#index-y_003a-2}

**child: child y: value**

Set the given child's y to value. The default implementation of this
method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #y method. You should not use this method, which is
automatically called by the child's #y: method, but you might want to
override it. The child's property slots whose name ends with 'Geom' are
reserved for this method. This method should never fail -- if it doesn't
apply to the kind of geometry management that the receiver does, just do
nothing.

[]{#index-child_003ayOffset_003a-3} []{#index-y-3}
[]{#index-yOffset_003a-2} []{#index-yOffset_003a-3}

**child: child yOffset: value**

Adjust the given child's y by a fixed amount of value pixel. This is
meaningful for the default implementation, using 'rubber-sheet' geometry
management as explained in the comment to BWidget's #y and #yOffset:
methods. You should not use this method, which is automatically called
by the child's #yOffset: method, but you might want to override it. if
it doesn't apply to the kind of geometry management that the receiver
does, just add value to the current y of the widget.

[]{#index-extent-1}

**extent**

Answer a Point containing the receiver's size

[]{#index-extent_003a-1}

**extent: extent**

Set the receiver's size to the width and height contained in extent (a
Point).

[]{#index-height} []{#index-heightOffset_003a-4}

**height**

Answer the 'variable' part of the receiver's height within the parent
widget. The value returned does not include any fixed amount of pixels
indicated by #heightOffset: and must be interpreted in a relative
fashion: the ratio of the returned value to the current size of the
parent will be preserved upon resize. This apparently complicated method
is known as 'rubber sheet' geometry management. Behavior if the left or
right edges are not within the client area of the parent is not defined
-- the window might be clamped or might be positioned according to the
specification.

[]{#index-height_003a}

**height: value**

Set to 'value' the height of the widget within the parent widget. The
value is specified in a relative fashion as an integer, so that the
ratio of 'value' to the current size of the parent will be preserved
upon resize. This apparently complicated method is known as 'rubber
sheet' geometry management.

[]{#index-heightAbsolute}

**heightAbsolute**

Force a recalculation of the layout of widgets in the receiver's parent,
then answer the current height of the receiver in pixels.

[]{#index-heightChild_003a-3} []{#index-height-4} []{#index-height-5}

**heightChild: child**

Answer the given child's height. The default implementation of this
method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #height method. You should not use this method,
which is automatically called by the child's #height method, but you
might want to override. The child's property slots whose name ends with
'Geom' are reserved for this method. This method should never fail -- if
it doesn't apply to the kind of geometry management that the receiver
does, just return 0.

[]{#index-heightOffset} []{#index-height_003a-4}

**heightOffset**

Private - Answer the pixels to be added or subtracted to the height of
the receiver, with respect to the value set in a relative fashion
through the #height: method.

[]{#index-heightOffset_003a} []{#index-height_003a-5}
[]{#index-inset_003a-1}

**heightOffset: value**

Add or subtract to the height of the receiver a fixed amount of 'value'
pixels, with respect to the value set in a relative fashion through the
#height: method. Usage of this method is deprecated; use #inset: and
BContainers instead.

[]{#index-heightPixels_003a} []{#index-height-6} []{#index-height-7}
[]{#index-height_003a-6}

**heightPixels: value**

Set the current height of the receiver to 'value' pixels. Note that,
after calling this method, #height will answer 0, which is logical
considering that there is no 'variable' part of the size (refer to
#height and #height: for more explanations).

[]{#index-inset_003a}

**inset: pixels**

Inset the receiver's bounding box by the specified amount.

[]{#index-left_003atop_003aright_003abottom_003a}

**left: left top: top right: right bottom: bottom**

Set the bounding box of the receiver through its components.

[]{#index-pos_003a}

**pos: position**

Set the receiver's origin to the width and height contained in position
(a Point).

[]{#index-posHoriz_003a}

**posHoriz: aBlox**

Position the receiver immediately to the right of aBlox.

[]{#index-posVert_003a}

**posVert: aBlox**

Position the receiver just below aBlox.

[]{#index-stretch_003a}

**stretch: aBoolean**

This method is only considered when on the path from the receiver to its
toplevel there is a BContainer. It decides whether we are among the
widgets that are stretched to fill the entire width of the BContainer.

[]{#index-width-3} []{#index-widthOffset_003a-4}

**width**

Answer the 'variable' part of the receiver's width within the parent
widget. The value returned does not include any fixed amount of pixels
indicated by #widthOffset: and must be interpreted in a relative
fashion: the ratio of the returned value to the current size of the
parent will be preserved upon resize. This apparently complicated method
is known as 'rubber sheet' geometry management. Behavior if the left or
right edges are not within the client area of the parent is not defined
-- the window might be clamped or might be positioned according to the
specification.

[]{#index-width_003a-3}

**width: value**

Set to 'value' the width of the widget within the parent widget. The
value is specified in a relative fashion as an integer, so that the
ratio of 'value' to the current size of the parent will be preserved
upon resize. This apparently complicated method is known as 'rubber
sheet' geometry management.

[]{#index-width_003aheight_003a}

**width: xSize height: ySize**

Set the size of the receiver through its components xSize and ySize.

[]{#index-widthAbsolute}

**widthAbsolute**

Force a recalculation of the layout of widgets in the receiver's parent,
then answer the current width of the receiver in pixels.

[]{#index-widthChild_003a-3} []{#index-width-7} []{#index-width-8}

**widthChild: child**

Answer the given child's width. The default implementation of this
method uses 'rubber-sheet' geometry management as explained in the
comment to BWidget's #width method. You should not use this method,
which is automatically called by the child's #width method, but you
might want to override. The child's property slots whose name ends with
'Geom' are reserved for this method. This method should never fail -- if
it doesn't apply to the kind of geometry management that the receiver
does, just return 0.

[]{#index-widthOffset} []{#index-width_003a-7}

**widthOffset**

Private - Answer the pixels to be added or subtracted to the width of
the receiver, with respect to the value set in a relative fashion
through the #width: method.

[]{#index-widthOffset_003a} []{#index-width_003a-8}
[]{#index-inset_003a-2}

**widthOffset: value**

Add or subtract to the width of the receiver a fixed amount of 'value'
pixels, with respect to the value set in a relative fashion through the
#width: method. Usage of this method is deprecated; use #inset: and
BContainers instead.

[]{#index-widthPixels_003a} []{#index-width-9} []{#index-width-10}
[]{#index-width_003a-9}

**widthPixels: value**

Set the current width of the receiver to 'value' pixels. Note that,
after calling this method, #width will answer 0, which is logical
considering that there is no 'variable' part of the size (refer to
#width and #width: for more explanations).

[]{#index-x} []{#index-xOffset_003a-4}

**x**

Answer the 'variable' part of the receiver's x within the parent widget.
The value returned does not include any fixed amount of pixels indicated
by #xOffset: and must be interpreted in a relative fashion: the ratio of
the returned value to the current size of the parent will be preserved
upon resize. This apparently complicated method is known as 'rubber
sheet' geometry management. Behavior if the left or right edges are not
within the client area of the parent is not defined -- the window might
be clamped or might be positioned according to the specification.

[]{#index-x_003a}

**x: value**

Set to 'value' the x of the widget within the parent widget. The value
is specified in a relative fashion as an integer, so that the ratio of
'value' to the current size of the parent will be preserved upon resize.
This apparently complicated method is known as 'rubber sheet' geometry
management.

[]{#index-x_003ay_003a}

**x: xPos y: yPos**

Set the origin of the receiver through its components xPos and yPos.

[]{#index-x_003ay_003awidth_003aheight_003a}

**x: xPos y: yPos width: xSize height: ySize**

Set the bounding box of the receiver through its origin and size.

[]{#index-xAbsolute}

**xAbsolute**

Force a recalculation of the layout of widgets in the receiver's parent,
then answer the current x of the receiver in pixels.

[]{#index-xChild_003a-3} []{#index-x-4} []{#index-x-5}

**xChild: child**

Answer the given child's x. The default implementation of this method
uses 'rubber-sheet' geometry management as explained in the comment to
BWidget's #x method. You should not use this method, which is
automatically called by the child's #x method, but you might want to
override. The child's property slots whose name ends with 'Geom' are
reserved for this method. This method should never fail -- if it doesn't
apply to the kind of geometry management that the receiver does, just
return 0.

[]{#index-xOffset} []{#index-x_003a-3}

**xOffset**

Private - Answer the pixels to be added or subtracted to the x of the
receiver, with respect to the value set in a relative fashion through
the #x: method.

[]{#index-xOffset_003a} []{#index-x_003a-4} []{#index-inset_003a-3}

**xOffset: value**

Add or subtract to the x of the receiver a fixed amount of 'value'
pixels, with respect to the value set in a relative fashion through the
#x: method. Usage of this method is deprecated; use #inset: and
BContainers instead.

[]{#index-xPixels_003a} []{#index-x-6} []{#index-x-7}
[]{#index-x_003a-5}

**xPixels: value**

Set the current x of the receiver to 'value' pixels. Note that, after
calling this method, #x will answer 0, which is logical considering that
there is no 'variable' part of the size (refer to #x and #x: for more
explanations).

[]{#index-xRoot}

**xRoot**

Answer the x position of the receiver with respect to the top-left
corner of the desktop (including the offset of the virtual root window
under X).

[]{#index-y} []{#index-yOffset_003a-4}

**y**

Answer the 'variable' part of the receiver's y within the parent widget.
The value returned does not include any fixed amount of pixels indicated
by #yOffset: and must be interpreted in a relative fashion: the ratio of
the returned value to the current size of the parent will be preserved
upon resize. This apparently complicated method is known as 'rubber
sheet' geometry management. Behavior if the left or right edges are not
within the client area of the parent is not defined -- the window might
be clamped or might be positioned according to the specification.

[]{#index-y_003a}

**y: value**

Set to 'value' the y of the widget within the parent widget. The value
is specified in a relative fashion as an integer, so that the ratio of
'value' to the current size of the parent will be preserved upon resize.
This apparently complicated method is known as 'rubber sheet' geometry
management.

[]{#index-yAbsolute}

**yAbsolute**

Force a recalculation of the layout of widgets in the receiver's parent,
then answer the current y of the receiver in pixels.

[]{#index-yChild_003a-3} []{#index-y-4} []{#index-y-5}

**yChild: child**

Answer the given child's y. The default implementation of this method
uses 'rubber-sheet' geometry management as explained in the comment to
BWidget's #y method. You should not use this method, which is
automatically called by the child's #y method, but you might want to
override. The child's property slots whose name ends with 'Geom' are
reserved for this method. This method should never fail -- if it doesn't
apply to the kind of geometry management that the receiver does, just
return 0.

[]{#index-yOffset} []{#index-y_003a-3}

**yOffset**

Private - Answer the pixels to be added or subtracted to the y of the
receiver, with respect to the value set in a relative fashion through
the #y: method.

[]{#index-yOffset_003a} []{#index-y_003a-4} []{#index-inset_003a-4}

**yOffset: value**

Add or subtract to the y of the receiver a fixed amount of 'value'
pixels, with respect to the value set in a relative fashion through the
#y: method. Usage of this method is deprecated; use #inset: and
BContainers instead.

[]{#index-yPixels_003a} []{#index-y-6} []{#index-y-7}
[]{#index-y_003a-5}

**yPixels: value**

Set the current y of the receiver to 'value' pixels. Note that, after
calling this method, #y will answer 0, which is logical considering that
there is no 'variable' part of the size (refer to #y and #y: for more
explanations).

[]{#index-yRoot}

**yRoot**

Answer the y position of the receiver with respect to the top-left
corner of the desktop (including the offset of the virtual root window
under X).

------------------------------------------------------------------------

::: header
Next: [BLOX.BWidget-widget
protocol](BLOX_002eBWidget_002dwidget-protocol.html#BLOX_002eBWidget_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BWidget-customization](BLOX_002eBWidget_002dcustomization.html#BLOX_002eBWidget_002dcustomization){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
