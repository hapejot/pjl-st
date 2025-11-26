[]{#DBI_002eConnection_002dquerying}

::: header
Previous:
[DBI.Connection-connecting](DBI_002eConnection_002dconnecting.html#DBI_002eConnection_002dconnecting){accesskey="p"
rel="prev"}, Up:
[DBI.Connection](DBI_002eConnection.html#DBI_002eConnection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eConnection_003a-querying}

#### 3.2.5 DBI.Connection: querying {#dbi.connection-querying .subsection}

[]{#index-do_003a-3} []{#index-rowsAffected-1}

**do: aSQLQuery**

Executes a SQL statement (usually one that doesn't return a result set).
Return value is a ResultSet, to which you can send #rowsAffected
(abstract).

[]{#index-prepare_003a}

**prepare: aSQLQuery**

Creates a statement object, that can be executed (with parameters, if
applicable) repeatedly (abstract).

[]{#index-primTableAt_003aifAbsent_003a}

**primTableAt: aString ifAbsent: aBlock**

Returns a Table object corresponding to the given table. Should be
overridden by subclasses.

[]{#index-select_003a-1}

**select: aSQLQuery**

Prepares and executes a SQL statement. Returns the result set or throws
an exception on failure (abstract).
