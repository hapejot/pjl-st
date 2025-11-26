[]{#BLOX_002eBToggle_002daccessing}

::: header
Up: [BLOX.BToggle](BLOX_002eBToggle.html#BLOX_002eBToggle){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBToggle_003a-accessing}

#### 1.46.1 BLOX.BToggle: accessing {#blox.btoggle-accessing .subsection}

[]{#index-callback_003amessage_003a-9}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
selector accepting at most two arguments) when the receiver is clicked.
If the method accepts two arguments, the receiver is passed as the first
parameter. If the method accepts one or two arguments, the state of the
widget (true if it is selected, false if it is not) is passed as the
last parameter.

[]{#index-invokeCallback-9}

**invokeCallback**

Generate a synthetic callback.

[]{#index-value-4}

**value**

Answer whether the button is in a selected (checked) state.

[]{#index-value_003a-4}

**value: aBoolean**

Set whether the button is in a selected (checked) state and generates a
callback accordingly.

[]{#index-variable_003a}

**variable: value**

Set the value of Tk's variable option for the widget.
