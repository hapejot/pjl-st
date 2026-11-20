[]{#BLOX_002eBEdit_002dwidget-protocol}

::: header
Previous:
[BLOX.BEdit-accessing](BLOX_002eBEdit_002daccessing.html#BLOX_002eBEdit_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BEdit](BLOX_002eBEdit.html#BLOX_002eBEdit){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBEdit_003a-widget-protocol}

#### 1.15.3 BLOX.BEdit: widget protocol {#blox.bedit-widget-protocol .subsection}

[]{#index-destroyed-2}

**destroyed**

Private - The receiver has been destroyed, clear the corresponding Tcl
variable to avoid memory leaks.

[]{#index-hasSelection}

**hasSelection**

Answer whether there is selected text in the widget

[]{#index-insertAtEnd_003a-1}

**insertAtEnd: aString**

Clear the selection and append aString at the end of the widget.

[]{#index-insertText_003a}

**insertText: aString**

Insert aString in the widget at the current insertion point, replacing
the currently selected text (if any).

[]{#index-invokeCallback-5}

**invokeCallback**

Generate a synthetic callback.

[]{#index-nextPut_003a}

**nextPut: aCharacter**

Clear the selection and append aCharacter at the end of the widget.

[]{#index-nextPutAll_003a}

**nextPutAll: aString**

Clear the selection and append aString at the end of the widget.

[]{#index-nl}

**nl**

Clear the selection and append a linefeed character at the end of the
widget.

[]{#index-replaceSelection_003a-1}

**replaceSelection: aString**

Insert aString in the widget at the current insertion point, replacing
the currently selected text (if any), and leaving the text selected.

[]{#index-selectAll-1}

**selectAll**

Select the whole contents of the widget.

[]{#index-selectFrom_003ato_003a-1}

**selectFrom: first to: last**

Sets the selection to include the characters starting with the one
indexed by first (the very first character in the widget having index 1)
and ending with the one just before last. If last refers to the same
character as first or an earlier one, then the widget's selection is
cleared.

[]{#index-selection-1}

**selection**

Answer an empty string if the widget has no selection, else answer the
currently selected text

[]{#index-selectionRange-1}

**selectionRange**

Answer nil if the widget has no selection, else answer an Interval
object whose first item is the index of the first character in the
selection, and whose last item is the index of the character just after
the last one in the selection.

[]{#index-space}

**space**

Clear the selection and append a space at the end of the widget.

------------------------------------------------------------------------

::: header
Previous:
[BLOX.BEdit-accessing](BLOX_002eBEdit_002daccessing.html#BLOX_002eBEdit_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BEdit](BLOX_002eBEdit.html#BLOX_002eBEdit){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
