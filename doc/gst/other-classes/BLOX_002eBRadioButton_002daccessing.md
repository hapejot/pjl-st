[]{#BLOX_002eBRadioButton_002daccessing}

::: header
Up:
[BLOX.BRadioButton](BLOX_002eBRadioButton.html#BLOX_002eBRadioButton){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBRadioButton_003a-accessing}

#### 1.37.1 BLOX.BRadioButton: accessing {#blox.bradiobutton-accessing .subsection}

[]{#index-callback_003amessage_003a-7}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
selector accepting at most two arguments) when the receiver is clicked.
If the method accepts two arguments, the receiver is passed as the first
parameter. If the method accepts one or two arguments, true is passed as
the last parameter for interoperability with BToggle widgets.

[]{#index-value-2}

**value**

Answer whether this widget is the selected one in its radio button
group.

[]{#index-value_003a-2}

**value: aBoolean**

Answer whether this widget is the selected one in its radio button
group. Setting this property to false for a group's currently selected
button unhighlights all the buttons in that group.
