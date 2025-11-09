| factorial |
        factorial := [:n| n = 1 ifTrue: [1] ifFalse: [(factorial value: n - 1) * n]].
        (1 to: 10) collect: factorial
