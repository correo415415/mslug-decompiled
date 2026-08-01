| ============================================================================
|  Metal Slug 1 - asm/input_check_channel_05d032.s
|  ----------------------------------------------------------------------------
|  Wave U (InputMask event dispatchers v2) - backend #1 clonado
|
|  InputMask_CheckChannelAvail_05d032  @ $05d032  (32 bytes, 3 callers)
|
|  Clon byte-a-byte de InputMask_CheckChannelAvail_05cfa8 (backend del
|  cluster #1). Codigo IDENTICO en ambas direcciones - la copia literal
|  cortada/pegada por el programador humano es firma inequivoca de asm
|  a mano: GCC habria factorizado la funcion.
|
|  Comprueba bit 7 de $100000 (disponibilidad de canal global) y $44 de
|  la struct P1 ($1001c0). Si ambos permiten, sale con C=0. Si no, cae
|  a InputMask_ReadCtxSwitchPlayer_05d052 (brazo alternativo).
|
|  Absorbe ClearXN_05d04c (falso positivo Wave F, 0 callers externos).
|  ============================================================================

        .text
        .globl  InputMask_CheckChannelAvail_05d032
        .type   InputMask_CheckChannelAvail_05d032, @function
        .section .text.InputMask_CheckChannelAvail_05d032, "ax", @progbits

InputMask_CheckChannelAvail_05d032:
        btst.b  #7, 0x100000                | +00  08 39 00 07 00 10 00 00
        bne.w   InputMask_ReadCtxSwitchPlayer_05d052   | +08  66 00 00 16
        lea     0x1001c0.l, a4              | +0c  49 f9 00 10 01 c0
        tst.b   0x44(a4)                    | +12  4a 2c 00 44
        beq.w   InputMask_ReadCtxSwitchPlayer_05d052   | +16  67 00 00 08
        andi.b  #0xee, ccr                  | +1a  02 3c 00 ee
        rts                                 | +1e  4e 75
        .size   InputMask_CheckChannelAvail_05d032, .-InputMask_CheckChannelAvail_05d032
