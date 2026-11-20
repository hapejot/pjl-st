[]{#BLOX_002eBDropDownList_002dcallbacks}

::: header
Next: [BLOX.BDropDownList-list box
accessing](BLOX_002eBDropDownList_002dlist-box-accessing.html#BLOX_002eBDropDownList_002dlist-box-accessing){accesskey="n"
rel="next"}, Previous:
[BLOX.BDropDownList-accessing](BLOX_002eBDropDownList_002daccessing.html#BLOX_002eBDropDownList_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDownList](BLOX_002eBDropDownList.html#BLOX_002eBDropDownList){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDownList_003a-callbacks}

#### 1.14.2 BLOX.BDropDownList: callbacks {#blox.bdropdownlist-callbacks .subsection}

[]{#index-callback_003amessage_003a-3}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
selector with at most two arguemtnts) when the active item in the
receiver changegs. If the method accepts two arguments, the receiver is
passed as the first parameter. If the method accepts one or two
arguments, the selected index is passed as the last parameter.

[]{#index-invokeCallback-4}

**invokeCallback**

Generate a synthetic callback.
