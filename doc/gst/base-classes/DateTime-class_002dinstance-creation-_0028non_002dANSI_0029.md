[]{#DateTime-class_002dinstance-creation-_0028non_002dANSI_0029}

::: header
Next:
[DateTime-basic](DateTime_002dbasic.html#DateTime_002dbasic){accesskey="n"
rel="next"}, Previous: [DateTime class-instance
creation](DateTime-class_002dinstance-creation.html#DateTime-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [DateTime](DateTime.html#DateTime){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DateTime-class_003a-instance-creation-_0028non_002dANSI_0029}

#### 1.60.3 DateTime class: instance creation (non-ANSI) {#datetime-class-instance-creation-non-ansi .subsection}

[]{#index-date_003atime_003a}

**date: aDate time: aTime**

Answer a DateTime denoting the given date and time. Set the offset field
to ofs (a Duration).

[]{#index-date_003atime_003aoffset_003a}

**date: aDate time: aTime offset: ofs**

Answer a DateTime denoting the given date and time. Set the offset field
to ofs (a Duration).

[]{#index-fromDays_003aseconds_003a}

**fromDays: days seconds: secs**

Answer a DateTime denoting the given date (as days since January 1,
1901) and time (as seconds since UTC midnight).

[]{#index-fromDays_003aseconds_003aoffset_003a}

**fromDays: days seconds: secs offset: ofs**

Answer a DateTime denoting the given date (as days since January 1,
1901) and time (as seconds since midnight). Set the offset field to ofs
(a Duration).

[]{#index-fromSeconds_003a-1}

**fromSeconds: secs**

Answer a DateTime denoting the given date and time (as seconds since
January 1, 1901 midnight UTC).

[]{#index-fromSeconds_003aoffset_003a}

**fromSeconds: secs offset: ofs**

Answer a DateTime denoting the given date and time (as seconds since
January 1, 1901 midnight). Set the offset field to ofs (a Duration).
