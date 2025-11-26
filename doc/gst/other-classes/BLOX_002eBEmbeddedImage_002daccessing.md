[]{#BLOX_002eBEmbeddedImage_002daccessing}

::: header
Up:
[BLOX.BEmbeddedImage](BLOX_002eBEmbeddedImage.html#BLOX_002eBEmbeddedImage){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBEmbeddedImage_003a-accessing}

#### 1.16.1 BLOX.BEmbeddedImage: accessing {#blox.bembeddedimage-accessing .subsection}

[]{#index-copyInto_003a-1}

**copyInto: aBlox**

Answer a new BCanvasObject identical to this but displayed into another
canvas, newCanvas. The new instance is not created at the time it is
returned.

[]{#index-data}

**data**

Answer the data of the image. The result will be a String containing
image data either as Base-64 encoded GIF data, as XPM data, or as PPM
data.

[]{#index-data_003a}

**data: aString**

Set the data of the image. aString may contain the data either as
Base-64 encoded GIF data, as XPM data, or as PPM data. No changes are
visible until you toggle a redraw using the appropriate method.

[]{#index-redraw-1}

**redraw**

Force the object to be displayed in the parent canvas, creating it if it
has not been inserted for real in the parent, and refresh its position
and image data if it has changed.
