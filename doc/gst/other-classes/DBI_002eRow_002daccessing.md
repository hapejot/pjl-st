[]{#DBI_002eRow_002daccessing}

::: header
Next:
[DBI.Row-printing](DBI_002eRow_002dprinting.html#DBI_002eRow_002dprinting){accesskey="n"
rel="next"}, Up: [DBI.Row](DBI_002eRow.html#DBI_002eRow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eRow_003a-accessing}

#### 3.6.1 DBI.Row: accessing {#dbi.row-accessing .subsection}

[]{#index-asArray}

**asArray**

Return the values of the columns.

[]{#index-asDictionary}

**asDictionary**

Return the names and values of the columns as a dictionary.

[]{#index-at_003a-4}

**at: aColumnName**

Return the value of the named column (abstract).

[]{#index-atIndex_003a}

**atIndex: aColumnIndex**

Return the value of the column at the given 1-based index (abstract).

[]{#index-columnAt_003a-1}

**columnAt: aIndex**

Return a ColumnInfo object for the aIndex-th column in the row.

[]{#index-columnCount}

**columnCount**

Return the number of columns in the row.

[]{#index-columnNames-1}

**columnNames**

Return an array of column names for the columns in the row.

[]{#index-columns-1}

**columns**

Return a Dictionary of ColumnInfo objects for the columns in the row,
where the keys are the column names.

[]{#index-keysAndValuesDo_003a}

**keysAndValuesDo: aBlock**

Pass to aBlock each column name and the corresponding value.

[]{#index-resultSet}

**resultSet**

Return the result set that includes the receiver.
