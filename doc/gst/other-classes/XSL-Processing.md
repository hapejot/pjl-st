[]{#XSL-Processing}

::: header
Next: [Attributions](Attributions.html#Attributions){accesskey="n"
rel="next"}, Previous: [Using
DTDs](Using-DTDs.html#Using-DTDs){accesskey="p" rel="prev"}, Up:
[XML/XPath/XSL
packages](XML_002fXPath_002fXSL-packages.html#XML_002fXPath_002fXSL-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#XSL-Processing-1}

### 8.4 XSL Processing {#xsl-processing .section}

I spent a night the other week trying to figure out how to get the XSL
libraries to do anything. I no longer need it now, but I did discover
some things others with an immediate need may want to be aware of.

-   Transforming an XML document requires you parse the XSL and XML
    documents separately first. After that, you tell the
    XSL.RuleDatabase to process the XML document. The result is another
    XML document with the transformations.

    A code snippet for doing just that appears below.

    ::: example
    ``` example
    | rules xmlDoc htmlDoc |

    rules := XSL.RuleDatabase new readFileNamed: 'paymentspending.xsl'.
    xmlDoc := XML.SAXParser defaultParserClass
                 processDocumentInFilename: 'paymentspending.xml'
                 beforeScanDo: [ :p | p validate: false ].
    htmlDoc := rules process: xmlDoc.
    ```
    :::

    There is also a `readString:` method which can be used instead of
    `readFileNamed:`.

-   The XSL library doesn't use the W3-approved stylesheet, but instead
    uses the draft version (same one Microsoft uses).
    `<xsl:stylesheet xmlns:xsl="http://www.w3.org/TR/WD-xsl">`

-   The functions `position()` and `count()` aren't implemented, or if
    they are, aren't implemented in the way other XSL tools implement
    it.
