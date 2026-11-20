[]{#I18N_002eLcTime_002dprinting}

::: header
Next:
[I18N.LcTime-tests](I18N_002eLcTime_002dtests.html#I18N_002eLcTime_002dtests){accesskey="n"
rel="next"}, Previous: [I18N.LcTime
class-accessing](I18N_002eLcTime-class_002daccessing.html#I18N_002eLcTime-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[I18N.LcTime](I18N_002eLcTime.html#I18N_002eLcTime){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcTime_003a-printing}

#### 5.20.2 I18N.LcTime: printing {#i18n.lctime-printing .subsection}

[]{#index-print_003aon_003a-5}

**print: aDateOrTimeOrArray on: aStream**

Print aDateOrTimeOrArray on aStream according to the receiver's
formatting conventions. It can be a Date, Time, DateTime, or an array
made of a Date and a Time

[]{#index-print_003aon_003aifFull_003aifDate_003aifTime_003a}
[]{#index-print_003atime_003aformat_003aon_003a-1}

**print: aDateOrTimeOrArray on: aStream ifFull: fullFmt ifDate: dateFmt
ifTime: timeFmt**

Print aDateOrTimeOrArray on aStream according to the receiver's
formatting conventions. It can be a Date, Time, DateTime, or an array
made of a Date and a Time: Date is printed with dateFmt and Time with
timeFmt, while in the other cases fullFmt is used. For information on
the formatting codes, see #print:time:format:on:.

[]{#index-print_003atime_003aformat_003aon_003a}

**print: aDate time: aTime format: aString on: aStream**

Print the specified date and time on aStream according to the receiver's
formatting conventions, using the given format. The valid abbreviations
are the same used by the C function strftime: abbreviated weekday (%a)
weekday (%A) abbreviated month (%b) month (%B) date & time (%c) century
(%C) day of the month (%d) date (US) (%D) day of the month (%e) year for
the ISO week (%g) year for the ISO week (%G) abbreviated month (%h)
hours (%H) hours (AM/PM) (%I) day of the year (%j) hours (%k) hours
(AM/PM) (%l) month (%m) minutes (%M) AM/PM (%p) lowercase AM/PM (%P)
AM/PM time (%r) time (US) (%R) time_t (%s) seconds (%S) time (US) (%T)
day of the week (%u) week number starting at Sun (%U) week number
starting at Thu (%V) day of the week, Sunday=0 (%w) week number starting
at Mon (%W) date (%x) time (%X) year (2-digit) (%y) year (4-digit) (%Y).

------------------------------------------------------------------------

::: header
Next:
[I18N.LcTime-tests](I18N_002eLcTime_002dtests.html#I18N_002eLcTime_002dtests){accesskey="n"
rel="next"}, Previous: [I18N.LcTime
class-accessing](I18N_002eLcTime-class_002daccessing.html#I18N_002eLcTime-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[I18N.LcTime](I18N_002eLcTime.html#I18N_002eLcTime){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
