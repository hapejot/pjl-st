[]{#BLOX_002eBMenu_002dcallback-registration}

::: header
Previous:
[BLOX.BMenu-accessing](BLOX_002eBMenu_002daccessing.html#BLOX_002eBMenu_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BMenu](BLOX_002eBMenu.html#BLOX_002eBMenu){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBMenu_003a-callback-registration}

#### 1.27.3 BLOX.BMenu: callback registration {#blox.bmenu-callback-registration .subsection}

[]{#index-addLine}

**addLine**

Add a separator item at the end of the menu

[]{#index-addMenuItemFor_003anotifying_003a}

**addMenuItemFor: anArray notifying: receiver**

Add a menu item described by anArray at the end of the menu. If anArray
is empty, insert a separator line. If anArray has a single item, a menu
item is created without a callback. If anArray has two or three items,
the second one is used as the selector sent to receiver, and the third
one (if present) is passed to the selector.

[]{#index-callback_003ausing_003a}
[]{#index-addMenuItemFor_003anotifying_003a-1}

**callback: receiver using: selectorPairs**

Add menu items described by anArray at the end of the menu. Each element
of selectorPairs must be in the format described in
BMenu\>\>#addMenuItemFor:notifying:. All the callbacks will be sent to
receiver.

[]{#index-destroy-1}

**destroy**

Destroy the menu widget; that is, simply remove ourselves from the
parent menu bar.

[]{#index-empty-1}

**empty**

Empty the menu widget; that is, remove all the children
