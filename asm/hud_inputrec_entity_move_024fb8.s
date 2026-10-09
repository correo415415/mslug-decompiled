| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave BBBBB — HUD del jugador (fix layer), grabación/replay de input del
|  attract, descriptores de slot de jugador y motor de movimiento+colisión
|  de entidades
|  Región: $024E10..$027400  (8,874 B, 88 entradas, 38 huecos)
| ============================================================================
|
|  A. QUÉ HAY AQUÍ
|  Cuatro bloques que rellenan los 38 huecos entre las islas C
|  (ClearXN/SetXN/SetC/ClearC, SetTaskHandler_*, JsrAbsThunk_*, JmpToScheduler,
|  NopCCR_*, EntityResetState, SetTaskW_026830) y los módulos ya cerrados
|  start_dispatcher_cluster_024e38.s, misc_batches_046ac6_024fec.s y
|  camera_list_ctx_helpers_wave_ii.s:
|
|  1) $024FB8..$025456  Input del jugador + grabación/replay (attract):
|     - PlayerCtx_ResetAndSetRepeat1002/SetRepeatRate/IsModeSingle: setters
|       de `$106EC8` (autorepeat: byte alto = retardo inicial, bajo = cadencia)
|       y test de `$106ECA` (modo de input: 0 = vivo, 1 = grabación/replay,
|       2 = deshabilitado).
|     - Input_Poll_LiveMode_025066: lee los puertos BIOS `$10FD96` (P1) y
|       `$10FD9C` (P2) y actualiza los dos bloques de 6 B `$106EB0`/`$106EB6`
|       {prev, cur, pressed(rising), repeat_out, repeat_timer}: rising edge con
|       `eor;and`, carga del timer con `$106EC8` (inicial) o `$106EC9`
|       (cadencia) — el mismo bloque que consume Input_RisingEdgeSnapshot_05CC0E.
|       Tras su `rts` hay un word de datos (`Input_ReplayFlag_025118`, leído
|       con `tst.b $25118.l`: 0 = reproducir, !=0 = grabar) y continúa el modo
|       1 (fuente = bytes `$106EBC/BD` en vez de los puertos) y el modo 2
|       (fuente = 0: input muerto).
|     - InputRec_*: codificador RLE de input para el demo del attract. Buffer
|       `$106EBE` (ptr, 512 B máx: `cmpi.w #$200`), write pos `$106EC4`,
|       pos del último "cambio" `$106EC2`, contador de run `$106EC6`.
|       RecordFrame: si el run llega a $0F (o $FF si ya hubo un bit guardado
|       en este byte) hace FlushRun; luego, por cada bit que cambia en los 16
|       bits P1|P2<<8, StoreBitToggle guarda {bit_idx<<4 | run_lo} y al
|       llenarse el buffer pasa a modo 2. PlaybackFrame: ReadNextRun decodifica
|       el run (nibble alto = bit a conmutar con `bchg` en `$106EBC/BD`).
|
|  2) $02545C..$0256B0  Descriptores de slot de jugador (datos):
|     - PlayerSlotIndex_02545c: 6 words 0..5.
|     - PlayerSlotDesc_*: 32 registros de 16 B {tmpl.l, entity.l, task.l,
|       flags.w, $FFFF}: P1 → entidad `$100440`, task `$10E200`, tmpl `$3364A`
|       (`$0400` = vacío); P2 → `$1004E0`, `$10E206`, tmpl `$336DC`. Los
|       registros se seleccionan en HUD_Task_Init con `$106ECE<<4` (escena) y
|       la fila 2/3 "Idle" cuando `$106ECE==$FF`.
|     - 18 punteros a HUD_Msg_*/HUD_Draw* (`$25668..$256B0`), indexados por
|       jugador (`$70(a6)<<2`) desde HUD_CallPerPlayer_IfMode2.
|
|  3) $025706..$026752  Tarea HUD por jugador (a6 = task; `$70(a6)` = índice
|     de jugador 0/1, `$72/$78(a6)` = ptrs a slot/entidad, `$77(a6)` = vidas
|     iniciales desde `$10FD88`, `$84/$85(a6)` = timers de parpadeo,
|     `$86/$8A(a6)` = barra de vida interpolada (16.16), `$8E/$90(a6)` =
|     munición/bombas cacheadas, `$92/$93(a6)` = estado de barra / "en Slug").
|     Máquina de estados instalada en `(a6)`:
|       HUD_Task_StartP1/P2 → HUD_State_InsertCoin (mensaje INSERT COIN; si
|       `$1E28` (créditos) hay → PushStartBlink y acepta START vía
|       `$10FDB6[p]`) → HUD_State_WaitPlayerSpawn (espera `(a0)==$400` del
|       slot) → HUD_State_BindPlayer (enlaza slot/entidad, `$66(a2)=1`,
|       Rank_DelayByDifficulty → `$106E92`) → HUD_State_PlayerDeath (al
|       morir: borra, elige "GAME OVER"/respawn por `$106F2A`/Player_GetEntity)
|       → HUD_State_Respawn | HUD_State_GameOverEntry (Spawn_MarkPending de
|       `$9B872` + Credits/CONTINUE) → HUD_State_ContinueCountdown (dígitos vía
|       Sub_Divide10_047656) → HUD_State_GameOverFinal/Done (vuelve a INSERT
|       COIN o, si `$106ED0>=6`, reinicia por `$5B6`/`$400`).
|     Dibujado al fix layer con `movem.w dX-dY,$3C0000` (VRAM addr + tile):
|       HUD_DrawPlayerLabels ("1UP=", "2UP=" de `$2785B8/BE`, paleta $53xx),
|       HUD_DrawLifeBarFrame/HUD_DrawLifeBar (tiles $A5/$A6/$AE/$AF, 6 celdas,
|       interpolación con amortiguación `d4 = d4/4 + 2*delta`), HUD_DrawBomb-
|       Gauge_P1/P2 (14 celdas, tiles $238A lleno / $2320 vacío, desde
|       `$106F4C/4E`), HUD_DrawScore (7 dígitos BCD desde `$106E94`/`$106E9C`,
|       ceros a la izquierda como espacio), HUD_DrawAmmoAndBombs (3 dígitos de
|       munición con Player_GetAmmoOrFFFF, "∞" = tiles $E3F2.. cuando $FFFF;
|       2 dígitos de bombas con Player_GetBombs o Slug_GetGauge si va en Slug;
|       parpadeo alternando paletas $E3xx/$F3xx), HUD_Msg_* (11 chars de
|       `$2785C4..` vía ThunkTarget_0477fc: INSERT COIN / PUSH START /
|       CONTINUE / GAME OVER / PLEASE WAIT) y HUD_ClearMsgRow.
|     Credits_BCDPtrTable_02674a + código: lee créditos BCD `$1081BF/C0` o,
|       en modo BIOS (`$10FD82`), inicializa `$10FDB0..B3` y llama al BIOS
|       `$C00450` (CREDIT_CHECK) para decidir si hay crédito (CCR C).
|
|  4) $0267F4..$027400  Motor de movimiento + colisión de entidades
|     (a5 = `$108080`; scratch -$1154..-$1143(a5) = {dx_hi, dy_hi, x_sub,
|     y_sub, probe_x, probe_y, saved_sub_x/y}):
|     - ClampVelocity (±$800), Entity_IntegrateVelocity (`$28/$2A += $2C/$2E`).
|     - Entity_MoveX_WallStop / Entity_MoveXY_Probe: aplican la velocidad con
|       acumulador subpíxel de 8 bits, anulan dx contra paredes (`$69(a6)`
|       bits 0/1), suman el scroll de cámara `$106F6C` en misión 1 (código
|       muerto tras `bra.w`), y en la escena 3 frenan a los jugadores
|       (`a6==$100440/$1004E0`) en `x>=$1F0`. Gravedad por `$34(a6)`:
|       0 = sin, $80/$400/$1000 = perfiles (con `trap #15` en la rama
|       imposible = assert del compilador original).
|     - Entity_MoveAndCollide_A/B/C: tres variantes del bucle de colisión con
|       el mapa: Handler_ApplyCameraGlobals, Entity_RestoreTransformSetC_027d94,
|       Trail_FindByKeyRange/FindNearest (`$998CA/$9993C`) para localizar el
|       tile bajo la entidad (tabla `$278BA8`, tile `$12(a3)` → `$106F31`),
|       probes de pared Sub_00027E9C/Sub_00027E7E (huecos futuros), suelo con
|       Entity_FloorProbe (offsets Right/Left/None según dx) y commit en
|       Entity_CommitMove* (posición `-$1148/-$1146(a5)`, bit 3/5 de `$5A(a6)`
|       = "en suelo"/"en rampa").
|     - Entity_SaveRegs_0273fc: `movem.l d3-d6/a1,-(a7)` de entrada a la
|       función que sigue en `$027400` (hueco futuro).
|
|  B. EVIDENCIAS
|  - Strings ASCII en `$2785B8..$278600` (banco alto): "1UP=", "2UP=",
|    "INSERT COIN", "PUSH START ", "CONTINUE   ", "GAME OVER  ", "PLEASE WAIT".
|  - `$3C0000` = VRAM addr/data del LSPC; `movem.w dX-dY` escribe addr+tile.
|  - `$10FD96/$10FD9C` = BIOS P1/P2 input; `$10FDB6/B7` = máscaras START
|    (Wave BB); `$10FD82` = BIOS_PLAYER_MOD1; `$1081BF/C0` = créditos BCD.
|  - Player_GetAmmoOrFFFF_032fba / Player_GetBombs_032fce / Slug_GetGauge_02a2ac
|    ya nombrados en waves anteriores confirman el HUD.
|  - Cluster `$027xxx` (Entity_Probe*/RestoreTransform*) con la misma scratch
|    a5-relativa `-$1148..-$1143` (Wave T/Z).
|
|  C. HIPÓTESIS (nombres provisionales)
|  - `Input_ReplayFlag_025118`: la polaridad (0 = replay) se deduce del
|    `beq → PlaybackFrame`; no se ha localizado quién lo escribe (ROM = 0).
|  - `HUD_IsSceneBCD_0266cc`: devuelve C=1 si `$106ECE ∈ {$B,$C,$D}`
|    (escenas de bonus/final) — nombre provisional.
|  - `Entity_MoveAndCollide_A/B/C`: difieren en el campo de gravedad usado
|    (`$34` vs `$64(a6)`) y en el test de `$100000` bit 0 (debug); falta
|    el análisis fino de cada rama.
|
|  D. CAMPOS (a6) usados en este archivo
|    (a6)      handler;  $08 parent; $0C sibling/other-task; $10 rank
|    $13 flags (bit4/5 = sin colisión pared/suelo, bit6/7)
|    $20/$21 flags locales HUD;  $22/$24 x/y px;  $28/$2A vel;  $2C/$2E accel
|    $34 gravedad;  $44 timer;  $5A/$5B estado suelo;  $64 gravedad alt
|    $69/$6B wall flags;  $70 player idx;  $72 slot;  $76 cache gauge
|    $77 vidas;  $78 entity;  $7C handler guardado;  $82 ack guardado
|    $84/$85 blink;  $86 bar.l;  $8A bar_vel.l;  $8E ammo;  $90 bombs
|    $92 bar state;  $93 in-slug;  $98 player idx (entidad spawn)
|
|  E. HELPERS EXTERNOS
|    ThunkTarget_0005fe / 0004ae / 0006fe (scheduler add / spawn template)
|    FUN_000005B6 (soft reset), `$1E28` (CREDIT? → C), `$1E4C/$1E56` (sonido)
|    Sub_Divide10_047656, ThunkTarget_0477fc (print 11 chars),
|    ThunkTarget_05dad8/05da56/05da9c (blits fix), Player_GetEntity_05e3a2,
|    Player_IncCounterAt7_051A86, Clear8Bytes_05180c, Spawn_MarkPending_04498E,
|    Rank_DelayByDifficulty_079970, Handler_ApplyCameraGlobals_044182,
|    Trail_FindByKeyRange_0998ca, Trail_FindNearest_09993c,
|    Slug_IsRiddenByPlayer_02ac0e, Slug_TestBit4Field8D_02abd2,
|    Slug_TestBit5Field8D_02a276, Entity_RestoreTransformSetC_027d94,
|    Sub_00027E7E/Sub_00027E9C/Sub_0002800E/Sub_00028074 (huecos futuros),
|    PcThunkTarget_0281c8, BIOS `$C00450`.
|
|  F. ESTADO
|  88/88 entradas byte-exactas (verify + matcher CI). Zona `$024E10..$027400`
|  al 100 %. Pendiente: análisis fino de Entity_MoveAndCollide_A/B/C y de
|  los huecos `$027400..$028200`.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_024e1c  @ $024E1C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_024e1c, "ax", @progbits
        .global ChildRank_CmpByte10_024e1c
ChildRank_CmpByte10_024e1c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_024e32                    | +00c

| ----------------------------------------------------------------------------
|  PlayerCtx_ResetAndSetRepeat1002_024fb8  @ $024FB8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerCtx_ResetAndSetRepeat1002_024fb8, "ax", @progbits
        .global PlayerCtx_ResetAndSetRepeat1002_024fb8
PlayerCtx_ResetAndSetRepeat1002_024fb8:
        moveq   #-1,d0                          | +000
        bsr.w   PlayerCtx_ResetTwoBlocks_024FEC | +002
        move.w  #0x1002,0x106ec8.l              | +006
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  PlayerCtx_SetRepeatRate_024fc8  @ $024FC8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerCtx_SetRepeatRate_024fc8, "ax", @progbits
        .global PlayerCtx_SetRepeatRate_024fc8
PlayerCtx_SetRepeatRate_024fc8:
        move.w  d0,0x106ec8.l                   | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  PlayerCtx_IsModeSingle_024fd0  @ $024FD0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerCtx_IsModeSingle_024fd0, "ax", @progbits
        .global PlayerCtx_IsModeSingle_024fd0
PlayerCtx_IsModeSingle_024fd0:
        cmpi.b  #0x2,0x106eca.l                 | +000
        bne.w   ClearC_024fe6                   | +008

| ----------------------------------------------------------------------------
|  PlayerCtx_ResetTwoBlocks_Thunk_024fe2  @ $024FE2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerCtx_ResetTwoBlocks_Thunk_024fe2, "ax", @progbits
        .global PlayerCtx_ResetTwoBlocks_Thunk_024fe2
PlayerCtx_ResetTwoBlocks_Thunk_024fe2:
        bra.w   PlayerCtx_ResetTwoBlocks_024FEC | +000

| ----------------------------------------------------------------------------
|  Input_Poll_LiveMode_025066  @ $025066  (178 B)
| ----------------------------------------------------------------------------
        .section .text.Input_Poll_LiveMode_025066, "ax", @progbits
        .global Input_Poll_LiveMode_025066
Input_Poll_LiveMode_025066:
        cmpi.b  #0x0,0x106eca.l                 | +000
        bne.w   Input_ReplayFlag_025118__L02511a | +008
        move.b  0x10fd96.l,d0                   | +00c
        lea     0x106eb0.l,a0                   | +012
        move.b  0x2(a0),d1                      | +018
        move.b  d1,0x1(a0)                      | +01c
        move.b  d0,0x2(a0)                      | +020
        move.b  d0,d2                           | +024
        eor.b   d2,d1                           | +026
        and.b   d2,d1                           | +028
        move.b  d1,0x3(a0)                      | +02a
        beq.w   .L0250a8                        | +02e
        move.b  d1,0x4(a0)                      | +032
        move.b  0x106ec8.l,0x5(a0)              | +036
        bra.w   .L0250c4                        | +03e
.L0250a8:
        subq.b  #0x1,0x5(a0)                    | +042
        bne.w   .L0250c0                        | +046
        move.b  d2,0x4(a0)                      | +04a
        move.b  0x106ec9.l,0x5(a0)              | +04e
        bra.w   .L0250c4                        | +056
.L0250c0:
        clr.b   0x4(a0)                         | +05a
.L0250c4:
        move.b  0x10fd9c.l,d0                   | +05e
        lea     0x106eb6.l,a0                   | +064
        move.b  0x2(a0),d1                      | +06a
        move.b  d1,0x1(a0)                      | +06e
        move.b  d0,0x2(a0)                      | +072
        move.b  d0,d2                           | +076
        eor.b   d2,d1                           | +078
        and.b   d2,d1                           | +07a
        move.b  d1,0x3(a0)                      | +07c
        beq.w   .L0250fa                        | +080
        move.b  d1,0x4(a0)                      | +084
        move.b  0x106ec8.l,0x5(a0)              | +088
        bra.w   .L025116                        | +090
.L0250fa:
        subq.b  #0x1,0x5(a0)                    | +094
        bne.w   .L025112                        | +098
        move.b  d2,0x4(a0)                      | +09c
        move.b  0x106ec9.l,0x5(a0)              | +0a0
        bra.w   .L025116                        | +0a8
.L025112:
        clr.b   0x4(a0)                         | +0ac
.L025116:
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  Input_ReplayFlag_025118  @ $025118  (362 B)
| ----------------------------------------------------------------------------
        .section .text.Input_ReplayFlag_025118, "ax", @progbits
        .global Input_ReplayFlag_025118
Input_ReplayFlag_025118:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .global Input_ReplayFlag_025118__L02511a
Input_ReplayFlag_025118__L02511a:
.L02511a:
        cmpi.b  #0x1,0x106eca.l                 | +002
        bne.w   .L0251e4                        | +00a
        tst.b   0x25118.l                       | +00e
        beq.w   .L025138                        | +014
        bsr.w   InputRec_RecordFrame_025282     | +018
        bra.w   .L02513c                        | +01c
.L025138:
        bsr.w   InputRec_PlaybackFrame_02531e   | +020
.L02513c:
        move.b  0x106ebc.l,d0                   | +024
        lea     0x106eb0.l,a0                   | +02a
        move.b  0x2(a0),d1                      | +030
        move.b  d1,0x1(a0)                      | +034
        move.b  d0,0x2(a0)                      | +038
        move.b  d0,d2                           | +03c
        eor.b   d2,d1                           | +03e
        and.b   d2,d1                           | +040
        move.b  d1,0x3(a0)                      | +042
        beq.w   .L025172                        | +046
        move.b  d1,0x4(a0)                      | +04a
        move.b  0x106ec8.l,0x5(a0)              | +04e
        bra.w   .L02518e                        | +056
.L025172:
        subq.b  #0x1,0x5(a0)                    | +05a
        bne.w   .L02518a                        | +05e
        move.b  d2,0x4(a0)                      | +062
        move.b  0x106ec9.l,0x5(a0)              | +066
        bra.w   .L02518e                        | +06e
.L02518a:
        clr.b   0x4(a0)                         | +072
.L02518e:
        move.b  0x106ebd.l,d0                   | +076
        lea     0x106eb6.l,a0                   | +07c
        move.b  0x2(a0),d1                      | +082
        move.b  d1,0x1(a0)                      | +086
        move.b  d0,0x2(a0)                      | +08a
        move.b  d0,d2                           | +08e
        eor.b   d2,d1                           | +090
        and.b   d2,d1                           | +092
        move.b  d1,0x3(a0)                      | +094
        beq.w   .L0251c4                        | +098
        move.b  d1,0x4(a0)                      | +09c
        move.b  0x106ec8.l,0x5(a0)              | +0a0
        bra.w   .L0251e0                        | +0a8
.L0251c4:
        subq.b  #0x1,0x5(a0)                    | +0ac
        bne.w   .L0251dc                        | +0b0
        move.b  d2,0x4(a0)                      | +0b4
        move.b  0x106ec9.l,0x5(a0)              | +0b8
        bra.w   .L0251e0                        | +0c0
.L0251dc:
        clr.b   0x4(a0)                         | +0c4
.L0251e0:
        bra.w   .L025280                        | +0c8
.L0251e4:
        clr.b   d0                              | +0cc
        lea     0x106eb0.l,a0                   | +0ce
        move.b  0x2(a0),d1                      | +0d4
        move.b  d1,0x1(a0)                      | +0d8
        move.b  d0,0x2(a0)                      | +0dc
        move.b  d0,d2                           | +0e0
        eor.b   d2,d1                           | +0e2
        and.b   d2,d1                           | +0e4
        move.b  d1,0x3(a0)                      | +0e6
        beq.w   .L025216                        | +0ea
        move.b  d1,0x4(a0)                      | +0ee
        move.b  0x106ec8.l,0x5(a0)              | +0f2
        bra.w   .L025232                        | +0fa
.L025216:
        subq.b  #0x1,0x5(a0)                    | +0fe
        bne.w   .L02522e                        | +102
        move.b  d2,0x4(a0)                      | +106
        move.b  0x106ec9.l,0x5(a0)              | +10a
        bra.w   .L025232                        | +112
.L02522e:
        clr.b   0x4(a0)                         | +116
.L025232:
        clr.b   d0                              | +11a
        lea     0x106eb6.l,a0                   | +11c
        move.b  0x2(a0),d1                      | +122
        move.b  d1,0x1(a0)                      | +126
        move.b  d0,0x2(a0)                      | +12a
        move.b  d0,d2                           | +12e
        eor.b   d2,d1                           | +130
        and.b   d2,d1                           | +132
        move.b  d1,0x3(a0)                      | +134
        beq.w   .L025264                        | +138
        move.b  d1,0x4(a0)                      | +13c
        move.b  0x106ec8.l,0x5(a0)              | +140
        bra.w   .L025280                        | +148
.L025264:
        subq.b  #0x1,0x5(a0)                    | +14c
        bne.w   .L02527c                        | +150
        move.b  d2,0x4(a0)                      | +154
        move.b  0x106ec9.l,0x5(a0)              | +158
        bra.w   .L025280                        | +160
.L02527c:
        clr.b   0x4(a0)                         | +164
.L025280:
        rts                                     | +168

| ----------------------------------------------------------------------------
|  InputRec_RecordFrame_025282  @ $025282  (156 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_RecordFrame_025282, "ax", @progbits
        .global InputRec_RecordFrame_025282
InputRec_RecordFrame_025282:
        movea.l 0x106ebe.l,a0                   | +000
        move.b  #0xf,d1                         | +006
        move.w  0x106ec2.l,d0                   | +00a
        cmp.w   0x106ec4.l,d0                   | +010
        beq.w   .L0252a0                        | +016
        move.b  #0xff,d1                        | +01a
.L0252a0:
        cmp.b   0x106ec6.l,d1                   | +01e
        bhi.w   .L0252ae                        | +024
        jsr     InputRec_FlushRun_025366(pc)    | +028
.L0252ae:
        clr.w   d0                              | +02c
        clr.w   d1                              | +02e
        move.b  0x10fd96.l,d0                   | +030
        move.b  0x106ebc.l,d2                   | +036
        move.b  d0,0x106ebc.l                   | +03c
        eor.b   d2,d0                           | +042
        move.b  0x10fd9c.l,d1                   | +044
        move.b  0x106ebd.l,d2                   | +04a
        move.b  d1,0x106ebd.l                   | +050
        eor.b   d2,d1                           | +056
        lsl.w   #0x8,d1                         | +058
        or.w    d1,d0                           | +05a
        move.w  #0xf,d6                         | +05c
.L0252e2:
        move.l  d6,-(a7)                        | +060
        lsl.w   #0x1,d0                         | +062
        bcc.w   .L0252f4                        | +064
        move.w  d6,d1                           | +068
        move.l  d0,-(a7)                        | +06a
        jsr     InputRec_StoreBitToggle_025390(pc) | +06c
        move.l  (a7)+,d0                        | +070
.L0252f4:
        move.l  (a7)+,d6                        | +072
        dbra    d6,.L0252e2                     | +074
        addq.b  #0x1,0x106ec6.l                 | +078
        cmpi.w  #0x200,0x106ec4.l               | +07e
        bcs.w   .L02531c                        | +086
        clr.w   d0                              | +08a
        move.w  d0,0x106ebc.l                   | +08c
        move.b  #0x2,0x106eca.l                 | +092
.L02531c:
        rts                                     | +09a

| ----------------------------------------------------------------------------
|  InputRec_PlaybackFrame_02531e  @ $02531E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_PlaybackFrame_02531e, "ax", @progbits
        .global InputRec_PlaybackFrame_02531e
InputRec_PlaybackFrame_02531e:
        movea.l 0x106ebe.l,a0                   | +000
        jsr     InputRec_ReadNextRun_0253be(pc) | +006
        bcc.w   .L02535e                        | +00a
        moveq   #0,d1                           | +00e
        andi.w  #0xf,d0                         | +010
        btst    #0x3,d0                         | +014
        bne.w   .L02534a                        | +018
        move.b  0x106ebc.l,d1                   | +01c
        bchg    d0,d1                           | +022
        move.b  d1,0x106ebc.l                   | +024
        bra.b   InputRec_PlaybackFrame_02531e   | +02a
.L02534a:
        andi.w  #0x7,d0                         | +02c
        move.b  0x106ebd.l,d1                   | +030
        bchg    d0,d1                           | +036
        move.b  d1,0x106ebd.l                   | +038
        bra.b   InputRec_PlaybackFrame_02531e   | +03e
.L02535e:
        addq.b  #0x1,0x106ec6.l                 | +040
        rts                                     | +046

| ----------------------------------------------------------------------------
|  InputRec_FlushRun_025366  @ $025366  (42 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_FlushRun_025366, "ax", @progbits
        .global InputRec_FlushRun_025366
InputRec_FlushRun_025366:
        move.w  0x106ec4.l,d0                   | +000
        cmpi.w  #0x200,d0                       | +006
        bcc.w   .L02538e                        | +00a
        move.b  0x106ec6.l,d2                   | +00e
        move.b  d2,(a0,d0.w)                    | +014
        move.b  #0x0,0x106ec6.l                 | +018
        addq.w  #0x1,d0                         | +020
        move.w  d0,0x106ec4.l                   | +022
.L02538e:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  InputRec_StoreBitToggle_025390  @ $025390  (46 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_StoreBitToggle_025390, "ax", @progbits
        .global InputRec_StoreBitToggle_025390
InputRec_StoreBitToggle_025390:
        jsr     InputRec_FlushRun_025366(pc)    | +000
        move.w  0x106ec2.l,d0                   | +004
        cmpi.w  #0x200,d0                       | +00a
        bcc.w   .L0253bc                        | +00e
        lsl.b   #0x4,d1                         | +012
        move.b  (a0,d0.w),d2                    | +014
        andi.b  #0xf,d2                         | +018
        or.b    d2,d1                           | +01c
        move.b  d1,(a0,d0.w)                    | +01e
        move.w  0x106ec4.l,0x106ec2.l           | +022
.L0253bc:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  InputRec_ReadNextRun_0253be  @ $0253BE  (98 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_ReadNextRun_0253be, "ax", @progbits
        .global InputRec_ReadNextRun_0253be
InputRec_ReadNextRun_0253be:
        move.w  0x106ec4.l,d0                   | +000
        cmpi.w  #0x200,d0                       | +006
        bcc.w   InputRec_SetModeDisabled_025432 | +00a
        move.b  (a0,d0.w),d1                    | +00e
        move.b  #0xff,d2                        | +012
        cmp.w   0x106ec2.l,d0                   | +016
        bne.w   .L0253e4                        | +01c
        move.b  #0xf,d2                         | +020
        and.b   d2,d1                           | +024
.L0253e4:
        cmp.b   0x106ec6.l,d1                   | +026
        bne.w   InputRec_SetReadPos_025426      | +02c
        move.b  #0x0,0x106ec6.l                 | +030
        addq.w  #0x1,d0                         | +038
        cmp.b   d2,d1                           | +03a
        bne.w   .L025408                        | +03c
        tst.b   (a0,d0.w)                       | +040
        bne.w   InputRec_SetReadPos_025426      | +044
        addq.w  #0x1,d0                         | +048
.L025408:
        move.w  d0,0x106ec4.l                   | +04a
        move.w  0x106ec2.l,d1                   | +050
        move.w  d0,0x106ec2.l                   | +056
        move.b  (a0,d1.w),d0                    | +05c
        lsr.b   #0x4,d0                         | +060

| ----------------------------------------------------------------------------
|  InputRec_SetReadPos_025426  @ $025426  (6 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_SetReadPos_025426, "ax", @progbits
        .global InputRec_SetReadPos_025426
InputRec_SetReadPos_025426:
        move.w  d0,0x106ec4.l                   | +000

| ----------------------------------------------------------------------------
|  InputRec_SetModeDisabled_025432  @ $025432  (8 B)
| ----------------------------------------------------------------------------
        .section .text.InputRec_SetModeDisabled_025432, "ax", @progbits
        .global InputRec_SetModeDisabled_025432
InputRec_SetModeDisabled_025432:
        move.b  #0x2,0x106eca.l                 | +000

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_025440  @ $025440  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_025440, "ax", @progbits
        .global ChildRank_CmpByte10_025440
ChildRank_CmpByte10_025440:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_025456                    | +00c

| ----------------------------------------------------------------------------
|  PlayerSlotIndex_02545c  @ $02545C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlotIndex_02545c, "ax", @progbits
        .global PlayerSlotIndex_02545c
PlayerSlotIndex_02545c:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +014  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +016  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +018  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +020  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +028  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerSlotDesc_Idle_025488  @ $025488  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlotDesc_Idle_025488, "ax", @progbits
        .global PlayerSlotDesc_Idle_025488
PlayerSlotDesc_Idle_025488:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerSlotDesc_Idle2_025498  @ $025498  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlotDesc_Idle2_025498, "ax", @progbits
        .global PlayerSlotDesc_Idle2_025498
PlayerSlotDesc_Idle2_025498:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerSlotDesc_P1Rows_0254a8  @ $0254A8  (224 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlotDesc_P1Rows_0254a8, "ax", @progbits
        .global PlayerSlotDesc_P1Rows_0254a8
PlayerSlotDesc_P1Rows_0254a8:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +010  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +020  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +028  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +030  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +038  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +040  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +048  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +058  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +060  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +068  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +070  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +078  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +080  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +088  (dato / opcode no decodificado)
        .dc.w   0xe200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0de  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerSlotDesc_P2Rows_025588  @ $025588  (382 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerSlotDesc_P2Rows_025588, "ax", @progbits
        .global PlayerSlotDesc_P2Rows_025588
PlayerSlotDesc_P2Rows_025588:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +010  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +014  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +020  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +024  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +028  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +030  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +034  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +038  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +040  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +044  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +048  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +054  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +058  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +060  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +064  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +068  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +070  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +074  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +078  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +080  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +084  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +088  (dato / opcode no decodificado)
        .dc.w   0xe206                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x01ff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +094  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x00ff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0de  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025668
PlayerSlotDesc_P2Rows_025588__L025668:
.L025668:
        .dc.w   0x0002                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x60c4                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x60cc                        | +0e6  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025670
PlayerSlotDesc_P2Rows_025588__L025670:
.L025670:
        .dc.w   0x0002                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x5ffc                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x6010                        | +0ee  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025678
PlayerSlotDesc_P2Rows_025588__L025678:
.L025678:
        .dc.w   0x0002                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x6024                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x6038                        | +0f6  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025680
PlayerSlotDesc_P2Rows_025588__L025680:
.L025680:
        .dc.w   0x0002                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x609c                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x60b0                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +100  (dato / opcode no decodificado)
        .dc.w   0x604c                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +104  (dato / opcode no decodificado)
        .dc.w   0x6060                        | +106  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025690
PlayerSlotDesc_P2Rows_025588__L025690:
.L025690:
        .dc.w   0x0002                        | +108  (dato / opcode no decodificado)
        .dc.w   0x6074                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x6088                        | +10e  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L025698
PlayerSlotDesc_P2Rows_025588__L025698:
.L025698:
        .dc.w   0x0002                        | +110  (dato / opcode no decodificado)
        .dc.w   0x614e                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +114  (dato / opcode no decodificado)
        .dc.w   0x6156                        | +116  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L0256a0
PlayerSlotDesc_P2Rows_025588__L0256a0:
.L0256a0:
        .dc.w   0x0002                        | +118  (dato / opcode no decodificado)
        .dc.w   0x630e                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x6372                        | +11e  (dato / opcode no decodificado)
        .global PlayerSlotDesc_P2Rows_025588__L0256a8
PlayerSlotDesc_P2Rows_025588__L0256a8:
.L0256a8:
        .dc.w   0x0002                        | +120  (dato / opcode no decodificado)
        .dc.w   0x63f2                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +124  (dato / opcode no decodificado)
        .dc.w   0x63d6                        | +126  (dato / opcode no decodificado)
        lea     0x10fd84.l,a3                   | +128
        move.b  0x4(a3),d1                      | +12e
        move.b  d1,0x77(a6)                     | +132
        jsr     0x25ed6.l                       | +136
        cmpi.b  #0x2,0x10fdaf.l                 | +13c
        bne.w   .L0256e2                        | +144
        jsr     0x25f02.l                       | +148
        jsr     0x260e2.l                       | +14e
        jsr     0x2614e.l                       | +154
.L0256e2:
        jsr     0x2630e.l                       | +15a
        lea     PlayerSlotDesc_P1Rows_0254a8(pc),a0 | +160
        moveq   #0,d0                           | +164
        move.b  0x106ece.l,d0                   | +166
        cmpi.b  #0xff,d0                        | +16c
        bne.w   .L025702                        | +170
        lea     PlayerSlotDesc_Idle_025488(pc),a0 | +174
        moveq   #0,d0                           | +178
.L025702:
        lsl.w   #0x4,d0                         | +17a
        rts                                     | +17c

| ----------------------------------------------------------------------------
|  HUD_Task_Init_025706  @ $025706  (86 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Task_Init_025706, "ax", @progbits
        .global HUD_Task_Init_025706
HUD_Task_Init_025706:
        lea     0x10fd84.l,a3                   | +000
        move.b  0x4(a3),d1                      | +006
        move.b  d1,0x77(a6)                     | +00a
        jsr     0x25ed6.l                       | +00e
        cmpi.b  #0x2,0x10fdaf.l                 | +014
        bne.w   .L025738                        | +01c
        jsr     0x25f4a.l                       | +020
        jsr     0x260ea.l                       | +026
        jsr     0x26156.l                       | +02c
.L025738:
        jsr     0x26372.l                       | +032
        lea     PlayerSlotDesc_P2Rows_025588(pc),a0 | +038
        moveq   #0,d0                           | +03c
        move.b  0x106ece.l,d0                   | +03e
        cmpi.b  #0xff,d0                        | +044
        bne.w   .L025758                        | +048
        lea     PlayerSlotDesc_Idle2_025498(pc),a0 | +04c
        clr.w   d0                              | +050
.L025758:
        lsl.w   #0x4,d0                         | +052
        rts                                     | +054

| ----------------------------------------------------------------------------
|  HUD_Task_StartP1_02575c  @ $02575C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Task_StartP1_02575c, "ax", @progbits
        .global HUD_Task_StartP1_02575c
HUD_Task_StartP1_02575c:
        move.w  #0x0,0x70(a6)                   | +000
        jmp     HUD_StartHandlerTable_025770__L025780(pc) | +006

| ----------------------------------------------------------------------------
|  HUD_Task_StartP2_025766  @ $025766  (10 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Task_StartP2_025766, "ax", @progbits
        .global HUD_Task_StartP2_025766
HUD_Task_StartP2_025766:
        move.w  #0x1,0x70(a6)                   | +000
        jmp     HUD_StartHandlerTable_025770__L025780(pc) | +006

| ----------------------------------------------------------------------------
|  HUD_StartHandlerTable_025770  @ $025770  (116 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_StartHandlerTable_025770, "ax", @progbits
        .global HUD_StartHandlerTable_025770
HUD_StartHandlerTable_025770:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x59b8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x57ec                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x59b8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x57ec                        | +00e  (dato / opcode no decodificado)
        .global HUD_StartHandlerTable_025770__L025780
HUD_StartHandlerTable_025770__L025780:
.L025780:
        lea     HUD_State_InsertCoin_0257ec(pc),a1 | +010
        move.l  a1,(a6)                         | +014
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +016
        cmpi.b  #0x1,d0                         | +01a
        bne.w   .L025798                        | +01e
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L025798:
        cmpi.b  #0x1,0x10fdaf.l                 | +028
        bne.w   .L0257dc                        | +030
        cmpi.b  #0x0,d0                         | +034
        bne.w   .L0257dc                        | +038
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
        lea     HUD_StartHandlerTable_025770(pc),a0 | +042
        move.b  0x10007b.l,d0                   | +046
        andi.l  #0x1,d0                         | +04c
        lsl.w   #0x3,d0                         | +052
        move.w  0x70(a6),d1                     | +054
        lsl.w   #0x2,d1                         | +058
        add.w   d0,d1                           | +05a
        movea.l (a0,d1.w),a1                    | +05c
        cmpa.l  #0xffffffff,a1                  | +060
        beq.w   .L0257dc                        | +066
        move.l  a1,(a6)                         | +06a
.L0257dc:
        lea     HUD_PerPlayerPtrTable_025922__L02594a(pc),a0 | +06c
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +070

| ----------------------------------------------------------------------------
|  HUD_State_InsertCoin_0257ec  @ $0257EC  (140 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_InsertCoin_0257ec, "ax", @progbits
        .global HUD_State_InsertCoin_0257ec
HUD_State_InsertCoin_0257ec:
        lea     PlayerSlotDesc_P2Rows_025588__L025670(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +004
        lea     .L0257fa(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L0257fa:
        jsr     0x1e28.l                        | +00e
        bcs.w   .L025824                        | +014
        lea     HUD_State_PushStartBlink_025e48(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
        move.l  #0x257ec,0x7c(a6)               | +01e
        lea     0x106ecc.l,a0                   | +026
        adda.w  0x70(a6),a0                     | +02c
        move.b  (a0),0x82(a6)                   | +030
        move.b  #0x1,(a0)                       | +034
.L025824:
        move.b  0x106f28.l,d0                   | +038
        andi.b  #0x1f,d0                        | +03e
        bne.w   .L02583e                        | +042
        lea     PlayerSlotDesc_P2Rows_025588__L025670(pc),a0 | +046
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +04a
        bra.w   .L02584e                        | +04e
.L02583e:
        cmpi.b  #0x18,d0                        | +052
        bne.w   .L02584e                        | +056
        lea     PlayerSlotDesc_P2Rows_025588__L025668(pc),a0 | +05a
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +05e
.L02584e:
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +062
        cmpi.b  #0x1,d0                         | +066
        bne.w   .L025866                        | +06a
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
        clr.b   0x20(a6)                        | +074
        rts                                     | +078
.L025866:
        move.b  #0x1,d2                         | +07a
        move.w  0x70(a6),d1                     | +07e
        jsr     0x26752.l                       | +082
        bcc.w   Stub_00025880                   | +088

| ----------------------------------------------------------------------------
|  HUD_State_Continue_025882  @ $025882  (152 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_Continue_025882, "ax", @progbits
        .global HUD_State_Continue_025882
HUD_State_Continue_025882:
        lea     PlayerSlotDesc_P2Rows_025588__L025678(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +004
        lea     .L025890(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L025890:
        jsr     0x1e28.l                        | +00e
        bcs.w   .L0258ba                        | +014
        lea     HUD_State_PushStartBlink_025e48(pc),a1 | +018
        move.l  a1,(a6)                         | +01c
        move.l  #0x25882,0x7c(a6)               | +01e
        lea     0x106ecc.l,a0                   | +026
        adda.w  0x70(a6),a0                     | +02c
        move.b  (a0),0x82(a6)                   | +030
        move.b  #0x1,(a0)                       | +034
.L0258ba:
        move.b  0x106f28.l,d0                   | +038
        andi.b  #0x1f,d0                        | +03e
        bne.w   .L0258d4                        | +042
        lea     PlayerSlotDesc_P2Rows_025588__L025678(pc),a0 | +046
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +04a
        bra.w   .L0258e4                        | +04e
.L0258d4:
        cmpi.b  #0x18,d0                        | +052
        bne.w   .L0258e4                        | +056
        lea     PlayerSlotDesc_P2Rows_025588__L025668(pc),a0 | +05a
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +05e
.L0258e4:
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +062
        cmpi.b  #0x1,d0                         | +066
        bne.w   .L025908                        | +06a
        lea     HUD_PerPlayerPtrTable_025922__L02594a(pc),a0 | +06e
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +072
        jsr     0x5180c.l                       | +076
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
        bra.w   SetHandlerRts_025920            | +082
.L025908:
        move.b  #0x1,d2                         | +086
        move.w  0x70(a6),d1                     | +08a
        jsr     0x26752.l                       | +08e
        bcs.w   SetHandlerRts_025920            | +094

| ----------------------------------------------------------------------------
|  HUD_PerPlayerPtrTable_025922  @ $025922  (48 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_PerPlayerPtrTable_025922, "ax", @progbits
        .global HUD_PerPlayerPtrTable_025922
HUD_PerPlayerPtrTable_025922:
        .dc.w   0x0009                        | +000  (dato / opcode no decodificado)
        .dc.w   0x7a48                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +004  (dato / opcode no decodificado)
        .dc.w   0x7a54                        | +006  (dato / opcode no decodificado)
        .global HUD_PerPlayerPtrTable_025922__L02592a
HUD_PerPlayerPtrTable_025922__L02592a:
.L02592a:
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x56b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x5706                        | +00e  (dato / opcode no decodificado)
        .global HUD_PerPlayerPtrTable_025922__L025932
HUD_PerPlayerPtrTable_025922__L025932:
.L025932:
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x56be                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x5714                        | +016  (dato / opcode no decodificado)
        .global HUD_PerPlayerPtrTable_025922__L02593a
HUD_PerPlayerPtrTable_025922__L02593a:
.L02593a:
        .dc.w   0x0002                        | +018  (dato / opcode no decodificado)
        .dc.w   0x56be                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x5714                        | +01e  (dato / opcode no decodificado)
        .global HUD_PerPlayerPtrTable_025922__L025942
HUD_PerPlayerPtrTable_025922__L025942:
.L025942:
        .dc.w   0x0010                        | +020  (dato / opcode no decodificado)
        .dc.w   0x6f4c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +024  (dato / opcode no decodificado)
        .dc.w   0x6f4e                        | +026  (dato / opcode no decodificado)
        .global HUD_PerPlayerPtrTable_025922__L02594a
HUD_PerPlayerPtrTable_025922__L02594a:
.L02594a:
        .dc.w   0x0010                        | +028  (dato / opcode no decodificado)
        .dc.w   0x6e94                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x6e9c                        | +02e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_State_WaitPlayerSpawn_025952  @ $025952  (70 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_WaitPlayerSpawn_025952, "ax", @progbits
        .global HUD_State_WaitPlayerSpawn_025952
HUD_State_WaitPlayerSpawn_025952:
        jsr     0x1e4c.l                        | +000
        tst.w   0x106e92.l                      | +006
        bne.w   .L025964                        | +00c
        rts                                     | +010
.L025964:
        lea     .L02596a(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L02596a:
        jsr     0x1e4c.l                        | +018
        movea.l 0x78(a6),a0                     | +01e
        cmpi.l  #0x400,(a0)                     | +022
        bne.w   SetHandlerRts_02599e            | +028
        cmpi.b  #0x1,0x10fdaf.l                 | +02c
        beq.w   HUD_State_BindPlayer_0259a0     | +034
        subi.b  #0x1,0x77(a6)                   | +038
        tst.b   0x77(a6)                        | +03e
        bne.w   HUD_State_BindPlayer_0259a0     | +042

| ----------------------------------------------------------------------------
|  HUD_State_BindPlayer_0259a0  @ $0259A0  (102 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_BindPlayer_0259a0, "ax", @progbits
        .global HUD_State_BindPlayer_0259a0
HUD_State_BindPlayer_0259a0:
        lea     HUD_PerPlayerPtrTable_025922__L025932(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92__L025e9e | +004
        bra.w   .L0259c0                        | +008
        .global HUD_State_BindPlayer_0259a0__L0259ac
HUD_State_BindPlayer_0259a0__L0259ac:
.L0259ac:
        lea     HUD_PerPlayerPtrTable_025922__L02593a(pc),a0 | +00c
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92__L025e9e | +010
        bra.w   .L0259c0                        | +014
        .global HUD_State_BindPlayer_0259a0__L0259b8
HUD_State_BindPlayer_0259a0__L0259b8:
.L0259b8:
        lea     HUD_PerPlayerPtrTable_025922__L02592a(pc),a0 | +018
        jsr     HUD_CallPerPlayer_IfMode2_025e92__L025e9e(pc) | +01c
.L0259c0:
        movea.l 0x8(a0,d0.w),a1                 | +020
        move.l  a1,0x72(a6)                     | +024
        movea.l (a0,d0.w),a1                    | +028
        movea.l 0x4(a0,d0.w),a2                 | +02c
        move.l  a6,0xc(a2)                      | +030
        move.l  a2,0x78(a6)                     | +034
        move.l  a1,(a2)                         | +038
        move.w  #0x1,0x66(a2)                   | +03a
        jsr     0x79970.l                       | +040
        cmp.w   0x106e92.l,d0                   | +046
        bcs.w   .L0259f6                        | +04c
        move.w  d0,0x106e92.l                   | +050
.L0259f6:
        lea     .L0259fc(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L0259fc:
        tst.b   0x106ed2.l                      | +05c
        bne.w   HUD_State_PlayerDeath_025a0e    | +062

| ----------------------------------------------------------------------------
|  HUD_State_PlayerDeath_025a0e  @ $025A0E  (186 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_PlayerDeath_025a0e, "ax", @progbits
        .global HUD_State_PlayerDeath_025a0e
HUD_State_PlayerDeath_025a0e:
        tst.b   0x106ed3.l                      | +000
        bne.w   .L025a2e                        | +006
        movea.l 0x78(a6),a0                     | +00a
        ori.b   #0x3c,0x45(a0)                  | +00e
        bclr    #0x0,0x13(a0)                   | +014
        bclr    #0x3,0x13(a0)                   | +01a
.L025a2e:
        movea.l 0x78(a6),a0                     | +020
        btst    #0x0,0x13(a0)                   | +024
        beq.w   .L025aae                        | +02a
        lea     HUD_PerPlayerPtrTable_025922__L025942(pc),a0 | +02e
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +032
        clr.w   (a1)                            | +036
        cmpi.b  #0x2,0x106f2a.l                 | +038
        bne.w   .L025aa2                        | +040
        move.w  #0x0,d0                         | +044
        jsr     0x5e3a2.l                       | +048
        bcc.w   .L025a7e                        | +04e
        move.w  #0x1,d0                         | +052
        jsr     0x5e3a2.l                       | +056
        bcc.w   .L025a7a                        | +05c
        clr.b   d0                              | +060
        jsr     0x1e56.l                        | +062
        bra.w   .L025aa2                        | +068
.L025a7a:
        bra.w   .L025a98                        | +06c
.L025a7e:
        move.w  #0x1,d0                         | +070
        jsr     0x5e3a2.l                       | +074
        bcs.w   .L025a98                        | +07a
        clr.b   d0                              | +07e
        jsr     0x1e56.l                        | +080
        bra.w   .L025aa2                        | +086
.L025a98:
        move.b  #0x1e,d0                        | +08a
        jsr     0x1e56.l                        | +08e
.L025aa2:
        jsr     0x1e4c.l                        | +094
        lea     HUD_State_WaitPlayerSpawn_025952(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L025aae:
        lea     PlayerSlotDesc_P2Rows_025588__L025698(pc),a0 | +0a0
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +0a4
        lea     PlayerSlotDesc_P2Rows_025588__L0256a0(pc),a0 | +0a8
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +0ac
        lea     PlayerSlotDesc_P2Rows_025588__L0256a8(pc),a0 | +0b0
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +0b4
        rts                                     | +0b8

| ----------------------------------------------------------------------------
|  HUD_PlayerEntityTable_025ac8  @ $025AC8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_PlayerEntityTable_025ac8, "ax", @progbits
        .global HUD_PlayerEntityTable_025ac8
HUD_PlayerEntityTable_025ac8:
        .dc.w   0x0010                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0440                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        .dc.w   0x364a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0x04e0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x36dc                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_State_Respawn_025ad8  @ $025AD8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_Respawn_025ad8, "ax", @progbits
        .global HUD_State_Respawn_025ad8
HUD_State_Respawn_025ad8:
        lea     PlayerSlotDesc_P2Rows_025588__L0256a0(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +004
        lea     PlayerSlotDesc_P2Rows_025588__L0256a8(pc),a0 | +008
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +00c
        movea.l 0x78(a6),a0                     | +010
        cmpi.l  #0x400,(a0)                     | +014
        bne.w   .L025b00                        | +01a
        tst.b   0x106ed2.l                      | +01e
        bne.w   .L025b02                        | +024
.L025b00:
        rts                                     | +028
.L025b02:
        jsr     0x1e28.l                        | +02a
        bcs.w   .L025b0e                        | +030
        rts                                     | +034
.L025b0e:
        lea     HUD_PlayerEntityTable_025ac8(pc),a4 | +036
        move.w  0x70(a6),d1                     | +03a
        lsl.w   #0x3,d1                         | +03e
        movea.l (a4,d1.w),a0                    | +040
        movea.l 0x4(a4,d1.w),a1                 | +044
        move.l  a1,(a0)                         | +048
        jsr     0x5fe.l                         | +04a
        jmp     HUD_State_BindPlayer_0259a0__L0259ac(pc) | +050

| ----------------------------------------------------------------------------
|  HUD_ContinueTmplTable_025b2c  @ $025B2C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_ContinueTmplTable_025b2c, "ax", @progbits
        .global HUD_ContinueTmplTable_025b2c
HUD_ContinueTmplTable_025b2c:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x66f6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x6714                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_State_GameOverEntry_025b34  @ $025B34  (272 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_GameOverEntry_025b34, "ax", @progbits
        .global HUD_State_GameOverEntry_025b34
HUD_State_GameOverEntry_025b34:
        lea     PlayerSlotDesc_P2Rows_025588__L025668(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +004
        tst.b   0x10fd82.l                      | +008
        beq.w   .L025b54                        | +00e
        tst.b   0x10fd8a.l                      | +012
        beq.w   HUD_State_GameOverFinal_025d64  | +018
        bra.w   .L025b66                        | +01c
.L025b54:
        move.b  #0x1,d2                         | +020
        move.w  0x70(a6),d1                     | +024
        jsr     0x26752.l                       | +028
        bcc.w   HUD_State_GameOverFinal_025d64  | +02e
.L025b66:
        lea     0x9b872.l,a1                    | +032
        jsr     0x4498e.l                       | +038
        move.w  0x70(a6),0x98(a0)               | +03e
        move.b  #0x2,d0                         | +044
        jsr     HUD_SetStartMask_025e74(pc)     | +048
        move.b  #0x1,d2                         | +04c
        move.w  0x70(a6),d1                     | +050
        jsr     0x26752.l                       | +054
        bcs.w   HUD_State_ContinueCountdown_025c44 | +05a
        lea     HUD_ContinueTmplTable_025b2c(pc),a4 | +05e
        move.w  0x70(a6),d1                     | +062
        lsl.w   #0x2,d1                         | +066
        movea.l (a4,d1.w),a1                    | +068
        jsr     0x4ae.l                         | +06c
        move.b  #0xff,0x21(a6)                  | +072
        move.b  #0xff,0x20(a6)                  | +078
        lea     HUD_ContinueTmplTable2_025cc6__L025cce(pc),a1 | +07e
        jsr     0x4ae.l                         | +082
        move.b  0x21(a6),0x21(a0)               | +088
        move.b  0x20(a6),0x20(a0)               | +08e
        lea     .L025bce(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L025bce:
        jsr     0x1e4c.l                        | +09a
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +0a0
        cmpi.b  #0x1,d0                         | +0a4
        bne.w   .L025bfc                        | +0a8
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
        clr.b   0x20(a6)                        | +0b2
        lea     HUD_PerPlayerPtrTable_025922__L02594a(pc),a0 | +0b6
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +0ba
        jsr     0x51a86.l                       | +0be
        bra.w   .L025c42                        | +0c4
.L025bfc:
        move.b  #0x1,d2                         | +0c8
        move.w  0x70(a6),d1                     | +0cc
        jsr     0x26752.l                       | +0d0
        bcc.w   .L025c1c                        | +0d6
        lea     HUD_State_ContinueCountdown_025c44(pc),a1 | +0da
        move.l  a1,(a6)                         | +0de
        clr.b   0x20(a6)                        | +0e0
        bra.w   .L025c42                        | +0e4
.L025c1c:
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +0e8
        cmpi.b  #0x1,d0                         | +0ec
        beq.w   .L025c42                        | +0f0
        tst.b   0x21(a6)                        | +0f4
        bne.w   .L025c42                        | +0f8
        lea     HUD_State_GameOverFinal_025d64(pc),a1 | +0fc
        move.l  a1,(a6)                         | +100
        move.b  #0x1,d0                         | +102
        jsr     HUD_SetStartMask_025e74(pc)     | +106
        bra.w   .L025c42                        | +10a
.L025c42:
        rts                                     | +10e

| ----------------------------------------------------------------------------
|  HUD_State_ContinueCountdown_025c44  @ $025C44  (122 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_ContinueCountdown_025c44, "ax", @progbits
        .global HUD_State_ContinueCountdown_025c44
HUD_State_ContinueCountdown_025c44:
        move.w  0x70(a6),d1                     | +000
        lea     HUD_ContinueTmplTable_025b2c(pc),a4 | +004
        lsl.w   #0x2,d1                         | +008
        movea.l (a4,d1.w),a1                    | +00a
        jsr     0x4ae.l                         | +00e
        move.b  #0xff,0x21(a6)                  | +014
        move.b  #0xff,0x20(a6)                  | +01a
        lea     HUD_ContinueTmplTable2_025cc6__L025cce(pc),a1 | +020
        jsr     0x4ae.l                         | +024
        move.b  0x21(a6),0x21(a0)               | +02a
        move.b  0x20(a6),0x20(a0)               | +030
        lea     .L025c80(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L025c80:
        jsr     0x1e4c.l                        | +03c
        bsr.w   HUD_SetStartMask_025e74__L025e84 | +042
        cmpi.b  #0x1,d0                         | +046
        bne.w   .L025cae                        | +04a
        clr.b   0x20(a6)                        | +04e
        lea     HUD_PerPlayerPtrTable_025922__L02594a(pc),a0 | +052
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +056
        jsr     0x51a86.l                       | +05a
        lea     HUD_State_BindPlayer_0259a0__L0259b8(pc),a1 | +060
        move.l  a1,(a6)                         | +064
        bra.w   SetHandlerRts_025cc4            | +066
.L025cae:
        tst.b   0x21(a6)                        | +06a
        bne.w   SetHandlerRts_025cc4            | +06e
        move.b  #0x1,d0                         | +072
        bsr.w   HUD_SetStartMask_025e74         | +076

| ----------------------------------------------------------------------------
|  HUD_ContinueTmplTable2_025cc6  @ $025CC6  (142 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_ContinueTmplTable2_025cc6, "ax", @progbits
        .global HUD_ContinueTmplTable2_025cc6
HUD_ContinueTmplTable2_025cc6:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x67f6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x6800                        | +006  (dato / opcode no decodificado)
        .global HUD_ContinueTmplTable2_025cc6__L025cce
HUD_ContinueTmplTable2_025cc6__L025cce:
.L025cce:
        lea     0x10fdb6.l,a4                   | +008
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x70(a0),d1                     | +012
        eori.w  #0x1,d1                         | +016
        move.b  (a4,d1.w),d0                    | +01a
        cmpi.b  #0x1,d0                         | +01e
        beq.w   JmpToScheduler_025d5c           | +022
        cmpi.b  #0x2,d0                         | +026
        beq.w   JmpToScheduler_025d5c           | +02a
        move.w  0x70(a0),d1                     | +02e
        lea     HUD_ContinueTmplTable2_025cc6(pc),a0 | +032
        lsl.w   #0x2,d1                         | +036
        movea.l (a0,d1.w),a1                    | +038
        jsr     0x6fe.l                         | +03c
        lea     .L025d0e(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L025d0e:
        movea.l 0xc(a6),a0                      | +048
        move.b  0x20(a0),0x20(a6)               | +04c
        lea     0x10fdb6.l,a4                   | +052
        move.w  0x70(a0),d1                     | +058
        eori.w  #0x1,d1                         | +05c
        move.b  (a4,d1.w),d0                    | +060
        cmpi.b  #0x0,d0                         | +064
        beq.w   .L025d3e                        | +068
        cmpi.b  #0x3,d0                         | +06c
        beq.w   .L025d3e                        | +070
        clr.b   0x20(a6)                        | +074
.L025d3e:
        tst.b   0x20(a6)                        | +078
        bne.w   .L025d4c                        | +07c
        lea     JmpToScheduler_025d5c(pc),a1    | +080
        move.l  a1,(a6)                         | +084
.L025d4c:
        tst.b   0x21(a6)                        | +086
        bne.w   SetHandlerRts_025d5a            | +08a

| ----------------------------------------------------------------------------
|  HUD_State_GameOverFinal_025d64  @ $025D64  (104 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_GameOverFinal_025d64, "ax", @progbits
        .global HUD_State_GameOverFinal_025d64
HUD_State_GameOverFinal_025d64:
        move.b  #0x1,d0                         | +000
        jsr     HUD_SetStartMask_025e74(pc)     | +004
        move.b  #0xff,0x21(a6)                  | +008
        move.b  #0xff,0x20(a6)                  | +00e
        move.w  0x70(a6),d1                     | +014
        lea     HUD_PerPlayerPtrTable_025922(pc),a0 | +018
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92__L025e9e | +01c
        lea     .L025d8a(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L025d8a:
        jsr     0x1e4c.l                        | +026
        tst.b   0x21(a6)                        | +02c
        bne.w   JsrPcRts_025dd0                 | +030
        lea     PlayerSlotDesc_P2Rows_025588__L025668(pc),a0 | +034
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +038
        cmpi.b  #0x6,0x106ed0.l                 | +03c
        bcc.w   .L025db6                        | +044
        lea     HUD_State_GameOverDone_025dd2(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
        bra.w   .L025dc4                        | +04e
.L025db6:
        jsr     0x5b6.l                         | +052
        lea     0x400.l,a1                      | +058
        move.l  a1,(a6)                         | +05e
.L025dc4:
        clr.b   0x20(a6)                        | +060
        move.b  #0x3,d0                         | +064

| ----------------------------------------------------------------------------
|  HUD_State_GameOverDone_025dd2  @ $025DD2  (110 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_GameOverDone_025dd2, "ax", @progbits
        .global HUD_State_GameOverDone_025dd2
HUD_State_GameOverDone_025dd2:
        move.w  0x70(a6),d1                     | +000
        lea     PlayerSlotDesc_P2Rows_025588__L025690(pc),a4 | +004
        lsl.w   #0x2,d1                         | +008
        movea.l (a4,d1.w),a0                    | +00a
        jsr     (a0)                            | +00e
        lea     HUD_PerPlayerPtrTable_025922__L025942(pc),a0 | +010
        bsr.w   HUD_LoadPerPlayerPtr_025eac     | +014
        clr.w   (a1)                            | +018
        lea     PlayerSlotDesc_P2Rows_025588__L0256a0(pc),a0 | +01a
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +01e
        move.b  #0x1e,0x44(a6)                  | +022
        lea     .L025e00(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L025e00:
        tst.b   0x44(a6)                        | +02e
        bne.w   SetHandlerRts_025e46            | +032
        jsr     0x1e28.l                        | +036
        bcc.w   SetHandlerRts_025e46            | +03c
        move.b  #0x0,d0                         | +040
        jsr     HUD_SetStartMask_025e74(pc)     | +044
        move.b  #0x1,d2                         | +048
        move.w  0x70(a6),d1                     | +04c
        jsr     0x26752.l                       | +050
        bcc.w   .L025e36                        | +056
        lea     HUD_State_Continue_025882(pc),a1 | +05a
        move.l  a1,(a6)                         | +05e
        bra.w   SetHandlerRts_025e46            | +060
.L025e36:
        tst.b   0x10fd82.l                      | +064
        beq.w   SetHandlerRts_025e46            | +06a

| ----------------------------------------------------------------------------
|  HUD_State_PushStartBlink_025e48  @ $025E48  (44 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_State_PushStartBlink_025e48, "ax", @progbits
        .global HUD_State_PushStartBlink_025e48
HUD_State_PushStartBlink_025e48:
        lea     PlayerSlotDesc_P2Rows_025588__L025680(pc),a0 | +000
        bsr.w   HUD_CallPerPlayer_IfMode2_025e92 | +004
        lea     .L025e56(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L025e56:
        jsr     0x1e28.l                        | +00e
        bcc.w   .L025e72                        | +014
        move.l  0x7c(a6),(a6)                   | +018
        lea     0x106ecc.l,a0                   | +01c
        adda.w  0x70(a6),a0                     | +022
        move.b  0x82(a6),(a0)                   | +026
.L025e72:
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  HUD_SetStartMask_025e74  @ $025E74  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_SetStartMask_025e74, "ax", @progbits
        .global HUD_SetStartMask_025e74
HUD_SetStartMask_025e74:
        lea     0x10fdb6.l,a0                   | +000
        move.w  0x70(a6),d1                     | +006
        move.b  d0,(a0,d1.w)                    | +00a
        rts                                     | +00e
        .global HUD_SetStartMask_025e74__L025e84
HUD_SetStartMask_025e74__L025e84:
.L025e84:
        lea     0x10fdb6.l,a0                   | +010
        adda.w  0x70(a6),a0                     | +016
        move.b  (a0),d0                         | +01a
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  HUD_CallPerPlayer_IfMode2_025e92  @ $025E92  (26 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_CallPerPlayer_IfMode2_025e92, "ax", @progbits
        .global HUD_CallPerPlayer_IfMode2_025e92
HUD_CallPerPlayer_IfMode2_025e92:
        cmpi.b  #0x2,0x10fdaf.l                 | +000
        bne.w   .L025eaa                        | +008
        .global HUD_CallPerPlayer_IfMode2_025e92__L025e9e
HUD_CallPerPlayer_IfMode2_025e92__L025e9e:
.L025e9e:
        move.w  0x70(a6),d1                     | +00c
        lsl.w   #0x2,d1                         | +010
        movea.l (a0,d1.w),a0                    | +012
        jmp     (a0)                            | +016
.L025eaa:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  HUD_LoadPerPlayerPtr_025eac  @ $025EAC  (14 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_LoadPerPlayerPtr_025eac, "ax", @progbits
        .global HUD_LoadPerPlayerPtr_025eac
HUD_LoadPerPlayerPtr_025eac:
        move.w  0x70(a6),d1                     | +000
        add.w   d1,d1                           | +004
        add.w   d1,d1                           | +006
        movea.l (a0,d1.w),a1                    | +008
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_025eba  @ $025EBA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_025eba, "ax", @progbits
        .global ChildRank_CmpByte10_025eba
ChildRank_CmpByte10_025eba:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_025ed0                    | +00c

| ----------------------------------------------------------------------------
|  HUD_ResetFields_025ed6  @ $025ED6  (44 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_ResetFields_025ed6, "ax", @progbits
        .global HUD_ResetFields_025ed6
HUD_ResetFields_025ed6:
        clr.b   0x76(a6)                        | +000
        move.b  #0x1,0x84(a6)                   | +004
        move.b  #0x1,0x85(a6)                   | +00a
        clr.b   0x93(a6)                        | +010
        clr.w   0x8e(a6)                        | +014
        clr.w   0x90(a6)                        | +018
        clr.l   0x86(a6)                        | +01c
        clr.l   0x8a(a6)                        | +020
        move.b  #0x0,0x92(a6)                   | +024
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  HUD_DrawPlayerLabels_025f02  @ $025F02  (144 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawPlayerLabels_025f02, "ax", @progbits
        .global HUD_DrawPlayerLabels_025f02
HUD_DrawPlayerLabels_025f02:
        movea.w #0x7065,a1                      | +000
        move.w  #0x5300,d0                      | +004
        lea     0x2785b8.l,a2                   | +008
        jsr     0x5dad8.l                       | +00e
        move.w  #0x7044,d0                      | +014
        bsr.w   HUD_DrawLifeBarFrame_025f92     | +018
        movea.w #0x7143,a1                      | +01c
        move.w  #0x7ab0,d0                      | +020
        move.w  #0x8,d1                         | +024
        move.w  #0x3,d2                         | +028
        jsr     0x5da56.l                       | +02c
        movea.w #0x71c3,a1                      | +032
        move.w  #0x7ae4,d0                      | +036
        move.w  #0x4,d1                         | +03a
        move.w  #0x1,d2                         | +03e
        jmp     0x5da56.l                       | +042
        movea.w #0x73e5,a1                      | +048
        move.w  #0x5300,d0                      | +04c
        lea     0x2785be.l,a2                   | +050
        jsr     0x5dad8.l                       | +056
        move.w  #0x73c4,d0                      | +05c
        bsr.w   HUD_DrawLifeBarFrame_025f92     | +060
        movea.w #0x72c3,a1                      | +064
        move.w  #0x7ab0,d0                      | +068
        move.w  #0x8,d1                         | +06c
        move.w  #0x3,d2                         | +070
        jsr     0x5da56.l                       | +074
        movea.w #0x7343,a1                      | +07a
        move.w  #0x7ae4,d0                      | +07e
        move.w  #0x4,d1                         | +082
        move.w  #0x1,d2                         | +086
        jmp     0x5da56.l                       | +08a

| ----------------------------------------------------------------------------
|  HUD_DrawLifeBarFrame_025f92  @ $025F92  (100 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawLifeBarFrame_025f92, "ax", @progbits
        .global HUD_DrawLifeBarFrame_025f92
HUD_DrawLifeBarFrame_025f92:
        move.w  #0x7aa5,d7                      | +000
        movem.w d0/d7,0x3c0000.l                | +004
        addi.w  #0x20,d0                        | +00c
        move.b  #0xa6,d7                        | +010
        movem.w d0/d7,0x3c0000.l                | +014
        addi.w  #0x20,d0                        | +01c
        movem.w d0/d7,0x3c0000.l                | +020
        addi.w  #0x20,d0                        | +028
        movem.w d0/d7,0x3c0000.l                | +02c
        addi.w  #0x20,d0                        | +034
        movem.w d0/d7,0x3c0000.l                | +038
        addi.w  #0x20,d0                        | +040
        movem.w d0/d7,0x3c0000.l                | +044
        addi.w  #0x20,d0                        | +04c
        movem.w d0/d7,0x3c0000.l                | +050
        addi.w  #0x20,d0                        | +058
        move.b  #0xaf,d7                        | +05c
        .dc.w   0x48b9                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0081                        | +062  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_Msg_InsertCoin_P1_025ffc  @ $025FFC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_InsertCoin_P1_025ffc, "ax", @progbits
        .global HUD_Msg_InsertCoin_P1_025ffc
HUD_Msg_InsertCoin_P1_025ffc:
        movea.w #0x7063,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785c4.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_InsertCoin_P2_026010  @ $026010  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_InsertCoin_P2_026010, "ax", @progbits
        .global HUD_Msg_InsertCoin_P2_026010
HUD_Msg_InsertCoin_P2_026010:
        movea.w #0x7343,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785c4.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_PushStart_P1_026024  @ $026024  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_PushStart_P1_026024, "ax", @progbits
        .global HUD_Msg_PushStart_P1_026024
HUD_Msg_PushStart_P1_026024:
        movea.w #0x7063,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785d0.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_PushStart_P2_026038  @ $026038  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_PushStart_P2_026038, "ax", @progbits
        .global HUD_Msg_PushStart_P2_026038
HUD_Msg_PushStart_P2_026038:
        movea.w #0x7343,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785d0.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_Continue_P1_02604c  @ $02604C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_Continue_P1_02604c, "ax", @progbits
        .global HUD_Msg_Continue_P1_02604c
HUD_Msg_Continue_P1_02604c:
        movea.w #0x7063,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785dc.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_Continue_P2_026060  @ $026060  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_Continue_P2_026060, "ax", @progbits
        .global HUD_Msg_Continue_P2_026060
HUD_Msg_Continue_P2_026060:
        movea.w #0x7343,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785dc.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_GameOver_P1_026074  @ $026074  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_GameOver_P1_026074, "ax", @progbits
        .global HUD_Msg_GameOver_P1_026074
HUD_Msg_GameOver_P1_026074:
        movea.w #0x7063,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785e8.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_GameOver_P2_026088  @ $026088  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_GameOver_P2_026088, "ax", @progbits
        .global HUD_Msg_GameOver_P2_026088
HUD_Msg_GameOver_P2_026088:
        movea.w #0x7343,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785e8.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_PleaseWait_P1_02609c  @ $02609C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_PleaseWait_P1_02609c, "ax", @progbits
        .global HUD_Msg_PleaseWait_P1_02609c
HUD_Msg_PleaseWait_P1_02609c:
        movea.w #0x7063,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785f4.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_Msg_PleaseWait_P2_0260b0  @ $0260B0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_Msg_PleaseWait_P2_0260b0, "ax", @progbits
        .global HUD_Msg_PleaseWait_P2_0260b0
HUD_Msg_PleaseWait_P2_0260b0:
        movea.w #0x7343,a1                      | +000
        move.b  #0x3,d1                         | +004
        lea     0x2785f4.l,a2                   | +008
        jmp     0x477fc.l                       | +00e

| ----------------------------------------------------------------------------
|  HUD_ClearMsgRow_0260c4  @ $0260C4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_ClearMsgRow_0260c4, "ax", @progbits
        .global HUD_ClearMsgRow_0260c4
HUD_ClearMsgRow_0260c4:
        movea.w #0x7043,a1                      | +000
        bra.w   .L0260d0                        | +004
        movea.w #0x72c3,a1                      | +008
.L0260d0:
        move.w  #0x300,d0                       | +00c
        move.w  #0x10,d1                        | +010
        move.w  #0x3,d2                         | +014
        jmp     0x5da9c.l                       | +018

| ----------------------------------------------------------------------------
|  HUD_DrawContinueDigits_0260e2  @ $0260E2  (70 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawContinueDigits_0260e2, "ax", @progbits
        .global HUD_DrawContinueDigits_0260e2
HUD_DrawContinueDigits_0260e2:
        move.w  #0x70e5,d2                      | +000
        bra.w   .L0260ee                        | +004
        move.w  #0x7465,d2                      | +008
.L0260ee:
        move.b  0x77(a6),d0                     | +00c
        andi.w  #0xff,d0                        | +010
        beq.w   .L0260fc                        | +014
        subq.w  #0x1,d0                         | +018
.L0260fc:
        move.w  d2,-(a7)                        | +01a
        jsr     0x47656.l                       | +01c
        move.w  (a7)+,d2                        | +022
        tst.w   d1                              | +024
        beq.w   HUD_DrawContinueDigit_Low_02612e | +026
        move.w  #0x5330,d3                      | +02a
        add.b   d1,d3                           | +02e
        movem.w d2-d3,0x3c0000.l                | +030
        addi.w  #0x20,d2                        | +038
        move.b  #0x30,d3                        | +03c
        add.b   d0,d3                           | +040
        .dc.w   0x48b9                        | +042  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +044  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_DrawContinueDigit_Low_02612e  @ $02612E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawContinueDigit_Low_02612e, "ax", @progbits
        .global HUD_DrawContinueDigit_Low_02612e
HUD_DrawContinueDigit_Low_02612e:
        move.w  #0x5330,d3                      | +000
        add.b   d0,d3                           | +004
        movem.w d2-d3,0x3c0000.l                | +006
        addi.w  #0x20,d2                        | +00e
        move.b  #0x20,d3                        | +012
        .dc.w   0x48b9                        | +016  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +018  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_DrawLifeBar_02614e  @ $02614E  (442 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawLifeBar_02614e, "ax", @progbits
        .global HUD_DrawLifeBar_02614e
HUD_DrawLifeBar_02614e:
        move.w  #0x7044,d0                      | +000
        bra.w   .L02615a                        | +004
        move.w  #0x73c4,d0                      | +008
.L02615a:
        move.w  d0,-(a7)                        | +00c
        movea.l 0x78(a6),a0                     | +00e
        jsr     0x2ac0e.l                       | +012
        scs.b   d1                              | +018
        move.b  0x92(a6),d2                     | +01a
        cmpi.b  #0x0,d2                         | +01e
        beq.w   .L026180                        | +022
        cmpi.b  #0x1,d2                         | +026
        beq.w   .L02619c                        | +02a
        bra.w   .L0261f2                        | +02e
.L026180:
        tst.b   d1                              | +032
        beq.w   .L02618c                        | +034
        move.b  #0x1,0x92(a6)                   | +038
.L02618c:
        move.w  0x86(a6),d5                     | +03e
.L026190:
        moveq   #0,d1                           | +042
        move.l  d1,0x86(a6)                     | +044
        clr.b   d2                              | +048
        bra.w   .L026270                        | +04a
.L02619c:
        tst.b   d1                              | +04e
        bne.w   .L0261ae                        | +050
        move.b  #0x1,d5                         | +054
        move.b  #0x0,0x92(a6)                   | +058
        bra.b   .L026190                        | +05e
.L0261ae:
        move.w  0x1005e6.l,d1                   | +060
        sub.w   0x86(a6),d1                     | +066
        move.w  d1,d5                           | +06a
        ext.l   d5                              | +06c
        swap    d5                              | +06e
        eor.w   d5,d1                           | +070
        sub.w   d5,d1                           | +072
        cmpi.w  #0x2,d1                         | +074
        bcs.w   .L0261ce                        | +078
        move.w  #0x2,d1                         | +07c
.L0261ce:
        eor.w   d5,d1                           | +080
        sub.w   d5,d1                           | +082
        move.w  d1,d5                           | +084
        add.w   d1,0x86(a6)                     | +086
        move.w  0x86(a6),d1                     | +08a
        tst.w   d5                              | +08e
        bne.w   .L0261ec                        | +090
        move.b  #0x2,0x92(a6)                   | +094
        clr.l   0x8a(a6)                        | +09a
.L0261ec:
        clr.b   d2                              | +09e
        bra.w   .L026270                        | +0a0
.L0261f2:
        tst.b   d1                              | +0a4
        beq.w   .L026208                        | +0a6
        move.w  0x1005e6.l,d1                   | +0aa
        cmpi.w  #0x11,d1                        | +0b0
        scs.b   d2                              | +0b4
        bra.w   .L02621a                        | +0b6
.L026208:
        move.b  #0x0,0x92(a6)                   | +0ba
        clr.l   0x8a(a6)                        | +0c0
        move.b  #0x1,d5                         | +0c4
        bra.w   .L026190                        | +0c8
.L02621a:
        move.w  d1,d3                           | +0cc
        move.w  d1,d6                           | +0ce
        swap    d3                              | +0d0
        clr.w   d3                              | +0d2
        sub.l   0x86(a6),d3                     | +0d4
        neg.l   d3                              | +0d8
        move.l  0x8a(a6),d4                     | +0da
        move.l  d4,d5                           | +0de
        lsl.l   #0x1,d3                         | +0e0
        asr.l   #0x2,d4                         | +0e2
        add.l   d3,d4                           | +0e4
        sub.l   d4,d5                           | +0e6
        move.l  d5,0x8a(a6)                     | +0e8
        move.l  d5,d1                           | +0ec
        add.l   0x86(a6),d1                     | +0ee
        move.l  d1,0x86(a6)                     | +0f2
        swap    d1                              | +0f6
        swap    d5                              | +0f8
        swap    d3                              | +0fa
        or.w    d5,d3                           | +0fc
        bne.w   .L026268                        | +0fe
        swap    d6                              | +102
        clr.w   d6                              | +104
        cmp.l   0x86(a6),d6                     | +106
        beq.w   .L026260                        | +10a
        move.w  #0x1,d5                         | +10e
.L026260:
        move.l  d6,0x86(a6)                     | +112
        clr.l   0x8a(a6)                        | +116
.L026268:
        tst.w   d1                              | +11a
        bpl.w   .L026270                        | +11c
        clr.w   d1                              | +120
.L026270:
        move.w  (a7)+,d0                        | +122
        tst.b   d2                              | +124
        beq.w   .L02629a                        | +126
        move.b  0x106f28.l,d4                   | +12a
        move.w  #0xda00,d7                      | +130
        andi.b  #0x7,d4                         | +134
        beq.w   .L0262a6                        | +138
        move.w  #0x7a00,d7                      | +13c
        cmpi.b  #0x2,d4                         | +140
        beq.w   .L0262a6                        | +144
        bra.w   .L02629e                        | +148
.L02629a:
        move.w  #0x7a00,d7                      | +14c
.L02629e:
        tst.w   d5                              | +150
        bne.w   .L0262a6                        | +152
        rts                                     | +156
.L0262a6:
        move.b  #0xa5,d7                        | +158
        movem.w d0/d7,0x3c0000.l                | +15c
        addi.w  #0x20,d0                        | +164
        move.w  #0x6,d2                         | +168
        move.b  #0xae,d7                        | +16c
.L0262be:
        subq.w  #0x8,d1                         | +170
        bmi.w   .L0262d8                        | +172
        movem.w d0/d7,0x3c0000.l                | +176
        addi.w  #0x20,d0                        | +17e
        subq.w  #0x1,d2                         | +182
        beq.w   .L026300                        | +184
        bra.b   .L0262be                        | +188
.L0262d8:
        add.b   d1,d7                           | +18a
        movem.w d0/d7,0x3c0000.l                | +18c
        addi.w  #0x20,d0                        | +194
        subq.w  #0x1,d2                         | +198
        beq.w   .L026300                        | +19a
        move.b  #0xa6,d7                        | +19e
.L0262f0:
        movem.w d0/d7,0x3c0000.l                | +1a2
        addi.w  #0x20,d0                        | +1aa
        subq.w  #0x1,d2                         | +1ae
        bne.b   .L0262f0                        | +1b0
.L026300:
        move.b  #0xaf,d7                        | +1b2
        .dc.w   0x48b9                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0081                        | +1b8  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_DrawBombGauge_P1_02630e  @ $02630E  (100 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawBombGauge_P1_02630e, "ax", @progbits
        .global HUD_DrawBombGauge_P1_02630e
HUD_DrawBombGauge_P1_02630e:
        move.w  0x106f4c.l,d0                   | +000
        cmp.b   0x76(a6),d0                     | +006
        bne.w   .L02631e                        | +00a
        rts                                     | +00e
.L02631e:
        move.b  d0,0x76(a6)                     | +010
        move.w  #0x707c,d1                      | +014
        move.w  #0x238a,d2                      | +018
        cmpi.b  #0xe,d0                         | +01c
        bls.w   .L026336                        | +020
        move.b  #0xe,d0                         | +024
.L026336:
        move.b  #0xe,d3                         | +028
        sub.b   d0,d3                           | +02c
.L02633c:
        cmpi.b  #0x0,d0                         | +02e
        ble.w   .L026354                        | +032
        movem.w d1-d2,0x3c0000.l                | +036
        addi.w  #0x20,d1                        | +03e
        subq.b  #0x1,d0                         | +042
        bra.b   .L02633c                        | +044
.L026354:
        move.w  #0x2320,d2                      | +046
.L026358:
        cmpi.b  #0x0,d3                         | +04a
        ble.w   .L026370                        | +04e
        movem.w d1-d2,0x3c0000.l                | +052
        addi.w  #0x20,d1                        | +05a
        subq.b  #0x1,d3                         | +05e
        bra.b   .L026358                        | +060
.L026370:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  HUD_DrawBombGauge_P2_026372  @ $026372  (100 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawBombGauge_P2_026372, "ax", @progbits
        .global HUD_DrawBombGauge_P2_026372
HUD_DrawBombGauge_P2_026372:
        move.w  0x106f4e.l,d0                   | +000
        cmp.b   0x76(a6),d0                     | +006
        bne.w   .L026382                        | +00a
        rts                                     | +00e
.L026382:
        move.b  d0,0x76(a6)                     | +010
        move.w  #0x72fc,d1                      | +014
        move.w  #0x2320,d2                      | +018
        cmpi.b  #0xe,d0                         | +01c
        bls.w   .L02639a                        | +020
        move.b  #0xe,d0                         | +024
.L02639a:
        move.b  #0xe,d3                         | +028
        sub.b   d0,d3                           | +02c
.L0263a0:
        cmpi.b  #0x0,d3                         | +02e
        ble.w   .L0263b8                        | +032
        movem.w d1-d2,0x3c0000.l                | +036
        addi.w  #0x20,d1                        | +03e
        subq.b  #0x1,d3                         | +042
        bra.b   .L0263a0                        | +044
.L0263b8:
        move.w  #0x238a,d2                      | +046
.L0263bc:
        cmpi.b  #0x0,d0                         | +04a
        ble.w   .L0263d4                        | +04e
        movem.w d1-d2,0x3c0000.l                | +052
        addi.w  #0x20,d1                        | +05a
        subq.b  #0x1,d0                         | +05e
        bra.b   .L0263bc                        | +060
.L0263d4:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  HUD_DrawScore_0263d6  @ $0263D6  (264 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawScore_0263d6, "ax", @progbits
        .global HUD_DrawScore_0263d6
HUD_DrawScore_0263d6:
        jsr     HUD_IsSceneBCD_0266cc(pc)       | +000
        bcc.w   .L0263e0                        | +004
        rts                                     | +008
.L0263e0:
        jsr     HUD_DrawAmmoAndBombs_0264e4(pc) | +00a
        move.w  #0x73c3,d0                      | +00e
        lea     0x106e9c.l,a1                   | +012
        bra.w   .L02640a                        | +018
        jsr     HUD_IsSceneBCD_0266cc(pc)       | +01c
        bcc.w   .L0263fc                        | +020
        rts                                     | +024
.L0263fc:
        jsr     HUD_DrawAmmoAndBombs_0264e4(pc) | +026
        move.w  #0x7063,d0                      | +02a
        lea     0x106e94.l,a1                   | +02e
.L02640a:
        adda.w  #0x1,a1                         | +034
        move.w  #0x2300,d1                      | +038
        clr.w   d2                              | +03c
        move.b  (a1)+,d1                        | +03e
        or.b    d1,d2                           | +040
        bne.w   .L026424                        | +042
        move.b  #0x20,d1                        | +046
        bra.w   .L026428                        | +04a
.L026424:
        addi.b  #0x30,d1                        | +04e
.L026428:
        movem.w d0-d1,0x3c0000.l                | +052
        addi.w  #0x20,d0                        | +05a
        move.b  (a1)+,d1                        | +05e
        or.b    d1,d2                           | +060
        bne.w   .L026444                        | +062
        move.b  #0x20,d1                        | +066
        bra.w   .L026448                        | +06a
.L026444:
        addi.b  #0x30,d1                        | +06e
.L026448:
        movem.w d0-d1,0x3c0000.l                | +072
        addi.w  #0x20,d0                        | +07a
        move.b  (a1)+,d1                        | +07e
        or.b    d1,d2                           | +080
        bne.w   .L026464                        | +082
        move.b  #0x20,d1                        | +086
        bra.w   .L026468                        | +08a
.L026464:
        addi.b  #0x30,d1                        | +08e
.L026468:
        movem.w d0-d1,0x3c0000.l                | +092
        addi.w  #0x20,d0                        | +09a
        move.b  (a1)+,d1                        | +09e
        or.b    d1,d2                           | +0a0
        bne.w   .L026484                        | +0a2
        move.b  #0x20,d1                        | +0a6
        bra.w   .L026488                        | +0aa
.L026484:
        addi.b  #0x30,d1                        | +0ae
.L026488:
        movem.w d0-d1,0x3c0000.l                | +0b2
        addi.w  #0x20,d0                        | +0ba
        move.b  (a1)+,d1                        | +0be
        or.b    d1,d2                           | +0c0
        bne.w   .L0264a4                        | +0c2
        move.b  #0x20,d1                        | +0c6
        bra.w   .L0264a8                        | +0ca
.L0264a4:
        addi.b  #0x30,d1                        | +0ce
.L0264a8:
        movem.w d0-d1,0x3c0000.l                | +0d2
        addi.w  #0x20,d0                        | +0da
        move.b  (a1)+,d1                        | +0de
        or.b    d1,d2                           | +0e0
        bne.w   .L0264c4                        | +0e2
        move.b  #0x20,d1                        | +0e6
        bra.w   .L0264c8                        | +0ea
.L0264c4:
        addi.b  #0x30,d1                        | +0ee
.L0264c8:
        movem.w d0-d1,0x3c0000.l                | +0f2
        addi.w  #0x20,d0                        | +0fa
        move.b  (a1),d1                         | +0fe
        addi.b  #0x30,d1                        | +100
        .dc.w   0x48b9                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +106  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_DrawAmmoAndBombs_0264e4  @ $0264E4  (482 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_DrawAmmoAndBombs_0264e4, "ax", @progbits
        .global HUD_DrawAmmoAndBombs_0264e4
HUD_DrawAmmoAndBombs_0264e4:
        movea.l 0x78(a6),a0                     | +000
        jsr     0x32fba.l                       | +004
        tst.w   d0                              | +00a
        bpl.w   .L0264f6                        | +00c
        clr.w   d0                              | +010
.L0264f6:
        cmp.w   0x8e(a6),d0                     | +012
        beq.w   .L026508                        | +016
        move.w  d0,0x8e(a6)                     | +01a
        move.b  #0x4,0x84(a6)                   | +01e
.L026508:
        move.b  0x84(a6),d1                     | +024
        beq.w   .L0265c2                        | +028
        subq.b  #0x1,d1                         | +02c
        move.b  d1,0x84(a6)                     | +02e
        andi.b  #0x1,d1                         | +032
        bne.w   .L026526                        | +036
        move.w  #0xe300,d7                      | +03a
        bra.w   .L02652a                        | +03e
.L026526:
        move.w  #0xf300,d7                      | +042
.L02652a:
        tst.w   0x70(a6)                        | +046
        bne.w   .L02653a                        | +04a
        move.w  #0x71a4,d6                      | +04e
        bra.w   .L02653e                        | +052
.L02653a:
        move.w  #0x7324,d6                      | +056
.L02653e:
        tst.w   d0                              | +05a
        bne.w   .L026570                        | +05c
        move.w  #0xe3f2,d7                      | +060
        movem.w d6-d7,0x3c0000.l                | +064
        subi.w  #0x20,d6                        | +06c
        subq.w  #0x1,d7                         | +070
        movem.w d6-d7,0x3c0000.l                | +072
        subi.w  #0x20,d6                        | +07a
        subq.w  #0x1,d7                         | +07e
        movem.w d6-d7,0x3c0000.l                | +080
        bra.w   .L0265c2                        | +088
.L026570:
        andi.l  #0xffff,d0                      | +08c
        divu.w  #0xa,d0                         | +092
        swap    d0                              | +096
        addi.b  #0x30,d0                        | +098
        move.b  d0,d7                           | +09c
        movem.w d6-d7,0x3c0000.l                | +09e
        subi.w  #0x20,d6                        | +0a6
        clr.w   d0                              | +0aa
        swap    d0                              | +0ac
        divu.w  #0xa,d0                         | +0ae
        swap    d0                              | +0b2
        addi.b  #0x30,d0                        | +0b4
        move.b  d0,d7                           | +0b8
        movem.w d6-d7,0x3c0000.l                | +0ba
        subi.w  #0x20,d6                        | +0c2
        clr.w   d0                              | +0c6
        swap    d0                              | +0c8
        divu.w  #0xa,d0                         | +0ca
        swap    d0                              | +0ce
        addi.b  #0x30,d0                        | +0d0
        move.b  d0,d7                           | +0d4
        movem.w d6-d7,0x3c0000.l                | +0d6
.L0265c2:
        movea.l 0x78(a6),a0                     | +0de
        jsr     0x2ac0e.l                       | +0e2
        bcc.w   .L0265dc                        | +0e8
        jsr     0x2abd2.l                       | +0ec
        scc.b   d0                              | +0f2
        bra.w   .L0265de                        | +0f4
.L0265dc:
        clr.b   d0                              | +0f8
.L0265de:
        cmp.b   0x93(a6),d0                     | +0fa
        beq.w   .L026624                        | +0fe
        move.b  d0,0x93(a6)                     | +102
        tst.w   0x70(a6)                        | +106
        bne.w   .L0265fa                        | +10a
        movea.w #0x71c3,a1                      | +10e
        bra.w   .L0265fe                        | +112
.L0265fa:
        movea.w #0x7343,a1                      | +116
.L0265fe:
        move.w  #0x7ae0,d0                      | +11a
        tst.b   0x93(a6)                        | +11e
        bne.w   .L02660c                        | +122
        addq.b  #0x4,d0                         | +126
.L02660c:
        move.w  #0x4,d1                         | +128
        move.w  #0x1,d2                         | +12c
        jsr     0x5da56.l                       | +130
        move.b  #0x4,0x85(a6)                   | +136
        bra.w   .L02665e                        | +13c
.L026624:
        tst.b   0x93(a6)                        | +140
        bne.w   .L02663a                        | +144
        movea.l 0x78(a6),a0                     | +148
        jsr     0x32fce.l                       | +14c
        bra.w   .L026640                        | +152
.L02663a:
        jsr     0x2a2ac.l                       | +156
.L026640:
        tst.b   d0                              | +15c
        bpl.w   .L026648                        | +15e
        clr.b   d0                              | +162
.L026648:
        andi.w  #0xff,d0                        | +164
        cmp.w   0x90(a6),d0                     | +168
        beq.w   .L02665e                        | +16c
        move.w  d0,0x90(a6)                     | +170
        move.b  #0x4,0x85(a6)                   | +174
.L02665e:
        move.b  0x85(a6),d0                     | +17a
        beq.w   NopCCRMid_0266ca                | +17e
        subq.b  #0x1,d0                         | +182
        move.b  d0,0x85(a6)                     | +184
        andi.b  #0x1,d0                         | +188
        bne.w   .L02667c                        | +18c
        move.w  #0xe300,d7                      | +190
        bra.w   .L026680                        | +194
.L02667c:
        move.w  #0xf300,d7                      | +198
.L026680:
        tst.w   0x70(a6)                        | +19c
        bne.w   .L026690                        | +1a0
        move.w  #0x7204,d6                      | +1a4
        bra.w   .L026694                        | +1a8
.L026690:
        move.w  #0x7384,d6                      | +1ac
.L026694:
        moveq   #0,d0                           | +1b0
        move.w  0x90(a6),d0                     | +1b2
        divu.w  #0xa,d0                         | +1b6
        swap    d0                              | +1ba
        addi.b  #0x30,d0                        | +1bc
        move.b  d0,d7                           | +1c0
        movem.w d6-d7,0x3c0000.l                | +1c2
        subi.w  #0x20,d6                        | +1ca
        clr.w   d0                              | +1ce
        swap    d0                              | +1d0
        divu.w  #0xa,d0                         | +1d2
        swap    d0                              | +1d6
        addi.b  #0x30,d0                        | +1d8
        move.b  d0,d7                           | +1dc
        .dc.w   0x48b9                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +1e0  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  HUD_IsSceneBCD_0266cc  @ $0266CC  (30 B)
| ----------------------------------------------------------------------------
        .section .text.HUD_IsSceneBCD_0266cc, "ax", @progbits
        .global HUD_IsSceneBCD_0266cc
HUD_IsSceneBCD_0266cc:
        move.b  0x106ece.l,d0                   | +000
        cmpi.b  #0xb,d0                         | +006
        beq.w   SetXN_0266f0                    | +00a
        cmpi.b  #0xc,d0                         | +00e
        beq.w   SetXN_0266f0                    | +012
        cmpi.b  #0xd,d0                         | +016
        beq.w   SetXN_0266f0                    | +01a

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_0266f6  @ $0266F6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_0266f6, "ax", @progbits
        .global ChildRank_CmpByte10_0266f6
ChildRank_CmpByte10_0266f6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_02670c                    | +00c

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_026712  @ $026712  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_026712, "ax", @progbits
        .global ChildRank_CmpByte10_026712
ChildRank_CmpByte10_026712:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_026728                    | +00c

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_02672e  @ $02672E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_02672e, "ax", @progbits
        .global ChildRank_CmpByte10_02672e
ChildRank_CmpByte10_02672e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_026744                    | +00c

| ----------------------------------------------------------------------------
|  Credits_BCDPtrTable_02674a  @ $02674A  (112 B)
| ----------------------------------------------------------------------------
        .section .text.Credits_BCDPtrTable_02674a, "ax", @progbits
        .global Credits_BCDPtrTable_02674a
Credits_BCDPtrTable_02674a:
        .dc.w   0x0010                        | +000  (dato / opcode no decodificado)
        .dc.w   0x81bf                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +004  (dato / opcode no decodificado)
        .dc.w   0x81c0                        | +006  (dato / opcode no decodificado)
        tst.b   0x10fd82.l                      | +008
        bne.w   .L02676c                        | +00e
        lea     Credits_BCDPtrTable_02674a(pc),a4 | +012
        lsl.w   #0x2,d1                         | +016
        movea.l (a4,d1.w),a4                    | +018
        move.b  (a4),d2                         | +01c
        bra.w   .L0267b4                        | +01e
.L02676c:
        lea     0x10fdb0.l,a4                   | +022
        move.b  d2,(a4)                         | +028
        move.b  d2,0x1(a4)                      | +02a
        clr.b   0x2(a4)                         | +02e
        clr.b   0x3(a4)                         | +032
        move.w  d1,-(a7)                        | +036
        jsr     0xc00450.l                      | +038
        move.w  (a7)+,d1                        | +03e
        cmpi.b  #0x1,0x10fd83.l                 | +040
        bne.w   .L02679e                        | +048
        move.b  (a4,d1.w),d2                    | +04c
        bra.w   .L0267b4                        | +050
.L02679e:
        cmpi.b  #0x2,0x10fdaf.l                 | +054
        bne.w   .L0267b0                        | +05c
        move.b  (a4),d2                         | +060
        bra.w   .L0267b4                        | +062
.L0267b0:
        move.b  (a4,d1.w),d2                    | +066
.L0267b4:
        tst.b   d2                              | +06a
        beq.w   ClearXN_0267c0                  | +06c

| ----------------------------------------------------------------------------
|  ChildRank_CmpByte10_0267c6  @ $0267C6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ChildRank_CmpByte10_0267c6, "ax", @progbits
        .global ChildRank_CmpByte10_0267c6
ChildRank_CmpByte10_0267c6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0267dc                    | +00c

| ----------------------------------------------------------------------------
|  ClampVelocity_Default_0267f4  @ $0267F4  (4 B)
| ----------------------------------------------------------------------------
        .section .text.ClampVelocity_Default_0267f4, "ax", @progbits
        .global ClampVelocity_Default_0267f4
ClampVelocity_Default_0267f4:
        bra.w   ClampVelocity_0267f8__L0267fc   | +000

| ----------------------------------------------------------------------------
|  ClampVelocity_0267f8  @ $0267F8  (28 B)
| ----------------------------------------------------------------------------
        .section .text.ClampVelocity_0267f8, "ax", @progbits
        .global ClampVelocity_0267f8
ClampVelocity_0267f8:
        move.w  #0x800,d1                       | +000
        .global ClampVelocity_0267f8__L0267fc
ClampVelocity_0267f8__L0267fc:
.L0267fc:
        cmp.w   d0,d1                           | +004
        bge.w   .L026808                        | +006
        move.w  d1,d0                           | +00a
        bra.w   .L026812                        | +00c
.L026808:
        neg.w   d1                              | +010
        cmp.w   d0,d1                           | +012
        ble.w   .L026812                        | +014
        move.w  d1,d0                           | +018
.L026812:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Entity_IntegrateVelocity_026814  @ $026814  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_IntegrateVelocity_026814, "ax", @progbits
        .global Entity_IntegrateVelocity_026814
Entity_IntegrateVelocity_026814:
        move.w  0x2c(a6),d0                     | +000
        add.w   0x28(a6),d0                     | +004
        jsr     ClampVelocity_0267f8(pc)        | +008
        move.w  d0,0x28(a6)                     | +00c
        move.w  0x2e(a6),d0                     | +010
        add.w   0x2a(a6),d0                     | +014
        jsr     ClampVelocity_0267f8(pc)        | +018

| ----------------------------------------------------------------------------
|  Entity_MoveX_WallStop_026836  @ $026836  (342 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MoveX_WallStop_026836, "ax", @progbits
        .global Entity_MoveX_WallStop_026836
Entity_MoveX_WallStop_026836:
        move.b  -0x1144(a5),-0x1150(a5)         | +000
        move.b  -0x1143(a5),-0x114f(a5)         | +006
        clr.w   -0x1154(a5)                     | +00c
        move.w  0x28(a6),d2                     | +010
        btst    #0x0,0x69(a6)                   | +014
        beq.w   .L026860                        | +01a
        tst.w   d2                              | +01e
        ble.w   .L026860                        | +020
        clr.w   d2                              | +024
        bra.w   .L026872                        | +026
.L026860:
        btst    #0x1,0x69(a6)                   | +02a
        beq.w   .L026872                        | +030
        tst.w   d2                              | +034
        bge.w   .L026872                        | +036
        clr.w   d2                              | +03a
.L026872:
        moveq   #0,d3                           | +03c
        cmpi.b  #0x1,0x106ece.l                 | +03e
        bne.w   .L0268c2                        | +046
        bra.w   .L0268c2                        | +04a
        tst.w   0x106f5e.l                      | +04e
        beq.w   .L0268c2                        | +054
        btst    #0x6,0x13(a6)                   | +058
        bne.w   .L0268c2                        | +05e
        move.w  0x106f6c.l,d3                   | +062
        lsl.w   #0x8,d3                         | +068
        btst    #0x0,0x6b(a6)                   | +06a
        beq.w   .L0268c2                        | +070
        btst    #0x7,0x13(a6)                   | +074
        bne.w   .L0268c2                        | +07a
        cmpi.w  #0x14,0x22(a6)                  | +07e
        bcc.w   .L0268c2                        | +084
        addi.w  #0x200,d3                       | +088
.L0268c2:
        cmpi.b  #0x3,0x106ece.l                 | +08c
        bne.w   .L0268f2                        | +094
        cmpa.l  #0x100440,a6                    | +098
        beq.w   .L0268e6                        | +09e
        cmpa.l  #0x1004e0,a6                    | +0a2
        beq.w   .L0268e6                        | +0a8
        bra.w   .L0268f2                        | +0ac
.L0268e6:
        cmpi.w  #0x1f0,0x24(a6)                 | +0b0
        blt.w   .L0268f2                        | +0b6
        clr.w   d2                              | +0ba
.L0268f2:
        cmpa.l  #0x100440,a6                    | +0bc
        beq.w   .L026922                        | +0c2
        cmpa.l  #0x1004e0,a6                    | +0c6
        beq.w   .L026922                        | +0cc
        cmpa.l  #0x100580,a6                    | +0d0
        bne.w   .L02691e                        | +0d6
        jsr     0x2a276.l                       | +0da
        bcc.w   .L02691e                        | +0e0
        bra.w   .L026922                        | +0e4
.L02691e:
        bra.w   .L026926                        | +0e8
.L026922:
        jsr     Sub_00028074(pc)                | +0ec  -> $028074 (hueco futuro, defsym forward)
.L026926:
        add.w   d3,d2                           | +0f0
        moveq   #0,d0                           | +0f2
        move.b  -0x1150(a5),d0                  | +0f4
        add.w   d2,d0                           | +0f8
        move.w  d0,d2                           | +0fa
        move.b  d2,-0x1150(a5)                  | +0fc
        asr.w   #0x8,d0                         | +100
        move.w  d0,-0x1154(a5)                  | +102
        moveq   #0,d1                           | +106
        move.b  -0x114f(a5),d1                  | +108
        move.w  0x2a(a6),d2                     | +10c
        cmpi.b  #0x3,0x106ece.l                 | +110
        bne.w   .L02697e                        | +118
        cmpa.l  #0x100440,a6                    | +11c
        beq.w   .L02696a                        | +122
        cmpa.l  #0x1004e0,a6                    | +126
        beq.w   .L02696a                        | +12c
        bra.w   .L02697e                        | +130
.L02696a:
        cmpi.w  #0x1f0,0x24(a6)                 | +134
        blt.w   .L02697e                        | +13a
        cmpi.w  #0x0,d2                         | +13e
        ble.w   .L02697e                        | +142
        clr.w   d2                              | +146
.L02697e:
        add.w   d2,d1                           | +148
        move.w  d1,d2                           | +14a
        move.b  d1,-0x114f(a5)                  | +14c
        asr.w   #0x8,d1                         | +150
        move.w  d1,-0x1152(a5)                  | +152

| ----------------------------------------------------------------------------
|  Entity_MoveXY_Probe_026992  @ $026992  (452 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MoveXY_Probe_026992, "ax", @progbits
        .global Entity_MoveXY_Probe_026992
Entity_MoveXY_Probe_026992:
        move.b  -0x1144(a5),-0x1150(a5)         | +000
        move.b  -0x1143(a5),-0x114f(a5)         | +006
        clr.w   -0x1154(a5)                     | +00c
        move.w  0x28(a6),d2                     | +010
        btst    #0x0,0x69(a6)                   | +014
        beq.w   .L0269bc                        | +01a
        tst.w   d2                              | +01e
        ble.w   .L0269bc                        | +020
        clr.w   d2                              | +024
        bra.w   .L0269ce                        | +026
.L0269bc:
        btst    #0x1,0x69(a6)                   | +02a
        beq.w   .L0269ce                        | +030
        tst.w   d2                              | +034
        bge.w   .L0269ce                        | +036
        clr.w   d2                              | +03a
.L0269ce:
        moveq   #0,d3                           | +03c
        cmpi.b  #0x1,0x106ece.l                 | +03e
        bne.w   .L026a1e                        | +046
        bra.w   .L026a1e                        | +04a
        tst.w   0x106f5e.l                      | +04e
        beq.w   .L026a1e                        | +054
        btst    #0x6,0x13(a6)                   | +058
        bne.w   .L026a1e                        | +05e
        move.w  0x106f6c.l,d3                   | +062
        lsl.w   #0x8,d3                         | +068
        btst    #0x0,0x6b(a6)                   | +06a
        beq.w   .L026a1e                        | +070
        btst    #0x7,0x13(a6)                   | +074
        bne.w   .L026a1e                        | +07a
        cmpi.w  #0x14,0x22(a6)                  | +07e
        bcc.w   .L026a1e                        | +084
        addi.w  #0x200,d3                       | +088
.L026a1e:
        cmpa.l  #0x100440,a6                    | +08c
        beq.w   .L026a4e                        | +092
        cmpa.l  #0x1004e0,a6                    | +096
        beq.w   .L026a4e                        | +09c
        cmpa.l  #0x100580,a6                    | +0a0
        bne.w   .L026a4a                        | +0a6
        jsr     0x2a276.l                       | +0aa
        bcc.w   .L026a4a                        | +0b0
        bra.w   .L026a4e                        | +0b4
.L026a4a:
        bra.w   .L026a52                        | +0b8
.L026a4e:
        jsr     Sub_00028074(pc)                | +0bc  -> $028074 (hueco futuro, defsym forward)
.L026a52:
        add.w   d3,d2                           | +0c0
        jsr     Sub_0002800E(pc)                | +0c2  -> $02800E (hueco futuro, defsym forward)
        moveq   #0,d0                           | +0c6
        move.b  -0x1150(a5),d0                  | +0c8
        add.w   d2,d0                           | +0cc
        move.w  d0,d2                           | +0ce
        move.b  d2,-0x1150(a5)                  | +0d0
        asr.w   #0x8,d0                         | +0d4
        move.w  d0,-0x1154(a5)                  | +0d6
        jsr     PcThunkTarget_0281c8(pc)        | +0da
        move.w  -0x1154(a5),d1                  | +0de
        moveq   #0,d0                           | +0e2
        move.w  0x34(a6),d2                     | +0e4
        cmpi.w  #0xffff,d2                      | +0e8
        beq.w   .L026ac2                        | +0ec
        tst.w   d2                              | +0f0
        beq.w   .L026ae0                        | +0f2
        bge.w   .L026a8e                        | +0f6
        moveq   #-1,d0                          | +0fa
.L026a8e:
        eor.w   d0,d2                           | +0fc
        sub.w   d0,d2                           | +0fe
        cmpi.w  #0x1000,d2                      | +100
        beq.w   .L026ae8                        | +104
        cmpi.w  #0x400,d2                       | +108
        beq.w   .L026ad4                        | +10c
        cmpi.w  #0x80,d2                        | +110
        beq.w   .L026abe                        | +114
        cmpi.w  #0x80,d2                        | +118
        beq.w   .L026abe                        | +11c
        nop                                     | +120
        nop                                     | +122
        cmpi.w  #0x80,d2                        | +124
        nop                                     | +128
        trap    #0xf                            | +12a
.L026abe:
        clr.w   -0x1154(a5)                     | +12c
.L026ac2:
        btst    #0x3,0x5b(a6)                   | +130
        beq.w   .L026ad0                        | +136
        clr.w   -0x1152(a5)                     | +13a
.L026ad0:
        bra.w   .L026b08                        | +13e
.L026ad4:
        add.w   d0,d1                           | +142
        eor.w   d0,d1                           | +144
        move.w  d1,-0x1152(a5)                  | +146
        bra.w   .L026b08                        | +14a
.L026ae0:
        clr.w   -0x1152(a5)                     | +14e
        bra.w   .L026b08                        | +152
.L026ae8:
        move.b  -0x114f(a5),d6                  | +156
        andi.w  #0xff,d6                        | +15a
        asl.w   #0x6,d1                         | +15e
        add.w   d0,d1                           | +160
        eor.w   d0,d1                           | +162
        add.w   d1,d6                           | +164
        move.w  d6,d1                           | +166
        move.b  d6,-0x114f(a5)                  | +168
        asr.w   #0x8,d1                         | +16c
        move.w  d1,-0x1152(a5)                  | +16e
        bra.w   .L026b08                        | +172
.L026b08:
        cmpi.b  #0x3,0x106ece.l                 | +176
        bne.w   .L026b54                        | +17e
        cmpa.l  #0x100440,a6                    | +182
        beq.w   .L026b2c                        | +188
        cmpa.l  #0x1004e0,a6                    | +18c
        beq.w   .L026b2c                        | +192
        bra.w   .L026b54                        | +196
.L026b2c:
        cmpi.w  #0x1f0,0x24(a6)                 | +19a
        blt.w   .L026b54                        | +1a0
        cmpi.w  #0x0,-0x1152(a5)                | +1a4
        ble.w   .L026b54                        | +1aa
        clr.w   -0x1154(a5)                     | +1ae
        clr.w   -0x1152(a5)                     | +1b2
        move.b  -0x1144(a5),-0x1150(a5)         | +1b6
        move.b  -0x1143(a5),-0x114f(a5)         | +1bc
.L026b54:
        rts                                     | +1c2

| ----------------------------------------------------------------------------
|  Entity_MoveAndCollide_A_026b56  @ $026B56  (576 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MoveAndCollide_A_026b56, "ax", @progbits
        .global Entity_MoveAndCollide_A_026b56
Entity_MoveAndCollide_A_026b56:
        jsr     Entity_IntegrateVelocity_026814(pc) | +000
        jsr     Entity_MoveXY_Probe_026992(pc)  | +004
        jsr     0x44182.l                       | +008
        move.w  -0x1148(a5),d1                  | +00e
        move.w  -0x1146(a5),d2                  | +012
        subq.w  #0x1,d2                         | +016
        jsr     Sub_00027DB2(pc)                | +018
        add.w   -0x1154(a5),d1                  | +01c
        add.w   -0x1152(a5),d2                  | +020
        jsr     0x998ca.l                       | +024
        bcc.w   .L026ba6                        | +02a
        moveq   #0,d0                           | +02e
        moveq   #0,d3                           | +030
        moveq   #0,d4                           | +032
        moveq   #17,d0                          | +034
        move.l  d0,d7                           | +036
        lea     0x278ba8.l,a1                   | +038
        movea.l a1,a4                           | +03e
        movea.l 0x4(a1),a3                      | +040
        move.b  0x12(a3,d3.w),0x106f31.l        | +044
        bra.w   .L026c82                        | +04c
.L026ba6:
        sub.w   -0x1154(a5),d1                  | +050
        sub.w   -0x1152(a5),d2                  | +054
        cmpi.b  #0x1,0xa(a1)                    | +058
        bne.w   .L026bc4                        | +05e
        tst.w   0x34(a6)                        | +062
        bne.w   .L026bc4                        | +066
        adda.w  #0x10,a1                        | +06a
.L026bc4:
        jsr     Sub_00027E9C(pc)                | +06e  -> $027E9C (hueco futuro, defsym forward)
        bcs.w   .L026bd4                        | +072
        jsr     Sub_00027E7E(pc)                | +076  -> $027E7E (hueco futuro, defsym forward)
        bcs.w   .L026bd4                        | +07a
.L026bd4:
        add.w   -0x1154(a5),d1                  | +07e
        add.w   -0x1152(a5),d2                  | +082
        movem.w d3-d4,-(a7)                     | +086
        movea.l a1,a4                           | +08a
        jsr     Sub_00027DB2(pc)                | +08c
        cmpi.b  #0x1,0xa(a1)                    | +090
        bne.w   .L026c74                        | +096
        tst.w   0x34(a6)                        | +09a
        bne.w   .L026c74                        | +09e
        tst.w   -0x1154(a5)                     | +0a2
        blt.w   .L026c14                        | +0a6
        cmpi.b  #0x23,d0                        | +0aa
        beq.w   .L026c28                        | +0ae
        cmpi.b  #0x33,d0                        | +0b2
        beq.w   .L026c28                        | +0b6
        bra.w   .L026c70                        | +0ba
.L026c14:
        cmpi.b  #0x22,d0                        | +0be
        beq.w   .L026c28                        | +0c2
        cmpi.b  #0x32,d0                        | +0c6
        beq.w   .L026c28                        | +0ca
        bra.w   .L026c70                        | +0ce
.L026c28:
        btst    #0x5,0x13(a6)                   | +0d2
        beq.w   .L026c70                        | +0d8
        tst.w   -0x1154(a5)                     | +0dc
        beq.w   .L026c70                        | +0e0
        btst    #0x5,0x5b(a6)                   | +0e4
        bne.w   .L026c66                        | +0ea
        cmpi.b  #0x0,0x6a(a6)                   | +0ee
        beq.w   .L026c66                        | +0f4
        move.b  0x6a(a6),d6                     | +0f8
        ext.w   d6                              | +0fc
        andi.w  #0x8000,d6                      | +0fe
        move.w  0x28(a6),d5                     | +102
        andi.w  #0x8000,d5                      | +106
        eor.w   d5,d6                           | +10a
        bne.w   .L026c70                        | +10c
.L026c66:
        bset    #0x3,0x5a(a6)                   | +110
        bra.w   .L026c74                        | +116
.L026c70:
        adda.w  #0x10,a1                        | +11a
.L026c74:
        movea.l 0x4(a1),a3                      | +11e
        move.b  0x12(a3,d3.w),-0x114f(a5)       | +122
        movem.w (a7)+,d3-d4                     | +128
.L026c82:
        tst.w   -0x1154(a5)                     | +12c
        blt.w   .L026cae                        | +130
        cmpi.b  #0x20,d0                        | +134
        beq.w   .L026cd2                        | +138
        cmpi.b  #0x30,d0                        | +13c
        beq.w   .L026cd2                        | +140
        cmpi.b  #0x21,d0                        | +144
        beq.w   .L026d1a                        | +148
        cmpi.b  #0x31,d0                        | +14c
        beq.w   .L026d1a                        | +150
        bra.w   .L026d24                        | +154
.L026cae:
        cmpi.b  #0x20,d0                        | +158
        beq.w   .L026d1a                        | +15c
        cmpi.b  #0x30,d0                        | +160
        beq.w   .L026d1a                        | +164
        cmpi.b  #0x21,d0                        | +168
        beq.w   .L026cd2                        | +16c
        cmpi.b  #0x31,d0                        | +170
        beq.w   .L026cd2                        | +174
        bra.w   .L026d24                        | +178
.L026cd2:
        btst    #0x4,0x13(a6)                   | +17c
        beq.w   .L026d1a                        | +182
        tst.w   -0x1154(a5)                     | +186
        beq.w   .L026d1a                        | +18a
        btst    #0x4,0x5b(a6)                   | +18e
        bne.w   .L026d10                        | +194
        cmpi.b  #0x0,0x6a(a6)                   | +198
        beq.w   .L026d10                        | +19e
        move.b  0x6a(a6),d6                     | +1a2
        ext.w   d6                              | +1a6
        andi.w  #0x8000,d6                      | +1a8
        move.w  0x28(a6),d5                     | +1ac
        andi.w  #0x8000,d5                      | +1b0
        eor.w   d5,d6                           | +1b4
        bne.w   .L026d1a                        | +1b6
.L026d10:
        bset    #0x3,0x5a(a6)                   | +1ba
        bra.w   .L026d24                        | +1c0
.L026d1a:
        lea     0x278704.l,a3                   | +1c4
        bra.w   .L026d24                        | +1ca
.L026d24:
        cmpi.b  #0x0,0x6a(a6)                   | +1ce
        beq.w   .L026d46                        | +1d4
        move.b  0x6a(a6),d6                     | +1d8
        ext.w   d6                              | +1dc
        andi.w  #0x8000,d6                      | +1de
        move.w  0x28(a6),d5                     | +1e2
        andi.w  #0x8000,d5                      | +1e6
        eor.w   d5,d6                           | +1ea
        bne.w   .L026d60                        | +1ec
.L026d46:
        cmpi.b  #0x10,d0                        | +1f0
        beq.w   .L026d56                        | +1f4
        cmpi.b  #0x27,d0                        | +1f8
        bne.w   .L026d60                        | +1fc
.L026d56:
        bset    #0x5,0x5a(a6)                   | +200
        bra.w   ClearXN_026dbc                  | +206
.L026d60:
        move.w  0x64(a6),d6                     | +20a
        cmp.w   0x1a(a3),d6                     | +20e
        bne.w   .L026d76                        | +212
        move.w  (a3),d6                         | +216
        cmp.w   0x34(a6),d6                     | +218
        beq.w   Entity_CommitMove_026d9c        | +21c
.L026d76:
        movea.l a4,a1                           | +220
        jsr     Entity_FloorProbe_026e0a(pc)    | +222
        jsr     Entity_WallProbeBoth_027020(pc) | +226
        clr.b   -0x1144(a5)                     | +22a
        move.b  0x12(a2,d3.w),d5                | +22e
        move.b  d5,-0x1143(a5)                  | +232
        addq.w  #0x1,d2                         | +236
        move.w  d1,-0x1148(a5)                  | +238
        move.w  d2,-0x1146(a5)                  | +23c

| ----------------------------------------------------------------------------
|  Entity_CommitMove_026d9c  @ $026D9C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CommitMove_026d9c, "ax", @progbits
        .global Entity_CommitMove_026d9c
Entity_CommitMove_026d9c:
        bclr    #0x3,0x5a(a6)                   | +000
        addq.w  #0x1,d2                         | +006
        move.w  d1,-0x1148(a5)                  | +008
        move.w  d2,-0x1146(a5)                  | +00c
        move.b  -0x1150(a5),d1                  | +010
        move.b  -0x114f(a5),d2                  | +014
        move.b  d1,-0x1144(a5)                  | +018
        move.b  d2,-0x1143(a5)                  | +01c

| ----------------------------------------------------------------------------
|  Entity_ProbeOffsets_Right_026dc2  @ $026DC2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ProbeOffsets_Right_026dc2, "ax", @progbits
        .global Entity_ProbeOffsets_Right_026dc2
Entity_ProbeOffsets_Right_026dc2:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +010  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +014  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_ProbeOffsets_Left_026dda  @ $026DDA  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ProbeOffsets_Left_026dda, "ax", @progbits
        .global Entity_ProbeOffsets_Left_026dda
Entity_ProbeOffsets_Left_026dda:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_ProbeOffsets_None_026df2  @ $026DF2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ProbeOffsets_None_026df2, "ax", @progbits
        .global Entity_ProbeOffsets_None_026df2
Entity_ProbeOffsets_None_026df2:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_FloorProbe_026e0a  @ $026E0A  (478 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_FloorProbe_026e0a, "ax", @progbits
        .global Entity_FloorProbe_026e0a
Entity_FloorProbe_026e0a:
        lea     Entity_ProbeOffsets_Right_026dc2(pc),a2 | +000
        tst.w   0x106f2c.l                      | +004
        beq.w   .L026e22                        | +00a
        tst.w   0x106f2c.l                      | +00e
        bra.w   .L026e26                        | +014
.L026e22:
        tst.w   0x28(a6)                        | +018
.L026e26:
        beq.w   .L026e36                        | +01c
        bge.w   .L026e3a                        | +020
        lea     Entity_ProbeOffsets_Left_026dda(pc),a2 | +024
        bra.w   .L026e3a                        | +028
.L026e36:
        lea     Entity_ProbeOffsets_None_026df2(pc),a2 | +02c
.L026e3a:
        clr.w   d5                              | +030
        cmpi.w  #0xffff,0x34(a6)                | +032
        beq.w   .L026e5a                        | +038
        tst.w   0x34(a6)                        | +03c
        beq.w   .L026e5a                        | +040
        bge.w   .L026e5e                        | +044
        move.w  #0x10,d5                        | +048
        bra.w   .L026e5e                        | +04c
.L026e5a:
        move.w  #0x8,d5                         | +050
        .global Entity_FloorProbe_026e0a__L026e5e
Entity_FloorProbe_026e0a__L026e5e:
.L026e5e:
        move.w  (a3),d6                         | +054
        cmpi.w  #0xffff,d6                      | +056
        beq.w   .L026e70                        | +05a
        cmp.w   0x34(a6),d6                     | +05e
        bge.w   .L026e72                        | +062
.L026e70:
        addq.w  #0x4,d5                         | +066
.L026e72:
        move.w  0x2(a2,d5.w),d6                 | +068
        move.w  (a2,d5.w),d5                    | +06c
        move.w  -0x1148(a5),d1                  | +070
        move.w  -0x1146(a5),d2                  | +074
        subq.w  #0x1,d2                         | +078
        sub.w   d3,d1                           | +07a
        add.w   d4,d2                           | +07c
        add.w   d5,d1                           | +07e
        add.w   d6,d2                           | +080
        jsr     Sub_00027DB2(pc)                | +082
        jsr     0x9993c.l                       | +086
        move.b  d6,0x106f44.l                   | +08c
        cmpi.b  #0xf,d6                         | +092
        beq.w   .L026eb6                        | +096
        moveq   #0,d0                           | +09a
        moveq   #0,d3                           | +09c
        move.w  d5,d4                           | +09e
        moveq   #17,d0                          | +0a0
        lea     0x278ba8.l,a1                   | +0a2
        bra.w   .L026eb6                        | +0a8
.L026eb6:
        cmpi.b  #0xc,d0                         | +0ac
        beq.w   .L026f72                        | +0b0
        cmpi.b  #0xd,d0                         | +0b4
        beq.w   .L026f72                        | +0b8
        cmpi.b  #0xe,d0                         | +0bc
        beq.w   .L026f72                        | +0c0
        cmpi.b  #0xf,d0                         | +0c4
        beq.w   .L026f72                        | +0c8
        cmpi.b  #0x1c,d0                        | +0cc
        beq.w   .L026f72                        | +0d0
        cmpi.b  #0x1d,d0                        | +0d4
        beq.w   .L026f72                        | +0d8
        cmpi.b  #0x1e,d0                        | +0dc
        beq.w   .L026f72                        | +0e0
        cmpi.b  #0x1f,d0                        | +0e4
        beq.w   .L026f72                        | +0e8
        cmpi.b  #0x34,d0                        | +0ec
        beq.w   .L026f72                        | +0f0
        cmpi.b  #0x2c,d0                        | +0f4
        beq.w   .L026f70                        | +0f8
        cmpi.b  #0x2d,d0                        | +0fc
        beq.w   .L026f70                        | +100
        cmpi.b  #0x2e,d0                        | +104
        beq.w   .L026f70                        | +108
        cmpi.b  #0x2f,d0                        | +10c
        beq.w   .L026f70                        | +110
        cmpi.b  #0x3c,d0                        | +114
        beq.w   .L026f70                        | +118
        cmpi.b  #0x3d,d0                        | +11c
        beq.w   .L026f70                        | +120
        cmpi.b  #0x3e,d0                        | +124
        beq.w   .L026f70                        | +128
        cmpi.b  #0x3f,d0                        | +12c
        beq.w   .L026f70                        | +130
        cmpi.b  #0x20,d0                        | +134
        beq.w   .L026f62                        | +138
        cmpi.b  #0x21,d0                        | +13c
        beq.w   .L026f62                        | +140
        cmpi.b  #0x30,d0                        | +144
        beq.w   .L026f62                        | +148
        cmpi.b  #0x31,d0                        | +14c
        beq.w   .L026f62                        | +150
        bra.w   .L026f9e                        | +154
.L026f62:
        btst    #0x4,0x13(a6)                   | +158
        beq.w   .L026f9e                        | +15e
        bra.w   .L026f72                        | +162
.L026f70:
        addq.w  #0x8,d2                         | +166
.L026f72:
        addq.w  #0x8,d2                         | +168
        jsr     Sub_00027DB2(pc)                | +16a
        jsr     0x9993c.l                       | +16e
        move.b  d6,0x106f44.l                   | +174
        cmpi.b  #0xf,d6                         | +17a
        beq.w   .L026f9e                        | +17e
        moveq   #0,d0                           | +182
        moveq   #0,d3                           | +184
        move.w  d5,d4                           | +186
        moveq   #17,d0                          | +188
        lea     0x278ba8.l,a1                   | +18a
        bra.w   .L026f9e                        | +190
.L026f9e:
        movea.l 0x4(a1),a2                      | +194
        move.w  (a2),0x34(a6)                   | +198
        move.w  0x1a(a2),0x64(a6)               | +19c
        cmpi.w  #0x80,0x34(a6)                  | +1a2
        bne.w   .L026fc4                        | +1a8
        nop                                     | +1ac
        nop                                     | +1ae
        cmpi.w  #0x80,0x34(a6)                  | +1b0
        nop                                     | +1b6
        trap    #0xf                            | +1b8
.L026fc4:
        cmpi.w  #0xff80,0x34(a6)                | +1ba
        bne.w   .L026fdc                        | +1c0
        nop                                     | +1c4
        nop                                     | +1c6
        cmpi.w  #0xff80,0x34(a6)                | +1c8
        nop                                     | +1ce
        trap    #0xf                            | +1d0
.L026fdc:
        asl.w   #0x1,d3                         | +1d2
        move.w  0x2(a2,d3.w),d4                 | +1d4
        asr.w   #0x1,d3                         | +1d8
        sub.w   d4,d2                           | +1da
        rts                                     | +1dc

| ----------------------------------------------------------------------------
|  Entity_FloorProbe_Reenter_026fe8  @ $026FE8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_FloorProbe_Reenter_026fe8, "ax", @progbits
        .global Entity_FloorProbe_Reenter_026fe8
Entity_FloorProbe_Reenter_026fe8:
        lea     Entity_ProbeOffsets_Right_026dc2(pc),a2 | +000
        tst.w   0x28(a6)                        | +004
        bge.w   .L026ff8                        | +008
        lea     Entity_ProbeOffsets_Left_026dda(pc),a2 | +00c
.L026ff8:
        clr.w   d5                              | +010
        cmpi.w  #0xffff,-0x114e(a5)             | +012
        beq.w   .L027018                        | +018
        tst.w   -0x114e(a5)                     | +01c
        beq.w   .L027018                        | +020
        bge.w   .L02701c                        | +024
        move.w  #0x10,d5                        | +028
        bra.w   .L02701c                        | +02c
.L027018:
        move.w  #0x8,d5                         | +030
.L02701c:
        jmp     Entity_FloorProbe_026e0a__L026e5e(pc) | +034

| ----------------------------------------------------------------------------
|  Entity_WallProbeBoth_027020  @ $027020  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_WallProbeBoth_027020, "ax", @progbits
        .global Entity_WallProbeBoth_027020
Entity_WallProbeBoth_027020:
        jsr     Sub_00027E9C(pc)                | +000  -> $027E9C (hueco futuro, defsym forward)
        bcs.w   .L027034                        | +004
        jsr     Sub_00027E7E(pc)                | +008  -> $027E7E (hueco futuro, defsym forward)
        bcs.w   .L027034                        | +00c
        bra.w   .L027034                        | +010
.L027034:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Entity_MoveAndCollide_B_027036  @ $027036  (580 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MoveAndCollide_B_027036, "ax", @progbits
        .global Entity_MoveAndCollide_B_027036
Entity_MoveAndCollide_B_027036:
        jsr     Entity_IntegrateVelocity_026814(pc) | +000
        jsr     Entity_MoveXY_Probe_026992(pc)  | +004
        jsr     0x44182.l                       | +008
        move.w  -0x1148(a5),d1                  | +00e
        move.w  -0x1146(a5),d2                  | +012
        subq.w  #0x1,d2                         | +016
        jsr     Sub_00027DB2(pc)                | +018
        add.w   -0x1154(a5),d1                  | +01c
        add.w   -0x1152(a5),d2                  | +020
        jsr     0x998ca.l                       | +024
        bcc.w   .L027086                        | +02a
        moveq   #0,d0                           | +02e
        moveq   #0,d3                           | +030
        moveq   #0,d4                           | +032
        moveq   #17,d0                          | +034
        move.l  d0,d7                           | +036
        lea     0x278ba8.l,a1                   | +038
        movea.l a1,a4                           | +03e
        movea.l 0x4(a1),a3                      | +040
        move.b  0x12(a3,d3.w),0x106f31.l        | +044
        bra.w   .L027166                        | +04c
.L027086:
        sub.w   -0x1154(a5),d1                  | +050
        sub.w   -0x1152(a5),d2                  | +054
        cmpi.b  #0x1,0xa(a1)                    | +058
        bne.w   .L0270a4                        | +05e
        tst.w   0x34(a6)                        | +062
        bne.w   .L0270a4                        | +066
        adda.w  #0x10,a1                        | +06a
.L0270a4:
        jsr     Sub_00027E9C(pc)                | +06e  -> $027E9C (hueco futuro, defsym forward)
        bcs.w   .L0270b8                        | +072
        jsr     Sub_00027E7E(pc)                | +076  -> $027E7E (hueco futuro, defsym forward)
        bcs.w   .L0270b8                        | +07a
        bra.w   .L0270b8                        | +07e
.L0270b8:
        add.w   -0x1154(a5),d1                  | +082
        add.w   -0x1152(a5),d2                  | +086
        movem.w d3-d4,-(a7)                     | +08a
        movea.l a1,a4                           | +08e
        jsr     Sub_00027DB2(pc)                | +090
        cmpi.b  #0x1,0xa(a1)                    | +094
        bne.w   .L027158                        | +09a
        tst.w   0x34(a6)                        | +09e
        bne.w   .L027158                        | +0a2
        tst.w   -0x1154(a5)                     | +0a6
        blt.w   .L0270f8                        | +0aa
        cmpi.b  #0x23,d0                        | +0ae
        beq.w   .L02710c                        | +0b2
        cmpi.b  #0x33,d0                        | +0b6
        beq.w   .L02710c                        | +0ba
        bra.w   .L027154                        | +0be
.L0270f8:
        cmpi.b  #0x22,d0                        | +0c2
        beq.w   .L02710c                        | +0c6
        cmpi.b  #0x32,d0                        | +0ca
        beq.w   .L02710c                        | +0ce
        bra.w   .L027154                        | +0d2
.L02710c:
        btst    #0x5,0x13(a6)                   | +0d6
        beq.w   .L027154                        | +0dc
        tst.w   -0x1154(a5)                     | +0e0
        beq.w   .L027154                        | +0e4
        btst    #0x5,0x5b(a6)                   | +0e8
        bne.w   .L02714a                        | +0ee
        cmpi.b  #0x0,0x6a(a6)                   | +0f2
        beq.w   .L02714a                        | +0f8
        move.b  0x6a(a6),d6                     | +0fc
        ext.w   d6                              | +100
        andi.w  #0x8000,d6                      | +102
        move.w  0x28(a6),d5                     | +106
        andi.w  #0x8000,d5                      | +10a
        eor.w   d5,d6                           | +10e
        bne.w   .L027154                        | +110
.L02714a:
        bset    #0x3,0x5a(a6)                   | +114
        bra.w   .L027158                        | +11a
.L027154:
        adda.w  #0x10,a1                        | +11e
.L027158:
        movea.l 0x4(a1),a3                      | +122
        move.b  0x12(a3,d3.w),-0x114f(a5)       | +126
        movem.w (a7)+,d3-d4                     | +12c
.L027166:
        tst.w   -0x1154(a5)                     | +130
        blt.w   .L027192                        | +134
        cmpi.b  #0x20,d0                        | +138
        beq.w   .L0271b6                        | +13c
        cmpi.b  #0x30,d0                        | +140
        beq.w   .L0271b6                        | +144
        cmpi.b  #0x21,d0                        | +148
        beq.w   .L0271fe                        | +14c
        cmpi.b  #0x31,d0                        | +150
        beq.w   .L0271fe                        | +154
        bra.w   .L027208                        | +158
.L027192:
        cmpi.b  #0x20,d0                        | +15c
        beq.w   .L0271fe                        | +160
        cmpi.b  #0x30,d0                        | +164
        beq.w   .L0271fe                        | +168
        cmpi.b  #0x21,d0                        | +16c
        beq.w   .L0271b6                        | +170
        cmpi.b  #0x31,d0                        | +174
        beq.w   .L0271b6                        | +178
        bra.w   .L027208                        | +17c
.L0271b6:
        btst    #0x4,0x13(a6)                   | +180
        beq.w   .L0271fe                        | +186
        tst.w   -0x1154(a5)                     | +18a
        beq.w   .L0271fe                        | +18e
        btst    #0x4,0x5b(a6)                   | +192
        bne.w   .L0271f4                        | +198
        cmpi.b  #0x0,0x6a(a6)                   | +19c
        beq.w   .L0271f4                        | +1a2
        move.b  0x6a(a6),d6                     | +1a6
        ext.w   d6                              | +1aa
        andi.w  #0x8000,d6                      | +1ac
        move.w  0x28(a6),d5                     | +1b0
        andi.w  #0x8000,d5                      | +1b4
        eor.w   d5,d6                           | +1b8
        bne.w   .L0271fe                        | +1ba
.L0271f4:
        bset    #0x3,0x5a(a6)                   | +1be
        bra.w   .L027208                        | +1c4
.L0271fe:
        lea     0x278704.l,a3                   | +1c8
        bra.w   .L027208                        | +1ce
.L027208:
        cmpi.b  #0x0,0x6a(a6)                   | +1d2
        beq.w   .L02722a                        | +1d8
        move.b  0x6a(a6),d6                     | +1dc
        ext.w   d6                              | +1e0
        andi.w  #0x8000,d6                      | +1e2
        move.w  0x28(a6),d5                     | +1e6
        andi.w  #0x8000,d5                      | +1ea
        eor.w   d5,d6                           | +1ee
        bne.w   .L027244                        | +1f0
.L02722a:
        cmpi.b  #0x10,d0                        | +1f4
        beq.w   .L02723a                        | +1f8
        cmpi.b  #0x27,d0                        | +1fc
        bne.w   .L027244                        | +200
.L02723a:
        bset    #0x5,0x5a(a6)                   | +204
        bra.w   ClearXN_0272a2                  | +20a
.L027244:
        move.w  0x64(a6),d6                     | +20e
        cmp.w   0x1a(a3),d6                     | +212
        bne.w   .L02725a                        | +216
        move.w  (a3),d6                         | +21a
        cmp.w   0x34(a6),d6                     | +21c
        beq.w   Entity_CommitMove_ClearBit3_027280__L027282 | +220
.L02725a:
        movea.l a4,a1                           | +224
        jsr     Entity_FloorProbe_026e0a(pc)    | +226
        jsr     Entity_WallProbeBoth_027020(pc) | +22a
        clr.b   -0x1144(a5)                     | +22e
        move.b  0x12(a2,d3.w),d5                | +232
        move.b  d5,-0x1143(a5)                  | +236
        addq.w  #0x1,d2                         | +23a
        move.w  d1,-0x1148(a5)                  | +23c
        move.w  d2,-0x1146(a5)                  | +240

| ----------------------------------------------------------------------------
|  Entity_CommitMove_ClearBit3_027280  @ $027280  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CommitMove_ClearBit3_027280, "ax", @progbits
        .global Entity_CommitMove_ClearBit3_027280
Entity_CommitMove_ClearBit3_027280:
        nop                                     | +000
        .global Entity_CommitMove_ClearBit3_027280__L027282
Entity_CommitMove_ClearBit3_027280__L027282:
.L027282:
        bclr    #0x3,0x5a(a6)                   | +002
        addq.w  #0x1,d2                         | +008
        move.w  d1,-0x1148(a5)                  | +00a
        move.w  d2,-0x1146(a5)                  | +00e
        move.b  -0x1150(a5),d1                  | +012
        move.b  -0x114f(a5),d2                  | +016
        move.b  d1,-0x1144(a5)                  | +01a
        move.b  d2,-0x1143(a5)                  | +01e

| ----------------------------------------------------------------------------
|  Entity_MoveAndCollide_C_0272a8  @ $0272A8  (302 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_MoveAndCollide_C_0272a8, "ax", @progbits
        .global Entity_MoveAndCollide_C_0272a8
Entity_MoveAndCollide_C_0272a8:
        jsr     Entity_IntegrateVelocity_026814(pc) | +000
        jsr     Entity_MoveXY_Probe_026992(pc)  | +004
        jsr     0x44182.l                       | +008
        move.w  -0x1148(a5),d1                  | +00e
        move.w  -0x1146(a5),d2                  | +012
        subq.w  #0x1,d2                         | +016
        jsr     Sub_00027DB2(pc)                | +018
        add.w   -0x1154(a5),d1                  | +01c
        add.w   -0x1152(a5),d2                  | +020
        jsr     0x998ca.l                       | +024
        bcc.w   .L0272f8                        | +02a
        moveq   #0,d0                           | +02e
        moveq   #0,d3                           | +030
        moveq   #0,d4                           | +032
        moveq   #17,d0                          | +034
        move.l  d0,d7                           | +036
        lea     0x278ba8.l,a1                   | +038
        movea.l a1,a4                           | +03e
        movea.l 0x4(a1),a3                      | +040
        move.b  0x12(a3,d3.w),0x106f31.l        | +044
        bra.w   .L027372                        | +04c
.L0272f8:
        sub.w   -0x1154(a5),d1                  | +050
        sub.w   -0x1152(a5),d2                  | +054
        cmpi.b  #0x1,0xa(a1)                    | +058
        bne.w   .L027316                        | +05e
        tst.w   0x64(a6)                        | +062
        bne.w   .L027316                        | +066
        adda.w  #0x10,a1                        | +06a
.L027316:
        jsr     Sub_00027E9C(pc)                | +06e  -> $027E9C (hueco futuro, defsym forward)
        bcs.w   .L027342                        | +072
        jsr     Sub_00027E7E(pc)                | +076  -> $027E7E (hueco futuro, defsym forward)
        bcs.w   .L027336                        | +07a
        btst    #0x0,0x100000.l                 | +07e
        beq.w   .L027332                        | +086
.L027332:
        bra.w   .L027342                        | +08a
.L027336:
        btst    #0x0,0x100000.l                 | +08e
        beq.w   .L027342                        | +096
.L027342:
        add.w   -0x1154(a5),d1                  | +09a
        add.w   -0x1152(a5),d2                  | +09e
        movem.w d3-d4,-(a7)                     | +0a2
        movea.l a1,a4                           | +0a6
        jsr     Sub_00027DB2(pc)                | +0a8
        cmpi.b  #0x1,0xa(a1)                    | +0ac
        bne.w   .L02736a                        | +0b2
        tst.w   0x34(a6)                        | +0b6
        bne.w   .L02736a                        | +0ba
        adda.w  #0x10,a1                        | +0be
.L02736a:
        movem.w (a7)+,d3-d4                     | +0c2
        movea.l 0x4(a1),a3                      | +0c6
.L027372:
        cmpi.b  #0x20,d0                        | +0ca
        beq.w   .L027396                        | +0ce
        cmpi.b  #0x21,d0                        | +0d2
        beq.w   .L027396                        | +0d6
        cmpi.b  #0x30,d0                        | +0da
        beq.w   .L027396                        | +0de
        cmpi.b  #0x31,d0                        | +0e2
        beq.w   .L027396                        | +0e6
        bra.w   .L0273a6                        | +0ea
.L027396:
        btst    #0x4,0x13(a6)                   | +0ee
        bne.w   .L0273a6                        | +0f4
        lea     0x278704.l,a3                   | +0f8
.L0273a6:
        cmpi.b  #0x10,d0                        | +0fe
        beq.w   .L0273b6                        | +102
        cmpi.b  #0x27,d0                        | +106
        bne.w   .L0273c0                        | +10a
.L0273b6:
        bset    #0x5,0x5a(a6)                   | +10e
        bra.w   SetXN_0273d6                    | +114
.L0273c0:
        move.w  0x64(a6),d6                     | +118
        cmp.w   0x1a(a3),d6                     | +11c
        bne.w   SetXN_0273d6                    | +120
        move.w  0x34(a6),d6                     | +124
        cmp.w   (a3),d6                         | +128
        beq.w   Entity_CommitMove_C_0273dc      | +12a

| ----------------------------------------------------------------------------
|  Entity_CommitMove_C_0273dc  @ $0273DC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CommitMove_C_0273dc, "ax", @progbits
        .global Entity_CommitMove_C_0273dc
Entity_CommitMove_C_0273dc:
        addq.w  #0x1,d2                         | +000
        move.w  d1,-0x1148(a5)                  | +002
        move.w  d2,-0x1146(a5)                  | +006
        move.b  -0x1150(a5),d1                  | +00a
        move.b  -0x114f(a5),d2                  | +00e
        move.b  d1,-0x1144(a5)                  | +012
        move.b  d2,-0x1143(a5)                  | +016

| ----------------------------------------------------------------------------
|  Entity_SaveRegs_0273fc  @ $0273FC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SaveRegs_0273fc, "ax", @progbits
        .global Entity_SaveRegs_0273fc
Entity_SaveRegs_0273fc:
        movem.l d3-d6/a1,-(a7)                  | +000
