[]{#DBI_002eResultSet}

::: header
Next: [DBI.Row](DBI_002eRow.html#DBI_002eRow){accesskey="n" rel="next"},
Previous:
[DBI.FieldConverter](DBI_002eFieldConverter.html#DBI_002eFieldConverter){accesskey="p"
rel="prev"}, Up: [DBI
package](DBI-package.html#DBI-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eResultSet-1}

### 3.5 DBI.ResultSet {#dbi.resultset .section}

[]{#index-DBI_002eResultSet}

**Defined in namespace DBI**\
**Superclass: Stream**\
**Category: DBI-Framework**

:   I represent a result set, ie. the set of rows returned from a SELECT
    statement. I may also be returned for DML statements (INSERT,
    UPDATE, DELETE), in which case I only hold the number of rows
    affected.

  ------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [DBI.ResultSet-accessing](DBI_002eResultSet_002daccessing.html#DBI_002eResultSet_002daccessing){accesskey="1"}:                          (instance)
  • [DBI.ResultSet-cursor access](DBI_002eResultSet_002dcursor-access.html#DBI_002eResultSet_002dcursor-access){accesskey="2"}:              (instance)
  • [DBI.ResultSet-printing](DBI_002eResultSet_002dprinting.html#DBI_002eResultSet_002dprinting){accesskey="3"}:                             (instance)
  • [DBI.ResultSet-stream protocol](DBI_002eResultSet_002dstream-protocol.html#DBI_002eResultSet_002dstream-protocol){accesskey="4"}:        (instance)
  ------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
