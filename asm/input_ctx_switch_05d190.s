| ============================================================================
|  Metal Slug 1 - asm/input_ctx_switch_05d190.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers) - sub-cluster #5, backend CtxSwitch
|
|  InputMask_ReadCtxSwitchPlayer_05d190  @ $05d190  (48 bytes, 0 callers
|                                                    directos matcheados;
|                                                    solo alcanzable como
|                                                    brazo alternativo de
|                                                    InputMask_CheckChannelAvail_05d170)
|
|  TERCER CLON byte-a-byte de InputMask_ReadCtxSwitchPlayer_05cfc8 y
|  _05d052. Distancia del lea pc-rel a Sub_00005CC08:
|    - $05cfc8: 45 fa fc 3e  ($5cc08 - $5cfcc = -$3c4)
|    - $05d052: 45 fa fb b4  ($5cc08 - $5d056 = -$44e)
|    - $05d190: 45 fa fa 76  ($5cc08 - $5d194 = -$58c)
|  Solo cambia el desplazamiento, el resto (cmpi.b + lea + movea.l + jsr)
|  es identico. Fall-through a InputMask_TestChannelNibbleXor_05d1c0.
|  ============================================================================

        .text
        .globl  InputMask_ReadCtxSwitchPlayer_05d190
        .type   InputMask_ReadCtxSwitchPlayer_05d190, @function
        .section .text.InputMask_ReadCtxSwitchPlayer_05d190, "ax", @progbits

InputMask_ReadCtxSwitchPlayer_05d190:
        lea     Sub_00005CC08(pc), a2       | +00  45 fa fa 76
        cmpi.b  #1, 0x6d(a6)                | +04  0c 2e 00 01 00 6d
        bne.w   .Lcheck_p2                  | +0a  66 00 00 0c
        lea     0x100300.l, a1              | +0e  43 f9 00 10 03 00
        movea.l 0x72(a1), a2                | +14  24 69 00 72
.Lcheck_p2:
        cmpi.b  #2, 0x6d(a6)                | +18  0c 2e 00 02 00 6d
        bne.w   .Lcall_probe                | +1e  66 00 00 0c
        lea     0x1003a0.l, a1              | +22  43 f9 00 10 03 a0
        movea.l 0x72(a1), a2                | +28  24 69 00 72
.Lcall_probe:
        jsr     Sub_00005D674(pc)           | +2c  4e ba 04 b6
                                            |      -> $5d674 (mismo probe
                                            |         real que en $5cff4 y $5d07e)
                                            |      fall-through a $05d1c0
        .size   InputMask_ReadCtxSwitchPlayer_05d190, .-InputMask_ReadCtxSwitchPlayer_05d190
