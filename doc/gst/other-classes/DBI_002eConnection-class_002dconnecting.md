[]{#DBI_002eConnection-class_002dconnecting}

::: header
Next: [DBI.Connection
class-initialization](DBI_002eConnection-class_002dinitialization.html#DBI_002eConnection-class_002dinitialization){accesskey="n"
rel="next"}, Up:
[DBI.Connection](DBI_002eConnection.html#DBI_002eConnection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eConnection-class_003a-connecting}

#### 3.2.1 DBI.Connection class: connecting {#dbi.connection-class-connecting .subsection}

[]{#index-connect_003auser_003apassword_003a}

**connect: aDSN user: aUserName password: aPassword**

Connect to the database server identified by aDSN using the given
username and password. The DSN is in the format
dbi:DriverName:dbname=database_name;host=hostname;port=port Where dbi is
constant, DriverName is the name of the driver, and everything else is
parameters in the form name1=value1;name2=value2;\...

Individual drivers may parse the parameters differently, though the
existing ones all support parameters dbname, host and port.

[]{#index-paramConnect_003auser_003apassword_003a}

**paramConnect: params user: aUserName password: aPassword**

Connect to the database server using the parameters in params (a
Dictionary) and the given username and password (abstract).
