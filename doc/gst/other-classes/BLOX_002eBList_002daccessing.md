[]{#BLOX_002eBList_002daccessing}

::: header
Next: [BLOX.BList-widget
protocol](BLOX_002eBList_002dwidget-protocol.html#BLOX_002eBList_002dwidget-protocol){accesskey="n"
rel="next"}, Up:
[BLOX.BList](BLOX_002eBList.html#BLOX_002eBList){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBList_003a-accessing}

#### 1.25.1 BLOX.BList: accessing {#blox.blist-accessing .subsection}

[]{#index-add_003aafterIndex_003a-1}

**add: anObject afterIndex: index**

Add an element with the given value after another element whose index is
contained in the index parameter. The label displayed in the widget is
anObject's displayString. Answer anObject.

[]{#index-add_003aelement_003aafterIndex_003a-1}

**add: aString element: anObject afterIndex: index**

Add an element with the aString label after another element whose index
is contained in the index parameter. This method allows the client to
decide autonomously the label that the widget will display.

If anObject is nil, then string is used as the element as well. If
aString is nil, then the element's displayString is used as the label.

Answer anObject or, if it is nil, aString.

[]{#index-addLast_003a-1}

**addLast: anObject**

Add an element with the given value at the end of the listbox. The label
displayed in the widget is anObject's displayString. Answer anObject.

[]{#index-addLast_003aelement_003a-1}

**addLast: aString element: anObject**

Add an element with the given value at the end of the listbox. This
method allows the client to decide autonomously the label that the
widget will display.

If anObject is nil, then string is used as the element as well. If
aString is nil, then the element's displayString is used as the label.

Answer anObject or, if it is nil, aString.

[]{#index-associationAt_003a-1}

**associationAt: anIndex**

Answer an association whose key is the item at the given position in the
listbox and whose value is the label used to display that item.

[]{#index-at_003a-2}

**at: anIndex**

Answer the element displayed at the given position in the list box.

[]{#index-backgroundColor-7}

**backgroundColor**

Answer the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a-9}

**backgroundColor: value**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-contents_003a-3}

**contents: elementList**

Set the elements displayed in the listbox, and set the labels to be
their displayStrings.

[]{#index-contents_003aelements_003a-1}

**contents: stringCollection elements: elementList**

Set the elements displayed in the listbox to be those in elementList,
and set the labels to be the corresponding elements in stringCollection.
The two collections must have the same size.

[]{#index-do_003a-2}

**do: aBlock**

Iterate over each element of the listbox and pass it to aBlock.

[]{#index-elements}

**elements**

Answer the collection of objects that represent the elements displayed
by the list box.

[]{#index-elements_003a-1}

**elements: elementList**

Set the elements displayed in the listbox, and set the labels to be
their displayStrings.

[]{#index-font-5}

**font**

Answer the value of the font option for the widget.

Specifies the font to use when drawing text inside the widget. The font
can be given as either an X font name or a Blox font description string.

X font names are given as many fields, each led by a minus, and each of
which can be replaced by an \* to indicate a default value is ok:
foundry, family, weight, slant, setwidth, addstyle, pixel size, point
size (the same as pixel size for historical reasons), horizontal
resolution, vertical resolution, spacing, width, charset and character
encoding.

Blox font description strings have three fields, which must be separated
by a space and of which only the first is mandatory: the font family,
the font size in points (or in pixels if a negative value is supplied),
and a number of styles separated by a space (valid styles are normal,
bold, italic, underline and overstrike). Examples of valid fonts are
"Helvetica 10 Bold", "Times -14", "Futura Bold Underline". You must
enclose the font family in braces if it is made of two or more words.

[]{#index-font_003a-7}

**font: value**

Set the value of the font option for the widget.

Specifies the font to use when drawing text inside the widget. The font
can be given as either an X font name or a Blox font description string.

X font names are given as many fields, each led by a minus, and each of
which can be replaced by an \* to indicate a default value is ok:
foundry, family, weight, slant, setwidth, addstyle, pixel size, point
size (the same as pixel size for historical reasons), horizontal
resolution, vertical resolution, spacing, width, charset and character
encoding.

Blox font description strings have three fields, which must be separated
by a space and of which only the first is mandatory: the font family,
the font size in points (or in pixels if a negative value is supplied),
and a number of styles separated by a space (valid styles are normal,
bold, italic, underline and overstrike). Examples of valid fonts are
"Helvetica 10 Bold", "Times -14", "Futura Bold Underline". You must
enclose the font family in braces if it is made of two or more words.

[]{#index-foregroundColor-6}

**foregroundColor**

Answer the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a-8}

**foregroundColor: value**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-highlightBackground-1}

**highlightBackground**

Answer the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the widget.

[]{#index-highlightBackground_003a-3}

**highlightBackground: value**

Set the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the widget.

[]{#index-highlightForeground-1}

**highlightForeground**

Answer the value of the highlightForeground option for the widget.

Specifies the foreground color to use when displaying selected items in
the widget.

[]{#index-highlightForeground_003a-3}

**highlightForeground: value**

Set the value of the highlightForeground option for the widget.

Specifies the foreground color to use when displaying selected items in
the widget.

[]{#index-index-1}

**index**

Answer the value of the index option for the widget.

Indicates the element that has the location cursor. This item will be
displayed in the highlightForeground color, and with the corresponding
background color.

[]{#index-indexAt_003a}

**indexAt: point**

Answer the index of the element that covers the point in the listbox
window specified by x and y (in pixel coordinates). If no element covers
that point, then the closest element to that point is used.

[]{#index-isSelected_003a}

**isSelected: index**

Answer whether the element indicated by index is currently selected.

[]{#index-label-2}

**label**

Return nil, it is here for Gtk+ support

[]{#index-label_003a-2}

**label: aString**

Do nothing, it is here for Gtk+ support

[]{#index-labelAt_003a-1}

**labelAt: anIndex**

Answer the label displayed at the given position in the list box.

[]{#index-labels}

**labels**

Answer the labels displayed by the list box.

[]{#index-labelsDo_003a-1}

**labelsDo: aBlock**

Iterate over each listbox element's label and pass it to aBlock.

[]{#index-mode}

**mode**

Answer the value of the mode option for the widget.

Specifies one of several styles for manipulating the selection. The
value of the option may be either single, browse, multiple, or extended.

If the selection mode is single or browse, at most one element can be
selected in the listbox at once. Clicking button 1 on an unselected
element selects it and deselects any other selected item, while clicking
on a selected element has no effect. In browse mode it is also possible
to drag the selection with button 1. That is, moving the mouse while
button 1 is pressed keeps the item under the cursor selected.

If the selection mode is multiple or extended, any number of elements
may be selected at once, including discontiguous ranges. In multiple
mode, clicking button 1 on an element toggles its selection state
without affecting any other elements. In extended mode, pressing button
1 on an element selects it, deselects everything else, and sets the
anchor to the element under the mouse; dragging the mouse with button 1
down extends the selection to include all the elements between the
anchor and the element under the mouse, inclusive.

In extended mode, the selected range can be adjusted by pressing button
1 with the Shift key down: this modifies the selection to consist of the
elements between the anchor and the element under the mouse, inclusive.
The un-anchored end of this new selection can also be dragged with the
button down. Also in extended mode, pressing button 1 with the Control
key down starts a toggle operation: the anchor is set to the element
under the mouse, and its selection state is reversed. The selection
state of other elements is not changed. If the mouse is dragged with
button 1 down, then the selection state of all elements between the
anchor and the element under the mouse is set to match that of the
anchor element; the selection state of all other elements remains what
it was before the toggle operation began.

Most people will probably want to use browse mode for single selections
and extended mode for multiple selections; the other modes appear to be
useful only in special situations.

[]{#index-mode_003a}

**mode: value**

Set the value of the mode option for the widget.

Specifies one of several styles for manipulating the selection. The
value of the option may be either single, browse, multiple, or extended.

If the selection mode is single or browse, at most one element can be
selected in the listbox at once. Clicking button 1 on an unselected
element selects it and deselects any other selected item, while clicking
on a selected element has no effect. In browse mode it is also possible
to drag the selection with button 1. That is, moving the mouse while
button 1 is pressed keeps the item under the cursor selected.

If the selection mode is multiple or extended, any number of elements
may be selected at once, including discontiguous ranges. In multiple
mode, clicking button 1 on an element toggles its selection state
without affecting any other elements. In extended mode, pressing button
1 on an element selects it, deselects everything else, and sets the
anchor to the element under the mouse; dragging the mouse with button 1
down extends the selection to include all the elements between the
anchor and the element under the mouse, inclusive.

In extended mode, the selected range can be adjusted by pressing button
1 with the Shift key down: this modifies the selection to consist of the
elements between the anchor and the element under the mouse, inclusive.
The un-anchored end of this new selection can also be dragged with the
button down. Also in extended mode, pressing button 1 with the Control
key down starts a toggle operation: the anchor is set to the element
under the mouse, and its selection state is reversed. The selection
state of other elements is not changed. If the mouse is dragged with
button 1 down, then the selection state of all elements between the
anchor and the element under the mouse is set to match that of the
anchor element; the selection state of all other elements remains what
it was before the toggle operation began.

Most people will probably want to use browse mode for single selections
and extended mode for multiple selections; the other modes appear to be
useful only in special situations.

[]{#index-numberOfStrings-1}

**numberOfStrings**

Answer the number of items in the list box

[]{#index-removeAtIndex_003a-1} []{#index-at_003a-8}

**removeAtIndex: index**

Remove the item at the given index in the list box, answering the object
associated to the element (i.e. the value that #at: would have returned
for the given index)

[]{#index-size-1}

**size**

Answer the number of items in the list box

------------------------------------------------------------------------

::: header
Next: [BLOX.BList-widget
protocol](BLOX_002eBList_002dwidget-protocol.html#BLOX_002eBList_002dwidget-protocol){accesskey="n"
rel="next"}, Up:
[BLOX.BList](BLOX_002eBList.html#BLOX_002eBList){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
