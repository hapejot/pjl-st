[]{#Package_002daccessing}

::: header
Next: [Package-still
unclassified](Package_002dstill-unclassified.html#Package_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [Package class-instance
creation](Package-class_002dinstance-creation.html#Package-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Package](Package.html#Package){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Package_003a-accessing}

#### 1.127.3 Package: accessing {#package-accessing .subsection}

[]{#index-addBuiltFile_003a}

**addBuiltFile: aString**

Not commented.

[]{#index-addCallout_003a}

**addCallout: aString**

Not commented.

[]{#index-addFeature_003a}

**addFeature: aString**

Not commented.

[]{#index-addFile_003a}

**addFile: aString**

Not commented.

[]{#index-addFileIn_003a}

**addFileIn: aString**

Not commented.

[]{#index-addLibrary_003a-1}

**addLibrary: aString**

Not commented.

[]{#index-addModule_003a-1}

**addModule: aString**

Not commented.

[]{#index-addPrerequisite_003a}

**addPrerequisite: aString**

Not commented.

[]{#index-addSunitScript_003a}

**addSunitScript: aString**

Not commented.

[]{#index-baseDirectories}

**baseDirectories**

Answer 'baseDirectories'.

[]{#index-baseDirectories_003a}

**baseDirectories: aCollection**

Check if it's possible to resolve the names in the package according to
the base directories in baseDirectories, which depend on where the
packages.xml is found: the three possible places are 1) the system
kernel directory's parent directory, 2) the local kernel directory's
parent directory, 3) the local image directory (in order of decreasing
priority).

For a packages.xml found in the system kernel directory's parent
directory, all three directories are searched. For a packages.xml found
in the local kernel directory's parent directory, only directories 2 and
3 are searched. For a packages.xml directory in the local image
directory, instead, only directory 3 is searched.

[]{#index-builtFiles}

**builtFiles**

Answer a (modifiable) OrderedCollection of files that are part of the
package but are not distributed.

[]{#index-callouts}

**callouts**

Answer a (modifiable) Set of call-outs that are required to load the
package. Their presence is checked after the libraries and modules are
loaded so that you can do a kind of versioning.

[]{#index-directory-1}

**directory**

Answer the base directory from which to load the package.

[]{#index-features}

**features**

Answer a (modifiable) Set of features provided by the package.

[]{#index-fileIns}

**fileIns**

Answer a (modifiable) OrderedCollections of files that are to be
filed-in to load the package. This is usually a subset of 'files' and
'builtFiles'.

[]{#index-files-1}

**files**

Answer a (modifiable) OrderedCollection of files that are part of the
package.

[]{#index-fullPathOf_003a}

**fullPathOf: fileName**

Try appending 'self directory' and fileName to each of the directory in
baseDirectories, and return the path to the first tried filename that
exists. Raise a PackageNotAvailable exception if no directory is found
that contains the file.

[]{#index-libraries}

**libraries**

Answer a (modifiable) Set of shared library names that are required to
load the package.

[]{#index-modules}

**modules**

Answer a (modifiable) Set of modules that are required to load the
package.

[]{#index-namespace}

**namespace**

Answer the namespace in which the package is loaded.

[]{#index-namespace_003a}

**namespace: aString**

Set to aString the namespace in which the package is loaded.

[]{#index-prerequisites}

**prerequisites**

Answer a (modifiable) Set of prerequisites.

[]{#index-primFileIn}

**primFileIn**

Private - File in the given package without paying attention at
dependencies and C callout availability

[]{#index-relativeDirectory}

**relativeDirectory**

Answer the directory, relative to the packages file, from which to load
the package.

[]{#index-relativeDirectory_003a}

**relativeDirectory: dir**

Set the directory, relative to the packages file, from which to load the
package, to dir.

[]{#index-startScript}

**startScript**

Answer the start script for the package.

[]{#index-startScript_003a}

**startScript: aString**

Set the start script for the package to aString.

[]{#index-stopScript}

**stopScript**

Answer the start script for the package.

[]{#index-stopScript_003a}

**stopScript: aString**

Set the stop script for the package to aString.

[]{#index-sunitScripts}

**sunitScripts**

Answer a (modifiable) OrderedCollection of SUnit scripts that compose
the package's test suite.

[]{#index-test}

**test**

Answer the test sub-package.

[]{#index-test_003a}

**test: aPackage**

Set the test sub-package to be aPackage.

[]{#index-url}

**url**

Answer the URL at which the package repository can be found.

[]{#index-url_003a}

**url: aString**

Set to aString the URL at which the package repository can be found.

[]{#index-version}

**version**

Not commented.

[]{#index-version_003a}

**version: aVersion**

Not commented.

------------------------------------------------------------------------

::: header
Next: [Package-still
unclassified](Package_002dstill-unclassified.html#Package_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [Package class-instance
creation](Package-class_002dinstance-creation.html#Package-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Package](Package.html#Package){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
