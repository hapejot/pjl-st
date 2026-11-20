[]{#BLOX_002eBMenuObject_002dcallback}

::: header
Previous:
[BLOX.BMenuObject-accessing](BLOX_002eBMenuObject_002daccessing.html#BLOX_002eBMenuObject_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BMenuObject](BLOX_002eBMenuObject.html#BLOX_002eBMenuObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBMenuObject_003a-callback}

#### 1.30.2 BLOX.BMenuObject: callback {#blox.bmenuobject-callback .subsection}

[]{#index-callback-5}

**callback**

Answer a DirectedMessage that is sent when the receiver is modified, or
nil if none has been set up.

[]{#index-callback_003amessage_003a-6}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is clicked. If the
method accepts an argument, the receiver is passed.

[]{#index-callback_003amessage_003aargument_003a}

**callback: aReceiver message: aSymbol argument: anObject**

Set up so that aReceiver is sent the aSymbol message (the name of a one-
or two-argument selector) when the receiver is clicked. If the method
accepts two argument, the receiver is passed together with anObject; if
it accepts a single one, instead, only anObject is passed.

[]{#index-invokeCallback-7}

**invokeCallback**

Generate a synthetic callback
