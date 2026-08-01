| ============================================================================
|  Metal Slug 1 - asm/input_ctx_switch_05d052.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers v2) - backend #2 clonado
|
|  InputMask_ReadCtxSwitchPlayer_05d052  @ $05d052  (48 bytes, 0 callers
|                                                    directos matcheados;
|                                                    solo alcanzable como brazo
|                                                    alternativo del backend #1
|                                                    del sub-cluster v2)
|
|  Clon byte-a-byte de InputMask_ReadCtxSwitchPlayer_05cfc8 (backend #2
|  del cluster #1). La unica diferencia entre ambas copias es la
|  distancia del lea.l pc-rel al mismo simbolo Sub_00005CC08:
|    - En $05cfc8: 45 fa fc 3e  ($5cc08 - $5cfcc = -$3c4)
|    - En $05d052: 45 fa fb b4  ($5cc08 - $5d056 = -$44e)
|  El desplazamiento cambia porque el pc origen cambia, pero el resto
|  del codigo (cmpi.b + lea + movea.l + jsr) es identico.
|
|  Elige el buffer de input segun $6d(a6) (1=P1 -> $100300, 2=P2 ->
|  $1003a0), carga a2 y llama al probe real Sub_00005D674. Cae por
|  fall-through a InputMask_TestChannelAllBits_05d082.
|  ============================================================================

        .text
        .globl  InputMask_ReadCtxSwitchPlayer_05d052
        .type   InputMask_ReadCtxSwitchPlayer_05d052, @function
        .section .text.InputMask_ReadCtxSwitchPlayer_05d052, "ax", @progbits

InputMask_ReadCtxSwitchPlayer_05d052:
        lea     Sub_00005CC08(pc), a2       | +00  45 fa fb b4   a2 = &$5cc08
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
        jsr     Sub_00005D674(pc)           | +2c  4e ba 05 f4
                                            |      -> $5d674 (probe real,
                                            |         mismo probe que $5cff4
                                            |         del cluster #1)
                                            |      fall-through a $05d082
        .size   InputMask_ReadCtxSwitchPlayer_05d052, .-InputMask_ReadCtxSwitchPlayer_05d052
