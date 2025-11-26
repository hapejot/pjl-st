[]{#PackageLoader-class_002daccessing}

::: header
Next: [PackageLoader
class-loading](PackageLoader-class_002dloading.html#PackageLoader-class_002dloading){accesskey="n"
rel="next"}, Up:
[PackageLoader](PackageLoader.html#PackageLoader){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PackageLoader-class_003a-accessing}

#### 1.128.1 PackageLoader class: accessing {#packageloader-class-accessing .subsection}

[]{#index-builtFilesFor_003a} []{#index-directoryFor_003a-1}

**builtFilesFor: package**

Answer a Set of Strings containing the filenames of the given package's
machine-generated files (relative to the directory answered by
#directoryFor:)

[]{#index-calloutsFor_003a} []{#index-directoryFor_003a-2}

**calloutsFor: package**

Answer a Set of Strings containing the filenames of the given package's
required callouts (relative to the directory answered by #directoryFor:)

[]{#index-directoryFor_003a}

**directoryFor: package**

Answer a Directory object to the given package's files

[]{#index-featuresFor_003a}

**featuresFor: package**

Answer a Set of Strings containing the features provided by the given
package.

[]{#index-fileInsFor_003a} []{#index-directoryFor_003a-3}

**fileInsFor: package**

Answer a Set of Strings containing the filenames of the given package's
file-ins (relative to the directory answered by #directoryFor:)

[]{#index-filesFor_003a} []{#index-directoryFor_003a-4}

**filesFor: package**

Answer a Set of Strings containing the filenames of the given package's
files (relative to the directory answered by #directoryFor:)

[]{#index-flush-2}

**flush**

Set to reload the 'packages.xml' file the next time it is needed.

[]{#index-ignoreCallouts}

**ignoreCallouts**

Answer whether unavailable C callouts must generate errors or not.

[]{#index-ignoreCallouts_003a}

**ignoreCallouts: aBoolean**

Set whether unavailable C callouts must generate errors or not.

[]{#index-librariesFor_003a} []{#index-directoryFor_003a-5}

**librariesFor: package**

Answer a Set of Strings containing the filenames of the given package's
libraries (relative to the directory answered by #directoryFor:)

[]{#index-modulesFor_003a} []{#index-directoryFor_003a-6}

**modulesFor: package**

Answer a Set of Strings containing the filenames of the given package's
modules (relative to the directory answered by #directoryFor:)

[]{#index-packageAt_003a}

**packageAt: package**

Answer a Package object for the given package

[]{#index-packageAt_003aifAbsent_003a}

**packageAt: package ifAbsent: aBlock**

Answer a Package object for the given package

[]{#index-prerequisitesFor_003a}

**prerequisitesFor: package**

Answer a Set of Strings containing the prerequisites for the given
package

[]{#index-refresh-2}

**refresh**

Reload the 'packages.xml' file in the image and kernel directories. The
three possible places are 1) the kernel directory's parent directory, 2)
the '.st' subdirectory of the user's home directory, 3) the local image
directory (in order of decreasing priority).

For a packages.xml found in the kernel directory's parent directory, all
three directories are searched. For a packages.xml found in the '.st'
subdirectory, only directories 2 and 3 are searched. For a packages.xml
directory in the local image directory, finally, only directory 3 is
searched.

[]{#index-sunitScriptFor_003a}

**sunitScriptFor: package**

Answer a Strings containing a SUnit script that describes the package's
test suite.

------------------------------------------------------------------------

::: header
Next: [PackageLoader
class-loading](PackageLoader-class_002dloading.html#PackageLoader-class_002dloading){accesskey="n"
rel="next"}, Up:
[PackageLoader](PackageLoader.html#PackageLoader){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
