| ============================================================================
|  Metal Slug 1 - asm/entity_restore_transform_027d32.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #8
|
|  Entity_RestoreTransformSetC_027d32  @ $027d32  (30 bytes, 0 callers directos
|                                                  matcheados; solo alcanzable
|                                                  como brazo bcs.w de T#7)
|
|  Brazo "colision detectada" del par probe/revert cuya cabecera es
|  Entity_ProbeTransformFreeCcr @ $027cee (Wave T#7). Restaura los 4
|  campos del transform de la entidad a6 (pos_x, pos_y, byte26, byte27)
|  desde el scratch en -$1148..-$1143(a5) y retorna con C=1 y X=1
|  forzados por ori.b #$11, ccr.
|
|  Se alcanza EXCLUSIVAMENTE via el `bcs.w Entity_RestoreTransformSetC_027d32`
|  interno de T#7 ($27D10). La ausencia de callers externos, unida al
|  hecho de que su epilogo tenia una entrada Wave F "SetXN_027d4a", es
|  la firma del mismo idioma de "dos brazos hermanos" que ya se
|  absorbio en T#7 (ClearXN_027d2c).
|
|  Flujo:
|      1. Restaurar pos_x, pos_y, byte26, byte27 desde scratch en a5.
|         Cuatro moves de 6 B cada uno con offsets identicos al brazo
|         "restore" del cuerpo principal de T#7 (misma secuencia byte
|         a byte $3d6deeb80022 / $3d6deeba0024 / $1d6deebc0026 /
|         $1d6deebd0027).
|      2. ori.b #$11, ccr  -> mascara %00010001: fuerza C=1 y X=1,
|         deja los demas bits intactos. Simetria perfecta con T#7,
|         que usa andi.b #$ee (=complemento negado de $11) para forzar
|         C=0 y X=0.
|      3. rts.
|
|  Entrada: a5 = &g_actor_ctx ($108080), a6 = entidad activa (invariante
|           heredado del caller T#7).
|
|  Salida por CCR:
|      C=1, X=1 : "movimiento rechazado, transform inalterado".
|      Los demas bits del CCR sobreviven del contexto del probe fallido.
|
|  --------------------------------------------------------------------------
|  ABSORCION DE FALSO POSITIVO Wave F
|  --------------------------------------------------------------------------
|  Los ultimos 6 bytes de esta funcion ($027d4a..$027d4f = "ori.b
|  #$11, ccr ; rts") estaban previamente registrados por la Wave F como
|  la funcion independiente SetXN_027d4a en src/ccr_helpers.c.
|
|  Evidencia forense (verificada con capstone sobre todo el REGISTRY):
|    - SetXN_027d4a: 0 callers externos desde codigo matcheado.
|    - $027d32:      0 callers directos matcheados (solo alcanzable via
|                    el bcs.w interno de T#7, que ya lo referencia como
|                    Entity_RestoreTransformSetC_027d32 en symbols.py).
|  Es el segundo par probe/revert absorbido en la Wave T. Anticipa
|  que la zona $027c06..$027dac contiene al menos 4 pares mas de
|  probe/revert hermanos con la misma estructura ClearXN/SetXN de 6 B
|  como colas compartidas (candidatos: $027c06/$027c24, $027c68/$027c86,
|  $027cca/$027ce8, $027d8e/$027dac, todos ellos ya registrados en
|  ccr_helpers.c como helpers independientes de 6 B).
|
|  Al absorberlo:
|    * -6 B en ccr_helpers.c (SetXN_027d4a eliminado)
|    * +30 B en task cluster (esta funcion)
|    * neto: +24 B, sin regresion.
|
|  Hallazgos forenses (asm a mano):
|    1. Continuacion literal del brazo "restore" de T#7: los 4 moves
|       de $027d32..$027d49 son byte-a-byte identicos a los de
|       $027d14..$027d2b (brazo exito de T#7). El "duplicado" es de
|       hecho un fall-through logico partido en dos entradas por el
|       propio hand-writer.
|    2. Retorno por CCR con brazo hermano (mismo patron que S#2
|       Entity_HasLinkedSlots y que el par completo T#7/T#8).
|    3. ori.b #$11 y andi.b #$ee son mascaras complementarias: $11 |
|       $EE = $FF, $11 & $EE = 0. Los dos brazos "escriben la misma
|       decision" en el CCR usando primitivas de bit-set/bit-clear
|       simetricas.
|  ============================================================================

        .text
        .globl  Entity_RestoreTransformSetC_027d32
        .type   Entity_RestoreTransformSetC_027d32, @function
        .section .text.Entity_RestoreTransformSetC_027d32, "ax", @progbits

Entity_RestoreTransformSetC_027d32:
        move.w  -0x1148(a5), 0x22(a6)   | +00  3d 6d ee b8 00 22    a6.pos_x  = scratch.pos_x
        move.w  -0x1146(a5), 0x24(a6)   | +06  3d 6d ee ba 00 24    a6.pos_y  = scratch.pos_y
        move.b  -0x1144(a5), 0x26(a6)   | +0c  1d 6d ee bc 00 26    a6.byte26 = scratch.byte26
        move.b  -0x1143(a5), 0x27(a6)   | +12  1d 6d ee bd 00 27    a6.byte27 = scratch.byte27
        ori.b   #0x11, ccr              | +18  00 3c 00 11          C=1, X=1 (fallo)
        rts                             | +1c  4e 75
        .size   Entity_RestoreTransformSetC_027d32, .-Entity_RestoreTransformSetC_027d32
