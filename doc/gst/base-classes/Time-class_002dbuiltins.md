[]{#Time-class_002dbuiltins}

::: header
Next: [Time
class-clocks](Time-class_002dclocks.html#Time-class_002dclocks){accesskey="n"
rel="next"}, Previous: [Time class-basic
(UTC)](Time-class_002dbasic-_0028UTC_0029.html#Time-class_002dbasic-_0028UTC_0029){accesskey="p"
rel="prev"}, Up: [Time](Time.html#Time){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Time-class_003a-builtins}

#### 1.199.2 Time class: builtins {#time-class-builtins .subsection}

[]{#index-primNanosecondClock}

**primNanosecondClock**

Returns the number of milliseconds since midnight.

[]{#index-primSecondClock}

**primSecondClock**

Returns the number of seconds to/from 1/1/2000.

[]{#index-timezone}

**timezone**

Answer a String associated with the current timezone (either standard or
daylight-saving) on this operating system. For example, the answer could
be 'EST' to indicate Eastern Standard Time; the answer can be empty and
can't be assumed to be a three-character code such as 'EST'.

[]{#index-timezoneBias}

**timezoneBias**

Specifies the current bias, in seconds, for local time translation for
the current time. The bias is the difference, in seconds, between
Coordinated Universal Time (UTC) and local time; a positive bias
indicates that the local timezone is to the east of Greenwich (e.g.
Europe, Asia), while a negative bias indicates that it is to the west
(e.g. America)

[]{#index-timezoneBias_003a}

**timezoneBias: seconds**

Specifies the bias, in seconds, for local time translation for the given
second clock value (0 being midnight of 1/1/1901). The bias is the
difference, in seconds, between Coordinated Universal Time (UTC) and
local time; a positive bias indicates that the local timezone is to the
east of Greenwich (e.g. Europe, Asia), while a negative bias indicates
that it is to the west (e.g. America)
