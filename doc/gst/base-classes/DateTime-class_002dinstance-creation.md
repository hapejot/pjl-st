[]{#DateTime-class_002dinstance-creation}

::: header
Next: [DateTime class-instance creation
(non-ANSI)](DateTime-class_002dinstance-creation-_0028non_002dANSI_0029.html#DateTime-class_002dinstance-creation-_0028non_002dANSI_0029){accesskey="n"
rel="next"}, Previous: [DateTime
class-information](DateTime-class_002dinformation.html#DateTime-class_002dinformation){accesskey="p"
rel="prev"}, Up: [DateTime](DateTime.html#DateTime){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DateTime-class_003a-instance-creation}

#### 1.60.2 DateTime class: instance creation {#datetime-class-instance-creation .subsection}

[]{#index-now}

**now**

Answer an instance of the receiver referring to the current date and
time.

[]{#index-readFrom_003a-1}

**readFrom: aStream**

Parse an instance of the receiver from aStream

[]{#index-today-1}

**today**

Answer an instance of the receiver referring to midnight of today in
local time.

[]{#index-year_003aday_003ahour_003aminute_003asecond_003a-1}

**year: y day: d hour: h minute: min second: s**

Answer a DateTime denoting the d-th day of the given year, and setting
the time part to the given hour, minute, and second

[]{#index-year_003aday_003ahour_003aminute_003asecond_003aoffset_003a}

**year: y day: d hour: h minute: min second: s offset: ofs**

Answer a DateTime denoting the d-th day of the given year. Set the
offset field to ofs (a Duration), and the time part to the given hour,
minute, and second

[]{#index-year_003amonth_003aday_003ahour_003aminute_003asecond_003a-1}

**year: y month: m day: d hour: h minute: min second: s**

Answer a DateTime denoting the d-th day of the given (as a number) month
and year, setting the time part to the given hour, minute, and second

[]{#index-year_003amonth_003aday_003ahour_003aminute_003asecond_003aoffset_003a}

**year: y month: m day: d hour: h minute: min second: s offset: ofs**

Answer a DateTime denoting the d-th day of the given (as a number) month
and year. Set the offset field to ofs (a Duration), and the the time
part to the given hour, minute, and second
