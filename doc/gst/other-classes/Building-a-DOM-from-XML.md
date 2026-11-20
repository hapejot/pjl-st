[]{#Building-a-DOM-from-XML}

::: header
Next: [Building XML](Building-XML.html#Building-XML){accesskey="n"
rel="next"}, Up: [XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Building-a-DOM-from-XML-1}

### 8.1 Building a DOM from XML {#building-a-dom-from-xml .section}

If you're like me, the first thing you may be trying to do is build a
Document Object Model (DOM) tree from some kind of XML input. Assuming
you've got the XML in a String the following code will build an XML
Document:

::: example
``` example
XML.SAXParser defaultParserClass processDocumentString: theXMLString 
    beforeScanDo: [ :p | p validate: false].
```
:::

Though the code above appears as though it should be easy to use,
there's some hidden features you should know about. First,
`theXMLString` can not contain any null bytes. Depending on where your
XML comes from it may have a NULL byte at the end (like mine did). Many
languages implement strings as an array of bytes (usually printable
ones) ending with a null (a character with integer value 0). In my case,
the XML was coming from a remote client written in C using middleware to
send the message to my server. Since the middleware doesn't assume to
know anything about the message it received, it's received into a
String, null-byte and all. To remove it I used:

::: example
``` example
XML.SAXParser defaultParserClass
    processDocumentString: (aString copyWithout: 0 asCharacter)
    beforeScanDo: [ :p | p validate: false].
```
:::

Starting out, I didn't know much about the value of DTDs either
(Document Type Definitions), so I wasn't using them (more on why you
should later). What you need to know is XML comes in two flavors, (three
if you include broken as a flavor) *well-formed* and *valid*.

*Well-formed XML* is simply XML following the basic rules, like only one
top-level (the document's root), no overlapping tags, and a few other
contraints. Valid XML means not only is the XML well-formed, but it's
also compliant with some kind of rule base about which elements are
allowed to follow which other ones, whether or not attributes are
permitted and what their values and defaults should be, etc.

There's no way to get around well-formedness. Most XML tools complain
vociferously about missing or open tags. What you may not have lying
around, though, is a DTD describing how the XML should be assembled. If
you need to skip validation for any reason you must include the
selector:

::: example
``` example
beforeScanDo: [ :p | p validate: false].
```
:::

Now that you have your XML document, you probably want to access its
contents (why else would you want one, right?). Let's take the following
(brief) XML as an example:

::: example
``` example
<porder porder_num="10351">
  <porder_head>
    <order_date>01/04/2000</order_date>
  </porder_head>
  <porder_line>
    <part>widget</part>
    <quantity>1.0000</quantity>
  </porder_line>
  <porder_line>
    <part>doodad</part>
    <quantity>2.0000</quantity>
  </porder_line>
</porder>
```
:::

The first thing you probably want to know is how to access the different
tags, and more specifically, how to access the contents of those tags.
First, by way of providing a roadmap to the elements I'll show you the
Smalltalk code for getting different pieces of the document, assuming
the variable you've assigned the document to is named *doc*. I'll also
create instance variables for the various elements as I go along:

  ------------------------------------- -------------------------------------------------------------------------------------
  *Element you want*                    *Code to get it*
  porder element                        `doc root`
  porder_head                           `doc root elementNamed: 'porder_head'`
  order_date (as a String)              `(porderHead elementNamed: 'order_date') characterData`
  order_date (as a Date)                `(Date readFrom: (porderHead elementNamed: 'order_date') characterData readStream)`
  a collection with both porder_lines   `doc root elementsNamed: 'porder_line'`
  ------------------------------------- -------------------------------------------------------------------------------------

I've deliberately left-out accessing `porder`'s attribute because
accessing them is different from accessing other nodes. You can get an
OrderedCollection of attributes using:

::: example
``` example
attributes := doc root attributes.
```
:::

but the ordered collection isn't really useful. To access any single
attribute you'd need to look for it in the collection:

::: example
``` example
porderNum := (attributes detect: [ :each | each key type = 'porder_num' ]) value.
```
:::

But that's not a whole lot of fun, especially if there's a lot you need
to get, and if there's any possibility the attribute may not exist. Then
you have to do the whole `detect:ifNone:` thing, and boy, does that make
the code readable! What I did instead was create a method in my objects'
abstract:

::: example
``` example
dictionaryForAttributes: aCollection
    ^Dictionary withAll: (aCollection
    collect: [ :each | each key type -> each value ])
```
:::

Now what you have is an incrementally more useful method for getting
attributes:

::: example
``` example
attributes := self dictionaryForAttributes: doc root attributes.
porderNum := attributes at: 'porder_num'.
```
:::

At first this appears like more code, and for a single attribute it
probably is. But if an element includes more than one attribute the
payoff is fairly decent. Of course, you still need to handle the absence
of an attribute in the dictionary but I think it reads a little better
using a Dictionary than an OrderedCollection:

::: example
``` example
porderNum := attributes at: 'porder_num' ifAbsent: [].
```
:::

------------------------------------------------------------------------

::: header
Next: [Building XML](Building-XML.html#Building-XML){accesskey="n"
rel="next"}, Up: [XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
