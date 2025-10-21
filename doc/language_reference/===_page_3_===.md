# === Page 3 ===

=== PAGE 3 ===
General Editing Commands
Key Description Notes
z Undo  
x Cut  
c Copy  
v Paste  
a Select all  
D Duplicate. Paste the current selection over the prior selection, if it is non-overlapping and legal 1
e Exchange. Exchange the contents of currentselection with the contents of the prior selection 1
y Swap. If there is no selection, swap the characters on either side of the insertion cursor, and advance the 
cursor. If the selection has 2 characters, swap them, and advance the cursor.  
w Delete preceding word  
Notes
1. These commands are a bit unusual: they concern and affect not only the current selection, but also the immediately preceding se lection.
Search and Replace
Key Description Notes
f Find. Set the search string from a string entered in a dialog. Then, advance the cursor to the next 
occurrence of the search string.  
g Find again. Advance the cursor to the next occurrence of the search string.  
h Set Search String from the selection.  
j Replace the next occurrence of the search string with the last replacement made  
A Advance argument. Advance the cursor to the next keyword argument, or to the end of string if no
keyword arguments remain.
 
J Replace all occurrences of the search string with the last replacement made  
S Replace all occurrences of the search string with the present change text  
Cancel/Accept
Key Description Notes
l Cancel (also "revert"). Cancel all edits made since the pane was opened or since the last save  
s Accept (also "save"). Save the changes made in the current pane.  
o Spawn. Open a new window containing the present contents of this pane, and then reset this window to 
its last saved state (that is, cancel the present window).  