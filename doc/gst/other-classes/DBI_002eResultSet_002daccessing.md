[]{#DBI_002eResultSet_002daccessing}

::: header
Next: [DBI.ResultSet-cursor
access](DBI_002eResultSet_002dcursor-access.html#DBI_002eResultSet_002dcursor-access){accesskey="n"
rel="next"}, Up:
[DBI.ResultSet](DBI_002eResultSet.html#DBI_002eResultSet){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eResultSet_003a-accessing}

#### 3.5.1 DBI.ResultSet: accessing {#dbi.resultset-accessing .subsection}

[]{#index-columnAt_003a}

**columnAt: aIndex**

Answer the aIndex'th column name.

[]{#index-columnNames}

**columnNames**

Answer an array of column names in order (abstract).

[]{#index-columns}

**columns**

Answer a Dictionary of column name -\> ColumnInfo pairs (abstract).

[]{#index-isDML}

**isDML**

Returns true if the statement was not a SELECT or similar operation
(e.g. SHOW, DESCRIBE, EXPLAIN).

[]{#index-isSelect}

**isSelect**

Returns true if the statement was a SELECT or similar operation (e.g.
SHOW, DESCRIBE, EXPLAIN), false otherwise.

[]{#index-rowCount}

**rowCount**

Returns the number of rows in the result set; error for DML statements.

[]{#index-rows}

**rows**

Answer the contents of the execution result as array of Rows.

[]{#index-rowsAffected}

**rowsAffected**

For DML statments, returns the number of rows affected; error for SELECT
statements.

[]{#index-statement}

**statement**

Return the Statement, if any, that generated the result set.
