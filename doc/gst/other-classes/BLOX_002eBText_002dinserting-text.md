[]{#BLOX_002eBText_002dinserting-text}

::: header
Next: [BLOX.BText-position &
lines](BLOX_002eBText_002dposition-_0026-lines.html#BLOX_002eBText_002dposition-_0026-lines){accesskey="n"
rel="next"}, Previous:
[BLOX.BText-images](BLOX_002eBText_002dimages.html#BLOX_002eBText_002dimages){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBText_003a-inserting-text}

#### 1.42.7 BLOX.BText: inserting text {#blox.btext-inserting-text .subsection}

[]{#index-insertAtEnd_003a-2}

**insertAtEnd: aString**

Clear the selection and append aString at the end of the widget.

[]{#index-insertSelectedText_003a}

**insertSelectedText: aString**

Insert aString in the widget at the current insertion point, leaving the
currently selected text (if any) in place, and selecting the text.

[]{#index-insertText_003a-1}

**insertText: aString**

Insert aString in the widget at the current insertion point, replacing
the currently selected text (if any).

[]{#index-insertText_003aat_003a}

**insertText: aString at: position**

Insert aString in the widget at the given position, replacing the
currently selected text (if any). The position is a Point object in
which both coordinates are 1-based: the first line is line 1, and the
first character in the first line is character 1.

[]{#index-insertTextSelection_003a}

**insertTextSelection: aString**

Insert aString in the widget after the current selection, leaving the
currently selected text (if any) intact.

[]{#index-invokeCallback-8}

**invokeCallback**

Generate a synthetic callback.

[]{#index-nextPut_003a-1}

**nextPut: aCharacter**

Clear the selection and append aCharacter at the end of the widget.

[]{#index-nextPutAll_003a-1}

**nextPutAll: aString**

Clear the selection and append aString at the end of the widget.

[]{#index-nl-1}

**nl**

Clear the selection and append a linefeed character at the end of the
widget.

[]{#index-refuseTabs}

**refuseTabs**

Arrange so that Tab characters, instead of being inserted in the widget,
traverse the widgets in the parent window.

[]{#index-replaceSelection_003a-2}

**replaceSelection: aString**

Insert aString in the widget at the current insertion point, replacing
the currently selected text (if any), and leaving the text selected.

[]{#index-searchString_003a}

**searchString: aString**

Search aString in the widget. If it is not found, answer zero, else
answer the 1-based line number and move the insertion point to the place
where the string was found.

[]{#index-space-1}

**space**

Clear the selection and append a space at the end of the widget.

------------------------------------------------------------------------

::: header
Next: [BLOX.BText-position &
lines](BLOX_002eBText_002dposition-_0026-lines.html#BLOX_002eBText_002dposition-_0026-lines){accesskey="n"
rel="next"}, Previous:
[BLOX.BText-images](BLOX_002eBText_002dimages.html#BLOX_002eBText_002dimages){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
