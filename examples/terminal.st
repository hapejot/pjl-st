| finished event |
Terminal with: [:screen |
    finished := false.
    [finished not] whileTrue: [
        screen clear.
        screen moveToColumn: 2 row: 1.
        screen foreground: #cyan.
        screen write: 'Smalltalk Terminal'.
        screen resetColors.
        screen moveToColumn: 2 row: 3.
        screen write: 'Press q to quit. Resize the terminal to redraw.'.
        event := screen nextEvent.
        ((event at: 1) = #key) ifTrue: [
            ((event at: 2) = $q) ifTrue: [finished := true]
        ]
    ]
].
^nil
