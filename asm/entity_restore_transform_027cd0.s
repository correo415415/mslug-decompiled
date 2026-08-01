| ============================================================================
|  Metal Slug 1 - asm/entity_restore_transform_027cd0.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #10
|
|  Entity_RestoreTransformSetC_027cd0  @ $027cd0  (30 bytes, 0 callers directos
|                                                  matcheados; solo alcanzable
|                                                  como brazo bcs.w de T#9)
|
|  Brazo "colision detectada" del par probe/revert cuya cabecera es
|  Entity_ProbeTransformFreeCcr_027c8c @ $027c8c (Wave T#9). Estructura
|  byte-a-byte identica a T#8 (Entity_RestoreTransformSetC_027d32):
|  4 movs de restore desde scratch en a5, seguidos de ori.b #$11, ccr
|  y rts.
|
|  Es el segundo par completo probe/revert absorbido en la Wave T
|  (T#9/T#10, precedido por T#7/T#8).
|
|  --------------------------------------------------------------------------
|  ABSORCION DE FALSO POSITIVO Wave F (cuarta del proyecto)
|  --------------------------------------------------------------------------
|  Los ultimos 6 bytes ($027ce8..$027ced = "ori.b #$11,ccr ; rts")
|  estaban registrados como SetXN_027ce8. Evidencia forense:
|    - SetXN_027ce8: 0 callers externos desde codigo matcheado.
|    - $027cd0:      0 callers directos matcheados (solo via bcs.w
|                    interno de T#9).
|  Al absorberlo:
|    * -6 B en ccr_helpers.c
|    * +30 B en task cluster
|    * neto: +24 B, sin regresion.
|
|  Hallazgos forenses (asm a mano):
|    1. Copia byte-a-byte de T#8 (Entity_RestoreTransformSetC_027d32):
|       misma secuencia de 4 movs + ori.b #$11 + rts. Un compilador
|       jamas emitiria dos copias literales; el codigo esta claramente
|       cortado y pegado en la ROM.
|    2. Retorno por CCR (C=1, X=1) igual que T#8.
|  ============================================================================

        .text
        .globl  Entity_RestoreTransformSetC_027cd0
        .type   Entity_RestoreTransformSetC_027cd0, @function
        .section .text.Entity_RestoreTransformSetC_027cd0, "ax", @progbits

Entity_RestoreTransformSetC_027cd0:
        move.w  -0x1148(a5), 0x22(a6)   | +00  3d 6d ee b8 00 22    a6.pos_x  = scratch.pos_x
        move.w  -0x1146(a5), 0x24(a6)   | +06  3d 6d ee ba 00 24    a6.pos_y  = scratch.pos_y
        move.b  -0x1144(a5), 0x26(a6)   | +0c  1d 6d ee bc 00 26    a6.byte26 = scratch.byte26
        move.b  -0x1143(a5), 0x27(a6)   | +12  1d 6d ee bd 00 27    a6.byte27 = scratch.byte27
        ori.b   #0x11, ccr              | +18  00 3c 00 11          C=1, X=1 (fallo)
        rts                             | +1c  4e 75
        .size   Entity_RestoreTransformSetC_027cd0, .-Entity_RestoreTransformSetC_027cd0
