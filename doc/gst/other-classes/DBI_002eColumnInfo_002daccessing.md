[]{#DBI_002eColumnInfo_002daccessing}

::: header
Next:
[DBI.ColumnInfo-printing](DBI_002eColumnInfo_002dprinting.html#DBI_002eColumnInfo_002dprinting){accesskey="n"
rel="next"}, Up:
[DBI.ColumnInfo](DBI_002eColumnInfo.html#DBI_002eColumnInfo){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eColumnInfo_003a-accessing}

#### 3.1.1 DBI.ColumnInfo: accessing {#dbi.columninfo-accessing .subsection}

[]{#index-index-2}

**index**

Return the 1-based index of the column in the result set (abstract).

[]{#index-isNullable}

**isNullable**

Return whether the column can be NULL (always returns true in
ColumnInfo).

[]{#index-name}

**name**

Return the name of the column (abstract).

[]{#index-size-2}

**size**

Return the size of the column (abstract).

[]{#index-type}

**type**

Return a string containing the type of the column (abstract).
