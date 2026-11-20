[]{#Rectangle_002dtesting}

::: header
Next:
[Rectangle-transforming](Rectangle_002dtransforming.html#Rectangle_002dtransforming){accesskey="n"
rel="next"}, Previous: [Rectangle-rectangle
functions](Rectangle_002drectangle-functions.html#Rectangle_002drectangle-functions){accesskey="p"
rel="prev"}, Up: [Rectangle](Rectangle.html#Rectangle){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Rectangle_003a-testing}

#### 1.142.6 Rectangle: testing {#rectangle-testing .subsection}

[]{#index-_003d-35}

**= aRectangle**

Answer whether the receiver is equal to aRectangle

[]{#index-contains_003a-1}

**contains: aRectangle**

Answer true if the receiver contains (see containsPoint:) both
aRectangle's origin and aRectangle's corner

[]{#index-containsPoint_003a}

**containsPoint: aPoint**

Answer true if aPoint is equal to, or below and to the right of, the
receiver's origin; and aPoint is above and to the left of the receiver's
corner

[]{#index-hash-30}

**hash**

Answer an hash value for the receiver

[]{#index-intersects_003a}

**intersects: aRectangle**

Answer true if the receiver intersect aRectangle, i.e. if it contains
(see containsPoint:) any of aRectangle corners or if aRectangle contains
the receiver
