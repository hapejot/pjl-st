[]{#Collection_002dconcatenating}

::: header
Next:
[Collection-converting](Collection_002dconverting.html#Collection_002dconverting){accesskey="n"
rel="next"}, Previous:
[Collection-compiler](Collection_002dcompiler.html#Collection_002dcompiler){accesskey="p"
rel="prev"}, Up: [Collection](Collection.html#Collection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Collection_003a-concatenating}

#### 1.37.5 Collection: concatenating {#collection-concatenating .subsection}

[]{#index-join}

**join**

Answer a new collection like my first element, with all the elements (in
order) of all my elements, which should be collections.

I use my first element instead of myself as a prototype because my
elements are more likely to share the desired properties than I am, such
as in:

#('hello, ' 'world') join =\> 'hello, world'
