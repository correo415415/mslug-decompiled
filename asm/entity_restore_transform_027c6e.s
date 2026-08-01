| ============================================================================
|  Metal Slug 1 - asm/entity_restore_transform_027c6e.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #14
|
|  Entity_RestoreTransformSetC_027c6e  @ $027c6e  (30 bytes, 0 callers
|                                                  directos matcheados;
|                                                  solo alcanzable como brazo
|                                                  bcs.w de T#13)
|
|  Brazo "colision" del par probe/revert cuya cabecera es
|  Entity_ProbeTransformFreeCcr_027c2a (T#13). Estructura byte-a-byte
|  identica a T#8, T#10 y T#12: 4 movs de restore + ori.b #$11,ccr + rts.
|
|  Absorbe la cola SetXN_027c86 (falso positivo Wave F, 0 callers).
|  Cierra el segundo doble par (T#11/T#12 + T#13/T#14), completando el
|  cluster de wrappers probe/revert de la region $027bc8..$027c8b (196 B
|  de codigo semantico previamente descrito por 4 helpers Wave F de 6 B).
|  ============================================================================

        .text
        .globl  Entity_RestoreTransformSetC_027c6e
        .type   Entity_RestoreTransformSetC_027c6e, @function
        .section .text.Entity_RestoreTransformSetC_027c6e, "ax", @progbits

Entity_RestoreTransformSetC_027c6e:
        move.w  -0x1148(a5), 0x22(a6)   | +00  3d 6d ee b8 00 22
        move.w  -0x1146(a5), 0x24(a6)   | +06  3d 6d ee ba 00 24
        move.b  -0x1144(a5), 0x26(a6)   | +0c  1d 6d ee bc 00 26
        move.b  -0x1143(a5), 0x27(a6)   | +12  1d 6d ee bd 00 27
        ori.b   #0x11, ccr              | +18  00 3c 00 11
        rts                             | +1c  4e 75
        .size   Entity_RestoreTransformSetC_027c6e, .-Entity_RestoreTransformSetC_027c6e
