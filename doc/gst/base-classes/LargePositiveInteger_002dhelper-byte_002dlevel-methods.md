[]{#LargePositiveInteger_002dhelper-byte_002dlevel-methods}

::: header
Next: [LargePositiveInteger-numeric
testing](LargePositiveInteger_002dnumeric-testing.html#LargePositiveInteger_002dnumeric-testing){accesskey="n"
rel="next"}, Previous:
[LargePositiveInteger-converting](LargePositiveInteger_002dconverting.html#LargePositiveInteger_002dconverting){accesskey="p"
rel="prev"}, Up:
[LargePositiveInteger](LargePositiveInteger.html#LargePositiveInteger){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LargePositiveInteger_003a-helper-byte_002dlevel-methods}

#### 1.100.3 LargePositiveInteger: helper byte-level methods {#largepositiveinteger-helper-byte-level-methods .subsection}

[]{#index-bytes_003afrom_003acompare_003a}

**bytes: byteArray1 from: j compare: byteArray2**

Private - Answer the sign of byteArray2 - byteArray1; the j-th byte of
byteArray1 is compared with the first of byteArray2, the j+1-th with the
second, and so on.

[]{#index-bytes_003afrom_003asubtract_003a}

**bytes: byteArray1 from: j subtract: byteArray2**

Private - Sutract the bytes in byteArray2 from those in byteArray1

[]{#index-bytes_003amultiply_003a}

**bytes: bytes multiply: anInteger**

Private - Multiply the bytes in bytes by anInteger, which must be \<
255. Put the result back in bytes.

[]{#index-bytesLeftShift_003a}

**bytesLeftShift: aByteArray**

Private - Left shift by 1 place the bytes in aByteArray

[]{#index-bytesLeftShift_003abig_003a}

**bytesLeftShift: aByteArray big: totalShift**

Private - Left shift the bytes in aByteArray by totalShift places

[]{#index-bytesLeftShift_003an_003a}

**bytesLeftShift: aByteArray n: shift**

Private - Left shift by shift places the bytes in aByteArray (shift \<=
7)

[]{#index-bytesRightShift_003abig_003a}

**bytesRightShift: aByteArray big: totalShift**

Private - Right shift the bytes in aByteArray by totalShift places

[]{#index-bytesRightShift_003an_003a}

**bytesRightShift: bytes n: aNumber**

Private - Right shift the bytes in 'bytes' by 'aNumber' places (shift
\<= 7)

[]{#index-bytesTrailingZeros_003a}

**bytesTrailingZeros: bytes**

Private - Answer the number of trailing zero bits in the receiver

[]{#index-primDivide_003a}

**primDivide: rhs**

Private - Implements Knuth's divide and correct algorithm from
'Seminumerical Algorithms' 3rd Edition, section 4.3.1 (which is
basically an enhanced version of the divide 'algorithm' for two-digit
divisors which is taught in primary school!!!)
