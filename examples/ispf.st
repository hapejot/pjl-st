| option dataset status field running redraw event key modifiers kind dimensions columns rows allowed message |
option := ''.
dataset := ''.
status := 'Bitte Option und Dataset eingeben.'.
field := 1.
running := true.
redraw := true.
allowed := 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._-/#$@ '.

Terminal with: [:screen |
    [running] whileTrue: [
        dimensions := screen size.
        columns := dimensions at: 1.
        rows := dimensions at: 2.
        screen hideCursor.
        redraw ifTrue: [
            screen background: #black.
            screen foreground: #cyan.
            screen clear.
            redraw := false.

            (columns >= 72 and: [rows >= 20]) ifTrue: [
                screen moveToColumn: 1 row: 0.
                screen write: '------------------ SMALLTALK ISPF-DEMO ------------------'.
                screen moveToColumn: 1 row: 2.
                screen foreground: #white.
                screen write: 'OPTION ===>'.

                screen foreground: #cyan.
                screen moveToColumn: 1 row: 4.
                screen write: '1  Anzeigen       Dataset-Namen anzeigen'.
                screen moveToColumn: 1 row: 5.
                screen write: '2  Bearbeiten     Bearbeitung simulieren'.
                screen moveToColumn: 1 row: 6.
                screen write: '3  Zuruecksetzen  Eingaben loeschen'.

                screen foreground: #white.
                screen moveToColumn: 1 row: 9.
                screen write: 'DATASET ===>'.

                screen foreground: #yellow.
                screen moveToColumn: 1 row: 13.
                screen write: 'MELDUNG:'.

                screen foreground: #cyan.
                screen moveToColumn: 1 row: 18.
                screen write: 'Enter Ausfuehren   Tab Feldwechsel   F3/Esc Ende'.
                screen moveToColumn: 1 row: 19.
                screen write: '----------------------------------------------------'
            ]
            ifFalse: [
                message := 'Terminal zu klein: mindestens 72 x 20.'.
                (columns > 0 and: [rows > 0]) ifTrue: [
                    columns < message size ifTrue: [
                        message := message copyFrom: 1 to: columns
                    ].
                    screen moveToColumn: 0 row: 0.
                    screen write: message
                ]
            ]
        ].
        (columns >= 72 and: [rows >= 20]) ifTrue: [
            screen foreground: #green.
            screen moveToColumn: 16 row: 2.
            screen write: '        '.
            screen moveToColumn: 16 row: 2.
            screen write: option.
            screen moveToColumn: 16 row: 9.
            screen write: '                        '.
            screen moveToColumn: 16 row: 9.
            screen write: dataset.

            screen foreground: #yellow.
            screen moveToColumn: 1 row: 14.
            screen write: '                                                                      '.
            screen moveToColumn: 1 row: 14.
            screen write: status.
            field = 1
                ifTrue: [screen moveToColumn: 16 + option size row: 2]
                ifFalse: [screen moveToColumn: 16 + dataset size row: 9].
            screen showCursor
        ].

        event := screen nextEvent.
        ((event at: 1) = #resize) ifTrue: [redraw := true].
        ((event at: 1) = #key) ifTrue: [
            key := event at: 2.
            modifiers := event at: 3.
            kind := event at: 4.
            (kind = #release) ifFalse: [
                ((key = #escape) or: [key = 3]) ifTrue: [running := false].
                (columns >= 72 and: [rows >= 20]) ifTrue: [
                    ((key = #tab) or: [key = #backTab]) ifTrue: [
                        field = 1 ifTrue: [field := 2] ifFalse: [field := 1]
                    ].
                    (key = #backspace) ifTrue: [
                        field = 1
                            ifTrue: [
                                option size > 0 ifTrue: [
                                    option := option copyFrom: 1 to: option size - 1
                                ]
                            ]
                            ifFalse: [
                                dataset size > 0 ifTrue: [
                                    dataset := dataset copyFrom: 1 to: dataset size - 1
                                ]
                            ]
                    ].
                    (key = #enter) ifTrue: [
                        status := 'Ungueltige Option. Bitte 1, 2 oder 3 eingeben.'.
                        (option = '1') ifTrue: [
                            status := 'Anzeigen: ', dataset, ' (Demo)'
                        ].
                        (option = '2') ifTrue: [
                            status := 'Bearbeiten: ', dataset, ' (Demo)'
                        ].
                        (option = '3') ifTrue: [
                            option := ''.
                            dataset := ''.
                            field := 1.
                            status := 'Eingaben geloescht.'
                        ]
                    ].
                    (key isCharacter and: [
                        (allowed includes: key) and: [
                            (modifiers includes: #control) not and: [
                                (modifiers includes: #alt) not and: [
                                    (modifiers includes: #super) not and: [
                                        (modifiers includes: #hyper) not and: [
                                            (modifiers includes: #meta) not
                                        ]
                                    ]
                                ]
                            ]
                        ]
                    ]) ifTrue: [
                        field = 1
                            ifTrue: [
                                option size < 8 ifTrue: [option := option, key asString]
                            ]
                            ifFalse: [
                                dataset size < 24 ifTrue: [dataset := dataset, key asString]
                            ]
                    ]
                ]
            ]
        ]
    ]
].
^nil
