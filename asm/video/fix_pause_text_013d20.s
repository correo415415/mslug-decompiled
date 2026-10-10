| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave TTT — Texto "PAUSE" del fix layer ($013D20..$013D6A)
|  Región: $013D20..$013D6A  (66 B, 5 entradas, 2 huecos)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  Isla de 74 B entre los helpers C ClrRamWord_013d18/013d2a:
|   * SetRamByteFF_013d20: `move.b #$FF,$10E274; rts` (flag de pausa ON;
|     ClrRamWord_013d18/013d2a lo apagan).
|   * Fix_Str_PAUSE_013d32 = "PAUSE\0", Fix_Str_Blank_013d38 = "     \0".
|   * Fix_DrawPauseBlank_013d3e (a2 = blank) / Fix_DrawPause_013d46 (a2 =
|     "PAUSE") -> Fix_DrawPause_Common: VRAM addr $725C (fix layer, columna
|     ~14 fila 28), autoinc $20 por carácter (`addi.l #$200000` sobre el
|     long {addr,tile}), tile = $2300 | ASCII; escribe por $3C0000 (REG_VRAMADDR
|     + REG_VRAMRW en un solo move.l). Termina en el NUL.
|
|  B) EVIDENCIAS: cadena ASCII literal "PAUSE" en ROM; patrón idéntico a
|     Fix_DrawGlyphList_09820e (Wave QQQ).
|  C) HIPÓTESIS: $10E274 = estado de pausa (byte); ambos drawers son los
|     handlers de parpadeo del texto PAUSE.
|  D) ESTRUCTURAS: strings NUL-terminated de 6 B, en .text.
|  E) ISLAS: ninguna absorbida.  F) ESTADO: 5/5 byte-exactas.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  SetRamByteFF_013d20  @ $013D20  (10 B)
| ----------------------------------------------------------------------------
        .section .text.SetRamByteFF_013d20, "ax", @progbits
        .global SetRamByteFF_013d20
SetRamByteFF_013d20:
        move.b  #0xff,0x10e274.l                | +000
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Fix_Str_PAUSE_013d32  @ $013D32  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_Str_PAUSE_013d32, "ax", @progbits
        .global Fix_Str_PAUSE_013d32
Fix_Str_PAUSE_013d32:
        .dc.w   0x5041                        | +000  (dato / opcode no decodificado)
        .dc.w   0x5553                        | +002  (dato / opcode no decodificado)
        .dc.w   0x4500                        | +004  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Fix_Str_Blank_013d38  @ $013D38  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_Str_Blank_013d38, "ax", @progbits
        .global Fix_Str_Blank_013d38
Fix_Str_Blank_013d38:
        .dc.w   0x2020                        | +000  (dato / opcode no decodificado)
        .dc.w   0x2020                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2000                        | +004  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Fix_DrawPauseBlank_013d3e  @ $013D3E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawPauseBlank_013d3e, "ax", @progbits
        .global Fix_DrawPauseBlank_013d3e
Fix_DrawPauseBlank_013d3e:
        lea     Fix_Str_Blank_013d38(pc),a2     | +000
        bra.w   Fix_DrawPause_Common_013d4a   | +004

| ----------------------------------------------------------------------------
|  Fix_DrawPause_013d46  @ $013D46  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_DrawPause_013d46, "ax", @progbits
        .global Fix_DrawPause_013d46
Fix_DrawPause_013d46:
        lea     Fix_Str_PAUSE_013d32(pc),a2     | +000
        .global Fix_DrawPause_Common_013d4a
Fix_DrawPause_Common_013d4a:
        move.w  #0x725c,d0                      | +004
        swap    d0                              | +008
        move.w  #0x2300,d0                      | +00a
.L013d54:
        move.b  (a2)+,d0                        | +00e
        beq.w   .L013d68                        | +010
        move.l  d0,0x3c0000.l                   | +014
        addi.l  #0x200000,d0                    | +01a
        bra.b   .L013d54                        | +020
.L013d68:
        rts                                     | +022
