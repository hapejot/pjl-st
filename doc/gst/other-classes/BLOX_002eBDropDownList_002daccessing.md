[]{#BLOX_002eBDropDownList_002daccessing}

::: header
Next:
[BLOX.BDropDownList-callbacks](BLOX_002eBDropDownList_002dcallbacks.html#BLOX_002eBDropDownList_002dcallbacks){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDownList](BLOX_002eBDropDownList.html#BLOX_002eBDropDownList){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDownList_003a-accessing}

#### 1.14.1 BLOX.BDropDownList: accessing {#blox.bdropdownlist-accessing .subsection}

[]{#index-backgroundColor_003a-4}

**backgroundColor: aColor**

Set the value of the backgroundColor for the widget, which in this class
is set for the list widget and, when the focus is outside the control,
for the text widget as well.

Specifies the normal background color to use when displaying the widget.

[]{#index-font_003a-3}

**font: aString**

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

[]{#index-foregroundColor_003a-4}

**foregroundColor: aColor**

Set the value of the foregroundColor for the widget, which in this class
is set for the list widget and, when the focus is outside the control,
for the text widget as well.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-highlightBackground_003a-2}

**highlightBackground: aColor**

Answer the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the list widget and, when the focus is inside the control, for the text
widget as well.

[]{#index-highlightForeground_003a-2}

**highlightForeground: aColor**

Answer the value of the highlightForeground option for the widget.

Specifies the foreground color to use when displaying selected items in
the list widget and, when the focus is inside the control, for the text
widget as well.

[]{#index-text-3}

**text**

Answer the text that the user has picked from the widget and/or typed in
the control (the exact way the text is entered will be established by
subclasses, since this is an abstract method).

------------------------------------------------------------------------

::: header
Next:
[BLOX.BDropDownList-callbacks](BLOX_002eBDropDownList_002dcallbacks.html#BLOX_002eBDropDownList_002dcallbacks){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDownList](BLOX_002eBDropDownList.html#BLOX_002eBDropDownList){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
