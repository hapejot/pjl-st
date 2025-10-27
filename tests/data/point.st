Point := Object subclass: [
    | x y |
    
    initializeWithX: xVal y: yVal [
        x := xVal.
        y := yVal.
        ^self
    ]
    
    x [^x]
    y [^y]
    
    + aPoint [
        ^Point new initializeWithX: (x + aPoint x) y: (y + aPoint y)
    ]
    
    printString [
        ^'Point(', x printString, ',', y printString, ')'
    ]
]