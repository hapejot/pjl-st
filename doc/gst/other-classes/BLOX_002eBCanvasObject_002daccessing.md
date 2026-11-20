[]{#BLOX_002eBCanvasObject_002daccessing}

::: header
Next: [BLOX.BCanvasObject-widget
protocol](BLOX_002eBCanvasObject_002dwidget-protocol.html#BLOX_002eBCanvasObject_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BCanvasObject class-instance
creation](BLOX_002eBCanvasObject-class_002dinstance-creation.html#BLOX_002eBCanvasObject-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvasObject](BLOX_002eBCanvasObject.html#BLOX_002eBCanvasObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBCanvasObject_003a-accessing}

#### 1.7.2 BLOX.BCanvasObject: accessing {#blox.bcanvasobject-accessing .subsection}

[]{#index-blox}

**blox**

Answer the parent canvas of the receiver

[]{#index-boundingBox-2}

**boundingBox**

Answer a Rectangle enclosing all of the receiver

[]{#index-color}

**color**

Answer the color to be used to fill this item's area.

[]{#index-color_003a}

**color: color**

Set the color to be used to fill this item's area.

[]{#index-copyInto_003a}

**copyInto: newCanvas**

Answer a new BCanvasObject identical to this but displayed into another
canvas, newCanvas. The new instance is not created at the time it is
returned.

[]{#index-copyObject} []{#index-copy-1} []{#index-copyObject-1}

**copyObject**

Answer a new BCanvasObject identical to this. Unlike #copy, which merely
creates a new Smalltalk object with the same data and referring to the
same canvas item, the object created with #copyObject is physically
distinct from the original. The new instance is not created at the time
it is returned.

[]{#index-createCopy} []{#index-copy-2} []{#index-copyObject-2}

**createCopy**

Answer a new BCanvasObject identical to this. Unlike #copy, which merely
creates a new Smalltalk object with the same data and referring to the
same canvas item, the object created with #copyObject is physically
distinct from the original. The new instance has already been created at
the time it is returned.

[]{#index-createCopyInto_003a}

**createCopyInto: newCanvas**

Answer a new BCanvasObject identical to this but displayed into another
canvas, newCanvas. The new instance has already been created at the time
it is returned.

[]{#index-deepCopy}

**deepCopy**

It does not make sense to make a copy, because it would make data
inconsistent across different objects; so answer the receiver

[]{#index-grayOut}

**grayOut**

Apply a 50% gray stippling pattern to the object

[]{#index-shallowCopy}

**shallowCopy**

It does not make sense to make a copy, because it would make data
inconsistent across different objects; so answer the receiver

------------------------------------------------------------------------

::: header
Next: [BLOX.BCanvasObject-widget
protocol](BLOX_002eBCanvasObject_002dwidget-protocol.html#BLOX_002eBCanvasObject_002dwidget-protocol){accesskey="n"
rel="next"}, Previous: [BLOX.BCanvasObject class-instance
creation](BLOX_002eBCanvasObject-class_002dinstance-creation.html#BLOX_002eBCanvasObject-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BCanvasObject](BLOX_002eBCanvasObject.html#BLOX_002eBCanvasObject){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
