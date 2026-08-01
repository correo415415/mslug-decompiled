| ============================================================================
|  Metal Slug 1 - asm/geom_proj_clamp_0436de.s
|  ----------------------------------------------------------------------------
|  Wave FF batch 2 - #1
|
|  Geom_Proj_Clamp_0436DE  @ $0436DE  (252 bytes)
|
|  Helper geometrico de proyeccion 2D con clamp. Muy llamado (13 callers
|  desde $08Cxxx y $096xxx) por lo que probablemente forma parte del
|  pipeline de sprites del jugador o de proyectiles con parallax.
|
|  Estructura interna: dos "sub-funciones" ligadas por bsr.w a una
|  subrutina de reduccion aritmetica interna ($04374C). Semanticamente:
|
|      1. Gate de entrada ($4A39 $108179): si el flag $108179 esta armado
|         o d0/d1 fuera de rango, hace clr.l d0 + clr.l d1 y salta a la
|         cola comun ($437DA).
|      2. Preprocesado de operandos: sustrae $200 de d1 y d3 (los negativiza),
|         y garantiza d0<=d2 y d1<=d3 por exchange (exg   ).
|      3. Primera reduccion (bsr $4374C con d4/d5/d6 = camera/const/const):
|         calcula (d0+d2)/2 - d4 con delta de $10000 y clamp a d5..d6.
|      4. Segunda reduccion (bsr $4374C con d4 leido de $108166-$10817A
|         y d5=0, d6=$D0).
|      5. Cola ($437DA): compara con $106f5e y sale (fuera del ambito
|         registrado como funcion propia).
|
|  Firma C conceptual:
|
|      /* Proyecta un rect (d0..d3) sobre el espacio de pantalla usando
|       * la camara actual (globals $108164/$108166 con offset $10817A),
|       * aplica clamp a las dimensiones ([$1E..$117] y [0..$D0]) y
|       * devuelve el rect resultante en d0/d1. La sub-rutina interna
|       * $04374C calcula la coordenada intermedia con clamp asimetrico
|       * en [-$80000, +$80000] antes de dividir por 16 (asr.l #$4). */
|      long Geom_Proj_Clamp(long x0, long y0, long x1, long y1);
|
|  Layout observado:
|      $0436DE  entry     gate + xchg + sub-fase 1
|      $04371E  fase 1    bsr $4374C con params camara-X
|      $04372C  fase 2    bsr $4374C con params camara-Y
|      $04374C  helper    aritmetica clamp con asr.l #$4 y comparacion
|      $0437D4  rts       exit 1 (valor calculado en d0)
|      $0437D8  rts       exit 2 (d0 = 0 - overflow detectado)
|      $0437DA  fall      cola compartida (siguiente funcion del ROM)
|
|  Notas forenses (por que NO es rederivable por GCC 1:1):
|    1. La secuencia `swap; clr.w` repetida sobre d0,d2,d4,d5,d6 es un
|       "shift left 16 bits" idiomatico en 68000: convierte una word en
|       long alineada al high half. GCC no emite este patron; usaria
|       lsl.l #16 o cambiaria el tipo del temporal.
|    2. Dos usos de `exg    dX,dY` (0xC1/0xC3 XX) para reordenar sin pasar
|       por RAM/pila. GCC no genera exg (no forma parte de su cost model).
|    3. La subrutina interna $04374C se invoca via `bsr.w` con desplazamiento
|       backward corto desde $04371E y desde $043742, y NO tiene entry
|       independiente en la tabla de simbolos. Es un helper "amigable" del
|       cuerpo principal. En C se representaria como static inline con
|       parametros por registro pero GCC nunca lo emitiria asi.
|    4. `move.l d7, -(a7); asr.l #1, d7; add.l (a7)+, d7` en $043782 es
|       un ceil((x+neg_x)/2) hand-coded (equivalent a ceildiv(x, 2))
|       con auto-decrement/increment de SP para reutilizar d7 como
|       backup temporal. GCC lo haria con dos moves + arithmetic o con
|       una sequencia mas explicita usando link/unlk.
|    5. Dos rts consecutivos (`$437D4: rts`, `$437D8: rts`) sin bra
|       intermedio - dos salidas independientes de la funcion. GCC casi
|       siempre emite un solo rts final con branch a el.
|
|  Toolchain:  m68k-linux-gnu-gcc -m68000 -Wa,--register-prefix-optional
|  ============================================================================

        .text
        .globl  Geom_Proj_Clamp_0436DE
        .type   Geom_Proj_Clamp_0436DE, @function
        .section .text.Geom_Proj_Clamp_0436DE, "ax", @progbits

Geom_Proj_Clamp_0436DE:
        tst.b   0x108179.l                     | +00  if (flag_108179 != 0)
        bne.w   .Lfail_clear                   | +06    goto fail
        tst.l   d0                             | +0a  if (d0 < 0)
        bmi.w   .Lfail_clear                   | +0c    goto fail
        tst.l   d1                             | +10  if (d1 >= 0)
        bpl.w   .Lok_start                     | +12    goto ok
.Lfail_clear:                                  | $0436F4
        clr.l   d0                             | +16  d0 = 0
        clr.l   d1                             | +18  d1 = 0
        bra.w   .Ltail_common                  | +1a  goto tail ($437DA)
                                              |
                                              | ---- fase de preprocesado ----
.Lok_start:                                    | $0436FC
        move.w  #0x200, d4                     | +1e  d4 = $200
        sub.w   d4, d1                         | +22  d1 -= $200
        neg.w   d1                             | +24  d1 = -d1
        sub.w   d4, d3                         | +26  d3 -= $200
        neg.w   d3                             | +28  d3 = -d3
        cmp.w   d0, d2                         | +2a  if (d2 >= d0)
        bcc.w   .Lskip_xchg1                   | +2c
        exg      d0, d2                         | +30    swap d0,d2
.Lskip_xchg1:                                  | $043710
        cmp.w   d1, d3                         | +32  if (d3 >= d1)
        bcc.w   .Lskip_xchg2                   | +34
        exg      d1, d3                         | +38    swap d1,d3
.Lskip_xchg2:                                  | $043718
                                              |
                                              | ---- fase 1: reduccion X ----
        move.w  0x108164.l, d4                 | +3a  d4 = camera_x
        move.w  #0x1e,  d5                     | +40  d5 = clamp_min = $1E
        move.w  #0x117, d6                     | +44  d6 = clamp_max = $117
        bsr.w   .Lreduce                       | +48  bsr $4374C
                                              |
                                              | ---- fase 2: reduccion Y ----
        exg      d1, d0                         | +4c  swap d0,d1 (X<->Y)
        move.w  d3, d2                         | +4e  d2 = d3
        move.w  0x108166.l, d4                 | +50  d4 = camera_y
        sub.w   0x10817a.l, d4                 | +56  d4 -= camera_y_offset
        move.w  #0x0,  d5                      | +5c  d5 = 0
        move.w  #0xd0, d6                      | +60  d6 = $D0
        bsr.w   .Lreduce                       | +64  bsr $4374C
        exg      d1, d0                         | +68  swap back Y<->X
        bra.w   .Ltail_common                  | +6a  goto tail ($437DA)
                                              |
                                              | ---- .Lreduce ($04374C): sub-rutina de reduccion ----
                                              | Toma d0,d2,d4,d5,d6 (todos "words con basura arriba"),
                                              | los sube 16 bits (swap+clr.w), calcula
                                              | (d0+d2)/2 - d4 + $10000, clampa a [$0..$20000],
                                              | y devuelve valor final en d0 o 0 en overflow.
.Lreduce:                                      | $04374C
        swap    d0                             | +6e  d0 = d0 << 16
        clr.w   d0                             | +70
        swap    d2                             | +72  d2 = d2 << 16
        clr.w   d2                             | +74
        swap    d4                             | +76  d4 = d4 << 16
        clr.w   d4                             | +78
        swap    d5                             | +7a  d5 = d5 << 16
        clr.w   d5                             | +7c
        swap    d6                             | +7e  d6 = d6 << 16
        clr.w   d6                             | +80
                                              |
        move.l  d0, d7                         | +82  d7 = d0
        add.l   d2, d7                         | +84  d7 += d2   (= d0+d2)
        asr.l   #0x1, d7                       | +86  d7 >>= 1    (= (d0+d2)/2)
        sub.l   d4, d7                         | +88  d7 -= d4    (= mid - camera)
        addi.l  #0x10000, d7                   | +8a  d7 += $10000
        cmpi.l  #0x20000, d7                   | +90  if (d7 >= $20000)
        bcs.w   .Loverflow                     | +96    goto overflow (unsigned <)
                                              |
                                              | NOTA CRITICA: bcs.w salta a $0437D6
                                              | (rama overflow: moveq #0, d0; rts).
                                              | Solo el fall-through llega al subi.l.
        subi.l  #0x10000, d7                   | +9a  d7 -= $10000  (fall-through)
        sub.l   d5, d0                         | +a0  d0 -= d5 (clamp_min)
        sub.l   d6, d2                         | +a2  d2 -= d6 (clamp_max)

| CORRECCION: el offset +$a2 real del subi.l seria +$9E si
| el bcs.w NO ocupara 4 B. Recalculamos: bcs.w=$65 00 00 60 (4 B),
| ROM $043774+4 = $043778 = fall-through position, y $043778 -
| $0436DE = $9A. Luego subi.l en +$9A (6 B) termina en +$A0.
| Sub.l d5,d0 en +$A0 termina en +$A2. Sub.l d6,d2 en +$A2
| termina en +$A4. Move.l d7,-(a7) en +$A4 termina en +$A6.

        move.l  d7, -(a7)                      | +a4  push d7
        asr.l   #0x1, d7                       | +a6  d7 >>= 1
        add.l   (a7)+, d7                      | +a8  d7 += pop  (=1.5*d7 aprox)
        bmi.w   .Lclamp_neg                    | +aa  if (d7 < 0) branch to $437A4
                                              |
                                              | ---- rama d7 >= 0 ($0437 8C) ----
        tst.l   d0                             | +ae  if (d0 < 0)
        bge.w   .Lok_d0_pos                    | +b0
        clr.l   d7                             | +b4    d7 = 0
        bra.w   .Lmerge_pos                    | +b6  goto $437A0 (merge)
.Lok_d0_pos:                                   | $043798
        cmp.l   d0, d7                         | +ba  if (d7 <= d0)
        ble.w   .Lmerge_pos                    | +bc  goto $437A0
        move.l  d0, d7                         | +c0    d7 = d0
                                              |
                                              | ---- merge point $0437A0 ----
.Lmerge_pos:                                   | $0437A0
        bra.w   .Lclamp_final                  | +c2  goto $437B8
                                              |
                                              | ---- rama d7 < 0 ($0437A4) ----
.Lclamp_neg:                                   | $0437A4
        tst.l   d2                             | +c6  if (d2 <= 0)
        ble.w   .Lclamp_zero                   | +c8
        clr.l   d7                             | +cc    d7 = 0
        bra.w   .Lclamp_final                  | +ce
.Lclamp_zero:                                  | $0437B0
        cmp.l   d2, d7                         | +d2  if (d7 >= d2)
        bge.w   .Lclamp_final                  | +d4
        move.l  d2, d7                         | +d8    d7 = d2
                                              |
                                              | ---- clamp final +/- $80000 ----
.Lclamp_final:                                 | $0437B8
        asr.l   #0x4, d7                       | +dc  d7 >>= 4  (/16)
        move.l  #0x80000, d2                   | +de  d2 = +$80000
        cmp.l   d2, d7                         | +e4  if (d7 <= +$80000)
        ble.w   .Lclamp_neg_bound              | +e6
        move.l  d2, d7                         | +ea    d7 = +$80000
.Lclamp_neg_bound:                             | $0437C8
        neg.l   d2                             | +ee  d2 = -$80000
        cmp.l   d2, d7                         | +f0  if (d7 >= -$80000)
        bge.w   .Lclamp_return                 | +f2
        move.l  d2, d7                         | +f6    d7 = -$80000
.Lclamp_return:                                | $0437D2
        move.l  d7, d0                         | +f4  return d7
        rts                                    | +f6  <-- $0437D4
                                              |
                                              | ---- overflow return ($0437D6) ----
.Loverflow:                                    | $0437D6
        moveq   #0x0, d0                       | +f8  d0 = 0
        rts                                    | +fa  <-- $0437D8

        .equ    .Ltail_common, Sub_000437DA    | cola compartida con la siguiente funcion

        .size   Geom_Proj_Clamp_0436DE, .-Geom_Proj_Clamp_0436DE
