[]{#RegexResults_002dtesting}

::: header
Previous:
[RegexResults-accessing](RegexResults_002daccessing.html#RegexResults_002daccessing){accesskey="p"
rel="prev"}, Up:
[RegexResults](RegexResults.html#RegexResults){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RegexResults_003a-testing}

#### 1.145.2 RegexResults: testing {#regexresults-testing .subsection}

[]{#index-ifMatched_003a}

**ifMatched: oneArgBlock**

If the regular expression was matched, pass the receiver to oneArgBlock
and return its result. Otherwise, return nil.

[]{#index-ifMatched_003aifNotMatched_003a}

**ifMatched: oneArgBlock ifNotMatched: zeroArgBlock**

If the regular expression was matched, evaluate oneArgBlock with the
receiver as the argument. If it was not, evaluate zeroArgBlock. Answer
the result of the block's evaluation.

[]{#index-ifNotMatched_003a}

**ifNotMatched: zeroArgBlock**

If the regular expression was matched, return the receiver. If it was
not, evaluate zeroArgBlock and return its result.

[]{#index-ifNotMatched_003aifMatched_003a}

**ifNotMatched: zeroArgBlock ifMatched: oneArgBlock**

If the regular expression was matched, evaluate oneArgBlock with the
receiver as the argument. If it was not, evaluate zeroArgBlock. Answer
the result of the block's evaluation.

[]{#index-matched}

**matched**

Answer whether the regular expression was matched
