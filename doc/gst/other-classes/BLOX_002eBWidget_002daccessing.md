[]{#BLOX_002eBWidget_002daccessing}

::: header
Next:
[BLOX.BWidget-customization](BLOX_002eBWidget_002dcustomization.html#BLOX_002eBWidget_002dcustomization){accesskey="n"
rel="next"}, Previous: [BLOX.BWidget
class-popups](BLOX_002eBWidget-class_002dpopups.html#BLOX_002eBWidget-class_002dpopups){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWidget_003a-accessing}

#### 1.49.2 BLOX.BWidget: accessing {#blox.bwidget-accessing .subsection}

[]{#index-borderWidth}

**borderWidth**

Answer the value of the borderWidth option for the widget.

Specifies a non-negative value indicating the width of the 3-D border to
draw around the outside of the widget (if such a border is being drawn;
the effect option typically determines this). The value may also be used
when drawing 3-D effects in the interior of the widget. The value is
measured in pixels.

[]{#index-borderWidth_003a}

**borderWidth: value**

Set the value of the borderWidth option for the widget.

Specifies a non-negative value indicating the width of the 3-D border to
draw around the outside of the widget (if such a border is being drawn;
the effect option typically determines this). The value may also be used
when drawing 3-D effects in the interior of the widget. The value is
measured in pixels.

[]{#index-cursor}

**cursor**

Answer the value of the cursor option for the widget.

Specifies the mouse cursor to be used for the widget. The value of the
option is given by the standard X cursor cursor, i.e., any of the names
defined in cursorcursor.h, without the leading XC\_.

[]{#index-cursor_003a}

**cursor: value**

Set the value of the cursor option for the widget.

Specifies the mouse cursor to be used for the widget. The value of the
option is given by the standard X cursor cursor, i.e., any of the names
defined in cursorcursor.h, without the leading XC\_.

[]{#index-effect}

**effect**

Answer the value of the effect option for the widget.

Specifies the effect desired for the widget's border. Acceptable values
are raised, sunken, flat, ridge, solid, and groove. The value indicates
how the interior of the widget should appear relative to its exterior;
for example, raised means the interior of the widget should appear to
protrude from the screen, relative to the exterior of the widget. Raised
and sunken give the traditional 3-D appearance (for example, that of
Xaw3D), while ridge and groove give a "chiseled" appearance like that of
Swing or GTK+'s Metal theme. Flat and solid are not 3-D.

[]{#index-effect_003a}

**effect: value**

Set the value of the effect option for the widget.

Specifies the effect desired for the widget's border. Acceptable values
are raised, sunken, flat, ridge, solid, and groove. The value indicates
how the interior of the widget should appear relative to its exterior;
for example, raised means the interior of the widget should appear to
protrude from the screen, relative to the exterior of the widget. Raised
and sunken give the traditional 3-D appearance (for example, that of
Xaw3D), while ridge and groove give a "chiseled" appearance like that of
Swing or GTK+'s Metal theme. Flat and solid are not 3-D.

[]{#index-tabStop}

**tabStop**

Answer the value of the tabStop option for the widget.

Determines whether the window accepts the focus during keyboard
traversal (e.g., Tab and Shift-Tab). Before setting the focus to a
window, Blox consults the value of the tabStop option. A value of false
means that the window should be skipped entirely during keyboard
traversal. true means that the window should receive the input focus as
long as it is viewable (it and all of its ancestors are mapped). If you
do not set this option, Blox makes the decision about whether or not to
focus on the window: the current algorithm is to skip the window if it
is disabled, it has no key bindings, or if it is not viewable. Of the
standard widgets, BForm, BContainer, BLabel and BImage have no key
bindings by default.

[]{#index-tabStop_003a}

**tabStop: value**

Set the value of the tabStop option for the widget.

Determines whether the window accepts the focus during keyboard
traversal (e.g., Tab and Shift-Tab). Before setting the focus to a
window, Blox consults the value of the tabStop option. A value of false
means that the window should be skipped entirely during keyboard
traversal. true means that the window should receive the input focus as
long as it is viewable (it and all of its ancestors are mapped). If you
do not set this option, Blox makes the decision about whether or not to
focus on the window: the current algorithm is to skip the window if it
is disabled, it has no key bindings, or if it is not viewable. Of the
standard widgets, BForm, BContainer, BLabel and BImage have no key
bindings by default.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BWidget-customization](BLOX_002eBWidget_002dcustomization.html#BLOX_002eBWidget_002dcustomization){accesskey="n"
rel="next"}, Previous: [BLOX.BWidget
class-popups](BLOX_002eBWidget-class_002dpopups.html#BLOX_002eBWidget-class_002dpopups){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
