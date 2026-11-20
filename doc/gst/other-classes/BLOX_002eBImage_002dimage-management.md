[]{#BLOX_002eBImage_002dimage-management}

::: header
Next: [BLOX.BImage-widget
protocol](BLOX_002eBImage_002dwidget-protocol.html#BLOX_002eBImage_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BImage-accessing](BLOX_002eBImage_002daccessing.html#BLOX_002eBImage_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BImage](BLOX_002eBImage.html#BLOX_002eBImage){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBImage_003a-image-management}

#### 1.22.7 BLOX.BImage: image management {#blox.bimage-image-management .subsection}

[]{#index-blank}

**blank**

Blank the corresponding image

[]{#index-data_003a-1}

**data: aString**

Set the image to be drawn to aString, which can be a GIF in Base-64
representation or an X pixelmap.

[]{#index-dither}

**dither**

Recalculate the dithered image in the window where the image is
displayed. The dithering algorithm used in displaying images propagates
quantization errors from one pixel to its neighbors. If the image data
is supplied in pieces, the dithered image may not be exactly correct.
Normally the difference is not noticeable, but if it is a problem, this
command can be used to fix it.

[]{#index-fillFrom_003aextent_003acolor_003a}

**fillFrom: origin extent: extent color: color**

Fill a rectangle with the given origin and extent, using the given
color.

[]{#index-fillFrom_003ato_003acolor_003a}

**fillFrom: origin to: corner color: color**

Fill a rectangle between the given corners, using the given color.

[]{#index-fillRectangle_003acolor_003a}

**fillRectangle: rectangle color: color**

Fill a rectangle having the given bounding box, using the given color.

[]{#index-image_003a}

**image: aFileStream**

Read a GIF or XPM image from aFileStream. The whole contents of the file
are read, not only from the file position.

[]{#index-imageHeight}

**imageHeight**

Specifies the height of the image, in pixels. This option is useful
primarily in situations where you wish to build up the contents of the
image piece by piece. A value of zero (the default) allows the image to
expand or shrink vertically to fit the data stored in it.

[]{#index-imageWidth}

**imageWidth**

Specifies the width of the image, in pixels. This option is useful
primarily in situations where you wish to build up the contents of the
image piece by piece. A value of zero (the default) allows the image to
expand or shrink horizontally to fit the data stored in it.

[]{#index-lineFrom_003aextent_003acolor_003a}

**lineFrom: origin extent: extent color: color**

Draw a line with the given origin and extent, using the given color.

[]{#index-lineFrom_003ato_003acolor_003a}

**lineFrom: origin to: corner color: color**

This method's functionality has not been implemented yet.

[]{#index-lineFrom_003atoX_003acolor_003a}

**lineFrom: origin toX: endX color: color**

Draw an horizontal line between the given corners, using the given
color.

[]{#index-lineFrom_003atoY_003acolor_003a}

**lineFrom: origin toY: endY color: color**

Draw a vertical line between the given corners, using the given color.

[]{#index-lineInside_003acolor_003a}

**lineInside: rectangle color: color**

Draw a line having the given bounding box, using the given color.

------------------------------------------------------------------------

::: header
Next: [BLOX.BImage-widget
protocol](BLOX_002eBImage_002dwidget-protocol.html#BLOX_002eBImage_002dwidget-protocol){accesskey="n"
rel="next"}, Previous:
[BLOX.BImage-accessing](BLOX_002eBImage_002daccessing.html#BLOX_002eBImage_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BImage](BLOX_002eBImage.html#BLOX_002eBImage){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
