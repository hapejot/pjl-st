[]{#Getopt-class_002dinstance-creation}

::: header
Up: [Getopt](Getopt.html#Getopt){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Getopt-class_003a-instance-creation}

#### 1.86.1 Getopt class: instance creation {#getopt-class-instance-creation .subsection}

[]{#index-parse_003awith_003ado_003a}
[]{#index-parse_003awith_003ado_003aifError_003a-1}

**parse: args with: pattern do: actionBlock**

Parse the command-line arguments in args according to the syntax
specified in pattern. For every command-line option found, the
two-argument block actionBlock is evaluated passing the option name and
the argument. For file names (or in general, other command-line
arguments than options) the block's first argument will be nil. For
options without arguments, or with unspecified optional arguments, the
block's second argument will be nil. The option name will be passed as a
character object for short options, and as a string for long options.

If an error is found, nil is returned. For more information on the
syntax of pattern, see #parse:with:do:ifError:.

[]{#index-parse_003awith_003ado_003aifError_003a}

**parse: args with: pattern do: actionBlock ifError: errorBlock**

Parse the command-line arguments in args according to the syntax
specified in pattern. For every command-line option found, the
two-argument block actionBlock is evaluated passing the option name and
the argument. For file names (or in general, other command-line
arguments than options) the block's first argument will be nil. For
options without arguments, or with unspecified optional arguments, the
block's second argument will be nil. The option name will be passed as a
character object for short options, and as a string for long options.

If an error is found, the parsing is interrupted, errorBlock is
evaluated, and the returned value is answered.

Every whitespace-separated part ('word') of pattern specifies a
command-line option. If a word ends with a colon, the option will have a
mandatory argument. If a word ends with two colons, the option will have
an optional argument. Before the colons, multiple option names (either
short names like '-l' or long names like '--long') can be specified.
Before passing the option to actionBlock, the name will be canonicalized
to the last one.

Prefixes of long options are accepted as long as they're unique, and
they are canonicalized to the full name before passing it to
actionBlock. Additionally, the full name of an option is accepted even
if it is the prefix of a longer option.

Mandatory arguments can appear in the next argument, or in the same
argument (separated by an = for arguments to long options). Optional
arguments must appear in the same argument.

------------------------------------------------------------------------

::: header
Up: [Getopt](Getopt.html#Getopt){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
