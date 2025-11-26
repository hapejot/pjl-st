[]{#DBI_002eConnection_002daccessing}

::: header
Next:
[DBI.Connection-connecting](DBI_002eConnection_002dconnecting.html#DBI_002eConnection_002dconnecting){accesskey="n"
rel="next"}, Previous: [DBI.Connection
class-initialization](DBI_002eConnection-class_002dinitialization.html#DBI_002eConnection-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[DBI.Connection](DBI_002eConnection.html#DBI_002eConnection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eConnection_003a-accessing}

#### 3.2.3 DBI.Connection: accessing {#dbi.connection-accessing .subsection}

[]{#index-_003e_003e}

**\>\> aString**

Returns a Table object corresponding to the given table.

[]{#index-database}

**database**

Returns the database name for this connection. This corresponds to the
catalog in SQL standard parlance (abstract).

[]{#index-fieldConverter}

**fieldConverter**

Returns a FieldConverter that can be used to insert Smalltalk objects
into queries.

[]{#index-tableAt_003a}

**tableAt: aString**

Returns a Table object corresponding to the given table.

[]{#index-tableAt_003aifAbsent_003a}

**tableAt: aString ifAbsent: aBlock**

Returns a Table object corresponding to the given table.
