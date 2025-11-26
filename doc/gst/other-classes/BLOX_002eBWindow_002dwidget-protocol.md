[]{#BLOX_002eBWindow_002dwidget-protocol}

::: header
Previous:
[BLOX.BWindow-accessing](BLOX_002eBWindow_002daccessing.html#BLOX_002eBWindow_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BWindow](BLOX_002eBWindow.html#BLOX_002eBWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWindow_003a-widget-protocol}

#### 1.50.3 BLOX.BWindow: widget protocol {#blox.bwindow-widget-protocol .subsection}

[]{#index-center-4}

**center**

Center the window in the screen

[]{#index-centerIn_003a-1}

**centerIn: view**

Center the window in the given widget

[]{#index-height-1}

**height**

Answer the height of the window, as deduced from the geometry that the
window manager imposed on the window.

[]{#index-height_003a-1}

**height: anInteger**

Ask the window manager to give the given height to the window.

[]{#index-heightAbsolute-1}

**heightAbsolute**

Answer the height of the window, as deduced from the geometry that the
window manager imposed on the window.

[]{#index-heightOffset_003a-1}

**heightOffset: value**

This method should not be called for instances of this class.

[]{#index-iconify}

**iconify**

Map a window and in iconified state. If a window has not been mapped
yet, this is achieved by mapping the window in withdrawn state first,
and then iconifying it.

[]{#index-isMapped}

**isMapped**

Answer whether the window is mapped

[]{#index-isWindow-1}

**isWindow**

Answer 'true'.

[]{#index-map-1}

**map**

Map the window and bring it to the topmost position in the Z-order.

[]{#index-modalMap}

**modalMap**

Map the window while establishing an application-local grab for it. An
event loop is started that ends only after the window has been
destroyed.

When a grab is set for a particular window, all pointer events are
restructed to the grab window and its descendants in Blox's window
hierarchy. Whenever the pointer is within the grab window's subtree, the
pointer will behave exactly the same as if there had been no grab grab
at all and all events will be reported in the normal fashion. When the
pointer is outside the window's tree, button presses and releases and
mouse motion events are reported to the grabbing window, and window
entry and window exit events are ignored. In other words, windows
outside the grab subtree will be visible on the screen but they will be
insensitive until the grab is released. The tree of windows underneath
the grab window can include top-level windows, in which case all of
those top-level windows and their descendants will continue to receive
mouse events during the grab. Keyboard events (key presses and key
releases) are delivered as usual: the window manager controls which
application receives keyboard events, and if they are sent to any window
in the grabbing application then they are redirected to the window
owning the focus.

[]{#index-state-1}

**state**

Set the value of the state option for the window.

Specifies one of four states for the window: either normal, iconic,
withdrawn, or (Windows only) zoomed.

[]{#index-state_003a-1} []{#index-map-2} []{#index-unmap-1}

**state: aSymbol**

Raise an error. To set a BWindow's state, use #map and #unmap.

[]{#index-unmap}

**unmap**

Unmap a window, causing it to be forgotten about by the window manager

[]{#index-width-4}

**width**

Answer the width of the window, as deduced from the geometry that the
window manager imposed on the window.

[]{#index-width_003a-4}

**width: anInteger**

Ask the window manager to give the given width to the window.

[]{#index-width_003aheight_003a-1}

**width: xSize height: ySize**

Ask the window manager to give the given width and height to the window.

[]{#index-widthAbsolute-1}

**widthAbsolute**

Answer the width of the window, as deduced from the geometry that the
window manager imposed on the window.

[]{#index-widthOffset_003a-1}

**widthOffset: value**

This method should not be called for instances of this class.

[]{#index-window-1}

**window**

Answer the receiver.

[]{#index-x-1}

**x**

Answer the x coordinate of the window's top-left corner, as deduced from
the geometry that the window manager imposed on the window.

[]{#index-x_003a-1}

**x: anInteger**

Ask the window manager to move the window's left border to the given x
coordinate, keeping the size unchanged

[]{#index-x_003ay_003a-1}

**x: xPos y: yPos**

Ask the window manager to move the window's top-left corner to the given
coordinates, keeping the size unchanged

[]{#index-x_003ay_003awidth_003aheight_003a-1}

**x: xPos y: yPos width: xSize height: ySize**

Ask the window manager to give the requested geometry to the window.

[]{#index-xAbsolute-1}

**xAbsolute**

Answer the x coordinate of the window's top-left corner, as deduced from
the geometry that the window manager imposed on the window.

[]{#index-xOffset_003a-1}

**xOffset: value**

This method should not be called for instances of this class.

[]{#index-y-1}

**y**

Answer the y coordinate of the window's top-left corner, as deduced from
the geometry that the window manager imposed on the window.

[]{#index-y_003a-1}

**y: anInteger**

Ask the window manager to move the window's left border to the given y
coordinate, keeping the size unchanged

[]{#index-yAbsolute-1}

**yAbsolute**

Answer the y coordinate of the window's top-left corner, as deduced from
the geometry that the window manager imposed on the window.

[]{#index-yOffset_003a-1}

**yOffset: value**

This method should not be called for instances of this class.

------------------------------------------------------------------------

::: header
Previous:
[BLOX.BWindow-accessing](BLOX_002eBWindow_002daccessing.html#BLOX_002eBWindow_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BWindow](BLOX_002eBWindow.html#BLOX_002eBWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
