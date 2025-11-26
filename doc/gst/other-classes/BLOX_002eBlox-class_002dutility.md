[]{#BLOX_002eBlox-class_002dutility}

::: header
Next:
[BLOX.Blox-accessing](BLOX_002eBlox_002daccessing.html#BLOX_002eBlox_002daccessing){accesskey="n"
rel="next"}, Previous: [BLOX.Blox class-instance
creation](BLOX_002eBlox-class_002dinstance-creation.html#BLOX_002eBlox-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox-class_003a-utility}

#### 1.26.4 BLOX.Blox class: utility {#blox.blox-class-utility .subsection}

[]{#index-active}

**active**

Answer the currently active Blox, or nil if the focus does not belong to
a Smalltalk window.

[]{#index-at_003a-3}

**at: aPoint**

Answer the Blox containing the given point on the screen, or nil if no
Blox contains the given point (either because no Smalltalk window is
there or because it is covered by another window).

[]{#index-atMouse}

**atMouse**

Answer the Blox under the mouse cursor's hot spot, or nil if no Blox
contains the given point (either because no Smalltalk window is there or
because it is covered by another window).

[]{#index-beep}

**beep**

Produce a bell

[]{#index-clearClipboard}

**clearClipboard**

Clear the clipboard, answer its old contents.

[]{#index-clipboard}

**clipboard**

Retrieve the text in the clipboard.

[]{#index-clipboard_003a}

**clipboard: aString**

Set the contents of the clipboard to aString (or empty the clipboard if
aString is nil).

[]{#index-createColor_003agreen_003ablue_003a}

**createColor: red green: green blue: blue**

Answer a color that can be passed to methods such as 'backgroundColor:'.
The color will have the given RGB components (range is 0\~65535).

[]{#index-createColor_003amagenta_003ayellow_003a}

**createColor: cyan magenta: magenta yellow: yellow**

Answer a color that can be passed to methods such as 'backgroundColor:'.
The color will have the given CMY components (range is 0\~65535).

[]{#index-createColor_003amagenta_003ayellow_003ablack_003a}

**createColor: cyan magenta: magenta yellow: yellow black: black**

Answer a color that can be passed to methods such as 'backgroundColor:'.
The color will have the given CMYK components (range is 0\~65535).

[]{#index-createColor_003asaturation_003avalue_003a}

**createColor: hue saturation: sat value: value**

Answer a color that can be passed to methods such as 'backgroundColor:'.
The color will have the given HSV components (range is 0\~65535).

[]{#index-defaultFont}

**defaultFont**

Answer the default font used by Blox.

[]{#index-fonts}

**fonts**

Answer the names of the font families in the system. Additionally,
'Times', 'Courier' and 'Helvetica' are always made available.

[]{#index-mousePointer}

**mousePointer**

If the mouse pointer is on the same screen as the application's windows,
returns a Point containing the pointer's x and y coordinates measured in
pixels in the screen's root window (under X, if a virtual root window is
in use on the screen, the position is computed in the whole desktop, not
relative to the top-left corner of the currently shown portion). If the
mouse pointer isn't on the same screen as window then answer nil.

[]{#index-platform} []{#index-unix} []{#index-macintosh}
[]{#index-windows}

**platform**

Answer the platform on which Blox is running; it can be either #unix,
#macintosh or #windows.

[]{#index-screenOrigin}

**screenOrigin**

Answer a Point indicating the coordinates of the upper left point of the
screen in the virtual root window on which the application's windows are
drawn (under Windows and the Macintosh, that's always 0 @ 0)

[]{#index-screenResolution}

**screenResolution**

Answer a Point containing the resolution in dots per inch of the screen,
in the x and y directions.

[]{#index-screenSize}

**screenSize**

Answer a Point containing the size of the virtual root window on which
the application's windows are drawn (under Windows and the Macintosh,
that's the size of the screen)

------------------------------------------------------------------------

::: header
Next:
[BLOX.Blox-accessing](BLOX_002eBlox_002daccessing.html#BLOX_002eBlox_002daccessing){accesskey="n"
rel="next"}, Previous: [BLOX.Blox class-instance
creation](BLOX_002eBlox-class_002dinstance-creation.html#BLOX_002eBlox-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
