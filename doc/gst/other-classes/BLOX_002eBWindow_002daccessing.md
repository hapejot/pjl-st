[]{#BLOX_002eBWindow_002daccessing}

::: header
Next: [BLOX.BWindow-widget
protocol](BLOX_002eBWindow_002dwidget-protocol.html#BLOX_002eBWindow_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BWindow class-instance
creation](BLOX_002eBWindow-class_002dinstance-creation.html#BLOX_002eBWindow-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BWindow](BLOX_002eBWindow.html#BLOX_002eBWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWindow_003a-accessing}

#### 1.50.2 BLOX.BWindow: accessing {#blox.bwindow-accessing .subsection}

[]{#index-callback-7}

**callback**

Answer a DirectedMessage that is sent to verify whether the receiver
must be destroyed when the user asks to unmap it.

[]{#index-callback_003amessage_003a-10}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the user asks to unmap the
receiver. If the method accepts an argument, the receiver is passed.

If the method returns true, the window and its children are destroyed
(which is the default action, taken if no callback is set up). If the
method returns false, the window is left in place.

[]{#index-invokeCallback-10}

**invokeCallback**

Generate a synthetic callback, destroying the window if no callback was
set up or if the callback method answers true.

[]{#index-label-5}

**label**

Answer the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

[]{#index-label_003a-5}

**label: value**

Set the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

[]{#index-menu_003a}

**menu: value**

Set the value of the menu option for the widget.

Specifies a menu widget to be used as a menubar. On the Macintosh, the
menubar will be displayed accross the top of the main monitor. On
Microsoft Windows and all UNIX platforms, the menu will appear accross
the toplevel window as part of the window dressing maintained by the
window manager.

[]{#index-resizable}

**resizable**

Answer the value of the resizable option for the widget.

Answer whether the user can be resize the window or not. If resizing is
disabled, then the window's size will be the size from the most recent
interactive resize or geometry-setting method. If there has been no such
operation then the window's natural size will be used.

[]{#index-resizable_003a}

**resizable: value**

Set the value of the resizable option for the widget.

Answer whether the user can be resize the window or not. If resizing is
disabled, then the window's size will be the size from the most recent
interactive resize or geometry-setting method. If there has been no such
operation then the window's natural size will be used.

------------------------------------------------------------------------

::: header
Next: [BLOX.BWindow-widget
protocol](BLOX_002eBWindow_002dwidget-protocol.html#BLOX_002eBWindow_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BWindow class-instance
creation](BLOX_002eBWindow-class_002dinstance-creation.html#BLOX_002eBWindow-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BWindow](BLOX_002eBWindow.html#BLOX_002eBWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
