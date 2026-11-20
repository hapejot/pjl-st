[]{#Date}

::: header
Next: [DateTime](DateTime.html#DateTime){accesskey="n" rel="next"},
Previous: [CUShort](CUShort.html#CUShort){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Date-1}

### 1.59 Date {#date .section}

[]{#index-Date}

**Defined in namespace Smalltalk**\
**Superclass: Magnitude**\
**Category: Language-Data types**

:   My instances represent dates. My base date is defined to be Jan
    1, 1901. I provide methods for instance creation (including via
    \"symbolic\" dates, such as \"Date newDay: 14 month: #Feb year:
    1990\".

    PLEASE BE WARNED -- use this class only for dates after 1582 AD;
    that's the beginning of the epoch. Dates before 1582 will not be
    correctly printed. In addition, since ten days were lost from
    October 5 through October 15, operations between a Gregorian date
    (after 15-Oct-1582) and a Julian date (before 5-Oct-1582) will give
    incorrect results; or, 4-Oct-1582 + 2 days will yield 6-Oct-1582 (a
    non-existent day!), not 16-Oct-1582.

    In fact, if you pass a year \< 1582 to a method like
    #newDay:month:year: it will assume that it is a two-digit year (e.g.
    90=1990, 1000=2900). The only way to create Julian calendar dates is
    with the #fromDays: instance creation method.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ ---- ------------
  • [Date class-basic](Date-class_002dbasic.html#Date-class_002dbasic){accesskey="1"}:                                                                                                (class)
  • [Date class-instance creation (ANSI)](Date-class_002dinstance-creation-_0028ANSI_0029.html#Date-class_002dinstance-creation-_0028ANSI_0029){accesskey="2"}:                       (class)
  • [Date class-instance creation (Blue Book)](Date-class_002dinstance-creation-_0028Blue-Book_0029.html#Date-class_002dinstance-creation-_0028Blue-Book_0029){accesskey="3"}:        (class)
  • [Date-basic](Date_002dbasic.html#Date_002dbasic){accesskey="4"}:                                                                                                                  (instance)
  • [Date-compatibility (non-ANSI)](Date_002dcompatibility-_0028non_002dANSI_0029.html#Date_002dcompatibility-_0028non_002dANSI_0029){accesskey="5"}:                                 (instance)
  • [Date-date computations](Date_002ddate-computations.html#Date_002ddate-computations){accesskey="6"}:                                                                              (instance)
  • [Date-printing](Date_002dprinting.html#Date_002dprinting){accesskey="7"}:                                                                                                         (instance)
  • [Date-still unclassified](Date_002dstill-unclassified.html#Date_002dstill-unclassified){accesskey="8"}:                                                                           (instance)
  • [Date-storing](Date_002dstoring.html#Date_002dstoring){accesskey="9"}:                                                                                                            (instance)
  • [Date-testing](Date_002dtesting.html#Date_002dtesting):                                                                                                                           (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ ---- ------------
