[]{#ClassDescription_002dorganization-of-messages-and-classes}

::: header
Next: [ClassDescription-parsing class
declarations](ClassDescription_002dparsing-class-declarations.html#ClassDescription_002dparsing-class-declarations){accesskey="n"
rel="next"}, Previous:
[ClassDescription-filing](ClassDescription_002dfiling.html#ClassDescription_002dfiling){accesskey="p"
rel="prev"}, Up:
[ClassDescription](ClassDescription.html#ClassDescription){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ClassDescription_003a-organization-of-messages-and-classes}

#### 1.32.5 ClassDescription: organization of messages and classes {#classdescription-organization-of-messages-and-classes .subsection}

[]{#index-classify_003aunder_003a}

**classify: aSelector under: aString**

Put the method identified by the selector aSelector under the category
given by aString.

[]{#index-createGetMethod_003a-1}

**createGetMethod: what**

Create a method accessing the variable 'what'.

[]{#index-createGetMethod_003adefault_003a-1}

**createGetMethod: what default: value**

Create a method accessing the variable 'what', with a default value of
'value', using lazy initialization

[]{#index-createSetMethod_003a-1}

**createSetMethod: what**

Create a method which sets the variable 'what'.

[]{#index-defineAsyncCFunc_003awithSelectorArgs_003aargs_003a-1}

**defineAsyncCFunc: cFuncNameString withSelectorArgs: selectorAndArgs
args: argsArray**

See documentation. This function is deprecated, you should use the
\<asyncCCall: \... \> special syntax instead.

[]{#index-defineCFunc_003awithSelectorArgs_003areturning_003aargs_003a-1}

**defineCFunc: cFuncNameString withSelectorArgs: selectorAndArgs
returning: returnTypeSymbol args: argsArray**

See documentation. This function is deprecated, you should use the
\<asyncCCall: \... \> special syntax instead.

[]{#index-removeCategory_003a}

**removeCategory: aString**

Remove from the receiver every method belonging to the given category

[]{#index-whichCategoryIncludesSelector_003a}

**whichCategoryIncludesSelector: selector**

Answer the category for the given selector, or nil if the selector is
not found
