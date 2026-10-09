| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave AAAAA — tabla de escenas, scripts de la VM de scroll (con callbacks 68000 embebidos), entidades-cámara y tablas de trigger
|  Región: $0916C8..$0967B4  (20,716 B, 56 entradas)
| ============================================================================
|
|  A. QUÉ ES ESTA REGIÓN
|  ----------------------------------------------------------------------------
|  Datos del subsistema de ESCENA/SCROLL (los "mapas" del juego): la tabla de
|  descriptores $0916C8 que `SceneLoader_Main_043568` indexa con el número de
|  escena (d0, 0..15) y todo lo que cuelga de ella. Cinco familias:
|
|    SceneDescTable_0916C8[16]   pares {script_vm.l, entities.l} (8 B). Las
|                                "entradas 16..255" que el loader admitiría NO
|                                existen: en $91748 ya empieza el primer script
|                                de entidades (tabla y datos solapados en ROM).
|    SceneEntities_XXXXXX  (14)  registros de 14 B {type, subop, tmpl.l,
|                                payload[8]} + terminador type=2. Los "templates"
|                                son SIEMPRE los 4 contextos de cámara
|                                ($1080E0/$108064/$107FE8/$106F6C, stride $7C,
|                                Wave CC#1): type 0 = cámara con scratch A
|                                (contador $20 en d1), type 1 = scratch B.
|    SceneScript_XXXXXX    (14)  bytecode de `SceneScriptVM_Frame_0437DA`
|                                transcrito op a op con los strides EXACTOS de
|                                la VM (scene_script_vm_0437da.s). Incluye 191
|                                CALLBACKS 68000 EMBEBIDOS (op $06 / $14).
|    SceneTrig_XXXXXX      (26)  tablas de words `{valor, umbral}` terminadas
|                                en $FFFF, instaladas por op $11 en $12(ent)
|                                (`move.l a1,$12(a0)`, $51B38): rampas de
|                                parallax/altura que la cámara consume según
|                                el progreso del scroll.
|    ChildRank_CmpByte10_0967A4  cola de CÓDIGO huérfana (16 B, sin callers):
|                                comparador CCR `cmp.b $10(a1),d0` que cae en
|                                la pareja ClearXN_0967b4/SetXN_0967ba. Hay
|                                copias byte-idénticas en $05279E y $097720
|                                (attract_sprite_lists_096bbc.s): epílogo que
|                                el ensamblador original pegaba tras cada
|                                bloque de datos.
|
|  Mapa de escenas (índice -> caller que hace `move.b #n,d0 ; jsr $43568`):
|    0..5  Attract_State0..5_Handler_0967FE..  (demo attract = 6 misiones)
|    6     = alias de la escena 1   7 = alias de la escena 2 (Attract state 7)
|    8     ($96580) escena corta, 3 cámaras (sin caller directo conocido)
|    9     anim_state_machine_08cxxx (+42)       10  GameOver_Boot_08f91a
|    11    HiScore_Tpl_Common_097816             12  ($9412C) sin caller directo
|    13    ($96782) escena vacía: 1 cámara, límites 0 (pantalla fija)
|    14/15 cutscene_anim_08baxx (+014 / +028,+06c): escenas de la cutscene
|          final (las dos más largas: 4,420 y 3,780 B, con 60+ callbacks).
|  La escena 0 también la carga `MemCard_LoadScene_05164a` vía la entry corta
|  $43562 (índice implícito 0).
|
|  B. EVIDENCIA
|  ----------------------------------------------------------------------------
|  * `lea.l 0x916c8.l,a0` en SceneLoader_Main_043568 (+06); `move.l (a0,d0.w),
|    $10815C` fija el PC de la VM = campo +0 del descriptor; `movea.l 4(a0,d0.w),
|    a0` = script de entidades (+4).
|  * tools/scene_script_dump.py recorre los 14 scripts con los 23 strides de la
|    VM sin UN SOLO opcode inválido; cada script termina exactamente donde
|    empieza la siguiente tabla/entidad. El op $06 cede el frame si el callback
|    devuelve d0!=0 y recarga el PC desde a1. Los 184 callbacks op $06 son:
|        cmpi.w #X,$106F50.l ; scs d0 ; lea next(pc),a1 ; rts      (x136)
|        cmpi.b #k,$10E39A.l ; sne d0 ; lea next(pc),a1 ; rts      (x25)
|        cmpi.w #Y,$106F54.l ; s?? d0 ; ...  / cmpi.b $10A2CF / cmpi.w $106E88
|    = "espera a que el scroll X llegue a X" (scs: C=1 mientras pos < X),
|    "espera al flag de borde $10E39A" (SceneScript_EdgeArrivalTest) o
|    "espera a que la cámara Y alcance Y". Los 7 callbacks op $14 son
|    `move.b #v,$10E39B/C.l` o `move.l #-$10000,$106F64.l` + `lea next(pc),a0`.
|  * op $0D (60 usos) apunta a los pre-thunks $52756/$5276C/$52776/$52780/
|    $52796 (cargan d1..d4 / d0 desde los 8 B de args inline y llaman a
|    ThunkTarget_002c26 / _05239e / _0523b2 / _05dd2a / _05026c) y a
|    Sub_000022C8 (x8).
|  * op $0E (spawn de tarea) no aparece en ninguna escena: las tareas se crean
|    con op $04 (`$51B1C`, 53 usos) sobre los 4 contextos de cámara.
|  * op $0A: 22 bloques `{slot,bank}` para Sub_00002B58 (slots de tiles ->
|    bancos $1CE00 + bank*64); la escena 0 remapea 116 slots de golpe.
|  * Las 26 SceneTrig están referenciadas por 37 op $11 (varias compartidas
|    entre las escenas 0/14 y 5/15: los mismos mapas reutilizados en la
|    cutscene final).
|  * Listas attract: ver attract_sprite_lists_096bbc.s (JumpTable_096B9C).
|
|  C. HIPÓTESIS (no verificadas en emulador)
|  ----------------------------------------------------------------------------
|  * Las escenas 0..5 son los 6 mapas de la demo attract = las 6 misiones
|    (misma numeración que MissionStream_Slot00..05). Los scripts 1 y 2 se
|    reutilizan como 6 y 7 para el segundo ciclo de la demo.
|  * `$10E39A` es el flag "scroll llegó al borde" que arma la VM (op $02 ->
|    SceneScript_EdgeArrivalTest) y que los callbacks `cmpi.b #1,$10E39A ; sne`
|    esperan; `$10E39B/C` son sub-flags que las escenas 14/15 fijan con op $14
|    para sincronizar la cutscene con cutscene_anim_08baxx.
|  * `$10A2CF` (cmpi.b x8) es el contador de fase del jefe/escena que los
|    scripts esperan antes de abrir el siguiente tramo de scroll.
|  * SceneTrig {valor,umbral}: `valor` = byte alto altura/parallax + byte bajo
|    sub-índice; `umbral` = progreso de scroll a partir del cual se aplica.
|    Consumidor exacto de $12(ent) pendiente (camera_list_ctx_helpers /
|    CameraApplyAll4).
|
|  D. FORMATOS
|  ----------------------------------------------------------------------------
|      struct SceneDescriptor { u8 *vm_script; SceneEntity *entities; };   /* 8 B  */
|      struct SceneEntity { u8 type; u8 subop; u32 camera_ctx; u8 payload[8]; }; /* 14 B */
|      op $00 bind_path  22 B: ent.l, ruta[8], cont.l, ancla_x.w, ancla_y.w
|      op $01 set_fields 12 B: ent.l, b72, pad, w74, w76
|      op $03 warp 6 B / $15 set_campos 6 B: x.w, y.w
|      op $04 spawn 8 B: tmpl.l, a.b, b.b          op $0D call_args 14 B: fn.l, args[8]
|      op $0F set_limits 12 B: minX, minY, maxX, maxY, slope
|      op $11 set_trig 10 B: ent.l, tabla.l        op $0A: pares {slot.w,bank.w} + $FFFF
|      op $06/$14: 2 B + código 68000 inline; el callback devuelve el nuevo PC
|      (a1 / a0) con `lea next(pc)` -> aquí etiquetas .Lxxxxxx.
|
|  E. HELPERS / RAM
|  ----------------------------------------------------------------------------
|  $10815C PC de la VM; $106F50 scroll X (16.16); $106F54 Y suave; $106F5C
|  high-water; $108168..70 límites; $108178 modo de eje; $10E39A flag borde;
|  $106EAC byte de escena (op $0C); $51B1C spawn, $51B38 set_trig, $51B3E
|  bind_path, $51ECE/$51ED6/$51F02 hooks de cámara, $2B58 slot_pairs.
|
|  F. ESTADO
|  ----------------------------------------------------------------------------
|  65 entradas (56 aquí + 9 en attract_sprite_lists_096bbc.s), 23,648 B,
|  todo byte-exacto. Generado por tools/scene_script_dump.py (regenerable);
|  nombres por familia + dirección. Cierra al 100 % la zona CODE
|  `$083000..$09C608`.
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

        .globl  SceneDescTable_0916C8
        .section .text.SceneDescTable_0916C8, "ax", @progbits
SceneDescTable_0916C8:                      | 16 descriptores x 8 B = 128 B (escenas 0..15)
        .dc.l   0x00091782,0x00091748            | [ 0] vm=$091782 entities=$091748  attract state 0
        .dc.l   0x00092034,0x00091ffa            | [ 1] vm=$092034 entities=$091FFA  attract state 1
        .dc.l   0x0009256e,0x00092534            | [ 2] vm=$09256E entities=$092534  attract state 2
        .dc.l   0x00092bd2,0x00092b98            | [ 3] vm=$092BD2 entities=$092B98  attract state 3
        .dc.l   0x000932f8,0x000932be            | [ 4] vm=$0932F8 entities=$0932BE  attract state 4
        .dc.l   0x0009357c,0x00093542            | [ 5] vm=$09357C entities=$093542  attract state 5
        .dc.l   0x00092034,0x00091ffa            | [ 6] vm=$092034 entities=$091FFA  = escena 1 (alias)
        .dc.l   0x0009256e,0x00092534            | [ 7] vm=$09256E entities=$092534  attract state 7 / = escena 2
        .dc.l   0x00096580,0x00096554            | [ 8] vm=$096580 entities=$096554  
        .dc.l   0x00093cda,0x00093ca0            | [ 9] vm=$093CDA entities=$093CA0  anim_state_machine_08cxxx (+42)
        .dc.l   0x00093f30,0x00093f20            | [10] vm=$093F30 entities=$093F20  GameOver_Boot_08f91a
        .dc.l   0x00094322,0x00094312            | [11] vm=$094322 entities=$094312  HiScore_Tpl_Common_097816
        .dc.l   0x0009412c,0x0009411c            | [12] vm=$09412C entities=$09411C  
        .dc.l   0x00096782,0x00096772            | [13] vm=$096782 entities=$096772  
        .dc.l   0x00094512,0x000944d8            | [14] vm=$094512 entities=$0944D8  cutscene_anim_08baxx (+014)
        .dc.l   0x00095690,0x00095656            | [15] vm=$095690 entities=$095656  cutscene_anim_08baxx (+028/+06c)

        .globl  SceneEntities_091748
        .section .text.SceneEntities_091748, "ax", @progbits
SceneEntities_091748:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0xff80,0x0015,0x0016
                | $091748 type=1 subop=00 tmpl=$1080E0 payload=0000ff8000150016
        .dc.w   0x0100,0x0010,0x8064,0x0000,0xff80,0x0015,0x0016
                | $091756 type=1 subop=00 tmpl=$108064 payload=0000ff8000150016
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0xff80,0x0015,0x001a
                | $091764 type=0 subop=00 tmpl=$107FE8 payload=0000ff800015001a
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0xff80,0x0020,0x0019
                | $091772 type=1 subop=00 tmpl=$106F6C payload=ffa0ff8000200019
        .dc.w   0x0250                        | $091780 terminador type=2

        .globl  SceneScript_091782
        .section .text.SceneScript_091782, "ax", @progbits
SceneScript_091782:                         | bytecode VM de escena (1898 B, 114 ops)
        .dc.w   0x0003,0x0000,0x0080
                | $091782 op $03 warp: camara=(0,128) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $091788 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x00c0                 |   slot $10 <- bank $00C0
        .dc.w   0x0011,0x00c1                 |   slot $11 <- bank $00C1
        .dc.w   0x0012,0x00c2                 |   slot $12 <- bank $00C2
        .dc.w   0x0013,0x00c3                 |   slot $13 <- bank $00C3
        .dc.w   0x0014,0x00c4                 |   slot $14 <- bank $00C4
        .dc.w   0x0015,0x00c5                 |   slot $15 <- bank $00C5
        .dc.w   0x0016,0x00c6                 |   slot $16 <- bank $00C6
        .dc.w   0x0017,0x00c7                 |   slot $17 <- bank $00C7
        .dc.w   0x0018,0x00c8                 |   slot $18 <- bank $00C8
        .dc.w   0x0019,0x00c9                 |   slot $19 <- bank $00C9
        .dc.w   0x001a,0x00ca                 |   slot $1A <- bank $00CA
        .dc.w   0x001b,0x00cb                 |   slot $1B <- bank $00CB
        .dc.w   0x001c,0x00cc                 |   slot $1C <- bank $00CC
        .dc.w   0x001d,0x00cd                 |   slot $1D <- bank $00CD
        .dc.w   0x001e,0x00ce                 |   slot $1E <- bank $00CE
        .dc.w   0x001f,0x00cf                 |   slot $1F <- bank $00CF
        .dc.w   0x0020,0x00d0                 |   slot $20 <- bank $00D0
        .dc.w   0x0021,0x00d1                 |   slot $21 <- bank $00D1
        .dc.w   0x0022,0x00d2                 |   slot $22 <- bank $00D2
        .dc.w   0x0023,0x00d3                 |   slot $23 <- bank $00D3
        .dc.w   0x0024,0x00d4                 |   slot $24 <- bank $00D4
        .dc.w   0x0025,0x00d5                 |   slot $25 <- bank $00D5
        .dc.w   0x0026,0x00d6                 |   slot $26 <- bank $00D6
        .dc.w   0x0027,0x00d7                 |   slot $27 <- bank $00D7
        .dc.w   0x0028,0x00d8                 |   slot $28 <- bank $00D8
        .dc.w   0x0029,0x00d9                 |   slot $29 <- bank $00D9
        .dc.w   0x002a,0x00da                 |   slot $2A <- bank $00DA
        .dc.w   0x002b,0x00db                 |   slot $2B <- bank $00DB
        .dc.w   0x002c,0x00dc                 |   slot $2C <- bank $00DC
        .dc.w   0x002d,0x00dd                 |   slot $2D <- bank $00DD
        .dc.w   0x002e,0x00de                 |   slot $2E <- bank $00DE
        .dc.w   0x002f,0x00df                 |   slot $2F <- bank $00DF
        .dc.w   0x0030,0x00e0                 |   slot $30 <- bank $00E0
        .dc.w   0x0031,0x00e1                 |   slot $31 <- bank $00E1
        .dc.w   0x0032,0x00e2                 |   slot $32 <- bank $00E2
        .dc.w   0x0033,0x00e3                 |   slot $33 <- bank $00E3
        .dc.w   0x0034,0x00e4                 |   slot $34 <- bank $00E4
        .dc.w   0x0035,0x00e5                 |   slot $35 <- bank $00E5
        .dc.w   0x0036,0x00e6                 |   slot $36 <- bank $00E6
        .dc.w   0x0037,0x00e7                 |   slot $37 <- bank $00E7
        .dc.w   0x0038,0x00e8                 |   slot $38 <- bank $00E8
        .dc.w   0x0039,0x00e9                 |   slot $39 <- bank $00E9
        .dc.w   0x003a,0x00ea                 |   slot $3A <- bank $00EA
        .dc.w   0x003b,0x00eb                 |   slot $3B <- bank $00EB
        .dc.w   0x003c,0x00ec                 |   slot $3C <- bank $00EC
        .dc.w   0x003d,0x00ed                 |   slot $3D <- bank $00ED
        .dc.w   0x003e,0x00ee                 |   slot $3E <- bank $00EE
        .dc.w   0x003f,0x00ef                 |   slot $3F <- bank $00EF
        .dc.w   0x0040,0x00f0                 |   slot $40 <- bank $00F0
        .dc.w   0x0041,0x00f1                 |   slot $41 <- bank $00F1
        .dc.w   0x0042,0x00f2                 |   slot $42 <- bank $00F2
        .dc.w   0x0043,0x00f3                 |   slot $43 <- bank $00F3
        .dc.w   0x0044,0x00f4                 |   slot $44 <- bank $00F4
        .dc.w   0x0045,0x00f5                 |   slot $45 <- bank $00F5
        .dc.w   0x0046,0x00f6                 |   slot $46 <- bank $00F6
        .dc.w   0x0047,0x00f7                 |   slot $47 <- bank $00F7
        .dc.w   0x0048,0x00f8                 |   slot $48 <- bank $00F8
        .dc.w   0x0049,0x00f9                 |   slot $49 <- bank $00F9
        .dc.w   0x004a,0x00fa                 |   slot $4A <- bank $00FA
        .dc.w   0x004b,0x00fb                 |   slot $4B <- bank $00FB
        .dc.w   0x004c,0x00fc                 |   slot $4C <- bank $00FC
        .dc.w   0x004d,0x00fd                 |   slot $4D <- bank $00FD
        .dc.w   0x004e,0x00fe                 |   slot $4E <- bank $00FE
        .dc.w   0x004f,0x00ff                 |   slot $4F <- bank $00FF
        .dc.w   0x0050,0x0100                 |   slot $50 <- bank $0100
        .dc.w   0x0051,0x0101                 |   slot $51 <- bank $0101
        .dc.w   0x0052,0x0102                 |   slot $52 <- bank $0102
        .dc.w   0x0053,0x0103                 |   slot $53 <- bank $0103
        .dc.w   0x0054,0x0104                 |   slot $54 <- bank $0104
        .dc.w   0x0055,0x0105                 |   slot $55 <- bank $0105
        .dc.w   0x0056,0x0106                 |   slot $56 <- bank $0106
        .dc.w   0x0057,0x0107                 |   slot $57 <- bank $0107
        .dc.w   0x0058,0x0108                 |   slot $58 <- bank $0108
        .dc.w   0x0059,0x0109                 |   slot $59 <- bank $0109
        .dc.w   0x005a,0x010a                 |   slot $5A <- bank $010A
        .dc.w   0x005b,0x010b                 |   slot $5B <- bank $010B
        .dc.w   0x005c,0x010c                 |   slot $5C <- bank $010C
        .dc.w   0x005d,0x010d                 |   slot $5D <- bank $010D
        .dc.w   0x005e,0x010e                 |   slot $5E <- bank $010E
        .dc.w   0x005f,0x010f                 |   slot $5F <- bank $010F
        .dc.w   0x0060,0x0110                 |   slot $60 <- bank $0110
        .dc.w   0x0061,0x0111                 |   slot $61 <- bank $0111
        .dc.w   0x0062,0x01d0                 |   slot $62 <- bank $01D0
        .dc.w   0x0063,0x01d1                 |   slot $63 <- bank $01D1
        .dc.w   0x0064,0x01d2                 |   slot $64 <- bank $01D2
        .dc.w   0x0065,0x01d3                 |   slot $65 <- bank $01D3
        .dc.w   0x0066,0x01d4                 |   slot $66 <- bank $01D4
        .dc.w   0x0067,0x01d5                 |   slot $67 <- bank $01D5
        .dc.w   0x0068,0x01d6                 |   slot $68 <- bank $01D6
        .dc.w   0x0069,0x01d7                 |   slot $69 <- bank $01D7
        .dc.w   0x006a,0x01d8                 |   slot $6A <- bank $01D8
        .dc.w   0x006b,0x01d9                 |   slot $6B <- bank $01D9
        .dc.w   0x006c,0x01da                 |   slot $6C <- bank $01DA
        .dc.w   0x006d,0x01db                 |   slot $6D <- bank $01DB
        .dc.w   0x006e,0x01dc                 |   slot $6E <- bank $01DC
        .dc.w   0x006f,0x01dd                 |   slot $6F <- bank $01DD
        .dc.w   0x0070,0x01de                 |   slot $70 <- bank $01DE
        .dc.w   0x0071,0x01df                 |   slot $71 <- bank $01DF
        .dc.w   0x0072,0x017d                 |   slot $72 <- bank $017D
        .dc.w   0x0073,0x017e                 |   slot $73 <- bank $017E
        .dc.w   0x0074,0x017f                 |   slot $74 <- bank $017F
        .dc.w   0x00ff,0x0109                 |   slot $FF <- bank $0109
        .dc.w   0xffff                              | $091922 fin de pares
        .dc.w   0x000c,0x0800
                | $091924 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x000f,0x05e0,0x0090,0x0600,0x0090,0x0000
                | $091928 op $0F set_limits: minX=1504 minY=144 maxX=1536 maxY=144 slope=0
        .dc.w   0x0010,0x0078,0x00a0
                | $091934 op $10 set_6466: $108164=0078 $108166=00A0
        .dc.w   0x0000,0x0010,0x6f6c,0x0078,0x0012,0x000a,0x18a8,0x0020,0x0000,0x0000,0x0080
                | $09193A op $00 bind_path: ent=$106F6C ruta=$780012.. cont=$200000 ancla=(0,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1ee0
                | $091950 op $11 set_trig: ent=$106F6C tabla=$091EE0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $09195A op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $091966 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $09196C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09196E --- callback 68000 embebido (16 B) -> PC=$09197E ---
        cmpi.w  #0x150, 0x106f50.l             | $09196E  cmpi.w #$150, $106f50.l
        scs     d0                             | $091976  scs.b d0
        lea     .L09197e(pc), a1              | $091978  a1 = nuevo PC
        rts                                    | $09197C  rts
.L09197e:
        .dc.w   0x0000,0x0010,0x7fe8,0x004b,0x0004,0x000a,0x13f8,0x0000,0x0000,0x0290,0x0160
                | $09197E op $00 bind_path: ent=$107FE8 ruta=$4B0004.. cont=$000000 ancla=(656,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $091994 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $09199C op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $0919A8 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0919AE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0919B0 --- callback 68000 embebido (16 B) -> PC=$0919C0 ---
        cmpi.w  #0x2e0, 0x106f50.l             | $0919B0  cmpi.w #$2e0, $106f50.l
        scs     d0                             | $0919B8  scs.b d0
        lea     .L0919c0(pc), a1              | $0919BA  a1 = nuevo PC
        rts                                    | $0919BE  rts
.L0919c0:
        .dc.w   0x0005,0x0010,0x80e0
                | $0919C0 op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $0919C6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0919C8 --- callback 68000 embebido (16 B) -> PC=$0919D8 ---
        cmpi.w  #0x590, 0x106f50.l             | $0919C8  cmpi.w #$590, $106f50.l
        scs     d0                             | $0919D0  scs.b d0
        lea     .L0919d8(pc), a1              | $0919D2  a1 = nuevo PC
        rts                                    | $0919D6  rts
.L0919d8:
        .dc.w   0x0000,0x0010,0x8064,0x0060,0x0012,0x000a,0x66a8,0x0000,0x0000,0x06d0,0x0030
                | $0919D8 op $00 bind_path: ent=$108064 ruta=$600012.. cont=$000000 ancla=(1744,48)
        .dc.w   0x0004,0x0010,0x8064,0x1200
                | $0919EE op $04 spawn: tmpl=$108064 a=12 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x00c0,0x0100
                | $0919F6 op $01 set_fields: ent=$108064 +72=03 +74=$00C0 +76=$0100
        .dc.w   0x0007,0x0010,0x8064
                | $091A02 op $07 call_ece: ent=$108064  ($51ECE)
        .dc.w   0x0006
                | $091A08 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091A0A --- callback 68000 embebido (16 B) -> PC=$091A1A ---
        cmpi.w  #0x5e0, 0x106f50.l             | $091A0A  cmpi.w #$5e0, $106f50.l
        scs     d0                             | $091A12  scs.b d0
        lea     .L091a1a(pc), a1              | $091A14  a1 = nuevo PC
        rts                                    | $091A18  rts
.L091a1a:
        .dc.w   0x0000,0x0010,0x6f6c,0x005c,0x0012,0x000a,0x3a68,0x0020,0x21c0,0x0780,0x0080
                | $091A1A op $00 bind_path: ent=$106F6C ruta=$5C0012.. cont=$2021C0 ancla=(1920,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1f0a
                | $091A30 op $11 set_trig: ent=$106F6C tabla=$091F0A  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $091A3A op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x6f6c
                | $091A46 op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0006
                | $091A4C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091A4E --- callback 68000 embebido (16 B) -> PC=$091A5E ---
        cmpi.w  #0x600, 0x106f50.l             | $091A4E  cmpi.w #$600, $106f50.l
        scs     d0                             | $091A56  scs.b d0
        lea     .L091a5e(pc), a1              | $091A58  a1 = nuevo PC
        rts                                    | $091A5C  rts
.L091a5e:
        .dc.w   0x0006
                | $091A5E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091A60 --- callback 68000 embebido (16 B) -> PC=$091A70 ---
        cmpi.w  #0x0, 0x106e88.l               | $091A60  cmpi.w #$0, $106e88.l
        sne     d0                             | $091A68  sne.b d0
        lea     .L091a70(pc), a1              | $091A6A  a1 = nuevo PC
        rts                                    | $091A6E  rts
.L091a70:
        .dc.w   0x000f,0x0900,0x0090,0x0920,0x0090,0x0000
                | $091A70 op $0F set_limits: minX=2304 minY=144 maxX=2336 maxY=144 slope=0
        .dc.w   0x0006
                | $091A7C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091A7E --- callback 68000 embebido (16 B) -> PC=$091A8E ---
        cmpi.w  #0x600, 0x106f50.l             | $091A7E  cmpi.w #$600, $106f50.l
        scs     d0                             | $091A86  scs.b d0
        lea     .L091a8e(pc), a1              | $091A88  a1 = nuevo PC
        rts                                    | $091A8C  rts
.L091a8e:
        .dc.w   0x0000,0x0010,0x7fe8,0x0004,0x0004,0x000a,0x0000,0x0000,0x0000,0x0740,0x0160
                | $091A8E op $00 bind_path: ent=$107FE8 ruta=$040004.. cont=$000000 ancla=(1856,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $091AA4 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $091AAC op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x7fe8
                | $091AB8 op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $091ABE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091AC0 --- callback 68000 embebido (16 B) -> PC=$091AD0 ---
        cmpi.w  #0x640, 0x106f50.l             | $091AC0  cmpi.w #$640, $106f50.l
        scs     d0                             | $091AC8  scs.b d0
        lea     .L091ad0(pc), a1              | $091ACA  a1 = nuevo PC
        rts                                    | $091ACE  rts
.L091ad0:
        .dc.w   0x0000,0x0010,0x7fe8,0x0036,0x0004,0x000a,0x0040,0x0000,0x0000,0x0780,0x0160
                | $091AD0 op $00 bind_path: ent=$107FE8 ruta=$360004.. cont=$000000 ancla=(1920,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $091AE6 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0007,0x0010,0x7fe8
                | $091AEE op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $091AF4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091AF6 --- callback 68000 embebido (16 B) -> PC=$091B06 ---
        cmpi.w  #0x920, 0x106f50.l             | $091AF6  cmpi.w #$920, $106f50.l
        scs     d0                             | $091AFE  scs.b d0
        lea     .L091b06(pc), a1              | $091B00  a1 = nuevo PC
        rts                                    | $091B04  rts
.L091b06:
        .dc.w   0x0006
                | $091B06 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091B08 --- callback 68000 embebido (16 B) -> PC=$091B18 ---
        cmpi.b  #0x1, 0x10e39a.l               | $091B08  cmpi.b #$1, $10e39a.l
        sne     d0                             | $091B10  sne.b d0
        lea     .L091b18(pc), a1              | $091B12  a1 = nuevo PC
        rts                                    | $091B16  rts
.L091b18:
        .dc.w   0x000f,0x0a20,0x0090,0x0a30,0x0090,0x0000
                | $091B18 op $0F set_limits: minX=2592 minY=144 maxX=2608 maxY=144 slope=0
        .dc.w   0x0006
                | $091B24 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091B26 --- callback 68000 embebido (16 B) -> PC=$091B36 ---
        cmpi.w  #0x980, 0x106f50.l             | $091B26  cmpi.w #$980, $106f50.l
        scs     d0                             | $091B2E  scs.b d0
        lea     .L091b36(pc), a1              | $091B30  a1 = nuevo PC
        rts                                    | $091B34  rts
.L091b36:
        .dc.w   0x0000,0x0010,0x80e0,0x001e,0x0001,0x000a,0x81a8,0x0000,0x0000,0x0ac0,0x0150
                | $091B36 op $00 bind_path: ent=$1080E0 ruta=$1E0001.. cont=$000000 ancla=(2752,336)
        .dc.w   0x0004,0x0010,0x80e0,0x0100
                | $091B4C op $04 spawn: tmpl=$1080E0 a=01 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0100,0x0100
                | $091B54 op $01 set_fields: ent=$1080E0 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $091B60 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000c,0x0000
                | $091B66 op $0C set_6eac: $106EAC.b = 00
        .dc.w   0x0006
                | $091B6A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091B6C --- callback 68000 embebido (16 B) -> PC=$091B7C ---
        cmpi.w  #0x9a0, 0x106f50.l             | $091B6C  cmpi.w #$9a0, $106f50.l
        scs     d0                             | $091B74  scs.b d0
        lea     .L091b7c(pc), a1              | $091B76  a1 = nuevo PC
        rts                                    | $091B7A  rts
.L091b7c:
        .dc.w   0x0000,0x0010,0x7fe8,0x0008,0x000b,0x000a,0x10f8,0x0000,0x0000,0x0ae0,0x00f0
                | $091B7C op $00 bind_path: ent=$107FE8 ruta=$08000B.. cont=$000000 ancla=(2784,240)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1ed6
                | $091B92 op $11 set_trig: ent=$107FE8 tabla=$091ED6  (-> $12(ent))
        .dc.w   0x0007,0x0010,0x7fe8
                | $091B9C op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $091BA2 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091BA4 --- callback 68000 embebido (16 B) -> PC=$091BB4 ---
        cmpi.w  #0xa18, 0x106f50.l             | $091BA4  cmpi.w #$a18, $106f50.l
        scs     d0                             | $091BAC  scs.b d0
        lea     .L091bb4(pc), a1              | $091BAE  a1 = nuevo PC
        rts                                    | $091BB2  rts
.L091bb4:
        .dc.w   0x0006
                | $091BB4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091BB6 --- callback 68000 embebido (16 B) -> PC=$091BC6 ---
        cmpi.b  #0x1, 0x10e39a.l               | $091BB6  cmpi.b #$1, $10e39a.l
        sne     d0                             | $091BBE  sne.b d0
        lea     .L091bc6(pc), a1              | $091BC0  a1 = nuevo PC
        rts                                    | $091BC4  rts
.L091bc6:
        .dc.w   0x000f,0x0a40,0x0090,0x0a58,0x0090,0x0000
                | $091BC6 op $0F set_limits: minX=2624 minY=144 maxX=2648 maxY=144 slope=0
        .dc.w   0x0006
                | $091BD2 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091BD4 --- callback 68000 embebido (16 B) -> PC=$091BE4 ---
        cmpi.w  #0xa20, 0x106f50.l             | $091BD4  cmpi.w #$a20, $106f50.l
        scs     d0                             | $091BDC  scs.b d0
        lea     .L091be4(pc), a1              | $091BDE  a1 = nuevo PC
        rts                                    | $091BE2  rts
.L091be4:
        .dc.w   0x0000,0x0010,0x7fe8,0x0022,0x0004,0x000a,0x1258,0x0000,0x0000,0x0b60,0x0160
                | $091BE4 op $00 bind_path: ent=$107FE8 ruta=$220004.. cont=$000000 ancla=(2912,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $091BFA op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0007,0x0010,0x7fe8
                | $091C02 op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $091C08 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091C0A --- callback 68000 embebido (16 B) -> PC=$091C1A ---
        cmpi.b  #0x2, 0x10e39a.l               | $091C0A  cmpi.b #$2, $10e39a.l
        sne     d0                             | $091C12  sne.b d0
        lea     .L091c1a(pc), a1              | $091C14  a1 = nuevo PC
        rts                                    | $091C18  rts
.L091c1a:
        .dc.w   0x000f,0x0b20,0x0090,0x0b40,0x0090,0x0000
                | $091C1A op $0F set_limits: minX=2848 minY=144 maxX=2880 maxY=144 slope=0
        .dc.w   0x0006
                | $091C26 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091C28 --- callback 68000 embebido (16 B) -> PC=$091C38 ---
        cmpi.w  #0xa90, 0x106f50.l             | $091C28  cmpi.w #$a90, $106f50.l
        scs     d0                             | $091C30  scs.b d0
        lea     .L091c38(pc), a1              | $091C32  a1 = nuevo PC
        rts                                    | $091C36  rts
.L091c38:
        .dc.w   0x0000,0x0010,0x80e0,0x0012,0x0013,0x000a,0x8220,0x0000,0x0000,0x0bd0,0x0030
                | $091C38 op $00 bind_path: ent=$1080E0 ruta=$120013.. cont=$000000 ancla=(3024,48)
        .dc.w   0x0004,0x0010,0x80e0,0x1300
                | $091C4E op $04 spawn: tmpl=$1080E0 a=13 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0100,0x0100
                | $091C56 op $01 set_fields: ent=$1080E0 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $091C62 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $091C68 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091C6A --- callback 68000 embebido (16 B) -> PC=$091C7A ---
        cmpi.w  #0xad0, 0x106f50.l             | $091C6A  cmpi.w #$ad0, $106f50.l
        scs     d0                             | $091C72  scs.b d0
        lea     .L091c7a(pc), a1              | $091C74  a1 = nuevo PC
        rts                                    | $091C78  rts
.L091c7a:
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0080,0x0100
                | $091C7A op $01 set_fields: ent=$1080E0 +72=03 +74=$0080 +76=$0100
        .dc.w   0x0006
                | $091C86 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091C88 --- callback 68000 embebido (16 B) -> PC=$091C98 ---
        cmpi.w  #0xb40, 0x106f50.l             | $091C88  cmpi.w #$b40, $106f50.l
        scs     d0                             | $091C90  scs.b d0
        lea     .L091c98(pc), a1              | $091C92  a1 = nuevo PC
        rts                                    | $091C96  rts
.L091c98:
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x00f1,0xffff,0x0000
                | $091C98 op $0D call_args: fn=$052756 args=00ff00f1ffff0000
        .dc.w   0x0006
                | $091CA6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091CA8 --- callback 68000 embebido (16 B) -> PC=$091CB8 ---
        cmpi.b  #0x1, 0x10e39a.l               | $091CA8  cmpi.b #$1, $10e39a.l
        sne     d0                             | $091CB0  sne.b d0
        lea     .L091cb8(pc), a1              | $091CB2  a1 = nuevo PC
        rts                                    | $091CB6  rts
.L091cb8:
        .dc.w   0x000f,0x0d00,0x0090,0x0d00,0x0090,0x0000
                | $091CB8 op $0F set_limits: minX=3328 minY=144 maxX=3328 maxY=144 slope=0
        .dc.w   0x0010,0x0078,0x00a0
                | $091CC4 op $10 set_6466: $108164=0078 $108166=00A0
        .dc.w   0x0006
                | $091CCA op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091CCC --- callback 68000 embebido (16 B) -> PC=$091CDC ---
        cmpi.w  #0xba0, 0x106f50.l             | $091CCC  cmpi.w #$ba0, $106f50.l
        scs     d0                             | $091CD4  scs.b d0
        lea     .L091cdc(pc), a1              | $091CD6  a1 = nuevo PC
        rts                                    | $091CDA  rts
.L091cdc:
        .dc.w   0x0000,0x0010,0x6f6c,0x0018,0x0017,0x000a,0x5448,0x0020,0x3ba0,0x0d40,0x0000
                | $091CDC op $00 bind_path: ent=$106F6C ruta=$180017.. cont=$203BA0 ancla=(3392,0)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1f58
                | $091CF2 op $11 set_trig: ent=$106F6C tabla=$091F58  (-> $12(ent))
        .dc.w   0x0007,0x0010,0x6f6c
                | $091CFC op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $091D02 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0006
                | $091D0E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091D10 --- callback 68000 embebido (16 B) -> PC=$091D20 ---
        cmpi.w  #0xba0, 0x106f50.l             | $091D10  cmpi.w #$ba0, $106f50.l
        scs     d0                             | $091D18  scs.b d0
        lea     .L091d20(pc), a1              | $091D1A  a1 = nuevo PC
        rts                                    | $091D1E  rts
.L091d20:
        .dc.w   0x0013,0x10b3
                | $091D20 op $13 clamp: d0=10B3  (ClampD0ToRange)
        .dc.w   0x0006
                | $091D24 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091D26 --- callback 68000 embebido (16 B) -> PC=$091D36 ---
        cmpi.w  #0xbc0, 0x106f50.l             | $091D26  cmpi.w #$bc0, $106f50.l
        scs     d0                             | $091D2E  scs.b d0
        lea     .L091d36(pc), a1              | $091D30  a1 = nuevo PC
        rts                                    | $091D34  rts
.L091d36:
        .dc.w   0x0000,0x0010,0x7fe8,0x001c,0x0018,0x000a,0x03a0,0x0000,0x0000,0x0d00,0x0020
                | $091D36 op $00 bind_path: ent=$107FE8 ruta=$1C0018.. cont=$000000 ancla=(3328,32)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1f8a
                | $091D4C op $11 set_trig: ent=$107FE8 tabla=$091F8A  (-> $12(ent))
        .dc.w   0x0007,0x0010,0x7fe8
                | $091D56 op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $091D5C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091D5E --- callback 68000 embebido (16 B) -> PC=$091D6E ---
        cmpi.w  #0xc80, 0x106f50.l             | $091D5E  cmpi.w #$c80, $106f50.l
        scs     d0                             | $091D66  scs.b d0
        lea     .L091d6e(pc), a1              | $091D68  a1 = nuevo PC
        rts                                    | $091D6C  rts
.L091d6e:
        .dc.w   0x000f,0x0e40,0x0020,0x0e40,0x0020,0x0040
                | $091D6E op $0F set_limits: minX=3648 minY=32 maxX=3648 maxY=32 slope=64
        .dc.w   0x0010,0x0078,0x00c0
                | $091D7A op $10 set_6466: $108164=0078 $108166=00C0
        .dc.w   0x0006
                | $091D80 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091D82 --- callback 68000 embebido (16 B) -> PC=$091D92 ---
        cmpi.w  #0xd20, 0x106f50.l             | $091D82  cmpi.w #$d20, $106f50.l
        scs     d0                             | $091D8A  scs.b d0
        lea     .L091d92(pc), a1              | $091D8C  a1 = nuevo PC
        rts                                    | $091D90  rts
.L091d92:
        .dc.w   0x0000,0x0010,0x6f6c,0x0018,0x0012,0x000a,0x5fe8,0x0020,0x4440,0x0ec0,0x0010
                | $091D92 op $00 bind_path: ent=$106F6C ruta=$180012.. cont=$204440 ancla=(3776,16)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1fcc
                | $091DA8 op $11 set_trig: ent=$106F6C tabla=$091FCC  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $091DB2 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x6f6c
                | $091DBE op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0006
                | $091DC4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091DC6 --- callback 68000 embebido (16 B) -> PC=$091DD6 ---
        cmpi.w  #0xd80, 0x106f50.l             | $091DC6  cmpi.w #$d80, $106f50.l
        scs     d0                             | $091DCE  scs.b d0
        lea     .L091dd6(pc), a1              | $091DD0  a1 = nuevo PC
        rts                                    | $091DD4  rts
.L091dd6:
        .dc.w   0x0000,0x0010,0x7fe8,0x001a,0x0007,0x000a,0x0e20,0x0000,0x0000,0x0ec0,0x00e0
                | $091DD6 op $00 bind_path: ent=$107FE8 ruta=$1A0007.. cont=$000000 ancla=(3776,224)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $091DEC op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1fea
                | $091DF8 op $11 set_trig: ent=$107FE8 tabla=$091FEA  (-> $12(ent))
        .dc.w   0x0009,0x0010,0x7fe8
                | $091E02 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0010,0x000c,0x000a,0x5ce8,0x0000,0x0000,0x0ec0,0x0020
                | $091E08 op $00 bind_path: ent=$108064 ruta=$10000C.. cont=$000000 ancla=(3776,32)
        .dc.w   0x0011,0x0010,0x8064,0x0009,0x1fbc
                | $091E1E op $11 set_trig: ent=$108064 tabla=$091FBC  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0100,0x0100
                | $091E28 op $01 set_fields: ent=$108064 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x8064
                | $091E34 op $07 call_ece: ent=$108064  ($51ECE)
        .dc.w   0x0006
                | $091E3A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091E3C --- callback 68000 embebido (16 B) -> PC=$091E4C ---
        cmpi.w  #0xdc0, 0x106f50.l             | $091E3C  cmpi.w #$dc0, $106f50.l
        scs     d0                             | $091E44  scs.b d0
        lea     .L091e4c(pc), a1              | $091E46  a1 = nuevo PC
        rts                                    | $091E4A  rts
.L091e4c:
        .dc.w   0x0013,0x10b4
                | $091E4C op $13 clamp: d0=10B4  (ClampD0ToRange)
        .dc.w   0x0006
                | $091E50 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091E52 --- callback 68000 embebido (16 B) -> PC=$091E62 ---
        cmpi.w  #0xde0, 0x106f50.l             | $091E52  cmpi.w #$de0, $106f50.l
        scs     d0                             | $091E5A  scs.b d0
        lea     .L091e62(pc), a1              | $091E5C  a1 = nuevo PC
        rts                                    | $091E60  rts
.L091e62:
        .dc.w   0x0010,0x0078,0x00d0
                | $091E62 op $10 set_6466: $108164=0078 $108166=00D0
        .dc.w   0x0006
                | $091E68 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091E6A --- callback 68000 embebido (16 B) -> PC=$091E7A ---
        cmpi.w  #0xe20, 0x106f50.l             | $091E6A  cmpi.w #$e20, $106f50.l
        scs     d0                             | $091E72  scs.b d0
        lea     .L091e7a(pc), a1              | $091E74  a1 = nuevo PC
        rts                                    | $091E78  rts
.L091e7a:
        .dc.w   0x0005,0x0010,0x80e0
                | $091E7A op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $091E80 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091E82 --- callback 68000 embebido (16 B) -> PC=$091E92 ---
        cmpi.w  #0xe40, 0x106f50.l             | $091E82  cmpi.w #$e40, $106f50.l
        scs     d0                             | $091E8A  scs.b d0
        lea     .L091e92(pc), a1              | $091E8C  a1 = nuevo PC
        rts                                    | $091E90  rts
.L091e92:
        .dc.w   0x0010,0x0058,0x00b0
                | $091E92 op $10 set_6466: $108164=0058 $108166=00B0
        .dc.w   0x0006
                | $091E98 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091E9A --- callback 68000 embebido (16 B) -> PC=$091EAA ---
        cmpi.w  #0xe40, 0x106f50.l             | $091E9A  cmpi.w #$e40, $106f50.l
        scs     d0                             | $091EA2  scs.b d0
        lea     .L091eaa(pc), a1              | $091EA4  a1 = nuevo PC
        rts                                    | $091EA8  rts
.L091eaa:
        .dc.w   0x000f,0x0ec0,0x0020,0x0f08,0x0020,0x0000
                | $091EAA op $0F set_limits: minX=3776 minY=32 maxX=3848 maxY=32 slope=0
        .dc.w   0x0006
                | $091EB6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $091EB8 --- callback 68000 embebido (16 B) -> PC=$091EC8 ---
        cmpi.w  #0xec0, 0x106f50.l             | $091EB8  cmpi.w #$ec0, $106f50.l
        scs     d0                             | $091EC0  scs.b d0
        lea     .L091ec8(pc), a1              | $091EC2  a1 = nuevo PC
        rts                                    | $091EC6  rts
.L091ec8:
        .dc.w   0x000f,0x0ec0,0x0010,0x0f08,0x0020,0x0000
                | $091EC8 op $0F set_limits: minX=3776 minY=16 maxX=3848 maxY=32 slope=0
        .dc.w   0x0002
                | $091ED4 op $02 END_FRAME: cede el frame (integra scroll+camara)
        .dc.w   0x0000,0x0b00,0x0008,0x0407,0xffff,0x0000,0x1200,0x0029,0x0f00,0x002d,0x1200
                | $091ED6 op $00 bind_path: ent=$B000008 ruta=$407FFFF.. cont=$290F00 ancla=(45,4608)

        .globl  SceneTrig_091EEC
        .section .text.SceneTrig_091EEC, "ax", @progbits
SceneTrig_091EEC:                           | tabla de trigger (op $11), 32 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0031,0x0f00,0x0035,0x1200,0x0038,0x0f00,0x0041,0x1200 | $091EEC
        .dc.w   0x0048,0x0f00,0x0067,0x1200,0x0070,0x0f00,0xffff,0x0000 | $091EFC

        .globl  SceneTrig_091F0C
        .section .text.SceneTrig_091F0C, "ax", @progbits
SceneTrig_091F0C:                           | tabla de trigger (op $11), 78 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0e00,0x000b,0x0a04,0x000e,0x0b03,0x000f,0x0c02,0x0010 | $091F0C
        .dc.w   0x0d01,0x0013,0x020c,0x0014,0x0806,0x0015,0x0905,0x0016 | $091F1C
        .dc.w   0x0a04,0x001c,0x0e00,0x0028,0x0806,0x002a,0x0608,0x002f | $091F2C
        .dc.w   0x0e00,0x0036,0x0707,0x0038,0x0b03,0x0041,0x0e00,0x0050 | $091F3C
        .dc.w   0x0608,0x0052,0x0000,0x0059,0x040a,0xffff,0x0000 | $091F4C

        .globl  SceneTrig_091F5A
        .section .text.SceneTrig_091F5A, "ax", @progbits
SceneTrig_091F5A:                           | tabla de trigger (op $11), 50 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0311,0x0005,0x060e,0x0007,0x060c,0x0008,0x0d05,0x0009 | $091F5A
        .dc.w   0x0d04,0x000b,0x0e03,0x000e,0x0d03,0x000f,0x0d02,0x0010 | $091F6A
        .dc.w   0x0c02,0x0012,0x0d01,0x0015,0x0c02,0x0018,0x0000,0xffff | $091F7A
        .dc.w   0x0000                                 | $091F8A

        .globl  SceneTrig_091F8C
        .section .text.SceneTrig_091F8C, "ax", @progbits
SceneTrig_091F8C:                           | tabla de trigger (op $11), 50 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0513,0x0002,0x0612,0x0003,0x0711,0x0007,0x0a0e,0x0009 | $091F8C
        .dc.w   0x090f,0x000c,0x0a0e,0x0011,0x0c0c,0x0013,0x1800,0x0016 | $091F9C
        .dc.w   0x0c0c,0x0018,0x0d0b,0x001a,0x090c,0x001c,0x0000,0xffff | $091FAC
        .dc.w   0x0000                                 | $091FBC

        .globl  SceneTrig_091FBE
        .section .text.SceneTrig_091FBE, "ax", @progbits
SceneTrig_091FBE:                           | tabla de trigger (op $11), 10 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0c00,0x0010,0x0000,0xffff,0x0000     | $091FBE

        .globl  SceneTrig_091FC8
        .section .text.SceneTrig_091FC8, "ax", @progbits
SceneTrig_091FC8:                           | tabla de trigger (op $11), 6 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0000,0xffff,0x0000                   | $091FC8

        .globl  SceneTrig_091FCE
        .section .text.SceneTrig_091FCE, "ax", @progbits
SceneTrig_091FCE:                           | tabla de trigger (op $11), 30 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0504,0x0002,0x0702,0x0003,0x0c01,0x0006,0x0c01,0x0008 | $091FCE
        .dc.w   0x0904,0x000a,0x0a03,0x000b,0x1200,0xffff,0x0000 | $091FDE

        .globl  SceneTrig_091FEC
        .section .text.SceneTrig_091FEC, "ax", @progbits
SceneTrig_091FEC:                           | tabla de trigger (op $11), 10 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0700,0x0010,0x0000,0xffff,0x0000     | $091FEC

        .globl  SceneTrig_091FF6
        .section .text.SceneTrig_091FF6, "ax", @progbits
SceneTrig_091FF6:                           | tabla de trigger (op $11), 4 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0000,0xffff                          | $091FF6

        .globl  SceneEntities_091FFA
        .section .text.SceneEntities_091FFA, "ax", @progbits
SceneEntities_091FFA:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0000,0x0010,0x80e0,0xffa0,0x0000,0x0020,0x0010
                | $091FFA type=0 subop=00 tmpl=$1080E0 payload=ffa0000000200010
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0005
                | $092008 type=1 subop=00 tmpl=$108064 payload=0000000000150005
        .dc.w   0x0100,0x0010,0x7fe8,0x0000,0xfe80,0x0018,0x0028
                | $092016 type=1 subop=00 tmpl=$107FE8 payload=0000fe8000180028
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0x0000,0x0020,0x0010
                | $092024 type=1 subop=00 tmpl=$106F6C payload=ffa0000000200010
        .dc.w   0x0250                        | $092032 terminador type=2

        .globl  SceneScript_092034
        .section .text.SceneScript_092034, "ax", @progbits
SceneScript_092034:                         | bytecode VM de escena (1146 B, 60 ops)
        .dc.w   0x0003,0x0000,0x0140
                | $092034 op $03 warp: camara=(0,320) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $09203A op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0003                 |   slot $10 <- bank $0003
        .dc.w   0x0011,0x0004                 |   slot $11 <- bank $0004
        .dc.w   0x0012,0x0005                 |   slot $12 <- bank $0005
        .dc.w   0x0013,0x0006                 |   slot $13 <- bank $0006
        .dc.w   0x0014,0x0007                 |   slot $14 <- bank $0007
        .dc.w   0x0015,0x0008                 |   slot $15 <- bank $0008
        .dc.w   0x0016,0x0009                 |   slot $16 <- bank $0009
        .dc.w   0x0017,0x000a                 |   slot $17 <- bank $000A
        .dc.w   0x0018,0x000b                 |   slot $18 <- bank $000B
        .dc.w   0x0019,0x000c                 |   slot $19 <- bank $000C
        .dc.w   0x001a,0x000d                 |   slot $1A <- bank $000D
        .dc.w   0x001b,0x000e                 |   slot $1B <- bank $000E
        .dc.w   0x001c,0x000f                 |   slot $1C <- bank $000F
        .dc.w   0x001d,0x0010                 |   slot $1D <- bank $0010
        .dc.w   0x001e,0x0011                 |   slot $1E <- bank $0011
        .dc.w   0x001f,0x0012                 |   slot $1F <- bank $0012
        .dc.w   0x0020,0x0014                 |   slot $20 <- bank $0014
        .dc.w   0x0021,0x0013                 |   slot $21 <- bank $0013
        .dc.w   0x0022,0x0015                 |   slot $22 <- bank $0015
        .dc.w   0x0023,0x0016                 |   slot $23 <- bank $0016
        .dc.w   0x0024,0x0017                 |   slot $24 <- bank $0017
        .dc.w   0x0025,0x0018                 |   slot $25 <- bank $0018
        .dc.w   0x0026,0x0019                 |   slot $26 <- bank $0019
        .dc.w   0x0027,0x001a                 |   slot $27 <- bank $001A
        .dc.w   0x0028,0x001b                 |   slot $28 <- bank $001B
        .dc.w   0x0029,0x001c                 |   slot $29 <- bank $001C
        .dc.w   0x002a,0x001d                 |   slot $2A <- bank $001D
        .dc.w   0x002b,0x001e                 |   slot $2B <- bank $001E
        .dc.w   0x002c,0x001f                 |   slot $2C <- bank $001F
        .dc.w   0x002d,0x0020                 |   slot $2D <- bank $0020
        .dc.w   0x002e,0x0021                 |   slot $2E <- bank $0021
        .dc.w   0x002f,0x0022                 |   slot $2F <- bank $0022
        .dc.w   0x0030,0x0023                 |   slot $30 <- bank $0023
        .dc.w   0x0031,0x0024                 |   slot $31 <- bank $0024
        .dc.w   0x0032,0x0025                 |   slot $32 <- bank $0025
        .dc.w   0x0033,0x0026                 |   slot $33 <- bank $0026
        .dc.w   0x0034,0x0027                 |   slot $34 <- bank $0027
        .dc.w   0x0035,0x0028                 |   slot $35 <- bank $0028
        .dc.w   0x0036,0x0029                 |   slot $36 <- bank $0029
        .dc.w   0x0037,0x002a                 |   slot $37 <- bank $002A
        .dc.w   0x0038,0x002b                 |   slot $38 <- bank $002B
        .dc.w   0x0039,0x002c                 |   slot $39 <- bank $002C
        .dc.w   0x003a,0x002d                 |   slot $3A <- bank $002D
        .dc.w   0x003b,0x002e                 |   slot $3B <- bank $002E
        .dc.w   0x003c,0x002f                 |   slot $3C <- bank $002F
        .dc.w   0x003d,0x0030                 |   slot $3D <- bank $0030
        .dc.w   0x003e,0x0031                 |   slot $3E <- bank $0031
        .dc.w   0x003f,0x0032                 |   slot $3F <- bank $0032
        .dc.w   0x0040,0x0033                 |   slot $40 <- bank $0033
        .dc.w   0x0041,0x0034                 |   slot $41 <- bank $0034
        .dc.w   0x0042,0x0035                 |   slot $42 <- bank $0035
        .dc.w   0x0043,0x0036                 |   slot $43 <- bank $0036
        .dc.w   0x0044,0x0037                 |   slot $44 <- bank $0037
        .dc.w   0x0045,0x0038                 |   slot $45 <- bank $0038
        .dc.w   0x0046,0x0039                 |   slot $46 <- bank $0039
        .dc.w   0x0047,0x003a                 |   slot $47 <- bank $003A
        .dc.w   0x0048,0x003b                 |   slot $48 <- bank $003B
        .dc.w   0x0049,0x003c                 |   slot $49 <- bank $003C
        .dc.w   0x004a,0x003d                 |   slot $4A <- bank $003D
        .dc.w   0x004b,0x003e                 |   slot $4B <- bank $003E
        .dc.w   0x004c,0x003f                 |   slot $4C <- bank $003F
        .dc.w   0x004d,0x00ba                 |   slot $4D <- bank $00BA
        .dc.w   0x004e,0x00bb                 |   slot $4E <- bank $00BB
        .dc.w   0x004f,0x00bc                 |   slot $4F <- bank $00BC
        .dc.w   0x0050,0x01e0                 |   slot $50 <- bank $01E0
        .dc.w   0x0051,0x01e1                 |   slot $51 <- bank $01E1
        .dc.w   0x0052,0x01e2                 |   slot $52 <- bank $01E2
        .dc.w   0x0053,0x01e3                 |   slot $53 <- bank $01E3
        .dc.w   0x0054,0x01e4                 |   slot $54 <- bank $01E4
        .dc.w   0x0055,0x01e5                 |   slot $55 <- bank $01E5
        .dc.w   0x0056,0x01e6                 |   slot $56 <- bank $01E6
        .dc.w   0x0057,0x01e7                 |   slot $57 <- bank $01E7
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0002                 |   slot $FF <- bank $0002
        .dc.w   0xffff                              | $0921D4 fin de pares
        .dc.w   0x000c,0x0800
                | $0921D6 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x000f,0xffff,0x0140,0xffff,0x0140,0x0000
                | $0921DA op $0F set_limits: minX=-1 minY=320 maxX=-1 maxY=320 slope=0
        .dc.w   0x0010,0x0080,0x00a0
                | $0921E6 op $10 set_6466: $108164=0080 $108166=00A0
        .dc.w   0x0000,0x0010,0x6f6c,0x00c8,0x0010,0x000a,0xbe50,0x0020,0x4b00,0x0000,0x0140
                | $0921EC op $00 bind_path: ent=$106F6C ruta=$C80010.. cont=$204B00 ancla=(0,320)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x2498
                | $092202 op $11 set_trig: ent=$106F6C tabla=$092498  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $09220C op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $092218 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x7fe8,0x0060,0x000d,0x000b,0x1350,0x0000,0x0000,0x0000,0x0140
                | $09221E op $00 bind_path: ent=$107FE8 ruta=$60000D.. cont=$000000 ancla=(0,320)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x24f0
                | $092234 op $11 set_trig: ent=$107FE8 tabla=$0924F0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0080,0x0100
                | $09223E op $01 set_fields: ent=$107FE8 +72=03 +74=$0080 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $09224A op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092250 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092252 --- callback 68000 embebido (16 B) -> PC=$092262 ---
        cmpi.w  #0x2b0, 0x106f50.l             | $092252  cmpi.w #$2b0, $106f50.l
        scs     d0                             | $09225A  scs.b d0
        lea     .L092262(pc), a1              | $09225C  a1 = nuevo PC
        rts                                    | $092260  rts
.L092262:
        .dc.w   0x000d,0x0005,0x2756,0x0021,0x0014,0xffff,0x0000
                | $092262 op $0D call_args: fn=$052756 args=00210014ffff0000
        .dc.w   0x0000,0x0010,0x8064,0x001c,0x0005,0x000b,0x34d0,0x0000,0x0000,0x03f0,0x0140
                | $092270 op $00 bind_path: ent=$108064 ruta=$1C0005.. cont=$000000 ancla=(1008,320)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0040,0x0100
                | $092286 op $01 set_fields: ent=$108064 +72=03 +74=$0040 +76=$0100
        .dc.w   0x0004,0x0010,0x8064,0x0500
                | $092292 op $04 spawn: tmpl=$108064 a=05 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x8064
                | $09229A op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0922A0 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0922A2 --- callback 68000 embebido (16 B) -> PC=$0922B2 ---
        cmpi.w  #0x840, 0x106f50.l             | $0922A2  cmpi.w #$840, $106f50.l
        scs     d0                             | $0922AA  scs.b d0
        lea     .L0922b2(pc), a1              | $0922AC  a1 = nuevo PC
        rts                                    | $0922B0  rts
.L0922b2:
        .dc.w   0x000f,0x0f00,0x0140,0x0f00,0x0140,0x0000
                | $0922B2 op $0F set_limits: minX=3840 minY=320 maxX=3840 maxY=320 slope=0
        .dc.w   0x0006
                | $0922BE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0922C0 --- callback 68000 embebido (16 B) -> PC=$0922D0 ---
        cmpi.w  #0x9f0, 0x106f50.l             | $0922C0  cmpi.w #$9f0, $106f50.l
        scs     d0                             | $0922C8  scs.b d0
        lea     .L0922d0(pc), a1              | $0922CA  a1 = nuevo PC
        rts                                    | $0922CE  rts
.L0922d0:
        .dc.w   0x0005,0x0010,0x8064
                | $0922D0 op $05 detach: ent=$108064  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0000,0x0010,0x7fe8,0x0018,0x0018,0x000b,0x26d0,0x0000,0x0000,0x09f0,0x00c0
                | $0922D6 op $00 bind_path: ent=$107FE8 ruta=$180018.. cont=$000000 ancla=(2544,192)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0080,0x0080
                | $0922EC op $01 set_fields: ent=$107FE8 +72=03 +74=$0080 +76=$0080
        .dc.w   0x0004,0x0010,0x7fe8,0x1800
                | $0922F8 op $04 spawn: tmpl=$107FE8 a=18 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $092300 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x7fe8,0x0018,0x0018,0x000b,0x26d0,0x0000,0x0000,0x0b70,0x00c0
                | $092306 op $00 bind_path: ent=$107FE8 ruta=$180018.. cont=$000000 ancla=(2928,192)
        .dc.w   0x0007,0x0010,0x7fe8
                | $09231C op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0008,0x0010,0x8064
                | $092322 op $08 call_ed6: ent=$108064  ($51ED6)
        .dc.w   0x0006
                | $092328 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09232A --- callback 68000 embebido (16 B) -> PC=$09233A ---
        cmpi.w  #0xa10, 0x106f50.l             | $09232A  cmpi.w #$a10, $106f50.l
        scs     d0                             | $092332  scs.b d0
        lea     .L09233a(pc), a1              | $092334  a1 = nuevo PC
        rts                                    | $092338  rts
.L09233a:
        .dc.w   0x000f,0x09e0,0x0140,0x0a10,0x0140,0x0000
                | $09233A op $0F set_limits: minX=2528 minY=320 maxX=2576 maxY=320 slope=0
        .dc.w   0x0006
                | $092346 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092348 --- callback 68000 embebido (16 B) -> PC=$092358 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092348  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092350  sne.b d0
        lea     .L092358(pc), a1              | $092352  a1 = nuevo PC
        rts                                    | $092356  rts
.L092358:
        .dc.w   0x000f,0x0ae0,0x0140,0x0ae0,0x0140,0x0000
                | $092358 op $0F set_limits: minX=2784 minY=320 maxX=2784 maxY=320 slope=0
        .dc.w   0x0006
                | $092364 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092366 --- callback 68000 embebido (16 B) -> PC=$092376 ---
        cmpi.w  #0xae0, 0x106f50.l             | $092366  cmpi.w #$ae0, $106f50.l
        scs     d0                             | $09236E  scs.b d0
        lea     .L092376(pc), a1              | $092370  a1 = nuevo PC
        rts                                    | $092374  rts
.L092376:
        .dc.w   0x0000,0x0010,0x6f6c,0x0040,0x0023,0x000a,0xf050,0x0020,0x7d00,0x0c80,0x0020
                | $092376 op $00 bind_path: ent=$106F6C ruta=$400023.. cont=$207D00 ancla=(3200,32)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x24ce
                | $09238C op $11 set_trig: ent=$106F6C tabla=$0924CE  (-> $12(ent))
        .dc.w   0x0007,0x0010,0x6f6c
                | $092396 op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $09239C op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x000f,0x0b20,0x0020,0x0b20,0x0140,0x0100
                | $0923A8 op $0F set_limits: minX=2848 minY=32 maxX=2848 maxY=320 slope=256
        .dc.w   0x0010,0x0078,0x00a0
                | $0923B4 op $10 set_6466: $108164=0078 $108166=00A0
        .dc.w   0x0006
                | $0923BA op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0923BC --- callback 68000 embebido (16 B) -> PC=$0923CC ---
        cmpi.w  #0xb20, 0x106f50.l             | $0923BC  cmpi.w #$b20, $106f50.l
        scs     d0                             | $0923C4  scs.b d0
        lea     .L0923cc(pc), a1              | $0923C6  a1 = nuevo PC
        rts                                    | $0923CA  rts
.L0923cc:
        .dc.w   0x000f,0x0ec0,0x00e0,0x0ec0,0x0140,0x0000
                | $0923CC op $0F set_limits: minX=3776 minY=224 maxX=3776 maxY=320 slope=0
        .dc.w   0x0006
                | $0923D8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0923DA --- callback 68000 embebido (16 B) -> PC=$0923EA ---
        cmpi.w  #0xc00, 0x106f50.l             | $0923DA  cmpi.w #$c00, $106f50.l
        scs     d0                             | $0923E2  scs.b d0
        lea     .L0923ea(pc), a1              | $0923E4  a1 = nuevo PC
        rts                                    | $0923E8  rts
.L0923ea:
        .dc.w   0x000f,0x0ec0,0x0020,0x0ec0,0x0140,0x0040
                | $0923EA op $0F set_limits: minX=3776 minY=32 maxX=3776 maxY=320 slope=64
        .dc.w   0x0006
                | $0923F6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0923F8 --- callback 68000 embebido (16 B) -> PC=$092408 ---
        cmpi.w  #0xdc0, 0x106f50.l             | $0923F8  cmpi.w #$dc0, $106f50.l
        scs     d0                             | $092400  scs.b d0
        lea     .L092408(pc), a1              | $092402  a1 = nuevo PC
        rts                                    | $092406  rts
.L092408:
        .dc.w   0x000f,0x0f00,0x0020,0x0f00,0x0020,0x0000
                | $092408 op $0F set_limits: minX=3840 minY=32 maxX=3840 maxY=32 slope=0
        .dc.w   0x0010,0x0078,0x0120
                | $092414 op $10 set_6466: $108164=0078 $108166=0120
        .dc.w   0x0013,0x10bf
                | $09241A op $13 clamp: d0=10BF  (ClampD0ToRange)
        .dc.w   0x0006
                | $09241E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092420 --- callback 68000 embebido (16 B) -> PC=$092430 ---
        cmpi.w  #0xdf0, 0x106f50.l             | $092420  cmpi.w #$df0, $106f50.l
        scs     d0                             | $092428  scs.b d0
        lea     .L092430(pc), a1              | $09242A  a1 = nuevo PC
        rts                                    | $09242E  rts
.L092430:
        .dc.w   0x000f,0xffff,0x0020,0xffff,0x0020,0x0000
                | $092430 op $0F set_limits: minX=-1 minY=32 maxX=-1 maxY=32 slope=0
        .dc.w   0x0006
                | $09243C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09243E --- callback 68000 embebido (16 B) -> PC=$09244E ---
        cmpi.w  #0xe10, 0x106f50.l             | $09243E  cmpi.w #$e10, $106f50.l
        scs     d0                             | $092446  scs.b d0
        lea     .L09244e(pc), a1              | $092448  a1 = nuevo PC
        rts                                    | $09244C  rts
.L09244e:
        .dc.w   0x0000,0x0010,0x80e0,0x0020,0x0004,0x000b,0x3700,0x0000,0x0000,0x0fb0,0x0100
                | $09244E op $00 bind_path: ent=$1080E0 ruta=$200004.. cont=$000000 ancla=(4016,256)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0100,0x0100
                | $092464 op $01 set_fields: ent=$1080E0 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x80e0,0x0400
                | $092470 op $04 spawn: tmpl=$1080E0 a=04 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $092478 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $09247E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092480 --- callback 68000 embebido (16 B) -> PC=$092490 ---
        cmpi.w  #0xea0, 0x106f50.l             | $092480  cmpi.w #$ea0, $106f50.l
        scs     d0                             | $092488  scs.b d0
        lea     .L092490(pc), a1              | $09248A  a1 = nuevo PC
        rts                                    | $09248E  rts
.L092490:
        .dc.w   0x0005,0x0010,0x7fe8
                | $092490 op $05 detach: ent=$107FE8  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0002
                | $092496 op $02 END_FRAME: cede el frame (integra scroll+camara)
        .dc.w   0x0000,0x1000,0x0048,0x0c04,0x0049,0x0b05,0x004a,0x0808,0x0097,0x0e02,0x0099
                | $092498 op $00 bind_path: ent=$10000048 ruta=$C040049.. cont=$8080097 ancla=(3586,153)

        .globl  SceneTrig_0924AE
        .section .text.SceneTrig_0924AE, "ax", @progbits
SceneTrig_0924AE:                           | tabla de trigger (op $11), 34 B  (sin referencia directa op $11 en este script)
        .dc.w   0x1000,0x00c0,0x0e02,0x00c1,0x0d03,0x00c2,0x0e02,0x009b | $0924AE
        .dc.w   0x0f01,0x009d,0x0e02,0x0094,0x0d03,0x0097,0x0c04,0xffff | $0924BE
        .dc.w   0x0000                                 | $0924CE

        .globl  SceneTrig_0924D0
        .section .text.SceneTrig_0924D0, "ax", @progbits
SceneTrig_0924D0:                           | tabla de trigger (op $11), 34 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0e14,0x0003,0x0b14,0x0008,0x0c12,0x0010,0x0d10,0x0017 | $0924D0
        .dc.w   0x1d00,0x001a,0x1a00,0x001c,0x1900,0x0020,0x1500,0xffff | $0924E0
        .dc.w   0x0000                                 | $0924F0

        .globl  SceneTrig_0924F2
        .section .text.SceneTrig_0924F2, "ax", @progbits
SceneTrig_0924F2:                           | tabla de trigger (op $11), 30 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0c00,0x0022,0x0d00,0x0026,0x030a,0x002c,0x0d00,0x0040 | $0924F2
        .dc.w   0x030a,0x0048,0x0c01,0x004d,0x030a,0xffff,0x0000 | $092502

        .globl  SceneTrig_092510
        .section .text.SceneTrig_092510, "ax", @progbits
SceneTrig_092510:                           | tabla de trigger (op $11), 36 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0100,0x0001,0x0200,0x0003,0x0300,0x0005,0x0400,0x0007 | $092510
        .dc.w   0x0500,0x0015,0x0400,0x0016,0x0300,0x0018,0x0200,0x001b | $092520
        .dc.w   0x0100,0xffff                          | $092530

        .globl  SceneEntities_092534
        .section .text.SceneEntities_092534, "ax", @progbits
SceneEntities_092534:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0000,0x0000
                | $092534 type=1 subop=00 tmpl=$1080E0 payload=0000000000000000
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $092542 type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0000,0x0000
                | $092550 type=0 subop=00 tmpl=$107FE8 payload=0000000000000000
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0xff80,0x0020,0x001b
                | $09255E type=1 subop=00 tmpl=$106F6C payload=ffa0ff800020001b
        .dc.w   0x0250                        | $09256C terminador type=2

        .globl  SceneScript_09256E
        .section .text.SceneScript_09256E, "ax", @progbits
SceneScript_09256E:                         | bytecode VM de escena (1390 B, 71 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $09256E op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $092574 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0040                 |   slot $10 <- bank $0040
        .dc.w   0x0011,0x0041                 |   slot $11 <- bank $0041
        .dc.w   0x0012,0x0042                 |   slot $12 <- bank $0042
        .dc.w   0x0013,0x0043                 |   slot $13 <- bank $0043
        .dc.w   0x0014,0x0044                 |   slot $14 <- bank $0044
        .dc.w   0x0015,0x0045                 |   slot $15 <- bank $0045
        .dc.w   0x0016,0x0046                 |   slot $16 <- bank $0046
        .dc.w   0x0017,0x0047                 |   slot $17 <- bank $0047
        .dc.w   0x0018,0x0048                 |   slot $18 <- bank $0048
        .dc.w   0x0019,0x0049                 |   slot $19 <- bank $0049
        .dc.w   0x001a,0x004a                 |   slot $1A <- bank $004A
        .dc.w   0x001b,0x004b                 |   slot $1B <- bank $004B
        .dc.w   0x001c,0x004c                 |   slot $1C <- bank $004C
        .dc.w   0x001d,0x004d                 |   slot $1D <- bank $004D
        .dc.w   0x001e,0x004e                 |   slot $1E <- bank $004E
        .dc.w   0x001f,0x004f                 |   slot $1F <- bank $004F
        .dc.w   0x0020,0x0050                 |   slot $20 <- bank $0050
        .dc.w   0x0021,0x0051                 |   slot $21 <- bank $0051
        .dc.w   0x0022,0x0052                 |   slot $22 <- bank $0052
        .dc.w   0x0023,0x0053                 |   slot $23 <- bank $0053
        .dc.w   0x0024,0x0054                 |   slot $24 <- bank $0054
        .dc.w   0x0025,0x0055                 |   slot $25 <- bank $0055
        .dc.w   0x0026,0x0056                 |   slot $26 <- bank $0056
        .dc.w   0x0027,0x0057                 |   slot $27 <- bank $0057
        .dc.w   0x0028,0x0058                 |   slot $28 <- bank $0058
        .dc.w   0x0029,0x0059                 |   slot $29 <- bank $0059
        .dc.w   0x002a,0x005a                 |   slot $2A <- bank $005A
        .dc.w   0x002b,0x005b                 |   slot $2B <- bank $005B
        .dc.w   0x002c,0x005c                 |   slot $2C <- bank $005C
        .dc.w   0x002d,0x005d                 |   slot $2D <- bank $005D
        .dc.w   0x002e,0x005e                 |   slot $2E <- bank $005E
        .dc.w   0x002f,0x005f                 |   slot $2F <- bank $005F
        .dc.w   0x0030,0x0060                 |   slot $30 <- bank $0060
        .dc.w   0x0031,0x0061                 |   slot $31 <- bank $0061
        .dc.w   0x0032,0x0062                 |   slot $32 <- bank $0062
        .dc.w   0x0033,0x0063                 |   slot $33 <- bank $0063
        .dc.w   0x0034,0x0064                 |   slot $34 <- bank $0064
        .dc.w   0x0035,0x0065                 |   slot $35 <- bank $0065
        .dc.w   0x0036,0x0066                 |   slot $36 <- bank $0066
        .dc.w   0x0037,0x0067                 |   slot $37 <- bank $0067
        .dc.w   0x0038,0x0068                 |   slot $38 <- bank $0068
        .dc.w   0x0039,0x0069                 |   slot $39 <- bank $0069
        .dc.w   0x003a,0x006a                 |   slot $3A <- bank $006A
        .dc.w   0x003b,0x006b                 |   slot $3B <- bank $006B
        .dc.w   0x003c,0x006c                 |   slot $3C <- bank $006C
        .dc.w   0x003d,0x006d                 |   slot $3D <- bank $006D
        .dc.w   0x003e,0x006e                 |   slot $3E <- bank $006E
        .dc.w   0x003f,0x006f                 |   slot $3F <- bank $006F
        .dc.w   0x0040,0x0070                 |   slot $40 <- bank $0070
        .dc.w   0x0041,0x0071                 |   slot $41 <- bank $0071
        .dc.w   0x0042,0x0072                 |   slot $42 <- bank $0072
        .dc.w   0x0043,0x0073                 |   slot $43 <- bank $0073
        .dc.w   0x0044,0x0074                 |   slot $44 <- bank $0074
        .dc.w   0x0045,0x0075                 |   slot $45 <- bank $0075
        .dc.w   0x0046,0x0076                 |   slot $46 <- bank $0076
        .dc.w   0x0047,0x0077                 |   slot $47 <- bank $0077
        .dc.w   0x0048,0x0078                 |   slot $48 <- bank $0078
        .dc.w   0x0049,0x0079                 |   slot $49 <- bank $0079
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x007a                 |   slot $FF <- bank $007A
        .dc.w   0xffff                              | $09270E fin de pares
        .dc.w   0x000f,0x0040,0x0008,0x0040,0x0020,0x0000
                | $092710 op $0F set_limits: minX=64 minY=8 maxX=64 maxY=32 slope=0
        .dc.w   0x0010,0x0078,0x00b0
                | $09271C op $10 set_6466: $108164=0078 $108166=00B0
        .dc.w   0x0000,0x0010,0x6f6c,0x0053,0x0012,0x000b,0x53ec,0x0020,0xb400,0x0000,0x0000
                | $092722 op $00 bind_path: ent=$106F6C ruta=$530012.. cont=$20B400 ancla=(0,0)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x2ac6
                | $092738 op $11 set_trig: ent=$106F6C tabla=$092AC6  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $092742 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $09274E op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x001e,0x000a,0x000b,0x9ab8,0x0000,0x0000,0x0000,0x0000
                | $092754 op $00 bind_path: ent=$108064 ruta=$1E000A.. cont=$000000 ancla=(0,0)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0080
                | $09276A op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0080
        .dc.w   0x0004,0x0010,0x8064,0x0a00
                | $092776 op $04 spawn: tmpl=$108064 a=0A b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x8064
                | $09277E op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092784 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092786 --- callback 68000 embebido (16 B) -> PC=$092796 ---
        cmpi.w  #0x1, 0x106f50.l               | $092786  cmpi.w #$1, $106f50.l
        scs     d0                             | $09278E  scs.b d0
        lea     .L092796(pc), a1              | $092790  a1 = nuevo PC
        rts                                    | $092794  rts
.L092796:
        .dc.w   0x0006
                | $092796 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092798 --- callback 68000 embebido (16 B) -> PC=$0927A8 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092798  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0927A0  sne.b d0
        lea     .L0927a8(pc), a1              | $0927A2  a1 = nuevo PC
        rts                                    | $0927A6  rts
.L0927a8:
        .dc.w   0x000f,0x0110,0x0008,0x0110,0x0020,0x0000
                | $0927A8 op $0F set_limits: minX=272 minY=8 maxX=272 maxY=32 slope=0
        .dc.w   0x0006
                | $0927B4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0927B6 --- callback 68000 embebido (16 B) -> PC=$0927C6 ---
        cmpi.w  #0x41, 0x106f50.l              | $0927B6  cmpi.w #$41, $106f50.l
        scs     d0                             | $0927BE  scs.b d0
        lea     .L0927c6(pc), a1              | $0927C0  a1 = nuevo PC
        rts                                    | $0927C4  rts
.L0927c6:
        .dc.w   0x0006
                | $0927C6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0927C8 --- callback 68000 embebido (16 B) -> PC=$0927D8 ---
        cmpi.b  #0x1, 0x10e39a.l               | $0927C8  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0927D0  sne.b d0
        lea     .L0927d8(pc), a1              | $0927D2  a1 = nuevo PC
        rts                                    | $0927D6  rts
.L0927d8:
        .dc.w   0x000f,0x0140,0x0008,0x0140,0x0020,0x0000
                | $0927D8 op $0F set_limits: minX=320 minY=8 maxX=320 maxY=32 slope=0
        .dc.w   0x0006
                | $0927E4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0927E6 --- callback 68000 embebido (16 B) -> PC=$0927F6 ---
        cmpi.w  #0x111, 0x106f50.l             | $0927E6  cmpi.w #$111, $106f50.l
        scs     d0                             | $0927EE  scs.b d0
        lea     .L0927f6(pc), a1              | $0927F0  a1 = nuevo PC
        rts                                    | $0927F4  rts
.L0927f6:
        .dc.w   0x0006
                | $0927F6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0927F8 --- callback 68000 embebido (16 B) -> PC=$092808 ---
        cmpi.b  #0x1, 0x10e39a.l               | $0927F8  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092800  sne.b d0
        lea     .L092808(pc), a1              | $092802  a1 = nuevo PC
        rts                                    | $092806  rts
.L092808:
        .dc.w   0x000f,0x0360,0x0008,0x03c0,0x0020,0x0000
                | $092808 op $0F set_limits: minX=864 minY=8 maxX=960 maxY=32 slope=0
        .dc.w   0x0010,0x0088,0x00b0
                | $092814 op $10 set_6466: $108164=0088 $108166=00B0
        .dc.w   0x0006
                | $09281A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09281C --- callback 68000 embebido (16 B) -> PC=$09282C ---
        cmpi.w  #0x300, 0x106f50.l             | $09281C  cmpi.w #$300, $106f50.l
        scs     d0                             | $092824  scs.b d0
        lea     .L09282c(pc), a1              | $092826  a1 = nuevo PC
        rts                                    | $09282A  rts
.L09282c:
        .dc.w   0x0000,0x0010,0x8064,0x005c,0x000c,0x000b,0x9f68,0x0000,0x0000,0x0300,0xfff0
                | $09282C op $00 bind_path: ent=$108064 ruta=$5C000C.. cont=$000000 ancla=(768,-16)
        .dc.w   0x0004,0x0010,0x8064,0x0c00
                | $092842 op $04 spawn: tmpl=$108064 a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0080
                | $09284A op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0080
        .dc.w   0x0009,0x0010,0x8064
                | $092856 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $09285C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09285E --- callback 68000 embebido (16 B) -> PC=$09286E ---
        cmpi.w  #0x381, 0x106f50.l             | $09285E  cmpi.w #$381, $106f50.l
        scs     d0                             | $092866  scs.b d0
        lea     .L09286e(pc), a1              | $092868  a1 = nuevo PC
        rts                                    | $09286C  rts
.L09286e:
        .dc.w   0x0006
                | $09286E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092870 --- callback 68000 embebido (16 B) -> PC=$092880 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092870  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092878  sne.b d0
        lea     .L092880(pc), a1              | $09287A  a1 = nuevo PC
        rts                                    | $09287E  rts
.L092880:
        .dc.w   0x000f,0x0a40,0x0008,0x0a50,0x0020,0x0000
                | $092880 op $0F set_limits: minX=2624 minY=8 maxX=2640 maxY=32 slope=0
        .dc.w   0x0010,0x0088,0x00bc
                | $09288C op $10 set_6466: $108164=0088 $108166=00BC
        .dc.w   0x0006
                | $092892 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092894 --- callback 68000 embebido (16 B) -> PC=$0928A4 ---
        cmpi.w  #0x390, 0x106f50.l             | $092894  cmpi.w #$390, $106f50.l
        scs     d0                             | $09289C  scs.b d0
        lea     .L0928a4(pc), a1              | $09289E  a1 = nuevo PC
        rts                                    | $0928A2  rts
.L0928a4:
        .dc.w   0x0000,0x0010,0x6f6c,0x004d,0x0011,0x000b,0x6b44,0x0020,0xcb58,0x0530,0x0010
                | $0928A4 op $00 bind_path: ent=$106F6C ruta=$4D0011.. cont=$20CB58 ancla=(1328,16)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x2af0
                | $0928BA op $11 set_trig: ent=$106F6C tabla=$092AF0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0928C4 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $0928D0 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0928D6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0928D8 --- callback 68000 embebido (16 B) -> PC=$0928E8 ---
        cmpi.w  #0x700, 0x106f50.l             | $0928D8  cmpi.w #$700, $106f50.l
        scs     d0                             | $0928E0  scs.b d0
        lea     .L0928e8(pc), a1              | $0928E2  a1 = nuevo PC
        rts                                    | $0928E6  rts
.L0928e8:
        .dc.w   0x000d,0x0005,0x2756,0x0040,0x0072,0x000a,0x0000
                | $0928E8 op $0D call_args: fn=$052756 args=00400072000a0000
        .dc.w   0x000d,0x0005,0x2756,0x0041,0x0073,0x000a,0x0000
                | $0928F6 op $0D call_args: fn=$052756 args=00410073000a0000
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x007b,0x000a,0x0000
                | $092904 op $0D call_args: fn=$052756 args=00ff007b000a0000
        .dc.w   0x0006
                | $092912 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092914 --- callback 68000 embebido (16 B) -> PC=$092924 ---
        cmpi.w  #0x860, 0x106f50.l             | $092914  cmpi.w #$860, $106f50.l
        scs     d0                             | $09291C  scs.b d0
        lea     .L092924(pc), a1              | $09291E  a1 = nuevo PC
        rts                                    | $092922  rts
.L092924:
        .dc.w   0x0000,0x0010,0x6f6c,0x0060,0x0012,0x000b,0x7fb8,0x0020,0xdfcc,0x0a00,0x0000
                | $092924 op $00 bind_path: ent=$106F6C ruta=$600012.. cont=$20DFCC ancla=(2560,0)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x2b0e
                | $09293A op $11 set_trig: ent=$106F6C tabla=$092B0E  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $092944 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $092950 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092956 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092958 --- callback 68000 embebido (16 B) -> PC=$092968 ---
        cmpi.w  #0x980, 0x106f50.l             | $092958  cmpi.w #$980, $106f50.l
        scs     d0                             | $092960  scs.b d0
        lea     .L092968(pc), a1              | $092962  a1 = nuevo PC
        rts                                    | $092966  rts
.L092968:
        .dc.w   0x000d,0x0005,0x2756,0x0040,0x0074,0x000a,0x0000
                | $092968 op $0D call_args: fn=$052756 args=00400074000a0000
        .dc.w   0x000d,0x0005,0x2756,0x0041,0x0075,0x000a,0x0000
                | $092976 op $0D call_args: fn=$052756 args=00410075000a0000
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x007c,0x000a,0x0000
                | $092984 op $0D call_args: fn=$052756 args=00ff007c000a0000
        .dc.w   0x0006
                | $092992 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092994 --- callback 68000 embebido (16 B) -> PC=$0929A4 ---
        cmpi.w  #0xa50, 0x106f50.l             | $092994  cmpi.w #$a50, $106f50.l
        scs     d0                             | $09299C  scs.b d0
        lea     .L0929a4(pc), a1              | $09299E  a1 = nuevo PC
        rts                                    | $0929A2  rts
.L0929a4:
        .dc.w   0x0006
                | $0929A4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0929A6 --- callback 68000 embebido (16 B) -> PC=$0929B6 ---
        cmpi.b  #0x1, 0x10e39a.l               | $0929A6  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0929AE  sne.b d0
        lea     .L0929b6(pc), a1              | $0929B0  a1 = nuevo PC
        rts                                    | $0929B4  rts
.L0929b6:
        .dc.w   0x000f,0x0b00,0x0008,0x0b20,0x0020,0x0000
                | $0929B6 op $0F set_limits: minX=2816 minY=8 maxX=2848 maxY=32 slope=0
        .dc.w   0x0010,0x0078,0x00c8
                | $0929C2 op $10 set_6466: $108164=0078 $108166=00C8
        .dc.w   0x0006
                | $0929C8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0929CA --- callback 68000 embebido (16 B) -> PC=$0929DA ---
        cmpi.w  #0xa80, 0x106f50.l             | $0929CA  cmpi.w #$a80, $106f50.l
        scs     d0                             | $0929D2  scs.b d0
        lea     .L0929da(pc), a1              | $0929D4  a1 = nuevo PC
        rts                                    | $0929D8  rts
.L0929da:
        .dc.w   0x000d,0x0005,0x2756,0x0040,0x0076,0x000a,0x0000
                | $0929DA op $0D call_args: fn=$052756 args=00400076000a0000
        .dc.w   0x000d,0x0005,0x2756,0x0041,0x0077,0x000a,0x0000
                | $0929E8 op $0D call_args: fn=$052756 args=00410077000a0000
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x007d,0x000a,0x0000
                | $0929F6 op $0D call_args: fn=$052756 args=00ff007d000a0000
        .dc.w   0x0006
                | $092A04 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A06 --- callback 68000 embebido (16 B) -> PC=$092A16 ---
        cmpi.w  #0xb10, 0x106f50.l             | $092A06  cmpi.w #$b10, $106f50.l
        scs     d0                             | $092A0E  scs.b d0
        lea     .L092a16(pc), a1              | $092A10  a1 = nuevo PC
        rts                                    | $092A14  rts
.L092a16:
        .dc.w   0x0006
                | $092A16 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A18 --- callback 68000 embebido (16 B) -> PC=$092A28 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092A18  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092A20  sne.b d0
        lea     .L092a28(pc), a1              | $092A22  a1 = nuevo PC
        rts                                    | $092A26  rts
.L092a28:
        .dc.w   0x000f,0x0b60,0x0008,0x0b80,0x0020,0x0000
                | $092A28 op $0F set_limits: minX=2912 minY=8 maxX=2944 maxY=32 slope=0
        .dc.w   0x0006
                | $092A34 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A36 --- callback 68000 embebido (16 B) -> PC=$092A46 ---
        cmpi.w  #0xb70, 0x106f50.l             | $092A36  cmpi.w #$b70, $106f50.l
        scs     d0                             | $092A3E  scs.b d0
        lea     .L092a46(pc), a1              | $092A40  a1 = nuevo PC
        rts                                    | $092A44  rts
.L092a46:
        .dc.w   0x0006
                | $092A46 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A48 --- callback 68000 embebido (16 B) -> PC=$092A58 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092A48  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092A50  sne.b d0
        lea     .L092a58(pc), a1              | $092A52  a1 = nuevo PC
        rts                                    | $092A56  rts
.L092a58:
        .dc.w   0x000f,0x0c60,0x0008,0x0c80,0x0020,0x0000
                | $092A58 op $0F set_limits: minX=3168 minY=8 maxX=3200 maxY=32 slope=0
        .dc.w   0x0006
                | $092A64 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A66 --- callback 68000 embebido (16 B) -> PC=$092A76 ---
        cmpi.w  #0xc58, 0x106f50.l             | $092A66  cmpi.w #$c58, $106f50.l
        scs     d0                             | $092A6E  scs.b d0
        lea     .L092a76(pc), a1              | $092A70  a1 = nuevo PC
        rts                                    | $092A74  rts
.L092a76:
        .dc.w   0x0006
                | $092A76 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A78 --- callback 68000 embebido (16 B) -> PC=$092A88 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092A78  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092A80  sne.b d0
        lea     .L092a88(pc), a1              | $092A82  a1 = nuevo PC
        rts                                    | $092A86  rts
.L092a88:
        .dc.w   0x000f,0x0ce0,0x0008,0x0d00,0x0020,0x0000
                | $092A88 op $0F set_limits: minX=3296 minY=8 maxX=3328 maxY=32 slope=0
        .dc.w   0x0006
                | $092A94 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092A96 --- callback 68000 embebido (16 B) -> PC=$092AA6 ---
        cmpi.w  #0xcd8, 0x106f50.l             | $092A96  cmpi.w #$cd8, $106f50.l
        scs     d0                             | $092A9E  scs.b d0
        lea     .L092aa6(pc), a1              | $092AA0  a1 = nuevo PC
        rts                                    | $092AA4  rts
.L092aa6:
        .dc.w   0x0006
                | $092AA6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092AA8 --- callback 68000 embebido (16 B) -> PC=$092AB8 ---
        cmpi.b  #0x1, 0x10e39a.l               | $092AA8  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092AB0  sne.b d0
        lea     .L092ab8(pc), a1              | $092AB2  a1 = nuevo PC
        rts                                    | $092AB6  rts
.L092ab8:
        .dc.w   0x000f,0x0e40,0x0008,0x0ea0,0x0020,0x0000
                | $092AB8 op $0F set_limits: minX=3648 minY=8 maxX=3744 maxY=32 slope=0
        .dc.w   0x0002
                | $092AC4 op $02 END_FRAME: cede el frame (integra scroll+camara)
        .dc.w   0x0000,0x0f03,0x0008,0x1002,0x0014,0x0d05,0x0016,0x0c06,0x0019,0x0d05,0x001b
                | $092AC6 op $00 bind_path: ent=$F030008 ruta=$10020014.. cont=$C060019 ancla=(3333,27)

        .globl  SceneTrig_092ADC
        .section .text.SceneTrig_092ADC, "ax", @progbits
SceneTrig_092ADC:                           | tabla de trigger (op $11), 22 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0e04,0x0020,0x0f03,0x002d,0x1002,0x002f,0x1101,0x0030 | $092ADC
        .dc.w   0x1200,0xffff,0x0000                   | $092AEC

        .globl  SceneTrig_092AF2
        .section .text.SceneTrig_092AF2, "ax", @progbits
SceneTrig_092AF2:                           | tabla de trigger (op $11), 30 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0b06,0x0001,0x070a,0x0009,0x0b06,0x0011,0x070a,0x0031 | $092AF2
        .dc.w   0x0b06,0x0042,0x1100,0x0049,0x0b06,0xffff,0x0000 | $092B02

        .globl  SceneTrig_092B10
        .section .text.SceneTrig_092B10, "ax", @progbits
SceneTrig_092B10:                           | tabla de trigger (op $11), 136 B  (sin referencia directa op $11 en este script)
        .dc.w   0x1002,0x000a,0x1200,0x000d,0x0f03,0x000e,0x0c06,0x000f | $092B10
        .dc.w   0x0b07,0x0012,0x0f03,0x0013,0x1002,0x0015,0x0f03,0x0017 | $092B20
        .dc.w   0x1101,0x0018,0x1200,0x001b,0x1101,0x001c,0x1002,0x001d | $092B30
        .dc.w   0x0d05,0x0020,0x0b07,0x0023,0x0d05,0x0024,0x0e04,0x0026 | $092B40
        .dc.w   0x0d05,0x0027,0x0b07,0x0028,0x0a08,0x0029,0x0c06,0x002a | $092B50
        .dc.w   0x0f03,0x002d,0x0c06,0x002e,0x0f03,0x002f,0x1002,0x0032 | $092B60
        .dc.w   0x0d05,0x0034,0x0b07,0x0036,0x1002,0x0039,0x0d05,0x003a | $092B70
        .dc.w   0x1101,0x003b,0x1200,0x003f,0x1002,0x0040,0x1200,0x0053 | $092B80
        .dc.w   0x0f03,0x0055,0x1200,0xffff            | $092B90

        .globl  SceneEntities_092B98
        .section .text.SceneEntities_092B98, "ax", @progbits
SceneEntities_092B98:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0015,0x0013
                | $092B98 type=1 subop=00 tmpl=$1080E0 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $092BA6 type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0015,0x0013
                | $092BB4 type=0 subop=00 tmpl=$107FE8 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0x0000,0x0020,0x0013
                | $092BC2 type=1 subop=00 tmpl=$106F6C payload=ffa0000000200013
        .dc.w   0x0250                        | $092BD0 terminador type=2

        .globl  SceneScript_092BD2
        .section .text.SceneScript_092BD2, "ax", @progbits
SceneScript_092BD2:                         | bytecode VM de escena (1522 B, 90 ops)
        .dc.w   0x0003,0x0000,0x0430
                | $092BD2 op $03 warp: camara=(0,1072) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $092BD8 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0112                 |   slot $10 <- bank $0112
        .dc.w   0x0011,0x0113                 |   slot $11 <- bank $0113
        .dc.w   0x0012,0x0114                 |   slot $12 <- bank $0114
        .dc.w   0x0013,0x0115                 |   slot $13 <- bank $0115
        .dc.w   0x0014,0x0116                 |   slot $14 <- bank $0116
        .dc.w   0x0015,0x0117                 |   slot $15 <- bank $0117
        .dc.w   0x0016,0x0118                 |   slot $16 <- bank $0118
        .dc.w   0x0017,0x0119                 |   slot $17 <- bank $0119
        .dc.w   0x0018,0x011a                 |   slot $18 <- bank $011A
        .dc.w   0x0019,0x011b                 |   slot $19 <- bank $011B
        .dc.w   0x001a,0x011c                 |   slot $1A <- bank $011C
        .dc.w   0x001b,0x011d                 |   slot $1B <- bank $011D
        .dc.w   0x001c,0x011e                 |   slot $1C <- bank $011E
        .dc.w   0x001d,0x011f                 |   slot $1D <- bank $011F
        .dc.w   0x001e,0x0120                 |   slot $1E <- bank $0120
        .dc.w   0x001f,0x0121                 |   slot $1F <- bank $0121
        .dc.w   0x0020,0x0122                 |   slot $20 <- bank $0122
        .dc.w   0x0021,0x0123                 |   slot $21 <- bank $0123
        .dc.w   0x0022,0x0124                 |   slot $22 <- bank $0124
        .dc.w   0x0023,0x0125                 |   slot $23 <- bank $0125
        .dc.w   0x0024,0x0126                 |   slot $24 <- bank $0126
        .dc.w   0x0025,0x0127                 |   slot $25 <- bank $0127
        .dc.w   0x0026,0x0128                 |   slot $26 <- bank $0128
        .dc.w   0x0027,0x0129                 |   slot $27 <- bank $0129
        .dc.w   0x0028,0x012a                 |   slot $28 <- bank $012A
        .dc.w   0x0029,0x012b                 |   slot $29 <- bank $012B
        .dc.w   0x002a,0x012c                 |   slot $2A <- bank $012C
        .dc.w   0x002b,0x012d                 |   slot $2B <- bank $012D
        .dc.w   0x002c,0x012e                 |   slot $2C <- bank $012E
        .dc.w   0x002d,0x012f                 |   slot $2D <- bank $012F
        .dc.w   0x002e,0x0130                 |   slot $2E <- bank $0130
        .dc.w   0x002f,0x0131                 |   slot $2F <- bank $0131
        .dc.w   0x0030,0x0132                 |   slot $30 <- bank $0132
        .dc.w   0x0031,0x0133                 |   slot $31 <- bank $0133
        .dc.w   0x0032,0x0134                 |   slot $32 <- bank $0134
        .dc.w   0x0033,0x0135                 |   slot $33 <- bank $0135
        .dc.w   0x0034,0x0136                 |   slot $34 <- bank $0136
        .dc.w   0x0035,0x0137                 |   slot $35 <- bank $0137
        .dc.w   0x0036,0x0138                 |   slot $36 <- bank $0138
        .dc.w   0x0037,0x0139                 |   slot $37 <- bank $0139
        .dc.w   0x0038,0x013a                 |   slot $38 <- bank $013A
        .dc.w   0x0039,0x013b                 |   slot $39 <- bank $013B
        .dc.w   0x003a,0x013c                 |   slot $3A <- bank $013C
        .dc.w   0x003b,0x013d                 |   slot $3B <- bank $013D
        .dc.w   0x003c,0x013e                 |   slot $3C <- bank $013E
        .dc.w   0x003d,0x013f                 |   slot $3D <- bank $013F
        .dc.w   0x003e,0x01f1                 |   slot $3E <- bank $01F1
        .dc.w   0x003f,0x01f2                 |   slot $3F <- bank $01F2
        .dc.w   0x0040,0x01f3                 |   slot $40 <- bank $01F3
        .dc.w   0x0041,0x01f4                 |   slot $41 <- bank $01F4
        .dc.w   0x0042,0x01f5                 |   slot $42 <- bank $01F5
        .dc.w   0x0043,0x01f6                 |   slot $43 <- bank $01F6
        .dc.w   0x0044,0x01e8                 |   slot $44 <- bank $01E8
        .dc.w   0x0045,0x01e9                 |   slot $45 <- bank $01E9
        .dc.w   0x0046,0x01ea                 |   slot $46 <- bank $01EA
        .dc.w   0x0047,0x01eb                 |   slot $47 <- bank $01EB
        .dc.w   0x0048,0x01ec                 |   slot $48 <- bank $01EC
        .dc.w   0x0049,0x01ed                 |   slot $49 <- bank $01ED
        .dc.w   0x004a,0x01ee                 |   slot $4A <- bank $01EE
        .dc.w   0x004b,0x01ef                 |   slot $4B <- bank $01EF
        .dc.w   0x004c,0x01f0                 |   slot $4C <- bank $01F0
        .dc.w   0x004d,0x01f7                 |   slot $4D <- bank $01F7
        .dc.w   0x004e,0x01f8                 |   slot $4E <- bank $01F8
        .dc.w   0x004f,0x01f9                 |   slot $4F <- bank $01F9
        .dc.w   0x0050,0x01fa                 |   slot $50 <- bank $01FA
        .dc.w   0x0051,0x01fb                 |   slot $51 <- bank $01FB
        .dc.w   0x0052,0x01fc                 |   slot $52 <- bank $01FC
        .dc.w   0x0053,0x01fd                 |   slot $53 <- bank $01FD
        .dc.w   0x0054,0x01fe                 |   slot $54 <- bank $01FE
        .dc.w   0x0055,0x01ff                 |   slot $55 <- bank $01FF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x013f                 |   slot $FF <- bank $013F
        .dc.w   0xffff                              | $092D72 fin de pares
        .dc.w   0x000f,0x0000,0x0430,0x0000,0x0430,0x0000
                | $092D74 op $0F set_limits: minX=0 minY=1072 maxX=0 maxY=1072 slope=0
        .dc.w   0x0010,0x0080,0x0090
                | $092D80 op $10 set_6466: $108164=0080 $108166=0090
        .dc.w   0x0012,0x0200
                | $092D86 op $12 set_axis: modo_eje=$02
        .dc.w   0x000c,0x0600
                | $092D8A op $0C set_6eac: $106EAC.b = 06
        .dc.w   0x0000,0x0010,0x6f6c,0x0020,0x0055,0x000c,0x62a4,0x0021,0x29b4,0xffc0,0xfff0
                | $092D8E op $00 bind_path: ent=$106F6C ruta=$200055.. cont=$2129B4 ancla=(-64,-16)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $092DA4 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x6f6c,0x2000
                | $092DB0 op $04 spawn: tmpl=$106F6C a=20 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $092DB8 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x80e0,0x0014,0x0033,0x000c,0x8d24,0x0000,0x0000,0x0000,0x01e0
                | $092DBE op $00 bind_path: ent=$1080E0 ruta=$140033.. cont=$000000 ancla=(0,480)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0080,0x0080
                | $092DD4 op $01 set_fields: ent=$1080E0 +72=03 +74=$0080 +76=$0080
        .dc.w   0x0004,0x0010,0x80e0,0x2000
                | $092DE0 op $04 spawn: tmpl=$1080E0 a=20 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $092DE8 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092DEE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092DF0 --- callback 68000 embebido (16 B) -> PC=$092E00 ---
        cmpi.w  #0x368, 0x106f54.l             | $092DF0  cmpi.w #$368, $106f54.l
        shi     d0                             | $092DF8  shi.b d0
        lea     .L092e00(pc), a1              | $092DFA  a1 = nuevo PC
        rts                                    | $092DFE  rts
.L092e00:
        .dc.w   0x0010,0x0080,0x0100
                | $092E00 op $10 set_6466: $108164=0080 $108166=0100
        .dc.w   0x0006
                | $092E06 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E08 --- callback 68000 embebido (16 B) -> PC=$092E18 ---
        cmpi.w  #0x340, 0x106f54.l             | $092E08  cmpi.w #$340, $106f54.l
        shi     d0                             | $092E10  shi.b d0
        lea     .L092e18(pc), a1              | $092E12  a1 = nuevo PC
        rts                                    | $092E16  rts
.L092e18:
        .dc.w   0x0010,0x0080,0x0090
                | $092E18 op $10 set_6466: $108164=0080 $108166=0090
        .dc.w   0x0006
                | $092E1E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E20 --- callback 68000 embebido (16 B) -> PC=$092E30 ---
        cmpi.w  #0x300, 0x106f54.l             | $092E20  cmpi.w #$300, $106f54.l
        shi     d0                             | $092E28  shi.b d0
        lea     .L092e30(pc), a1              | $092E2A  a1 = nuevo PC
        rts                                    | $092E2E  rts
.L092e30:
        .dc.w   0x0006
                | $092E30 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E32 --- callback 68000 embebido (16 B) -> PC=$092E42 ---
        cmpi.w  #0x270, 0x106f54.l             | $092E32  cmpi.w #$270, $106f54.l
        shi     d0                             | $092E3A  shi.b d0
        lea     .L092e42(pc), a1              | $092E3C  a1 = nuevo PC
        rts                                    | $092E40  rts
.L092e42:
        .dc.w   0x000f,0x0000,0x0230,0x0000,0x0230,0x0000
                | $092E42 op $0F set_limits: minX=0 minY=560 maxX=0 maxY=560 slope=0
        .dc.w   0x0010,0x0080,0x0100
                | $092E4E op $10 set_6466: $108164=0080 $108166=0100
        .dc.w   0x0006
                | $092E54 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E56 --- callback 68000 embebido (16 B) -> PC=$092E66 ---
        cmpi.w  #0x230, 0x106f54.l             | $092E56  cmpi.w #$230, $106f54.l
        shi     d0                             | $092E5E  shi.b d0
        lea     .L092e66(pc), a1              | $092E60  a1 = nuevo PC
        rts                                    | $092E64  rts
.L092e66:
        .dc.w   0x0010,0x0080,0x0090
                | $092E66 op $10 set_6466: $108164=0080 $108166=0090
        .dc.w   0x0006
                | $092E6C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E6E --- callback 68000 embebido (16 B) -> PC=$092E7E ---
        cmpi.b  #0x1, 0x10e39a.l               | $092E6E  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092E76  sne.b d0
        lea     .L092e7e(pc), a1              | $092E78  a1 = nuevo PC
        rts                                    | $092E7C  rts
.L092e7e:
        .dc.w   0x0006
                | $092E7E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E80 --- callback 68000 embebido (16 B) -> PC=$092E90 ---
        cmpi.w  #0x60, 0x106f54.l              | $092E80  cmpi.w #$60, $106f54.l
        shi     d0                             | $092E88  shi.b d0
        lea     .L092e90(pc), a1              | $092E8A  a1 = nuevo PC
        rts                                    | $092E8E  rts
.L092e90:
        .dc.w   0x0010,0x0080,0x0100
                | $092E90 op $10 set_6466: $108164=0080 $108166=0100
        .dc.w   0x0006
                | $092E96 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092E98 --- callback 68000 embebido (16 B) -> PC=$092EA8 ---
        cmpi.w  #0x10, 0x106f54.l              | $092E98  cmpi.w #$10, $106f54.l
        shi     d0                             | $092EA0  shi.b d0
        lea     .L092ea8(pc), a1              | $092EA2  a1 = nuevo PC
        rts                                    | $092EA6  rts
.L092ea8:
        .dc.w   0x0010,0x0080,0x00a0
                | $092EA8 op $10 set_6466: $108164=0080 $108166=00A0
        .dc.w   0x000d,0x0005,0x2796,0x0000,0x0000,0x0000,0x0000
                | $092EAE op $0D call_args: fn=$052796 args=0000000000000000
        .dc.w   0x000f,0x0180,0x0010,0x0180,0x0010,0x0000
                | $092EBC op $0F set_limits: minX=384 minY=16 maxX=384 maxY=16 slope=0
        .dc.w   0x0012,0x0100
                | $092EC8 op $12 set_axis: modo_eje=$01
        .dc.w   0x0000,0x0010,0x80e0,0x0020,0x0011,0x000c,0x9f34,0x0000,0x0000,0x0140,0xffd0
                | $092ECC op $00 bind_path: ent=$1080E0 ruta=$200011.. cont=$000000 ancla=(320,-48)
        .dc.w   0x0011,0x0010,0x80e0,0x0009,0x329a
                | $092EE2 op $11 set_trig: ent=$1080E0 tabla=$09329A  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0080,0x0080
                | $092EEC op $01 set_fields: ent=$1080E0 +72=03 +74=$0080 +76=$0080
        .dc.w   0x0009,0x0010,0x80e0
                | $092EF8 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092EFE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092F00 --- callback 68000 embebido (16 B) -> PC=$092F10 ---
        cmpi.w  #0x20, 0x106f50.l              | $092F00  cmpi.w #$20, $106f50.l
        scs     d0                             | $092F08  scs.b d0
        lea     .L092f10(pc), a1              | $092F0A  a1 = nuevo PC
        rts                                    | $092F0E  rts
.L092f10:
        .dc.w   0x0000,0x0010,0x6f6c,0x009e,0x0013,0x000c,0x33bc,0x0020,0xfacc,0x01c0,0x0000
                | $092F10 op $00 bind_path: ent=$106F6C ruta=$9E0013.. cont=$20FACC ancla=(448,0)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x31ee
                | $092F26 op $11 set_trig: ent=$106F6C tabla=$0931EE  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $092F30 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x6f6c
                | $092F3C op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0006
                | $092F42 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092F44 --- callback 68000 embebido (16 B) -> PC=$092F54 ---
        cmpi.w  #0x150, 0x106f50.l             | $092F44  cmpi.w #$150, $106f50.l
        scs     d0                             | $092F4C  scs.b d0
        lea     .L092f54(pc), a1              | $092F4E  a1 = nuevo PC
        rts                                    | $092F52  rts
.L092f54:
        .dc.w   0x0000,0x0010,0x7fe8,0x0022,0x000a,0x000c,0x2750,0x0000,0x0000,0x0290,0x0070
                | $092F54 op $00 bind_path: ent=$107FE8 ruta=$22000A.. cont=$000000 ancla=(656,112)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x31ae
                | $092F6A op $11 set_trig: ent=$107FE8 tabla=$0931AE  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $092F74 op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $092F80 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092F86 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092F88 --- callback 68000 embebido (16 B) -> PC=$092F98 ---
        cmpi.w  #0x180, 0x106f50.l             | $092F88  cmpi.w #$180, $106f50.l
        scs     d0                             | $092F90  scs.b d0
        lea     .L092f98(pc), a1              | $092F92  a1 = nuevo PC
        rts                                    | $092F96  rts
.L092f98:
        .dc.w   0x0006
                | $092F98 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092F9A --- callback 68000 embebido (16 B) -> PC=$092FAA ---
        cmpi.b  #0x1, 0x10e39a.l               | $092F9A  cmpi.b #$1, $10e39a.l
        sne     d0                             | $092FA2  sne.b d0
        lea     .L092faa(pc), a1              | $092FA4  a1 = nuevo PC
        rts                                    | $092FA8  rts
.L092faa:
        .dc.w   0x000f,0x04f0,0x0010,0x04f0,0x0010,0x0000
                | $092FAA op $0F set_limits: minX=1264 minY=16 maxX=1264 maxY=16 slope=0
        .dc.w   0x0006
                | $092FB6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092FB8 --- callback 68000 embebido (16 B) -> PC=$092FC8 ---
        cmpi.w  #0x220, 0x106f50.l             | $092FB8  cmpi.w #$220, $106f50.l
        scs     d0                             | $092FC0  scs.b d0
        lea     .L092fc8(pc), a1              | $092FC2  a1 = nuevo PC
        rts                                    | $092FC6  rts
.L092fc8:
        .dc.w   0x0000,0x0010,0x8064,0x0011,0x0008,0x000c,0x9d14,0x0000,0x0000,0x0360,0x0016
                | $092FC8 op $00 bind_path: ent=$108064 ruta=$110008.. cont=$000000 ancla=(864,22)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x00c0,0x0080
                | $092FDE op $01 set_fields: ent=$108064 +72=03 +74=$00C0 +76=$0080
        .dc.w   0x0004,0x0010,0x8064,0x0800
                | $092FEA op $04 spawn: tmpl=$108064 a=08 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x8064
                | $092FF2 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $092FF8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $092FFA --- callback 68000 embebido (16 B) -> PC=$09300A ---
        cmpi.w  #0x350, 0x106f50.l             | $092FFA  cmpi.w #$350, $106f50.l
        scs     d0                             | $093002  scs.b d0
        lea     .L09300a(pc), a1              | $093004  a1 = nuevo PC
        rts                                    | $093008  rts
.L09300a:
        .dc.w   0x0005,0x0010,0x7fe8
                | $09300A op $05 detach: ent=$107FE8  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093010 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093012 --- callback 68000 embebido (16 B) -> PC=$093022 ---
        cmpi.w  #0x3e0, 0x106f50.l             | $093012  cmpi.w #$3e0, $106f50.l
        scs     d0                             | $09301A  scs.b d0
        lea     .L093022(pc), a1              | $09301C  a1 = nuevo PC
        rts                                    | $093020  rts
.L093022:
        .dc.w   0x0005,0x0010,0x80e0
                | $093022 op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093028 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09302A --- callback 68000 embebido (16 B) -> PC=$09303A ---
        cmpi.w  #0x4a0, 0x106f50.l             | $09302A  cmpi.w #$4a0, $106f50.l
        scs     d0                             | $093032  scs.b d0
        lea     .L09303a(pc), a1              | $093034  a1 = nuevo PC
        rts                                    | $093038  rts
.L09303a:
        .dc.w   0x0010,0x0030,0x00a0
                | $09303A op $10 set_6466: $108164=0030 $108166=00A0
        .dc.w   0x0006
                | $093040 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093042 --- callback 68000 embebido (16 B) -> PC=$093052 ---
        cmpi.w  #0x4b0, 0x106f50.l             | $093042  cmpi.w #$4b0, $106f50.l
        scs     d0                             | $09304A  scs.b d0
        lea     .L093052(pc), a1              | $09304C  a1 = nuevo PC
        rts                                    | $093050  rts
.L093052:
        .dc.w   0x0005,0x0010,0x8064
                | $093052 op $05 detach: ent=$108064  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093058 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09305A --- callback 68000 embebido (16 B) -> PC=$09306A ---
        cmpi.w  #0x4f0, 0x106f50.l             | $09305A  cmpi.w #$4f0, $106f50.l
        scs     d0                             | $093062  scs.b d0
        lea     .L09306a(pc), a1              | $093064  a1 = nuevo PC
        rts                                    | $093068  rts
.L09306a:
        .dc.w   0x000d,0x0005,0x2756,0x0027,0x013f,0xffff,0x0000
                | $09306A op $0D call_args: fn=$052756 args=0027013fffff0000
        .dc.w   0x000c,0x0100
                | $093078 op $0C set_6eac: $106EAC.b = 01
        .dc.w   0x0000,0x0010,0x80e0,0x0035,0x000a,0x000c,0xa7b4,0x0000,0x0000,0x04f0,0x0000
                | $09307C op $00 bind_path: ent=$1080E0 ruta=$35000A.. cont=$000000 ancla=(1264,0)
        .dc.w   0x0011,0x0010,0x80e0,0x0009,0x32b0
                | $093092 op $11 set_trig: ent=$1080E0 tabla=$0932B0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0040,0x0100
                | $09309C op $01 set_fields: ent=$1080E0 +72=03 +74=$0040 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $0930A8 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0010,0x0080,0x00a0
                | $0930AE op $10 set_6466: $108164=0080 $108166=00A0
        .dc.w   0x0006
                | $0930B4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0930B6 --- callback 68000 embebido (16 B) -> PC=$0930C6 ---
        cmpi.b  #0x2, 0x10e39a.l               | $0930B6  cmpi.b #$2, $10e39a.l
        sne     d0                             | $0930BE  sne.b d0
        lea     .L0930c6(pc), a1              | $0930C0  a1 = nuevo PC
        rts                                    | $0930C4  rts
.L0930c6:
        .dc.w   0x0000,0x0010,0x7fe8,0x0041,0x0007,0x000c,0x2ca0,0x0000,0x0000,0x04f0,0x00a8
                | $0930C6 op $00 bind_path: ent=$107FE8 ruta=$410007.. cont=$000000 ancla=(1264,168)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x31d4
                | $0930DC op $11 set_trig: ent=$107FE8 tabla=$0931D4  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0180,0x0100
                | $0930E6 op $01 set_fields: ent=$107FE8 +72=03 +74=$0180 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $0930F2 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0930F8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0930FA --- callback 68000 embebido (16 B) -> PC=$09310A ---
        cmpi.b  #0x1, 0x10e39a.l               | $0930FA  cmpi.b #$1, $10e39a.l
        sne     d0                             | $093102  sne.b d0
        lea     .L09310a(pc), a1              | $093104  a1 = nuevo PC
        rts                                    | $093108  rts
.L09310a:
        .dc.w   0x000f,0x08e8,0x0000,0x08e8,0x0010,0x0000
                | $09310A op $0F set_limits: minX=2280 minY=0 maxX=2280 maxY=16 slope=0
        .dc.w   0x0006
                | $093116 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093118 --- callback 68000 embebido (16 B) -> PC=$093128 ---
        cmpi.w  #0x6c0, 0x106f50.l             | $093118  cmpi.w #$6c0, $106f50.l
        scs     d0                             | $093120  scs.b d0
        lea     .L093128(pc), a1              | $093122  a1 = nuevo PC
        rts                                    | $093126  rts
.L093128:
        .dc.w   0x0005,0x0010,0x7fe8
                | $093128 op $05 detach: ent=$107FE8  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $09312E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093130 --- callback 68000 embebido (16 B) -> PC=$093140 ---
        cmpi.w  #0x7d0, 0x106f50.l             | $093130  cmpi.w #$7d0, $106f50.l
        scs     d0                             | $093138  scs.b d0
        lea     .L093140(pc), a1              | $09313A  a1 = nuevo PC
        rts                                    | $09313E  rts
.L093140:
        .dc.w   0x0005,0x0010,0x80e0
                | $093140 op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093146 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093148 --- callback 68000 embebido (16 B) -> PC=$093158 ---
        cmpi.w  #0x8e8, 0x106f50.l             | $093148  cmpi.w #$8e8, $106f50.l
        scs     d0                             | $093150  scs.b d0
        lea     .L093158(pc), a1              | $093152  a1 = nuevo PC
        rts                                    | $093156  rts
.L093158:
        .dc.w   0x0006
                | $093158 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09315A --- callback 68000 embebido (16 B) -> PC=$09316A ---
        cmpi.w  #0x0, 0x106e88.l               | $09315A  cmpi.w #$0, $106e88.l
        sne     d0                             | $093162  sne.b d0
        lea     .L09316a(pc), a1              | $093164  a1 = nuevo PC
        rts                                    | $093168  rts
.L09316a:
        .dc.w   0x000f,0x0a60,0x0000,0x0a60,0x0010,0x0000
                | $09316A op $0F set_limits: minX=2656 minY=0 maxX=2656 maxY=16 slope=0
        .dc.w   0x0010,0x0060,0x0040
                | $093176 op $10 set_6466: $108164=0060 $108166=0040
        .dc.w   0x0006
                | $09317C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09317E --- callback 68000 embebido (16 B) -> PC=$09318E ---
        cmpi.w  #0xa48, 0x106f50.l             | $09317E  cmpi.w #$a48, $106f50.l
        scs     d0                             | $093186  scs.b d0
        lea     .L09318e(pc), a1              | $093188  a1 = nuevo PC
        rts                                    | $09318C  rts
.L09318e:
        .dc.w   0x0006
                | $09318E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093190 --- callback 68000 embebido (16 B) -> PC=$0931A0 ---
        cmpi.b  #0x1, 0x10e39a.l               | $093190  cmpi.b #$1, $10e39a.l
        sne     d0                             | $093198  sne.b d0
        lea     .L0931a0(pc), a1              | $09319A  a1 = nuevo PC
        rts                                    | $09319E  rts
.L0931a0:
        .dc.w   0x000f,0x0a00,0x0000,0x0a60,0x0010,0x0000
                | $0931A0 op $0F set_limits: minX=2560 minY=0 maxX=2656 maxY=16 slope=0
        .dc.w   0x0002
                | $0931AC op $02 END_FRAME: cede el frame (integra scroll+camara)
        .dc.w   0x0000,0x0300,0x0001,0x0400,0x0002,0x0a00,0x0003,0x0901,0x0004,0x0802,0x0005
                | $0931AE op $00 bind_path: ent=$3000001 ruta=$4000002.. cont=$9010004 ancla=(2050,5)

        .globl  SceneTrig_0931C4
        .section .text.SceneTrig_0931C4, "ax", @progbits
SceneTrig_0931C4:                           | tabla de trigger (op $11), 18 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0406,0x0006,0x0307,0x000d,0x0208,0x000e,0x0000,0xffff | $0931C4
        .dc.w   0x0000                                 | $0931D4

        .globl  SceneTrig_0931D6
        .section .text.SceneTrig_0931D6, "ax", @progbits
SceneTrig_0931D6:                           | tabla de trigger (op $11), 26 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0700,0x0027,0x0601,0x0028,0x0502,0x0029,0x0304,0x002a | $0931D6
        .dc.w   0x0205,0x002b,0x0000,0xffff,0x0000     | $0931E6

        .globl  SceneTrig_0931F0
        .section .text.SceneTrig_0931F0, "ax", @progbits
SceneTrig_0931F0:                           | tabla de trigger (op $11), 142 B  (sin referencia directa op $11 en este script)
        .dc.w   0x1100,0x0005,0x0a07,0x0006,0x0908,0x0007,0x0a07,0x0008 | $0931F0
        .dc.w   0x0b06,0x0009,0x0c05,0x000a,0x0d04,0x000b,0x1100,0x0011 | $093200
        .dc.w   0x1001,0x0012,0x0e03,0x0013,0x0d04,0x0017,0x0e03,0x001c | $093210
        .dc.w   0x0f02,0x001d,0x1001,0x0014,0x1100,0x0024,0x0f00,0x0025 | $093220
        .dc.w   0x0800,0x0026,0x0b06,0x0027,0x1100,0x002a,0x1001,0x002b | $093230
        .dc.w   0x0f02,0x002d,0x0e03,0x002f,0x1100,0x0047,0x0d04,0x0048 | $093240
        .dc.w   0x0804,0x004c,0x0f02,0x004e,0x0e03,0x004f,0x0d04,0x0050 | $093250
        .dc.w   0x0b06,0x0053,0x0c05,0x0055,0x0d04,0x0056,0x0e03,0x0058 | $093260
        .dc.w   0x0c05,0x005b,0x0b06,0x0060,0x1100,0xffff,0x0000 | $093270

        .globl  SceneTrig_09327E
        .section .text.SceneTrig_09327E, "ax", @progbits
SceneTrig_09327E:                           | tabla de trigger (op $11), 30 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0a00,0x0022,0x0b00,0x0023,0x0c00,0x0030,0x0804,0x0033 | $09327E
        .dc.w   0x0705,0x0035,0x0804,0x0037,0x0000,0xffff,0x0000 | $09328E

        .globl  SceneTrig_09329C
        .section .text.SceneTrig_09329C, "ax", @progbits
SceneTrig_09329C:                           | tabla de trigger (op $11), 22 B  (sin referencia directa op $11 en este script)
        .dc.w   0x1100,0x0006,0x0f00,0x0007,0x0d00,0x0008,0x0a00,0x0009 | $09329C
        .dc.w   0x0900,0xffff,0x0000                   | $0932AC

        .globl  SceneTrig_0932B2
        .section .text.SceneTrig_0932B2, "ax", @progbits
SceneTrig_0932B2:                           | tabla de trigger (op $11), 12 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0a00,0x0014,0x0700,0x0021,0x0000,0xffff | $0932B2

        .globl  SceneEntities_0932BE
        .section .text.SceneEntities_0932BE, "ax", @progbits
SceneEntities_0932BE:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0015,0x0013
                | $0932BE type=1 subop=00 tmpl=$1080E0 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $0932CC type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0015,0x0013
                | $0932DA type=0 subop=00 tmpl=$107FE8 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0x0000,0x0020,0x0013
                | $0932E8 type=1 subop=00 tmpl=$106F6C payload=ffa0000000200013
        .dc.w   0x0250                        | $0932F6 terminador type=2

        .globl  SceneScript_0932F8
        .section .text.SceneScript_0932F8, "ax", @progbits
SceneScript_0932F8:                         | bytecode VM de escena (586 B, 16 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $0932F8 op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $0932FE op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0140                 |   slot $10 <- bank $0140
        .dc.w   0x0011,0x0141                 |   slot $11 <- bank $0141
        .dc.w   0x0012,0x0142                 |   slot $12 <- bank $0142
        .dc.w   0x0013,0x0143                 |   slot $13 <- bank $0143
        .dc.w   0x0014,0x0144                 |   slot $14 <- bank $0144
        .dc.w   0x0015,0x0145                 |   slot $15 <- bank $0145
        .dc.w   0x0016,0x0146                 |   slot $16 <- bank $0146
        .dc.w   0x0017,0x0147                 |   slot $17 <- bank $0147
        .dc.w   0x0018,0x0148                 |   slot $18 <- bank $0148
        .dc.w   0x0019,0x0149                 |   slot $19 <- bank $0149
        .dc.w   0x001a,0x014a                 |   slot $1A <- bank $014A
        .dc.w   0x001b,0x014b                 |   slot $1B <- bank $014B
        .dc.w   0x001c,0x014c                 |   slot $1C <- bank $014C
        .dc.w   0x001d,0x014d                 |   slot $1D <- bank $014D
        .dc.w   0x001e,0x014e                 |   slot $1E <- bank $014E
        .dc.w   0x001f,0x014f                 |   slot $1F <- bank $014F
        .dc.w   0x0020,0x0150                 |   slot $20 <- bank $0150
        .dc.w   0x0021,0x0151                 |   slot $21 <- bank $0151
        .dc.w   0x0022,0x0152                 |   slot $22 <- bank $0152
        .dc.w   0x0023,0x0153                 |   slot $23 <- bank $0153
        .dc.w   0x0024,0x0154                 |   slot $24 <- bank $0154
        .dc.w   0x0025,0x0155                 |   slot $25 <- bank $0155
        .dc.w   0x0026,0x0156                 |   slot $26 <- bank $0156
        .dc.w   0x0027,0x0157                 |   slot $27 <- bank $0157
        .dc.w   0x0028,0x0158                 |   slot $28 <- bank $0158
        .dc.w   0x0029,0x0159                 |   slot $29 <- bank $0159
        .dc.w   0x002a,0x015a                 |   slot $2A <- bank $015A
        .dc.w   0x002b,0x015b                 |   slot $2B <- bank $015B
        .dc.w   0x002c,0x015c                 |   slot $2C <- bank $015C
        .dc.w   0x002d,0x015d                 |   slot $2D <- bank $015D
        .dc.w   0x002e,0x015e                 |   slot $2E <- bank $015E
        .dc.w   0x002f,0x015f                 |   slot $2F <- bank $015F
        .dc.w   0x0030,0x0160                 |   slot $30 <- bank $0160
        .dc.w   0x0031,0x0161                 |   slot $31 <- bank $0161
        .dc.w   0x0032,0x0162                 |   slot $32 <- bank $0162
        .dc.w   0x0033,0x0163                 |   slot $33 <- bank $0163
        .dc.w   0x0034,0x0164                 |   slot $34 <- bank $0164
        .dc.w   0x0035,0x0165                 |   slot $35 <- bank $0165
        .dc.w   0x0036,0x0166                 |   slot $36 <- bank $0166
        .dc.w   0x0037,0x0167                 |   slot $37 <- bank $0167
        .dc.w   0x0038,0x0168                 |   slot $38 <- bank $0168
        .dc.w   0x0039,0x0169                 |   slot $39 <- bank $0169
        .dc.w   0x003a,0x016a                 |   slot $3A <- bank $016A
        .dc.w   0x003b,0x016b                 |   slot $3B <- bank $016B
        .dc.w   0x003c,0x016c                 |   slot $3C <- bank $016C
        .dc.w   0x003d,0x016d                 |   slot $3D <- bank $016D
        .dc.w   0x003e,0x016e                 |   slot $3E <- bank $016E
        .dc.w   0x003f,0x016f                 |   slot $3F <- bank $016F
        .dc.w   0x0040,0x0170                 |   slot $40 <- bank $0170
        .dc.w   0x0041,0x0171                 |   slot $41 <- bank $0171
        .dc.w   0x0042,0x0172                 |   slot $42 <- bank $0172
        .dc.w   0x0043,0x0173                 |   slot $43 <- bank $0173
        .dc.w   0x0044,0x0174                 |   slot $44 <- bank $0174
        .dc.w   0x0045,0x0175                 |   slot $45 <- bank $0175
        .dc.w   0x0046,0x0176                 |   slot $46 <- bank $0176
        .dc.w   0x0047,0x0177                 |   slot $47 <- bank $0177
        .dc.w   0x0048,0x0178                 |   slot $48 <- bank $0178
        .dc.w   0x0049,0x0179                 |   slot $49 <- bank $0179
        .dc.w   0x004a,0x017a                 |   slot $4A <- bank $017A
        .dc.w   0x004b,0x017b                 |   slot $4B <- bank $017B
        .dc.w   0x004c,0x017c                 |   slot $4C <- bank $017C
        .dc.w   0x004d,0x017d                 |   slot $4D <- bank $017D
        .dc.w   0x004e,0x017e                 |   slot $4E <- bank $017E
        .dc.w   0x004f,0x017f                 |   slot $4F <- bank $017F
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x013f                 |   slot $FF <- bank $013F
        .dc.w   0xffff                              | $093498 fin de pares
        .dc.w   0x000f,0x0b10,0x0000,0x0b20,0x0000,0x0000
                | $09349A op $0F set_limits: minX=2832 minY=0 maxX=2848 maxY=0 slope=0
        .dc.w   0x0010,0x0078,0x00a0
                | $0934A6 op $10 set_6466: $108164=0078 $108166=00A0
        .dc.w   0x000c,0x1800
                | $0934AC op $0C set_6eac: $106EAC.b = 18
        .dc.w   0x0000,0x0010,0x6f6c,0x010e,0x0010,0x000c,0xe5fc,0x0021,0x5434,0x0000,0x0000
                | $0934B0 op $00 bind_path: ent=$106F6C ruta=$10E0010.. cont=$215434 ancla=(0,0)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0934C6 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x6f6c,0x1000
                | $0934D2 op $04 spawn: tmpl=$106F6C a=10 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $0934DA op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0934E0 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0934E2 --- callback 68000 embebido (16 B) -> PC=$0934F2 ---
        cmpi.w  #0xb20, 0x106f50.l             | $0934E2  cmpi.w #$b20, $106f50.l
        scs     d0                             | $0934EA  scs.b d0
        lea     .L0934f2(pc), a1              | $0934EC  a1 = nuevo PC
        rts                                    | $0934F0  rts
.L0934f2:
        .dc.w   0x0006
                | $0934F2 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0934F4 --- callback 68000 embebido (16 B) -> PC=$093504 ---
        cmpi.b  #0x1, 0x10e39a.l               | $0934F4  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0934FC  sne.b d0
        lea     .L093504(pc), a1              | $0934FE  a1 = nuevo PC
        rts                                    | $093502  rts
.L093504:
        .dc.w   0x000f,0x0c90,0x0000,0x0ca0,0x0000,0x0000
                | $093504 op $0F set_limits: minX=3216 minY=0 maxX=3232 maxY=0 slope=0
        .dc.w   0x0006
                | $093510 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093512 --- callback 68000 embebido (16 B) -> PC=$093522 ---
        cmpi.w  #0xca0, 0x106f50.l             | $093512  cmpi.w #$ca0, $106f50.l
        scs     d0                             | $09351A  scs.b d0
        lea     .L093522(pc), a1              | $09351C  a1 = nuevo PC
        rts                                    | $093520  rts
.L093522:
        .dc.w   0x0006
                | $093522 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093524 --- callback 68000 embebido (16 B) -> PC=$093534 ---
        cmpi.b  #0x1, 0x10e39a.l               | $093524  cmpi.b #$1, $10e39a.l
        sne     d0                             | $09352C  sne.b d0
        lea     .L093534(pc), a1              | $09352E  a1 = nuevo PC
        rts                                    | $093532  rts
.L093534:
        .dc.w   0x000f,0x0f40,0x0000,0x0fa0,0x0000,0x0000
                | $093534 op $0F set_limits: minX=3904 minY=0 maxX=4000 maxY=0 slope=0
        .dc.w   0x0002
                | $093540 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_093542
        .section .text.SceneEntities_093542, "ax", @progbits
SceneEntities_093542:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0015,0x0013
                | $093542 type=1 subop=00 tmpl=$1080E0 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $093550 type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0015,0x0013
                | $09355E type=0 subop=00 tmpl=$107FE8 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0x0000,0x0020,0x0020
                | $09356C type=1 subop=00 tmpl=$106F6C payload=ffa0000000200020
        .dc.w   0x0250                        | $09357A terminador type=2

        .globl  SceneScript_09357C
        .section .text.SceneScript_09357C, "ax", @progbits
SceneScript_09357C:                         | bytecode VM de escena (1642 B, 96 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $09357C op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $093582 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0080                 |   slot $10 <- bank $0080
        .dc.w   0x0011,0x0081                 |   slot $11 <- bank $0081
        .dc.w   0x0012,0x0082                 |   slot $12 <- bank $0082
        .dc.w   0x0013,0x0083                 |   slot $13 <- bank $0083
        .dc.w   0x0014,0x0084                 |   slot $14 <- bank $0084
        .dc.w   0x0015,0x0085                 |   slot $15 <- bank $0085
        .dc.w   0x0016,0x0086                 |   slot $16 <- bank $0086
        .dc.w   0x0017,0x0087                 |   slot $17 <- bank $0087
        .dc.w   0x0018,0x0088                 |   slot $18 <- bank $0088
        .dc.w   0x0019,0x0089                 |   slot $19 <- bank $0089
        .dc.w   0x001a,0x008a                 |   slot $1A <- bank $008A
        .dc.w   0x001b,0x008b                 |   slot $1B <- bank $008B
        .dc.w   0x001c,0x008c                 |   slot $1C <- bank $008C
        .dc.w   0x001d,0x008d                 |   slot $1D <- bank $008D
        .dc.w   0x001e,0x008e                 |   slot $1E <- bank $008E
        .dc.w   0x001f,0x008f                 |   slot $1F <- bank $008F
        .dc.w   0x0020,0x0090                 |   slot $20 <- bank $0090
        .dc.w   0x0021,0x0091                 |   slot $21 <- bank $0091
        .dc.w   0x0022,0x0092                 |   slot $22 <- bank $0092
        .dc.w   0x0023,0x0093                 |   slot $23 <- bank $0093
        .dc.w   0x0024,0x0094                 |   slot $24 <- bank $0094
        .dc.w   0x0025,0x0095                 |   slot $25 <- bank $0095
        .dc.w   0x0026,0x0096                 |   slot $26 <- bank $0096
        .dc.w   0x0027,0x0097                 |   slot $27 <- bank $0097
        .dc.w   0x0028,0x0098                 |   slot $28 <- bank $0098
        .dc.w   0x0029,0x0099                 |   slot $29 <- bank $0099
        .dc.w   0x002a,0x009a                 |   slot $2A <- bank $009A
        .dc.w   0x002b,0x009b                 |   slot $2B <- bank $009B
        .dc.w   0x002c,0x009c                 |   slot $2C <- bank $009C
        .dc.w   0x002d,0x009d                 |   slot $2D <- bank $009D
        .dc.w   0x002e,0x009e                 |   slot $2E <- bank $009E
        .dc.w   0x002f,0x009f                 |   slot $2F <- bank $009F
        .dc.w   0x0030,0x00a0                 |   slot $30 <- bank $00A0
        .dc.w   0x0031,0x00a1                 |   slot $31 <- bank $00A1
        .dc.w   0x0032,0x00a2                 |   slot $32 <- bank $00A2
        .dc.w   0x0033,0x00a3                 |   slot $33 <- bank $00A3
        .dc.w   0x0034,0x00a4                 |   slot $34 <- bank $00A4
        .dc.w   0x0035,0x00a5                 |   slot $35 <- bank $00A5
        .dc.w   0x0036,0x00a6                 |   slot $36 <- bank $00A6
        .dc.w   0x0037,0x00a7                 |   slot $37 <- bank $00A7
        .dc.w   0x0038,0x00a8                 |   slot $38 <- bank $00A8
        .dc.w   0x0039,0x00a9                 |   slot $39 <- bank $00A9
        .dc.w   0x003a,0x00aa                 |   slot $3A <- bank $00AA
        .dc.w   0x003b,0x00ab                 |   slot $3B <- bank $00AB
        .dc.w   0x003c,0x00ac                 |   slot $3C <- bank $00AC
        .dc.w   0x003d,0x00ad                 |   slot $3D <- bank $00AD
        .dc.w   0x003e,0x00ae                 |   slot $3E <- bank $00AE
        .dc.w   0x003f,0x00af                 |   slot $3F <- bank $00AF
        .dc.w   0x0040,0x00b0                 |   slot $40 <- bank $00B0
        .dc.w   0x0041,0x00b1                 |   slot $41 <- bank $00B1
        .dc.w   0x0042,0x00b2                 |   slot $42 <- bank $00B2
        .dc.w   0x0043,0x00b3                 |   slot $43 <- bank $00B3
        .dc.w   0x0044,0x00b4                 |   slot $44 <- bank $00B4
        .dc.w   0x0045,0x00b5                 |   slot $45 <- bank $00B5
        .dc.w   0x0046,0x00b6                 |   slot $46 <- bank $00B6
        .dc.w   0x0047,0x00b7                 |   slot $47 <- bank $00B7
        .dc.w   0x0048,0x00bf                 |   slot $48 <- bank $00BF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x00b0                 |   slot $FF <- bank $00B0
        .dc.w   0xffff                              | $09371C fin de pares
        .dc.w   0x000f,0x0670,0x0000,0x0670,0x0000,0x0000
                | $09371E op $0F set_limits: minX=1648 minY=0 maxX=1648 maxY=0 slope=0
        .dc.w   0x0010,0x0068,0x00a0
                | $09372A op $10 set_6466: $108164=0068 $108166=00A0
        .dc.w   0x000c,0x0a00
                | $093730 op $0C set_6eac: $106EAC.b = 0A
        .dc.w   0x0000,0x0010,0x6f6c,0x007b,0x0010,0x000d,0x6e54,0x0021,0x97b4,0x0000,0x0008
                | $093734 op $00 bind_path: ent=$106F6C ruta=$7B0010.. cont=$2197B4 ancla=(0,8)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x3bd0
                | $09374A op $11 set_trig: ent=$106F6C tabla=$093BD0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $093754 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $093760 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x80e0,0x0010,0x0010,0x000d,0xc30c,0x0000,0x0000,0x0040,0x0008
                | $093766 op $00 bind_path: ent=$1080E0 ruta=$100010.. cont=$000000 ancla=(64,8)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0080,0x0080
                | $09377C op $01 set_fields: ent=$1080E0 +72=03 +74=$0080 +76=$0080
        .dc.w   0x0004,0x0010,0x80e0,0x1000
                | $093788 op $04 spawn: tmpl=$1080E0 a=10 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $093790 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $093796 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093798 --- callback 68000 embebido (16 B) -> PC=$0937A8 ---
        cmpi.w  #0xb0, 0x106f50.l              | $093798  cmpi.w #$b0, $106f50.l
        scs     d0                             | $0937A0  scs.b d0
        lea     .L0937a8(pc), a1              | $0937A2  a1 = nuevo PC
        rts                                    | $0937A6  rts
.L0937a8:
        .dc.w   0x0010,0x0028,0x00a0
                | $0937A8 op $10 set_6466: $108164=0028 $108166=00A0
        .dc.w   0x0006
                | $0937AE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0937B0 --- callback 68000 embebido (16 B) -> PC=$0937C0 ---
        cmpi.w  #0x100, 0x106f50.l             | $0937B0  cmpi.w #$100, $106f50.l
        scs     d0                             | $0937B8  scs.b d0
        lea     .L0937c0(pc), a1              | $0937BA  a1 = nuevo PC
        rts                                    | $0937BE  rts
.L0937c0:
        .dc.w   0x0010,0x0068,0x00a0
                | $0937C0 op $10 set_6466: $108164=0068 $108166=00A0
        .dc.w   0x0006
                | $0937C6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0937C8 --- callback 68000 embebido (16 B) -> PC=$0937D8 ---
        cmpi.w  #0x100, 0x106f50.l             | $0937C8  cmpi.w #$100, $106f50.l
        scs     d0                             | $0937D0  scs.b d0
        lea     .L0937d8(pc), a1              | $0937D2  a1 = nuevo PC
        rts                                    | $0937D6  rts
.L0937d8:
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x00b1,0xffff,0x0000
                | $0937D8 op $0D call_args: fn=$052756 args=00ff00b1ffff0000
        .dc.w   0x0010,0x0088,0x00a0
                | $0937E6 op $10 set_6466: $108164=0088 $108166=00A0
        .dc.w   0x0006
                | $0937EC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0937EE --- callback 68000 embebido (16 B) -> PC=$0937FE ---
        cmpi.w  #0x140, 0x106f50.l             | $0937EE  cmpi.w #$140, $106f50.l
        scs     d0                             | $0937F6  scs.b d0
        lea     .L0937fe(pc), a1              | $0937F8  a1 = nuevo PC
        rts                                    | $0937FC  rts
.L0937fe:
        .dc.w   0x0000,0x0010,0x80e0,0x0020,0x0007,0x000d,0xc70c,0x0000,0x0000,0x0280,0x0008
                | $0937FE op $00 bind_path: ent=$1080E0 ruta=$200007.. cont=$000000 ancla=(640,8)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x00e0,0x0000
                | $093814 op $01 set_fields: ent=$1080E0 +72=03 +74=$00E0 +76=$0000
        .dc.w   0x0004,0x0010,0x80e0,0x0700
                | $093820 op $04 spawn: tmpl=$1080E0 a=07 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $093828 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $09382E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093830 --- callback 68000 embebido (16 B) -> PC=$093840 ---
        cmpi.w  #0x300, 0x106f50.l             | $093830  cmpi.w #$300, $106f50.l
        scs     d0                             | $093838  scs.b d0
        lea     .L093840(pc), a1              | $09383A  a1 = nuevo PC
        rts                                    | $09383E  rts
.L093840:
        .dc.w   0x000f,0x0670,0x0000,0x0670,0x0010,0x0000
                | $093840 op $0F set_limits: minX=1648 minY=0 maxX=1648 maxY=16 slope=0
        .dc.w   0x0006
                | $09384C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09384E --- callback 68000 embebido (16 B) -> PC=$09385E ---
        cmpi.w  #0x508, 0x106f50.l             | $09384E  cmpi.w #$508, $106f50.l
        scs     d0                             | $093856  scs.b d0
        lea     .L09385e(pc), a1              | $093858  a1 = nuevo PC
        rts                                    | $09385C  rts
.L09385e:
        .dc.w   0x0010,0x0028,0x00a0
                | $09385E op $10 set_6466: $108164=0028 $108166=00A0
        .dc.w   0x0006
                | $093864 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093866 --- callback 68000 embebido (16 B) -> PC=$093876 ---
        cmpi.w  #0x558, 0x106f50.l             | $093866  cmpi.w #$558, $106f50.l
        scs     d0                             | $09386E  scs.b d0
        lea     .L093876(pc), a1              | $093870  a1 = nuevo PC
        rts                                    | $093874  rts
.L093876:
        .dc.w   0x0010,0x0088,0x00a0
                | $093876 op $10 set_6466: $108164=0088 $108166=00A0
        .dc.w   0x0006
                | $09387C op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09387E --- callback 68000 embebido (16 B) -> PC=$09388E ---
        cmpi.w  #0x640, 0x106f50.l             | $09387E  cmpi.w #$640, $106f50.l
        scs     d0                             | $093886  scs.b d0
        lea     .L09388e(pc), a1              | $093888  a1 = nuevo PC
        rts                                    | $09388C  rts
.L09388e:
        .dc.w   0x0010,0x0028,0x00a0
                | $09388E op $10 set_6466: $108164=0028 $108166=00A0
        .dc.w   0x0006
                | $093894 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093896 --- callback 68000 embebido (16 B) -> PC=$0938A6 ---
        cmpi.w  #0x670, 0x106f50.l             | $093896  cmpi.w #$670, $106f50.l
        scs     d0                             | $09389E  scs.b d0
        lea     .L0938a6(pc), a1              | $0938A0  a1 = nuevo PC
        rts                                    | $0938A4  rts
.L0938a6:
        .dc.w   0x0010,0x0088,0x00a0
                | $0938A6 op $10 set_6466: $108164=0088 $108166=00A0
        .dc.w   0x0014
                | $0938AC op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $0938AE --- callback 68000 embebido (14 B) -> PC=$0938BC ---
        move.b  #0x1, 0x10e39c.l               | $0938AE  move.b #$1, $10e39c.l
        lea     .L0938bc(pc), a0              | $0938B6  a0 = nuevo PC
        rts                                    | $0938BA  rts
.L0938bc:
        .dc.w   0x0006
                | $0938BC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0938BE --- callback 68000 embebido (16 B) -> PC=$0938CE ---
        cmpi.b  #0x1, 0x10e39a.l               | $0938BE  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0938C6  sne.b d0
        lea     .L0938ce(pc), a1              | $0938C8  a1 = nuevo PC
        rts                                    | $0938CC  rts
.L0938ce:
        .dc.w   0x000b,0x0010,0x6f6c
                | $0938CE op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0008,0x0010,0x6f6c
                | $0938D4 op $08 call_ed6: ent=$106F6C  ($51ED6)
        .dc.w   0x000f,0x0670,0x0000,0x0670,0x0280,0x0000
                | $0938DA op $0F set_limits: minX=1648 minY=0 maxX=1648 maxY=640 slope=0
        .dc.w   0x0010,0x0088,0x0050
                | $0938E6 op $10 set_6466: $108164=0088 $108166=0050
        .dc.w   0x0000,0x0010,0x8064,0x0014,0x0010,0x000d,0xca8c,0x0000,0x0000,0x0670,0x0100
                | $0938EC op $00 bind_path: ent=$108064 ruta=$140010.. cont=$000000 ancla=(1648,256)
        .dc.w   0x0011,0x0010,0x8064,0x0009,0x3c76
                | $093902 op $11 set_trig: ent=$108064 tabla=$093C76  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x00c0,0x0200
                | $09390C op $01 set_fields: ent=$108064 +72=03 +74=$00C0 +76=$0200
        .dc.w   0x0009,0x0010,0x8064
                | $093918 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x00b5,0x0004,0x0000
                | $09391E op $0D call_args: fn=$052756 args=00ff00b500040000
        .dc.w   0x000d,0x0005,0x2756,0x001d,0x009c,0x0004,0x0000
                | $09392C op $0D call_args: fn=$052756 args=001d009c00040000
        .dc.w   0x0006
                | $09393A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09393C --- callback 68000 embebido (16 B) -> PC=$09394C ---
        cmpi.w  #0x200, 0x106f54.l             | $09393C  cmpi.w #$200, $106f54.l
        scs     d0                             | $093944  scs.b d0
        lea     .L09394c(pc), a1              | $093946  a1 = nuevo PC
        rts                                    | $09394A  rts
.L09394c:
        .dc.w   0x0005,0x0010,0x8064
                | $09394C op $05 detach: ent=$108064  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093952 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093954 --- callback 68000 embebido (16 B) -> PC=$093964 ---
        cmpi.w  #0x150, 0x106f54.l             | $093954  cmpi.w #$150, $106f54.l
        scs     d0                             | $09395C  scs.b d0
        lea     .L093964(pc), a1              | $09395E  a1 = nuevo PC
        rts                                    | $093962  rts
.L093964:
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x0012,0x000d,0x8d14,0x0021,0xb674,0x0670,0x0280
                | $093964 op $00 bind_path: ent=$106F6C ruta=$480012.. cont=$21B674 ancla=(1648,640)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x3bea
                | $09397A op $11 set_trig: ent=$106F6C tabla=$093BEA  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $093984 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $093990 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $093996 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093998 --- callback 68000 embebido (16 B) -> PC=$0939A8 ---
        cmpi.w  #0x230, 0x106f54.l             | $093998  cmpi.w #$230, $106f54.l
        scs     d0                             | $0939A0  scs.b d0
        lea     .L0939a8(pc), a1              | $0939A2  a1 = nuevo PC
        rts                                    | $0939A6  rts
.L0939a8:
        .dc.w   0x0000,0x0010,0x7fe8,0x0061,0x0002,0x000d,0x6b4c,0x0000,0x0000,0x0670,0x0360
                | $0939A8 op $00 bind_path: ent=$107FE8 ruta=$610002.. cont=$000000 ancla=(1648,864)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $0939BE op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x7fe8,0x0200
                | $0939CA op $04 spawn: tmpl=$107FE8 a=02 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $0939D2 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0939D8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0939DA --- callback 68000 embebido (16 B) -> PC=$0939EA ---
        cmpi.w  #0x280, 0x106f54.l             | $0939DA  cmpi.w #$280, $106f54.l
        scs     d0                             | $0939E2  scs.b d0
        lea     .L0939ea(pc), a1              | $0939E4  a1 = nuevo PC
        rts                                    | $0939E8  rts
.L0939ea:
        .dc.w   0x0006
                | $0939EA op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0939EC --- callback 68000 embebido (16 B) -> PC=$0939FC ---
        cmpi.b  #0x1, 0x10e39a.l               | $0939EC  cmpi.b #$1, $10e39a.l
        sne     d0                             | $0939F4  sne.b d0
        lea     .L0939fc(pc), a1              | $0939F6  a1 = nuevo PC
        rts                                    | $0939FA  rts
.L0939fc:
        .dc.w   0x000f,0x0671,0x0280,0x0671,0x0280,0x0000
                | $0939FC op $0F set_limits: minX=1649 minY=640 maxX=1649 maxY=640 slope=0
        .dc.w   0x0010,0x0088,0x00a0
                | $093A08 op $10 set_6466: $108164=0088 $108166=00A0
        .dc.w   0x0006
                | $093A0E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A10 --- callback 68000 embebido (16 B) -> PC=$093A20 ---
        cmpi.w  #0x671, 0x106f50.l             | $093A10  cmpi.w #$671, $106f50.l
        scs     d0                             | $093A18  scs.b d0
        lea     .L093a20(pc), a1              | $093A1A  a1 = nuevo PC
        rts                                    | $093A1E  rts
.L093a20:
        .dc.w   0x0006
                | $093A20 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A22 --- callback 68000 embebido (16 B) -> PC=$093A32 ---
        cmpi.w  #0x0, 0x106e88.l               | $093A22  cmpi.w #$0, $106e88.l
        sne     d0                             | $093A2A  sne.b d0
        lea     .L093a32(pc), a1              | $093A2C  a1 = nuevo PC
        rts                                    | $093A30  rts
.L093a32:
        .dc.w   0x000f,0x0672,0x0280,0x0672,0x0280,0x0000
                | $093A32 op $0F set_limits: minX=1650 minY=640 maxX=1650 maxY=640 slope=0
        .dc.w   0x0006
                | $093A3E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A40 --- callback 68000 embebido (16 B) -> PC=$093A50 ---
        cmpi.w  #0x672, 0x106f50.l             | $093A40  cmpi.w #$672, $106f50.l
        scs     d0                             | $093A48  scs.b d0
        lea     .L093a50(pc), a1              | $093A4A  a1 = nuevo PC
        rts                                    | $093A4E  rts
.L093a50:
        .dc.w   0x0006
                | $093A50 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A52 --- callback 68000 embebido (16 B) -> PC=$093A62 ---
        cmpi.w  #0x0, 0x106e88.l               | $093A52  cmpi.w #$0, $106e88.l
        sne     d0                             | $093A5A  sne.b d0
        lea     .L093a62(pc), a1              | $093A5C  a1 = nuevo PC
        rts                                    | $093A60  rts
.L093a62:
        .dc.w   0x000f,0x07d0,0x0280,0x07d0,0x0280,0x0000
                | $093A62 op $0F set_limits: minX=2000 minY=640 maxX=2000 maxY=640 slope=0
        .dc.w   0x0006
                | $093A6E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A70 --- callback 68000 embebido (16 B) -> PC=$093A80 ---
        cmpi.w  #0x7d0, 0x106f50.l             | $093A70  cmpi.w #$7d0, $106f50.l
        scs     d0                             | $093A78  scs.b d0
        lea     .L093a80(pc), a1              | $093A7A  a1 = nuevo PC
        rts                                    | $093A7E  rts
.L093a80:
        .dc.w   0x0006
                | $093A80 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093A82 --- callback 68000 embebido (16 B) -> PC=$093A92 ---
        cmpi.b  #0x1, 0x10e39a.l               | $093A82  cmpi.b #$1, $10e39a.l
        sne     d0                             | $093A8A  sne.b d0
        lea     .L093a92(pc), a1              | $093A8C  a1 = nuevo PC
        rts                                    | $093A90  rts
.L093a92:
        .dc.w   0x000f,0x0950,0x0280,0x0950,0x0280,0x0000
                | $093A92 op $0F set_limits: minX=2384 minY=640 maxX=2384 maxY=640 slope=0
        .dc.w   0x0006
                | $093A9E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093AA0 --- callback 68000 embebido (16 B) -> PC=$093AB0 ---
        cmpi.w  #0x950, 0x106f50.l             | $093AA0  cmpi.w #$950, $106f50.l
        scs     d0                             | $093AA8  scs.b d0
        lea     .L093ab0(pc), a1              | $093AAA  a1 = nuevo PC
        rts                                    | $093AAE  rts
.L093ab0:
        .dc.w   0x0006
                | $093AB0 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093AB2 --- callback 68000 embebido (16 B) -> PC=$093AC2 ---
        cmpi.b  #0x1, 0x10e39a.l               | $093AB2  cmpi.b #$1, $10e39a.l
        sne     d0                             | $093ABA  sne.b d0
        lea     .L093ac2(pc), a1              | $093ABC  a1 = nuevo PC
        rts                                    | $093AC0  rts
.L093ac2:
        .dc.w   0x000f,0x0ee0,0x0280,0x0ee0,0x0280,0x0000
                | $093AC2 op $0F set_limits: minX=3808 minY=640 maxX=3808 maxY=640 slope=0
        .dc.w   0x0006
                | $093ACE op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093AD0 --- callback 68000 embebido (16 B) -> PC=$093AE0 ---
        cmpi.w  #0x950, 0x106f50.l             | $093AD0  cmpi.w #$950, $106f50.l
        scs     d0                             | $093AD8  scs.b d0
        lea     .L093ae0(pc), a1              | $093ADA  a1 = nuevo PC
        rts                                    | $093ADE  rts
.L093ae0:
        .dc.w   0x0000,0x0010,0x6f6c,0x0053,0x001a,0x000d,0xa154,0x0021,0xcab4,0x0af0,0x0200
                | $093AE0 op $00 bind_path: ent=$106F6C ruta=$53001A.. cont=$21CAB4 ancla=(2800,512)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x3c0c
                | $093AF6 op $11 set_trig: ent=$106F6C tabla=$093C0C  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $093B00 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $093B0C op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $093B12 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093B14 --- callback 68000 embebido (16 B) -> PC=$093B24 ---
        cmpi.w  #0xaf0, 0x106f50.l             | $093B14  cmpi.w #$af0, $106f50.l
        scs     d0                             | $093B1C  scs.b d0
        lea     .L093b24(pc), a1              | $093B1E  a1 = nuevo PC
        rts                                    | $093B22  rts
.L093b24:
        .dc.w   0x000f,0x0d60,0x0200,0x0d60,0x0280,0x0040
                | $093B24 op $0F set_limits: minX=3424 minY=512 maxX=3424 maxY=640 slope=64
        .dc.w   0x0006
                | $093B30 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093B32 --- callback 68000 embebido (16 B) -> PC=$093B42 ---
        cmpi.w  #0xb40, 0x106f50.l             | $093B32  cmpi.w #$b40, $106f50.l
        scs     d0                             | $093B3A  scs.b d0
        lea     .L093b42(pc), a1              | $093B3C  a1 = nuevo PC
        rts                                    | $093B40  rts
.L093b42:
        .dc.w   0x0005,0x0010,0x7fe8
                | $093B42 op $05 detach: ent=$107FE8  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $093B48 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093B4A --- callback 68000 embebido (16 B) -> PC=$093B5A ---
        cmpi.w  #0xc40, 0x106f50.l             | $093B4A  cmpi.w #$c40, $106f50.l
        scs     d0                             | $093B52  scs.b d0
        lea     .L093b5a(pc), a1              | $093B54  a1 = nuevo PC
        rts                                    | $093B58  rts
.L093b5a:
        .dc.w   0x0005,0x0010,0x80e0
                | $093B5A op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x00b6,0xffff,0x0000
                | $093B60 op $0D call_args: fn=$052756 args=00ff00b6ffff0000
        .dc.w   0x0006
                | $093B6E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093B70 --- callback 68000 embebido (16 B) -> PC=$093B80 ---
        cmpi.w  #0xcb0, 0x106f50.l             | $093B70  cmpi.w #$cb0, $106f50.l
        scs     d0                             | $093B78  scs.b d0
        lea     .L093b80(pc), a1              | $093B7A  a1 = nuevo PC
        rts                                    | $093B7E  rts
.L093b80:
        .dc.w   0x0000,0x0010,0x80e0,0x0020,0x0007,0x000d,0xcf8c,0x0000,0x0000,0x0cb0,0x0218
                | $093B80 op $00 bind_path: ent=$1080E0 ruta=$200007.. cont=$000000 ancla=(3248,536)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0050,0x0080
                | $093B96 op $01 set_fields: ent=$1080E0 +72=03 +74=$0050 +76=$0080
        .dc.w   0x0004,0x0010,0x80e0,0x0700
                | $093BA2 op $04 spawn: tmpl=$1080E0 a=07 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $093BAA op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $093BB0 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $093BB2 --- callback 68000 embebido (16 B) -> PC=$093BC2 ---
        cmpi.w  #0xd60, 0x106f50.l             | $093BB2  cmpi.w #$d60, $106f50.l
        scs     d0                             | $093BBA  scs.b d0
        lea     .L093bc2(pc), a1              | $093BBC  a1 = nuevo PC
        rts                                    | $093BC0  rts
.L093bc2:
        .dc.w   0x000f,0x0e98,0x01f0,0x0ee0,0x0200,0x0000
                | $093BC2 op $0F set_limits: minX=3736 minY=496 maxX=3808 maxY=512 slope=0
        .dc.w   0x0002
                | $093BCE op $02 END_FRAME: cede el frame (integra scroll+camara)
        .dc.w   0x0000,0x1000,0x000a,0x0808,0x000d,0x1000,0x0032,0x0907,0x0068,0x0a06,0x0076
                | $093BD0 op $00 bind_path: ent=$1000000A ruta=$808000D.. cont=$9070068 ancla=(2566,118)

        .globl  SceneTrig_093BE6
        .section .text.SceneTrig_093BE6, "ax", @progbits
SceneTrig_093BE6:                           | tabla de trigger (op $11), 6 B  (sin referencia directa op $11 en este script)
        .dc.w   0x040c,0xffff,0x0000                   | $093BE6

        .globl  SceneTrig_093BEC
        .section .text.SceneTrig_093BEC, "ax", @progbits
SceneTrig_093BEC:                           | tabla de trigger (op $11), 34 B  (sin referencia directa op $11 en este script)
        .dc.w   0x020c,0x001a,0x0905,0x002e,0x0a04,0x002f,0x0905,0x003d | $093BEC
        .dc.w   0x0a04,0x003e,0x0c02,0x0041,0x0a04,0x0044,0x020c,0xffff | $093BFC
        .dc.w   0x0000                                 | $093C0C

        .globl  SceneTrig_093C0E
        .section .text.SceneTrig_093C0E, "ax", @progbits
SceneTrig_093C0E:                           | tabla de trigger (op $11), 106 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0214,0x0001,0x0511,0x0002,0x090d,0x0004,0x0e08,0x0005 | $093C0E
        .dc.w   0x1008,0x0007,0x0f09,0x000b,0x1107,0x000d,0x1206,0x0014 | $093C1E
        .dc.w   0x1800,0x0025,0x1700,0x0029,0x1600,0x002d,0x1500,0x0031 | $093C2E
        .dc.w   0x1400,0x0035,0x1300,0x0036,0x0d06,0x0038,0x0d05,0x0039 | $093C3E
        .dc.w   0x0f03,0x003b,0x1200,0x003c,0x1100,0x0042,0x0b06,0x0045 | $093C4E
        .dc.w   0x0c05,0x0046,0x0d04,0x004c,0x0e03,0x004e,0x0f02,0x0052 | $093C5E
        .dc.w   0x0e03,0x0053,0x0000,0xffff,0x0000     | $093C6E

        .globl  SceneTrig_093C78
        .section .text.SceneTrig_093C78, "ax", @progbits
SceneTrig_093C78:                           | tabla de trigger (op $11), 40 B  (sin referencia directa op $11 en este script)
        .dc.w   0x0f00,0x0002,0x0e00,0x0003,0x0d00,0x0005,0x0600,0x0007 | $093C78
        .dc.w   0x0709,0x0009,0x0907,0x000a,0x0b05,0x000c,0x0c04,0x000e | $093C88
        .dc.w   0x0d03,0x0014,0x0000,0xffff            | $093C98

        .globl  SceneEntities_093CA0
        .section .text.SceneEntities_093CA0, "ax", @progbits
SceneEntities_093CA0:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0015,0x0013
                | $093CA0 type=1 subop=00 tmpl=$1080E0 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $093CAE type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0015,0x0013
                | $093CBC type=0 subop=00 tmpl=$107FE8 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0015,0x0013
                | $093CCA type=1 subop=00 tmpl=$106F6C payload=0000000000150013
        .dc.w   0x0250                        | $093CD8 terminador type=2

        .globl  SceneScript_093CDA
        .section .text.SceneScript_093CDA, "ax", @progbits
SceneScript_093CDA:                         | bytecode VM de escena (582 B, 17 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $093CDA op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $093CE0 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x01a1                 |   slot $10 <- bank $01A1
        .dc.w   0x0011,0x01a2                 |   slot $11 <- bank $01A2
        .dc.w   0x0012,0x01a3                 |   slot $12 <- bank $01A3
        .dc.w   0x0013,0x01a4                 |   slot $13 <- bank $01A4
        .dc.w   0x0014,0x0070                 |   slot $14 <- bank $0070
        .dc.w   0x0015,0x01ae                 |   slot $15 <- bank $01AE
        .dc.w   0x0016,0x01af                 |   slot $16 <- bank $01AF
        .dc.w   0x0017,0x01b0                 |   slot $17 <- bank $01B0
        .dc.w   0x0018,0xffff                 |   slot $18 <- bank $FFFF
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $093E7A fin de pares
        .dc.w   0x000f,0x0000,0x0000,0x0000,0x0000,0x0000
                | $093E7C op $0F set_limits: minX=0 minY=0 maxX=0 maxY=0 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $093E88 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x0012,0x0009,0x000e,0x3170,0x0021,0xec6c,0x0010,0x0020
                | $093E8E op $00 bind_path: ent=$106F6C ruta=$120009.. cont=$21EC6C ancla=(16,32)
        .dc.w   0x0004,0x0010,0x6f6c,0x0900
                | $093EA4 op $04 spawn: tmpl=$106F6C a=09 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $093EAC op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $093EB8 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0012,0x0004,0x000e,0x33f8,0x0000,0x0000,0x0010,0x0020
                | $093EBE op $00 bind_path: ent=$108064 ruta=$120004.. cont=$000000 ancla=(16,32)
        .dc.w   0x0004,0x0010,0x8064,0x0400
                | $093ED4 op $04 spawn: tmpl=$108064 a=04 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0100,0x0100
                | $093EDC op $01 set_fields: ent=$108064 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $093EE8 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x7fe8,0x0014,0x0010,0x000e,0x3518,0x0000,0x0000,0x0000,0x0000
                | $093EEE op $00 bind_path: ent=$107FE8 ruta=$140010.. cont=$000000 ancla=(0,0)
        .dc.w   0x0004,0x0010,0x7fe8,0x1000
                | $093F04 op $04 spawn: tmpl=$107FE8 a=10 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $093F0C op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $093F18 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0002
                | $093F1E op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_093F20
        .section .text.SceneEntities_093F20, "ax", @progbits
SceneEntities_093F20:                       | 1 registros de 14 B + terminador (16 B)
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0015,0x0013
                | $093F20 type=1 subop=00 tmpl=$106F6C payload=0000000000150013
        .dc.w   0x0200                        | $093F2E terminador type=2

        .globl  SceneScript_093F30
        .section .text.SceneScript_093F30, "ax", @progbits
SceneScript_093F30:                         | bytecode VM de escena (492 B, 10 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $093F30 op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $093F36 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0810,0x0198                 |   slot $810 <- bank $0198
        .dc.w   0x0811,0x0199                 |   slot $811 <- bank $0199
        .dc.w   0x0812,0x019a                 |   slot $812 <- bank $019A
        .dc.w   0x0013,0xffff                 |   slot $13 <- bank $FFFF
        .dc.w   0x0014,0xffff                 |   slot $14 <- bank $FFFF
        .dc.w   0x0015,0xffff                 |   slot $15 <- bank $FFFF
        .dc.w   0x0016,0xffff                 |   slot $16 <- bank $FFFF
        .dc.w   0x0017,0xffff                 |   slot $17 <- bank $FFFF
        .dc.w   0x0018,0xffff                 |   slot $18 <- bank $FFFF
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $0940D0 fin de pares
        .dc.w   0x000f,0x0000,0x0000,0x0000,0x0000,0x0000
                | $0940D2 op $0F set_limits: minX=0 minY=0 maxX=0 maxY=0 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0940DE op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x0014,0x0009,0x000e,0x29a0,0x0000,0x0000,0x0000,0x0040
                | $0940E4 op $00 bind_path: ent=$106F6C ruta=$140009.. cont=$000000 ancla=(0,64)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0000,0x0000
                | $0940FA op $01 set_fields: ent=$106F6C +72=03 +74=$0000 +76=$0000
        .dc.w   0x0004,0x0010,0x6f6c,0x0900
                | $094106 op $04 spawn: tmpl=$106F6C a=09 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $09410E op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0007,0x0010,0x6f6c
                | $094114 op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0002
                | $09411A op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_09411C
        .section .text.SceneEntities_09411C, "ax", @progbits
SceneEntities_09411C:                       | 1 registros de 14 B + terminador (16 B)
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0015,0x0013
                | $09411C type=1 subop=00 tmpl=$106F6C payload=0000000000150013
        .dc.w   0x0250                        | $09412A terminador type=2

        .globl  SceneScript_09412C
        .section .text.SceneScript_09412C, "ax", @progbits
SceneScript_09412C:                         | bytecode VM de escena (486 B, 9 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $09412C op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $094132 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x00b8                 |   slot $10 <- bank $00B8
        .dc.w   0x0011,0x00b9                 |   slot $11 <- bank $00B9
        .dc.w   0x0012,0xffff                 |   slot $12 <- bank $FFFF
        .dc.w   0x0013,0xffff                 |   slot $13 <- bank $FFFF
        .dc.w   0x0014,0xffff                 |   slot $14 <- bank $FFFF
        .dc.w   0x0015,0xffff                 |   slot $15 <- bank $FFFF
        .dc.w   0x0016,0xffff                 |   slot $16 <- bank $FFFF
        .dc.w   0x0017,0xffff                 |   slot $17 <- bank $FFFF
        .dc.w   0x0018,0xffff                 |   slot $18 <- bank $FFFF
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $0942CC fin de pares
        .dc.w   0x000f,0x0000,0x0000,0x0000,0x0000,0x0000
                | $0942CE op $0F set_limits: minX=0 minY=0 maxX=0 maxY=0 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0942DA op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x0014,0x0010,0x000e,0x2c70,0x0000,0x0000,0x0000,0x0000
                | $0942E0 op $00 bind_path: ent=$106F6C ruta=$140010.. cont=$000000 ancla=(0,0)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0000,0x0000
                | $0942F6 op $01 set_fields: ent=$106F6C +72=03 +74=$0000 +76=$0000
        .dc.w   0x0004,0x0010,0x6f6c,0x1000
                | $094302 op $04 spawn: tmpl=$106F6C a=10 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $09430A op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0002
                | $094310 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_094312
        .section .text.SceneEntities_094312, "ax", @progbits
SceneEntities_094312:                       | 1 registros de 14 B + terminador (16 B)
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0015,0x0013
                | $094312 type=1 subop=00 tmpl=$106F6C payload=0000000000150013
        .dc.w   0x0200                        | $094320 terminador type=2

        .globl  SceneScript_094322
        .section .text.SceneScript_094322, "ax", @progbits
SceneScript_094322:                         | bytecode VM de escena (438 B, 5 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $094322 op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $094328 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0xffff                 |   slot $10 <- bank $FFFF
        .dc.w   0x0011,0xffff                 |   slot $11 <- bank $FFFF
        .dc.w   0x0012,0xffff                 |   slot $12 <- bank $FFFF
        .dc.w   0x0013,0xffff                 |   slot $13 <- bank $FFFF
        .dc.w   0x0014,0xffff                 |   slot $14 <- bank $FFFF
        .dc.w   0x0015,0xffff                 |   slot $15 <- bank $FFFF
        .dc.w   0x0016,0xffff                 |   slot $16 <- bank $FFFF
        .dc.w   0x0017,0xffff                 |   slot $17 <- bank $FFFF
        .dc.w   0x0018,0xffff                 |   slot $18 <- bank $FFFF
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $0944C2 fin de pares
        .dc.w   0x000f,0x0000,0x0000,0x0000,0x0000,0x0000
                | $0944C4 op $0F set_limits: minX=0 minY=0 maxX=0 maxY=0 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0944D0 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0002
                | $0944D6 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_0944D8
        .section .text.SceneEntities_0944D8, "ax", @progbits
SceneEntities_0944D8:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0xff80,0x0015,0x0016
                | $0944D8 type=1 subop=00 tmpl=$1080E0 payload=0000ff8000150016
        .dc.w   0x0100,0x0010,0x8064,0x0000,0xff80,0x0015,0x0016
                | $0944E6 type=1 subop=00 tmpl=$108064 payload=0000ff8000150016
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0xff80,0x0015,0x001a
                | $0944F4 type=0 subop=00 tmpl=$107FE8 payload=0000ff800015001a
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0xff80,0x0020,0x0019
                | $094502 type=1 subop=00 tmpl=$106F6C payload=ffa0ff8000200019
        .dc.w   0x0250                        | $094510 terminador type=2

        .globl  SceneScript_094512
        .section .text.SceneScript_094512, "ax", @progbits
SceneScript_094512:                         | bytecode VM de escena (4420 B, 200 ops)
        .dc.w   0x0014
                | $094512 op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $094514 --- callback 68000 embebido (14 B) -> PC=$094522 ---
        move.b  #0x0, 0x10e39b.l               | $094514  move.b #$0, $10e39b.l
        lea     .L094522(pc), a0              | $09451C  a0 = nuevo PC
        rts                                    | $094520  rts
.L094522:
        .dc.w   0x0003,0x0000,0x0010
                | $094522 op $03 warp: camara=(0,16) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $094528 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x001c                 |   slot $10 <- bank $001C
        .dc.w   0x0011,0x001d                 |   slot $11 <- bank $001D
        .dc.w   0x0012,0x001e                 |   slot $12 <- bank $001E
        .dc.w   0x0013,0x001f                 |   slot $13 <- bank $001F
        .dc.w   0x0014,0x003d                 |   slot $14 <- bank $003D
        .dc.w   0x0015,0x006f                 |   slot $15 <- bank $006F
        .dc.w   0x0016,0x0079                 |   slot $16 <- bank $0079
        .dc.w   0x0017,0x007e                 |   slot $17 <- bank $007E
        .dc.w   0x0018,0x007f                 |   slot $18 <- bank $007F
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $0946C2 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $0946C4 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000f,0x0340,0x0010,0x0340,0x0010,0x0000
                | $0946D2 op $0F set_limits: minX=832 minY=16 maxX=832 maxY=16 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0946DE op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x0800
                | $0946E4 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x000c,0x000e,0x0d20,0x0021,0xeef4,0x0000,0x0060
                | $0946E8 op $00 bind_path: ent=$106F6C ruta=$48000C.. cont=$21EEF4 ancla=(0,96)
        .dc.w   0x0004,0x0010,0x6f6c,0x0c00
                | $0946FE op $04 spawn: tmpl=$106F6C a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $094706 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $094712 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0028,0x000c,0x000e,0x1aa0,0x0000,0x0000,0x0000,0x0000
                | $094718 op $00 bind_path: ent=$108064 ruta=$28000C.. cont=$000000 ancla=(0,0)
        .dc.w   0x0004,0x0010,0x8064,0x0c00
                | $09472E op $04 spawn: tmpl=$108064 a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0100
                | $094736 op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $094742 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094748 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09474A --- callback 68000 embebido (16 B) -> PC=$09475A ---
        cmpi.w  #0x320, 0x106f50.l             | $09474A  cmpi.w #$320, $106f50.l
        scs     d0                             | $094752  scs.b d0
        lea     .L09475a(pc), a1              | $094754  a1 = nuevo PC
        rts                                    | $094758  rts
.L09475a:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $09475A op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $094768 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09476A --- callback 68000 embebido (16 B) -> PC=$09477A ---
        cmpi.w  #0x340, 0x106f50.l             | $09476A  cmpi.w #$340, $106f50.l
        scs     d0                             | $094772  scs.b d0
        lea     .L09477a(pc), a1              | $094774  a1 = nuevo PC
        rts                                    | $094778  rts
.L09477a:
        .dc.w   0x0006
                | $09477A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09477C --- callback 68000 embebido (16 B) -> PC=$09478C ---
        cmpi.b  #0x0, 0x10a2cf.l               | $09477C  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $094784  seq.b d0
        lea     .L09478c(pc), a1              | $094786  a1 = nuevo PC
        rts                                    | $09478A  rts
.L09478c:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $09478C op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $09479A op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $0947A8 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $0947AE op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $0947B4 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $0947BA op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0000,0x0030
                | $0947C0 op $15 set_campos: camara=(0,48)
        .dc.w   0x0016,0x1000
                | $0947C6 op $16 set_progress: high_water/base=$1000
        .dc.w   0x000a                              | $0947CA op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0140                 |   slot $10 <- bank $0140
        .dc.w   0x0011,0x0141                 |   slot $11 <- bank $0141
        .dc.w   0x0012,0x0142                 |   slot $12 <- bank $0142
        .dc.w   0x0013,0x0143                 |   slot $13 <- bank $0143
        .dc.w   0x0014,0x0144                 |   slot $14 <- bank $0144
        .dc.w   0x0015,0x0145                 |   slot $15 <- bank $0145
        .dc.w   0x0016,0x0146                 |   slot $16 <- bank $0146
        .dc.w   0x0017,0x0147                 |   slot $17 <- bank $0147
        .dc.w   0x0018,0x0148                 |   slot $18 <- bank $0148
        .dc.w   0x0019,0x0149                 |   slot $19 <- bank $0149
        .dc.w   0x001a,0x014a                 |   slot $1A <- bank $014A
        .dc.w   0x001b,0x014b                 |   slot $1B <- bank $014B
        .dc.w   0x001c,0x014c                 |   slot $1C <- bank $014C
        .dc.w   0x001d,0x014d                 |   slot $1D <- bank $014D
        .dc.w   0x001e,0x014e                 |   slot $1E <- bank $014E
        .dc.w   0x001f,0x014f                 |   slot $1F <- bank $014F
        .dc.w   0x0020,0x0150                 |   slot $20 <- bank $0150
        .dc.w   0x0021,0x0151                 |   slot $21 <- bank $0151
        .dc.w   0x0022,0x0152                 |   slot $22 <- bank $0152
        .dc.w   0x0023,0x0153                 |   slot $23 <- bank $0153
        .dc.w   0x0024,0x0154                 |   slot $24 <- bank $0154
        .dc.w   0x0025,0x0155                 |   slot $25 <- bank $0155
        .dc.w   0x0026,0x0156                 |   slot $26 <- bank $0156
        .dc.w   0x0027,0x0157                 |   slot $27 <- bank $0157
        .dc.w   0x0028,0x0158                 |   slot $28 <- bank $0158
        .dc.w   0x0029,0x0159                 |   slot $29 <- bank $0159
        .dc.w   0x002a,0x015a                 |   slot $2A <- bank $015A
        .dc.w   0x002b,0x015b                 |   slot $2B <- bank $015B
        .dc.w   0x002c,0x015c                 |   slot $2C <- bank $015C
        .dc.w   0x002d,0x015d                 |   slot $2D <- bank $015D
        .dc.w   0x002e,0x015e                 |   slot $2E <- bank $015E
        .dc.w   0x002f,0x015f                 |   slot $2F <- bank $015F
        .dc.w   0x0030,0x0160                 |   slot $30 <- bank $0160
        .dc.w   0x0031,0x0161                 |   slot $31 <- bank $0161
        .dc.w   0x0032,0x0162                 |   slot $32 <- bank $0162
        .dc.w   0x0033,0x0163                 |   slot $33 <- bank $0163
        .dc.w   0x0034,0x0164                 |   slot $34 <- bank $0164
        .dc.w   0x0035,0x0165                 |   slot $35 <- bank $0165
        .dc.w   0x0036,0x0166                 |   slot $36 <- bank $0166
        .dc.w   0x0037,0x0167                 |   slot $37 <- bank $0167
        .dc.w   0x0038,0x0168                 |   slot $38 <- bank $0168
        .dc.w   0x0039,0x0169                 |   slot $39 <- bank $0169
        .dc.w   0x003a,0x016a                 |   slot $3A <- bank $016A
        .dc.w   0x003b,0x016b                 |   slot $3B <- bank $016B
        .dc.w   0x003c,0x016c                 |   slot $3C <- bank $016C
        .dc.w   0x003d,0x016d                 |   slot $3D <- bank $016D
        .dc.w   0x003e,0x016e                 |   slot $3E <- bank $016E
        .dc.w   0x003f,0x016f                 |   slot $3F <- bank $016F
        .dc.w   0x0040,0x0170                 |   slot $40 <- bank $0170
        .dc.w   0x0041,0x0171                 |   slot $41 <- bank $0171
        .dc.w   0x0042,0x0172                 |   slot $42 <- bank $0172
        .dc.w   0x0043,0x0173                 |   slot $43 <- bank $0173
        .dc.w   0x0044,0x0174                 |   slot $44 <- bank $0174
        .dc.w   0x0045,0x0175                 |   slot $45 <- bank $0175
        .dc.w   0x0046,0x0176                 |   slot $46 <- bank $0176
        .dc.w   0x0047,0x0177                 |   slot $47 <- bank $0177
        .dc.w   0x0048,0x0178                 |   slot $48 <- bank $0178
        .dc.w   0x0049,0x0179                 |   slot $49 <- bank $0179
        .dc.w   0x004a,0x017a                 |   slot $4A <- bank $017A
        .dc.w   0x004b,0x017b                 |   slot $4B <- bank $017B
        .dc.w   0x004c,0x017c                 |   slot $4C <- bank $017C
        .dc.w   0x004d,0x017d                 |   slot $4D <- bank $017D
        .dc.w   0x004e,0x017e                 |   slot $4E <- bank $017E
        .dc.w   0x004f,0x017f                 |   slot $4F <- bank $017F
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x013f                 |   slot $FF <- bank $013F
        .dc.w   0xffff                              | $094964 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $094966 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000f,0x0590,0x0030,0x0590,0x0030,0x0000
                | $094974 op $0F set_limits: minX=1424 minY=48 maxX=1424 maxY=48 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $094980 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x1800
                | $094986 op $0C set_6eac: $106EAC.b = 18
        .dc.w   0x0000,0x0010,0x6f6c,0x010e,0x0010,0x000c,0xe5fc,0x0021,0x5434,0x0000,0x0000
                | $09498A op $00 bind_path: ent=$106F6C ruta=$10E0010.. cont=$215434 ancla=(0,0)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0949A0 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x6f6c,0x1000
                | $0949AC op $04 spawn: tmpl=$106F6C a=10 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $0949B4 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0949BA op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0949BC --- callback 68000 embebido (16 B) -> PC=$0949CC ---
        cmpi.w  #0x580, 0x106f50.l             | $0949BC  cmpi.w #$580, $106f50.l
        scs     d0                             | $0949C4  scs.b d0
        lea     .L0949cc(pc), a1              | $0949C6  a1 = nuevo PC
        rts                                    | $0949CA  rts
.L0949cc:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $0949CC op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $0949DA op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0949DC --- callback 68000 embebido (16 B) -> PC=$0949EC ---
        cmpi.w  #0x590, 0x106f50.l             | $0949DC  cmpi.w #$590, $106f50.l
        scs     d0                             | $0949E4  scs.b d0
        lea     .L0949ec(pc), a1              | $0949E6  a1 = nuevo PC
        rts                                    | $0949EA  rts
.L0949ec:
        .dc.w   0x0006
                | $0949EC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0949EE --- callback 68000 embebido (16 B) -> PC=$0949FE ---
        cmpi.b  #0x0, 0x10a2cf.l               | $0949EE  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $0949F6  seq.b d0
        lea     .L0949fe(pc), a1              | $0949F8  a1 = nuevo PC
        rts                                    | $0949FC  rts
.L0949fe:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $0949FE op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $094A0C op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $094A1A op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $094A20 op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $094A26 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $094A2C op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0b90,0x0090
                | $094A32 op $15 set_campos: camara=(2960,144)
        .dc.w   0x0016,0x2b90
                | $094A38 op $16 set_progress: high_water/base=$2B90
        .dc.w   0x000a                              | $094A3C op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x00c0                 |   slot $10 <- bank $00C0
        .dc.w   0x0011,0x00c1                 |   slot $11 <- bank $00C1
        .dc.w   0x0012,0x00c2                 |   slot $12 <- bank $00C2
        .dc.w   0x0013,0x00c3                 |   slot $13 <- bank $00C3
        .dc.w   0x0014,0x00c4                 |   slot $14 <- bank $00C4
        .dc.w   0x0015,0x00c5                 |   slot $15 <- bank $00C5
        .dc.w   0x0016,0x00c6                 |   slot $16 <- bank $00C6
        .dc.w   0x0017,0x00c7                 |   slot $17 <- bank $00C7
        .dc.w   0x0018,0x00c8                 |   slot $18 <- bank $00C8
        .dc.w   0x0019,0x00c9                 |   slot $19 <- bank $00C9
        .dc.w   0x001a,0x00ca                 |   slot $1A <- bank $00CA
        .dc.w   0x001b,0x00cb                 |   slot $1B <- bank $00CB
        .dc.w   0x001c,0x00cc                 |   slot $1C <- bank $00CC
        .dc.w   0x001d,0x00cd                 |   slot $1D <- bank $00CD
        .dc.w   0x001e,0x00ce                 |   slot $1E <- bank $00CE
        .dc.w   0x001f,0x00cf                 |   slot $1F <- bank $00CF
        .dc.w   0x0020,0x00d0                 |   slot $20 <- bank $00D0
        .dc.w   0x0021,0x00d1                 |   slot $21 <- bank $00D1
        .dc.w   0x0022,0x00d2                 |   slot $22 <- bank $00D2
        .dc.w   0x0023,0x00d3                 |   slot $23 <- bank $00D3
        .dc.w   0x0024,0x00d4                 |   slot $24 <- bank $00D4
        .dc.w   0x0025,0x00d5                 |   slot $25 <- bank $00D5
        .dc.w   0x0026,0x00d6                 |   slot $26 <- bank $00D6
        .dc.w   0x0027,0x00d7                 |   slot $27 <- bank $00D7
        .dc.w   0x0028,0x00d8                 |   slot $28 <- bank $00D8
        .dc.w   0x0029,0x00d9                 |   slot $29 <- bank $00D9
        .dc.w   0x002a,0x00da                 |   slot $2A <- bank $00DA
        .dc.w   0x002b,0x00db                 |   slot $2B <- bank $00DB
        .dc.w   0x002c,0x00dc                 |   slot $2C <- bank $00DC
        .dc.w   0x002d,0x00dd                 |   slot $2D <- bank $00DD
        .dc.w   0x002e,0x00de                 |   slot $2E <- bank $00DE
        .dc.w   0x002f,0x00df                 |   slot $2F <- bank $00DF
        .dc.w   0x0030,0x00e0                 |   slot $30 <- bank $00E0
        .dc.w   0x0031,0x00e1                 |   slot $31 <- bank $00E1
        .dc.w   0x0032,0x00e2                 |   slot $32 <- bank $00E2
        .dc.w   0x0033,0x00e3                 |   slot $33 <- bank $00E3
        .dc.w   0x0034,0x00e4                 |   slot $34 <- bank $00E4
        .dc.w   0x0035,0x00e5                 |   slot $35 <- bank $00E5
        .dc.w   0x0036,0x00e6                 |   slot $36 <- bank $00E6
        .dc.w   0x0037,0x00e7                 |   slot $37 <- bank $00E7
        .dc.w   0x0038,0x00e8                 |   slot $38 <- bank $00E8
        .dc.w   0x0039,0x00e9                 |   slot $39 <- bank $00E9
        .dc.w   0x003a,0x00ea                 |   slot $3A <- bank $00EA
        .dc.w   0x003b,0x00eb                 |   slot $3B <- bank $00EB
        .dc.w   0x003c,0x00ec                 |   slot $3C <- bank $00EC
        .dc.w   0x003d,0x00ed                 |   slot $3D <- bank $00ED
        .dc.w   0x003e,0x00ee                 |   slot $3E <- bank $00EE
        .dc.w   0x003f,0x00ef                 |   slot $3F <- bank $00EF
        .dc.w   0x0040,0x00f0                 |   slot $40 <- bank $00F0
        .dc.w   0x0041,0x00f1                 |   slot $41 <- bank $00F1
        .dc.w   0x0042,0x00f2                 |   slot $42 <- bank $00F2
        .dc.w   0x0043,0x00f3                 |   slot $43 <- bank $00F3
        .dc.w   0x0044,0x00f4                 |   slot $44 <- bank $00F4
        .dc.w   0x0045,0x00f5                 |   slot $45 <- bank $00F5
        .dc.w   0x0046,0x00f6                 |   slot $46 <- bank $00F6
        .dc.w   0x0047,0x00f7                 |   slot $47 <- bank $00F7
        .dc.w   0x0048,0x00f8                 |   slot $48 <- bank $00F8
        .dc.w   0x0049,0x00f9                 |   slot $49 <- bank $00F9
        .dc.w   0x004a,0x00fa                 |   slot $4A <- bank $00FA
        .dc.w   0x004b,0x00fb                 |   slot $4B <- bank $00FB
        .dc.w   0x004c,0x00fc                 |   slot $4C <- bank $00FC
        .dc.w   0x004d,0x00fd                 |   slot $4D <- bank $00FD
        .dc.w   0x004e,0x00fe                 |   slot $4E <- bank $00FE
        .dc.w   0x004f,0x00ff                 |   slot $4F <- bank $00FF
        .dc.w   0x0050,0x0100                 |   slot $50 <- bank $0100
        .dc.w   0x0051,0x0101                 |   slot $51 <- bank $0101
        .dc.w   0x0052,0x0102                 |   slot $52 <- bank $0102
        .dc.w   0x0053,0x0103                 |   slot $53 <- bank $0103
        .dc.w   0x0054,0x0104                 |   slot $54 <- bank $0104
        .dc.w   0x0055,0x0105                 |   slot $55 <- bank $0105
        .dc.w   0x0056,0x0106                 |   slot $56 <- bank $0106
        .dc.w   0x0057,0x0107                 |   slot $57 <- bank $0107
        .dc.w   0x0058,0x0108                 |   slot $58 <- bank $0108
        .dc.w   0x0059,0x0109                 |   slot $59 <- bank $0109
        .dc.w   0x005a,0x010a                 |   slot $5A <- bank $010A
        .dc.w   0x005b,0x010b                 |   slot $5B <- bank $010B
        .dc.w   0x005c,0x010c                 |   slot $5C <- bank $010C
        .dc.w   0x005d,0x010d                 |   slot $5D <- bank $010D
        .dc.w   0x005e,0x010e                 |   slot $5E <- bank $010E
        .dc.w   0x005f,0x010f                 |   slot $5F <- bank $010F
        .dc.w   0x0060,0x0110                 |   slot $60 <- bank $0110
        .dc.w   0x0061,0x0111                 |   slot $61 <- bank $0111
        .dc.w   0x0062,0x01d0                 |   slot $62 <- bank $01D0
        .dc.w   0x0063,0x01d1                 |   slot $63 <- bank $01D1
        .dc.w   0x0064,0x01d2                 |   slot $64 <- bank $01D2
        .dc.w   0x0065,0x01d3                 |   slot $65 <- bank $01D3
        .dc.w   0x0066,0x01d4                 |   slot $66 <- bank $01D4
        .dc.w   0x0067,0x01d5                 |   slot $67 <- bank $01D5
        .dc.w   0x0068,0x01d6                 |   slot $68 <- bank $01D6
        .dc.w   0x0069,0x01d7                 |   slot $69 <- bank $01D7
        .dc.w   0x006a,0x01d8                 |   slot $6A <- bank $01D8
        .dc.w   0x006b,0x01d9                 |   slot $6B <- bank $01D9
        .dc.w   0x006c,0x01da                 |   slot $6C <- bank $01DA
        .dc.w   0x006d,0x01db                 |   slot $6D <- bank $01DB
        .dc.w   0x006e,0x01dc                 |   slot $6E <- bank $01DC
        .dc.w   0x006f,0x01dd                 |   slot $6F <- bank $01DD
        .dc.w   0x0070,0x01de                 |   slot $70 <- bank $01DE
        .dc.w   0x0071,0x01df                 |   slot $71 <- bank $01DF
        .dc.w   0x0072,0x017d                 |   slot $72 <- bank $017D
        .dc.w   0x0073,0x017e                 |   slot $73 <- bank $017E
        .dc.w   0x0074,0x017f                 |   slot $74 <- bank $017F
        .dc.w   0x00ff,0x0109                 |   slot $FF <- bank $0109
        .dc.w   0xffff                              | $094BD6 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $094BD8 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000c,0x0800
                | $094BE6 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x000f,0x0f00,0x0030,0x0f00,0x0090,0x0000
                | $094BEA op $0F set_limits: minX=3840 minY=48 maxX=3840 maxY=144 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $094BF6 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x0078,0x0012,0x000a,0x18a8,0x0020,0x0000,0x0000,0x0080
                | $094BFC op $00 bind_path: ent=$106F6C ruta=$780012.. cont=$200000 ancla=(0,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1ee0
                | $094C12 op $11 set_trig: ent=$106F6C tabla=$091EE0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $094C1C op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $094C28 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094C2E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094C30 --- callback 68000 embebido (16 B) -> PC=$094C40 ---
        cmpi.w  #0x2e0, 0x106f50.l             | $094C30  cmpi.w #$2e0, $106f50.l
        scs     d0                             | $094C38  scs.b d0
        lea     .L094c40(pc), a1              | $094C3A  a1 = nuevo PC
        rts                                    | $094C3E  rts
.L094c40:
        .dc.w   0x0005,0x0010,0x80e0
                | $094C40 op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $094C46 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094C48 --- callback 68000 embebido (16 B) -> PC=$094C58 ---
        cmpi.w  #0x5e0, 0x106f50.l             | $094C48  cmpi.w #$5e0, $106f50.l
        scs     d0                             | $094C50  scs.b d0
        lea     .L094c58(pc), a1              | $094C52  a1 = nuevo PC
        rts                                    | $094C56  rts
.L094c58:
        .dc.w   0x0000,0x0010,0x6f6c,0x005c,0x0012,0x000a,0x3a68,0x0020,0x21c0,0x0780,0x0080
                | $094C58 op $00 bind_path: ent=$106F6C ruta=$5C0012.. cont=$2021C0 ancla=(1920,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1f0a
                | $094C6E op $11 set_trig: ent=$106F6C tabla=$091F0A  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $094C78 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $094C84 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094C8A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094C8C --- callback 68000 embebido (16 B) -> PC=$094C9C ---
        cmpi.w  #0x600, 0x106f50.l             | $094C8C  cmpi.w #$600, $106f50.l
        scs     d0                             | $094C94  scs.b d0
        lea     .L094c9c(pc), a1              | $094C96  a1 = nuevo PC
        rts                                    | $094C9A  rts
.L094c9c:
        .dc.w   0x0000,0x0010,0x7fe8,0x0004,0x0004,0x000a,0x0000,0x0000,0x0000,0x0740,0x0160
                | $094C9C op $00 bind_path: ent=$107FE8 ruta=$040004.. cont=$000000 ancla=(1856,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $094CB2 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $094CBA op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $094CC6 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094CCC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094CCE --- callback 68000 embebido (16 B) -> PC=$094CDE ---
        cmpi.w  #0x640, 0x106f50.l             | $094CCE  cmpi.w #$640, $106f50.l
        scs     d0                             | $094CD6  scs.b d0
        lea     .L094cde(pc), a1              | $094CD8  a1 = nuevo PC
        rts                                    | $094CDC  rts
.L094cde:
        .dc.w   0x0000,0x0010,0x7fe8,0x0036,0x0004,0x000a,0x0040,0x0000,0x0000,0x0780,0x0160
                | $094CDE op $00 bind_path: ent=$107FE8 ruta=$360004.. cont=$000000 ancla=(1920,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $094CF4 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0007,0x0010,0x7fe8
                | $094CFC op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $094D02 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094D04 --- callback 68000 embebido (16 B) -> PC=$094D14 ---
        cmpi.w  #0x720, 0x106f50.l             | $094D04  cmpi.w #$720, $106f50.l
        scs     d0                             | $094D0C  scs.b d0
        lea     .L094d14(pc), a1              | $094D0E  a1 = nuevo PC
        rts                                    | $094D12  rts
.L094d14:
        .dc.w   0x0000,0x0010,0x8064,0x0060,0x0012,0x000a,0x66a8,0x0000,0x0000,0x0860,0x0030
                | $094D14 op $00 bind_path: ent=$108064 ruta=$600012.. cont=$000000 ancla=(2144,48)
        .dc.w   0x0004,0x0010,0x8064,0x1200
                | $094D2A op $04 spawn: tmpl=$108064 a=12 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x00c0,0x0100
                | $094D32 op $01 set_fields: ent=$108064 +72=03 +74=$00C0 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $094D3E op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000c,0x0000
                | $094D44 op $0C set_6eac: $106EAC.b = 00
        .dc.w   0x0006
                | $094D48 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094D4A --- callback 68000 embebido (16 B) -> PC=$094D5A ---
        cmpi.w  #0x9a0, 0x106f50.l             | $094D4A  cmpi.w #$9a0, $106f50.l
        scs     d0                             | $094D52  scs.b d0
        lea     .L094d5a(pc), a1              | $094D54  a1 = nuevo PC
        rts                                    | $094D58  rts
.L094d5a:
        .dc.w   0x0000,0x0010,0x7fe8,0x0008,0x000b,0x000a,0x10f8,0x0000,0x0000,0x0ae0,0x00f0
                | $094D5A op $00 bind_path: ent=$107FE8 ruta=$08000B.. cont=$000000 ancla=(2784,240)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1ed6
                | $094D70 op $11 set_trig: ent=$107FE8 tabla=$091ED6  (-> $12(ent))
        .dc.w   0x0009,0x0010,0x7fe8
                | $094D7A op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094D80 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094D82 --- callback 68000 embebido (16 B) -> PC=$094D92 ---
        cmpi.w  #0x9f0, 0x106f50.l             | $094D82  cmpi.w #$9f0, $106f50.l
        scs     d0                             | $094D8A  scs.b d0
        lea     .L094d92(pc), a1              | $094D8C  a1 = nuevo PC
        rts                                    | $094D90  rts
.L094d92:
        .dc.w   0x0000,0x0010,0x80e0,0x001e,0x0001,0x000a,0x81a8,0x0000,0x0000,0x0b30,0x0150
                | $094D92 op $00 bind_path: ent=$1080E0 ruta=$1E0001.. cont=$000000 ancla=(2864,336)
        .dc.w   0x0004,0x0010,0x80e0,0x0100
                | $094DA8 op $04 spawn: tmpl=$1080E0 a=01 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0100,0x0100
                | $094DB0 op $01 set_fields: ent=$1080E0 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $094DBC op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094DC2 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094DC4 --- callback 68000 embebido (16 B) -> PC=$094DD4 ---
        cmpi.w  #0xa20, 0x106f50.l             | $094DC4  cmpi.w #$a20, $106f50.l
        scs     d0                             | $094DCC  scs.b d0
        lea     .L094dd4(pc), a1              | $094DCE  a1 = nuevo PC
        rts                                    | $094DD2  rts
.L094dd4:
        .dc.w   0x0000,0x0010,0x7fe8,0x0022,0x0004,0x000a,0x1258,0x0000,0x0000,0x0b60,0x0160
                | $094DD4 op $00 bind_path: ent=$107FE8 ruta=$220004.. cont=$000000 ancla=(2912,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $094DEA op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $094DF2 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094DF8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094DFA --- callback 68000 embebido (16 B) -> PC=$094E0A ---
        cmpi.w  #0xb00, 0x106f50.l             | $094DFA  cmpi.w #$b00, $106f50.l
        scs     d0                             | $094E02  scs.b d0
        lea     .L094e0a(pc), a1              | $094E04  a1 = nuevo PC
        rts                                    | $094E08  rts
.L094e0a:
        .dc.w   0x0000,0x0010,0x80e0,0x0012,0x0013,0x000a,0x8220,0x0000,0x0000,0x0c40,0x0030
                | $094E0A op $00 bind_path: ent=$1080E0 ruta=$120013.. cont=$000000 ancla=(3136,48)
        .dc.w   0x0004,0x0010,0x80e0,0x1300
                | $094E20 op $04 spawn: tmpl=$1080E0 a=13 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0080,0x0100
                | $094E28 op $01 set_fields: ent=$1080E0 +72=03 +74=$0080 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $094E34 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094E3A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094E3C --- callback 68000 embebido (16 B) -> PC=$094E4C ---
        cmpi.w  #0xb40, 0x106f50.l             | $094E3C  cmpi.w #$b40, $106f50.l
        scs     d0                             | $094E44  scs.b d0
        lea     .L094e4c(pc), a1              | $094E46  a1 = nuevo PC
        rts                                    | $094E4A  rts
.L094e4c:
        .dc.w   0x000d,0x0005,0x2756,0x00ff,0x00f1,0xffff,0x0000
                | $094E4C op $0D call_args: fn=$052756 args=00ff00f1ffff0000
        .dc.w   0x0006
                | $094E5A op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094E5C --- callback 68000 embebido (16 B) -> PC=$094E6C ---
        cmpi.w  #0xba0, 0x106f50.l             | $094E5C  cmpi.w #$ba0, $106f50.l
        scs     d0                             | $094E64  scs.b d0
        lea     .L094e6c(pc), a1              | $094E66  a1 = nuevo PC
        rts                                    | $094E6A  rts
.L094e6c:
        .dc.w   0x0000,0x0010,0x6f6c,0x0018,0x0017,0x000a,0x5448,0x0020,0x3ba0,0x0d40,0x0000
                | $094E6C op $00 bind_path: ent=$106F6C ruta=$180017.. cont=$203BA0 ancla=(3392,0)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1f58
                | $094E82 op $11 set_trig: ent=$106F6C tabla=$091F58  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $094E8C op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $094E98 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094E9E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094EA0 --- callback 68000 embebido (16 B) -> PC=$094EB0 ---
        cmpi.w  #0xba0, 0x106f50.l             | $094EA0  cmpi.w #$ba0, $106f50.l
        scs     d0                             | $094EA8  scs.b d0
        lea     .L094eb0(pc), a1              | $094EAA  a1 = nuevo PC
        rts                                    | $094EAE  rts
.L094eb0:
        .dc.w   0x0013,0x10b3
                | $094EB0 op $13 clamp: d0=10B3  (ClampD0ToRange)
        .dc.w   0x0006
                | $094EB4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094EB6 --- callback 68000 embebido (16 B) -> PC=$094EC6 ---
        cmpi.w  #0xbc0, 0x106f50.l             | $094EB6  cmpi.w #$bc0, $106f50.l
        scs     d0                             | $094EBE  scs.b d0
        lea     .L094ec6(pc), a1              | $094EC0  a1 = nuevo PC
        rts                                    | $094EC4  rts
.L094ec6:
        .dc.w   0x0000,0x0010,0x7fe8,0x001c,0x0018,0x000a,0x03a0,0x0000,0x0000,0x0d00,0x0020
                | $094EC6 op $00 bind_path: ent=$107FE8 ruta=$1C0018.. cont=$000000 ancla=(3328,32)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1f8a
                | $094EDC op $11 set_trig: ent=$107FE8 tabla=$091F8A  (-> $12(ent))
        .dc.w   0x0009,0x0010,0x7fe8
                | $094EE6 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094EEC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094EEE --- callback 68000 embebido (16 B) -> PC=$094EFE ---
        cmpi.w  #0xce0, 0x106f50.l             | $094EEE  cmpi.w #$ce0, $106f50.l
        scs     d0                             | $094EF6  scs.b d0
        lea     .L094efe(pc), a1              | $094EF8  a1 = nuevo PC
        rts                                    | $094EFC  rts
.L094efe:
        .dc.w   0x0014
                | $094EFE op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $094F00 --- callback 68000 embebido (16 B) -> PC=$094F10 ---
        move.l  #0xffff0000, 0x106f64.l        | $094F00  move.l #$ffff0000, $106f64.l
        lea     .L094f10(pc), a0              | $094F0A  a0 = nuevo PC
        rts                                    | $094F0E  rts
.L094f10:
        .dc.w   0x0006
                | $094F10 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094F12 --- callback 68000 embebido (16 B) -> PC=$094F22 ---
        cmpi.w  #0xd20, 0x106f50.l             | $094F12  cmpi.w #$d20, $106f50.l
        scs     d0                             | $094F1A  scs.b d0
        lea     .L094f22(pc), a1              | $094F1C  a1 = nuevo PC
        rts                                    | $094F20  rts
.L094f22:
        .dc.w   0x0000,0x0010,0x6f6c,0x0018,0x0012,0x000a,0x5fe8,0x0020,0x4440,0x0ec0,0x0010
                | $094F22 op $00 bind_path: ent=$106F6C ruta=$180012.. cont=$204440 ancla=(3776,16)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1fcc
                | $094F38 op $11 set_trig: ent=$106F6C tabla=$091FCC  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $094F42 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $094F4E op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094F54 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094F56 --- callback 68000 embebido (16 B) -> PC=$094F66 ---
        cmpi.w  #0xd80, 0x106f50.l             | $094F56  cmpi.w #$d80, $106f50.l
        scs     d0                             | $094F5E  scs.b d0
        lea     .L094f66(pc), a1              | $094F60  a1 = nuevo PC
        rts                                    | $094F64  rts
.L094f66:
        .dc.w   0x0000,0x0010,0x7fe8,0x001a,0x0007,0x000a,0x0e20,0x0000,0x0000,0x0ec0,0x00e0
                | $094F66 op $00 bind_path: ent=$107FE8 ruta=$1A0007.. cont=$000000 ancla=(3776,224)
        .dc.w   0x0004,0x0010,0x7fe8,0x0700
                | $094F7C op $04 spawn: tmpl=$107FE8 a=07 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $094F84 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0010,0x000c,0x000a,0x5ce8,0x0000,0x0000,0x0ec0,0x0020
                | $094F8A op $00 bind_path: ent=$108064 ruta=$10000C.. cont=$000000 ancla=(3776,32)
        .dc.w   0x0011,0x0010,0x8064,0x0009,0x1fbc
                | $094FA0 op $11 set_trig: ent=$108064 tabla=$091FBC  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0100,0x0100
                | $094FAA op $01 set_fields: ent=$108064 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $094FB6 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $094FBC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094FBE --- callback 68000 embebido (16 B) -> PC=$094FCE ---
        cmpi.w  #0xdc0, 0x106f50.l             | $094FBE  cmpi.w #$dc0, $106f50.l
        scs     d0                             | $094FC6  scs.b d0
        lea     .L094fce(pc), a1              | $094FC8  a1 = nuevo PC
        rts                                    | $094FCC  rts
.L094fce:
        .dc.w   0x0014
                | $094FCE op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $094FD0 --- callback 68000 embebido (16 B) -> PC=$094FE0 ---
        move.l  #0x0, 0x106f64.l               | $094FD0  move.l #$0, $106f64.l
        lea     .L094fe0(pc), a0              | $094FDA  a0 = nuevo PC
        rts                                    | $094FDE  rts
.L094fe0:
        .dc.w   0x0006
                | $094FE0 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094FE2 --- callback 68000 embebido (16 B) -> PC=$094FF2 ---
        cmpi.w  #0xdc0, 0x106f50.l             | $094FE2  cmpi.w #$dc0, $106f50.l
        scs     d0                             | $094FEA  scs.b d0
        lea     .L094ff2(pc), a1              | $094FEC  a1 = nuevo PC
        rts                                    | $094FF0  rts
.L094ff2:
        .dc.w   0x0013,0x10b4
                | $094FF2 op $13 clamp: d0=10B4  (ClampD0ToRange)
        .dc.w   0x0006
                | $094FF6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $094FF8 --- callback 68000 embebido (16 B) -> PC=$095008 ---
        cmpi.w  #0xe10, 0x106f50.l             | $094FF8  cmpi.w #$e10, $106f50.l
        scs     d0                             | $095000  scs.b d0
        lea     .L095008(pc), a1              | $095002  a1 = nuevo PC
        rts                                    | $095006  rts
.L095008:
        .dc.w   0x0005,0x0010,0x80e0
                | $095008 op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $09500E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095010 --- callback 68000 embebido (16 B) -> PC=$095020 ---
        cmpi.w  #0xe80, 0x106f50.l             | $095010  cmpi.w #$e80, $106f50.l
        scs     d0                             | $095018  scs.b d0
        lea     .L095020(pc), a1              | $09501A  a1 = nuevo PC
        rts                                    | $09501E  rts
.L095020:
        .dc.w   0x0000,0x0010,0x7fe8,0x001a,0x0007,0x000a,0x0e20,0x0000,0x0000,0x0fc0,0x00e0
                | $095020 op $00 bind_path: ent=$107FE8 ruta=$1A0007.. cont=$000000 ancla=(4032,224)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $095036 op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x7fe8,0x0000
                | $095042 op $04 spawn: tmpl=$107FE8 a=00 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $09504A op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095050 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095052 --- callback 68000 embebido (16 B) -> PC=$095062 ---
        cmpi.w  #0xef0, 0x106f50.l             | $095052  cmpi.w #$ef0, $106f50.l
        scs     d0                             | $09505A  scs.b d0
        lea     .L095062(pc), a1              | $09505C  a1 = nuevo PC
        rts                                    | $095060  rts
.L095062:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $095062 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $095070 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095072 --- callback 68000 embebido (16 B) -> PC=$095082 ---
        cmpi.w  #0xf00, 0x106f50.l             | $095072  cmpi.w #$f00, $106f50.l
        scs     d0                             | $09507A  scs.b d0
        lea     .L095082(pc), a1              | $09507C  a1 = nuevo PC
        rts                                    | $095080  rts
.L095082:
        .dc.w   0x0006
                | $095082 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095084 --- callback 68000 embebido (16 B) -> PC=$095094 ---
        cmpi.b  #0x0, 0x10a2cf.l               | $095084  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $09508C  seq.b d0
        lea     .L095094(pc), a1              | $09508E  a1 = nuevo PC
        rts                                    | $095092  rts
.L095094:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $095094 op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $0950A2 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $0950B0 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $0950B6 op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $0950BC op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $0950C2 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0000,0x0110
                | $0950C8 op $15 set_campos: camara=(0,272)
        .dc.w   0x0016,0x3000
                | $0950CE op $16 set_progress: high_water/base=$3000
        .dc.w   0x000a                              | $0950D2 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0003                 |   slot $10 <- bank $0003
        .dc.w   0x0011,0x0004                 |   slot $11 <- bank $0004
        .dc.w   0x0012,0x0005                 |   slot $12 <- bank $0005
        .dc.w   0x0013,0x0006                 |   slot $13 <- bank $0006
        .dc.w   0x0014,0x0007                 |   slot $14 <- bank $0007
        .dc.w   0x0015,0x0008                 |   slot $15 <- bank $0008
        .dc.w   0x0016,0x0009                 |   slot $16 <- bank $0009
        .dc.w   0x0017,0x000a                 |   slot $17 <- bank $000A
        .dc.w   0x0018,0x000b                 |   slot $18 <- bank $000B
        .dc.w   0x0019,0x000c                 |   slot $19 <- bank $000C
        .dc.w   0x001a,0x000d                 |   slot $1A <- bank $000D
        .dc.w   0x001b,0x000e                 |   slot $1B <- bank $000E
        .dc.w   0x001c,0x000f                 |   slot $1C <- bank $000F
        .dc.w   0x001d,0x0010                 |   slot $1D <- bank $0010
        .dc.w   0x001e,0x0011                 |   slot $1E <- bank $0011
        .dc.w   0x001f,0x0012                 |   slot $1F <- bank $0012
        .dc.w   0x0020,0x0014                 |   slot $20 <- bank $0014
        .dc.w   0x0021,0x0013                 |   slot $21 <- bank $0013
        .dc.w   0x0022,0x0015                 |   slot $22 <- bank $0015
        .dc.w   0x0023,0x0016                 |   slot $23 <- bank $0016
        .dc.w   0x0024,0x0017                 |   slot $24 <- bank $0017
        .dc.w   0x0025,0x0018                 |   slot $25 <- bank $0018
        .dc.w   0x0026,0x0019                 |   slot $26 <- bank $0019
        .dc.w   0x0027,0x001a                 |   slot $27 <- bank $001A
        .dc.w   0x0028,0x001b                 |   slot $28 <- bank $001B
        .dc.w   0x0029,0x001c                 |   slot $29 <- bank $001C
        .dc.w   0x002a,0x001d                 |   slot $2A <- bank $001D
        .dc.w   0x002b,0x001e                 |   slot $2B <- bank $001E
        .dc.w   0x002c,0x001f                 |   slot $2C <- bank $001F
        .dc.w   0x002d,0x0020                 |   slot $2D <- bank $0020
        .dc.w   0x002e,0x0021                 |   slot $2E <- bank $0021
        .dc.w   0x002f,0x0022                 |   slot $2F <- bank $0022
        .dc.w   0x0030,0x0023                 |   slot $30 <- bank $0023
        .dc.w   0x0031,0x0024                 |   slot $31 <- bank $0024
        .dc.w   0x0032,0x0025                 |   slot $32 <- bank $0025
        .dc.w   0x0033,0x0026                 |   slot $33 <- bank $0026
        .dc.w   0x0034,0x0027                 |   slot $34 <- bank $0027
        .dc.w   0x0035,0x0028                 |   slot $35 <- bank $0028
        .dc.w   0x0036,0x0029                 |   slot $36 <- bank $0029
        .dc.w   0x0037,0x002a                 |   slot $37 <- bank $002A
        .dc.w   0x0038,0x002b                 |   slot $38 <- bank $002B
        .dc.w   0x0039,0x002c                 |   slot $39 <- bank $002C
        .dc.w   0x003a,0x002d                 |   slot $3A <- bank $002D
        .dc.w   0x003b,0x002e                 |   slot $3B <- bank $002E
        .dc.w   0x003c,0x002f                 |   slot $3C <- bank $002F
        .dc.w   0x003d,0x0030                 |   slot $3D <- bank $0030
        .dc.w   0x003e,0x0031                 |   slot $3E <- bank $0031
        .dc.w   0x003f,0x0032                 |   slot $3F <- bank $0032
        .dc.w   0x0040,0x0033                 |   slot $40 <- bank $0033
        .dc.w   0x0041,0x0034                 |   slot $41 <- bank $0034
        .dc.w   0x0042,0x0035                 |   slot $42 <- bank $0035
        .dc.w   0x0043,0x0036                 |   slot $43 <- bank $0036
        .dc.w   0x0044,0x0037                 |   slot $44 <- bank $0037
        .dc.w   0x0045,0x0038                 |   slot $45 <- bank $0038
        .dc.w   0x0046,0x0039                 |   slot $46 <- bank $0039
        .dc.w   0x0047,0x003a                 |   slot $47 <- bank $003A
        .dc.w   0x0048,0x003b                 |   slot $48 <- bank $003B
        .dc.w   0x0049,0x003c                 |   slot $49 <- bank $003C
        .dc.w   0x004a,0x003d                 |   slot $4A <- bank $003D
        .dc.w   0x004b,0x003e                 |   slot $4B <- bank $003E
        .dc.w   0x004c,0x003f                 |   slot $4C <- bank $003F
        .dc.w   0x004d,0x00ba                 |   slot $4D <- bank $00BA
        .dc.w   0x004e,0x00bb                 |   slot $4E <- bank $00BB
        .dc.w   0x004f,0x00bc                 |   slot $4F <- bank $00BC
        .dc.w   0x0050,0x01e0                 |   slot $50 <- bank $01E0
        .dc.w   0x0051,0x01e1                 |   slot $51 <- bank $01E1
        .dc.w   0x0052,0x01e2                 |   slot $52 <- bank $01E2
        .dc.w   0x0053,0x01e3                 |   slot $53 <- bank $01E3
        .dc.w   0x0054,0x01e4                 |   slot $54 <- bank $01E4
        .dc.w   0x0055,0x01e5                 |   slot $55 <- bank $01E5
        .dc.w   0x0056,0x01e6                 |   slot $56 <- bank $01E6
        .dc.w   0x0057,0x01e7                 |   slot $57 <- bank $01E7
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0002                 |   slot $FF <- bank $0002
        .dc.w   0xffff                              | $09526C fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $09526E op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000c,0x0800
                | $09527C op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x000f,0x0500,0x0110,0x0500,0x0110,0x0000
                | $095280 op $0F set_limits: minX=1280 minY=272 maxX=1280 maxY=272 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $09528C op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x00c8,0x0010,0x000a,0xbe50,0x0020,0x4b00,0x0000,0x0100
                | $095292 op $00 bind_path: ent=$106F6C ruta=$C80010.. cont=$204B00 ancla=(0,256)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x2498
                | $0952A8 op $11 set_trig: ent=$106F6C tabla=$092498  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0952B2 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $0952BE op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0060,0x000d,0x000b,0x1350,0x0000,0x0000,0x0000,0x0100
                | $0952C4 op $00 bind_path: ent=$108064 ruta=$60000D.. cont=$000000 ancla=(0,256)
        .dc.w   0x0011,0x0010,0x8064,0x0009,0x24f0
                | $0952DA op $11 set_trig: ent=$108064 tabla=$0924F0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0080,0x0100
                | $0952E4 op $01 set_fields: ent=$108064 +72=03 +74=$0080 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $0952F0 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000d,0x0005,0x2756,0x0021,0x0014,0xffff,0x0000
                | $0952F6 op $0D call_args: fn=$052756 args=00210014ffff0000
        .dc.w   0x0006
                | $095304 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095306 --- callback 68000 embebido (16 B) -> PC=$095316 ---
        cmpi.w  #0x2b0, 0x106f50.l             | $095306  cmpi.w #$2b0, $106f50.l
        scs     d0                             | $09530E  scs.b d0
        lea     .L095316(pc), a1              | $095310  a1 = nuevo PC
        rts                                    | $095314  rts
.L095316:
        .dc.w   0x0000,0x0010,0x80e0,0x001c,0x0005,0x000b,0x34d0,0x0000,0x0000,0x03f0,0x0100
                | $095316 op $00 bind_path: ent=$1080E0 ruta=$1C0005.. cont=$000000 ancla=(1008,256)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0040,0x0100
                | $09532C op $01 set_fields: ent=$1080E0 +72=03 +74=$0040 +76=$0100
        .dc.w   0x0004,0x0010,0x80e0,0x0500
                | $095338 op $04 spawn: tmpl=$1080E0 a=05 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $095340 op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095346 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095348 --- callback 68000 embebido (16 B) -> PC=$095358 ---
        cmpi.w  #0x480, 0x106f50.l             | $095348  cmpi.w #$480, $106f50.l
        scs     d0                             | $095350  scs.b d0
        lea     .L095358(pc), a1              | $095352  a1 = nuevo PC
        rts                                    | $095356  rts
.L095358:
        .dc.w   0x000d,0x0005,0x2756,0x0021,0x0014,0xffff,0x0000
                | $095358 op $0D call_args: fn=$052756 args=00210014ffff0000
        .dc.w   0x0006
                | $095366 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095368 --- callback 68000 embebido (16 B) -> PC=$095378 ---
        cmpi.w  #0x4f0, 0x106f50.l             | $095368  cmpi.w #$4f0, $106f50.l
        scs     d0                             | $095370  scs.b d0
        lea     .L095378(pc), a1              | $095372  a1 = nuevo PC
        rts                                    | $095376  rts
.L095378:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $095378 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $095386 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095388 --- callback 68000 embebido (16 B) -> PC=$095398 ---
        cmpi.w  #0x500, 0x106f50.l             | $095388  cmpi.w #$500, $106f50.l
        scs     d0                             | $095390  scs.b d0
        lea     .L095398(pc), a1              | $095392  a1 = nuevo PC
        rts                                    | $095396  rts
.L095398:
        .dc.w   0x0006
                | $095398 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $09539A --- callback 68000 embebido (16 B) -> PC=$0953AA ---
        cmpi.b  #0x0, 0x10a2cf.l               | $09539A  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $0953A2  seq.b d0
        lea     .L0953aa(pc), a1              | $0953A4  a1 = nuevo PC
        rts                                    | $0953A8  rts
.L0953aa:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $0953AA op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $0953B8 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $0953C6 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $0953CC op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $0953D2 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $0953D8 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0000,0x0110
                | $0953DE op $15 set_campos: camara=(0,272)
        .dc.w   0x0016,0x4000
                | $0953E4 op $16 set_progress: high_water/base=$4000
        .dc.w   0x000a                              | $0953E8 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x001c                 |   slot $10 <- bank $001C
        .dc.w   0x0011,0x001d                 |   slot $11 <- bank $001D
        .dc.w   0x0012,0x001e                 |   slot $12 <- bank $001E
        .dc.w   0x0013,0x001f                 |   slot $13 <- bank $001F
        .dc.w   0x0014,0x003d                 |   slot $14 <- bank $003D
        .dc.w   0x0015,0x006f                 |   slot $15 <- bank $006F
        .dc.w   0x0016,0x0079                 |   slot $16 <- bank $0079
        .dc.w   0x0017,0x007e                 |   slot $17 <- bank $007E
        .dc.w   0x0018,0x007f                 |   slot $18 <- bank $007F
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $095582 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $095584 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000d,0x0005,0x2756,0x0011,0x001c,0xffff,0x0000
                | $095592 op $0D call_args: fn=$052756 args=0011001cffff0000
        .dc.w   0x000d,0x0005,0x2756,0x0012,0x003d,0xffff,0x0000
                | $0955A0 op $0D call_args: fn=$052756 args=0012003dffff0000
        .dc.w   0x000d,0x0005,0x2756,0x0013,0x006f,0xffff,0x0000
                | $0955AE op $0D call_args: fn=$052756 args=0013006fffff0000
        .dc.w   0x000f,0x0340,0x0078,0x0340,0x0110,0x0000
                | $0955BC op $0F set_limits: minX=832 minY=120 maxX=832 maxY=272 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0955C8 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x0800
                | $0955CE op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x000c,0x000e,0x0d20,0x0021,0xeef4,0x0000,0x0160
                | $0955D2 op $00 bind_path: ent=$106F6C ruta=$48000C.. cont=$21EEF4 ancla=(0,352)
        .dc.w   0x0004,0x0010,0x6f6c,0x0c00
                | $0955E8 op $04 spawn: tmpl=$106F6C a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0955F0 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $0955FC op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0020,0x000f,0x000e,0x2220,0x0000,0x0000,0x0000,0x0100
                | $095602 op $00 bind_path: ent=$108064 ruta=$20000F.. cont=$000000 ancla=(0,256)
        .dc.w   0x0004,0x0010,0x8064,0x0f00
                | $095618 op $04 spawn: tmpl=$108064 a=0F b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0028
                | $095620 op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0028
        .dc.w   0x0009,0x0010,0x8064
                | $09562C op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095632 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095634 --- callback 68000 embebido (16 B) -> PC=$095644 ---
        cmpi.w  #0x340, 0x106f50.l             | $095634  cmpi.w #$340, $106f50.l
        scs     d0                             | $09563C  scs.b d0
        lea     .L095644(pc), a1              | $09563E  a1 = nuevo PC
        rts                                    | $095642  rts
.L095644:
        .dc.w   0x0014
                | $095644 op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $095646 --- callback 68000 embebido (14 B) -> PC=$095654 ---
        move.b  #0x1, 0x10e39b.l               | $095646  move.b #$1, $10e39b.l
        lea     .L095654(pc), a0              | $09564E  a0 = nuevo PC
        rts                                    | $095652  rts
.L095654:
        .dc.w   0x0002
                | $095654 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_095656
        .section .text.SceneEntities_095656, "ax", @progbits
SceneEntities_095656:                       | 4 registros de 14 B + terminador (58 B)
        .dc.w   0x0100,0x0010,0x80e0,0x0000,0x0000,0x0015,0x0013
                | $095656 type=1 subop=00 tmpl=$1080E0 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0013
                | $095664 type=1 subop=00 tmpl=$108064 payload=0000000000150013
        .dc.w   0x0000,0x0010,0x7fe8,0x0000,0x0000,0x0015,0x0013
                | $095672 type=0 subop=00 tmpl=$107FE8 payload=0000000000150013
        .dc.w   0x0100,0x0010,0x6f6c,0xffa0,0x0000,0x0020,0x0013
                | $095680 type=1 subop=00 tmpl=$106F6C payload=ffa0000000200013
        .dc.w   0x0260                        | $09568E terminador type=2

        .globl  SceneScript_095690
        .section .text.SceneScript_095690, "ax", @progbits
SceneScript_095690:                         | bytecode VM de escena (3780 B, 154 ops)
        .dc.w   0x0014
                | $095690 op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $095692 --- callback 68000 embebido (14 B) -> PC=$0956A0 ---
        move.b  #0x0, 0x10e39b.l               | $095692  move.b #$0, $10e39b.l
        lea     .L0956a0(pc), a0              | $09569A  a0 = nuevo PC
        rts                                    | $09569E  rts
.L0956a0:
        .dc.w   0x0003,0x0000,0x0010
                | $0956A0 op $03 warp: camara=(0,16) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $0956A6 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x001c                 |   slot $10 <- bank $001C
        .dc.w   0x0011,0x001d                 |   slot $11 <- bank $001D
        .dc.w   0x0012,0x001e                 |   slot $12 <- bank $001E
        .dc.w   0x0013,0x001f                 |   slot $13 <- bank $001F
        .dc.w   0x0014,0x003d                 |   slot $14 <- bank $003D
        .dc.w   0x0015,0x006f                 |   slot $15 <- bank $006F
        .dc.w   0x0016,0x0079                 |   slot $16 <- bank $0079
        .dc.w   0x0017,0x007e                 |   slot $17 <- bank $007E
        .dc.w   0x0018,0x007f                 |   slot $18 <- bank $007F
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $095840 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $095842 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000f,0x0340,0x0010,0x0340,0x0010,0x0000
                | $095850 op $0F set_limits: minX=832 minY=16 maxX=832 maxY=16 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $09585C op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x0800
                | $095862 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x000c,0x000e,0x0d20,0x0021,0xeef4,0x0000,0x0060
                | $095866 op $00 bind_path: ent=$106F6C ruta=$48000C.. cont=$21EEF4 ancla=(0,96)
        .dc.w   0x0004,0x0010,0x6f6c,0x0c00
                | $09587C op $04 spawn: tmpl=$106F6C a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $095884 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $095890 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0028,0x000c,0x000e,0x1aa0,0x0000,0x0000,0x0000,0x0000
                | $095896 op $00 bind_path: ent=$108064 ruta=$28000C.. cont=$000000 ancla=(0,0)
        .dc.w   0x0004,0x0010,0x8064,0x0c00
                | $0958AC op $04 spawn: tmpl=$108064 a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0100
                | $0958B4 op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $0958C0 op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $0958C6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0958C8 --- callback 68000 embebido (16 B) -> PC=$0958D8 ---
        cmpi.w  #0x320, 0x106f50.l             | $0958C8  cmpi.w #$320, $106f50.l
        scs     d0                             | $0958D0  scs.b d0
        lea     .L0958d8(pc), a1              | $0958D2  a1 = nuevo PC
        rts                                    | $0958D6  rts
.L0958d8:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $0958D8 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $0958E6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0958E8 --- callback 68000 embebido (16 B) -> PC=$0958F8 ---
        cmpi.w  #0x340, 0x106f50.l             | $0958E8  cmpi.w #$340, $106f50.l
        scs     d0                             | $0958F0  scs.b d0
        lea     .L0958f8(pc), a1              | $0958F2  a1 = nuevo PC
        rts                                    | $0958F6  rts
.L0958f8:
        .dc.w   0x0006
                | $0958F8 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $0958FA --- callback 68000 embebido (16 B) -> PC=$09590A ---
        cmpi.b  #0x0, 0x10a2cf.l               | $0958FA  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $095902  seq.b d0
        lea     .L09590a(pc), a1              | $095904  a1 = nuevo PC
        rts                                    | $095908  rts
.L09590a:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $09590A op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $095918 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $095926 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $09592C op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $095932 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $095938 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0670,0x0280
                | $09593E op $15 set_campos: camara=(1648,640)
        .dc.w   0x0016,0x1670
                | $095944 op $16 set_progress: high_water/base=$1670
        .dc.w   0x000a                              | $095948 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0080                 |   slot $10 <- bank $0080
        .dc.w   0x0011,0x0081                 |   slot $11 <- bank $0081
        .dc.w   0x0012,0x0082                 |   slot $12 <- bank $0082
        .dc.w   0x0013,0x0083                 |   slot $13 <- bank $0083
        .dc.w   0x0014,0x0084                 |   slot $14 <- bank $0084
        .dc.w   0x0015,0x0085                 |   slot $15 <- bank $0085
        .dc.w   0x0016,0x0086                 |   slot $16 <- bank $0086
        .dc.w   0x0017,0x0087                 |   slot $17 <- bank $0087
        .dc.w   0x0018,0x0088                 |   slot $18 <- bank $0088
        .dc.w   0x0019,0x0089                 |   slot $19 <- bank $0089
        .dc.w   0x001a,0x008a                 |   slot $1A <- bank $008A
        .dc.w   0x001b,0x008b                 |   slot $1B <- bank $008B
        .dc.w   0x001c,0x008c                 |   slot $1C <- bank $008C
        .dc.w   0x001d,0x008d                 |   slot $1D <- bank $008D
        .dc.w   0x001e,0x008e                 |   slot $1E <- bank $008E
        .dc.w   0x001f,0x008f                 |   slot $1F <- bank $008F
        .dc.w   0x0020,0x0090                 |   slot $20 <- bank $0090
        .dc.w   0x0021,0x0091                 |   slot $21 <- bank $0091
        .dc.w   0x0022,0x0092                 |   slot $22 <- bank $0092
        .dc.w   0x0023,0x0093                 |   slot $23 <- bank $0093
        .dc.w   0x0024,0x0094                 |   slot $24 <- bank $0094
        .dc.w   0x0025,0x0095                 |   slot $25 <- bank $0095
        .dc.w   0x0026,0x0096                 |   slot $26 <- bank $0096
        .dc.w   0x0027,0x0097                 |   slot $27 <- bank $0097
        .dc.w   0x0028,0x0098                 |   slot $28 <- bank $0098
        .dc.w   0x0029,0x0099                 |   slot $29 <- bank $0099
        .dc.w   0x002a,0x009a                 |   slot $2A <- bank $009A
        .dc.w   0x002b,0x009b                 |   slot $2B <- bank $009B
        .dc.w   0x002c,0x009c                 |   slot $2C <- bank $009C
        .dc.w   0x002d,0x009d                 |   slot $2D <- bank $009D
        .dc.w   0x002e,0x009e                 |   slot $2E <- bank $009E
        .dc.w   0x002f,0x009f                 |   slot $2F <- bank $009F
        .dc.w   0x0030,0x00a0                 |   slot $30 <- bank $00A0
        .dc.w   0x0031,0x00a1                 |   slot $31 <- bank $00A1
        .dc.w   0x0032,0x00a2                 |   slot $32 <- bank $00A2
        .dc.w   0x0033,0x00a3                 |   slot $33 <- bank $00A3
        .dc.w   0x0034,0x00a4                 |   slot $34 <- bank $00A4
        .dc.w   0x0035,0x00a5                 |   slot $35 <- bank $00A5
        .dc.w   0x0036,0x00a6                 |   slot $36 <- bank $00A6
        .dc.w   0x0037,0x00a7                 |   slot $37 <- bank $00A7
        .dc.w   0x0038,0x00a8                 |   slot $38 <- bank $00A8
        .dc.w   0x0039,0x00a9                 |   slot $39 <- bank $00A9
        .dc.w   0x003a,0x00aa                 |   slot $3A <- bank $00AA
        .dc.w   0x003b,0x00ab                 |   slot $3B <- bank $00AB
        .dc.w   0x003c,0x00ac                 |   slot $3C <- bank $00AC
        .dc.w   0x003d,0x00ad                 |   slot $3D <- bank $00AD
        .dc.w   0x003e,0x00ae                 |   slot $3E <- bank $00AE
        .dc.w   0x003f,0x00af                 |   slot $3F <- bank $00AF
        .dc.w   0x0040,0x00b0                 |   slot $40 <- bank $00B0
        .dc.w   0x0041,0x00b1                 |   slot $41 <- bank $00B1
        .dc.w   0x0042,0x00b2                 |   slot $42 <- bank $00B2
        .dc.w   0x0043,0x00b3                 |   slot $43 <- bank $00B3
        .dc.w   0x0044,0x00b4                 |   slot $44 <- bank $00B4
        .dc.w   0x0045,0x00b5                 |   slot $45 <- bank $00B5
        .dc.w   0x0046,0x00b6                 |   slot $46 <- bank $00B6
        .dc.w   0x0047,0x00b7                 |   slot $47 <- bank $00B7
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x00b0                 |   slot $FF <- bank $00B0
        .dc.w   0xffff                              | $095AE2 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $095AE4 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000f,0x0a70,0x0280,0x0a70,0x0280,0x0000
                | $095AF2 op $0F set_limits: minX=2672 minY=640 maxX=2672 maxY=640 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $095AFE op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x0a00
                | $095B04 op $0C set_6eac: $106EAC.b = 0A
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x0012,0x000d,0x8d14,0x0021,0xb674,0x0670,0x0280
                | $095B08 op $00 bind_path: ent=$106F6C ruta=$480012.. cont=$21B674 ancla=(1648,640)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x3bea
                | $095B1E op $11 set_trig: ent=$106F6C tabla=$093BEA  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $095B28 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $095B34 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0005,0x0010,0x8064
                | $095B3A op $05 detach: ent=$108064  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0000,0x0010,0x80e0,0x0020,0x0007,0x000d,0xc70c,0x0000,0x0000,0x0670,0x0280
                | $095B40 op $00 bind_path: ent=$1080E0 ruta=$200007.. cont=$000000 ancla=(1648,640)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x00e0,0x0000
                | $095B56 op $01 set_fields: ent=$1080E0 +72=03 +74=$00E0 +76=$0000
        .dc.w   0x0004,0x0010,0x80e0,0x0700
                | $095B62 op $04 spawn: tmpl=$1080E0 a=07 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x80e0
                | $095B6A op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095B70 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095B72 --- callback 68000 embebido (16 B) -> PC=$095B82 ---
        cmpi.w  #0x950, 0x106f50.l             | $095B72  cmpi.w #$950, $106f50.l
        scs     d0                             | $095B7A  scs.b d0
        lea     .L095b82(pc), a1              | $095B7C  a1 = nuevo PC
        rts                                    | $095B80  rts
.L095b82:
        .dc.w   0x0000,0x0010,0x6f6c,0x0053,0x001a,0x000d,0xa154,0x0021,0xcab4,0x0af0,0x0200
                | $095B82 op $00 bind_path: ent=$106F6C ruta=$53001A.. cont=$21CAB4 ancla=(2800,512)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x3c0c
                | $095B98 op $11 set_trig: ent=$106F6C tabla=$093C0C  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $095BA2 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $095BAE op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095BB4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095BB6 --- callback 68000 embebido (16 B) -> PC=$095BC6 ---
        cmpi.w  #0xa60, 0x106f50.l             | $095BB6  cmpi.w #$a60, $106f50.l
        scs     d0                             | $095BBE  scs.b d0
        lea     .L095bc6(pc), a1              | $095BC0  a1 = nuevo PC
        rts                                    | $095BC4  rts
.L095bc6:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $095BC6 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $095BD4 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095BD6 --- callback 68000 embebido (16 B) -> PC=$095BE6 ---
        cmpi.w  #0xa70, 0x106f50.l             | $095BD6  cmpi.w #$a70, $106f50.l
        scs     d0                             | $095BDE  scs.b d0
        lea     .L095be6(pc), a1              | $095BE0  a1 = nuevo PC
        rts                                    | $095BE4  rts
.L095be6:
        .dc.w   0x0006
                | $095BE6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095BE8 --- callback 68000 embebido (16 B) -> PC=$095BF8 ---
        cmpi.b  #0x0, 0x10a2cf.l               | $095BE8  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $095BF0  seq.b d0
        lea     .L095bf8(pc), a1              | $095BF2  a1 = nuevo PC
        rts                                    | $095BF6  rts
.L095bf8:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $095BF8 op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $095C06 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $095C14 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $095C1A op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $095C20 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $095C26 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0530,0x0090
                | $095C2C op $15 set_campos: camara=(1328,144)
        .dc.w   0x0016,0x2530
                | $095C32 op $16 set_progress: high_water/base=$2530
        .dc.w   0x000a                              | $095C36 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x00c0                 |   slot $10 <- bank $00C0
        .dc.w   0x0011,0x00c1                 |   slot $11 <- bank $00C1
        .dc.w   0x0012,0x00c2                 |   slot $12 <- bank $00C2
        .dc.w   0x0013,0x00c3                 |   slot $13 <- bank $00C3
        .dc.w   0x0014,0x00c4                 |   slot $14 <- bank $00C4
        .dc.w   0x0015,0x00c5                 |   slot $15 <- bank $00C5
        .dc.w   0x0016,0x00c6                 |   slot $16 <- bank $00C6
        .dc.w   0x0017,0x00c7                 |   slot $17 <- bank $00C7
        .dc.w   0x0018,0x00c8                 |   slot $18 <- bank $00C8
        .dc.w   0x0019,0x00c9                 |   slot $19 <- bank $00C9
        .dc.w   0x001a,0x00ca                 |   slot $1A <- bank $00CA
        .dc.w   0x001b,0x00cb                 |   slot $1B <- bank $00CB
        .dc.w   0x001c,0x00cc                 |   slot $1C <- bank $00CC
        .dc.w   0x001d,0x00cd                 |   slot $1D <- bank $00CD
        .dc.w   0x001e,0x00ce                 |   slot $1E <- bank $00CE
        .dc.w   0x001f,0x00cf                 |   slot $1F <- bank $00CF
        .dc.w   0x0020,0x00d0                 |   slot $20 <- bank $00D0
        .dc.w   0x0021,0x00d1                 |   slot $21 <- bank $00D1
        .dc.w   0x0022,0x00d2                 |   slot $22 <- bank $00D2
        .dc.w   0x0023,0x00d3                 |   slot $23 <- bank $00D3
        .dc.w   0x0024,0x00d4                 |   slot $24 <- bank $00D4
        .dc.w   0x0025,0x00d5                 |   slot $25 <- bank $00D5
        .dc.w   0x0026,0x00d6                 |   slot $26 <- bank $00D6
        .dc.w   0x0027,0x00d7                 |   slot $27 <- bank $00D7
        .dc.w   0x0028,0x00d8                 |   slot $28 <- bank $00D8
        .dc.w   0x0029,0x00d9                 |   slot $29 <- bank $00D9
        .dc.w   0x002a,0x00da                 |   slot $2A <- bank $00DA
        .dc.w   0x002b,0x00db                 |   slot $2B <- bank $00DB
        .dc.w   0x002c,0x00dc                 |   slot $2C <- bank $00DC
        .dc.w   0x002d,0x00dd                 |   slot $2D <- bank $00DD
        .dc.w   0x002e,0x00de                 |   slot $2E <- bank $00DE
        .dc.w   0x002f,0x00df                 |   slot $2F <- bank $00DF
        .dc.w   0x0030,0x00e0                 |   slot $30 <- bank $00E0
        .dc.w   0x0031,0x00e1                 |   slot $31 <- bank $00E1
        .dc.w   0x0032,0x00e2                 |   slot $32 <- bank $00E2
        .dc.w   0x0033,0x00e3                 |   slot $33 <- bank $00E3
        .dc.w   0x0034,0x00e4                 |   slot $34 <- bank $00E4
        .dc.w   0x0035,0x00e5                 |   slot $35 <- bank $00E5
        .dc.w   0x0036,0x00e6                 |   slot $36 <- bank $00E6
        .dc.w   0x0037,0x00e7                 |   slot $37 <- bank $00E7
        .dc.w   0x0038,0x00e8                 |   slot $38 <- bank $00E8
        .dc.w   0x0039,0x00e9                 |   slot $39 <- bank $00E9
        .dc.w   0x003a,0x00ea                 |   slot $3A <- bank $00EA
        .dc.w   0x003b,0x00eb                 |   slot $3B <- bank $00EB
        .dc.w   0x003c,0x00ec                 |   slot $3C <- bank $00EC
        .dc.w   0x003d,0x00ed                 |   slot $3D <- bank $00ED
        .dc.w   0x003e,0x00ee                 |   slot $3E <- bank $00EE
        .dc.w   0x003f,0x00ef                 |   slot $3F <- bank $00EF
        .dc.w   0x0040,0x00f0                 |   slot $40 <- bank $00F0
        .dc.w   0x0041,0x00f1                 |   slot $41 <- bank $00F1
        .dc.w   0x0042,0x00f2                 |   slot $42 <- bank $00F2
        .dc.w   0x0043,0x00f3                 |   slot $43 <- bank $00F3
        .dc.w   0x0044,0x00f4                 |   slot $44 <- bank $00F4
        .dc.w   0x0045,0x00f5                 |   slot $45 <- bank $00F5
        .dc.w   0x0046,0x00f6                 |   slot $46 <- bank $00F6
        .dc.w   0x0047,0x00f7                 |   slot $47 <- bank $00F7
        .dc.w   0x0048,0x00f8                 |   slot $48 <- bank $00F8
        .dc.w   0x0049,0x00f9                 |   slot $49 <- bank $00F9
        .dc.w   0x004a,0x00fa                 |   slot $4A <- bank $00FA
        .dc.w   0x004b,0x00fb                 |   slot $4B <- bank $00FB
        .dc.w   0x004c,0x00fc                 |   slot $4C <- bank $00FC
        .dc.w   0x004d,0x00fd                 |   slot $4D <- bank $00FD
        .dc.w   0x004e,0x00fe                 |   slot $4E <- bank $00FE
        .dc.w   0x004f,0x00ff                 |   slot $4F <- bank $00FF
        .dc.w   0x0050,0x0100                 |   slot $50 <- bank $0100
        .dc.w   0x0051,0x0101                 |   slot $51 <- bank $0101
        .dc.w   0x0052,0x0102                 |   slot $52 <- bank $0102
        .dc.w   0x0053,0x0103                 |   slot $53 <- bank $0103
        .dc.w   0x0054,0x0104                 |   slot $54 <- bank $0104
        .dc.w   0x0055,0x0105                 |   slot $55 <- bank $0105
        .dc.w   0x0056,0x0106                 |   slot $56 <- bank $0106
        .dc.w   0x0057,0x0107                 |   slot $57 <- bank $0107
        .dc.w   0x0058,0x0108                 |   slot $58 <- bank $0108
        .dc.w   0x0059,0x0109                 |   slot $59 <- bank $0109
        .dc.w   0x005a,0x010a                 |   slot $5A <- bank $010A
        .dc.w   0x005b,0x010b                 |   slot $5B <- bank $010B
        .dc.w   0x005c,0x010c                 |   slot $5C <- bank $010C
        .dc.w   0x005d,0x010d                 |   slot $5D <- bank $010D
        .dc.w   0x005e,0x010e                 |   slot $5E <- bank $010E
        .dc.w   0x005f,0x010f                 |   slot $5F <- bank $010F
        .dc.w   0x0060,0x0110                 |   slot $60 <- bank $0110
        .dc.w   0x0061,0x0111                 |   slot $61 <- bank $0111
        .dc.w   0x0062,0x01d0                 |   slot $62 <- bank $01D0
        .dc.w   0x0063,0x01d1                 |   slot $63 <- bank $01D1
        .dc.w   0x0064,0x01d2                 |   slot $64 <- bank $01D2
        .dc.w   0x0065,0x01d3                 |   slot $65 <- bank $01D3
        .dc.w   0x0066,0x01d4                 |   slot $66 <- bank $01D4
        .dc.w   0x0067,0x01d5                 |   slot $67 <- bank $01D5
        .dc.w   0x0068,0x01d6                 |   slot $68 <- bank $01D6
        .dc.w   0x0069,0x01d7                 |   slot $69 <- bank $01D7
        .dc.w   0x006a,0x01d8                 |   slot $6A <- bank $01D8
        .dc.w   0x006b,0x01d9                 |   slot $6B <- bank $01D9
        .dc.w   0x006c,0x01da                 |   slot $6C <- bank $01DA
        .dc.w   0x006d,0x01db                 |   slot $6D <- bank $01DB
        .dc.w   0x006e,0x01dc                 |   slot $6E <- bank $01DC
        .dc.w   0x006f,0x01dd                 |   slot $6F <- bank $01DD
        .dc.w   0x0070,0x01de                 |   slot $70 <- bank $01DE
        .dc.w   0x0071,0x01df                 |   slot $71 <- bank $01DF
        .dc.w   0x0072,0x017d                 |   slot $72 <- bank $017D
        .dc.w   0x0073,0x017e                 |   slot $73 <- bank $017E
        .dc.w   0x0074,0x017f                 |   slot $74 <- bank $017F
        .dc.w   0x00ff,0x0109                 |   slot $FF <- bank $0109
        .dc.w   0xffff                              | $095DD0 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $095DD2 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000c,0x0800
                | $095DE0 op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x000f,0x0f00,0x0080,0x0f00,0x0090,0x0000
                | $095DE4 op $0F set_limits: minX=3840 minY=128 maxX=3840 maxY=144 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $095DF0 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0000,0x0010,0x6f6c,0x0078,0x0012,0x000a,0x18a8,0x0020,0x0000,0x0000,0x0080
                | $095DF6 op $00 bind_path: ent=$106F6C ruta=$780012.. cont=$200000 ancla=(0,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1ee0
                | $095E0C op $11 set_trig: ent=$106F6C tabla=$091EE0  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $095E16 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $095E22 op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095E28 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095E2A --- callback 68000 embebido (16 B) -> PC=$095E3A ---
        cmpi.w  #0x2e0, 0x106f50.l             | $095E2A  cmpi.w #$2e0, $106f50.l
        scs     d0                             | $095E32  scs.b d0
        lea     .L095e3a(pc), a1              | $095E34  a1 = nuevo PC
        rts                                    | $095E38  rts
.L095e3a:
        .dc.w   0x0005,0x0010,0x80e0
                | $095E3A op $05 detach: ent=$1080E0  (+$0E=0, $51ED6, commit MMIO)
        .dc.w   0x0006
                | $095E40 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095E42 --- callback 68000 embebido (16 B) -> PC=$095E52 ---
        cmpi.w  #0x590, 0x106f50.l             | $095E42  cmpi.w #$590, $106f50.l
        scs     d0                             | $095E4A  scs.b d0
        lea     .L095e52(pc), a1              | $095E4C  a1 = nuevo PC
        rts                                    | $095E50  rts
.L095e52:
        .dc.w   0x0000,0x0010,0x8064,0x0060,0x0012,0x000a,0x66a8,0x0000,0x0000,0x06d0,0x0030
                | $095E52 op $00 bind_path: ent=$108064 ruta=$600012.. cont=$000000 ancla=(1744,48)
        .dc.w   0x0004,0x0010,0x8064,0x1200
                | $095E68 op $04 spawn: tmpl=$108064 a=12 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x00c0,0x0100
                | $095E70 op $01 set_fields: ent=$108064 +72=03 +74=$00C0 +76=$0100
        .dc.w   0x0007,0x0010,0x8064
                | $095E7C op $07 call_ece: ent=$108064  ($51ECE)
        .dc.w   0x0006
                | $095E82 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095E84 --- callback 68000 embebido (16 B) -> PC=$095E94 ---
        cmpi.w  #0x5e0, 0x106f50.l             | $095E84  cmpi.w #$5e0, $106f50.l
        scs     d0                             | $095E8C  scs.b d0
        lea     .L095e94(pc), a1              | $095E8E  a1 = nuevo PC
        rts                                    | $095E92  rts
.L095e94:
        .dc.w   0x0000,0x0010,0x6f6c,0x005c,0x0012,0x000a,0x3a68,0x0020,0x21c0,0x0780,0x0080
                | $095E94 op $00 bind_path: ent=$106F6C ruta=$5C0012.. cont=$2021C0 ancla=(1920,128)
        .dc.w   0x0011,0x0010,0x6f6c,0x0009,0x1f0a
                | $095EAA op $11 set_trig: ent=$106F6C tabla=$091F0A  (-> $12(ent))
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $095EB4 op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0007,0x0010,0x6f6c
                | $095EC0 op $07 call_ece: ent=$106F6C  ($51ECE)
        .dc.w   0x0006
                | $095EC6 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095EC8 --- callback 68000 embebido (16 B) -> PC=$095ED8 ---
        cmpi.w  #0x600, 0x106f50.l             | $095EC8  cmpi.w #$600, $106f50.l
        scs     d0                             | $095ED0  scs.b d0
        lea     .L095ed8(pc), a1              | $095ED2  a1 = nuevo PC
        rts                                    | $095ED6  rts
.L095ed8:
        .dc.w   0x0000,0x0010,0x7fe8,0x0004,0x0004,0x000a,0x0000,0x0000,0x0000,0x0740,0x0160
                | $095ED8 op $00 bind_path: ent=$107FE8 ruta=$040004.. cont=$000000 ancla=(1856,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $095EEE op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0100,0x0100
                | $095EF6 op $01 set_fields: ent=$107FE8 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x7fe8
                | $095F02 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $095F08 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095F0A --- callback 68000 embebido (16 B) -> PC=$095F1A ---
        cmpi.w  #0x640, 0x106f50.l             | $095F0A  cmpi.w #$640, $106f50.l
        scs     d0                             | $095F12  scs.b d0
        lea     .L095f1a(pc), a1              | $095F14  a1 = nuevo PC
        rts                                    | $095F18  rts
.L095f1a:
        .dc.w   0x0000,0x0010,0x7fe8,0x0036,0x0004,0x000a,0x0040,0x0000,0x0000,0x0780,0x0160
                | $095F1A op $00 bind_path: ent=$107FE8 ruta=$360004.. cont=$000000 ancla=(1920,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $095F30 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0007,0x0010,0x7fe8
                | $095F38 op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $095F3E op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095F40 --- callback 68000 embebido (16 B) -> PC=$095F50 ---
        cmpi.w  #0x980, 0x106f50.l             | $095F40  cmpi.w #$980, $106f50.l
        scs     d0                             | $095F48  scs.b d0
        lea     .L095f50(pc), a1              | $095F4A  a1 = nuevo PC
        rts                                    | $095F4E  rts
.L095f50:
        .dc.w   0x0000,0x0010,0x80e0,0x001e,0x0001,0x000a,0x81a8,0x0000,0x0000,0x0ac0,0x0150
                | $095F50 op $00 bind_path: ent=$1080E0 ruta=$1E0001.. cont=$000000 ancla=(2752,336)
        .dc.w   0x0004,0x0010,0x80e0,0x0100
                | $095F66 op $04 spawn: tmpl=$1080E0 a=01 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x80e0,0x0300,0x0100,0x0100
                | $095F6E op $01 set_fields: ent=$1080E0 +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x80e0
                | $095F7A op $09 probe_ece: ent=$1080E0  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000c,0x0000
                | $095F80 op $0C set_6eac: $106EAC.b = 00
        .dc.w   0x0006
                | $095F84 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095F86 --- callback 68000 embebido (16 B) -> PC=$095F96 ---
        cmpi.w  #0x9a0, 0x106f50.l             | $095F86  cmpi.w #$9a0, $106f50.l
        scs     d0                             | $095F8E  scs.b d0
        lea     .L095f96(pc), a1              | $095F90  a1 = nuevo PC
        rts                                    | $095F94  rts
.L095f96:
        .dc.w   0x0000,0x0010,0x7fe8,0x0008,0x000b,0x000a,0x10f8,0x0000,0x0000,0x0ae0,0x00f0
                | $095F96 op $00 bind_path: ent=$107FE8 ruta=$08000B.. cont=$000000 ancla=(2784,240)
        .dc.w   0x0011,0x0010,0x7fe8,0x0009,0x1ed6
                | $095FAC op $11 set_trig: ent=$107FE8 tabla=$091ED6  (-> $12(ent))
        .dc.w   0x0007,0x0010,0x7fe8
                | $095FB6 op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $095FBC op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095FBE --- callback 68000 embebido (16 B) -> PC=$095FCE ---
        cmpi.w  #0xa20, 0x106f50.l             | $095FBE  cmpi.w #$a20, $106f50.l
        scs     d0                             | $095FC6  scs.b d0
        lea     .L095fce(pc), a1              | $095FC8  a1 = nuevo PC
        rts                                    | $095FCC  rts
.L095fce:
        .dc.w   0x0000,0x0010,0x7fe8,0x0022,0x0004,0x000a,0x1258,0x0000,0x0000,0x0b60,0x0160
                | $095FCE op $00 bind_path: ent=$107FE8 ruta=$220004.. cont=$000000 ancla=(2912,352)
        .dc.w   0x0004,0x0010,0x7fe8,0x0400
                | $095FE4 op $04 spawn: tmpl=$107FE8 a=04 b=00  ($51B1C)
        .dc.w   0x0007,0x0010,0x7fe8
                | $095FEC op $07 call_ece: ent=$107FE8  ($51ECE)
        .dc.w   0x0006
                | $095FF2 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $095FF4 --- callback 68000 embebido (16 B) -> PC=$096004 ---
        cmpi.w  #0xa10, 0x106f50.l             | $095FF4  cmpi.w #$a10, $106f50.l
        scs     d0                             | $095FFC  scs.b d0
        lea     .L096004(pc), a1              | $095FFE  a1 = nuevo PC
        rts                                    | $096002  rts
.L096004:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $096004 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $096012 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096014 --- callback 68000 embebido (16 B) -> PC=$096024 ---
        cmpi.w  #0xa30, 0x106f50.l             | $096014  cmpi.w #$a30, $106f50.l
        scs     d0                             | $09601C  scs.b d0
        lea     .L096024(pc), a1              | $09601E  a1 = nuevo PC
        rts                                    | $096022  rts
.L096024:
        .dc.w   0x0006
                | $096024 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096026 --- callback 68000 embebido (16 B) -> PC=$096036 ---
        cmpi.b  #0x0, 0x10a2cf.l               | $096026  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $09602E  seq.b d0
        lea     .L096036(pc), a1              | $096030  a1 = nuevo PC
        rts                                    | $096034  rts
.L096036:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $096036 op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $096044 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $096052 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $096058 op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $09605E op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $096064 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0040,0x0010
                | $09606A op $15 set_campos: camara=(64,16)
        .dc.w   0x0016,0x3040
                | $096070 op $16 set_progress: high_water/base=$3040
        .dc.w   0x000a                              | $096074 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0140                 |   slot $10 <- bank $0140
        .dc.w   0x0011,0x0141                 |   slot $11 <- bank $0141
        .dc.w   0x0012,0x0142                 |   slot $12 <- bank $0142
        .dc.w   0x0013,0x0143                 |   slot $13 <- bank $0143
        .dc.w   0x0014,0x0144                 |   slot $14 <- bank $0144
        .dc.w   0x0015,0x0145                 |   slot $15 <- bank $0145
        .dc.w   0x0016,0x0146                 |   slot $16 <- bank $0146
        .dc.w   0x0017,0x0147                 |   slot $17 <- bank $0147
        .dc.w   0x0018,0x0148                 |   slot $18 <- bank $0148
        .dc.w   0x0019,0x0149                 |   slot $19 <- bank $0149
        .dc.w   0x001a,0x014a                 |   slot $1A <- bank $014A
        .dc.w   0x001b,0x014b                 |   slot $1B <- bank $014B
        .dc.w   0x001c,0x014c                 |   slot $1C <- bank $014C
        .dc.w   0x001d,0x014d                 |   slot $1D <- bank $014D
        .dc.w   0x001e,0x014e                 |   slot $1E <- bank $014E
        .dc.w   0x001f,0x014f                 |   slot $1F <- bank $014F
        .dc.w   0x0020,0x0150                 |   slot $20 <- bank $0150
        .dc.w   0x0021,0x0151                 |   slot $21 <- bank $0151
        .dc.w   0x0022,0x0152                 |   slot $22 <- bank $0152
        .dc.w   0x0023,0x0153                 |   slot $23 <- bank $0153
        .dc.w   0x0024,0x0154                 |   slot $24 <- bank $0154
        .dc.w   0x0025,0x0155                 |   slot $25 <- bank $0155
        .dc.w   0x0026,0x0156                 |   slot $26 <- bank $0156
        .dc.w   0x0027,0x0157                 |   slot $27 <- bank $0157
        .dc.w   0x0028,0x0158                 |   slot $28 <- bank $0158
        .dc.w   0x0029,0x0159                 |   slot $29 <- bank $0159
        .dc.w   0x002a,0x015a                 |   slot $2A <- bank $015A
        .dc.w   0x002b,0x015b                 |   slot $2B <- bank $015B
        .dc.w   0x002c,0x015c                 |   slot $2C <- bank $015C
        .dc.w   0x002d,0x015d                 |   slot $2D <- bank $015D
        .dc.w   0x002e,0x015e                 |   slot $2E <- bank $015E
        .dc.w   0x002f,0x015f                 |   slot $2F <- bank $015F
        .dc.w   0x0030,0x0160                 |   slot $30 <- bank $0160
        .dc.w   0x0031,0x0161                 |   slot $31 <- bank $0161
        .dc.w   0x0032,0x0162                 |   slot $32 <- bank $0162
        .dc.w   0x0033,0x0163                 |   slot $33 <- bank $0163
        .dc.w   0x0034,0x0164                 |   slot $34 <- bank $0164
        .dc.w   0x0035,0x0165                 |   slot $35 <- bank $0165
        .dc.w   0x0036,0x0166                 |   slot $36 <- bank $0166
        .dc.w   0x0037,0x0167                 |   slot $37 <- bank $0167
        .dc.w   0x0038,0x0168                 |   slot $38 <- bank $0168
        .dc.w   0x0039,0x0169                 |   slot $39 <- bank $0169
        .dc.w   0x003a,0x016a                 |   slot $3A <- bank $016A
        .dc.w   0x003b,0x016b                 |   slot $3B <- bank $016B
        .dc.w   0x003c,0x016c                 |   slot $3C <- bank $016C
        .dc.w   0x003d,0x016d                 |   slot $3D <- bank $016D
        .dc.w   0x003e,0x016e                 |   slot $3E <- bank $016E
        .dc.w   0x003f,0x016f                 |   slot $3F <- bank $016F
        .dc.w   0x0040,0x0170                 |   slot $40 <- bank $0170
        .dc.w   0x0041,0x0171                 |   slot $41 <- bank $0171
        .dc.w   0x0042,0x0172                 |   slot $42 <- bank $0172
        .dc.w   0x0043,0x0173                 |   slot $43 <- bank $0173
        .dc.w   0x0044,0x0174                 |   slot $44 <- bank $0174
        .dc.w   0x0045,0x0175                 |   slot $45 <- bank $0175
        .dc.w   0x0046,0x0176                 |   slot $46 <- bank $0176
        .dc.w   0x0047,0x0177                 |   slot $47 <- bank $0177
        .dc.w   0x0048,0x0178                 |   slot $48 <- bank $0178
        .dc.w   0x0049,0x0179                 |   slot $49 <- bank $0179
        .dc.w   0x004a,0x017a                 |   slot $4A <- bank $017A
        .dc.w   0x004b,0x017b                 |   slot $4B <- bank $017B
        .dc.w   0x004c,0x017c                 |   slot $4C <- bank $017C
        .dc.w   0x004d,0x017d                 |   slot $4D <- bank $017D
        .dc.w   0x004e,0x017e                 |   slot $4E <- bank $017E
        .dc.w   0x004f,0x017f                 |   slot $4F <- bank $017F
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x013f                 |   slot $FF <- bank $013F
        .dc.w   0xffff                              | $09620E fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $096210 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000f,0x0540,0x0010,0x0540,0x0010,0x0000
                | $09621E op $0F set_limits: minX=1344 minY=16 maxX=1344 maxY=16 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $09622A op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x1800
                | $096230 op $0C set_6eac: $106EAC.b = 18
        .dc.w   0x0000,0x0010,0x6f6c,0x0120,0x0010,0x000c,0xe5fc,0x0021,0x5434,0x0000,0x0000
                | $096234 op $00 bind_path: ent=$106F6C ruta=$1200010.. cont=$215434 ancla=(0,0)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $09624A op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0004,0x0010,0x6f6c,0x1000
                | $096256 op $04 spawn: tmpl=$106F6C a=10 b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x6f6c
                | $09625E op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $096264 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096266 --- callback 68000 embebido (16 B) -> PC=$096276 ---
        cmpi.w  #0x520, 0x106f50.l             | $096266  cmpi.w #$520, $106f50.l
        scs     d0                             | $09626E  scs.b d0
        lea     .L096276(pc), a1              | $096270  a1 = nuevo PC
        rts                                    | $096274  rts
.L096276:
        .dc.w   0x000d,0x0005,0x276c,0x0001,0x0000,0x0000,0x0000
                | $096276 op $0D call_args: fn=$05276C args=0001000000000000
        .dc.w   0x0006
                | $096284 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096286 --- callback 68000 embebido (16 B) -> PC=$096296 ---
        cmpi.w  #0x540, 0x106f50.l             | $096286  cmpi.w #$540, $106f50.l
        scs     d0                             | $09628E  scs.b d0
        lea     .L096296(pc), a1              | $096290  a1 = nuevo PC
        rts                                    | $096294  rts
.L096296:
        .dc.w   0x0006
                | $096296 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096298 --- callback 68000 embebido (16 B) -> PC=$0962A8 ---
        cmpi.b  #0x0, 0x10a2cf.l               | $096298  cmpi.b #$0, $10a2cf.l
        seq     d0                             | $0962A0  seq.b d0
        lea     .L0962a8(pc), a1              | $0962A2  a1 = nuevo PC
        rts                                    | $0962A6  rts
.L0962a8:
        .dc.w   0x000d,0x0005,0x2780,0x0000,0x0000,0x0000,0x0000
                | $0962A8 op $0D call_args: fn=$052780 args=0000000000000000
        .dc.w   0x000d,0x0000,0x22c8,0x0000,0x0000,0x0000,0x0000
                | $0962B6 op $0D call_args: fn=$0022C8 args=0000000000000000
        .dc.w   0x000b,0x0010,0x6f6c
                | $0962C4 op $0B cond_clear: ent=$106F6C  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x7fe8
                | $0962CA op $0B cond_clear: ent=$107FE8  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x8064
                | $0962D0 op $0B cond_clear: ent=$108064  ($51F02; si +78 -> clear collmap)
        .dc.w   0x000b,0x0010,0x80e0
                | $0962D6 op $0B cond_clear: ent=$1080E0  ($51F02; si +78 -> clear collmap)
        .dc.w   0x0015,0x0000,0x0110
                | $0962DC op $15 set_campos: camara=(0,272)
        .dc.w   0x0016,0x4000
                | $0962E2 op $16 set_progress: high_water/base=$4000
        .dc.w   0x000a                              | $0962E6 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x001c                 |   slot $10 <- bank $001C
        .dc.w   0x0011,0x001d                 |   slot $11 <- bank $001D
        .dc.w   0x0012,0x001e                 |   slot $12 <- bank $001E
        .dc.w   0x0013,0x001f                 |   slot $13 <- bank $001F
        .dc.w   0x0014,0x003d                 |   slot $14 <- bank $003D
        .dc.w   0x0015,0x006f                 |   slot $15 <- bank $006F
        .dc.w   0x0016,0x0079                 |   slot $16 <- bank $0079
        .dc.w   0x0017,0x007e                 |   slot $17 <- bank $007E
        .dc.w   0x0018,0x007f                 |   slot $18 <- bank $007F
        .dc.w   0x0019,0xffff                 |   slot $19 <- bank $FFFF
        .dc.w   0x001a,0xffff                 |   slot $1A <- bank $FFFF
        .dc.w   0x001b,0xffff                 |   slot $1B <- bank $FFFF
        .dc.w   0x001c,0xffff                 |   slot $1C <- bank $FFFF
        .dc.w   0x001d,0xffff                 |   slot $1D <- bank $FFFF
        .dc.w   0x001e,0xffff                 |   slot $1E <- bank $FFFF
        .dc.w   0x001f,0xffff                 |   slot $1F <- bank $FFFF
        .dc.w   0x0020,0xffff                 |   slot $20 <- bank $FFFF
        .dc.w   0x0021,0xffff                 |   slot $21 <- bank $FFFF
        .dc.w   0x0022,0xffff                 |   slot $22 <- bank $FFFF
        .dc.w   0x0023,0xffff                 |   slot $23 <- bank $FFFF
        .dc.w   0x0024,0xffff                 |   slot $24 <- bank $FFFF
        .dc.w   0x0025,0xffff                 |   slot $25 <- bank $FFFF
        .dc.w   0x0026,0xffff                 |   slot $26 <- bank $FFFF
        .dc.w   0x0027,0xffff                 |   slot $27 <- bank $FFFF
        .dc.w   0x0028,0xffff                 |   slot $28 <- bank $FFFF
        .dc.w   0x0029,0xffff                 |   slot $29 <- bank $FFFF
        .dc.w   0x002a,0xffff                 |   slot $2A <- bank $FFFF
        .dc.w   0x002b,0xffff                 |   slot $2B <- bank $FFFF
        .dc.w   0x002c,0xffff                 |   slot $2C <- bank $FFFF
        .dc.w   0x002d,0xffff                 |   slot $2D <- bank $FFFF
        .dc.w   0x002e,0xffff                 |   slot $2E <- bank $FFFF
        .dc.w   0x002f,0xffff                 |   slot $2F <- bank $FFFF
        .dc.w   0x0030,0xffff                 |   slot $30 <- bank $FFFF
        .dc.w   0x0031,0xffff                 |   slot $31 <- bank $FFFF
        .dc.w   0x0032,0xffff                 |   slot $32 <- bank $FFFF
        .dc.w   0x0033,0xffff                 |   slot $33 <- bank $FFFF
        .dc.w   0x0034,0xffff                 |   slot $34 <- bank $FFFF
        .dc.w   0x0035,0xffff                 |   slot $35 <- bank $FFFF
        .dc.w   0x0036,0xffff                 |   slot $36 <- bank $FFFF
        .dc.w   0x0037,0xffff                 |   slot $37 <- bank $FFFF
        .dc.w   0x0038,0xffff                 |   slot $38 <- bank $FFFF
        .dc.w   0x0039,0xffff                 |   slot $39 <- bank $FFFF
        .dc.w   0x003a,0xffff                 |   slot $3A <- bank $FFFF
        .dc.w   0x003b,0xffff                 |   slot $3B <- bank $FFFF
        .dc.w   0x003c,0xffff                 |   slot $3C <- bank $FFFF
        .dc.w   0x003d,0xffff                 |   slot $3D <- bank $FFFF
        .dc.w   0x003e,0xffff                 |   slot $3E <- bank $FFFF
        .dc.w   0x003f,0xffff                 |   slot $3F <- bank $FFFF
        .dc.w   0x0040,0xffff                 |   slot $40 <- bank $FFFF
        .dc.w   0x0041,0xffff                 |   slot $41 <- bank $FFFF
        .dc.w   0x0042,0xffff                 |   slot $42 <- bank $FFFF
        .dc.w   0x0043,0xffff                 |   slot $43 <- bank $FFFF
        .dc.w   0x0044,0xffff                 |   slot $44 <- bank $FFFF
        .dc.w   0x0045,0xffff                 |   slot $45 <- bank $FFFF
        .dc.w   0x0046,0xffff                 |   slot $46 <- bank $FFFF
        .dc.w   0x0047,0xffff                 |   slot $47 <- bank $FFFF
        .dc.w   0x0048,0xffff                 |   slot $48 <- bank $FFFF
        .dc.w   0x0049,0xffff                 |   slot $49 <- bank $FFFF
        .dc.w   0x004a,0xffff                 |   slot $4A <- bank $FFFF
        .dc.w   0x004b,0xffff                 |   slot $4B <- bank $FFFF
        .dc.w   0x004c,0xffff                 |   slot $4C <- bank $FFFF
        .dc.w   0x004d,0xffff                 |   slot $4D <- bank $FFFF
        .dc.w   0x004e,0xffff                 |   slot $4E <- bank $FFFF
        .dc.w   0x004f,0xffff                 |   slot $4F <- bank $FFFF
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $096480 fin de pares
        .dc.w   0x000d,0x0005,0x2776,0x0001,0x0000,0x0000,0x0000
                | $096482 op $0D call_args: fn=$052776 args=0001000000000000
        .dc.w   0x000d,0x0005,0x2756,0x0011,0x001c,0xffff,0x0000
                | $096490 op $0D call_args: fn=$052756 args=0011001cffff0000
        .dc.w   0x000d,0x0005,0x2756,0x0012,0x003d,0xffff,0x0000
                | $09649E op $0D call_args: fn=$052756 args=0012003dffff0000
        .dc.w   0x000d,0x0005,0x2756,0x0013,0x006f,0xffff,0x0000
                | $0964AC op $0D call_args: fn=$052756 args=0013006fffff0000
        .dc.w   0x000f,0x0340,0x0078,0x0340,0x0110,0x0000
                | $0964BA op $0F set_limits: minX=832 minY=120 maxX=832 maxY=272 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $0964C6 op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x000c,0x0800
                | $0964CC op $0C set_6eac: $106EAC.b = 08
        .dc.w   0x0000,0x0010,0x6f6c,0x0048,0x000c,0x000e,0x0d20,0x0021,0xeef4,0x0000,0x0160
                | $0964D0 op $00 bind_path: ent=$106F6C ruta=$48000C.. cont=$21EEF4 ancla=(0,352)
        .dc.w   0x0004,0x0010,0x6f6c,0x0c00
                | $0964E6 op $04 spawn: tmpl=$106F6C a=0C b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $0964EE op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $0964FA op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0000,0x0010,0x8064,0x0020,0x000f,0x000e,0x2220,0x0000,0x0000,0x0000,0x0100
                | $096500 op $00 bind_path: ent=$108064 ruta=$20000F.. cont=$000000 ancla=(0,256)
        .dc.w   0x0004,0x0010,0x8064,0x0f00
                | $096516 op $04 spawn: tmpl=$108064 a=0F b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0300,0x0060,0x0028
                | $09651E op $01 set_fields: ent=$108064 +72=03 +74=$0060 +76=$0028
        .dc.w   0x0009,0x0010,0x8064
                | $09652A op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0006
                | $096530 op $06 wait_cb: callback embebido: d0!=0 -> re-poll; a1 = nuevo PC
                | $096532 --- callback 68000 embebido (16 B) -> PC=$096542 ---
        cmpi.w  #0x340, 0x106f50.l             | $096532  cmpi.w #$340, $106f50.l
        scs     d0                             | $09653A  scs.b d0
        lea     .L096542(pc), a1              | $09653C  a1 = nuevo PC
        rts                                    | $096540  rts
.L096542:
        .dc.w   0x0014
                | $096542 op $14 call_newpc: callback embebido: a0 = nuevo PC
                | $096544 --- callback 68000 embebido (14 B) -> PC=$096552 ---
        move.b  #0x1, 0x10e39b.l               | $096544  move.b #$1, $10e39b.l
        lea     .L096552(pc), a0              | $09654C  a0 = nuevo PC
        rts                                    | $096550  rts
.L096552:
        .dc.w   0x0002
                | $096552 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_096554
        .section .text.SceneEntities_096554, "ax", @progbits
SceneEntities_096554:                       | 3 registros de 14 B + terminador (44 B)
        .dc.w   0x0100,0x0010,0x8064,0x0000,0x0000,0x0015,0x0005
                | $096554 type=1 subop=00 tmpl=$108064 payload=0000000000150005
        .dc.w   0x0100,0x0010,0x7fe8,0x0000,0xff80,0x0018,0x0018
                | $096562 type=1 subop=00 tmpl=$107FE8 payload=0000ff8000180018
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0020,0x0010
                | $096570 type=1 subop=00 tmpl=$106F6C payload=0000000000200010
        .dc.w   0x0250                        | $09657E terminador type=2

        .globl  SceneScript_096580
        .section .text.SceneScript_096580, "ax", @progbits
SceneScript_096580:                         | bytecode VM de escena (498 B, 13 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $096580 op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $096586 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x0010,0x0003                 |   slot $10 <- bank $0003
        .dc.w   0x0011,0x0004                 |   slot $11 <- bank $0004
        .dc.w   0x0012,0x0005                 |   slot $12 <- bank $0005
        .dc.w   0x0013,0x0006                 |   slot $13 <- bank $0006
        .dc.w   0x0014,0x0007                 |   slot $14 <- bank $0007
        .dc.w   0x0015,0x0008                 |   slot $15 <- bank $0008
        .dc.w   0x0016,0x0009                 |   slot $16 <- bank $0009
        .dc.w   0x0017,0x000a                 |   slot $17 <- bank $000A
        .dc.w   0x0018,0x000b                 |   slot $18 <- bank $000B
        .dc.w   0x0019,0x000c                 |   slot $19 <- bank $000C
        .dc.w   0x001a,0x000d                 |   slot $1A <- bank $000D
        .dc.w   0x001b,0x000e                 |   slot $1B <- bank $000E
        .dc.w   0x001c,0x000f                 |   slot $1C <- bank $000F
        .dc.w   0x001d,0x0010                 |   slot $1D <- bank $0010
        .dc.w   0x001e,0x0011                 |   slot $1E <- bank $0011
        .dc.w   0x001f,0x0012                 |   slot $1F <- bank $0012
        .dc.w   0x0020,0x0014                 |   slot $20 <- bank $0014
        .dc.w   0x0021,0x0013                 |   slot $21 <- bank $0013
        .dc.w   0x0022,0x0015                 |   slot $22 <- bank $0015
        .dc.w   0x0023,0x0016                 |   slot $23 <- bank $0016
        .dc.w   0x0024,0x0017                 |   slot $24 <- bank $0017
        .dc.w   0x0025,0x0018                 |   slot $25 <- bank $0018
        .dc.w   0x0026,0x0019                 |   slot $26 <- bank $0019
        .dc.w   0x0027,0x001a                 |   slot $27 <- bank $001A
        .dc.w   0x0028,0x001b                 |   slot $28 <- bank $001B
        .dc.w   0x0029,0x001c                 |   slot $29 <- bank $001C
        .dc.w   0x002a,0x001d                 |   slot $2A <- bank $001D
        .dc.w   0x002b,0x001e                 |   slot $2B <- bank $001E
        .dc.w   0x002c,0x001f                 |   slot $2C <- bank $001F
        .dc.w   0x002d,0x0020                 |   slot $2D <- bank $0020
        .dc.w   0x002e,0x0021                 |   slot $2E <- bank $0021
        .dc.w   0x002f,0x0022                 |   slot $2F <- bank $0022
        .dc.w   0x0030,0x0023                 |   slot $30 <- bank $0023
        .dc.w   0x0031,0x0024                 |   slot $31 <- bank $0024
        .dc.w   0x0032,0x0025                 |   slot $32 <- bank $0025
        .dc.w   0x0033,0x0026                 |   slot $33 <- bank $0026
        .dc.w   0x0034,0x0027                 |   slot $34 <- bank $0027
        .dc.w   0x0035,0x0028                 |   slot $35 <- bank $0028
        .dc.w   0x0036,0x0029                 |   slot $36 <- bank $0029
        .dc.w   0x0037,0x002a                 |   slot $37 <- bank $002A
        .dc.w   0x0038,0x002b                 |   slot $38 <- bank $002B
        .dc.w   0x0039,0x002c                 |   slot $39 <- bank $002C
        .dc.w   0x003a,0x002d                 |   slot $3A <- bank $002D
        .dc.w   0x003b,0x002e                 |   slot $3B <- bank $002E
        .dc.w   0x003c,0x002f                 |   slot $3C <- bank $002F
        .dc.w   0x003d,0x0030                 |   slot $3D <- bank $0030
        .dc.w   0x003e,0x0031                 |   slot $3E <- bank $0031
        .dc.w   0x003f,0x0032                 |   slot $3F <- bank $0032
        .dc.w   0x0040,0x0033                 |   slot $40 <- bank $0033
        .dc.w   0x0041,0x0034                 |   slot $41 <- bank $0034
        .dc.w   0x0042,0x0035                 |   slot $42 <- bank $0035
        .dc.w   0x0043,0x0036                 |   slot $43 <- bank $0036
        .dc.w   0x0044,0x0037                 |   slot $44 <- bank $0037
        .dc.w   0x0045,0x0038                 |   slot $45 <- bank $0038
        .dc.w   0x0046,0x0039                 |   slot $46 <- bank $0039
        .dc.w   0x0047,0x003a                 |   slot $47 <- bank $003A
        .dc.w   0x0048,0x003b                 |   slot $48 <- bank $003B
        .dc.w   0x0049,0x003c                 |   slot $49 <- bank $003C
        .dc.w   0x004a,0x003d                 |   slot $4A <- bank $003D
        .dc.w   0x004b,0x003e                 |   slot $4B <- bank $003E
        .dc.w   0x004c,0x003f                 |   slot $4C <- bank $003F
        .dc.w   0x004d,0x00ba                 |   slot $4D <- bank $00BA
        .dc.w   0x004e,0x00bb                 |   slot $4E <- bank $00BB
        .dc.w   0x004f,0x00bc                 |   slot $4F <- bank $00BC
        .dc.w   0x0050,0xffff                 |   slot $50 <- bank $FFFF
        .dc.w   0x0051,0xffff                 |   slot $51 <- bank $FFFF
        .dc.w   0x0052,0xffff                 |   slot $52 <- bank $FFFF
        .dc.w   0x0053,0xffff                 |   slot $53 <- bank $FFFF
        .dc.w   0x0054,0xffff                 |   slot $54 <- bank $FFFF
        .dc.w   0x0055,0xffff                 |   slot $55 <- bank $FFFF
        .dc.w   0x0056,0xffff                 |   slot $56 <- bank $FFFF
        .dc.w   0x0057,0xffff                 |   slot $57 <- bank $FFFF
        .dc.w   0x0058,0xffff                 |   slot $58 <- bank $FFFF
        .dc.w   0x0059,0xffff                 |   slot $59 <- bank $FFFF
        .dc.w   0x005a,0xffff                 |   slot $5A <- bank $FFFF
        .dc.w   0x005b,0xffff                 |   slot $5B <- bank $FFFF
        .dc.w   0x005c,0xffff                 |   slot $5C <- bank $FFFF
        .dc.w   0x005d,0xffff                 |   slot $5D <- bank $FFFF
        .dc.w   0x005e,0xffff                 |   slot $5E <- bank $FFFF
        .dc.w   0x005f,0xffff                 |   slot $5F <- bank $FFFF
        .dc.w   0x0060,0xffff                 |   slot $60 <- bank $FFFF
        .dc.w   0x0061,0xffff                 |   slot $61 <- bank $FFFF
        .dc.w   0x0062,0xffff                 |   slot $62 <- bank $FFFF
        .dc.w   0x0063,0xffff                 |   slot $63 <- bank $FFFF
        .dc.w   0x0064,0xffff                 |   slot $64 <- bank $FFFF
        .dc.w   0x0065,0xffff                 |   slot $65 <- bank $FFFF
        .dc.w   0x0066,0xffff                 |   slot $66 <- bank $FFFF
        .dc.w   0x0067,0xffff                 |   slot $67 <- bank $FFFF
        .dc.w   0x0068,0xffff                 |   slot $68 <- bank $FFFF
        .dc.w   0x0069,0xffff                 |   slot $69 <- bank $FFFF
        .dc.w   0x006a,0xffff                 |   slot $6A <- bank $FFFF
        .dc.w   0x006b,0xffff                 |   slot $6B <- bank $FFFF
        .dc.w   0x006c,0xffff                 |   slot $6C <- bank $FFFF
        .dc.w   0x006d,0xffff                 |   slot $6D <- bank $FFFF
        .dc.w   0x006e,0xffff                 |   slot $6E <- bank $FFFF
        .dc.w   0x006f,0xffff                 |   slot $6F <- bank $FFFF
        .dc.w   0x0070,0xffff                 |   slot $70 <- bank $FFFF
        .dc.w   0x0071,0xffff                 |   slot $71 <- bank $FFFF
        .dc.w   0x0072,0xffff                 |   slot $72 <- bank $FFFF
        .dc.w   0x0073,0xffff                 |   slot $73 <- bank $FFFF
        .dc.w   0x0074,0xffff                 |   slot $74 <- bank $FFFF
        .dc.w   0xffff                              | $09671C fin de pares
        .dc.w   0x0001,0x0010,0x6f6c,0x0300,0x0100,0x0100
                | $09671E op $01 set_fields: ent=$106F6C +72=03 +74=$0100 +76=$0100
        .dc.w   0x0009,0x0010,0x6f6c
                | $09672A op $09 probe_ece: ent=$106F6C  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0001,0x0010,0x7fe8,0x0300,0x0080,0x0100
                | $096730 op $01 set_fields: ent=$107FE8 +72=03 +74=$0080 +76=$0100
        .dc.w   0x0004,0x0010,0x7fe8,0x0d00
                | $09673C op $04 spawn: tmpl=$107FE8 a=0D b=00  ($51B1C)
        .dc.w   0x0009,0x0010,0x7fe8
                | $096744 op $09 probe_ece: ent=$107FE8  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x0004,0x0010,0x8064,0x0800
                | $09674A op $04 spawn: tmpl=$108064 a=08 b=00  ($51B1C)
        .dc.w   0x0001,0x0010,0x8064,0x0100,0x0040,0x0100
                | $096752 op $01 set_fields: ent=$108064 +72=01 +74=$0040 +76=$0100
        .dc.w   0x0009,0x0010,0x8064
                | $09675E op $09 probe_ece: ent=$108064  (CameraHook_Probe08 + $51ECE)
        .dc.w   0x000c,0x0400
                | $096764 op $0C set_6eac: $106EAC.b = 04
        .dc.w   0x0004,0x0010,0x6f6c,0x0808
                | $096768 op $04 spawn: tmpl=$106F6C a=08 b=08  ($51B1C)
        .dc.w   0x0002
                | $096770 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  SceneEntities_096772
        .section .text.SceneEntities_096772, "ax", @progbits
SceneEntities_096772:                       | 1 registros de 14 B + terminador (16 B)
        .dc.w   0x0100,0x0010,0x6f6c,0x0000,0x0000,0x0015,0x0013
                | $096772 type=1 subop=00 tmpl=$106F6C payload=0000000000150013
        .dc.w   0x0200                        | $096780 terminador type=2

        .globl  SceneScript_096782
        .section .text.SceneScript_096782, "ax", @progbits
SceneScript_096782:                         | bytecode VM de escena (34 B, 5 ops)
        .dc.w   0x0003,0x0000,0x0000
                | $096782 op $03 warp: camara=(0,0) modo_eje=1 high_water=0
        .dc.w   0x000a                              | $096788 op $0A slot_pairs ($2B58): {slot,bank} hasta $FFFF
        .dc.w   0x00ff,0x0000                 |   slot $FF <- bank $0000
        .dc.w   0xffff                              | $09678E fin de pares
        .dc.w   0x000f,0x0000,0x0000,0x0000,0x0000,0x0000
                | $096790 op $0F set_limits: minX=0 minY=0 maxX=0 maxY=0 slope=0
        .dc.w   0x0010,0x0000,0x0000
                | $09679C op $10 set_6466: $108164=0000 $108166=0000
        .dc.w   0x0002
                | $0967A2 op $02 END_FRAME: cede el frame (integra scroll+camara)

        .globl  ChildRank_CmpByte10_0967A4
        .section .text.ChildRank_CmpByte10_0967A4, "ax", @progbits
ChildRank_CmpByte10_0967A4:                 | CCR: C=1 si child.rank(+$10) > ent.rank(+$10). Sin callers (huerfana)
        movea.l 0x8(a6), a1                    | +00  a1 = entidad hija (+$08)
        move.b  0x10(a6), d0                   | +04  d0 = rango propio
        cmp.b   0x10(a1), d0                   | +08  vs rango de la hija
        bcs.w   SetXN_0967ba                   | +0c  menor -> C=1 (isla SetXN)
                                               | +10  cae en ClearXN_0967b4
