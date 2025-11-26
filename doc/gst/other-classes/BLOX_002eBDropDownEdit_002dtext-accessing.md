[]{#BLOX_002eBDropDownEdit_002dtext-accessing}

::: header
Previous:
[BLOX.BDropDownEdit-accessing-overrides](BLOX_002eBDropDownEdit_002daccessing_002doverrides.html#BLOX_002eBDropDownEdit_002daccessing_002doverrides){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDownEdit](BLOX_002eBDropDownEdit.html#BLOX_002eBDropDownEdit){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDownEdit_003a-text-accessing}

#### 1.13.3 BLOX.BDropDownEdit: text accessing {#blox.bdropdownedit-text-accessing .subsection}

[]{#index-insertAtEnd_003a}

**insertAtEnd: aString**

Clear the selection and append aString at the end of the text widget.

[]{#index-replaceSelection_003a}

**replaceSelection: aString**

Insert aString in the text widget at the current insertion point,
replacing the currently selected text (if any), and leaving the text
selected.

[]{#index-selectAll}

**selectAll**

Select the whole contents of the text widget

[]{#index-selectFrom_003ato_003a}

**selectFrom: first to: last**

Sets the selection of the text widget to include the characters starting
with the one indexed by first (the very first character in the widget
having index 1) and ending with the one just before last. If last refers
to the same character as first or an earlier one, then the text widget's
selection is cleared.

[]{#index-selection}

**selection**

Answer an empty string if the text widget has no selection, else answer
the currently selected text

[]{#index-selectionRange}

**selectionRange**

Answer nil if the text widget has no selection, else answer an Interval
object whose first item is the index of the first character in the
selection, and whose last item is the index of the character just after
the last one in the selection.

[]{#index-text_003a-2}

**text: aString**

Set the contents of the text widget and select them.
