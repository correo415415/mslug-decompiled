| ============================================================================
|  Metal Slug 1 - asm/entity_restore_transform_027d94.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #16
|
|  Entity_RestoreTransformSetC_027d94  @ $027d94  (30 bytes, 0 callers
|                                                  directos matcheados;
|                                                  solo alcanzable como brazo
|                                                  bcs.w de T#15)
|
|  Brazo "colision" del par probe/revert cuya cabecera es
|  Entity_ProbeTransformFreeCcr_027d50 (T#15). Estructura byte-a-byte
|  identica a T#8, T#10, T#12 y T#14: 4 movs de restore + ori.b #$11,ccr
|  + rts.
|
|  Absorbe la cola SetXN_027dac (falso positivo Wave F, 0 callers).
|  Cierra el CLUSTER COMPLETO de wrappers probe/revert en
|  $027bc8..$027db1: 5 pares completos, 10 funciones, 480 B de codigo
|  semantico que previamente estaban descritos por 10 helpers Wave F
|  de 6 B.
|  ============================================================================

        .text
        .globl  Entity_RestoreTransformSetC_027d94
        .type   Entity_RestoreTransformSetC_027d94, @function
        .section .text.Entity_RestoreTransformSetC_027d94, "ax", @progbits

Entity_RestoreTransformSetC_027d94:
        move.w  -0x1148(a5), 0x22(a6)   | +00  3d 6d ee b8 00 22
        move.w  -0x1146(a5), 0x24(a6)   | +06  3d 6d ee ba 00 24
        move.b  -0x1144(a5), 0x26(a6)   | +0c  1d 6d ee bc 00 26
        move.b  -0x1143(a5), 0x27(a6)   | +12  1d 6d ee bd 00 27
        ori.b   #0x11, ccr              | +18  00 3c 00 11
        rts                             | +1c  4e 75
        .size   Entity_RestoreTransformSetC_027d94, .-Entity_RestoreTransformSetC_027d94
