[]{#BLOX_002eBWidget-class_002dpopups}

::: header
Next:
[BLOX.BWidget-accessing](BLOX_002eBWidget_002daccessing.html#BLOX_002eBWidget_002daccessing){accesskey="n"
rel="next"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWidget-class_003a-popups}

#### 1.49.1 BLOX.BWidget class: popups {#blox.bwidget-class-popups .subsection}

[]{#index-new-5} []{#index-destroy-2}

**new**

Create an instance of the receiver inside a BPopupWindow; do not map the
window, answer the new widget. The created widget will become a child of
the window and be completely attached to it (e.g. the geometry methods
will modify the window's geometry). Note that while the widget \*seems\*
to be directly painted on the root window, it actually belongs to the
BPopupWindow; so don't send #destroy to the widget to remove it, but
rather to the window.

[]{#index-popup_003a} []{#index-destroy-3}

**popup: initializationBlock**

Create an instance of the receiver inside a BPopupWindow; before
returning, pass the widget to the supplied initializationBlock, then map
the window. Answer the new widget. The created widget will become a
child of the window and be completely attached to it (e.g. the geometry
methods will modify the window's geometry). Note that while the widget
\*seems\* to be directly painted on the root window, it actually belongs
to the BPopupWindow; so don't send #destroy to the widget to remove it,
but rather to the window.
