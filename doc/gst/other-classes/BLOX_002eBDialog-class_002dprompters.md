[]{#BLOX_002eBDialog-class_002dprompters}

::: header
Next:
[BLOX.BDialog-accessing](BLOX_002eBDialog_002daccessing.html#BLOX_002eBDialog_002daccessing){accesskey="n"
rel="next"}, Previous: [BLOX.BDialog class-instance
creation](BLOX_002eBDialog-class_002dinstance-creation.html#BLOX_002eBDialog-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BDialog](BLOX_002eBDialog.html#BLOX_002eBDialog){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDialog-class_003a-prompters}

#### 1.11.2 BLOX.BDialog class: prompters {#blox.bdialog-class-prompters .subsection}

[]{#index-chooseColor_003alabel_003adefault_003a}

**chooseColor: parent label: aLabel default: color**

Prompt for a color. The dialog box is created with the given parent
window and with aLabel as its title bar text, and initially it selects
the color given in the color parameter.

If the dialog box is canceled, nil is answered, else the selected color
is returned as a String with its RGB value.

[]{#index-chooseFileToOpen_003alabel_003adefault_003adefaultExtension_003atypes_003a}

**chooseFileToOpen: parent label: aLabel default: name defaultExtension:
ext types: typeList**

Pop up a dialog box for the user to select a file to open. Its purpose
is for the user to select an existing file only. If the user enters an
non-existent file, the dialog box gives the user an error prompt and
requires the user to give an alternative selection or to cancel the
selection. If an application allows the user to create new files, it
should do so by providing a separate New menu command.

If the dialog box is canceled, nil is answered, else the selected file
name is returned as a String.

The dialog box is created with the given parent window and with aLabel
as its title bar text. The name parameter indicates which file is
initially selected, and the default extension specifies a string that
will be appended to the filename if the user enters a filename without
an extension.

The typeList parameter is an array of arrays, like #(('Text files'
'.txt' '.diz') ('Smalltalk files' '.st')), and is used to construct a
listbox of file types. When the user chooses a file type in the listbox,
only the files of that type are listed. Each item in the array contains
a list of strings: the first one is the name of the file type described
by a particular file pattern, and is the text string that appears in the
File types listbox, while the other ones are the possible extensions
that belong to this particular file type.

[]{#index-chooseFileToSave_003alabel_003adefault_003adefaultExtension_003atypes_003a}

**chooseFileToSave: parent label: aLabel default: name defaultExtension:
ext types: typeList**

Pop up a dialog box for the user to select a file to save; this differs
from the file open dialog box in that non-existent file names are
accepted and existing file names trigger a confirmation dialog box,
asking the user whether the file should be overwritten or not.

If the dialog box is canceled, nil is answered, else the selected file
name is returned as a String.

The dialog box is created with the given parent window and with aLabel
as its title bar text. The name parameter indicates which file is
initially selected, and the default extension specifies a string that
will be appended to the filename if the user enters a filename without
an extension.

The typeList parameter is an array of arrays, like #(('Text files'
'.txt' '.diz') ('Smalltalk files' '.st')), and is used to construct a
listbox of file types. When the user chooses a file type in the listbox,
only the files of that type are listed. Each item in the array contains
a list of strings: the first one is the name of the file type described
by a particular file pattern, and is the text string that appears in the
File types listbox, while the other ones are the possible extensions
that belong to this particular file type.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BDialog-accessing](BLOX_002eBDialog_002daccessing.html#BLOX_002eBDialog_002daccessing){accesskey="n"
rel="next"}, Previous: [BLOX.BDialog class-instance
creation](BLOX_002eBDialog-class_002dinstance-creation.html#BLOX_002eBDialog-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BDialog](BLOX_002eBDialog.html#BLOX_002eBDialog){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
