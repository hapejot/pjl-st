[]{#BLOX_002eBList_002dwidget-protocol}

::: header
Previous:
[BLOX.BList-accessing](BLOX_002eBList_002daccessing.html#BLOX_002eBList_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BList](BLOX_002eBList.html#BLOX_002eBList){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBList_003a-widget-protocol}

#### 1.25.2 BLOX.BList: widget protocol {#blox.blist-widget-protocol .subsection}

[]{#index-callback-4}

**callback**

Answer a DirectedMessage that is sent when the active item in the
receiver changes, or nil if none has been set up.

[]{#index-callback_003amessage_003a-5}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
selector with at most two arguemtnts) when the active item in the
receiver changegs. If the method accepts two arguments, the receiver is
passed as the first parameter. If the method accepts one or two
arguments, the selected index is passed as the last parameter.

[]{#index-highlight_003a}

**highlight: index**

Highlight the item at the given position in the listbox.

[]{#index-invokeCallback-6}

**invokeCallback**

Generate a synthetic callback.

[]{#index-select_003a}

**select: index**

Highlight the item at the given position in the listbox, without
unhighlighting other items. This is meant for multiple- or extended-mode
listboxes, but can be used with other selection mode in particular
cases.

[]{#index-show_003a}

**show: index**

Ensure that the item at the given position in the listbox is visible.

[]{#index-unhighlight}

**unhighlight**

Unhighlight all the items in the listbox.

[]{#index-unselect_003a}

**unselect: index**

Unhighlight the item at the given position in the listbox, without
affecting the state of the other items.
