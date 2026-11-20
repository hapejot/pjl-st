[]{#Integer_002dbit-operators}

::: header
Next:
[Integer-converting](Integer_002dconverting.html#Integer_002dconverting){accesskey="n"
rel="next"}, Previous:
[Integer-basic](Integer_002dbasic.html#Integer_002dbasic){accesskey="p"
rel="prev"}, Up: [Integer](Integer.html#Integer){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Integer_003a-bit-operators}

#### 1.92.4 Integer: bit operators {#integer-bit-operators .subsection}

[]{#index-allMask_003a}

**allMask: anInteger**

True if all 1 bits in anInteger are 1 in the receiver

[]{#index-anyMask_003a}

**anyMask: anInteger**

True if any 1 bits in anInteger are 1 in the receiver

[]{#index-bitAt_003a}

**bitAt: index**

Answer the index-th bit of the receiver (the LSB has an index of 1)

[]{#index-bitAt_003aput_003a}

**bitAt: index put: value**

Answer an integer which is identical to the receiver, possibly with the
exception of the index-th bit of the receiver (the LSB having an index
of 1), which assumes a value equal to the low-order bit of the second
parameter.

[]{#index-bitClear_003a}

**bitClear: aMask**

Answer an Integer equal to the receiver, except that all the bits that
are set in aMask are cleared.

[]{#index-bitInvert}

**bitInvert**

Return the 1's complement of the bits of the receiver

[]{#index-clearBit_003a}

**clearBit: index**

Clear the index-th bit of the receiver and answer a new Integer

[]{#index-digitAt_003a}

**digitAt: index**

Answer the index-th base-256 digit of the receiver (byte), expressed in
two's complement

[]{#index-highBit}

**highBit**

Return the index of the highest order 1 bit of the receiver.

[]{#index-isBitSet_003a}

**isBitSet: index**

Answer whether the index-th bit of the receiver is set

[]{#index-lowBit}

**lowBit**

Return the index of the lowest order 1 bit of the receiver.

[]{#index-noMask_003a}

**noMask: anInteger**

Answer true if no 1 bits in anInteger are 1 in the receiver.

[]{#index-setBit_003a}

**setBit: index**

Set the index-th bit of the receiver and answer a new Integer

------------------------------------------------------------------------

::: header
Next:
[Integer-converting](Integer_002dconverting.html#Integer_002dconverting){accesskey="n"
rel="next"}, Previous:
[Integer-basic](Integer_002dbasic.html#Integer_002dbasic){accesskey="p"
rel="prev"}, Up: [Integer](Integer.html#Integer){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
