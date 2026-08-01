| ============================================================================
|  Metal Slug 1 - asm/entity_probe_transform_027c2a.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #13
|
|  Entity_ProbeTransformFreeCcr_027c2a  @ $027c2a  (68 bytes, 0 callers
|                                                   matcheados)
|
|  Cuarto wrapper probe/revert del cluster $027Cxx. Comparte probe interno
|  con T#11: ambos llaman a Sub_000027444. Estructura byte-a-byte identica
|  a T#7, T#9 y T#11.
|
|  Absorbe la cola ClearXN_027c68 (falso positivo Wave F, 0 callers).
|
|  Hallazgo forense adicional: T#11 y T#13 son wrappers CONSECUTIVOS que
|  llaman al MISMO probe. Un compilador con inlining habria colapsado los
|  duplicados o los habria emitido con nombres uniformes; aqui la copia
|  literal cortada/pegada es firma inequivoca de asm a mano.
|  ============================================================================

        .text
        .globl  Entity_ProbeTransformFreeCcr_027c2a
        .type   Entity_ProbeTransformFreeCcr_027c2a, @function
        .section .text.Entity_ProbeTransformFreeCcr_027c2a, "ax", @progbits

Entity_ProbeTransformFreeCcr_027c2a:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80
        move.w  0x22(a6), -0x1148(a5)   | +06  3b 6e 00 22 ee b8
        move.w  0x24(a6), -0x1146(a5)   | +0c  3b 6e 00 24 ee ba
        move.b  0x26(a6), -0x1144(a5)   | +12  1b 6e 00 26 ee bc
        move.b  0x27(a6), -0x1143(a5)   | +18  1b 6e 00 27 ee bd
        jsr     .Lprobe(pc)             | +1e  4e ba f7 fa    -> Sub_000027444
        bcs.w   Entity_RestoreTransformSetC_027c6e    | +22  65 00 00 20
        move.w  -0x1148(a5), 0x22(a6)   | +26  3d 6d ee b8 00 22
        move.w  -0x1146(a5), 0x24(a6)   | +2c  3d 6d ee ba 00 24
        move.b  -0x1144(a5), 0x26(a6)   | +32  1d 6d ee bc 00 26
        move.b  -0x1143(a5), 0x27(a6)   | +38  1d 6d ee bd 00 27
        andi.b  #0xee, ccr              | +3e  02 3c 00 ee
        rts                             | +42  4e 75
        .size   Entity_ProbeTransformFreeCcr_027c2a, .-Entity_ProbeTransformFreeCcr_027c2a

        .equ    .Lprobe, Sub_000027444
