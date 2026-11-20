[]{#DateTime_002dtime-zones}

::: header
Previous:
[DateTime-testing](DateTime_002dtesting.html#DateTime_002dtesting){accesskey="p"
rel="prev"}, Up: [DateTime](DateTime.html#DateTime){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DateTime_003a-time-zones}

#### 1.60.10 DateTime: time zones {#datetime-time-zones .subsection}

[]{#index-asLocal}

**asLocal**

Answer the receiver, since DateTime objects store themselves in Local
time

[]{#index-asUTC}

**asUTC**

Convert the receiver to UTC time, and answer a new DateTime object.

[]{#index-offset}

**offset**

Answer the receiver's offset from UTC to local time (e.g. +3600 seconds
for Central Europe Time, -3600\*6 seconds for Eastern Standard Time).
The offset is expressed as a Duration

[]{#index-offset_003a}

**offset: anOffset**

Answer a copy of the receiver with the offset from UTC to local time
changed to anOffset (a Duration).

[]{#index-timeZoneAbbreviation}

**timeZoneAbbreviation**

Answer an abbreviated indication of the receiver's offset, expressed as
'shhmm', where 'hh' is the number of hours and 'mm' is the number of
minutes between UTC and local time, and 's' can be '+' for the Eastern
hemisphere and '-' for the Western hemisphere.

[]{#index-timeZoneName} []{#index-timeZoneAbbreviation-1}

**timeZoneName**

Answer the time zone name for the receiver (currently, it is simply 'GMT
+xxxx', where 'xxxx' is the receiver's #timeZoneAbbreviation).
