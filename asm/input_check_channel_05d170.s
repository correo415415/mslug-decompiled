| ============================================================================
|  Metal Slug 1 - asm/input_check_channel_05d170.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-cluster #4, backend CheckChannel
|
|  InputMask_CheckChannelAvail_05d170  @ $05d170  (32 bytes, 4 callers de v3
|                                                  desde thunks del bloque #4b)
|
|  TERCER CLON byte-a-byte de InputMask_CheckChannelAvail_05cfa8 y
|  InputMask_CheckChannelAvail_05d032. Tres copias literales identicas
|  del mismo idioma "test bit 7 de $100000 + tst.b $44 de $1001c0". El
|  programador humano cortando y pegando la misma rutina para dos
|  clusters distintos - firma inequivoca de asm a mano.
|
|  Absorbe ClearXN_05d18a (falso positivo Wave F, 0 callers externos).
|  Si el canal esta ocupado, salta a InputMask_ReadCtxSwitchPlayer_05d190.
|  ============================================================================

        .text
        .globl  InputMask_CheckChannelAvail_05d170
        .type   InputMask_CheckChannelAvail_05d170, @function
        .section .text.InputMask_CheckChannelAvail_05d170, "ax", @progbits

InputMask_CheckChannelAvail_05d170:
        btst.b  #7, 0x100000                | +00  08 39 00 07 00 10 00 00
        bne.w   InputMask_ReadCtxSwitchPlayer_05d190   | +08  66 00 00 16
        lea     0x1001c0.l, a4              | +0c  49 f9 00 10 01 c0
        tst.b   0x44(a4)                    | +12  4a 2c 00 44
        beq.w   InputMask_ReadCtxSwitchPlayer_05d190   | +16  67 00 00 08
        andi.b  #0xee, ccr                  | +1a  02 3c 00 ee
        rts                                 | +1e  4e 75
        .size   InputMask_CheckChannelAvail_05d170, .-InputMask_CheckChannelAvail_05d170
