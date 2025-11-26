[]{#BLOX_002eBDropDown_002dflexibility}

::: header
Next: [BLOX.BDropDown-list box
accessing](BLOX_002eBDropDown_002dlist-box-accessing.html#BLOX_002eBDropDown_002dlist-box-accessing){accesskey="n"
rel="next"}, Previous:
[BLOX.BDropDown-callbacks](BLOX_002eBDropDown_002dcallbacks.html#BLOX_002eBDropDown_002dcallbacks){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDown_003a-flexibility}

#### 1.12.3 BLOX.BDropDown: flexibility {#blox.bdropdown-flexibility .subsection}

[]{#index-createList}

**createList**

Create the popup widget to be used for the 'drop-down list'. It is a
BList by default, but you can use any other widget, overriding the 'list
box accessing' methods if necessary.

[]{#index-createTextWidget}

**createTextWidget**

Create the widget that will hold the string chosen from the list box and
answer it. The widget must be a child of 'self primitive'.

[]{#index-itemHeight} []{#index-font-8}

**itemHeight**

Answer the height of an item in the drop-down list. The default
implementation assumes that the receiver understands #font, but you can
modify it if you want.

[]{#index-listCallback}

**listCallback**

Called when an item of the listbox is highlighted. Do nothing by default

[]{#index-listSelectAt_003a}

**listSelectAt: aPoint**

Select the item lying at the given position in the list box. The default
implementation assumes that list is a BList, but you can modify it if
you want.

[]{#index-listText}

**listText**

Answer the text currently chosen in the list box. The default
implementation assumes that list is a BList, but you can modify it if
you want.

[]{#index-text-1}

**text**

Answer the text that the user has picked from the widget and/or typed in
the control (the exact way the text is entered will be established by
subclasses, since this is an abstract method).

[]{#index-text_003a-1}

**text: aString**

Set the text widget to aString
