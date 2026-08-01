| ============================================================================
|  Metal Slug 1 - asm/entity_probe_transform_027cee.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #7
|
|  Entity_ProbeTransformFreeCcr  @ $027cee  (68 bytes, 7 callers)
|
|  "Probe" no destructivo del transform de la entidad a6 contra la
|  rutina interna $0277c4 (heuristicamente TryMoveAndCheckCollision).
|  Retorna en el CCR:
|      C=0 -> el destino esta libre, el transform ha sido REVERTIDO al
|             estado original (el caller puede reintentar con otras
|             coordenadas o aplicar el movimiento en otra fase).
|      C=1 -> el probe devolvio "colision"; el brazo hermano en
|             $027d32 realiza la misma revision del transform y sale
|             con C=1 forzado por ori.b #$11,ccr.
|
|  Flujo:
|      1. a5 = &g_actor_ctx (=$108080), MISMO base pointer que en
|         T#5 (ActorCtxWrapper_02783a). Este cluster de wrappers ("familia
|         actor_ctx" alrededor de $108080) sugiere que $108080-$1148..-$1143
|         (= $106F38..$106F3D, 6 bytes) es el "scratch transform" del
|         engine, usado como buffer temporal para probes reversibles.
|      2. Guardar en scratch los 4 campos de transform de a6:
|             pos_x  ($22, word)  -> -$1148(a5)
|             pos_y  ($24, word)  -> -$1146(a5)
|             byte26 ($26, byte)  -> -$1144(a5)
|             byte27 ($27, byte)  -> -$1143(a5)
|      3. jsr $0277c4(pc)  -> Sub_0002_77C4  (probe/collision).
|         Distancia: $277c4 - ($27d0c+2) = -$54a. PC-rel corto (4B).
|      4. bcs.w Entity_RestoreTransformSetC_027d32
|         (si probe devolvio C=1, saltar al brazo hermano).
|      5. Restaurar los 4 campos desde scratch: revierte el intento.
|      6. andi.b #$ee, ccr  -> mascara %11101110: borra C y X.
|         Los otros bits del CCR (Z, N, V, ...) sobreviven; el caller
|         solo mira C.
|      7. rts.
|
|  Entrada (registros absolutos, convencion no-C):
|      a6 : entidad cuyo transform se prueba
|      (el probe $0277c4 lee sus propios parametros, presumiblemente de
|       otros campos de a6 o del propio scratch)
|
|  Salida por CCR:
|      C=0 : destino libre, transform inalterado.
|      C=1 : destino ocupado (via brazo hermano $027d32).
|
|  --------------------------------------------------------------------------
|  ABSORCION DE FALSO POSITIVO Wave F
|  --------------------------------------------------------------------------
|  Los ultimos 6 bytes de esta funcion ($027d2c..$027d31 = "andi.b
|  #$ee,ccr ; rts") estaban previamente registrados por la Wave F como
|  la funcion independiente ClearXN_027d2c en src/ccr_helpers.c.
|
|  Evidencia forense recogida en scan_unmatched_callees:
|    - ClearXN_027d2c: 0 callers externos desde codigo matcheado.
|    - $027cee: 7 callers (JsrAbsThunk_*).
|  Es decir, esos 6 bytes forman el epilogo compartido de la funcion
|  semantica $027cee, no una utilidad reusable. Es la MISMA firma de
|  epilogo compartido que ya se corrigio en:
|    - Sprite_InvokeBlit8Params  (S#1) absorbio JsrAbsThunk_050248.
|    - Task_AllocFromFreeList    (T#4) usa JsrAbsThunk_0004fe (fall-through
|                                real, no absorbido: el thunk tiene otros
|                                callers propios).
|    - Entity_ProbeTransformFreeCcr (T#7, ESTA) absorbe ClearXN_027d2c.
|
|  Al absorberlo:
|    * -6 B en ccr_helpers.c (ClearXN_027d2c eliminado)
|    * +68 B en task cluster (esta funcion)
|    * neto: +62 B, sin regresion (los 6 B siguen presentes en la ROM
|      resultante, solo cambia el simbolo que los aporta).
|
|  Hallazgos forenses (asm a mano):
|    1. movea.l #$108080, a5 con constante empotrada (mismo idioma que T#5).
|    2. Ocho instrucciones simetricas de 6 B cada una (movegh word/byte
|       entre $22..$27(a6) y desplazamientos -$1148..-$1143(a5)), con
|       displacement de 16 bits sobre a5. GCC habria elegido un movem
|       si el layout fuera compatible.
|    3. Retorno por CCR con dos brazos hermanos (ClearXN vs SetXN) - el
|       propio compilador jamas emite este patron.
|    4. bcs.w a un target FUERA del rango de la funcion que continua el
|       flujo con codigo estructuralmente identico (mismo scratch, mismo
|       orden de fields) - fall-through logico entre dos funciones.
|  ============================================================================

        .text
        .globl  Entity_ProbeTransformFreeCcr
        .type   Entity_ProbeTransformFreeCcr, @function
        .section .text.Entity_ProbeTransformFreeCcr, "ax", @progbits

Entity_ProbeTransformFreeCcr:
        movea.l #0x108080, a5           | +00  2a 7c 00 10 80 80    a5 = &g_actor_ctx
        move.w  0x22(a6), -0x1148(a5)   | +06  3b 6e 00 22 ee b8    scratch.pos_x  = a6.pos_x
        move.w  0x24(a6), -0x1146(a5)   | +0c  3b 6e 00 24 ee ba    scratch.pos_y  = a6.pos_y
        move.b  0x26(a6), -0x1144(a5)   | +12  1b 6e 00 26 ee bc    scratch.byte26 = a6.byte26
        move.b  0x27(a6), -0x1143(a5)   | +18  1b 6e 00 27 ee bd    scratch.byte27 = a6.byte27
        jsr     .Lprobe(pc)             | +1e  4e ba fa b6          -> Sub_0002_77C4
        bcs.w   Entity_RestoreTransformSetC_027d32   | +22  65 00 00 20
                                        |               si C=1 -> brazo hermano
        move.w  -0x1148(a5), 0x22(a6)   | +26  3d 6d ee b8 00 22    a6.pos_x  = scratch.pos_x
        move.w  -0x1146(a5), 0x24(a6)   | +2c  3d 6d ee ba 00 24    a6.pos_y  = scratch.pos_y
        move.b  -0x1144(a5), 0x26(a6)   | +32  1d 6d ee bc 00 26    a6.byte26 = scratch.byte26
        move.b  -0x1143(a5), 0x27(a6)   | +38  1d 6d ee bd 00 27    a6.byte27 = scratch.byte27
        andi.b  #0xee, ccr              | +3e  02 3c 00 ee          C=0, X=0 (exito)
        rts                             | +42  4e 75
        .size   Entity_ProbeTransformFreeCcr, .-Entity_ProbeTransformFreeCcr

| Aliases externos (resueltos via --defsym en tools/symbols.py):
|   .Lprobe                                = Sub_000277C4
|   Entity_RestoreTransformSetC_027d32     = 0x00027D32 (brazo hermano)
        .equ    .Lprobe, Sub_000277C4
