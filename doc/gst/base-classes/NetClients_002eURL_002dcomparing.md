[]{#NetClients_002eURL_002dcomparing}

::: header
Next:
[NetClients.URL-copying](NetClients_002eURL_002dcopying.html#NetClients_002eURL_002dcopying){accesskey="n"
rel="next"}, Previous:
[NetClients.URL-accessing](NetClients_002eURL_002daccessing.html#NetClients_002eURL_002daccessing){accesskey="p"
rel="prev"}, Up:
[NetClients.URL](NetClients_002eURL.html#NetClients_002eURL){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#NetClients_002eURL_003a-comparing}

#### 1.118.4 NetClients.URL: comparing {#netclients.url-comparing .subsection}

[]{#index-_003d-32}

**= anURL**

Answer whether the two URLs are equal. The file and anchor are converted
to full 8-bit ASCII (contrast with urlencoded) and the comparison is
case-sensitive; on the other hand, the protocol and host are compared
without regard to case.

[]{#index-hash-27}

**hash**

Answer an hash value for the receiver
