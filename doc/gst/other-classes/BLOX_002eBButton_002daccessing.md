[]{#BLOX_002eBButton_002daccessing}

::: header
Previous: [BLOX.BButton class-instance
creation](BLOX_002eBButton-class_002dinstance-creation.html#BLOX_002eBButton-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BButton](BLOX_002eBButton.html#BLOX_002eBButton){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBButton_003a-accessing}

#### 1.4.2 BLOX.BButton: accessing {#blox.bbutton-accessing .subsection}

[]{#index-backgroundColor}

**backgroundColor**

Answer the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a}

**backgroundColor: value**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-callback}

**callback**

Answer a DirectedMessage that is sent when the receiver is clicked, or
nil if none has been set up.

[]{#index-callback_003amessage_003a}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is clicked. If the
method accepts an argument, the receiver is passed.

[]{#index-font}

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

[]{#index-font_003a}

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

[]{#index-foregroundColor}

**foregroundColor**

Answer the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a}

**foregroundColor: value**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-invokeCallback}

**invokeCallback**

Generate a synthetic callback

[]{#index-label}

**label**

Answer the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

[]{#index-label_003a}

**label: value**

Set the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

------------------------------------------------------------------------

::: header
Previous: [BLOX.BButton class-instance
creation](BLOX_002eBButton-class_002dinstance-creation.html#BLOX_002eBButton-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BButton](BLOX_002eBButton.html#BLOX_002eBButton){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
