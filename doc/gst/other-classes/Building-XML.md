[]{#Building-XML}

::: header
Next: [Using DTDs](Using-DTDs.html#Using-DTDs){accesskey="n"
rel="next"}, Previous: [Building a DOM from
XML](Building-a-DOM-from-XML.html#Building-a-DOM-from-XML){accesskey="p"
rel="prev"}, Up: [XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Building-XML-1}

### 8.2 Building XML {#building-xml .section}

There's little reason to build an XML document if its not going to be
processed by something down the road. Most XML tools require XML
documents have a document root. A root is a tag inside which all other
tags exist, or put another way, a single parent node from which all
other nodes descend. In my case, a co-worker was attempting to use
Sablot's sabcmd to transform the XML from my server into HTML. So start
your document with the root ready to go:

::: example
``` example
replyDoc := XML.Document new.
replyDoc addNode: (XML.Element tag: 'response').
```
:::

Before doing anything more complex, we can play with our new XML
document. Assuming you're going to want to send the XML text to someone
or write it to a file, you may first want to capture it in a string.
Even if you don't want to first capture it into a string our example is
going to:

::: example
``` example
replyStream := String new writeStream.
replyDoc printOn: replyStream.
```
:::

If we examine'd the contents of our replyStream (`replyStream contents`)
we'd see:

::: example
``` example
<response/>
```
:::

Which is what an empty tag looks like.

Let's add some text to our XML document now. Let's say we want it to
look like:

::: example
``` example
<response>Hello, world!</response>
```
:::

Building this actually requires two nodes be added to a new XML
document. The first node (or element) is named `response`. The second
node adds text to the first:

::: example
``` example
replyDoc := XML.Document new.
replyDoc addNode: (XML.Element tag: response). "our root node"
replyDoc root addNode: (XML.Text text: 'Hello, world!').
```
:::

Another way of writing it, and the way I've adopted in my code is to
create the whole node before adding it. This is not just to reduce the
appearance of assignments, but it suggests a template for cascading
`#addNode:` messages to an element, which, if you're building any kind
of nontrivial XML, you'll be doing a lot of:

::: example
``` example
replyDoc := XML.Document new.
replyDoc addNode: (
    (XML.Element tag: response)
        addNode: (XML.Text text: 'Hello, world!')
).
```
:::

Unless you're absolutely sure you'll never accidentally add text nodes
that have an ampersand (&) in them, you'll need to escape it to get past
XML parsers. The way I got around this was to escape them whenever I
added text nodes. To make it easier, I (again) created a method in my
objects' abstract superclass:

::: example
``` example
asXMLElement: tag value: aValue
    | n |

    n := XML.Element tag: tag.
    aValue isNil ifFalse: [
    n addNode: (XML.Text
        text: (aValue displayString copyReplaceAll: '&' with: '&amp;'))].
    ^n
```
:::

Calls to `self asXMLElement: 'sometagname' value: anInstanceVariable`
are littered throughout my code.

Adding attributes to documents is, thankfully, easier than accessing
them. If we wanted to add an attribute to our document above we can do
so with a single statement:

::: example
``` example
replyDoc root addAttribute: (XML.Attribute name: 'isExample' value: 'yes').
```
:::

Now, our XML looks like:

::: example
``` example
<response isExample="yes">Hello, world!</response>
```
:::

------------------------------------------------------------------------

::: header
Next: [Using DTDs](Using-DTDs.html#Using-DTDs){accesskey="n"
rel="next"}, Previous: [Building a DOM from
XML](Building-a-DOM-from-XML.html#Building-a-DOM-from-XML){accesskey="p"
rel="prev"}, Up: [XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
