[]{#TextCollector_002daccessing}

::: header
Next:
[TextCollector-printing](TextCollector_002dprinting.html#TextCollector_002dprinting){accesskey="n"
rel="next"}, Previous: [TextCollector
class-accessing](TextCollector-class_002daccessing.html#TextCollector-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[TextCollector](TextCollector.html#TextCollector){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#TextCollector_003a-accessing}

#### 1.198.2 TextCollector: accessing {#textcollector-accessing .subsection}

[]{#index-cr-2}

**cr**

Emit a new-line (carriage return) to the Transcript

[]{#index-critical_003a-2}

**critical: aBlock**

Evaluate aBlock while holding the Transcript lock

[]{#index-endEntry}

**endEntry**

Emit two new-lines. This method is present for compatibility with
VisualWorks.

[]{#index-next_003aput_003a-1}

**next: anInteger put: anObject**

Write anInteger copies of anObject to the Transcript

[]{#index-next_003aputAll_003astartingAt_003a-3}

**next: n putAll: aString startingAt: pos**

Write aString to the Transcript

[]{#index-nextPut_003a-6}

**nextPut: aCharacter**

Emit aCharacter to the Transcript

[]{#index-show_003a}

**show: aString**

Write aString to the Transcript

[]{#index-showCr_003a}

**showCr: aString**

Write aString to the Transcript, followed by a new-line character

[]{#index-showOnNewLine_003a}

**showOnNewLine: aString**

Write aString to the Transcript, preceded by a new-line character
