| ============================================================================
|  Metal Slug 1 - asm/input_test_bit_05d0e2.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-cluster #3, backend TestBit
|
|  InputMask_TestChannelBit_05d0e2  @ $05d0e2  (22 bytes, 4 callers de v3
|                                              via bra.w desde thunks del
|                                              sub-cluster #3)
|
|  Clon byte-a-byte de InputMask_TestChannelBit_05cff8 (cluster #1). Test
|  "algun bit activo" con bne.w. Absorbe ClearXN_05d0ec + SetXN_05d0f2
|  (falsos positivos Wave F, 0 callers externos).
|  ============================================================================

        .text
        .globl  InputMask_TestChannelBit_05d0e2
        .type   InputMask_TestChannelBit_05d0e2, @function
        .section .text.InputMask_TestChannelBit_05d0e2, "ax", @progbits

InputMask_TestChannelBit_05d0e2:
        move.b  (a2, d0.w), d0              | +00  10 32 00 00
        and.b   d1, d0                      | +04  c0 01
        bne.w   .Lset_c                     | +06  66 00 00 08
        andi.b  #0xee, ccr                  | +0a  02 3c 00 ee   C=0 (bit inactivo)
        rts                                 | +0e  4e 75
.Lset_c:
        ori.b   #0x11, ccr                  | +10  00 3c 00 11   C=1 (bit activo)
        rts                                 | +14  4e 75
        .size   InputMask_TestChannelBit_05d0e2, .-InputMask_TestChannelBit_05d0e2
