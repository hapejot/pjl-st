[]{#DLD-class_002ddynamic-linking}

::: header
Previous: [DLD class-C
call-outs](DLD-class_002dC-call_002douts.html#DLD-class_002dC-call_002douts){accesskey="p"
rel="prev"}, Up: [DLD](DLD.html#DLD){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DLD-class_003a-dynamic-linking}

#### 1.67.2 DLD class: dynamic linking {#dld-class-dynamic-linking .subsection}

[]{#index-addLibrary_003a}

**addLibrary: library**

Add library to the search path of libraries to be used by DLD.

[]{#index-addLibraryHandle_003a}

**addLibraryHandle: libraryHandle**

This is called internally by gst_dlopen. The library will be open and
put in the search path.

[]{#index-addModule_003a}

**addModule: library**

Add library to the list of modules to be loaded when the image is
started. The gst_initModule function in the library is called, but the
library will not be put in the search path used whenever a C function is
requested but not registered.

[]{#index-defineExternFunc_003a} []{#index-primDefineExternFunc_003a-1}

**defineExternFunc: aFuncName**

This method calls #primDefineExternFunc: to try to link to a function
with the given name, and answers whether the linkage was successful. You
can redefine this method to restrict the ability to do dynamic linking.

[]{#index-initialize-6}

**initialize**

Private - Initialize the receiver's class variables

[]{#index-libraryList}

**libraryList**

Answer a copy of the search path of libraries to be used by DLD

[]{#index-moduleList}

**moduleList**

Answer a copy of the modules reloaded when the image is started

[]{#index-primDefineExternFunc_003a}

**primDefineExternFunc: aFuncName**

This method tries to link to a function with the given name, and answers
whether the linkage was successful. It should not be overridden.

[]{#index-update_003a-1}

**update: aspect**

Called on startup - Make DLD re-link and reset the addresses of all the
externally defined functions
