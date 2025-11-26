[]{#String_002dregex}

::: header
Next: [String-still
unclassified](String_002dstill-unclassified.html#String_002dstill-unclassified){accesskey="n"
rel="next"}, Previous:
[String-printing](String_002dprinting.html#String_002dprinting){accesskey="p"
rel="prev"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String_003a-regex}

#### 1.158.10 String: regex {#string-regex .subsection}

[]{#index-_003d_007e}

**=\~ pattern**

Answer a RegexResults object for matching the receiver against the Regex
or String object pattern.

[]{#index-allOccurrencesOfRegex_003a}

**allOccurrencesOfRegex: pattern**

Find all the matches of pattern within the receiver and collect them
into an OrderedCollection.

[]{#index-allOccurrencesOfRegex_003ado_003a}

**allOccurrencesOfRegex: pattern do: aBlock**

Find all the matches of pattern within the receiver and pass the
RegexResults objects to aBlock.

[]{#index-allOccurrencesOfRegex_003afrom_003ato_003a}

**allOccurrencesOfRegex: pattern from: from to: to**

Find all the matches of pattern within the receiver and within the given
range of indices. Collect them into an OrderedCollection, which is then
returned.

[]{#index-allOccurrencesOfRegex_003afrom_003ato_003ado_003a}

**allOccurrencesOfRegex: pattern from: from to: to do: aBlock**

Find all the matches of pattern within the receiver and within the given
range of indices. For each match, pass the RegexResults object to
aBlock.

[]{#index-asRegex-1}

**asRegex**

Answer the receiver, converted to a Regex object.

[]{#index-copyFrom_003ato_003areplacingAllRegex_003awith_003a}
[]{#index-_0025-1}

**copyFrom: from to: to replacingAllRegex: pattern with:
aStringOrBlock**

Returns the substring of the receiver between from and to. Any match of
pattern in that part of the string is replaced using aStringOrBlock as
follows: if it is a block, a RegexResults object is passed, while if it
is a string, %n sequences are replaced with the captured subexpressions
of the match (as in #%).

[]{#index-copyFrom_003ato_003areplacingRegex_003awith_003a}
[]{#index-_0025-2}

**copyFrom: from to: to replacingRegex: pattern with: aStringOrBlock**

Returns the substring of the receiver between from and to. If pattern
has a match in that part of the string, the match is replaced using
aStringOrBlock as follows: if it is a block, a RegexResults object is
passed, while if it is a string, %n sequences are replaced with the
captured subexpressions of the match (as in #%).

[]{#index-copyReplacingAllRegex_003awith_003a} []{#index-_0025-3}

**copyReplacingAllRegex: pattern with: aStringOrBlock**

Returns the receiver after replacing all the matches of pattern (if any)
using aStringOrBlock as follows: if it is a block, a RegexResults object
is passed, while if it is a string, %n sequences are replaced with the
captured subexpressions of the match (as in #%).

[]{#index-copyReplacingRegex_003awith_003a} []{#index-_0025-4}

**copyReplacingRegex: pattern with: aStringOrBlock**

Returns the receiver after replacing the first match of pattern (if any)
using aStringOrBlock as follows: if it is a block, a RegexResults object
is passed, while if it is a string, %n sequences are replaced with the
captured subexpressions of the match (as in #%).

[]{#index-indexOfRegex_003a}

**indexOfRegex: regexString**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match. Otherwise return
nil.

[]{#index-indexOfRegex_003afrom_003ato_003a}

**indexOfRegex: regexString from: from to: to**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match occurring within
the given range of indices. Otherwise return nil.

[]{#index-indexOfRegex_003afrom_003ato_003aifAbsent_003a}

**indexOfRegex: regexString from: from to: to ifAbsent: excBlock**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match occurring within
the given indices. Otherwise, evaluate excBlock and return the result.

[]{#index-indexOfRegex_003aifAbsent_003a}

**indexOfRegex: regexString ifAbsent: excBlock**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match. Otherwise,
evaluate excBlock and return the result.

[]{#index-indexOfRegex_003astartingAt_003a}

**indexOfRegex: regexString startingAt: index**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match starting after the
given index. Otherwise return nil.

[]{#index-indexOfRegex_003astartingAt_003aifAbsent_003a}

**indexOfRegex: regexString startingAt: index ifAbsent: excBlock**

If an occurrence of the regex is present in the receiver, return the
Interval corresponding to the leftmost-longest match starting after the
given index. Otherwise, evaluate excBlock and return the result.

[]{#index-matchRegex_003a}

**matchRegex: pattern**

Answer whether the receiver is an exact match for the pattern. This
means that the pattern is implicitly anchored at the beginning and the
end.

[]{#index-matchRegex_003afrom_003ato_003a}

**matchRegex: pattern from: from to: to**

Answer whether the given range of indices is an exact match for the
pattern. This means that there is a match starting at from and ending at
to (which is not necessarily the longest match starting at from).

[]{#index-occurrencesOfRegex_003a}

**occurrencesOfRegex: pattern**

Returns count of how many times pattern repeats in the receiver.

[]{#index-occurrencesOfRegex_003afrom_003ato_003a}

**occurrencesOfRegex: pattern from: from to: to**

Return a count of how many times pattern repeats in the receiver within
the given range of index.

[]{#index-occurrencesOfRegex_003astartingAt_003a}

**occurrencesOfRegex: pattern startingAt: index**

Returns count of how many times pattern repeats in the receiver,
starting the search at the given index.

[]{#index-onOccurrencesOfRegex_003ado_003a}

**onOccurrencesOfRegex: pattern do: body**

Find all the matches of pattern within the receiver and, for each match,
pass the RegexResults object to aBlock.

[]{#index-onOccurrencesOfRegex_003afrom_003ato_003ado_003a}

**onOccurrencesOfRegex: pattern from: from to: to do: aBlock**

Find all the matches of pattern within the receiver and within the given
range of indices. For each match, pass the RegexResults object to
aBlock.

[]{#index-replacingAllRegex_003awith_003a} []{#index-_0025-5}

**replacingAllRegex: pattern with: aStringOrBlock**

Returns the receiver if the pattern has no match in it. Otherwise, any
match of pattern in that part of the string is replaced using
aStringOrBlock as follows: if it is a block, a RegexResults object is
passed, while if it is a string, %n sequences are replaced with the
captured subexpressions of the match (as in #%).

[]{#index-replacingRegex_003awith_003a} []{#index-_0025-6}

**replacingRegex: pattern with: aStringOrBlock**

Returns the receiver if the pattern has no match in it. If it has a
match, it is replaced using aStringOrBlock as follows: if it is a block,
a RegexResults object is passed, while if it is a string, %n sequences
are replaced with the captured subexpressions of the match (as in #%).

[]{#index-searchRegex_003a} []{#index-_003d_007e-1}

**searchRegex: pattern**

A synonym for #=\~. Answer a RegexResults object for matching the
receiver against the Regex or String object pattern.

[]{#index-searchRegex_003afrom_003ato_003a}

**searchRegex: pattern from: from to: to**

Answer a RegexResults object for matching the receiver against the Regex
or String object pattern, restricting the match to the specified range
of indices.

[]{#index-searchRegex_003astartingAt_003a}

**searchRegex: pattern startingAt: anIndex**

Answer a RegexResults object for matching the receiver against the Regex
or String object pattern, starting the match at index anIndex.

[]{#index-tokenize_003a}

**tokenize: pattern**

Split the receiver at every occurrence of pattern. All parts that do not
match pattern are separated and stored into an Array of Strings that is
returned.

[]{#index-tokenize_003afrom_003ato_003a}

**tokenize: pattern from: from to: to**

Split the receiver at every occurrence of pattern (considering only the
indices between from and to). All parts that do not match pattern are
separated and stored into an Array of Strings that is returned.

[]{#index-_007e}

**\~ pattern**

Answer whether the receiver matched against the Regex or String object
pattern.

------------------------------------------------------------------------

::: header
Next: [String-still
unclassified](String_002dstill-unclassified.html#String_002dstill-unclassified){accesskey="n"
rel="next"}, Previous:
[String-printing](String_002dprinting.html#String_002dprinting){accesskey="p"
rel="prev"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
