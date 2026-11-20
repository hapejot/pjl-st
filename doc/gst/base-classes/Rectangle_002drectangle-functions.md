[]{#Rectangle_002drectangle-functions}

::: header
Next:
[Rectangle-testing](Rectangle_002dtesting.html#Rectangle_002dtesting){accesskey="n"
rel="next"}, Previous:
[Rectangle-printing](Rectangle_002dprinting.html#Rectangle_002dprinting){accesskey="p"
rel="prev"}, Up: [Rectangle](Rectangle.html#Rectangle){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Rectangle_003a-rectangle-functions}

#### 1.142.5 Rectangle: rectangle functions {#rectangle-rectangle-functions .subsection}

[]{#index-amountToTranslateWithin_003a}

**amountToTranslateWithin: aRectangle**

Answer a Point so that if aRectangle is translated by that point, its
origin lies within the receiver's.

[]{#index-area}

**area**

Answer the receiver's area. The area is the width times the height, so
it is possible for it to be negative if the rectangle is not normalized.

[]{#index-areasOutside_003a}

**areasOutside: aRectangle**

Answer a collection of rectangles containing the parts of the receiver
outside of aRectangle. For all points in the receiver, but outside
aRectangle, exactly one rectangle in the collection will contain that
point.

[]{#index-expandBy_003a}

**expandBy: delta**

Answer a new rectangle that is the receiver expanded by aValue: if
aValue is a rectangle, calculate origin=origin-aValue origin,
corner=corner+aValue corner; else calculate origin=origin-aValue,
corner=corner+aValue.

[]{#index-insetBy_003a}

**insetBy: delta**

Answer a new rectangle that is the receiver inset by aValue: if aValue
is a rectangle, calculate origin=origin+aValue origin,
corner=corner-aValue corner; else calculate origin=origin+aValue,
corner=corner-aValue.

[]{#index-insetOriginBy_003acorner_003a}

**insetOriginBy: originDelta corner: cornerDelta**

Answer a new rectangle that is the receiver inset so that
origin=origin+originDelta, corner=corner-cornerDelta. The deltas can be
points or numbers

[]{#index-intersect_003a}

**intersect: aRectangle**

Answers the rectangle (if any) created by the overlap of rectangles A
and B. Answers nil if the rectangles do not overlap

[]{#index-merge_003a}

**merge: aRectangle**

Answer a new rectangle which is the smallest rectangle containing both
the receiver and aRectangle.

[]{#index-translatedToBeWithin_003a}

**translatedToBeWithin: aRectangle**

Answer a copy of the receiver that does not extend beyond aRectangle.
