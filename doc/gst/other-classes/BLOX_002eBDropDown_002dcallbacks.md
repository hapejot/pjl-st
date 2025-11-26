[]{#BLOX_002eBDropDown_002dcallbacks}

::: header
Next:
[BLOX.BDropDown-flexibility](BLOX_002eBDropDown_002dflexibility.html#BLOX_002eBDropDown_002dflexibility){accesskey="n"
rel="next"}, Previous:
[BLOX.BDropDown-accessing](BLOX_002eBDropDown_002daccessing.html#BLOX_002eBDropDown_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDown_003a-callbacks}

#### 1.12.2 BLOX.BDropDown: callbacks {#blox.bdropdown-callbacks .subsection}

[]{#index-callback-2}

**callback**

Answer a DirectedMessage that is sent when the receiver is clicked, or
nil if none has been set up.

[]{#index-callback_003amessage_003a-2}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is clicked. If the
method accepts an argument, the receiver is passed.

[]{#index-invokeCallback-3}

**invokeCallback**

Generate a synthetic callback
