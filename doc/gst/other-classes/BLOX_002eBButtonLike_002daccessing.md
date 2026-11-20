[]{#BLOX_002eBButtonLike_002daccessing}

::: header
Up:
[BLOX.BButtonLike](BLOX_002eBButtonLike.html#BLOX_002eBButtonLike){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBButtonLike_003a-accessing}

#### 1.5.1 BLOX.BButtonLike: accessing {#blox.bbuttonlike-accessing .subsection}

[]{#index-callback-1}

**callback**

Answer a DirectedMessage that is sent when the receiver is clicked, or
nil if none has been set up.

[]{#index-callback_003amessage_003a-1}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is clicked. If the
method accepts an argument, the receiver is passed.

[]{#index-invokeCallback-1}

**invokeCallback**

Generate a synthetic callback

[]{#index-pressed}

**pressed**

This is the default callback for the widget; it does nothing if you
don't override it. Of course if a subclass overriddes this you (user of
the class) might desire to call this method from your own callback.
