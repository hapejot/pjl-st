[]{#BindingDictionary_002daccessing}

::: header
Next: [BindingDictionary-basic &
copying](BindingDictionary_002dbasic-_0026-copying.html#BindingDictionary_002dbasic-_0026-copying){accesskey="n"
rel="next"}, Up:
[BindingDictionary](BindingDictionary.html#BindingDictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BindingDictionary_003a-accessing}

#### 1.10.1 BindingDictionary: accessing {#bindingdictionary-accessing .subsection}

[]{#index-define_003a} []{#index-at_003aput_003a-25}

**define: aSymbol**

Define aSymbol as equal to nil inside the receiver. Fail if such a
variable already exists (use #at:put: if you don't want to fail)

[]{#index-doesNotUnderstand_003a-1} []{#index-Variable}
[]{#index-Variable-1}

**doesNotUnderstand: aMessage**

Try to map unary selectors to read accesses to the Namespace, and
one-argument keyword selectors to write accesses. Note that: a) this
works only if the selector has an uppercase first letter; and b)
'aNamespace Variable: value' is the same as 'aNamespace set: #Variable
to: value', not the same as 'aNamespace at: #Variable put: value' ---
the latter always refers to the current namespace, while the former
won't define a new variable, instead searching in superspaces (and
raising an error if the variable cannot be found).

[]{#index-environment-2}

**environment**

Answer the environment to which the receiver is connected. This can be
the class for a dictionary that holds class variables, or the
super-namespace. In general it is used to compute the receiver's name.

[]{#index-environment_003a-1}

**environment: anObject**

Set the environment to which the receiver is connected. This can be the
class for a dictionary that holds class variables, or the
super-namespace. In general it is used to compute the receiver's name.

[]{#index-import_003afrom_003a}

**import: aSymbol from: aNamespace**

Add to the receiver the symbol aSymbol, associated to the same value as
in aNamespace. Fail if aNamespace does not contain the given key.

[]{#index-name-2}

**name**

Answer the receiver's name, which by default is the same as the name of
the receiver's environment.

[]{#index-nameIn_003a-2}

**nameIn: aNamespace**

Answer the receiver's name when referred to from aNamespace; by default
the computation is deferred to the receiver's environment.

------------------------------------------------------------------------

::: header
Next: [BindingDictionary-basic &
copying](BindingDictionary_002dbasic-_0026-copying.html#BindingDictionary_002dbasic-_0026-copying){accesskey="n"
rel="next"}, Up:
[BindingDictionary](BindingDictionary.html#BindingDictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
