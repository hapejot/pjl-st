[]{#NetClients_002eURIResolver-class_002dapi}

::: header
Next: [NetClients.URIResolver class-instance
creation](NetClients_002eURIResolver-class_002dinstance-creation.html#NetClients_002eURIResolver-class_002dinstance-creation){accesskey="n"
rel="next"}, Up:
[NetClients.URIResolver](NetClients_002eURIResolver.html#NetClients_002eURIResolver){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#NetClients_002eURIResolver-class_003a-api}

#### 1.117.1 NetClients.URIResolver class: api {#netclients.uriresolver-class-api .subsection}

[]{#index-openOn_003a}

**openOn: aURI**

Always raise an error, as this method is not supported without loading
the additional NetClients package.

[]{#index-openOn_003aifFail_003a}

**openOn: aURI ifFail: aBlock**

Always evaluate aBlock and answer the result if the additional
NetClients package is not loaded. If it is, instead, return a WebEntity
with the contents of the resource specified by anURI, and only evaluate
the block if loading the resource fails.

[]{#index-openStreamOn_003a}

**openStreamOn: aURI**

Check if aURI can be fetched from the Internet or from the local system,
and if so return a Stream with its contents. If this is not possible,
raise an exception.

[]{#index-openStreamOn_003aifFail_003a}

**openStreamOn: aURI ifFail: aBlock**

Check if aURI can be fetched from the Internet or from the local system,
and if so return a Stream with its contents. If this is not possible,
instead, evaluate the zero-argument block aBlock and answer the result
of the evaluation.
