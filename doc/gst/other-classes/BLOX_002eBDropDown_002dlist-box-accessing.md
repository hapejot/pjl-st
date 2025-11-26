[]{#BLOX_002eBDropDown_002dlist-box-accessing}

::: header
Next: [BLOX.BDropDown-widget
protocol](BLOX_002eBDropDown_002dwidget-protocol.html#BLOX_002eBDropDown_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BDropDown-flexibility](BLOX_002eBDropDown_002dflexibility.html#BLOX_002eBDropDown_002dflexibility){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDown_003a-list-box-accessing}

#### 1.12.4 BLOX.BDropDown: list box accessing {#blox.bdropdown-list-box-accessing .subsection}

[]{#index-add_003aafterIndex_003a}

**add: anObject afterIndex: index**

Add an element with the given value after another element whose index is
contained in the index parameter. The label displayed in the widget is
anObject's displayString. Answer anObject.

[]{#index-add_003aelement_003aafterIndex_003a}

**add: aString element: anObject afterIndex: index**

Add an element with the aString label after another element whose index
is contained in the index parameter. This method allows the client to
decide autonomously the label that the widget will display.

If anObject is nil, then string is used as the element as well. If
aString is nil, then the element's displayString is used as the label.

Answer anObject or, if it is nil, aString.

[]{#index-addLast_003a}

**addLast: anObject**

Add an element with the given value at the end of the listbox. The label
displayed in the widget is anObject's displayString. Answer anObject.

[]{#index-addLast_003aelement_003a}

**addLast: aString element: anObject**

Add an element with the given value at the end of the listbox. This
method allows the client to decide autonomously the label that the
widget will display.

If anObject is nil, then string is used as the element as well. If
aString is nil, then the element's displayString is used as the label.

Answer anObject or, if it is nil, aString.

[]{#index-associationAt_003a}

**associationAt: anIndex**

Answer an association whose key is the item at the given position in the
listbox and whose value is the label used to display that item.

[]{#index-at_003a-1}

**at: anIndex**

Answer the element displayed at the given position in the list box.

[]{#index-contents_003a-1}

**contents: stringCollection**

Set the elements displayed in the listbox, and set the labels to be
their displayStrings.

[]{#index-contents_003aelements_003a}

**contents: stringCollection elements: elementList**

Set the elements displayed in the listbox to be those in elementList,
and set the labels to be the corresponding elements in stringCollection.
The two collections must have the same size.

[]{#index-do_003a-1}

**do: aBlock**

Iterate over each element of the listbox and pass it to aBlock.

[]{#index-elements_003a}

**elements: elementList**

Set the elements displayed in the listbox, and set the labels to be
their displayStrings.

[]{#index-index_003a}

**index: newIndex**

Highlight the item at the given position in the listbox, and transfer
the text in the list box to the text widget.

[]{#index-labelAt_003a}

**labelAt: anIndex**

Answer the label displayed at the given position in the list box.

[]{#index-labelsDo_003a}

**labelsDo: aBlock**

Iterate over the labels in the list widget and pass each of them to
aBlock.

[]{#index-numberOfStrings}

**numberOfStrings**

Answer the number of items in the list box

[]{#index-removeAtIndex_003a} []{#index-at_003a-7}

**removeAtIndex: index**

Remove the item at the given index in the list box, answering the object
associated to the element (i.e. the value that #at: would have returned
for the given index)

[]{#index-size}

**size**

Answer the number of items in the list box

------------------------------------------------------------------------

::: header
Next: [BLOX.BDropDown-widget
protocol](BLOX_002eBDropDown_002dwidget-protocol.html#BLOX_002eBDropDown_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BDropDown-flexibility](BLOX_002eBDropDown_002dflexibility.html#BLOX_002eBDropDown_002dflexibility){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
