[]{#DBI_002eConnectionInfo-class_002dinstance-creation}

::: header
Next:
[DBI.ConnectionInfo-accessing](DBI_002eConnectionInfo_002daccessing.html#DBI_002eConnectionInfo_002daccessing){accesskey="n"
rel="next"}, Up:
[DBI.ConnectionInfo](DBI_002eConnectionInfo.html#DBI_002eConnectionInfo){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DBI_002eConnectionInfo-class_003a-instance-creation}

#### 3.3.1 DBI.ConnectionInfo class: instance creation {#dbi.connectioninfo-class-instance-creation .subsection}

[]{#index-fromDSN_003a}

**fromDSN: aDSN**

Parse a DSN in the format
dbi:DriverName:dbname=database_name;host=hostname;port=port where dbi is
constant, DriverName is the name of the driver, and everything else is
parameters in the form name1=value1;name2=value2;\...
