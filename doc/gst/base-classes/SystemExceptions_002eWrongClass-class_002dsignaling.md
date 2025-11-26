[]{#SystemExceptions_002eWrongClass-class_002dsignaling}

::: header
Next:
[SystemExceptions.WrongClass-accessing](SystemExceptions_002eWrongClass_002daccessing.html#SystemExceptions_002eWrongClass_002daccessing){accesskey="n"
rel="next"}, Up:
[SystemExceptions.WrongClass](SystemExceptions_002eWrongClass.html#SystemExceptions_002eWrongClass){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SystemExceptions_002eWrongClass-class_003a-signaling}

#### 1.196.1 SystemExceptions.WrongClass class: signaling {#systemexceptions.wrongclass-class-signaling .subsection}

[]{#index-signalOn_003amustBe_003a}

**signalOn: anObject mustBe: aClassOrArray**

Raise an exception. The given object should have been an instance of one
of the classes indicated by aClassOrArray (which should be a single
class or an array of classes). Whether instances of subclasses are
allowed should be clear from the context, though in general (i.e. with
the exception of a few system messages) they should be.
