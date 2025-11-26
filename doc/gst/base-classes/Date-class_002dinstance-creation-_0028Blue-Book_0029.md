[]{#Date-class_002dinstance-creation-_0028Blue-Book_0029}

::: header
Next: [Date-basic](Date_002dbasic.html#Date_002dbasic){accesskey="n"
rel="next"}, Previous: [Date class-instance creation
(ANSI)](Date-class_002dinstance-creation-_0028ANSI_0029.html#Date-class_002dinstance-creation-_0028ANSI_0029){accesskey="p"
rel="prev"}, Up: [Date](Date.html#Date){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Date-class_003a-instance-creation-_0028Blue-Book_0029}

#### 1.59.3 Date class: instance creation (Blue Book) {#date-class-instance-creation-blue-book .subsection}

[]{#index-dateAndTimeNow}

**dateAndTimeNow**

Answer an array containing the current date and time

[]{#index-fromDays_003a}

**fromDays: dayCount**

Answer a Date denoting dayCount days past 1/1/1901

[]{#index-fromJulian_003a}

**fromJulian: jd**

Answer a Date denoting the jd-th day in the astronomical Julian
calendar.

[]{#index-fromSeconds_003a}

**fromSeconds: time**

Answer a Date denoting the date time seconds past Jan 1st, 1901

[]{#index-newDay_003amonth_003ayear_003a}

**newDay: day month: monthName year: yearInteger**

Answer a Date denoting the dayCount day of the given (named) month and
year

[]{#index-newDay_003amonthIndex_003ayear_003a}

**newDay: day monthIndex: monthIndex year: yearInteger**

Answer a Date denoting the dayCount day of the given (as a number) month
and year

[]{#index-newDay_003ayear_003a}

**newDay: dayCount year: yearInteger**

Answer a Date denoting the dayCount day of the yearInteger year

[]{#index-readFrom_003a}

**readFrom: aStream**

Parse an instance of the receiver from aStream

[]{#index-today}

**today**

Answer a Date denoting the current date in local time

[]{#index-utcDateAndTimeNow}

**utcDateAndTimeNow**

Answer an array containing the current date and time in Coordinated
Universal Time (UTC)

[]{#index-utcToday}

**utcToday**

Answer a Date denoting the current date in Coordinated Universal Time
(UTC)
