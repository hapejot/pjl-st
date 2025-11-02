 | adder1 adder2 |
        adder1 := [:x :y | x + y ].
        adder2 := [:x | adder1 value: 1 value: x].
        ^adder2 value: 2