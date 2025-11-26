[]{#BLOX_002eBTransientWindow_002dwidget-protocol}

::: header
Previous: [BLOX.BTransientWindow class-instance
creation](BLOX_002eBTransientWindow-class_002dinstance-creation.html#BLOX_002eBTransientWindow-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BTransientWindow](BLOX_002eBTransientWindow.html#BLOX_002eBTransientWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBTransientWindow_003a-widget-protocol}

#### 1.47.2 BLOX.BTransientWindow: widget protocol {#blox.btransientwindow-widget-protocol .subsection}

[]{#index-map}

**map**

Map the window and inform the windows manager that the receiver is a
transient window working on behalf of its parent. The window is also put
in its parent window's window group: the window manager might use this
information, for example, to unmap all of the windows in a group when
the group's leader is iconified.
