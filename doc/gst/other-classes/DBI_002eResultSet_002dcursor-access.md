[]{#DBI_002eResultSet_002dcursor-access}

::: header
Next:
[DBI.ResultSet-printing](DBI_002eResultSet_002dprinting.html#DBI_002eResultSet_002dprinting){accesskey="n"
rel="next"}, Previous:
[DBI.ResultSet-accessing](DBI_002eResultSet_002daccessing.html#DBI_002eResultSet_002daccessing){accesskey="p"
rel="prev"}, Up:
[DBI.ResultSet](DBI_002eResultSet.html#DBI_002eResultSet){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eResultSet_003a-cursor-access}

#### 3.5.2 DBI.ResultSet: cursor access {#dbi.resultset-cursor-access .subsection}

[]{#index-atEnd}

**atEnd**

Return whether all the rows in the result set have been consumed.
(abstract).

[]{#index-fetch}

**fetch**

Return the next row, or nil if at the end of the result set.

[]{#index-next}

**next**

Return the next row, or raise an error if at the end of the stream
(abstract).
