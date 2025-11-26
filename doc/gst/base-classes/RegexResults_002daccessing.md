[]{#RegexResults_002daccessing}

::: header
Next:
[RegexResults-testing](RegexResults_002dtesting.html#RegexResults_002dtesting){accesskey="n"
rel="next"}, Up:
[RegexResults](RegexResults.html#RegexResults){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RegexResults_003a-accessing}

#### 1.145.1 RegexResults: accessing {#regexresults-accessing .subsection}

[]{#index-asArray-1}

**asArray**

If the regular expression was matched, return an Array with the
subexpressions that were present in the regular expression.

[]{#index-at_003a-18}

**at: anIndex**

If the regular expression was matched, return the text of the anIndex-th
subexpression in the successful match.

[]{#index-from}

**from**

If the regular expression was matched, return the index of the first
character in the successful match.

[]{#index-fromAt_003a}

**fromAt: anIndex**

If the regular expression was matched, return the index of the first
character of the anIndex-th subexpression in the successful match.

[]{#index-intervalAt_003a}

**intervalAt: anIndex**

If the regular expression was matched, return an Interval for the range
of indices in the anIndex-th subexpression of the successful match.

[]{#index-match}

**match**

If the regular expression was matched, return the text of the successful
match.

[]{#index-matchInterval}

**matchInterval**

If the regular expression was matched, return an Interval for the range
of indices of the successful match.

[]{#index-size-21}

**size**

If the regular expression was matched, return the number of
subexpressions that were present in the regular expression.

[]{#index-subject}

**subject**

If the regular expression was matched, return the text that was matched
against it.

[]{#index-to}

**to**

If the regular expression was matched, return the index of the last
character in the successful match.

[]{#index-toAt_003a}

**toAt: anIndex**

If the regular expression was matched, return the index of the last
character of the anIndex-th subexpression in the successful match.
