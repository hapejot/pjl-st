# Binary Messages

Binary Messages
3 + 4 " ==> 7 "
3 + 4 * 5 " ==> 35 (  23) "
3 + 4 factorial " ==> 27 (  5040) "
total - 1
total <= max  "true if total is less than or equal to max"
(4/3)*3 = 4 "==> true  equality is just a binary message, and Fractions are exact"
(3/4) == (3/4) "==> false  two equal Fractions, but not the same object"
not
not
Binary messages have a receiver, the left hand side, and a single argument, the right hand side. The first expression above sends 3 the 
message comprising the selector #+ with the argument 4.
Binary messages are  parsed left to right, without regard to precedence of numeric operators, unless corrected with parentheses.always