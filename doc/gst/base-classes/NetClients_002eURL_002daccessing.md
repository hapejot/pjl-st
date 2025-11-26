[]{#NetClients_002eURL_002daccessing}

::: header
Next:
[NetClients.URL-comparing](NetClients_002eURL_002dcomparing.html#NetClients_002eURL_002dcomparing){accesskey="n"
rel="next"}, Previous: [NetClients.URL class-instance
creation](NetClients_002eURL-class_002dinstance-creation.html#NetClients_002eURL-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[NetClients.URL](NetClients_002eURL.html#NetClients_002eURL){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#NetClients_002eURL_003a-accessing}

#### 1.118.3 NetClients.URL: accessing {#netclients.url-accessing .subsection}

[]{#index-asString-8}

**asString**

Answer the full request string corresponding to the URL. This is how the
URL would be printed in the address bar of a web browser, except that
the query data is printed even if it is to be sent through a POST
request.

[]{#index-decodedFields}

**decodedFields**

Convert the form fields to a Dictionary, answer nil if no question mark
is found in the URL.

[]{#index-decodedFile}

**decodedFile**

Answer the file part of the URL, decoding it from x-www-form-urlencoded
format.

[]{#index-decodedFragment}

**decodedFragment**

Answer the fragment part of the URL, decoding it from
x-www-form-urlencoded format.

[]{#index-fragment}

**fragment**

Answer the fragment part of the URL, leaving it in x-www-form-urlencoded
format.

[]{#index-fragment_003a}

**fragment: aString**

Set the fragment part of the URL, which should be in
x-www-form-urlencoded format.

[]{#index-fullRequestString}

**fullRequestString**

Answer the full request string corresponding to the URL. This is how the
URL would be printed in the address bar of a web browser, except that
the query data is printed even if it is to be sent through a POST
request.

[]{#index-hasPostData}

**hasPostData**

Answer whether the URL has a query part but is actually for an HTTP POST
request and not really part of the URL (as it would be for the HTTP GET
request).

[]{#index-hasPostData_003a}

**hasPostData: aBoolean**

Set whether the query part of the URL is actually the data for an HTTP
POST request and not really part of the URL (as it would be for the HTTP
GET request).

[]{#index-host}

**host**

Answer the host part of the URL.

[]{#index-host_003a}

**host: aString**

Set the host part of the URL to aString.

[]{#index-newsGroup}

**newsGroup**

If the receiver is an nntp url, return the news group.

[]{#index-password}

**password**

Answer the password part of the URL.

[]{#index-password_003a}

**password: aString**

Set the password part of the URL to aString.

[]{#index-path-2}

**path**

Answer the path part of the URL.

[]{#index-path_003a-1}

**path: aString**

Set the path part of the URL to aString.

[]{#index-port}

**port**

Answer the port number part of the URL.

[]{#index-port_003a}

**port: anInteger**

Set the port number part of the URL to anInteger.

[]{#index-postData}

**postData**

Answer whether the URL has a query part and it is meant for an HTTP POST
request, answer it. Else answer nil.

[]{#index-postData_003a}

**postData: aString**

Associate to the URL some data that is meant to be sent through an HTTP
POST request, answer it.

[]{#index-query}

**query**

Answer the query data associated to the URL.

[]{#index-query_003a}

**query: aString**

Set the query data associated to the URL to aString.

[]{#index-requestString}

**requestString**

Answer the URL as it would be sent in an HTTP stream (that is, the path
and the query data, the latter only if it is to be sent with an HTTP
POST request).

[]{#index-scheme}

**scheme**

Answer the URL's scheme.

[]{#index-scheme_003a}

**scheme: aString**

Set the URL's scheme to be aString.

[]{#index-username}

**username**

Answer the username part of the URL.

[]{#index-username_003a}

**username: aString**

Set the username part of the URL to aString.

------------------------------------------------------------------------

::: header
Next:
[NetClients.URL-comparing](NetClients_002eURL_002dcomparing.html#NetClients_002eURL_002dcomparing){accesskey="n"
rel="next"}, Previous: [NetClients.URL class-instance
creation](NetClients_002eURL-class_002dinstance-creation.html#NetClients_002eURL-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[NetClients.URL](NetClients_002eURL.html#NetClients_002eURL){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
