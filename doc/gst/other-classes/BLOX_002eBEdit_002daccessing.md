[]{#BLOX_002eBEdit_002daccessing}

::: header
Next: [BLOX.BEdit-widget
protocol](BLOX_002eBEdit_002dwidget-protocol.html#BLOX_002eBEdit_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BEdit class-instance
creation](BLOX_002eBEdit-class_002dinstance-creation.html#BLOX_002eBEdit-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BEdit](BLOX_002eBEdit.html#BLOX_002eBEdit){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBEdit_003a-accessing}

#### 1.15.2 BLOX.BEdit: accessing {#blox.bedit-accessing .subsection}

[]{#index-backgroundColor-3}

**backgroundColor**

Answer the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a-5}

**backgroundColor: value**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-callback-3}

**callback**

Answer a DirectedMessage that is sent when the receiver is modified, or
nil if none has been set up.

[]{#index-callback_003amessage_003a-4}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is modified. If the
method accepts an argument, the receiver is passed.

[]{#index-contents-1}

**contents**

Return the contents of the widget

[]{#index-contents_003a-2}

**contents: newText**

Set the contents of the widget

[]{#index-font-2}

**font**

Answer the value of the font option for the widget.

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

[]{#index-font_003a-4}

**font: value**

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

[]{#index-foregroundColor-3}

**foregroundColor**

Answer the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a-5}

**foregroundColor: value**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-selectBackground}

**selectBackground**

Answer the value of the selectBackground option for the widget.

Specifies the background color to use when displaying selected parts of
the widget.

[]{#index-selectBackground_003a}

**selectBackground: value**

Set the value of the selectBackground option for the widget.

Specifies the background color to use when displaying selected parts of
the widget.

[]{#index-selectForeground}

**selectForeground**

Answer the value of the selectForeground option for the widget.

Specifies the foreground color to use when displaying selected parts of
the widget.

[]{#index-selectForeground_003a}

**selectForeground: value**

Set the value of the selectForeground option for the widget.

Specifies the foreground color to use when displaying selected parts of
the widget.

------------------------------------------------------------------------

::: header
Next: [BLOX.BEdit-widget
protocol](BLOX_002eBEdit_002dwidget-protocol.html#BLOX_002eBEdit_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BEdit class-instance
creation](BLOX_002eBEdit-class_002dinstance-creation.html#BLOX_002eBEdit-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BEdit](BLOX_002eBEdit.html#BLOX_002eBEdit){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
