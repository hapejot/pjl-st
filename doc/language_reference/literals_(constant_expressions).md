# Literals (Constant Expressions)

Literals (Constant Expressions)
Numbers (Instances of class Number)
In the following, ==> means "prints as".
Decimal integer: ,
Octal integer: ,
Hex integer: ,
Arbitrary base integer:  ==> 10
Integer with exponent:  ==> 12300,  ==> 40
Float (double precision): 
Arbitrary base float:  ==> 1.5
Float with exponent:  ==> 6.0
1234 12345678901234567890
8r177 8r1777777777777777777777
16rFF 16r123456789ABCDEF012345
2r1010
123e2 2r1010e2
3.14e-10
2r1.1
2r1.1e2
Squeak supports SmallInteger arithmetic (integers between -2  and 2 ) with fast internal primitives.30 30-1
Squeak supports arbitrary precision arithmetic seamlessly (automatically coercing SmallInteger to LargePositiveInteger and 
LargeNegativeInteger whereappropriate), albeit at a slight cost in speed.
Squeak supports several other kinds of "numeric" value, such as Fractions (arbitrary precision rational numbers) and Points. While there are 
no literals for these objects, they are naturally expressed as operations on built-in literals. ( "2/3" and "2@3", respectively)
Numbers may be represented in many radices, but the radix specification itself is always expressed in base 10. The base for the exponent 
part is the same as the radix. So:  ==> 10,  ==> 1000 (=10 x 10 ), but  ==> 40 (=10 x 2 )2r1010 10e2 2 2r1010e2 2
Characters (Instances of class Character)