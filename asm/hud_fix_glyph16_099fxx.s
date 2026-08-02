| ============================================================================
|  Metal Slug 1 - asm/hud_fix_glyph16_099fxx.s
|  ----------------------------------------------------------------------------
|  Wave SS#7..#12 - cluster de dibujado de "glifos 16x16" en el Fix Layer
|  ($099F3A..$09A0B3).  Seis funciones cortas que pintan bloques de 2x2
|  tiles de 8x8 directamente por el puerto VRAM del LSPC ($3C0000 = registro
|  de direccion, $3C0002 = registro de datos; movem.w d0-d1 escribe ambos
|  en un solo acceso - idioma ya visto en los blitters de Wave CC#2).
|
|  Patron comun del bloque 2x2 (tile codes de un sheet de 16 columnas):
|
|      (addr+0x00) <- tile+0        | columna izquierda, fila superior
|      (addr+0x20) <- tile+1        | columna izquierda, fila inferior
|      (addr+0x01) <- tile+16       | columna derecha,  fila superior
|      (addr+0x21) <- tile+17       | columna derecha,  fila inferior
|
|  (el fix map es column-major: +0x20 = misma columna, fila+1; +1 = columna
|  siguiente).  Tras pintar, d0 vuelve al addr original (subi #$21) - los
|  callers encadenan llamadas reutilizando d0.
|
|  Funciones (todas con a6 = entity/task de contexto):
|
|   SS#7  FixGlyph16_DrawCursorA_099F3A  ($099F3A, 76 B, 3 callers:
|         $099CE4/$099DB8/$099DDC)  Tile base $4B22 (palette 4, tile $B22)
|         en la direccion fix leida de la tabla a6->+0x80 indexada por el
|         word a6->+0x78: cursor de menu en posicion variable.
|
|   SS#8  FixGlyph16_DrawCursorB_099F86  ($099F86, 76 B, sin caller jsr
|         directo localizado - probablemente invocada via el mismo cluster
|         $099Cxx aun no matcheado)  Clon byte-a-byte de SS#7 salvo el tile
|         base $4B40: el glifo "alternativo" del cursor (estado
|         seleccionado/parpadeo).  9o par de clones no factorizados.
|
|   SS#9  FixGlyphRun_Draw2F61F0_099FD2  ($099FD2, 24 B)  Prepara
|         a2 = $2F61F0 + (a6->+0x72)*8 (records ROM de 8 B = 4 words de
|         tile codes), a1 = $72EB (direccion fix destino), d1 = 4 (numero
|         de glifos) y cae por fall-through en JsrAbsThunk_099fea
|         (jsr ThunkTarget_04784c = renderizador de tiras de glifos).
|
|   SS#10 FixGlyph16_DrawDigit72EF_099FF2  ($099FF2, 74 B, 3 callers:
|         $099CDC/$099E30/$099E52)  Direccion fix FIJA $72EF, tile
|         $4B60 + 2*(byte a6->+0x73): digito/contador en celda fija.
|
|   SS#11 FixGlyph16_DrawDigit72F3_09A03C  ($09A03C, 74 B, 2 callers:
|         $099E74/$099E96)  Gemela de SS#10 en $72F3 con byte a6->+0x74.
|         (10o par de clones no factorizados.)
|
|   SS#12 FixGlyphRun_DrawPad2P_09A086  ($09A086, 46 B)  Solo si el byte
|         BIOS $10FD83 == 2 (dos creditos/jugadores activos, hipotesis):
|         elige la tira ROM $2F6226 (si a6->+0x76 == 1) o $2F621A y cae en
|         JsrAbsThunk_09a0b4 (jsr ThunkTarget_0477fc) con a1 = $72F3,
|         d1 = 4.  Si no, SALTA AL RTS INTERNO del propio thunk matcheado
|         (bne.w $9A0BA) - variante nueva del idioma "fall-through a thunk
|         matcheado": no cae sobre su inicio sino sobre su ultimo opcode.
|
|  Contexto: el cluster $099Cxx..$09A0xx es la pantalla de seleccion /
|  HUD del attract (Entity_TrailRecord_099812 y Attract_InitSingle_099AE2
|  ya matcheados son vecinos).  Las celdas $72EB..$72F3 son consecutivas
|  en la fila del fix map: forman un marcador "XXXX NN NN" (texto de 4
|  glifos + dos contadores de 1 glifo).
|
|  Toolchain:  m68k-linux-gnu-as -m68000 --register-prefix-optional
|  ============================================================================

        .text

| ----------------------------------------------------------------------------
|  SS#7  FixGlyph16_DrawCursorA_099F3A  ($099F3A..$099F85, 76 B)
|  void FixGlyph16_DrawCursorA(Entity *e /*a6*/)
|      addr = ((u16*)e->+0x80)[e->+0x78];  tile = $4B22
| ----------------------------------------------------------------------------
        .globl  FixGlyph16_DrawCursorA_099F3A
        .type   FixGlyph16_DrawCursorA_099F3A, @function
        .section .text.FixGlyph16_DrawCursorA_099F3A, "ax", @progbits

FixGlyph16_DrawCursorA_099F3A:
        movea.l 0x80(a6), a1                    | +000  a1 = tabla de posiciones
        move.w  0x78(a6), d2                    | +004  d2 = indice cursor
        lsl.w   #1, d2                          | +008  *2 (tabla de words)
        move.w  (a1,d2.w), d0                   | +00a  d0 = addr fix destino
        move.w  #0x4b22, d1                     | +00e  d1 = tile cursor A
        movem.w d0-d1, 0x3c0000                 | +012  (addr) <- tile
        addi.w  #0x20, d0                       | +01a  fila inferior
        addq.w  #1, d1                          | +01e  tile+1
        movem.w d0-d1, 0x3c0000                 | +020  (addr+20) <- tile+1
        subi.w  #0x1f, d0                       | +028  columna derecha, fila sup
        addi.w  #0xf, d1                        | +02c  tile+16
        movem.w d0-d1, 0x3c0000                 | +030  (addr+1) <- tile+16
        addi.w  #0x20, d0                       | +038  fila inferior
        addq.w  #1, d1                          | +03c  tile+17
        movem.w d0-d1, 0x3c0000                 | +03e  (addr+21) <- tile+17
        subi.w  #0x21, d0                       | +046  d0 = addr original
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  SS#8  FixGlyph16_DrawCursorB_099F86  ($099F86..$099FD1, 76 B)
|  Clon de SS#7 con tile base $4B40 (glifo cursor "B").
| ----------------------------------------------------------------------------
        .globl  FixGlyph16_DrawCursorB_099F86
        .type   FixGlyph16_DrawCursorB_099F86, @function
        .section .text.FixGlyph16_DrawCursorB_099F86, "ax", @progbits

FixGlyph16_DrawCursorB_099F86:
        movea.l 0x80(a6), a1                    | +000
        move.w  0x78(a6), d2                    | +004
        lsl.w   #1, d2                          | +008
        move.w  (a1,d2.w), d0                   | +00a
        move.w  #0x4b40, d1                     | +00e  d1 = tile cursor B
        movem.w d0-d1, 0x3c0000                 | +012
        addi.w  #0x20, d0                       | +01a
        addq.w  #1, d1                          | +01e
        movem.w d0-d1, 0x3c0000                 | +020
        subi.w  #0x1f, d0                       | +028
        addi.w  #0xf, d1                        | +02c
        movem.w d0-d1, 0x3c0000                 | +030
        addi.w  #0x20, d0                       | +038
        addq.w  #1, d1                          | +03c
        movem.w d0-d1, 0x3c0000                 | +03e
        subi.w  #0x21, d0                       | +046
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  SS#9  FixGlyphRun_Draw2F61F0_099FD2  ($099FD2..$099FE9, 24 B)
|  Prepara args y cae en JsrAbsThunk_099fea (jsr ThunkTarget_04784c; rts).
| ----------------------------------------------------------------------------
        .globl  FixGlyphRun_Draw2F61F0_099FD2
        .type   FixGlyphRun_Draw2F61F0_099FD2, @function
        .section .text.FixGlyphRun_Draw2F61F0_099FD2, "ax", @progbits

FixGlyphRun_Draw2F61F0_099FD2:
        clr.l   d0                              | +000
        move.b  0x72(a6), d0                    | +002  d0 = indice de tira
        lsl.l   #3, d0                          | +006  *8 (records de 8 B)
        lea     0x2f61f0.l, a2                  | +008  a2 = base tiras ROM
        adda.l  d0, a2                          | +00e  a2 = tira elegida
        movea.w #0x72eb, a1                     | +010  a1 = addr fix destino
        move.w  #4, d1                          | +014  d1 = 4 glifos
        | fall-through en JsrAbsThunk_099fea ($099FEA, Wave I)         | +018

| ----------------------------------------------------------------------------
|  SS#10  FixGlyph16_DrawDigit72EF_099FF2  ($099FF2..$09A03B, 74 B)
|  Addr fija $72EF, tile $4B60 + 2*(a6->+0x73).
| ----------------------------------------------------------------------------
        .globl  FixGlyph16_DrawDigit72EF_099FF2
        .type   FixGlyph16_DrawDigit72EF_099FF2, @function
        .section .text.FixGlyph16_DrawDigit72EF_099FF2, "ax", @progbits

FixGlyph16_DrawDigit72EF_099FF2:
        move.w  #0x72ef, d0                     | +000  d0 = addr fix fija
        clr.w   d1                              | +004
        move.b  0x73(a6), d1                    | +006  d1 = valor del digito
        lsl.w   #1, d1                          | +00a  *2 (2 columnas/glifo)
        addi.w  #0x4b60, d1                     | +00c  tile base digitos
        movem.w d0-d1, 0x3c0000                 | +010
        addi.w  #0x20, d0                       | +018
        addq.w  #1, d1                          | +01c
        movem.w d0-d1, 0x3c0000                 | +01e
        subi.w  #0x1f, d0                       | +026
        addi.w  #0xf, d1                        | +02a
        movem.w d0-d1, 0x3c0000                 | +02e
        addi.w  #0x20, d0                       | +036
        addq.w  #1, d1                          | +03a
        movem.w d0-d1, 0x3c0000                 | +03c
        subi.w  #0x21, d0                       | +044
        rts                                     | +048

| ----------------------------------------------------------------------------
|  SS#11  FixGlyph16_DrawDigit72F3_09A03C  ($09A03C..$09A085, 74 B)
|  Gemela de SS#10: addr $72F3, byte a6->+0x74.
| ----------------------------------------------------------------------------
        .globl  FixGlyph16_DrawDigit72F3_09A03C
        .type   FixGlyph16_DrawDigit72F3_09A03C, @function
        .section .text.FixGlyph16_DrawDigit72F3_09A03C, "ax", @progbits

FixGlyph16_DrawDigit72F3_09A03C:
        move.w  #0x72f3, d0                     | +000
        clr.w   d1                              | +004
        move.b  0x74(a6), d1                    | +006
        lsl.w   #1, d1                          | +00a
        addi.w  #0x4b60, d1                     | +00c
        movem.w d0-d1, 0x3c0000                 | +010
        addi.w  #0x20, d0                       | +018
        addq.w  #1, d1                          | +01c
        movem.w d0-d1, 0x3c0000                 | +01e
        subi.w  #0x1f, d0                       | +026
        addi.w  #0xf, d1                        | +02a
        movem.w d0-d1, 0x3c0000                 | +02e
        addi.w  #0x20, d0                       | +036
        addq.w  #1, d1                          | +03a
        movem.w d0-d1, 0x3c0000                 | +03c
        subi.w  #0x21, d0                       | +044
        rts                                     | +048

| ----------------------------------------------------------------------------
|  SS#12  FixGlyphRun_DrawPad2P_09A086  ($09A086..$09A0B3, 46 B)
|  Solo si $10FD83 == 2; elige tira por a6->+0x76 y cae en
|  JsrAbsThunk_09a0b4.  La salida temprana salta a JsrAbsRts_09a0ba,
|  el rts INTERNO de ese mismo thunk matcheado.
| ----------------------------------------------------------------------------
        .globl  FixGlyphRun_DrawPad2P_09A086
        .type   FixGlyphRun_DrawPad2P_09A086, @function
        .section .text.FixGlyphRun_DrawPad2P_09A086, "ax", @progbits

FixGlyphRun_DrawPad2P_09A086:
        cmpi.b  #2, 0x10fd83.l                  | +000  modo 2 activo ?
        bne.w   JsrAbsRts_09a0ba                | +008  no: rts del thunk vecino
        cmpi.b  #1, 0x76(a6)                    | +00c  variante de tira ?
        bne.w   .Lother                         | +012
        lea     0x2f6226.l, a2                  | +016  tira "variante 1"
        bra.w   .Ljoin                          | +01c
.Lother:
        lea     0x2f621a.l, a2                  | +020  tira "variante 0"
.Ljoin:
        movea.w #0x72f3, a1                     | +026  a1 = addr fix destino
        move.w  #4, d1                          | +02a  d1 = 4 glifos
        | fall-through en JsrAbsThunk_09a0b4 ($09A0B4, Wave I)         | +02e
