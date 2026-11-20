[]{#BLOX_002eBlox_002dwidget-protocol}

::: header
Previous:
[BLOX.Blox-customization](BLOX_002eBlox_002dcustomization.html#BLOX_002eBlox_002dcustomization){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox_003a-widget-protocol}

#### 1.26.9 BLOX.Blox: widget protocol {#blox.blox-widget-protocol .subsection}

[]{#index-asPrimitiveWidget-1}

**asPrimitiveWidget**

Answer the primitive widget that implements the receiver.

[]{#index-childrenCount}

**childrenCount**

Answer how many children the receiver has

[]{#index-childrenDo_003a}

**childrenDo: aBlock**

Evaluate aBlock once for each of the receiver's child widgets, passing
the widget to aBlock as a parameter

[]{#index-destroy}

**destroy**

Destroy the receiver

[]{#index-drawingArea}

**drawingArea**

Answer a Rectangle identifying the receiver's drawing area. The
rectangle's corners specify the upper-left and lower-right corners of
the client area. Because coordinates are relative to the upper-left
corner of a window's drawing area, the coordinates of the rectangle's
corner are (0,0).

[]{#index-enabled} []{#index-state-2}

**enabled**

Answer whether the receiver is enabled to input. Although defined here,
this method is only used for widgets that define a #state method

[]{#index-enabled_003a} []{#index-state_003a-2}

**enabled: enabled**

Set whether the receiver is enabled to input (enabled is a boolean).
Although defined here, this method is only used for widgets that define
a #state: method

[]{#index-exists}

**exists**

Answer whether the receiver has been destroyed or not (answer false in
the former case, true in the latter).

[]{#index-fontHeight_003a} []{#index-font-9}

**fontHeight: aString**

Answer the height of aString in pixels, when displayed in the same font
as the receiver. Although defined here, this method is only used for
widgets that define a #font method

[]{#index-fontWidth_003a} []{#index-font-10}

**fontWidth: aString**

Answer the width of aString in pixels, when displayed in the same font
as the receiver. Although defined here, this method is only used for
widgets that define a #font method

[]{#index-isWindow}

**isWindow**

Answer whether the receiver represents a window on the screen.

[]{#index-parent}

**parent**

Answer the receiver's parent (or nil for a top-level window).

[]{#index-toplevel}

**toplevel**

Answer the top-level object (typically a BWindow or BPopupWindow)
connected to the receiver.

[]{#index-window} []{#index-toplevel-1}

**window**

Answer the window in which the receiver stays. Note that while #toplevel
won't answer a BTransientWindow, this method will.

[]{#index-withChildrenDo_003a}

**withChildrenDo: aBlock**

Evaluate aBlock passing the receiver, and then once for each of the
receiver's child widgets.

------------------------------------------------------------------------

::: header
Previous:
[BLOX.Blox-customization](BLOX_002eBlox_002dcustomization.html#BLOX_002eBlox_002dcustomization){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
