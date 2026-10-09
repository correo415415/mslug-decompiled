| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave IIIII — motor de paletas, spawn de rejillas de sprites, sondeo de
|  pausa  (asm/palette_engine_sprite_grid_pause_0133b0.s)
|  Región: $0133B0..$013D18  (2,082 B, 26 entradas, 8 huecos)
| ============================================================================
|
|  A. QUÉ HAY AQUÍ
|  Runtime temprano de vídeo que cierra la zona `$0133B0..$013D6A` entre
|  Entity_FlushSlotHistory_013600 / sprite_allocator_0139xx.s /
|  fix_pause_text_013d20.s. Tres bloques:
|
|  1) $0133B0..$01390E  Motor de paletas. Paleta sombra en `$10A2D4`
|     (256 slots x 32 B = 16 colores; word 0 de cada slot = flag "dirty"),
|     tabla `$2F30` (32768 words: índice RGB555 r<<10|g<<5|b → color Neo Geo
|     con bit de brillo), rampas `$12F30` (32 punteros a tablas de 32 B que
|     dan, para cada distancia |dst-src| y paso t, el incremento por canal),
|     bloque de fade `$10A2C8..D2` (C8 modo: 0 sin fade, 1 oscurecer, $FF
|     aclarar; CA/CB/CC nivel por canal 0..31; CE flag flush; D0/D1 retardo
|     de refresco; D2 banco del white-out).
|     - Pal_LoadRaw16_0133b0: 15 colores desde (a2) en formato 3 bytes
|       {r,g,b} (+1 relleno) → convierte por `$2F30` y marca el slot dirty;
|       limpia bit 0 del descriptor (a1,d1). Pal_ClearSlot16_0133e6: slot a
|       cero y descriptor = $80.
|     - PalAnim_StepSlot_013408: descriptor de animación en (a1,d1): +2 ptr
|       origen, +$A ptr destino, +8 periodo ($FFFF = fin), +$10 contador,
|       +$E paso actual (0..31), +$F incremento. Cada periodo avanza el paso
|       y llama a PalAnim_Blend16_013480; al llegar a 32 o terminar
|       (d7≠0: todos los canales igualados) copia destino→origen y pone
|       bit 0 (idle) / limpia bit 1.
|     - PalAnim_Blend16_013480: para los 16 colores, PalAnim_StepRGB_013504
|       interpola cada canal de (a2) hacia (a3) usando la rampa `$12F30`
|       [|Δ|][paso] (signo aparte), cuenta canales igualados (d7==3 ⇒ color
|       terminado, d0=-1); luego aplica el fade global según `$10A2C8`
|       (Pal_ApplyFadeDarken_013624 resta rampa[c][nivel], _Lighten_013694
|       suma rampa[31-c][nivel]) y empaqueta vía `$2F30` en la sombra.
|     - Pal_PackRGB_01370a / Pal_UnpackRGB_013752: conversión entre RGB555
|       (con bits 13/14 de brillo bajo) y el formato Neo Geo
|       (dark bit 15, r/g/b 4 bits + LSB común en bits 12..14).
|     - Pal_ShadowClearAll_01379a: borra los 8 KB de sombra, `$10A2D2`=0 y
|       encadena SpriteTable_Init256_0526b8.
|     - Pal_FlushDirtyToHW_0137c6: si `$10A2CE`, selecciona banco de paleta
|       (`$3A000F`) y copia cada slot dirty (196 primeros + el slot $FF
|       aparte) a la RAM de paleta `$400000`, limpiando el flag; gestiona el
|       retardo `$10A2D0/D1` alternando `$3A000F`/`$3A001F`.
|     - Pal_WhiteOutNextBank_01387e / Pal_WhiteOutIsDone_0138e6: rellena un
|       banco de 16 slots con $7FFF (blanco) por frame (`$10A2D2` 0..15, $FF
|       = fin) sobre el banco alternativo `$3A001F`.
|     - Sprite_AdvanceY74_013906: `$1C(a0)` += $74 (relleno de 8 B).
|
|  2) $0139FE..$013C3C  Rejillas de sprites (strip de tiles → SCB1):
|     - Sprite_FillTileGrid_0139fe: con a0 = mapa de tiles (filas de d3
|       words), d4 = {alto<<16 | ancho}, d7 flags de flip (bit0 H, bit1 V →
|       recorre el mapa invertido y aplica eor a los atributos), escribe por
|       columna de sprite d0 los pares {tile, attr} en SCB1 (`$3C0000`,
|       dirección (col+1)<<6 + fila*2 con wrap a 32 filas), avanzando a la
|       siguiente columna o volviendo a d5 si supera d6.
|     - SpriteAlloc_ResetCounters_013aac / SpriteAlloc_LoadBase_013ac8:
|       `$10E1F4/FE` a 0, d1 = `$10E1F6`-1; d0 = `$10E1FA` → `$10E1FC`,
|       d1 = $17B (380 sprites).
|     - Sprite_SpawnGridB_013ade / _GridA_013b36: reservan ancho sprites con
|       Spawn_TypeB_013952 / Spawn_TypeA_013982, rellenan la rejilla y
|       escriben SCB2 (shrink `$8201+n` = alto<<7|ancho) y SCB3 (`$8401+n`
|       = y<<7 | alto). Sprite_SpawnGridB_Scaled_013b4c: idem con zoom d5
|       (hi = H, lo = V): calcula posiciones centradas y escribe SCB2
|       individual por columna distribuyendo el ancho reducido (acumulador
|       de error d7/d2) + variantes de entrada (GridA / sólo refresco).
|     - Vec_PolarToXY_013c0e: d1,d2 = r*cos/sin(ángulo d0) con tablas
|       `$2C07AC` / `$2C072C` (8.8). Div_FixedRatio_013c2c: d1 = (d1<<8)/|d2|
|       +1.
|
|  3) $013C3C..$013D18  Sondeo de pausa (PAUSE):
|     - Pause_Poll_013c3c: sin modo de pago (`$10FD82`=0) y con `$10E274`
|       (en partida), construye en d7 la máscara de START de los jugadores
|       activos (`$10FDB6/B7`==1 → bits 1/3) y la compara con las pulsaciones
|       nuevas `$10E20D`. Flanco → `$10E272`=$FF (pausado), `$10E273`=0 y
|       sonido $10E0 (InputGuardCall219c); si ya pausado → Pause_Active.
|     - Pause_Active_013cae: START de nuevo → despausa (sonido $10E0 vía
|       `$2222`, $10E1 vía `$2352`, borra el texto); si no, parpadea "PAUSE"
|       con `$10E273` (dibuja en 0 mod 32, borra en 24) → SetC (pausado).
|     - Pause_Poll_Reject_013caa / Pause_Clear_013d12: ClearC; `$10E272`=0.
|
|  B. CÓMO SE DESCUBRIÓ
|  Huecos de measure_coverage en `$0133B0..$013D6A`. Los campos del bloque
|  `$10A2C8..D2` ya estaban documentados en pubcleaner_10a2cx_052712.s y en
|  los PalFade_* de Wave HHHHH (que escriben CA/CB/CC); `$2F30` y `$12F30`
|  son tablas en zonas DATA (pendientes de transcribir) referenciadas por
|  `lea`. Los nombres de rejilla salen de los callers SpriteBlock20x14_* y
|  de los registros SCB1-3 del LSPC.
|
|  C. DEPENDENCIAS EXTERNAS
|  Spawn_TypeA_013982, Spawn_TypeB_013952, SpriteTable_Init256_0526b8,
|  Fix_DrawPause_013d46, Fix_DrawPauseBlank_013d3e, InputGuardCall219c
|  ($2352), `$2222` (sonido), tablas `$2F30`, `$12F30`, `$2C072C/$2C07AC`,
|  islas C SetXN_0138f8 / SetC_013d06 / ClearC_013d0c. HW: `$3A000F/$3A001F`
|  (banco de paleta), `$400000` (RAM de paleta), `$3C0000` (LSPC).
|
|  D. ESTADO
|  26/26 entradas byte-exactas. Zona `$0133B0..$013D6A` cerrada. Pendiente:
|  transcribir `$12F30` (rampas) y `$2F30` (LUT RGB) como datos.
|
|  E. NOTAS
|  PalAnim_StepRGB repite el mismo bloque tres veces (R,G,B) con registros
|  distintos (d2/d3/d1) en vez de un bucle: así en la ROM.
|
|  F. VERIFICACIÓN
|  Cada sección .text.<Sym> se coloca en su dirección CPU absoluta y
|  reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Pal_LoadRaw16_0133b0  @ $0133B0  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_LoadRaw16_0133b0, "ax", @progbits
        .global Pal_LoadRaw16_0133b0
Pal_LoadRaw16_0133b0:
        movea.l a3,a4                           | +000
        addq.w  #0x2,a3                         | +002
        addq.l  #0x4,a2                         | +004
        move.w  #0xe,d5                         | +006
.L0133ba:
        moveq   #0,d4                           | +00a
        move.b  (a2)+,d4                        | +00c
        lsl.w   #0x5,d4                         | +00e
        or.b    (a2)+,d4                        | +010
        lsl.w   #0x5,d4                         | +012
        or.b    (a2)+,d4                        | +014
        addq.l  #0x1,a2                         | +016
        lea     0x2f30.l,a0                     | +018
        add.w   d4,d4                           | +01e
        adda.l  d4,a0                           | +020
        move.w  (a0),d4                         | +022
        move.w  d4,(a3)+                        | +024
        dbra    d5,.L0133ba                     | +026
        andi.b  #0xfe,(a1,d1.w)                 | +02a
        move.w  #0x1,(a4)                       | +030
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Pal_ClearSlot16_0133e6  @ $0133E6  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_ClearSlot16_0133e6, "ax", @progbits
        .global Pal_ClearSlot16_0133e6
Pal_ClearSlot16_0133e6:
        movea.l a3,a4                           | +000
        addq.w  #0x2,a3                         | +002
        moveq   #0,d4                           | +004
        move.w  d4,(a3)+                        | +006
        move.l  d4,(a3)+                        | +008
        move.l  d4,(a3)+                        | +00a
        move.l  d4,(a3)+                        | +00c
        move.l  d4,(a3)+                        | +00e
        move.l  d4,(a3)+                        | +010
        move.l  d4,(a3)+                        | +012
        move.l  d4,(a3)+                        | +014
        move.b  #0x80,(a1,d1.w)                 | +016
        move.w  #0x1,(a4)                       | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  PalAnim_StepSlot_013408  @ $013408  (120 B)
| ----------------------------------------------------------------------------
        .section .text.PalAnim_StepSlot_013408, "ax", @progbits
        .global PalAnim_StepSlot_013408
PalAnim_StepSlot_013408:
        movem.l d0-d7/a0-a6,-(a7)               | +000
        move.w  0x8(a1,d1.w),d2                 | +004
        cmpi.w  #0xffff,d2                      | +008
        beq.w   .L013462                        | +00c
        subq.w  #0x1,0x10(a1,d1.w)              | +010
        tst.w   0x10(a1,d1.w)                   | +014
        bne.w   .L01345c                        | +018
        move.w  0x8(a1,d1.w),d2                 | +01c
        move.w  d2,0x10(a1,d1.w)                | +020
        move.b  0xf(a1,d1.w),d2                 | +024
        bne.w   .L013438                        | +028
        move.b  #0x1,d2                         | +02c
.L013438:
        add.b   d2,0xe(a1,d1.w)                 | +030
        cmpi.b  #0x20,0xe(a1,d1.w)              | +034
        bge.w   .L013462                        | +03a
        movea.l 0x2(a1,d1.w),a2                 | +03e
        movea.l 0xa(a1,d1.w),a3                 | +042
        move.b  0xe(a1,d1.w),d5                 | +046
        jsr     PalAnim_Blend16_013480(pc)      | +04a
        and.w   d7,d7                           | +04e
        bne.w   .L013462                        | +050
.L01345c:
        movem.l (a7)+,d0-d7/a0-a6               | +054
        rts                                     | +058
.L013462:
        move.l  0xa(a1,d1.w),d2                 | +05a
        move.l  d2,0x2(a1,d1.w)                 | +05e
        andi.b  #0xfd,(a1,d1.w)                 | +062
        ori.b   #0x1,(a1,d1.w)                  | +068
        clr.w   0x8(a1,d1.w)                    | +06e
        movem.l (a7)+,d0-d7/a0-a6               | +072
        rts                                     | +076

| ----------------------------------------------------------------------------
|  PalAnim_Blend16_013480  @ $013480  (132 B)
| ----------------------------------------------------------------------------
        .section .text.PalAnim_Blend16_013480, "ax", @progbits
        .global PalAnim_Blend16_013480
PalAnim_Blend16_013480:
        movem.l d1/a1,-(a7)                     | +000
        lea     0x10a2d4.l,a4                   | +004
        move.w  #0xf,d6                         | +00a
        moveq   #0,d0                           | +00e
        moveq   #0,d4                           | +010
        move.w  #0xffff,d7                      | +012
.L013496:
        jsr     PalAnim_StepRGB_013504(pc)      | +016
        andi.w  #0x1f,d2                        | +01a
        andi.w  #0x1f,d3                        | +01e
        andi.w  #0x1f,d4                        | +022
        tst.b   0x10a2c8.l                      | +026
        beq.w   .L0134cc                        | +02c
        cmpi.b  #0xff,0x10a2c8.l                | +030
        beq.w   .L0134c4                        | +038
        jsr     Pal_ApplyFadeDarken_013624(pc)  | +03c
        bra.w   .L0134d4                        | +040
.L0134c4:
        jsr     Pal_ApplyFadeLighten_013694(pc) | +044
        bra.w   .L0134d4                        | +048
.L0134cc:
        lsl.w   #0x5,d2                         | +04c
        or.w    d3,d2                           | +04e
        lsl.w   #0x5,d2                         | +050
        or.w    d2,d4                           | +052
.L0134d4:
        lea     0x2f30.l,a0                     | +054
        andi.l  #0xffff,d4                      | +05a
        add.l   d4,d4                           | +060
        adda.l  d4,a0                           | +062
        move.w  (a0),d4                         | +064
        move.w  d4,(a4,d1.w)                    | +066
        and.w   d0,d7                           | +06a
        addq.w  #0x2,d1                         | +06c
        dbra    d6,.L013496                     | +06e
        movem.l (a7)+,d1/a1                     | +072
        lea     0x10a2d4.l,a4                   | +076
        move.w  #0x1,(a4,d1.w)                  | +07c
        rts                                     | +082

| ----------------------------------------------------------------------------
|  PalAnim_StepRGB_013504  @ $013504  (252 B)
| ----------------------------------------------------------------------------
        .section .text.PalAnim_StepRGB_013504, "ax", @progbits
        .global PalAnim_StepRGB_013504
PalAnim_StepRGB_013504:
        move.w  d1,-(a7)                        | +000
        movem.l d5-d7/a0,-(a7)                  | +002
        tst.b   d5                              | +006
        beq.w   .L0135f6                        | +008
        andi.w  #0x1f,d5                        | +00c
        subq.b  #0x1,d5                         | +010
        moveq   #0,d7                           | +012
        clr.w   d0                              | +014
        clr.w   d4                              | +016
        move.b  (a2)+,d0                        | +018
        move.b  (a3)+,d4                        | +01a
        move.b  d4,d6                           | +01c
        clr.b   d2                              | +01e
        sub.w   d0,d4                           | +020
        beq.w   .L013554                        | +022
        bpl.w   .L013534                        | +026
        neg.w   d4                              | +02a
        move.b  #0xff,d2                        | +02c
.L013534:
        add.w   d4,d4                           | +030
        add.w   d4,d4                           | +032
        lea     Sub_00012F30(pc),a0             | +034  -> $012F30 (hueco futuro, defsym forward)
        movea.l (a0,d4.w),a0                    | +038
        move.b  (a0,d5.w),d4                    | +03c
        tst.b   d2                              | +040
        beq.w   .L01354c                        | +042
        neg.b   d4                              | +046
.L01354c:
        add.b   d4,d0                           | +048
        cmp.b   d0,d6                           | +04a
        bne.w   .L013558                        | +04c
.L013554:
        addq.b  #0x1,d7                         | +050
        move.b  d6,d0                           | +052
.L013558:
        move.b  d0,d2                           | +054
        clr.w   d0                              | +056
        clr.w   d4                              | +058
        move.b  (a2)+,d0                        | +05a
        move.b  (a3)+,d4                        | +05c
        move.b  d4,d6                           | +05e
        clr.b   d3                              | +060
        sub.w   d0,d4                           | +062
        beq.w   .L013596                        | +064
        bpl.w   .L013576                        | +068
        neg.w   d4                              | +06c
        move.b  #0xff,d3                        | +06e
.L013576:
        add.w   d4,d4                           | +072
        add.w   d4,d4                           | +074
        lea     Sub_00012F30(pc),a0             | +076  -> $012F30 (hueco futuro, defsym forward)
        movea.l (a0,d4.w),a0                    | +07a
        move.b  (a0,d5.w),d4                    | +07e
        tst.b   d3                              | +082
        beq.w   .L01358e                        | +084
        neg.b   d4                              | +088
.L01358e:
        add.b   d4,d0                           | +08a
        cmp.b   d0,d6                           | +08c
        bne.w   .L01359a                        | +08e
.L013596:
        addq.b  #0x1,d7                         | +092
        move.b  d6,d0                           | +094
.L01359a:
        move.b  d0,d3                           | +096
        clr.w   d0                              | +098
        clr.w   d4                              | +09a
        move.b  (a2)+,d0                        | +09c
        move.b  (a3)+,d4                        | +09e
        move.b  d4,d6                           | +0a0
        clr.b   d1                              | +0a2
        sub.w   d0,d4                           | +0a4
        beq.w   .L0135d8                        | +0a6
        bpl.w   .L0135b8                        | +0aa
        neg.w   d4                              | +0ae
        move.b  #0xff,d1                        | +0b0
.L0135b8:
        add.w   d4,d4                           | +0b4
        add.w   d4,d4                           | +0b6
        lea     Sub_00012F30(pc),a0             | +0b8  -> $012F30 (hueco futuro, defsym forward)
        movea.l (a0,d4.w),a0                    | +0bc
        move.b  (a0,d5.w),d4                    | +0c0
        tst.b   d1                              | +0c4
        beq.w   .L0135d0                        | +0c6
        neg.b   d4                              | +0ca
.L0135d0:
        add.b   d4,d0                           | +0cc
        cmp.b   d0,d6                           | +0ce
        bne.w   .L0135dc                        | +0d0
.L0135d8:
        addq.b  #0x1,d7                         | +0d4
        move.b  d6,d0                           | +0d6
.L0135dc:
        move.b  d0,d4                           | +0d8
        addq.l  #0x1,a2                         | +0da
        addq.l  #0x1,a3                         | +0dc
        cmpi.b  #0x3,d7                         | +0de
        bne.w   .L0135f6                        | +0e2
        move.w  #0xffff,d0                      | +0e6
        movem.l (a7)+,d5-d7/a0                  | +0ea
        move.w  (a7)+,d1                        | +0ee
        rts                                     | +0f0
.L0135f6:
        clr.w   d0                              | +0f2
        movem.l (a7)+,d5-d7/a0                  | +0f4
        move.w  (a7)+,d1                        | +0f8
        rts                                     | +0fa

| ----------------------------------------------------------------------------
|  Pal_ApplyFadeDarken_013624  @ $013624  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_ApplyFadeDarken_013624, "ax", @progbits
        .global Pal_ApplyFadeDarken_013624
Pal_ApplyFadeDarken_013624:
        move.l  d0,-(a7)                        | +000
        movem.l d5-d6/a0-a1,-(a7)               | +002
        clr.w   d5                              | +006
        lea     Sub_00012F30(pc),a1             | +008  -> $012F30 (hueco futuro, defsym forward)
        clr.w   d0                              | +00c
        move.b  d2,d0                           | +00e
        add.w   d0,d0                           | +010
        add.w   d0,d0                           | +012
        movea.l (a1,d0.w),a0                    | +014
        clr.w   d6                              | +018
        move.b  0x10a2ca.l,d6                   | +01a
        move.b  (a0,d6.w),d6                    | +020
        sub.b   d6,d2                           | +024
        move.b  d2,d5                           | +026
        lsl.w   #0x5,d5                         | +028
        clr.w   d0                              | +02a
        move.b  d3,d0                           | +02c
        add.w   d0,d0                           | +02e
        add.w   d0,d0                           | +030
        movea.l (a1,d0.w),a0                    | +032
        clr.w   d6                              | +036
        move.b  0x10a2cb.l,d6                   | +038
        move.b  (a0,d6.w),d6                    | +03e
        sub.b   d6,d3                           | +042
        or.b    d3,d5                           | +044
        lsl.w   #0x5,d5                         | +046
        clr.w   d0                              | +048
        move.b  d4,d0                           | +04a
        add.w   d0,d0                           | +04c
        add.w   d0,d0                           | +04e
        movea.l (a1,d0.w),a0                    | +050
        clr.w   d6                              | +054
        move.b  0x10a2cc.l,d6                   | +056
        move.b  (a0,d6.w),d6                    | +05c
        sub.b   d6,d4                           | +060
        andi.w  #0x1f,d4                        | +062
        or.w    d5,d4                           | +066
        movem.l (a7)+,d5-d6/a0-a1               | +068
        move.l  (a7)+,d0                        | +06c
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  Pal_ApplyFadeLighten_013694  @ $013694  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_ApplyFadeLighten_013694, "ax", @progbits
        .global Pal_ApplyFadeLighten_013694
Pal_ApplyFadeLighten_013694:
        move.l  d0,-(a7)                        | +000
        movem.l d5-d6/a0-a1,-(a7)               | +002
        lea     Sub_00012F30(pc),a1             | +006  -> $012F30 (hueco futuro, defsym forward)
        clr.w   d5                              | +00a
        move.w  #0x1f,d0                        | +00c
        sub.b   d2,d0                           | +010
        add.w   d0,d0                           | +012
        add.w   d0,d0                           | +014
        movea.l (a1,d0.w),a0                    | +016
        clr.w   d6                              | +01a
        move.b  0x10a2ca.l,d6                   | +01c
        move.b  (a0,d6.w),d6                    | +022
        add.b   d6,d2                           | +026
        move.b  d2,d5                           | +028
        lsl.w   #0x5,d5                         | +02a
        move.w  #0x1f,d0                        | +02c
        sub.b   d3,d0                           | +030
        add.w   d0,d0                           | +032
        add.w   d0,d0                           | +034
        movea.l (a1,d0.w),a0                    | +036
        clr.w   d6                              | +03a
        move.b  0x10a2cb.l,d6                   | +03c
        move.b  (a0,d6.w),d6                    | +042
        add.b   d6,d3                           | +046
        or.b    d3,d5                           | +048
        lsl.w   #0x5,d5                         | +04a
        move.w  #0x1f,d0                        | +04c
        sub.b   d4,d0                           | +050
        add.w   d0,d0                           | +052
        add.w   d0,d0                           | +054
        movea.l (a1,d0.w),a0                    | +056
        clr.w   d6                              | +05a
        move.b  0x10a2cc.l,d6                   | +05c
        move.b  (a0,d6.w),d6                    | +062
        add.b   d6,d4                           | +066
        andi.w  #0x1f,d4                        | +068
        or.w    d5,d4                           | +06c
        movem.l (a7)+,d5-d6/a0-a1               | +06e
        move.l  (a7)+,d0                        | +072
        rts                                     | +074

| ----------------------------------------------------------------------------
|  Pal_PackRGB_01370a  @ $01370A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_PackRGB_01370a, "ax", @progbits
        .global Pal_PackRGB_01370a
Pal_PackRGB_01370a:
        movem.l d1-d2,-(a7)                     | +000
        move.b  d4,d2                           | +004
        move.w  d4,d1                           | +006
        lsl.w   #0x3,d1                         | +008
        andi.w  #0x7800,d1                      | +00a
        bclr    #0xe,d4                         | +00e
        beq.w   .L013724                        | +012
        ori.w   #0x400,d1                       | +016
.L013724:
        add.w   d2,d2                           | +01a
        add.w   d2,d2                           | +01c
        andi.w  #0x3c0,d2                       | +01e
        bclr    #0xd,d4                         | +022
        beq.w   .L013738                        | +026
        ori.w   #0x20,d2                        | +02a
.L013738:
        add.w   d4,d4                           | +02e
        bclr    #0xd,d4                         | +030
        beq.w   .L013744                        | +034
        addq.w  #0x1,d4                         | +038
.L013744:
        andi.w  #0x1f,d4                        | +03a
        or.w    d1,d4                           | +03e
        or.w    d2,d4                           | +040
        movem.l (a7)+,d1-d2                     | +042
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Pal_UnpackRGB_013752  @ $013752  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_UnpackRGB_013752, "ax", @progbits
        .global Pal_UnpackRGB_013752
Pal_UnpackRGB_013752:
        movem.l d1-d2,-(a7)                     | +000
        move.w  d4,d1                           | +004
        lsr.w   #0x3,d1                         | +006
        andi.w  #0xf00,d1                       | +008
        bclr    #0xa,d4                         | +00c
        beq.w   .L01376a                        | +010
        ori.w   #0x4000,d1                      | +014
.L01376a:
        move.w  d4,d2                           | +018
        lsr.w   #0x2,d2                         | +01a
        andi.w  #0xf0,d2                        | +01c
        bclr    #0x5,d4                         | +020
        beq.w   .L01377e                        | +024
        ori.w   #0x2000,d1                      | +028
.L01377e:
        andi.w  #0x1f,d4                        | +02c
        bclr    #0x0,d4                         | +030
        beq.w   .L01378e                        | +034
        ori.w   #0x2000,d4                      | +038
.L01378e:
        lsr.w   #0x1,d4                         | +03c
        or.w    d1,d4                           | +03e
        or.w    d2,d4                           | +040
        movem.l (a7)+,d1-d2                     | +042
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Pal_ShadowClearAll_01379a  @ $01379A  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_ShadowClearAll_01379a, "ax", @progbits
        .global Pal_ShadowClearAll_01379a
Pal_ShadowClearAll_01379a:
        lea     0x10a2d4.l,a0                   | +000
        moveq   #0,d1                           | +006
        move.w  #0xff,d0                        | +008
.L0137a6:
        move.l  d1,(a0)+                        | +00c
        move.l  d1,(a0)+                        | +00e
        move.l  d1,(a0)+                        | +010
        move.l  d1,(a0)+                        | +012
        move.l  d1,(a0)+                        | +014
        move.l  d1,(a0)+                        | +016
        move.l  d1,(a0)+                        | +018
        move.l  d1,(a0)+                        | +01a
        dbra    d0,.L0137a6                     | +01c
        clr.b   0x10a2d2.l                      | +020
        jmp     0x526b8.l                       | +026

| ----------------------------------------------------------------------------
|  Pal_FlushDirtyToHW_0137c6  @ $0137C6  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_FlushDirtyToHW_0137c6, "ax", @progbits
        .global Pal_FlushDirtyToHW_0137c6
Pal_FlushDirtyToHW_0137c6:
        tst.b   0x10a2ce.l                      | +000
        beq.w   .L01382a                        | +006
        move.b  #0x1,0x3a000f.l                 | +00a
        lea     0x10a2d4.l,a0                   | +012
        movea.l #0x400000,a1                    | +018
        moveq   #32,d1                          | +01e
        move.w  #0xc3,d0                        | +020
.L0137ea:
        tst.w   (a0)                            | +024
        bne.w   .L013866                        | +026
        adda.l  d1,a0                           | +02a
        adda.l  d1,a1                           | +02c
.L0137f4:
        dbra    d0,.L0137ea                     | +02e
        lea     0x10a2d4.l,a0                   | +032
        tst.w   0x1fe0(a0)                      | +038
        beq.w   .L01382a                        | +03c
        adda.l  #0x1fe0,a0                      | +040
        movea.l #0x400000,a1                    | +046
        adda.l  #0x1fe2,a1                      | +04c
        clr.w   (a0)+                           | +052
        move.w  (a0)+,(a1)+                     | +054
        move.l  (a0)+,(a1)+                     | +056
        move.l  (a0)+,(a1)+                     | +058
        move.l  (a0)+,(a1)+                     | +05a
        move.l  (a0)+,(a1)+                     | +05c
        move.l  (a0)+,(a1)+                     | +05e
        move.l  (a0)+,(a1)+                     | +060
        move.l  (a0)+,(a1)+                     | +062
.L01382a:
        tst.b   0x10a2d1.l                      | +064
        bne.w   .L013856                        | +06a
        tst.b   0x10a2d0.l                      | +06e
        beq.w   .L01384c                        | +074
        subq.b  #0x1,0x10a2d0.l                 | +078
        move.b  #0x1,0x10a2d1.l                 | +07e
.L01384c:
        move.b  #0x1,0x3a000f.l                 | +086
        rts                                     | +08e
.L013856:
        move.b  #0x1,0x3a001f.l                 | +090
        subq.b  #0x1,0x10a2d1.l                 | +098
        rts                                     | +09e
.L013866:
        clr.w   (a0)+                           | +0a0
        addq.w  #0x2,a1                         | +0a2
        move.w  (a0)+,(a1)+                     | +0a4
        move.l  (a0)+,(a1)+                     | +0a6
        move.l  (a0)+,(a1)+                     | +0a8
        move.l  (a0)+,(a1)+                     | +0aa
        move.l  (a0)+,(a1)+                     | +0ac
        move.l  (a0)+,(a1)+                     | +0ae
        move.l  (a0)+,(a1)+                     | +0b0
        move.l  (a0)+,(a1)+                     | +0b2
        bra.w   .L0137f4                        | +0b4

| ----------------------------------------------------------------------------
|  Pal_WhiteOutNextBank_01387e  @ $01387E  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_WhiteOutNextBank_01387e, "ax", @progbits
        .global Pal_WhiteOutNextBank_01387e
Pal_WhiteOutNextBank_01387e:
        move.b  #0x1,0x3a001f.l                 | +000
        movea.l #0x400000,a0                    | +008
        move.b  0x10a2d2.l,d0                   | +00e
        andi.l  #0xff,d0                        | +014
        add.l   d0,d0                           | +01a
        lsl.l   #0x8,d0                         | +01c
        adda.l  d0,a0                           | +01e
        move.l  #0x7fff,d2                      | +020
        move.l  #0x7fff7fff,d1                  | +026
        move.w  #0xf,d0                         | +02c
.L0138ae:
        move.l  d2,(a0)+                        | +030
        move.l  d1,(a0)+                        | +032
        move.l  d1,(a0)+                        | +034
        move.l  d1,(a0)+                        | +036
        move.l  d1,(a0)+                        | +038
        move.l  d1,(a0)+                        | +03a
        move.l  d1,(a0)+                        | +03c
        move.l  d1,(a0)+                        | +03e
        dbra    d0,.L0138ae                     | +040
        move.b  #0x1,0x3a000f.l                 | +044
        addq.b  #0x1,0x10a2d2.l                 | +04c
        cmpi.b  #0x10,0x10a2d2.l                | +052
        blt.w   .L0138e4                        | +05a
        move.b  #0xff,0x10a2d2.l                | +05e
.L0138e4:
        rts                                     | +066

| ----------------------------------------------------------------------------
|  Pal_WhiteOutIsDone_0138e6  @ $0138E6  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Pal_WhiteOutIsDone_0138e6, "ax", @progbits
        .global Pal_WhiteOutIsDone_0138e6
Pal_WhiteOutIsDone_0138e6:
        cmpi.b  #0xff,0x10a2d2.l                | +000
        beq.w   SetXN_0138f8                    | +008

| ----------------------------------------------------------------------------
|  Sprite_AdvanceY74_013906  @ $013906  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_AdvanceY74_013906, "ax", @progbits
        .global Sprite_AdvanceY74_013906
Sprite_AdvanceY74_013906:
        addi.w  #0x74,0x1c(a0)                  | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Sprite_FillTileGrid_0139fe  @ $0139FE  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_FillTileGrid_0139fe, "ax", @progbits
        .global Sprite_FillTileGrid_0139fe
Sprite_FillTileGrid_0139fe:
        andi.w  #0x3,d7                         | +000
        movea.l #0x3c0000,a3                    | +004
        move.l  d0,-(a7)                        | +00a
        add.w   d3,d3                           | +00c
        add.w   d3,d3                           | +00e
        move.w  d2,d0                           | +010
        sub.w   d2,d2                           | +012
        mulu.w  d3,d0                           | +014
        swap    d2                              | +016
        add.w   d2,d2                           | +018
        add.w   d2,d2                           | +01a
        add.l   d2,d0                           | +01c
        adda.l  d0,a0                           | +01e
        moveq   #2,d2                           | +020
        btst    #0x1,d7                         | +022
        beq.w   .L013a38                        | +026
        swap    d4                              | +02a
        move.w  d4,d0                           | +02c
        swap    d4                              | +02e
        subq.w  #0x1,d0                         | +030
        add.w   d0,d0                           | +032
        add.w   d0,d0                           | +034
        adda.w  d0,a0                           | +036
        neg.l   d2                              | +038
.L013a38:
        btst    #0x0,d7                         | +03a
        beq.w   .L013a4a                        | +03e
        move.w  d4,d0                           | +042
        subq.w  #0x1,d0                         | +044
        mulu.w  d3,d0                           | +046
        adda.l  d0,a0                           | +048
        neg.w   d3                              | +04a
.L013a4a:
        move.l  (a7)+,d0                        | +04c
        andi.w  #0x1f,d1                        | +04e
        add.w   d1,d1                           | +052
        addq.w  #0x1,d0                         | +054
        lsl.w   #0x6,d0                         | +056
        add.w   d1,d0                           | +058
        addq.w  #0x1,d5                         | +05a
        lsl.w   #0x6,d5                         | +05c
        add.w   d1,d5                           | +05e
        addq.w  #0x1,d6                         | +060
        lsl.w   #0x6,d6                         | +062
        add.w   d1,d6                           | +064
.L013a64:
        swap    d4                              | +066
        movem.l d0/d4/a0,-(a7)                  | +068
.L013a6a:
        move.w  (a0),d1                         | +06c
        movem.w d0-d1,(a3)                      | +06e
        adda.w  d2,a0                           | +072
        addq.w  #0x1,d0                         | +074
        move.w  (a0),d1                         | +076
        eor.w   d7,d1                           | +078
        movem.w d0-d1,(a3)                      | +07a
        adda.w  d2,a0                           | +07e
        addq.w  #0x1,d0                         | +080
        move.b  d0,d1                           | +082
        andi.b  #0x3f,d1                        | +084
        bne.w   .L013a8e                        | +088
        subi.w  #0x40,d0                        | +08c
.L013a8e:
        subq.w  #0x1,d4                         | +090
        bne.b   .L013a6a                        | +092
        movem.l (a7)+,d0/d4/a0                  | +094
        adda.w  d3,a0                           | +098
        addi.w  #0x40,d0                        | +09a
        cmp.w   d0,d6                           | +09e
        bcc.w   .L013aa4                        | +0a0
        move.w  d5,d0                           | +0a4
.L013aa4:
        swap    d4                              | +0a6
        subq.w  #0x1,d4                         | +0a8
        bne.b   .L013a64                        | +0aa
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  SpriteAlloc_ResetCounters_013aac  @ $013AAC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteAlloc_ResetCounters_013aac, "ax", @progbits
        .global SpriteAlloc_ResetCounters_013aac
SpriteAlloc_ResetCounters_013aac:
        clr.w   0x10e1f4.l                      | +000
        clr.w   0x10e1fe.l                      | +006
        moveq   #0,d0                           | +00c
        move.w  0x10e1f6.l,d1                   | +00e
        subq.w  #0x1,d1                         | +014

| ----------------------------------------------------------------------------
|  SpriteAlloc_LoadBase_013ac8  @ $013AC8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteAlloc_LoadBase_013ac8, "ax", @progbits
        .global SpriteAlloc_LoadBase_013ac8
SpriteAlloc_LoadBase_013ac8:
        move.w  0x10e1fa.l,d0                   | +000
        move.w  d0,0x10e1fc.l                   | +006
        move.w  #0x17b,d1                       | +00c

| ----------------------------------------------------------------------------
|  Sprite_SpawnGridB_013ade  @ $013ADE  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_SpawnGridB_013ade, "ax", @progbits
        .global Sprite_SpawnGridB_013ade
Sprite_SpawnGridB_013ade:
        move.b  d4,d7                           | +000
        move.w  d1,d4                           | +002
        swap    d4                              | +004
        move.w  d0,d4                           | +006
        movem.l d2-d4/d7/a0,-(a7)               | +008
        jsr     Spawn_TypeB_013952(pc)          | +00c
        movem.l (a7)+,d2-d4/d7/a0               | +010
        .global Sprite_SpawnGridB_013ade__L013af2
Sprite_SpawnGridB_013ade__L013af2:
.L013af2:
        movem.l d0-d4,-(a7)                     | +014
        move.w  d0,d5                           | +018
        move.w  d1,d6                           | +01a
        moveq   #0,d1                           | +01c
        moveq   #0,d2                           | +01e
        swap    d4                              | +020
        move.w  d4,d3                           | +022
        swap    d4                              | +024
        jsr     Sprite_FillTileGrid_0139fe(pc)  | +026
        movem.l (a7)+,d0-d4                     | +02a
        movem.w d0-d1,-(a7)                     | +02e
        addi.w  #0x8201,d0                      | +032
        lsl.w   #0x7,d3                         | +036
        swap    d4                              | +038
        add.w   d4,d3                           | +03a
        movem.w d0/d3,0x3c0000.l                | +03c
        addi.w  #0x200,d0                       | +044
        lsl.w   #0x7,d2                         | +048
        movem.w d0/d2,0x3c0000.l                | +04a
        movem.w (a7)+,d0-d1                     | +052
        rts                                     | +056

| ----------------------------------------------------------------------------
|  Sprite_SpawnGridA_013b36  @ $013B36  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_SpawnGridA_013b36, "ax", @progbits
        .global Sprite_SpawnGridA_013b36
Sprite_SpawnGridA_013b36:
        move.b  d4,d7                           | +000
        move.w  d1,d4                           | +002
        swap    d4                              | +004
        move.w  d0,d4                           | +006
        movem.l d2-d4/d7/a0,-(a7)               | +008
        jsr     Spawn_TypeA_013982(pc)          | +00c
        movem.l (a7)+,d2-d4/d7/a0               | +010
        bra.b   Sprite_SpawnGridB_013ade__L013af2 | +014

| ----------------------------------------------------------------------------
|  Sprite_SpawnGridB_Scaled_013b4c  @ $013B4C  (194 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_SpawnGridB_Scaled_013b4c, "ax", @progbits
        .global Sprite_SpawnGridB_Scaled_013b4c
Sprite_SpawnGridB_Scaled_013b4c:
        movem.l d0/d2-d3/d5-d7,-(a7)            | +000
        jsr     Sprite_SpawnGridB_013ade(pc)    | +004
        movea.w d0,a0                           | +008
        movea.w d1,a1                           | +00a
        movem.l (a7)+,d0/d2-d3/d5-d7            | +00c
.L013b5c:
        movea.l #0x3c0000,a3                    | +010
        add.w   d6,d2                           | +016
        add.w   d7,d3                           | +018
        neg.w   d6                              | +01a
        neg.w   d7                              | +01c
        move.w  d5,-(a7)                        | +01e
        move.w  d5,d4                           | +020
        andi.w  #0xff,d5                        | +022
        addq.w  #0x1,d5                         | +026
        muls.w  d5,d7                           | +028
        asr.l   #0x8,d7                         | +02a
        add.w   d3,d7                           | +02c
        lsl.w   #0x7,d7                         | +02e
        subq.w  #0x1,d5                         | +030
        mulu.w  d5,d1                           | +032
        lsr.w   #0x8,d1                         | +034
        addq.w  #0x1,d1                         | +036
        add.w   d1,d7                           | +038
        lsr.w   #0x8,d4                         | +03a
        addq.w  #0x1,d4                         | +03c
        muls.w  d4,d6                           | +03e
        asr.l   #0x8,d6                         | +040
        add.w   d2,d6                           | +042
        lsl.w   #0x7,d6                         | +044
        move.w  a0,d4                           | +046
        addi.w  #0x8201,d4                      | +048
        movem.w d4/d7,(a3)                      | +04c
        addi.w  #0x200,d4                       | +050
        movem.w d4/d6,(a3)                      | +054
        move.w  (a7)+,d5                        | +058
        subi.w  #0x400,d4                       | +05a
        move.w  d5,d6                           | +05e
        move.w  d5,d7                           | +060
        andi.w  #0xff,d5                        | +062
        lsr.w   #0x4,d6                         | +066
        andi.w  #0xf00,d6                       | +068
        lsr.w   #0x4,d7                         | +06c
        not.b   d7                              | +06e
        andi.b  #0xf0,d7                        | +070
        or.w    d6,d5                           | +074
        moveq   #0,d2                           | +076
        subq.w  #0x1,d0                         | +078
.L013bc6:
        move.w  d5,d6                           | +07a
        add.b   d7,d2                           | +07c
        bcc.w   .L013bda                        | +07e
        cmpi.w  #0xff,d6                        | +082
        bls.w   .L013bda                        | +086
        subi.w  #0x100,d6                       | +08a
.L013bda:
        movem.w d4/d6,(a3)                      | +08e
        addq.w  #0x1,d4                         | +092
        dbra    d0,.L013bc6                     | +094
        move.w  a0,d0                           | +098
        move.w  a1,d1                           | +09a
        rts                                     | +09c
        movem.l d0/d2-d3/d5-d7,-(a7)            | +09e
        jsr     Sprite_SpawnGridA_013b36(pc)    | +0a2
        movea.w d0,a0                           | +0a6
        movea.w d1,a1                           | +0a8
        movem.l (a7)+,d0/d2-d3/d5-d7            | +0aa
        bra.w   .L013b5c                        | +0ae
        movea.w d0,a0                           | +0b2
        movea.w d1,a1                           | +0b4
        sub.w   d1,d0                           | +0b6
        neg.w   d0                              | +0b8
        addq.w  #0x1,d0                         | +0ba
        move.w  d4,d1                           | +0bc
        bra.w   .L013b5c                        | +0be

| ----------------------------------------------------------------------------
|  Vec_PolarToXY_013c0e  @ $013C0E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Vec_PolarToXY_013c0e, "ax", @progbits
        .global Vec_PolarToXY_013c0e
Vec_PolarToXY_013c0e:
        add.w   d0,d0                           | +000
        move.w  d1,d2                           | +002
        lea     0x2c07ac.l,a2                   | +004
        muls.w  (a2,d0.w),d1                    | +00a
        asr.l   #0x8,d1                         | +00e
        lea     0x2c072c.l,a2                   | +010
        muls.w  (a2,d0.w),d2                    | +016
        asr.l   #0x8,d2                         | +01a
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Div_FixedRatio_013c2c  @ $013C2C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Div_FixedRatio_013c2c, "ax", @progbits
        .global Div_FixedRatio_013c2c
Div_FixedRatio_013c2c:
        tst.w   d2                              | +000
        bge.w   .L013c34                        | +002
        neg.w   d2                              | +006
.L013c34:
        asl.w   #0x8,d1                         | +008
        divs.w  d2,d1                           | +00a
        addq.w  #0x1,d1                         | +00c
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Pause_Poll_013c3c  @ $013C3C  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Pause_Poll_013c3c, "ax", @progbits
        .global Pause_Poll_013c3c
Pause_Poll_013c3c:
        tst.b   0x10fd82.l                      | +000
        beq.w   .L013c4a                        | +006
        bra.w   ClearC_013d0c                   | +00a
.L013c4a:
        tst.b   0x10e274.l                      | +00e
        beq.w   ClearC_013d0c                   | +014
        move.b  0x10fdb6.l,d0                   | +018
        cmpi.b  #0x1,d0                         | +01e
        seq.b   d7                              | +022
        andi.b  #0x2,d7                         | +024
        move.b  0x10fdb7.l,d0                   | +028
        cmpi.b  #0x1,d0                         | +02e
        seq.b   d6                              | +032
        andi.b  #0x8,d6                         | +034
        or.b    d6,d7                           | +038
        tst.b   0x10e272.l                      | +03a
        bne.w   Pause_Active_013cae             | +040
        move.b  0x10e20d.l,d0                   | +044
        and.b   d7,d0                           | +04a
        beq.w   Pause_Poll_Reject_013caa        | +04c
        move.b  #0xff,0x10e272.l                | +050
        clr.b   0x10e273.l                      | +058
        move.w  #0x10e0,d0                      | +05e
        jsr     0x2352.l                        | +062

| ----------------------------------------------------------------------------
|  Pause_Poll_Reject_013caa  @ $013CAA  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Pause_Poll_Reject_013caa, "ax", @progbits
        .global Pause_Poll_Reject_013caa
Pause_Poll_Reject_013caa:
        bra.w   ClearC_013d0c                   | +000

| ----------------------------------------------------------------------------
|  Pause_Active_013cae  @ $013CAE  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Pause_Active_013cae, "ax", @progbits
        .global Pause_Active_013cae
Pause_Active_013cae:
        move.b  0x10e20d.l,d0                   | +000
        and.b   d7,d0                           | +006
        beq.w   .L013cdc                        | +008
        clr.b   0x10e272.l                      | +00c
        bsr.w   Fix_DrawPauseBlank_013d3e       | +012
        move.w  #0x10e0,d0                      | +016
        jsr     0x2222.l                        | +01a
        move.w  #0x10e1,d0                      | +020
        jsr     0x2352.l                        | +024
        bra.w   ClearC_013d0c                   | +02a
.L013cdc:
        move.b  0x10e273.l,d0                   | +02e
        addq.b  #0x1,0x10e273.l                 | +034
        move.b  d0,d1                           | +03a
        andi.b  #0x1f,d0                        | +03c
        bne.w   .L013cfa                        | +040
        bsr.w   Fix_DrawPause_013d46            | +044
        bra.w   SetC_013d06                     | +048
.L013cfa:
        cmpi.b  #0x18,d0                        | +04c
        bne.w   SetC_013d06                     | +050
        bsr.w   Fix_DrawPauseBlank_013d3e       | +054

| ----------------------------------------------------------------------------
|  Pause_Clear_013d12  @ $013D12  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Pause_Clear_013d12, "ax", @progbits
        .global Pause_Clear_013d12
Pause_Clear_013d12:
        clr.b   0x10e272.l                      | +000
