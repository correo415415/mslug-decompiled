| ============================================================================
|  Metal Slug 1 - asm/entity_probe_transform_027d50.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #15
|
|  Entity_ProbeTransformFreeCcr_027d50  @ $027d50  (68 bytes, 0 callers
|                                                   matcheados directos)
|
|  Quinto y ultimo wrapper probe/revert del cluster $027Cxx..$027Dxx.
|  Estructura byte-a-byte identica a T#7, T#9, T#11 y T#13 con el probe
|  interno parametrizado a Sub_00002773C (= $2773c). Es el quinto probe
|  DISTINTO del cluster (T#7 usaba $277c4, T#9 usaba $273fc, T#11+T#13
|  compartian $27444, T#15 usa $2773c).
|
|  Absorbe la cola ClearXN_027d8e (falso positivo Wave F, 0 callers).
|  ============================================================================

        .text
        .globl  Entity_ProbeTransformFreeCcr_027d50
        .type   Entity_ProbeTransformFreeCcr_027d50, @function
        .section .text.Entity_ProbeTransformFreeCcr_027d50, "ax", @progbits

Entity_ProbeTransformFreeCcr_027d50:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80
        move.w  0x22(a6), -0x1148(a5)   | +06  3b 6e 00 22 ee b8
        move.w  0x24(a6), -0x1146(a5)   | +0c  3b 6e 00 24 ee ba
        move.b  0x26(a6), -0x1144(a5)   | +12  1b 6e 00 26 ee bc
        move.b  0x27(a6), -0x1143(a5)   | +18  1b 6e 00 27 ee bd
        jsr     .Lprobe(pc)             | +1e  4e ba f9 cc    -> Sub_00002773C
        bcs.w   Entity_RestoreTransformSetC_027d94    | +22  65 00 00 20
        move.w  -0x1148(a5), 0x22(a6)   | +26  3d 6d ee b8 00 22
        move.w  -0x1146(a5), 0x24(a6)   | +2c  3d 6d ee ba 00 24
        move.b  -0x1144(a5), 0x26(a6)   | +32  1d 6d ee bc 00 26
        move.b  -0x1143(a5), 0x27(a6)   | +38  1d 6d ee bd 00 27
        andi.b  #0xee, ccr              | +3e  02 3c 00 ee
        rts                             | +42  4e 75
        .size   Entity_ProbeTransformFreeCcr_027d50, .-Entity_ProbeTransformFreeCcr_027d50

        .equ    .Lprobe, Sub_00002773C
