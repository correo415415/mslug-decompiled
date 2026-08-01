| ============================================================================
|  Metal Slug 1 - asm/entity_probe_transform_027c8c.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #9
|
|  Entity_ProbeTransformFreeCcr_027c8c  @ $027c8c  (68 bytes, 1 caller
|                                                   matcheado: JsrAbsThunk_067f72)
|
|  Variante hermana de T#7 (Entity_ProbeTransformFreeCcr @ $027cee):
|  misma estructura byte-a-byte excepto por el target del probe interno.
|  Aqui llama a $0273fc (Sub_000273FC) en lugar de $0277c4 (Sub_000277C4).
|  Es la evidencia dura de que el juego tiene un CLUSTER de wrappers
|  probar-transform parametrizados por el probe interno.
|
|  Layout identico a T#7:
|      +00..+05 : movea.l #$108080, a5
|      +06..+1D : 4 movs de save (transform -> scratch)
|      +1E..+21 : jsr $273fc(pc)     [distancia: $273fc - ($27cac+2) = -$8b0]
|      +22..+25 : bcs.w Entity_RestoreTransformSetC_027cd0
|      +26..+3D : 4 movs de restore (scratch -> transform)
|      +3E..+41 : andi.b #$ee, ccr   [C=0, X=0 -> exito]
|      +42..+43 : rts
|
|  --------------------------------------------------------------------------
|  ABSORCION DE FALSO POSITIVO Wave F (tercera del proyecto)
|  --------------------------------------------------------------------------
|  Los ultimos 6 bytes ($027cca..$027ccf = "andi.b #$ee,ccr ; rts")
|  estaban registrados como ClearXN_027cca. Evidencia forense:
|    - ClearXN_027cca: 0 callers externos desde codigo matcheado.
|    - $027c8c: 1 caller matcheado (JsrAbsThunk_067f72).
|  Es el mismo idioma probe/revert de T#7 con el probe cambiado.
|  Al absorberlo:
|    * -6 B en ccr_helpers.c
|    * +68 B en task cluster
|    * neto: +62 B, sin regresion.
|
|  Hallazgos forenses (asm a mano):
|    1. Copia byte-a-byte de T#7 con solo el jsr(pc) cambiado. Un
|       compilador C con macros habria emitido dos copias distintas
|       por reordering de peephole; aqui las dos secuencias son
|       identicas modulo el desplazamiento del probe.
|    2. Retorno por CCR con brazo hermano en $027cd0.
|    3. La misma cadena de 4 moves de save/restore aparece ahora
|       *tres veces* en la ROM (T#7, T#7-restore, T#9): el
|       codigo ha sido cortado y pegado, no generado por optimizador.
|  ============================================================================

        .text
        .globl  Entity_ProbeTransformFreeCcr_027c8c
        .type   Entity_ProbeTransformFreeCcr_027c8c, @function
        .section .text.Entity_ProbeTransformFreeCcr_027c8c, "ax", @progbits

Entity_ProbeTransformFreeCcr_027c8c:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80    a5 = &g_actor_ctx
        move.w  0x22(a6), -0x1148(a5)   | +06  3b 6e 00 22 ee b8    scratch.pos_x  = a6.pos_x
        move.w  0x24(a6), -0x1146(a5)   | +0c  3b 6e 00 24 ee ba    scratch.pos_y  = a6.pos_y
        move.b  0x26(a6), -0x1144(a5)   | +12  1b 6e 00 26 ee bc    scratch.byte26 = a6.byte26
        move.b  0x27(a6), -0x1143(a5)   | +18  1b 6e 00 27 ee bd    scratch.byte27 = a6.byte27
        jsr     .Lprobe(pc)             | +1e  4e ba f7 50          -> Sub_000273FC
        bcs.w   Entity_RestoreTransformSetC_027cd0    | +22  65 00 00 20
                                        |               si C=1 -> brazo hermano
        move.w  -0x1148(a5), 0x22(a6)   | +26  3d 6d ee b8 00 22    a6.pos_x  = scratch.pos_x
        move.w  -0x1146(a5), 0x24(a6)   | +2c  3d 6d ee ba 00 24    a6.pos_y  = scratch.pos_y
        move.b  -0x1144(a5), 0x26(a6)   | +32  1d 6d ee bc 00 26    a6.byte26 = scratch.byte26
        move.b  -0x1143(a5), 0x27(a6)   | +38  1d 6d ee bd 00 27    a6.byte27 = scratch.byte27
        andi.b  #0xee, ccr              | +3e  02 3c 00 ee          C=0, X=0 (exito)
        rts                             | +42  4e 75
        .size   Entity_ProbeTransformFreeCcr_027c8c, .-Entity_ProbeTransformFreeCcr_027c8c

        .equ    .Lprobe, Sub_000273FC
