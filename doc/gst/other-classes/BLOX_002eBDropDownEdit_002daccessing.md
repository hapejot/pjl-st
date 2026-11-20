[]{#BLOX_002eBDropDownEdit_002daccessing}

::: header
Next:
[BLOX.BDropDownEdit-accessing-overrides](BLOX_002eBDropDownEdit_002daccessing_002doverrides.html#BLOX_002eBDropDownEdit_002daccessing_002doverrides){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDownEdit](BLOX_002eBDropDownEdit.html#BLOX_002eBDropDownEdit){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDownEdit_003a-accessing}

#### 1.13.1 BLOX.BDropDownEdit: accessing {#blox.bdropdownedit-accessing .subsection}

[]{#index-backgroundColor_003a-3}

**backgroundColor: aColor**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-font_003a-2}

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

[]{#index-foregroundColor_003a-3}

**foregroundColor: aColor**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-highlightBackground_003a-1}

**highlightBackground: aColor**

Set the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the list widget and the selection in the text widget.

[]{#index-highlightForeground_003a-1}

**highlightForeground: aColor**

Set the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the list widget and the selection in the text widget.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BDropDownEdit-accessing-overrides](BLOX_002eBDropDownEdit_002daccessing_002doverrides.html#BLOX_002eBDropDownEdit_002daccessing_002doverrides){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDownEdit](BLOX_002eBDropDownEdit.html#BLOX_002eBDropDownEdit){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
