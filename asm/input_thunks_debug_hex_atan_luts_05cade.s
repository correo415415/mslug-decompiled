| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave GGGGG — thunks de input, HUD hexadecimal de debug, LUTs de atan,
|  comprobaciones de pantalla  (asm/input_thunks_debug_hex_atan_luts_05cade.s)
|  Región: $05CADE..$05E000  (2,320 B, 55 entradas, 22 huecos)
| ============================================================================
|
|  A. QUÉ HAY AQUÍ
|  Último tramo del "runtime medio" antes de late_props_turrets_05exxx.s.
|  Cierra los 22 huecos que quedaban entre las islas C (SetXN/ClearXN,
|  SetC, SetTaskW, SetTaskHandler, Jsr/JmpThunk) y los módulos ya cerrados
|  input_evt_thunks.s, input_aim_tables_05d316.s, sprite_hex_format4/8,
|  long_divide_05d920.s, fix_blit_rect.s, fix_layer_backends_05dbxx.s y
|  camera_list_ctx_helpers_wave_ii.s:
|
|  1) $05CADE..$05CC0E  Restos del scheduler/bootstrap:
|     - Nop_Rts_05cade: rts suelto (relleno tras Camera_ResetCenter_05CACE).
|     - Scheduler_CompareField10_05cae0 / _05cbec: comparan `$10(a6)` con
|       `$10(a1)` de la tarea enlazada `$8(a6)` y caen en SetXN_* (C).
|     - SpriteBlock20x14_Setup_05cafc / _SetupDup_05cb68: reservan un bloque
|       de sprites con Spawn_TypeB_013952 (d0=$14), programan `$139FE`
|       (20 sprites, 14 filas, d6=d0+$13), escriben la cabecera en LSPC
|       (`$3C0000`/`$3C0002`, latch `$106EE4`): shrink $F78E en +$8201 y
|       posición 0 en +$8401; _SetupDup repite el shrink (bug/duplicado en
|       el original) y acaba en `lea $2B7460; jsr $2A7C` (carga de mapa de
|       tiles). _Setup salta al epílogo de _SetupDup.
|     - Pad_Zero_05cc08: 6 B de ceros.
|
|  2) $05CCC8..$05D310  Thunks de eventos de input (misma familia que
|     input_evt_thunks.s; backends InputMask_CheckChannelAvail_05cfa8 e
|     InputMask_TestChannelBit_05cff8):
|     - Entity_CopyField6D_05ccc8: copia `$6D(a6)` → `$6D(a0)`.
|     - InputEvtThunk_05ccd0..05cd4e (8): máscara $10/$20/$40/$80, canal 2,
|       contexto `$10E200` (jugador 1) o `$10E206` (jugador 2) →
|       TestChannelBit.
|     - InputEvtThunk_05cd60..05cdf0 (13): máscara $10..$F0/$E0/$30/$50/$70,
|       canal 2 ó 3 → CheckChannelAvail.
|     - InputEvt_Thunk4aThenMask60_05d1da / Mask40_05d210: llaman a
|       InputEvtThunk_05ce4a y si C=1 testean la máscara $60/$40 en
|       `$10E206+2` → SetXN/ClearXN.
|     - InputEvt_Thunk4aThenToggle_05d240 y InputEvt_ToggleChain_05d288:
|       cadena de tests (máscaras $08/$01/$04 canal 0/1, bit $80 canal 3)
|       que comparan con el byte de `$10E20C+canal` (estado previo) con
|       eor.b; devuelven d0=0/-1 (flanco). Con 3 puntos de entrada
|       intermedios alcanzados por jsr desde los thunks.
|     - InputEvt_ThunkMaskF0_05d302: InputEvtThunk_05cd90 → ClearXN si C=0.
|
|  3) $05D6B0..$05D8F2  HUD hexadecimal de debug (Debug_DrawHUDVars_096A80):
|     - Sprite_HexFormat8_Prologue_05d6b0: prepara a2=`$10E21E`, d2=28,
|       d3=8 y entra en el bucle de Sprite_HexFormat4_05D6C2 (label
|       promovido __L05d6d0).
|     - HEX_TABLE_5D71C: "0123456789ABCDEF".
|     - Debug_HexDrawToFix8_05d7be / _ToFix4_05d7d8 (entrada forzada):
|       extraen 8/4 nibbles de d0 al buffer `$10E21E`, luego por cada
|       nibble eligen tile de HexDigit_FixTileTable_05d864 (17 words: 0-9,
|       A-F y tile vacío $0B80 para ceros a la izquierda) y lo pintan con
|       Fix_BlitRectToFixLayer (2x2) avanzando a1 += $40 (una columna FIX).
|     - Bin16_ToBcd4_05d886 (entrada forzada): convierte d0 (0..9999) en
|       4 dígitos BCD empaquetados por divu.w #10 encadenados.
|
|  4) $05D944..$05DA56  Trap y ruido:
|     - Trap15_DivByZero_05D944: trap #15 (brazo d1==0 de Sub_LongDivide).
|     - Noise_LookupByIndex_05d946 (entrada forzada): d0 = byte de
|       NoiseLut256_05d956[`$10E22E` & $FF]; NoiseLut256 son 256 B
|       pseudoaleatorios (tabla de ruido/jitter).
|
|  5) $05DB6A..$05DE12  Helpers de entidad y cursor de lista:
|     - ListCursor_Step_05db6a / _LoadEntry_05dba6: avanzan el cursor
|       `$3C(a6)` sobre una lista de entradas de 8 B {tipo, ptr, dur};
|       $FFFF = fin (SetC), tipo 0 = cabecera (InstallListPubHead_05DB58),
|       si no carga la duración en `$46(a6)` y reinicia el contexto.
|     - Entity_NegIfFacing_05dca4 / Entity_SetVx_NegIfFacing_05dcb6: niegan
|       d0 / `$28(a6)` según el bit 0 de `$3A(a6)` (orientación) y caen en
|       SetTaskW_* (C).
|     - Spawn_ChildFromDesc_05dcce: según `(a1)`==2 usa Spawn_MarkPending
|       (pool de jefes) o Task_AllocFromFreeList sobre `$1008A0`; guarda el
|       hijo en `$3C(a0)`.
|     - Handler_ApplyCameraSelf_CopyTransform_05dd22: jsr $440E4 + bra
|       Entity_CopyTransform.
|     - ScreenBox_Default_05dd4c = {0,1,0,1,$FFFF} (márgenes por defecto).
|     - Entity_SetOffscreenFlag_05dd56: bset #7,`$13(a6)`.
|     - Entity_CheckOnScreenBox_05dd5c: con a0 = caja (o la default si
|       a0=-1), si el bit 7 de `$13(a6)` está puesto comprueba que
|       X `$22(a6)` ∈ [0+box0, $140+box1] e Y `$24(a6)` ∈ [$100-box3,
|       $1F0-box2]; fuera → SetC. Sin bit 7 → Entity_CheckEnterScreen_05ddbe
|       (pone el bit cuando entra en (0,$140)x($100,$1F0)) →
|       Entity_CheckLeaveScreenWide_05ddf2 (lo quita fuera de
|       (-$80,$200)x(0,$280)).
|
|  6) $05DE18..$05E000  LUTs de Atan2_Angle256_05e018:
|     - AtanLog_Table_05de18 (256 B): log-ratio por cociente menor/mayor.
|     - AtanExp_Table_05df18 (232 B + cola AtanTable_Tail_05e000): mapea la
|       diferencia de logs a ángulo 0..$20 (octante).
|
|  B. CÓMO SE DESCUBRIÓ
|  Huecos de measure_coverage en $05CADE..$05E000. Islas de datos por
|  `lea X(pc)` (HEX_TABLE, FixTileTable, ScreenBox, LUTs) y por opcodes
|  inválidos (NoiseLut256). Las entradas forzadas son destinos de jsr
|  externos que no caen en el flujo lineal (Debug_HexDrawToFix4 tiene un
|  prólogo alternativo; Bin16_ToBcd4 sigue a la tabla; Noise_Lookup sigue
|  al trap). Nombres por backends/contextos ya identificados en
|  input_evt_thunks.s y por el caller Debug_DrawHUDVars_096A80.
|
|  C. DEPENDENCIAS EXTERNAS
|  Spawn_TypeB_013952, `$139FE`, `$2A7C`, Fix_BlitRectToFixLayer,
|  Sprite_HexFormat4_05D6C2__L05d6d0, InputEvtThunk_05ce4a,
|  InputMask_CheckChannelAvail_05cfa8, InputMask_TestChannelBit_05cff8,
|  ListCursor_Reinit_05DBC2, ListCursor_ReinitClipped_05DBDC,
|  InstallListPubHead_05DB58, Task_AllocFromFreeList, Spawn_MarkPending_04498E,
|  Handler_ApplyCameraSelf_0440E4, Entity_CopyTransform, islas C SetXN/
|  ClearXN/ClearXNV/SetC/SetTaskW. RAM: `$10E200/$10E206` (contextos de
|  input P1/P2), `$10E20C` (estado previo), `$10E21E` (buffer hex),
|  `$10E22E` (índice de ruido), `$106EE4` (latch LSPC).
|
|  D. ESTADO
|  55/55 entradas byte-exactas. Zona $05E000 ahora contigua con
|  late_props_turrets_05exxx.s. Pendiente: nombres para `$139FE`/`$2A7C`.
|
|  E. NOTAS
|  SpriteBlock20x14_SetupDup repite `move.w #$F78E,$3C0002` dos veces; es
|  byte-exacto con la ROM (duplicado en el original).
|
|  F. VERIFICACIÓN
|  Cada sección .text.<Sym> se coloca en su dirección CPU absoluta y
|  reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Nop_Rts_05cade  @ $05CADE  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Nop_Rts_05cade, "ax", @progbits
        .global Nop_Rts_05cade
Nop_Rts_05cade:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_05cae0  @ $05CAE0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_05cae0, "ax", @progbits
        .global Scheduler_CompareField10_05cae0
Scheduler_CompareField10_05cae0:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05caf6                    | +00c

| ----------------------------------------------------------------------------
|  SpriteBlock20x14_Setup_05cafc  @ $05CAFC  (108 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteBlock20x14_Setup_05cafc, "ax", @progbits
        .global SpriteBlock20x14_Setup_05cafc
SpriteBlock20x14_Setup_05cafc:
        move.w  #0x14,d0                        | +000
        jsr     0x13952.l                       | +004
        moveq   #0,d2                           | +00a
        move.w  #0xe,d3                         | +00c
        move.w  #0xe,d4                         | +010
        swap    d4                              | +014
        move.w  #0x14,d4                        | +016
        move.w  d0,d5                           | +01a
        move.w  d0,d6                           | +01c
        addi.w  #0x13,d6                        | +01e
        move.w  #0x0,d1                         | +022
        move.b  #0x0,d7                         | +026
        movem.w d0,-(a7)                        | +02a
        jsr     0x139fe.l                       | +02e
        movem.w (a7)+,d0                        | +034
        addi.w  #0x8201,d0                      | +038
        move.w  d0,0x106ee4.l                   | +03c
        move.w  d0,0x3c0000.l                   | +042
        move.w  #0xf78e,0x3c0002.l              | +048
        addi.w  #0x200,d0                       | +050
        move.w  d0,0x106ee4.l                   | +054
        move.w  d0,0x3c0000.l                   | +05a
        move.w  #0x0,0x3c0002.l                 | +060
        jmp     SpriteBlock20x14_SetupDup_05cb68__L05cbd8(pc) | +068

| ----------------------------------------------------------------------------
|  SpriteBlock20x14_SetupDup_05cb68  @ $05CB68  (124 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteBlock20x14_SetupDup_05cb68, "ax", @progbits
        .global SpriteBlock20x14_SetupDup_05cb68
SpriteBlock20x14_SetupDup_05cb68:
        move.w  #0x14,d0                        | +000
        jsr     0x13952.l                       | +004
        moveq   #0,d2                           | +00a
        move.w  #0xe,d3                         | +00c
        move.w  #0xe,d4                         | +010
        swap    d4                              | +014
        move.w  #0x14,d4                        | +016
        move.w  d0,d5                           | +01a
        move.w  d0,d6                           | +01c
        addi.w  #0x13,d6                        | +01e
        move.w  #0x0,d1                         | +022
        move.b  #0x0,d7                         | +026
        movem.w d0,-(a7)                        | +02a
        jsr     0x139fe.l                       | +02e
        movem.w (a7)+,d0                        | +034
        addi.w  #0x8201,d0                      | +038
        move.w  d0,0x106ee4.l                   | +03c
        move.w  d0,0x3c0000.l                   | +042
        move.w  #0xf78e,0x3c0002.l              | +048
        move.w  #0xf78e,0x3c0002.l              | +050
        addi.w  #0x200,d0                       | +058
        move.w  d0,0x106ee4.l                   | +05c
        move.w  d0,0x3c0000.l                   | +062
        move.w  #0x0,0x3c0002.l                 | +068
        .global SpriteBlock20x14_SetupDup_05cb68__L05cbd8
SpriteBlock20x14_SetupDup_05cb68__L05cbd8:
.L05cbd8:
        lea     0x2b7460.l,a0                   | +070
        jsr     0x2a7c.l                        | +076

| ----------------------------------------------------------------------------
|  Scheduler_CompareField10_05cbec  @ $05CBEC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Scheduler_CompareField10_05cbec, "ax", @progbits
        .global Scheduler_CompareField10_05cbec
Scheduler_CompareField10_05cbec:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_05cc02                    | +00c

| ----------------------------------------------------------------------------
|  Pad_Zero_05cc08  @ $05CC08  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Pad_Zero_05cc08, "ax", @progbits
        .global Pad_Zero_05cc08
Pad_Zero_05cc08:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x00                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x00                          | +005  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Entity_CopyField6D_05ccc8  @ $05CCC8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CopyField6D_05ccc8, "ax", @progbits
        .global Entity_CopyField6D_05ccc8
Entity_CopyField6D_05ccc8:
        move.b  0x6d(a6),0x6d(a0)               | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  InputEvtThunk_05ccd0  @ $05CCD0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05ccd0, "ax", @progbits
        .global InputEvtThunk_05ccd0
InputEvtThunk_05ccd0:
        move.b  #0x10,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e200.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cce2  @ $05CCE2  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cce2, "ax", @progbits
        .global InputEvtThunk_05cce2
InputEvtThunk_05cce2:
        move.b  #0x20,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e200.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05ccf4  @ $05CCF4  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05ccf4, "ax", @progbits
        .global InputEvtThunk_05ccf4
InputEvtThunk_05ccf4:
        move.b  #0x40,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e200.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd06  @ $05CD06  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd06, "ax", @progbits
        .global InputEvtThunk_05cd06
InputEvtThunk_05cd06:
        move.b  #0x80,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e200.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd18  @ $05CD18  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd18, "ax", @progbits
        .global InputEvtThunk_05cd18
InputEvtThunk_05cd18:
        move.b  #0x10,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e206.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd2a  @ $05CD2A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd2a, "ax", @progbits
        .global InputEvtThunk_05cd2a
InputEvtThunk_05cd2a:
        move.b  #0x20,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e206.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd3c  @ $05CD3C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd3c, "ax", @progbits
        .global InputEvtThunk_05cd3c
InputEvtThunk_05cd3c:
        move.b  #0x40,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e206.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd4e  @ $05CD4E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd4e, "ax", @progbits
        .global InputEvtThunk_05cd4e
InputEvtThunk_05cd4e:
        move.b  #0x80,d1                        | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e206.l,a2                   | +008
        bra.w   InputMask_TestChannelBit_05cff8 | +00e

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd60  @ $05CD60  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd60, "ax", @progbits
        .global InputEvtThunk_05cd60
InputEvtThunk_05cd60:
        move.b  #0x10,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd6c  @ $05CD6C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd6c, "ax", @progbits
        .global InputEvtThunk_05cd6c
InputEvtThunk_05cd6c:
        move.b  #0x20,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd78  @ $05CD78  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd78, "ax", @progbits
        .global InputEvtThunk_05cd78
InputEvtThunk_05cd78:
        move.b  #0x40,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd84  @ $05CD84  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd84, "ax", @progbits
        .global InputEvtThunk_05cd84
InputEvtThunk_05cd84:
        move.b  #0x80,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd90  @ $05CD90  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd90, "ax", @progbits
        .global InputEvtThunk_05cd90
InputEvtThunk_05cd90:
        move.b  #0xf0,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cd9c  @ $05CD9C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cd9c, "ax", @progbits
        .global InputEvtThunk_05cd9c
InputEvtThunk_05cd9c:
        move.b  #0xe0,d1                        | +000
        move.w  #0x2,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cda8  @ $05CDA8  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cda8, "ax", @progbits
        .global InputEvtThunk_05cda8
InputEvtThunk_05cda8:
        move.b  #0x10,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cdb4  @ $05CDB4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cdb4, "ax", @progbits
        .global InputEvtThunk_05cdb4
InputEvtThunk_05cdb4:
        move.b  #0x20,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cdc0  @ $05CDC0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cdc0, "ax", @progbits
        .global InputEvtThunk_05cdc0
InputEvtThunk_05cdc0:
        move.b  #0x40,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cdcc  @ $05CDCC  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cdcc, "ax", @progbits
        .global InputEvtThunk_05cdcc
InputEvtThunk_05cdcc:
        move.b  #0x80,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cdd8  @ $05CDD8  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cdd8, "ax", @progbits
        .global InputEvtThunk_05cdd8
InputEvtThunk_05cdd8:
        move.b  #0x30,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cde4  @ $05CDE4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cde4, "ax", @progbits
        .global InputEvtThunk_05cde4
InputEvtThunk_05cde4:
        move.b  #0x50,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvtThunk_05cdf0  @ $05CDF0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvtThunk_05cdf0, "ax", @progbits
        .global InputEvtThunk_05cdf0
InputEvtThunk_05cdf0:
        move.b  #0x70,d1                        | +000
        move.w  #0x3,d0                         | +004
        bra.w   InputMask_CheckChannelAvail_05cfa8 | +008

| ----------------------------------------------------------------------------
|  InputEvt_Thunk4aThenMask60_05d1da  @ $05D1DA  (42 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvt_Thunk4aThenMask60_05d1da, "ax", @progbits
        .global InputEvt_Thunk4aThenMask60_05d1da
InputEvt_Thunk4aThenMask60_05d1da:
        jsr     InputEvtThunk_05ce4a(pc)        | +000
        bcc.w   ClearXN_05d204                  | +004
        move.b  #0x20,d1                        | +008
        ori.b   #0x40,d1                        | +00c
        move.w  #0x2,d0                         | +010
        lea     0x10e206.l,a2                   | +014
        bra.w   .L05d1f8                        | +01a
.L05d1f8:
        move.b  (a2,d0.w),d0                    | +01e
        and.b   d1,d0                           | +022
        cmp.b   d1,d0                           | +024
        beq.w   SetXN_05d20a                    | +026

| ----------------------------------------------------------------------------
|  InputEvt_Thunk4aThenMask40_05d210  @ $05D210  (36 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvt_Thunk4aThenMask40_05d210, "ax", @progbits
        .global InputEvt_Thunk4aThenMask40_05d210
InputEvt_Thunk4aThenMask40_05d210:
        jsr     InputEvtThunk_05ce4a(pc)        | +000
        bcc.b   ClearXN_05d204                  | +004
        move.b  #0x40,d1                        | +006
        move.w  #0x2,d0                         | +00a
        lea     0x10e206.l,a2                   | +00e
        bra.w   .L05d228                        | +014
.L05d228:
        move.b  (a2,d0.w),d0                    | +018
        and.b   d1,d0                           | +01c
        cmp.b   d1,d0                           | +01e
        beq.w   SetXN_05d23a                    | +020

| ----------------------------------------------------------------------------
|  InputEvt_Thunk4aThenToggle_05d240  @ $05D240  (72 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvt_Thunk4aThenToggle_05d240, "ax", @progbits
        .global InputEvt_Thunk4aThenToggle_05d240
InputEvt_Thunk4aThenToggle_05d240:
        jsr     InputEvtThunk_05ce4a(pc)        | +000
        bcs.w   .L05d24c                        | +004
        bra.w   InputEvt_ToggleChain_05d288__L05d2ee | +008
.L05d24c:
        move.b  #0x8,d1                         | +00c
        move.w  #0x0,d0                         | +010
        bra.w   InputEvt_ToggleChain_05d288__L05d2de | +014
        jsr     InputEvtThunk_05cd06(pc)        | +018
        bcs.w   .L05d264                        | +01c
        bra.w   InputEvt_ToggleChain_05d288__L05d2ee | +020
.L05d264:
        move.b  #0x1,d1                         | +024
        move.w  #0x0,d0                         | +028
        bra.w   InputEvt_ToggleChain_05d288__L05d2de | +02c
        jsr     InputEvtThunk_05cd06(pc)        | +030
        bcs.w   .L05d27c                        | +034
        bra.w   InputEvt_ToggleChain_05d288__L05d2ee | +038
.L05d27c:
        move.b  #0x4,d1                         | +03c
        move.w  #0x0,d0                         | +040
        bra.w   InputEvt_ToggleChain_05d288__L05d2de | +044

| ----------------------------------------------------------------------------
|  InputEvt_ToggleChain_05d288  @ $05D288  (122 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvt_ToggleChain_05d288, "ax", @progbits
        .global InputEvt_ToggleChain_05d288
InputEvt_ToggleChain_05d288:
        move.b  #0x1,d1                         | +000
        move.w  #0x2,d0                         | +004
        lea     0x10e200.l,a2                   | +008
        jsr     InputMask_TestChannelBit_05cff8(pc) | +00e
        bcs.w   .L05d2a2                        | +012
        bra.w   .L05d2ee                        | +016
.L05d2a2:
        move.b  #0x80,d1                        | +01a
        move.w  #0x3,d0                         | +01e
        lea     0x10e200.l,a2                   | +022
        bra.w   InputMask_TestChannelBit_05cff8 | +028
        jsr     InputEvtThunk_05cd06(pc)        | +02c
        bcs.w   .L05d2c0                        | +030
        bra.w   .L05d2ee                        | +034
.L05d2c0:
        move.b  #0x80,d1                        | +038
        move.w  #0x3,d0                         | +03c
        lea     0x10e200.l,a2                   | +040
        bra.w   InputMask_TestChannelBit_05cff8 | +046
        move.b  #0x4,d1                         | +04a
        move.w  #0x1,d0                         | +04e
        bra.w   .L05d2de                        | +052
        .global InputEvt_ToggleChain_05d288__L05d2de
InputEvt_ToggleChain_05d288__L05d2de:
.L05d2de:
        lea     0x10e20c.l,a2                   | +056
        move.b  (a2,d0.w),d0                    | +05c
        eor.b   d1,d0                           | +060
        beq.w   .L05d2f8                        | +062
        .global InputEvt_ToggleChain_05d288__L05d2ee
InputEvt_ToggleChain_05d288__L05d2ee:
.L05d2ee:
        move.b  #0x1,d0                         | +066
        subi.b  #0x1,d0                         | +06a
        rts                                     | +06e
.L05d2f8:
        move.b  #0x0,d0                         | +070
        subi.b  #0x1,d0                         | +074
        rts                                     | +078

| ----------------------------------------------------------------------------
|  InputEvt_ThunkMaskF0_05d302  @ $05D302  (8 B)
| ----------------------------------------------------------------------------
        .section .text.InputEvt_ThunkMaskF0_05d302, "ax", @progbits
        .global InputEvt_ThunkMaskF0_05d302
InputEvt_ThunkMaskF0_05d302:
        jsr     InputEvtThunk_05cd90(pc)        | +000
        bcc.w   ClearXN_05d310                  | +004

| ----------------------------------------------------------------------------
|  Sprite_HexFormat8_Prologue_05d6b0  @ $05D6B0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sprite_HexFormat8_Prologue_05d6b0, "ax", @progbits
        .global Sprite_HexFormat8_Prologue_05d6b0
Sprite_HexFormat8_Prologue_05d6b0:
        lea     0x10e21e.l,a2                   | +000
        moveq   #28,d2                          | +006
        move.w  #0x8,d3                         | +008
        move.w  d3,-(a7)                        | +00c
        jmp     Sprite_HexFormat4_05D6C2__L05d6d0(pc) | +00e

| ----------------------------------------------------------------------------
|  HEX_TABLE_5D71C  @ $05D71C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HEX_TABLE_5D71C, "ax", @progbits
        .global HEX_TABLE_5D71C
HEX_TABLE_5D71C:
        .dc.b   0x30                          | +000  '0'  (dato, rango --data)
        .dc.b   0x31                          | +001  '1'  (dato, rango --data)
        .dc.b   0x32                          | +002  '2'  (dato, rango --data)
        .dc.b   0x33                          | +003  '3'  (dato, rango --data)
        .dc.b   0x34                          | +004  '4'  (dato, rango --data)
        .dc.b   0x35                          | +005  '5'  (dato, rango --data)
        .dc.b   0x36                          | +006  '6'  (dato, rango --data)
        .dc.b   0x37                          | +007  '7'  (dato, rango --data)
        .dc.b   0x38                          | +008  '8'  (dato, rango --data)
        .dc.b   0x39                          | +009  '9'  (dato, rango --data)
        .dc.b   0x41                          | +00a  'A'  (dato, rango --data)
        .dc.b   0x42                          | +00b  'B'  (dato, rango --data)
        .dc.b   0x43                          | +00c  'C'  (dato, rango --data)
        .dc.b   0x44                          | +00d  'D'  (dato, rango --data)
        .dc.b   0x45                          | +00e  'E'  (dato, rango --data)
        .dc.b   0x46                          | +00f  'F'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Debug_HexDrawToFix8_05d7be  @ $05D7BE  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Debug_HexDrawToFix8_05d7be, "ax", @progbits
        .global Debug_HexDrawToFix8_05d7be
Debug_HexDrawToFix8_05d7be:
        swap    d1                              | +000
        lsr.l   #0x4,d1                         | +002
        andi.w  #0xf000,d1                      | +004
        lea     0x10e21e.l,a2                   | +008
        moveq   #28,d2                          | +00e
        move.w  #0x8,d3                         | +010
        move.w  d3,-(a7)                        | +014
        bra.w   Debug_HexDrawToFix4_05d7d8__L05d7ee | +016

| ----------------------------------------------------------------------------
|  Debug_HexDrawToFix4_05d7d8  @ $05D7D8  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Debug_HexDrawToFix4_05d7d8, "ax", @progbits
        .global Debug_HexDrawToFix4_05d7d8
Debug_HexDrawToFix4_05d7d8:
        swap    d1                              | +000
        lsr.l   #0x4,d1                         | +002
        andi.w  #0xf000,d1                      | +004
        lea     0x10e21e.l,a2                   | +008
        moveq   #12,d2                          | +00e
        move.w  #0x4,d3                         | +010
        move.w  d3,-(a7)                        | +014
        .global Debug_HexDrawToFix4_05d7d8__L05d7ee
Debug_HexDrawToFix4_05d7d8__L05d7ee:
.L05d7ee:
        move.l  d2,-(a7)                        | +016
        move.l  d0,d3                           | +018
        lsr.l   d2,d3                           | +01a
        andi.l  #0xf,d3                         | +01c
        move.w  d3,(a2)                         | +022
        addq.l  #0x2,a2                         | +024
        move.l  (a7)+,d2                        | +026
        subq.l  #0x4,d2                         | +028
        bne.b   .L05d7ee                        | +02a
        andi.l  #0xf,d0                         | +02c
        move.w  d0,(a2)                         | +032
        move.w  (a7)+,d3                        | +034
        subq.w  #0x1,d3                         | +036
        clr.w   d2                              | +038
        lea     0x10e21e.l,a2                   | +03a
        lea     HexDigit_FixTileTable_05d864(pc),a3 | +040
.L05d81c:
        move.w  (a2)+,d4                        | +044
        tst.w   d4                              | +046
        bne.w   .L05d838                        | +048
        tst.w   d2                              | +04c
        bne.w   .L05d838                        | +04e
        tst.w   d3                              | +052
        beq.w   .L05d838                        | +054
        move.w  #0x10,d4                        | +058
        bra.w   .L05d83c                        | +05c
.L05d838:
        move.w  #0xffff,d2                      | +060
.L05d83c:
        add.w   d4,d4                           | +064
        move.w  (a3,d4.w),d0                    | +066
        add.w   d1,d0                           | +06a
        movem.l d1-d4/a1-a3,-(a7)               | +06c
        move.w  #0x2,d1                         | +070
        move.w  #0x2,d2                         | +074
        jsr     0x5da56.l                       | +078
        movem.l (a7)+,d1-d4/a1-a3               | +07e
        adda.w  #0x40,a1                        | +082
        dbra    d3,.L05d81c                     | +086
        rts                                     | +08a

| ----------------------------------------------------------------------------
|  HexDigit_FixTileTable_05d864  @ $05D864  (34 B)
| ----------------------------------------------------------------------------
        .section .text.HexDigit_FixTileTable_05d864, "ax", @progbits
        .global HexDigit_FixTileTable_05d864
HexDigit_FixTileTable_05d864:
        .dc.b   0x0b                          | +000  '.'  (dato, rango --data)
        .dc.b   0x60                          | +001  '`'  (dato, rango --data)
        .dc.b   0x0b                          | +002  '.'  (dato, rango --data)
        .dc.b   0x62                          | +003  'b'  (dato, rango --data)
        .dc.b   0x0b                          | +004  '.'  (dato, rango --data)
        .dc.b   0x64                          | +005  'd'  (dato, rango --data)
        .dc.b   0x0b                          | +006  '.'  (dato, rango --data)
        .dc.b   0x66                          | +007  'f'  (dato, rango --data)
        .dc.b   0x0b                          | +008  '.'  (dato, rango --data)
        .dc.b   0x68                          | +009  'h'  (dato, rango --data)
        .dc.b   0x0b                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x6a                          | +00b  'j'  (dato, rango --data)
        .dc.b   0x0b                          | +00c  '.'  (dato, rango --data)
        .dc.b   0x6c                          | +00d  'l'  (dato, rango --data)
        .dc.b   0x0b                          | +00e  '.'  (dato, rango --data)
        .dc.b   0x6e                          | +00f  'n'  (dato, rango --data)
        .dc.b   0x0c                          | +010  '.'  (dato, rango --data)
        .dc.b   0x60                          | +011  '`'  (dato, rango --data)
        .dc.b   0x0c                          | +012  '.'  (dato, rango --data)
        .dc.b   0x62                          | +013  'b'  (dato, rango --data)
        .dc.b   0x0b                          | +014  '.'  (dato, rango --data)
        .dc.b   0x82                          | +015  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +016  '.'  (dato, rango --data)
        .dc.b   0x84                          | +017  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +018  '.'  (dato, rango --data)
        .dc.b   0x86                          | +019  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x88                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x8a                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x8c                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x0b                          | +020  '.'  (dato, rango --data)
        .dc.b   0x80                          | +021  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Bin16_ToBcd4_05d886  @ $05D886  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Bin16_ToBcd4_05d886, "ax", @progbits
        .global Bin16_ToBcd4_05d886
Bin16_ToBcd4_05d886:
        movem.l d1-d3,-(a7)                     | +000
        moveq   #0,d3                           | +004
        move.w  #0xa,d2                         | +006
        andi.l  #0xffff,d0                      | +00a
        beq.w   .L05d8ea                        | +010
        divu.w  d2,d0                           | +014
        swap    d0                              | +016
        andi.w  #0xf,d0                         | +018
        move.w  d0,d3                           | +01c
        swap    d0                              | +01e
        andi.l  #0xffff,d0                      | +020
        beq.w   .L05d8ea                        | +026
        divu.w  d2,d0                           | +02a
        swap    d0                              | +02c
        andi.w  #0xf,d0                         | +02e
        lsl.w   #0x4,d0                         | +032
        or.w    d0,d3                           | +034
        swap    d0                              | +036
        andi.l  #0xffff,d0                      | +038
        beq.w   .L05d8ea                        | +03e
        divu.w  d2,d0                           | +042
        swap    d0                              | +044
        andi.w  #0xf,d0                         | +046
        lsl.w   #0x8,d0                         | +04a
        or.w    d0,d3                           | +04c
        swap    d0                              | +04e
        andi.l  #0xffff,d0                      | +050
        divu.w  d2,d0                           | +056
        swap    d0                              | +058
        andi.w  #0xf,d0                         | +05a
        lsl.w   #0x8,d0                         | +05e
        lsl.w   #0x4,d0                         | +060
        or.w    d0,d3                           | +062
.L05d8ea:
        move.w  d3,d0                           | +064
        movem.l (a7)+,d1-d3                     | +066
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  Trap15_DivByZero_05D944  @ $05D944  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Trap15_DivByZero_05D944, "ax", @progbits
        .global Trap15_DivByZero_05D944
Trap15_DivByZero_05D944:
        trap    #0xf                            | +000

| ----------------------------------------------------------------------------
|  Noise_LookupByIndex_05d946  @ $05D946  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Noise_LookupByIndex_05d946, "ax", @progbits
        .global Noise_LookupByIndex_05d946
Noise_LookupByIndex_05d946:
        move.w  0x10e22e.l,d0                   | +000
        andi.w  #0xff,d0                        | +006
        move.b  NoiseLut256_05d956(pc,d0.w),d0  | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  NoiseLut256_05d956  @ $05D956  (256 B)
| ----------------------------------------------------------------------------
        .section .text.NoiseLut256_05d956, "ax", @progbits
        .global NoiseLut256_05d956
NoiseLut256_05d956:
        .dc.b   0x34                          | +000  '4'  (dato, rango --data)
        .dc.b   0x72                          | +001  'r'  (dato, rango --data)
        .dc.b   0xce                          | +002  '.'  (dato, rango --data)
        .dc.b   0x3b                          | +003  ';'  (dato, rango --data)
        .dc.b   0x83                          | +004  '.'  (dato, rango --data)
        .dc.b   0xc5                          | +005  '.'  (dato, rango --data)
        .dc.b   0x92                          | +006  '.'  (dato, rango --data)
        .dc.b   0x4a                          | +007  'J'  (dato, rango --data)
        .dc.b   0x84                          | +008  '.'  (dato, rango --data)
        .dc.b   0x79                          | +009  'y'  (dato, rango --data)
        .dc.b   0x8e                          | +00a  '.'  (dato, rango --data)
        .dc.b   0x6b                          | +00b  'k'  (dato, rango --data)
        .dc.b   0x0b                          | +00c  '.'  (dato, rango --data)
        .dc.b   0xd3                          | +00d  '.'  (dato, rango --data)
        .dc.b   0x8d                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xc7                          | +00f  '.'  (dato, rango --data)
        .dc.b   0x4c                          | +010  'L'  (dato, rango --data)
        .dc.b   0x9b                          | +011  '.'  (dato, rango --data)
        .dc.b   0x4e                          | +012  'N'  (dato, rango --data)
        .dc.b   0xc3                          | +013  '.'  (dato, rango --data)
        .dc.b   0x4d                          | +014  'M'  (dato, rango --data)
        .dc.b   0x68                          | +015  'h'  (dato, rango --data)
        .dc.b   0x8f                          | +016  '.'  (dato, rango --data)
        .dc.b   0x33                          | +017  '3'  (dato, rango --data)
        .dc.b   0x9d                          | +018  '.'  (dato, rango --data)
        .dc.b   0x5b                          | +019  '['  (dato, rango --data)
        .dc.b   0x64                          | +01a  'd'  (dato, rango --data)
        .dc.b   0x58                          | +01b  'X'  (dato, rango --data)
        .dc.b   0x0c                          | +01c  '.'  (dato, rango --data)
        .dc.b   0xef                          | +01d  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x19                          | +01f  '.'  (dato, rango --data)
        .dc.b   0x7c                          | +020  '|'  (dato, rango --data)
        .dc.b   0xaf                          | +021  '.'  (dato, rango --data)
        .dc.b   0x21                          | +022  '!'  (dato, rango --data)
        .dc.b   0xa5                          | +023  '.'  (dato, rango --data)
        .dc.b   0xde                          | +024  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +025  '.'  (dato, rango --data)
        .dc.b   0x56                          | +026  'V'  (dato, rango --data)
        .dc.b   0x7a                          | +027  'z'  (dato, rango --data)
        .dc.b   0xac                          | +028  '.'  (dato, rango --data)
        .dc.b   0x80                          | +029  '.'  (dato, rango --data)
        .dc.b   0xa7                          | +02a  '.'  (dato, rango --data)
        .dc.b   0xda                          | +02b  '.'  (dato, rango --data)
        .dc.b   0x87                          | +02c  '.'  (dato, rango --data)
        .dc.b   0x37                          | +02d  '7'  (dato, rango --data)
        .dc.b   0x12                          | +02e  '.'  (dato, rango --data)
        .dc.b   0xbf                          | +02f  '.'  (dato, rango --data)
        .dc.b   0xa8                          | +030  '.'  (dato, rango --data)
        .dc.b   0x51                          | +031  'Q'  (dato, rango --data)
        .dc.b   0x02                          | +032  '.'  (dato, rango --data)
        .dc.b   0x94                          | +033  '.'  (dato, rango --data)
        .dc.b   0xb4                          | +034  '.'  (dato, rango --data)
        .dc.b   0xbe                          | +035  '.'  (dato, rango --data)
        .dc.b   0x6e                          | +036  'n'  (dato, rango --data)
        .dc.b   0x23                          | +037  '#'  (dato, rango --data)
        .dc.b   0xbb                          | +038  '.'  (dato, rango --data)
        .dc.b   0x01                          | +039  '.'  (dato, rango --data)
        .dc.b   0x0e                          | +03a  '.'  (dato, rango --data)
        .dc.b   0x05                          | +03b  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +03c  '.'  (dato, rango --data)
        .dc.b   0x55                          | +03d  'U'  (dato, rango --data)
        .dc.b   0x9f                          | +03e  '.'  (dato, rango --data)
        .dc.b   0x76                          | +03f  'v'  (dato, rango --data)
        .dc.b   0xf1                          | +040  '.'  (dato, rango --data)
        .dc.b   0xd7                          | +041  '.'  (dato, rango --data)
        .dc.b   0xd1                          | +042  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +043  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +044  '.'  (dato, rango --data)
        .dc.b   0x7f                          | +045  '.'  (dato, rango --data)
        .dc.b   0x57                          | +046  'W'  (dato, rango --data)
        .dc.b   0xff                          | +047  '.'  (dato, rango --data)
        .dc.b   0x36                          | +048  '6'  (dato, rango --data)
        .dc.b   0x54                          | +049  'T'  (dato, rango --data)
        .dc.b   0xb0                          | +04a  '.'  (dato, rango --data)
        .dc.b   0x41                          | +04b  'A'  (dato, rango --data)
        .dc.b   0x1b                          | +04c  '.'  (dato, rango --data)
        .dc.b   0x81                          | +04d  '.'  (dato, rango --data)
        .dc.b   0x32                          | +04e  '2'  (dato, rango --data)
        .dc.b   0xc1                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xc6                          | +050  '.'  (dato, rango --data)
        .dc.b   0xae                          | +051  '.'  (dato, rango --data)
        .dc.b   0x89                          | +052  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +053  '.'  (dato, rango --data)
        .dc.b   0x97                          | +054  '.'  (dato, rango --data)
        .dc.b   0xc9                          | +055  '.'  (dato, rango --data)
        .dc.b   0x27                          | +056  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +057  '.'  (dato, rango --data)
        .dc.b   0x46                          | +058  'F'  (dato, rango --data)
        .dc.b   0x06                          | +059  '.'  (dato, rango --data)
        .dc.b   0x6a                          | +05a  'j'  (dato, rango --data)
        .dc.b   0x22                          | +05b  '"'  (dato, rango --data)
        .dc.b   0x03                          | +05c  '.'  (dato, rango --data)
        .dc.b   0x39                          | +05d  '9'  (dato, rango --data)
        .dc.b   0x25                          | +05e  '%'  (dato, rango --data)
        .dc.b   0xe1                          | +05f  '.'  (dato, rango --data)
        .dc.b   0xa2                          | +060  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +061  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +062  '.'  (dato, rango --data)
        .dc.b   0x11                          | +063  '.'  (dato, rango --data)
        .dc.b   0x8a                          | +064  '.'  (dato, rango --data)
        .dc.b   0x18                          | +065  '.'  (dato, rango --data)
        .dc.b   0x4f                          | +066  'O'  (dato, rango --data)
        .dc.b   0x04                          | +067  '.'  (dato, rango --data)
        .dc.b   0xd6                          | +068  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +069  '.'  (dato, rango --data)
        .dc.b   0x60                          | +06a  '`'  (dato, rango --data)
        .dc.b   0xb6                          | +06b  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +06c  '.'  (dato, rango --data)
        .dc.b   0x6f                          | +06d  'o'  (dato, rango --data)
        .dc.b   0xfa                          | +06e  '.'  (dato, rango --data)
        .dc.b   0xb1                          | +06f  '.'  (dato, rango --data)
        .dc.b   0x61                          | +070  'a'  (dato, rango --data)
        .dc.b   0x26                          | +071  '&'  (dato, rango --data)
        .dc.b   0x0a                          | +072  '.'  (dato, rango --data)
        .dc.b   0xb3                          | +073  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +074  '.'  (dato, rango --data)
        .dc.b   0x5c                          | +075  '.'  (dato, rango --data)
        .dc.b   0x65                          | +076  'e'  (dato, rango --data)
        .dc.b   0x91                          | +077  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +078  '.'  (dato, rango --data)
        .dc.b   0x5d                          | +079  ']'  (dato, rango --data)
        .dc.b   0x9c                          | +07a  '.'  (dato, rango --data)
        .dc.b   0xd9                          | +07b  '.'  (dato, rango --data)
        .dc.b   0x67                          | +07c  'g'  (dato, rango --data)
        .dc.b   0x8b                          | +07d  '.'  (dato, rango --data)
        .dc.b   0x6d                          | +07e  'm'  (dato, rango --data)
        .dc.b   0x31                          | +07f  '1'  (dato, rango --data)
        .dc.b   0x15                          | +080  '.'  (dato, rango --data)
        .dc.b   0x09                          | +081  '.'  (dato, rango --data)
        .dc.b   0x62                          | +082  'b'  (dato, rango --data)
        .dc.b   0xbd                          | +083  '.'  (dato, rango --data)
        .dc.b   0x29                          | +084  ')'  (dato, rango --data)
        .dc.b   0x2b                          | +085  '+'  (dato, rango --data)
        .dc.b   0x3e                          | +086  '>'  (dato, rango --data)
        .dc.b   0x5f                          | +087  '_'  (dato, rango --data)
        .dc.b   0x53                          | +088  'S'  (dato, rango --data)
        .dc.b   0xa3                          | +089  '.'  (dato, rango --data)
        .dc.b   0xca                          | +08a  '.'  (dato, rango --data)
        .dc.b   0x40                          | +08b  '@'  (dato, rango --data)
        .dc.b   0x2d                          | +08c  '-'  (dato, rango --data)
        .dc.b   0x28                          | +08d  '('  (dato, rango --data)
        .dc.b   0xf9                          | +08e  '.'  (dato, rango --data)
        .dc.b   0x66                          | +08f  'f'  (dato, rango --data)
        .dc.b   0x13                          | +090  '.'  (dato, rango --data)
        .dc.b   0xb8                          | +091  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +092  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +093  '.'  (dato, rango --data)
        .dc.b   0x35                          | +094  '5'  (dato, rango --data)
        .dc.b   0x88                          | +095  '.'  (dato, rango --data)
        .dc.b   0x9a                          | +096  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +097  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +098  '.'  (dato, rango --data)
        .dc.b   0x16                          | +099  '.'  (dato, rango --data)
        .dc.b   0x17                          | +09a  '.'  (dato, rango --data)
        .dc.b   0x7b                          | +09b  '{'  (dato, rango --data)
        .dc.b   0x48                          | +09c  'H'  (dato, rango --data)
        .dc.b   0x63                          | +09d  'c'  (dato, rango --data)
        .dc.b   0x43                          | +09e  'C'  (dato, rango --data)
        .dc.b   0xfc                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0x45                          | +0a1  'E'  (dato, rango --data)
        .dc.b   0x08                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xd5                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0x2f                          | +0a4  '/'  (dato, rango --data)
        .dc.b   0x75                          | +0a5  'u'  (dato, rango --data)
        .dc.b   0x14                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0x71                          | +0a7  'q'  (dato, rango --data)
        .dc.b   0x44                          | +0a8  'D'  (dato, rango --data)
        .dc.b   0xed                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0xcf                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0x74                          | +0ab  't'  (dato, rango --data)
        .dc.b   0xfb                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0x73                          | +0ad  's'  (dato, rango --data)
        .dc.b   0x90                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0xb7                          | +0af  '.'  (dato, rango --data)
        .dc.b   0xa1                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0x2a                          | +0b1  '*'  (dato, rango --data)
        .dc.b   0x10                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0xdb                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0x7d                          | +0b4  '}'  (dato, rango --data)
        .dc.b   0xd2                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0x2c                          | +0b6  ','  (dato, rango --data)
        .dc.b   0x69                          | +0b7  'i'  (dato, rango --data)
        .dc.b   0x78                          | +0b8  'x'  (dato, rango --data)
        .dc.b   0x38                          | +0b9  '8'  (dato, rango --data)
        .dc.b   0x95                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0x52                          | +0bb  'R'  (dato, rango --data)
        .dc.b   0xcd                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0xb9                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0x8c                          | +0be  '.'  (dato, rango --data)
        .dc.b   0x3d                          | +0bf  '='  (dato, rango --data)
        .dc.b   0xdf                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0x49                          | +0c1  'I'  (dato, rango --data)
        .dc.b   0xcc                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0xc4                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0x07                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0xa6                          | +0c6  '.'  (dato, rango --data)
        .dc.b   0x1c                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0x50                          | +0c8  'P'  (dato, rango --data)
        .dc.b   0x4b                          | +0c9  'K'  (dato, rango --data)
        .dc.b   0x47                          | +0ca  'G'  (dato, rango --data)
        .dc.b   0x99                          | +0cb  '.'  (dato, rango --data)
        .dc.b   0x82                          | +0cc  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0cd  '.'  (dato, rango --data)
        .dc.b   0x24                          | +0ce  '$'  (dato, rango --data)
        .dc.b   0x1a                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0x59                          | +0d0  'Y'  (dato, rango --data)
        .dc.b   0xc0                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0x3a                          | +0d2  ':'  (dato, rango --data)
        .dc.b   0xeb                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0xdd                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0x2e                          | +0d5  '.'  (dato, rango --data)
        .dc.b   0x5a                          | +0d6  'Z'  (dato, rango --data)
        .dc.b   0xd0                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0xad                          | +0d8  '.'  (dato, rango --data)
        .dc.b   0xcb                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0x00                          | +0da  '.'  (dato, rango --data)
        .dc.b   0xa4                          | +0db  '.'  (dato, rango --data)
        .dc.b   0x1f                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0dd  '.'  (dato, rango --data)
        .dc.b   0x85                          | +0de  '.'  (dato, rango --data)
        .dc.b   0xab                          | +0df  '.'  (dato, rango --data)
        .dc.b   0x86                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0x96                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0x30                          | +0e3  '0'  (dato, rango --data)
        .dc.b   0x0f                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0xdc                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0x6c                          | +0e7  'l'  (dato, rango --data)
        .dc.b   0x20                          | +0e8  ' '  (dato, rango --data)
        .dc.b   0xb2                          | +0e9  '.'  (dato, rango --data)
        .dc.b   0x1e                          | +0ea  '.'  (dato, rango --data)
        .dc.b   0xaa                          | +0eb  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0ec  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0ed  '.'  (dato, rango --data)
        .dc.b   0x1d                          | +0ee  '.'  (dato, rango --data)
        .dc.b   0xa9                          | +0ef  '.'  (dato, rango --data)
        .dc.b   0x3c                          | +0f0  '<'  (dato, rango --data)
        .dc.b   0x77                          | +0f1  'w'  (dato, rango --data)
        .dc.b   0x5e                          | +0f2  '^'  (dato, rango --data)
        .dc.b   0x42                          | +0f3  'B'  (dato, rango --data)
        .dc.b   0xbc                          | +0f4  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +0f5  '.'  (dato, rango --data)
        .dc.b   0xb5                          | +0f6  '.'  (dato, rango --data)
        .dc.b   0x70                          | +0f7  'p'  (dato, rango --data)
        .dc.b   0x9e                          | +0f8  '.'  (dato, rango --data)
        .dc.b   0xd8                          | +0f9  '.'  (dato, rango --data)
        .dc.b   0x3f                          | +0fa  '?'  (dato, rango --data)
        .dc.b   0xba                          | +0fb  '.'  (dato, rango --data)
        .dc.b   0x93                          | +0fc  '.'  (dato, rango --data)
        .dc.b   0x7e                          | +0fd  '~'  (dato, rango --data)
        .dc.b   0x98                          | +0fe  '.'  (dato, rango --data)
        .dc.b   0x0d                          | +0ff  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  ListCursor_Step_05db6a  @ $05DB6A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.ListCursor_Step_05db6a, "ax", @progbits
        .global ListCursor_Step_05db6a
ListCursor_Step_05db6a:
        movea.l 0x3c(a6),a1                     | +000
        cmpi.w  #0xffff,(a1)                    | +004
        beq.w   SetC_05dbbc                     | +008
        subq.w  #0x1,0x46(a6)                   | +00c
        cmpi.w  #0x0,0x46(a6)                   | +010
        bgt.w   ClearXNV_05dba0                 | +016
        jsr     ListCursor_ReinitClipped_05DBDC(pc) | +01a
        addq.l  #0x8,0x3c(a6)                   | +01e
        movea.l 0x3c(a6),a1                     | +022
        cmpi.w  #0x0,(a1)                       | +026
        bne.w   ListCursor_LoadEntry_05dba6     | +02a
        movea.l 0x2(a1),a0                      | +02e
        jsr     InstallListPubHead_05DB58(pc)   | +032

| ----------------------------------------------------------------------------
|  ListCursor_LoadEntry_05dba6  @ $05DBA6  (22 B)
| ----------------------------------------------------------------------------
        .section .text.ListCursor_LoadEntry_05dba6, "ax", @progbits
        .global ListCursor_LoadEntry_05dba6
ListCursor_LoadEntry_05dba6:
        cmpi.w  #0xffff,(a1)                    | +000
        beq.w   SetC_05dbbc                     | +004
        move.w  0x6(a1),d0                      | +008
        move.w  d0,0x46(a6)                     | +00c
        jsr     ListCursor_Reinit_05DBC2(pc)    | +010
        bra.b   ClearXNV_05dba0                 | +014

| ----------------------------------------------------------------------------
|  Entity_NegIfFacing_05dca4  @ $05DCA4  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_NegIfFacing_05dca4, "ax", @progbits
        .global Entity_NegIfFacing_05dca4
Entity_NegIfFacing_05dca4:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   SetTaskW_05dcb0                 | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  Entity_SetVx_NegIfFacing_05dcb6  @ $05DCB6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SetVx_NegIfFacing_05dcb6, "ax", @progbits
        .global Entity_SetVx_NegIfFacing_05dcb6
Entity_SetVx_NegIfFacing_05dcb6:
        move.w  d0,0x36(a6)                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   SetTaskW_05dcc8                 | +00a
        neg.w   0x28(a6)                        | +00e

| ----------------------------------------------------------------------------
|  Spawn_ChildFromDesc_05dcce  @ $05DCCE  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Spawn_ChildFromDesc_05dcce, "ax", @progbits
        .global Spawn_ChildFromDesc_05dcce
Spawn_ChildFromDesc_05dcce:
        movem.l a1/a6,-(a7)                     | +000
        cmpi.b  #0x2,(a1)                       | +004
        beq.w   .L05dcee                        | +008
        lea     0x1008a0.l,a6                   | +00c
        movea.l 0x6(a1),a1                      | +012
        jsr     0x4ae.l                         | +016
        bra.w   .L05dcf8                        | +01c
.L05dcee:
        movea.l 0x6(a1),a1                      | +020
        jsr     0x4498e.l                       | +024
.L05dcf8:
        movem.l (a7)+,a1/a6                     | +02a
        move.l  a1,0x3c(a0)                     | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Handler_ApplyCameraSelf_CopyTransform_05dd22  @ $05DD22  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Handler_ApplyCameraSelf_CopyTransform_05dd22, "ax", @progbits
        .global Handler_ApplyCameraSelf_CopyTransform_05dd22
Handler_ApplyCameraSelf_CopyTransform_05dd22:
        jsr     0x440e4.l                       | +000
        bra.b   Entity_CopyTransform            | +006

| ----------------------------------------------------------------------------
|  ScreenBox_Default_05dd4c  @ $05DD4C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.ScreenBox_Default_05dd4c, "ax", @progbits
        .global ScreenBox_Default_05dd4c
ScreenBox_Default_05dd4c:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x00                          | +002  '.'  (dato, rango --data)
        .dc.b   0x01                          | +003  '.'  (dato, rango --data)
        .dc.b   0x00                          | +004  '.'  (dato, rango --data)
        .dc.b   0x00                          | +005  '.'  (dato, rango --data)
        .dc.b   0x00                          | +006  '.'  (dato, rango --data)
        .dc.b   0x01                          | +007  '.'  (dato, rango --data)
        .dc.b   0xff                          | +008  '.'  (dato, rango --data)
        .dc.b   0xff                          | +009  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  Entity_SetOffscreenFlag_05dd56  @ $05DD56  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SetOffscreenFlag_05dd56, "ax", @progbits
        .global Entity_SetOffscreenFlag_05dd56
Entity_SetOffscreenFlag_05dd56:
        bset    #0x7,0x13(a6)                   | +000

| ----------------------------------------------------------------------------
|  Entity_CheckOnScreenBox_05dd5c  @ $05DD5C  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CheckOnScreenBox_05dd5c, "ax", @progbits
        .global Entity_CheckOnScreenBox_05dd5c
Entity_CheckOnScreenBox_05dd5c:
        cmpa.l  #0xffffffff,a0                  | +000
        bne.w   .L05dd6a                        | +006
        lea     ScreenBox_Default_05dd4c(pc),a0 | +00a
.L05dd6a:
        btst    #0x7,0x13(a6)                   | +00e
        beq.w   Entity_CheckEnterScreen_05ddbe  | +014
        move.w  #0x0,d0                         | +018
        move.w  #0x140,d1                       | +01c
        move.w  #0x100,d2                       | +020
        move.w  #0x1f0,d3                       | +024
        add.w   (a0),d0                         | +028
        add.w   0x2(a0),d1                      | +02a
        sub.w   0x6(a0),d2                      | +02e
        sub.w   0x4(a0),d3                      | +032
        cmp.w   0x22(a6),d0                     | +036
        bgt.w   SetC_05ddb8                     | +03a
        cmp.w   0x22(a6),d1                     | +03e
        blt.w   SetC_05ddb8                     | +042
        cmp.w   0x24(a6),d2                     | +046
        bgt.w   SetC_05ddb8                     | +04a
        cmp.w   0x24(a6),d3                     | +04e
        blt.w   SetC_05ddb8                     | +052

| ----------------------------------------------------------------------------
|  Entity_CheckEnterScreen_05ddbe  @ $05DDBE  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CheckEnterScreen_05ddbe, "ax", @progbits
        .global Entity_CheckEnterScreen_05ddbe
Entity_CheckEnterScreen_05ddbe:
        cmpi.w  #0x140,0x22(a6)                 | +000
        bge.w   Entity_CheckLeaveScreenWide_05ddf2 | +006
        cmpi.w  #0x0,0x22(a6)                   | +00a
        ble.w   Entity_CheckLeaveScreenWide_05ddf2 | +010
        cmpi.w  #0x1f0,0x24(a6)                 | +014
        bge.w   Entity_CheckLeaveScreenWide_05ddf2 | +01a
        cmpi.w  #0x100,0x24(a6)                 | +01e
        ble.w   Entity_CheckLeaveScreenWide_05ddf2 | +024
        .global Entity_CheckEnterScreen_05ddbe__L05dde6
Entity_CheckEnterScreen_05ddbe__L05dde6:
.L05dde6:
        bset    #0x7,0x13(a6)                   | +028

| ----------------------------------------------------------------------------
|  Entity_CheckLeaveScreenWide_05ddf2  @ $05DDF2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CheckLeaveScreenWide_05ddf2, "ax", @progbits
        .global Entity_CheckLeaveScreenWide_05ddf2
Entity_CheckLeaveScreenWide_05ddf2:
        cmpi.w  #0xff80,0x22(a6)                | +000
        ble.b   Entity_CheckEnterScreen_05ddbe__L05dde6 | +006
        cmpi.w  #0x200,0x22(a6)                 | +008
        bge.b   Entity_CheckEnterScreen_05ddbe__L05dde6 | +00e
        cmpi.w  #0x0,0x24(a6)                   | +010
        ble.b   Entity_CheckEnterScreen_05ddbe__L05dde6 | +016
        cmpi.w  #0x280,0x24(a6)                 | +018
        bge.b   Entity_CheckEnterScreen_05ddbe__L05dde6 | +01e

| ----------------------------------------------------------------------------
|  AtanLog_Table_05de18  @ $05DE18  (256 B)
| ----------------------------------------------------------------------------
        .section .text.AtanLog_Table_05de18, "ax", @progbits
        .global AtanLog_Table_05de18
AtanLog_Table_05de18:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0x00                          | +001  '.'  (dato, rango --data)
        .dc.b   0x20                          | +002  ' '  (dato, rango --data)
        .dc.b   0x32                          | +003  '2'  (dato, rango --data)
        .dc.b   0x40                          | +004  '@'  (dato, rango --data)
        .dc.b   0x4a                          | +005  'J'  (dato, rango --data)
        .dc.b   0x52                          | +006  'R'  (dato, rango --data)
        .dc.b   0x59                          | +007  'Y'  (dato, rango --data)
        .dc.b   0x60                          | +008  '`'  (dato, rango --data)
        .dc.b   0x65                          | +009  'e'  (dato, rango --data)
        .dc.b   0x6a                          | +00a  'j'  (dato, rango --data)
        .dc.b   0x6e                          | +00b  'n'  (dato, rango --data)
        .dc.b   0x72                          | +00c  'r'  (dato, rango --data)
        .dc.b   0x76                          | +00d  'v'  (dato, rango --data)
        .dc.b   0x79                          | +00e  'y'  (dato, rango --data)
        .dc.b   0x7d                          | +00f  '}'  (dato, rango --data)
        .dc.b   0x80                          | +010  '.'  (dato, rango --data)
        .dc.b   0x82                          | +011  '.'  (dato, rango --data)
        .dc.b   0x85                          | +012  '.'  (dato, rango --data)
        .dc.b   0x87                          | +013  '.'  (dato, rango --data)
        .dc.b   0x8a                          | +014  '.'  (dato, rango --data)
        .dc.b   0x8c                          | +015  '.'  (dato, rango --data)
        .dc.b   0x8e                          | +016  '.'  (dato, rango --data)
        .dc.b   0x90                          | +017  '.'  (dato, rango --data)
        .dc.b   0x92                          | +018  '.'  (dato, rango --data)
        .dc.b   0x94                          | +019  '.'  (dato, rango --data)
        .dc.b   0x96                          | +01a  '.'  (dato, rango --data)
        .dc.b   0x98                          | +01b  '.'  (dato, rango --data)
        .dc.b   0x99                          | +01c  '.'  (dato, rango --data)
        .dc.b   0x9b                          | +01d  '.'  (dato, rango --data)
        .dc.b   0x9d                          | +01e  '.'  (dato, rango --data)
        .dc.b   0x9e                          | +01f  '.'  (dato, rango --data)
        .dc.b   0xa0                          | +020  '.'  (dato, rango --data)
        .dc.b   0xa1                          | +021  '.'  (dato, rango --data)
        .dc.b   0xa2                          | +022  '.'  (dato, rango --data)
        .dc.b   0xa4                          | +023  '.'  (dato, rango --data)
        .dc.b   0xa5                          | +024  '.'  (dato, rango --data)
        .dc.b   0xa6                          | +025  '.'  (dato, rango --data)
        .dc.b   0xa7                          | +026  '.'  (dato, rango --data)
        .dc.b   0xa9                          | +027  '.'  (dato, rango --data)
        .dc.b   0xaa                          | +028  '.'  (dato, rango --data)
        .dc.b   0xab                          | +029  '.'  (dato, rango --data)
        .dc.b   0xac                          | +02a  '.'  (dato, rango --data)
        .dc.b   0xad                          | +02b  '.'  (dato, rango --data)
        .dc.b   0xae                          | +02c  '.'  (dato, rango --data)
        .dc.b   0xaf                          | +02d  '.'  (dato, rango --data)
        .dc.b   0xb0                          | +02e  '.'  (dato, rango --data)
        .dc.b   0xb1                          | +02f  '.'  (dato, rango --data)
        .dc.b   0xb2                          | +030  '.'  (dato, rango --data)
        .dc.b   0xb3                          | +031  '.'  (dato, rango --data)
        .dc.b   0xb4                          | +032  '.'  (dato, rango --data)
        .dc.b   0xb5                          | +033  '.'  (dato, rango --data)
        .dc.b   0xb6                          | +034  '.'  (dato, rango --data)
        .dc.b   0xb7                          | +035  '.'  (dato, rango --data)
        .dc.b   0xb8                          | +036  '.'  (dato, rango --data)
        .dc.b   0xb9                          | +037  '.'  (dato, rango --data)
        .dc.b   0xb9                          | +038  '.'  (dato, rango --data)
        .dc.b   0xba                          | +039  '.'  (dato, rango --data)
        .dc.b   0xbb                          | +03a  '.'  (dato, rango --data)
        .dc.b   0xbc                          | +03b  '.'  (dato, rango --data)
        .dc.b   0xbd                          | +03c  '.'  (dato, rango --data)
        .dc.b   0xbd                          | +03d  '.'  (dato, rango --data)
        .dc.b   0xbe                          | +03e  '.'  (dato, rango --data)
        .dc.b   0xbf                          | +03f  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +040  '.'  (dato, rango --data)
        .dc.b   0xc0                          | +041  '.'  (dato, rango --data)
        .dc.b   0xc1                          | +042  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +043  '.'  (dato, rango --data)
        .dc.b   0xc2                          | +044  '.'  (dato, rango --data)
        .dc.b   0xc3                          | +045  '.'  (dato, rango --data)
        .dc.b   0xc4                          | +046  '.'  (dato, rango --data)
        .dc.b   0xc4                          | +047  '.'  (dato, rango --data)
        .dc.b   0xc5                          | +048  '.'  (dato, rango --data)
        .dc.b   0xc6                          | +049  '.'  (dato, rango --data)
        .dc.b   0xc6                          | +04a  '.'  (dato, rango --data)
        .dc.b   0xc7                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xc7                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xc8                          | +04d  '.'  (dato, rango --data)
        .dc.b   0xc9                          | +04e  '.'  (dato, rango --data)
        .dc.b   0xc9                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xca                          | +050  '.'  (dato, rango --data)
        .dc.b   0xca                          | +051  '.'  (dato, rango --data)
        .dc.b   0xcb                          | +052  '.'  (dato, rango --data)
        .dc.b   0xcc                          | +053  '.'  (dato, rango --data)
        .dc.b   0xcc                          | +054  '.'  (dato, rango --data)
        .dc.b   0xcd                          | +055  '.'  (dato, rango --data)
        .dc.b   0xcd                          | +056  '.'  (dato, rango --data)
        .dc.b   0xce                          | +057  '.'  (dato, rango --data)
        .dc.b   0xce                          | +058  '.'  (dato, rango --data)
        .dc.b   0xcf                          | +059  '.'  (dato, rango --data)
        .dc.b   0xcf                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xd0                          | +05b  '.'  (dato, rango --data)
        .dc.b   0xd0                          | +05c  '.'  (dato, rango --data)
        .dc.b   0xd1                          | +05d  '.'  (dato, rango --data)
        .dc.b   0xd1                          | +05e  '.'  (dato, rango --data)
        .dc.b   0xd2                          | +05f  '.'  (dato, rango --data)
        .dc.b   0xd2                          | +060  '.'  (dato, rango --data)
        .dc.b   0xd3                          | +061  '.'  (dato, rango --data)
        .dc.b   0xd3                          | +062  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +063  '.'  (dato, rango --data)
        .dc.b   0xd4                          | +064  '.'  (dato, rango --data)
        .dc.b   0xd5                          | +065  '.'  (dato, rango --data)
        .dc.b   0xd5                          | +066  '.'  (dato, rango --data)
        .dc.b   0xd5                          | +067  '.'  (dato, rango --data)
        .dc.b   0xd6                          | +068  '.'  (dato, rango --data)
        .dc.b   0xd6                          | +069  '.'  (dato, rango --data)
        .dc.b   0xd7                          | +06a  '.'  (dato, rango --data)
        .dc.b   0xd7                          | +06b  '.'  (dato, rango --data)
        .dc.b   0xd8                          | +06c  '.'  (dato, rango --data)
        .dc.b   0xd8                          | +06d  '.'  (dato, rango --data)
        .dc.b   0xd9                          | +06e  '.'  (dato, rango --data)
        .dc.b   0xd9                          | +06f  '.'  (dato, rango --data)
        .dc.b   0xd9                          | +070  '.'  (dato, rango --data)
        .dc.b   0xda                          | +071  '.'  (dato, rango --data)
        .dc.b   0xda                          | +072  '.'  (dato, rango --data)
        .dc.b   0xdb                          | +073  '.'  (dato, rango --data)
        .dc.b   0xdb                          | +074  '.'  (dato, rango --data)
        .dc.b   0xdb                          | +075  '.'  (dato, rango --data)
        .dc.b   0xdc                          | +076  '.'  (dato, rango --data)
        .dc.b   0xdc                          | +077  '.'  (dato, rango --data)
        .dc.b   0xdd                          | +078  '.'  (dato, rango --data)
        .dc.b   0xdd                          | +079  '.'  (dato, rango --data)
        .dc.b   0xdd                          | +07a  '.'  (dato, rango --data)
        .dc.b   0xde                          | +07b  '.'  (dato, rango --data)
        .dc.b   0xde                          | +07c  '.'  (dato, rango --data)
        .dc.b   0xde                          | +07d  '.'  (dato, rango --data)
        .dc.b   0xdf                          | +07e  '.'  (dato, rango --data)
        .dc.b   0xdf                          | +07f  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +080  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +081  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +082  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +083  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +084  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +085  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +086  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +087  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +088  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +089  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08a  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08b  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +08c  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +08d  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +08e  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +08f  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +090  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +091  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +092  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +093  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +094  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +095  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +096  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +097  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +098  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +099  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +09a  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +09b  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +09c  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +09d  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +09e  '.'  (dato, rango --data)
        .dc.b   0xea                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0ad  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0af  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0b1  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0b7  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0b8  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0b9  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0be  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0bf  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0c1  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0c6  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0c8  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0c9  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0ca  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0cb  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0cc  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0cd  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0ce  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +0d0  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0xf6                          | +0d2  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0d5  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0d6  '.'  (dato, rango --data)
        .dc.b   0xf7                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +0d8  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +0da  '.'  (dato, rango --data)
        .dc.b   0xf8                          | +0db  '.'  (dato, rango --data)
        .dc.b   0xf9                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0xf9                          | +0dd  '.'  (dato, rango --data)
        .dc.b   0xf9                          | +0de  '.'  (dato, rango --data)
        .dc.b   0xf9                          | +0df  '.'  (dato, rango --data)
        .dc.b   0xf9                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0xfa                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0xfa                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0xfa                          | +0e3  '.'  (dato, rango --data)
        .dc.b   0xfa                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0xfa                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0xfb                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0xfb                          | +0e7  '.'  (dato, rango --data)
        .dc.b   0xfb                          | +0e8  '.'  (dato, rango --data)
        .dc.b   0xfb                          | +0e9  '.'  (dato, rango --data)
        .dc.b   0xfb                          | +0ea  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0eb  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0ec  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0ed  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0ee  '.'  (dato, rango --data)
        .dc.b   0xfc                          | +0ef  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f0  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f1  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f2  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f3  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f4  '.'  (dato, rango --data)
        .dc.b   0xfd                          | +0f5  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0f6  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0f7  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0f8  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0f9  '.'  (dato, rango --data)
        .dc.b   0xfe                          | +0fa  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fb  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fc  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fd  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0fe  '.'  (dato, rango --data)
        .dc.b   0xff                          | +0ff  '.'  (dato, rango --data)

| ----------------------------------------------------------------------------
|  AtanExp_Table_05df18  @ $05DF18  (232 B)
| ----------------------------------------------------------------------------
        .section .text.AtanExp_Table_05df18, "ax", @progbits
        .global AtanExp_Table_05df18
AtanExp_Table_05df18:
        .dc.b   0x00                          | +000  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +001  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +002  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +003  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +004  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +005  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +006  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +007  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +008  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +009  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00a  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00b  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00c  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00d  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00e  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +00f  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +010  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +011  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +012  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +013  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +014  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +015  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +016  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +017  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +018  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +019  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01a  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01b  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01c  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01d  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01e  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +01f  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +020  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +021  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +022  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +023  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +024  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +025  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +026  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +027  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +028  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +029  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02a  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02b  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02c  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02d  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02e  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +02f  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +030  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +031  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +032  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +033  '.'  (dato, rango --data)
        .dc.b   0xe0                          | +034  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +035  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +036  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +037  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +038  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +039  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03a  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03b  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03c  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03d  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03e  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +03f  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +040  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +041  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +042  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +043  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +044  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +045  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +046  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +047  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +048  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +049  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04a  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04b  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04c  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04d  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04e  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +04f  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +050  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +051  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +052  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +053  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +054  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +055  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +056  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +057  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +058  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +059  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05a  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05b  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05c  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05d  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05e  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +05f  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +060  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +061  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +062  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +063  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +064  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +065  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +066  '.'  (dato, rango --data)
        .dc.b   0xe1                          | +067  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +068  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +069  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06a  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06b  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06c  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06d  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06e  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +06f  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +070  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +071  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +072  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +073  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +074  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +075  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +076  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +077  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +078  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +079  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07a  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07b  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07c  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07d  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07e  '.'  (dato, rango --data)
        .dc.b   0xe2                          | +07f  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +080  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +081  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +082  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +083  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +084  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +085  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +086  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +087  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +088  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +089  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08a  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08b  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08c  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08d  '.'  (dato, rango --data)
        .dc.b   0xe3                          | +08e  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +08f  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +090  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +091  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +092  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +093  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +094  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +095  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +096  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +097  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +098  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +099  '.'  (dato, rango --data)
        .dc.b   0xe4                          | +09a  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +09b  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +09c  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +09d  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +09e  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +09f  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +0a0  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +0a1  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +0a2  '.'  (dato, rango --data)
        .dc.b   0xe5                          | +0a3  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a4  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a5  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a6  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a7  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a8  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0a9  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0aa  '.'  (dato, rango --data)
        .dc.b   0xe6                          | +0ab  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0ac  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0ad  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0ae  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0af  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0b0  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0b1  '.'  (dato, rango --data)
        .dc.b   0xe7                          | +0b2  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b3  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b4  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b5  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b6  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b7  '.'  (dato, rango --data)
        .dc.b   0xe8                          | +0b8  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0b9  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0ba  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0bb  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0bc  '.'  (dato, rango --data)
        .dc.b   0xe9                          | +0bd  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0be  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0bf  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0c0  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0c1  '.'  (dato, rango --data)
        .dc.b   0xea                          | +0c2  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0c3  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0c4  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0c5  '.'  (dato, rango --data)
        .dc.b   0xeb                          | +0c6  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0c7  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0c8  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0c9  '.'  (dato, rango --data)
        .dc.b   0xec                          | +0ca  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0cb  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0cc  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0cd  '.'  (dato, rango --data)
        .dc.b   0xed                          | +0ce  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0cf  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0d0  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0d1  '.'  (dato, rango --data)
        .dc.b   0xee                          | +0d2  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0d3  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0d4  '.'  (dato, rango --data)
        .dc.b   0xef                          | +0d5  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0d6  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0d7  '.'  (dato, rango --data)
        .dc.b   0xf0                          | +0d8  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0d9  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0da  '.'  (dato, rango --data)
        .dc.b   0xf1                          | +0db  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0dc  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0dd  '.'  (dato, rango --data)
        .dc.b   0xf2                          | +0de  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0df  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0e0  '.'  (dato, rango --data)
        .dc.b   0xf3                          | +0e1  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0e2  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0e3  '.'  (dato, rango --data)
        .dc.b   0xf4                          | +0e4  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0e5  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0e6  '.'  (dato, rango --data)
        .dc.b   0xf5                          | +0e7  '.'  (dato, rango --data)
