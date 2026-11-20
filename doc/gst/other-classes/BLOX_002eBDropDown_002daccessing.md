[]{#BLOX_002eBDropDown_002daccessing}

::: header
Next:
[BLOX.BDropDown-callbacks](BLOX_002eBDropDown_002dcallbacks.html#BLOX_002eBDropDown_002dcallbacks){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDown_003a-accessing}

#### 1.12.1 BLOX.BDropDown: accessing {#blox.bdropdown-accessing .subsection}

[]{#index-backgroundColor-2}

**backgroundColor**

Answer the value of the backgroundColor for the widget, which in this
class is only set for the list widget (that is, the pop-up widget).
Subclasses should override this method so that the color is set properly
for the text widget as well.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a-2}

**backgroundColor: aColor**

Set the value of the backgroundColor for the widget, which in this class
is only set for the list widget (that is, the pop-up widget). Subclasses
should override this method so that the color is set properly for the
text widget as well.

Specifies the normal background color to use when displaying the widget.

[]{#index-droppedRows}

**droppedRows**

Answer the number of items that are visible at any time in the listbox.

[]{#index-droppedRows_003a}

**droppedRows: anInteger**

Set the number of items that are visible at any time in the listbox.

[]{#index-font-1}

**font**

Answer the value of the font option for the widget, which in this class
is only set for the list widget (that is, the pop-up widget). Subclasses
should override this method so that the color is set properly for the
text widget as well.

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

[]{#index-font_003a-1}

**font: value**

Set the value of the font option for the widget, which in this class is
only set for the list widget (that is, the pop-up widget). Subclasses
should override this method so that the color is set properly for the
text widget as well.

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

[]{#index-foregroundColor-2}

**foregroundColor**

Answer the value of the foregroundColor for the widget, which in this
class is only set for the list widget (that is, the pop-up widget).
Subclasses should override this method so that the color is set properly
for the text widget as well.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a-2}

**foregroundColor: aColor**

Set the value of the foregroundColor for the widget, which in this class
is only set for the list widget (that is, the pop-up widget). Subclasses
should override this method so that the color is set properly for the
text widget as well.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-highlightBackground}

**highlightBackground**

Answer the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the list widget.

[]{#index-highlightBackground_003a}

**highlightBackground: aColor**

Set the value of the highlightBackground option for the widget.

Specifies the background color to use when displaying selected items in
the list widget.

[]{#index-highlightForeground}

**highlightForeground**

Answer the value of the highlightForeground option for the widget.

Specifies the foreground color to use when displaying selected items in
the list widget.

[]{#index-highlightForeground_003a}

**highlightForeground: aColor**

Set the value of the highlightForeground option for the widget.

Specifies the foreground color to use when displaying selected items in
the list widget.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BDropDown-callbacks](BLOX_002eBDropDown_002dcallbacks.html#BLOX_002eBDropDown_002dcallbacks){accesskey="n"
rel="next"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
