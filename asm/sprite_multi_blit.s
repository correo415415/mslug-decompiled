| ============================================================================
|  Metal Slug 1 - asm/sprite_multi_blit.s
|  ----------------------------------------------------------------------------
|  Wave T (Script / Draw / Task primitives) - funcion #2
|
|  Sprite_MultiBlitClippedX  @ $043fac  (98 bytes, 14 callers)
|
|  Recorre una lista de "comandos de sprite" apuntada por a1 y, para
|  cada uno, llama al blitter de fila $43f5e con clipping X en
|  [-80, +392] (rango visible extendido del Neo Geo horizontal).
|
|  Formato del bloque apuntado por a1 (a byte offsets, little-endian?
|  no: 68k es big-endian):
|      +$00  word  dx0        // desplazamiento X inicial (word)
|      +$02  word  dy0        // desplazamiento Y inicial (word)
|      +$04  word  outer_cnt  // #filas outer (d2)
|      +$06  word  inner_cnt  // #columnas inner (d3)
|      +$08  ...   payload    // bytes leidos por (a1)+ en el bucle
|
|  Los offsets pos_x/pos_y de la entidad viven en $22/$24 (word), igual
|  que en Entity_CopyTransform (Wave S).
|
|  El bit 0 de $3a(a6) marca "aplica delta"; hay DOS `btst.b #0, $3a(a6)`
|  seguidos de `beq.w +2` que caen a la instruccion inmediata: el branch
|  es sintacticamente valido pero semanticamente muerto porque la caida
|  natural y el destino del branch son la MISMA direccion. Un compilador
|  jamas emite esto - es un artefacto de codigo hand-written que
|  posiblemente serialice una decision temprana ya evaluada.
|
|  Entrada (registros absolutos):
|      a6 : entidad actual (pos_x en $22, pos_y en $24, flag en $3a)
|      a1 : puntero al bloque de comandos de sprite
|
|  Salida: ninguna (retorna via rts).
|
|  Hallazgos forenses (asm a mano):
|    1. movem.l asimetrico anidado: exterior guarda d1/d3, interior
|       d0-d3/a1. GCC nunca emite dos movem con diferente mascara sobre
|       la misma pila para un unico frame.
|    2. `btst.b #0, $3a(a6) ; beq.w +2` duplicado, con destino y caida
|       natural iguales. Rama muerta.
|    3. jsr PC-relativo $43f5e(pc) codificado en 4 bytes (4EBA FF72),
|       mientras que otros jsr del ROM son abs.l (6 B). Uso mixto de
|       PC-rel y abs.l dentro de la misma funcion es firma de asm a mano.
|    4. dbra sustituido por combos `subq.w #1,dX ; bne.b`. dbra habria
|       ahorrado 2 bytes por bucle, pero el codigo aqui usa la forma
|       larga (mas comun en macros de assemblers de la epoca).
|  ============================================================================

        .text
        .globl  Sprite_MultiBlitClippedX
        .type   Sprite_MultiBlitClippedX, @function
        .section .text.Sprite_MultiBlitClippedX, "ax", @progbits

Sprite_MultiBlitClippedX:
        move.w  0x22(a6), d0            | +00  30 2e 00 22    d0 = pos_x
        move.w  0x24(a6), d1            | +04  32 2e 00 24    d1 = pos_y
        move.w  (a1), d2                | +08  34 11          d2 = dx0
        btst.b  #0, 0x3a(a6)            | +0a  08 2e 00 00 00 3a    prueba flag
        beq.w   .Lafter_flag_entry      | +10  67 00 00 02    rama muerta: cae en +14
.Lafter_flag_entry:
        add.w   d2, d0                  | +14  d0 42          d0 += dx0
        add.w   2(a1), d1               | +16  d2 69 00 02    d1 += dy0
        move.w  4(a1), d2               | +1a  34 29 00 04    d2 = outer_cnt
        move.w  6(a1), d3               | +1e  36 29 00 06    d3 = inner_cnt
        addq.l  #8, a1                  | +22  50 89          a1 += 8 (skip header)
.Louter:
        movem.l d1/d3, -(a7)            | +24  48 e7 50 00    save (d1,d3)
.Linner:
        cmpi.w  #0xffb0, d0             | +28  0c 40 ff b0    d0 < -80 ?
        blt.w   .Linner_skip            | +2c  6d 00 00 18    si, skip
        cmpi.w  #0x188, d0              | +30  0c 40 01 88    d0 > +392 ?
        bgt.w   .Linner_skip            | +34  6e 00 00 10    si, skip
        move.b  (a1)+, d7               | +38  1e 19          d7 = *a1++
        movem.l d0-d3/a1, -(a7)         | +3a  48 e7 f0 40    save (d0..d3,a1)
        jsr     .Lblitter_row(pc)       | +3e  4e ba ff 72    -> $43f5e (PC-rel corto)
        movem.l (a7)+, d0-d3/a1         | +42  4c df 02 0f    restore
.Linner_skip:
        subq.w  #8, d1                  | +46  51 41          d1 -= 8 (avanza fila Y)
        subq.w  #1, d3                  | +48  53 43          d3--
        bne.b   .Linner                 | +4a  66 dc          bucle inner
        movem.l (a7)+, d1/d3            | +4c  4c df 00 0a    restore (d1,d3)
        addq.w  #8, d0                  | +50  50 40          d0 += 8 (avanza col X)
        btst.b  #0, 0x3a(a6)            | +52  08 2e 00 00 00 3a    prueba flag (2a vez)
        beq.w   .Lafter_flag_outer      | +58  67 00 00 02    rama muerta: cae en +5c
.Lafter_flag_outer:
        subq.w  #1, d2                  | +5c  53 42          d2--
        bne.b   .Louter                 | +5e  66 c4          bucle outer
        rts                             | +60  4e 75
        .size   Sprite_MultiBlitClippedX, .-Sprite_MultiBlitClippedX

| ---------------------------------------------------------------------------
|  Alias PC-relativo hacia $43f5e (blitter de fila). Como esta funcion
|  vive en $43fac y jsr(pc) usa desplazamiento con signo de 16 bits
|  centrado en PC+2, la distancia (-$4E) cabe holgadamente en la forma
|  corta 4EBA FF72.  El simbolo Sub_00043F5E se define via --defsym en
|  tools/symbols.py.
| ---------------------------------------------------------------------------
        .equ    .Lblitter_row, Sub_00043F5E
