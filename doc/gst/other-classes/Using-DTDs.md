[]{#Using-DTDs}

::: header
Next: [XSL Processing](XSL-Processing.html#XSL-Processing){accesskey="n"
rel="next"}, Previous: [Building
XML](Building-XML.html#Building-XML){accesskey="p" rel="prev"}, Up:
[XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Using-DTDs-1}

### 8.3 Using DTDs {#using-dtds .section}

What I didn't appreciate in my first XML project (this one) was how much
error checking I was doing just to verify the format of incoming XML.
During testing I'd go looking for attributes or elements that *should*
have been there but for various reasons were not. Because I was coding
fast and furious I overlooked some and ignored others. Testing quickly
ferreted out my carelessnes and my application started throwing
exceptions faster than election officials throw chads.

The cure, at least for formatting, is having a DTD, or Document Type
Definition describing the XML format. You can read more about the syntax
of DTDs in the XML specification.

There's not a lot programmers are able to do with DTDs in VisualWorks,
except requiring incoming XML to include DOCTYPE statements. There is
something programmers need to do to handle the exceptions the XML parser
throws when it finds errors.

I'm not an expert at writing Smalltalk exception handling code, and I
haven't decided on what those exceptions should look like to the client
who sent the poorly formatted XML in the first place. The code below
does a decent job of catching the errors and putting the description of
the error into an XML response. It's also a fairly decent example of XML
document building as discussed earlier.

::: example
``` example
replyDoc := XML.Document new.
replyDoc addNode: (XML.Element tag: 'response').

[
    doc := XML.SAXParser defaultParserClass processDocumentString: (anIsdMessage message copyWithout: 0) asString
] on: Exception do: [ :ex |
    replyDoc root 
        addAttribute: (XML.Attribute name: 'type' value: 'Exception');
        addNode: ((XML.Element tag: 'description')
            addNode: (XML.Text text: ex signal description));
        addNode: ((XML.Element tag: 'message')
            addNode: (XML.Text text: ex messageText))
].
```
:::

I said before there's not a lot programmers can do with DTDs, but there
are some things I wish the XML library would do:

-   I'd like to make sure the documents I build are built correctly. It
    would be great if a DTD could be attached to an empty XML document
    so that exceptions could be thrown as misplaced elements were added.
-   It would be great to specify which DTD the XML parser should use
    when parsing incoming XML so that the incoming XML wouldn't always
    have to include a \<!DOCTYPE\> tag. Though it's fairly easy to add
    the tag at the start of XML text, it's really not that simple. You
    need to know the XML's root element before adding the \<!DOCTYPE\>
    tag but you really don't know that until after you've parsed the XML
    You would have to parse the XML, determine the root tag, then parse
    the output of the first into a new XML document with validation
    turned-on.
-   Another reason to be able to create a DTD document to use with
    subsequent parsing is to avoid the overhead of parsing the same DTD
    over and over again. In transaction processing systems this kind of
    redundant task could be eliminated and the spare CPU cycles put to
    better use.

------------------------------------------------------------------------

::: header
Next: [XSL Processing](XSL-Processing.html#XSL-Processing){accesskey="n"
rel="next"}, Previous: [Building
XML](Building-XML.html#Building-XML){accesskey="p" rel="prev"}, Up:
[XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
