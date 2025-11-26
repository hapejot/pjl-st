[]{#BLOX_002eBDialog_002dwidget-protocol}

::: header
Previous:
[BLOX.BDialog-accessing](BLOX_002eBDialog_002daccessing.html#BLOX_002eBDialog_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BDialog](BLOX_002eBDialog.html#BLOX_002eBDialog){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDialog_003a-widget-protocol}

#### 1.11.4 BLOX.BDialog: widget protocol {#blox.bdialog-widget-protocol .subsection}

[]{#index-center-1}

**center**

Center the dialog box's parent window in the screen

[]{#index-centerIn_003a}

**centerIn: view**

Center the dialog box's parent window in the given widget

[]{#index-destroyed-1}

**destroyed**

Private - The receiver has been destroyed, clear the corresponding Tcl
variable to avoid memory leaks.

[]{#index-invokeCallback_003a}

**invokeCallback: index**

Generate a synthetic callback corresponding to the index-th button being
pressed, and destroy the parent window (triggering its callback if one
was established).

[]{#index-loop} []{#index-modalMap-1}

**loop**

Map the parent window modally. In other words, an event loop is started
that ends only after the window has been destroyed. For more information
on the treatment of events for modal windows, refer to
BWindow\>\>#modalMap.
