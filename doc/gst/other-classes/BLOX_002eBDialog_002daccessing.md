[]{#BLOX_002eBDialog_002daccessing}

::: header
Next: [BLOX.BDialog-widget
protocol](BLOX_002eBDialog_002dwidget-protocol.html#BLOX_002eBDialog_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BDialog
class-prompters](BLOX_002eBDialog-class_002dprompters.html#BLOX_002eBDialog-class_002dprompters){accesskey="p"
rel="prev"}, Up:
[BLOX.BDialog](BLOX_002eBDialog.html#BLOX_002eBDialog){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDialog_003a-accessing}

#### 1.11.3 BLOX.BDialog: accessing {#blox.bdialog-accessing .subsection}

[]{#index-addButton_003areceiver_003aindex_003a}
[]{#index-dispatch_003a}

**addButton: aLabel receiver: anObject index: anInt**

Add a button to the dialog box that, when clicked, will cause the
#dispatch: method to be triggered in anObject, passing anInt as the
argument of the callback. The caption of the button is set to aLabel.

[]{#index-addButton_003areceiver_003amessage_003a}

**addButton: aLabel receiver: anObject message: aSymbol**

Add a button to the dialog box that, when clicked, will cause the
aSymbol unary selector to be sent to anObject. The caption of the button
is set to aLabel.

[]{#index-addButton_003areceiver_003amessage_003aargument_003a}

**addButton: aLabel receiver: anObject message: aSymbol argument: arg**

Add a button to the dialog box that, when clicked, will cause the
aSymbol one-argument selector to be sent to anObject, passing arg as the
argument of the callback. The caption of the button is set to aLabel.

[]{#index-contents}

**contents**

Answer the text that is displayed in the entry widget associated to the
dialog box.

[]{#index-contents_003a}

**contents: newText**

Display newText in the entry widget associated to the dialog box.
