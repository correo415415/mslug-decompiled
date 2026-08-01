| ============================================================================
|  Metal Slug 1 - asm/entity_probe_transform_027bc8.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #11
|
|  Entity_ProbeTransformFreeCcr_027bc8  @ $027bc8  (68 bytes, 0 callers
|                                                   matcheados: alcanzada
|                                                   por indireccion registrada)
|
|  Tercer wrapper probe/revert del cluster $027Cxx. Estructura byte-a-byte
|  identica a T#7 y T#9 con el probe interno parametrizado a Sub_000273FC
|  (= $027444, mismo probe que compartira con T#13).
|
|  Absorbe la cola ClearXN_027c06 (falso positivo Wave F, 0 callers).
|  ============================================================================

        .text
        .globl  Entity_ProbeTransformFreeCcr_027bc8
        .type   Entity_ProbeTransformFreeCcr_027bc8, @function
        .section .text.Entity_ProbeTransformFreeCcr_027bc8, "ax", @progbits

Entity_ProbeTransformFreeCcr_027bc8:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80
        move.w  0x22(a6), -0x1148(a5)   | +06  3b 6e 00 22 ee b8
        move.w  0x24(a6), -0x1146(a5)   | +0c  3b 6e 00 24 ee ba
        move.b  0x26(a6), -0x1144(a5)   | +12  1b 6e 00 26 ee bc
        move.b  0x27(a6), -0x1143(a5)   | +18  1b 6e 00 27 ee bd
        jsr     .Lprobe(pc)             | +1e  4e ba f8 5c    -> Sub_000027444
        bcs.w   Entity_RestoreTransformSetC_027c0c    | +22  65 00 00 20
        move.w  -0x1148(a5), 0x22(a6)   | +26  3d 6d ee b8 00 22
        move.w  -0x1146(a5), 0x24(a6)   | +2c  3d 6d ee ba 00 24
        move.b  -0x1144(a5), 0x26(a6)   | +32  1d 6d ee bc 00 26
        move.b  -0x1143(a5), 0x27(a6)   | +38  1d 6d ee bd 00 27
        andi.b  #0xee, ccr              | +3e  02 3c 00 ee
        rts                             | +42  4e 75
        .size   Entity_ProbeTransformFreeCcr_027bc8, .-Entity_ProbeTransformFreeCcr_027bc8

        .equ    .Lprobe, Sub_000027444
