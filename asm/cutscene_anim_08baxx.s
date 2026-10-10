| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave MMM — Proyectiles de rebote, script de animación, iconos de ranura,
|             cutscene de texto y arranque de escenas B/C
|  Región: $08BA04..$08D17A  (4,990 B, 67 entradas, 39 huecos cerrados)
| ============================================================================
|
|  Región heterogénea entre el cluster de la escena 5 (Wave LLL) y la
|  máquina de estados de animación $08C008..$08C2B8 (anim_state_machine_08cxxx.s,
|  Wave GG), cuyos 6 task-adds apuntan a los Icon_* de esta wave. Puntos de
|  entrada externos: plantillas Mission VM $E8000[244]=Cut_Dropper_08ccea,
|  [245]=Cut_Item_08ce1e, [246]=Cut_SetMode2_08c7e2, [248]=Cut_SetVariant1,
|  [320]=Cut_SetVariant2; MissionDriver_Init_0442E6 arranca Cut_Watcher_Init
|  ($8C864) como tarea paralela; el dispatcher de modo ($1600..$1700) carga
|  SceneB_Init ($8CE64, $106ECE=$0B), SceneC_Init ($8D0A8, $106ECE=$0C) y
|  Cut_Fade ($8C956); Proj_Tmpl168/169/170 (LLL) encadenan Proj_Tmpl_Init* y
|  Proj_Bounce_V1/V2.
|
|  A) $08BA0C..$08BC58 — PROYECTILES DE REBOTE Y RÁFAGA
|     Proj_Bounce_V1/V2 ($8BA0C/$8BA52): variantes del Proj_Bounce de LLL
|       (mapas $29CA0A/$29BFC4; V2 bset bit0 +$3A); física $2783A, slots
|       $28D70, daño $2870A y despacho por tabla Proj_Bounce_HitTable_08b944
|       vía Table_LookupPointerBounded ($772) con a1=-1 (sin fallback).
|     Proj_Burst_HitTable/HitTable2 ($8BA9E/$8BAAE, 4 ptrs cada una, todos a
|       Proj_Burst_08baf6) seleccionadas al azar (RNG_LFSRStep $5E9B6 & 3)
|       por Proj_Burst_RandDispatch/_2 — tablas de 4 entradas idénticas:
|       probablemente placeholders de variantes nunca implementadas.
|     Proj_Burst_08baf6: snd $2F, mapa $4B136, física; al quedarse sin hijos
|       pasa a JmpToScheduler_08bb84 (muere).
|     Proj_Tmpl_InitHitboxProbe/InitHitbox ($8BB34/$8BB5E): snd $0E, hitbox
|       Hitbox_08b950 (LLL) en +$48, [probe loop Entity_ProbeTransformLoop
|       ($27C8C hasta C=1)], lanza anim $776E2 con $4AE.
|     Proj_Shell_08bc0c: snd $1B, anim $7773E, copia xf $5DD02, hitbox
|       Hitbox_08bb8c (2 cajas), mapa SpriteMap_08bbde (tiles $235764/$23578E,
|       4 celdas + puntero a sí mismo), bclr bit3 +$13.
|
|  B) $08BC74..$08C008 — INTÉRPRETE DE SCRIPT DE ANIMACIÓN (registros 8 B)
|     Anim_ScriptStep_08bc74 / _Alt_08bf96: cursor +$78 sobre lista +$7C;
|       cada registro {dur.w, val.w, b4, b5, pad}: +$70=dur, +$76=val,
|       +$92/+$93 = bytes 4/5 (Alt copia b4 en ambos); dur=$FFFF = fin
|       (ceros). Devuelven C=0 vía SetXN ($FFFF en +$70 = inactivo -> C=1).
|     Anim_ScriptStepFix_A/B ($8BCE6/$8BE28): igual pero cursor +$82 y
|       registros {dur, code.w, map.l}: carga mapa de sprites ($28CD4) y
|       según code: 1 = blit Fix_BlitRect $5DA9C (#$7084,$2320,$20,$8);
|       $D = reposiciona cursor fix +$86; 2 = spawn Icon_Anchor_Drop; 3 =
|       spawn $2AE50 en ($A0,$1FF) con +$98=3; otro = carácter fix: A
|       escribe vía $47872 (paleta 4, o 9 para códigos $5xx/$6xx/$7xx/>=$Dxx),
|       B escribe directo en VRAM $3C0000 (ori #$2300; códigos $E6/$E7..$EF
|       con ajuste de celda). Anim_ScriptStepFix_Next_08be12 avanza +$82 y
|       repite (varios registros por frame).
|
|  C) $08C2B8..$08C730 — ICONOS DE RANURA (4 slots × 48 px, fila y=$12B)
|     Icon_Base_08c2b8: prio $E000, snd $B1, mapa $2F2E0A en x=$40; cada
|       frame elige mapa de $2F2DCA[padre+$92 & $F].
|     Icon_Slot1..4 ($8C322/$8C37E/$8C3DA/$8C436): snd $AE/$AF/$B0/$B2, x=
|       $70/$A0/$D0/$100, máscara +$8A=$10/$20/$40/$80, spawnean su
|       Icon_SlotN_Lit (mapas $2F2F22/2E/3A/46) y corren Icon_Slot_Run_08c4a0:
|       si (padre+$93 & máscara) -> +$76=1, mapa $2F2EE2 (encendido); si
|       (padre+$92 & máscara)=0 y estaba encendido -> mapa $2F2F02 (apagado).
|       Slot4 además hace Fix_BlitRectToFixLayer (#$73D7,$2E80,4,4).
|     Icon_Anchor_Init_08c5b2: +$8A = idioma/región según $10FD83/$10FD92
|       (0..3), snd $97, mapa $2F2F52 en ($E0,$160), script +$7C=$2F0056,
|       cursor fix +$86=$7084. Icon_Anchor_Run_08c678: cuando el padre
|       cambia +$76 selecciona el script en $2EFACC/$2EFC2C/$2EFD8C/$2EFEEC
|       [+$8A] (ptr, cursor) y ejecuta Anim_ScriptStepFix_A (idioma 0) o
|       _B (otros); muere si x <= $150 tras probe $27CEE.
|     Icon_Anchor_Drop_08c730: objeto que cae desde ($140,$180), snd $179,
|       vel y=-$800, mapa $2EF80E, hitbox $2EF84C; al tocar suelo (Entity_SetOffscreenFlag_05dd56
|       con $2EF8A0) o bit1 +$13 spawnea Explosion_Fire_077f6a y muere.
|
|  D) $08C7C6..$08C9A6 — CUTSCENE: MODO, VIGILANTE Y FUNDIDO
|     Cut_SetMode2 ($10E2EF=2), Cut_SetVariant1/2 ($10E2EE=1/2) + Rts_*:
|       plantillas de 1 instrucción (templates 246/248/320).
|     Cut_InstallListByVariant_08c820: InstallListPubHead ($5DB58) con lista
|       $2F3688 / $2F35FE / $2F3574 según $10E2EE.
|     Cut_Watcher_Init/Run/Play ($8C864/$8C880/$8C8FA): tarea paralela del
|       Mission VM: vigila el high-water $106F5C (+$70) con debounce $78 f;
|       si el scroll se detiene, $106E88/$106E8A=0, $5E1AA y
|       SceneScript_EdgeArrivalTest ($43CE2) no bloquean y $106ED3 != 0,
|       arma $10E2EF=2 y pasa a Play: instala la lista, snd $10D5 ($2352),
|       espera $5DB6A, decrementa $10E2EF y reinicia el cursor ($5DBDC).
|     Cut_Fade_08c956: $78 frames; List_ApplyWithSentinelFF ($4784C) sobre
|       $2F4A12 al entrar y $2F4A22 al salir (paleta $70AE), clr $106ED2.
|
|  E) $08C9A6..$08CC66 — PANEL DE TEXTO EN LA CAPA FIX
|     Fix_TextRow_Draw_08c9a6: recorre la cadena +$80 (words): $FFFE = salto
|       a columna $709C, 0 = fin de línea (guarda cursor), $FFFF = fin,
|       $FFFD = ignorado; cada carácter se dibuja como 2×2 tiles (base
|       $709A, paleta $9000 o $4000 para $Bxx/$Cxx) en VRAM $3C0000.
|     Fix_TextRow_Clear_08ca5e: limpia el rect ($701A,$20,$28,2) y
|       redibuja; Fix_TextRow_DrawOne_08cb18: variante por carácter con
|       cursor +$84 (usada por Cut_TextPanel_Type para efecto máquina de
|       escribir, hasta 32 caracteres/frame según $106F29).
|     Cut_TextPanel_08cbc6: cadena $2F4712, snd $15E, mapa $2F3732 en
|       ($A0,$121); Cut_Banner_08cc66: $F0 frames, mapa $2F373E, luego
|       $46A96 y muere.
|
|  F) $08CCC6..$08CE64 — SCROLL Y OBJETOS DE LA CUTSCENE
|     Scroll_StepVelX/Y: publican +$80/+$88 en $106F60/$106F64 y suman
|       las aceleraciones +$84/+$8C.
|     Cut_Dropper_08ccea (template 244): vel y = +$99<<5, snd $136, mapa
|       $2F455C[+$98 (0..$A)]; cada frame Pos_IntegrateY88_08d2d4 (hueco futuro); si
|       +$98=0 y misión $B y y>=$170 -> WaitScroll (muere al llegar
|       $106F5C>=$1000); si +$98=$A y y>=$170 -> Finish ($78 f, snd $60 en
|       misión $C, spawn $5239E #4, $A0 f) -> Exit (clr $106ED2).
|     Cut_Item_08ce1e (template 245): snd $5F, mapa $2F3B2C, Pos_IntegrateY88_08d2d4,
|       contador +$99, luego Phys_GroundKill_08efb0.
|
|  G) $08CE64..$08D17A — ARRANQUE DE ESCENAS B Y C (misiones $0B/$0C)
|     SceneB_Init_08ce64: snd $29, SceneLoader_Main ($43568, #$E), +$21=6,
|       $106F5E=-1, vel scroll $106F60=$1C000 (1P, $106EAE=1) / $18000,
|       $46A96, $2230, tarea Capsule_Fly_08d3b4; luego por etapas con
|       SceneScriptVM_Frame + AttractCuller_Cam1 + Debug_DrawHUDVars: al
|       cruzar $106F5C >= $100/$1000/$2B94/$2D00/$3000/$4000 cambia la
|       velocidad ($1A000/$16000, $16000/$12000, $16000/$12000,
|       $14000/$10000, $26000/$20000, $26000/$20000) y encadena
|       SceneB_Stage2..6 -> SceneB_Tail.
|     SceneC_Init_08d0a8: spawnea Cut_Banner y Cut_TextPanel, snd $2A,
|       SceneLoader_Main #$F, +$21=7, $106F60=$18000; contiene un bloque
|       muerto (segunda inicialización tras `bra.w`) y acaba en el mismo
|       bucle de frame.
|
|  Callees pendientes: $46A96, $2230, $5E1AA, $5DB6A, $2AE50, $776E2,
|  $7773E, $47872, $2308; forward: $8D24C, $8D2D4, $8D3B4.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Proj_Bounce_V1_08ba0c  @ $08BA0C  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Bounce_V1_08ba0c, "ax", @progbits
        .global Proj_Bounce_V1_08ba0c
Proj_Bounce_V1_08ba0c:
        move.w  #0x0,0x28(a6)                   | +000
        lea     0x29ca0a.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L08ba24(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L08ba24:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L08ba3a                        | +024
        lea     Proj_Bounce_V1_08ba0c(pc),a1    | +028
        move.l  a1,(a6)                         | +02c
.L08ba3a:
        jsr     0x2870a.l                       | +02e
        lea     Proj_Bounce_HitTable_08b944(pc),a0 | +034
        movea.l #0xffffffff,a1                  | +038

| ----------------------------------------------------------------------------
|  Proj_Bounce_V2_08ba52  @ $08BA52  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Bounce_V2_08ba52, "ax", @progbits
        .global Proj_Bounce_V2_08ba52
Proj_Bounce_V2_08ba52:
        bset    #0x0,0x3a(a6)                   | +000
        move.w  #0x0,0x28(a6)                   | +006
        lea     0x29bfc4.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L08ba70(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L08ba70:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L08ba86                        | +02a
        lea     Proj_Bounce_V1_08ba0c(pc),a1    | +02e
        move.l  a1,(a6)                         | +032
.L08ba86:
        jsr     0x2870a.l                       | +034
        lea     Proj_Bounce_HitTable_08b944(pc),a0 | +03a
        movea.l #0xffffffff,a1                  | +03e

| ----------------------------------------------------------------------------
|  Proj_Burst_HitTable_08ba9e  @ $08BA9E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Burst_HitTable_08ba9e, "ax", @progbits
        .global Proj_Burst_HitTable_08ba9e
Proj_Burst_HitTable_08ba9e:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +004  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +008  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Proj_Burst_HitTable2_08baae  @ $08BAAE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Burst_HitTable2_08baae, "ax", @progbits
        .global Proj_Burst_HitTable2_08baae
Proj_Burst_HitTable2_08baae:
        .dc.w   0x0008                        | +000  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +004  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +008  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xbaf6                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Proj_Burst_RandDispatch_08babe  @ $08BABE  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Burst_RandDispatch_08babe, "ax", @progbits
        .global Proj_Burst_RandDispatch_08babe
Proj_Burst_RandDispatch_08babe:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        lea     Proj_Burst_HitTable_08ba9e(pc),a0 | +00a
        movea.l #0xffffffff,a1                  | +00e
        jsr     0x772.l                         | +014
        jmp     (a1)                            | +01a

| ----------------------------------------------------------------------------
|  Proj_Burst_RandDispatch2_08bada  @ $08BADA  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Burst_RandDispatch2_08bada, "ax", @progbits
        .global Proj_Burst_RandDispatch2_08bada
Proj_Burst_RandDispatch2_08bada:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        lea     Proj_Burst_HitTable2_08baae(pc),a0 | +00a
        movea.l #0xffffffff,a1                  | +00e
        jsr     0x772.l                         | +014
        jmp     (a1)                            | +01a

| ----------------------------------------------------------------------------
|  Proj_Burst_08baf6  @ $08BAF6  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Burst_08baf6, "ax", @progbits
        .global Proj_Burst_08baf6
Proj_Burst_08baf6:
        move.w  #0x2f,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x4b136.l,a0                    | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L08bb12(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L08bb12:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   SetHandlerRts_08bb28            | +028

| ----------------------------------------------------------------------------
|  Entity_ProbeTransformLoop_08bb2a  @ $08BB2A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ProbeTransformLoop_08bb2a, "ax", @progbits
        .global Entity_ProbeTransformLoop_08bb2a
Entity_ProbeTransformLoop_08bb2a:
        jsr     0x27c8c.l                       | +000
        bcc.b   Entity_ProbeTransformLoop_08bb2a | +006
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Proj_Tmpl_InitHitboxProbe_08bb34  @ $08BB34  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Tmpl_InitHitboxProbe_08bb34, "ax", @progbits
        .global Proj_Tmpl_InitHitboxProbe_08bb34
Proj_Tmpl_InitHitboxProbe_08bb34:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     Hitbox_08b950(pc),a0            | +00a
        move.l  a0,0x48(a6)                     | +00e
        jsr     Entity_ProbeTransformLoop_08bb2a(pc) | +012
        lea     0x776e2.l,a1                    | +016
        jsr     0x4ae.l                         | +01c

| ----------------------------------------------------------------------------
|  Proj_Tmpl_InitHitbox_08bb5e  @ $08BB5E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Tmpl_InitHitbox_08bb5e, "ax", @progbits
        .global Proj_Tmpl_InitHitbox_08bb5e
Proj_Tmpl_InitHitbox_08bb5e:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     Hitbox_08b950(pc),a0            | +00a
        move.l  a0,0x48(a6)                     | +00e
        lea     0x776e2.l,a1                    | +012
        jsr     0x4ae.l                         | +018

| ----------------------------------------------------------------------------
|  Hitbox_08bb8c  @ $08BB8C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Hitbox_08bb8c, "ax", @progbits
        .global Hitbox_08bb8c
Hitbox_08bb8c:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  SpriteMap_08bbde  @ $08BBDE  (46 B)
| ----------------------------------------------------------------------------
        .section .text.SpriteMap_08bbde, "ax", @progbits
        .global SpriteMap_08bbde
SpriteMap_08bbde:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +004  (dato / opcode no decodificado)
        .dc.w   0x5764                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x578e                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +018  (dato / opcode no decodificado)
        .dc.w   0x5764                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +022  (dato / opcode no decodificado)
        .dc.w   0x578e                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xbbde                        | +02c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Proj_Shell_08bc0c  @ $08BC0C  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Shell_08bc0c, "ax", @progbits
        .global Proj_Shell_08bc0c
Proj_Shell_08bc0c:
        move.w  #0x1b,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x7773e.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        jsr     Entity_ProbeTransformLoop_08bb2a(pc) | +01c
        lea     Hitbox_08bb8c(pc),a0            | +020
        move.l  a0,0x48(a6)                     | +024
        lea     SpriteMap_08bbde(pc),a0         | +028
        jsr     0x28cd4.l                       | +02c
        lea     .L08bc44(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08bc44:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bclr    #0x3,0x13(a6)                   | +044
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08bc58  @ $08BC58  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08bc58, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08bc58
Entity_CmpPrioWithSibling_08bc58:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08bc6e                    | +00c

| ----------------------------------------------------------------------------
|  Anim_ScriptStep_08bc74  @ $08BC74  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Anim_ScriptStep_08bc74, "ax", @progbits
        .global Anim_ScriptStep_08bc74
Anim_ScriptStep_08bc74:
        cmpi.w  #0xffff,0x70(a6)                | +000
        beq.w   SetXN_08bce0                    | +006
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   .L08bcd2                        | +014
        movea.l 0x7c(a6),a1                     | +018
        adda.l  0x78(a6),a1                     | +01c
        move.w  (a1),0x70(a6)                   | +020
        cmpi.w  #0xffff,0x70(a6)                | +024
        beq.w   .L08bcbc                        | +02a
        move.w  0x2(a1),0x76(a6)                | +02e
        move.b  0x4(a1),0x92(a6)                | +034
        move.b  0x5(a1),0x93(a6)                | +03a
        addq.l  #0x8,0x78(a6)                   | +040
        bra.w   .L08bcce                        | +044
.L08bcbc:
        move.w  #0x0,0x76(a6)                   | +048
        move.b  #0x0,0x92(a6)                   | +04e
        move.b  #0x0,0x93(a6)                   | +054
.L08bcce:
        bra.w   .L08bcd8                        | +05a
.L08bcd2:
        move.b  #0x0,0x93(a6)                   | +05e
.L08bcd8:
        andi.b  #0xee,ccr                       | +064
        bra.w   SetXNMid_08bce4                 | +068

| ----------------------------------------------------------------------------
|  Anim_ScriptStepFix_A_08bce6  @ $08BCE6  (316 B)
| ----------------------------------------------------------------------------
        .section .text.Anim_ScriptStepFix_A_08bce6, "ax", @progbits
        .global Anim_ScriptStepFix_A_08bce6
Anim_ScriptStepFix_A_08bce6:
        cmpi.w  #0xffff,0x70(a6)                | +000
        beq.w   SetXN_08be22                    | +006
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   .L08be1a                        | +014
.L08bcfe:
        movea.l 0x7c(a6),a1                     | +018
        adda.l  0x82(a6),a1                     | +01c
        move.w  (a1),0x70(a6)                   | +020
        cmpi.w  #0xffff,0x70(a6)                | +024
        beq.w   .L08be1a                        | +02a
        move.w  0x2(a1),0x80(a6)                | +02e
        cmpi.l  #0x0,0x4(a1)                    | +034
        beq.w   .L08bd30                        | +03c
        movea.l 0x4(a1),a0                      | +040
        jsr     0x28cd4.l                       | +044
.L08bd30:
        cmpi.w  #0x1,0x80(a6)                   | +04a
        bne.w   .L08bd54                        | +050
        movea.w #0x7084,a1                      | +054
        move.w  #0x2320,d0                      | +058
        move.w  #0x20,d1                        | +05c
        move.w  #0x8,d2                         | +060
        jsr     0x5da9c.l                       | +064
        bra.w   Anim_ScriptStepFix_Next_08be12 | +06a
.L08bd54:
        cmpi.w  #0xd,0x80(a6)                   | +06e
        bne.w   .L08bd6a                        | +074
        move.l  #0x70a9,0x86(a6)                | +078
        bra.w   Anim_ScriptStepFix_Next_08be12 | +080
.L08bd6a:
        cmpi.w  #0x2,0x80(a6)                   | +084
        bne.w   .L08bd82                        | +08a
        lea     Icon_Anchor_Drop_08c730(pc),a1  | +08e
        jsr     0x4ae.l                         | +092
        bra.w   Anim_ScriptStepFix_Next_08be12 | +098
.L08bd82:
        cmpi.w  #0x3,0x80(a6)                   | +09c
        bne.w   .L08bdae                        | +0a2
        lea     0x2ae50.l,a1                    | +0a6
        jsr     0x4ae.l                         | +0ac
        move.w  #0xa0,0x22(a0)                  | +0b2
        move.w  #0x1ff,0x24(a0)                 | +0b8
        move.b  #0x3,0x98(a0)                   | +0be
        bra.w   Anim_ScriptStepFix_Next_08be12 | +0c4
.L08bdae:
        cmpi.w  #0x0,0x80(a6)                   | +0c8
        bne.w   .L08bdc0                        | +0ce
        bra.w   Anim_ScriptStepFix_Next_08be12 | +0d2
        bra.w   Anim_ScriptStepFix_Next_08be12 | +0d6
.L08bdc0:
        movea.l 0x86(a6),a1                     | +0da
        addi.l  #0x40,0x86(a6)                  | +0de
        move.w  0x80(a6),d0                     | +0e6
        andi.w  #0xff00,d0                      | +0ea
        move.w  #0x4,d1                         | +0ee
        cmpi.w  #0x500,d0                       | +0f2
        bne.w   .L08bde4                        | +0f6
        move.w  #0x9,d1                         | +0fa
.L08bde4:
        cmpi.w  #0x600,d0                       | +0fe
        bne.w   .L08bdf0                        | +102
        move.w  #0x9,d1                         | +106
.L08bdf0:
        cmpi.w  #0x700,d0                       | +10a
        bne.w   .L08bdfc                        | +10e
        move.w  #0x9,d1                         | +112
.L08bdfc:
        cmpi.w  #0xd00,d0                       | +116
        bcs.w   .L08be08                        | +11a
        move.w  #0x9,d1                         | +11e
.L08be08:
        move.w  0x80(a6),d0                     | +122
        jsr     0x47872.l                       | +126
        .global Anim_ScriptStepFix_Next_08be12
Anim_ScriptStepFix_Next_08be12:
        addq.l  #0x8,0x82(a6)                   | +12c
        bra.w   .L08bcfe                        | +130
.L08be1a:
        andi.b  #0xee,ccr                       | +134
        bra.w   SetXNMid_08be26                 | +138

| ----------------------------------------------------------------------------
|  Anim_ScriptStepFix_B_08be28  @ $08BE28  (360 B)
| ----------------------------------------------------------------------------
        .section .text.Anim_ScriptStepFix_B_08be28, "ax", @progbits
        .global Anim_ScriptStepFix_B_08be28
Anim_ScriptStepFix_B_08be28:
        cmpi.w  #0xffff,0x70(a6)                | +000
        beq.w   SetXN_08bf90                    | +006
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   .L08bf88                        | +014
.L08be40:
        movea.l 0x7c(a6),a1                     | +018
        adda.l  0x82(a6),a1                     | +01c
        move.w  (a1),0x70(a6)                   | +020
        cmpi.w  #0xffff,0x70(a6)                | +024
        beq.w   .L08bf88                        | +02a
        move.w  0x2(a1),0x80(a6)                | +02e
        cmpi.l  #0x0,0x4(a1)                    | +034
        beq.w   .L08be72                        | +03c
        movea.l 0x4(a1),a0                      | +040
        jsr     0x28cd4.l                       | +044
.L08be72:
        cmpi.w  #0x1,0x80(a6)                   | +04a
        bne.w   .L08be96                        | +050
        movea.w #0x7084,a1                      | +054
        move.w  #0x2320,d0                      | +058
        move.w  #0x20,d1                        | +05c
        move.w  #0x8,d2                         | +060
        jsr     0x5da9c.l                       | +064
        bra.w   Anim_ScriptStepFix_Next_08be12 | +06a
.L08be96:
        cmpi.w  #0xd,0x80(a6)                   | +06e
        bne.w   .L08bebe                        | +074
        move.l  0x86(a6),d0                     | +078
        andi.l  #0xf00f,d0                      | +07c
        addi.l  #0xa0,d0                        | +082
        addi.l  #0x2,d0                         | +088
        move.l  d0,0x86(a6)                     | +08e
        bra.w   .L08bf80                        | +092
.L08bebe:
        cmpi.w  #0x2,0x80(a6)                   | +096
        bne.w   .L08bed6                        | +09c
        lea     Icon_Anchor_Drop_08c730(pc),a1  | +0a0
        jsr     0x4ae.l                         | +0a4
        bra.w   Anim_ScriptStepFix_Next_08be12 | +0aa
.L08bed6:
        cmpi.w  #0x3,0x80(a6)                   | +0ae
        bne.w   .L08bf02                        | +0b4
        lea     0x2ae50.l,a1                    | +0b8
        jsr     0x4ae.l                         | +0be
        move.w  #0xa0,0x22(a0)                  | +0c4
        move.w  #0x1ff,0x24(a0)                 | +0ca
        move.b  #0x3,0x98(a0)                   | +0d0
        bra.w   Anim_ScriptStepFix_Next_08be12 | +0d6
.L08bf02:
        cmpi.w  #0x0,0x80(a6)                   | +0da
        bne.w   .L08bf14                        | +0e0
        bra.w   .L08bf80                        | +0e4
        bra.w   .L08bf80                        | +0e8
.L08bf14:
        move.l  0x86(a6),d0                     | +0ec
        andi.l  #0xffff,d0                      | +0f0
        addi.l  #0x20,0x86(a6)                  | +0f6
        move.w  0x80(a6),d1                     | +0fe
        ori.w   #0x2300,d1                      | +102
        movem.w d0-d1,0x3c0000.l                | +106
        cmpi.w  #0xe6,0x80(a6)                  | +10e
        bne.w   .L08bf56                        | +114
        addi.l  #0x1,d0                         | +118
        addi.w  #0x10,d1                        | +11e
        movem.w d0-d1,0x3c0000.l                | +122
        bra.w   .L08bf80                        | +12a
.L08bf56:
        cmpi.w  #0xe7,0x80(a6)                  | +12e
        bcs.w   .L08bf80                        | +134
        cmpi.w  #0xef,0x80(a6)                  | +138
        bhi.w   .L08bf80                        | +13e
        subi.l  #0x1,d0                         | +142
        subi.w  #0x10,d1                        | +148
        movem.w d0-d1,0x3c0000.l                | +14c
        bra.w   .L08bf80                        | +154
.L08bf80:
        addq.l  #0x8,0x82(a6)                   | +158
        bra.w   .L08be40                        | +15c
.L08bf88:
        andi.b  #0xee,ccr                       | +160
        bra.w   SetXNMid_08bf94                 | +164

| ----------------------------------------------------------------------------
|  Anim_ScriptStep_Alt_08bf96  @ $08BF96  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Anim_ScriptStep_Alt_08bf96, "ax", @progbits
        .global Anim_ScriptStep_Alt_08bf96
Anim_ScriptStep_Alt_08bf96:
        cmpi.w  #0xffff,0x70(a6)                | +000
        beq.w   SetXN_08c002                    | +006
        subq.w  #0x1,0x70(a6)                   | +00a
        cmpi.w  #0x0,0x70(a6)                   | +00e
        bgt.w   .L08bff4                        | +014
        movea.l 0x7c(a6),a1                     | +018
        adda.l  0x78(a6),a1                     | +01c
        move.w  (a1),0x70(a6)                   | +020
        cmpi.w  #0xffff,0x70(a6)                | +024
        beq.w   .L08bfde                        | +02a
        move.w  0x2(a1),0x76(a6)                | +02e
        move.b  0x4(a1),0x92(a6)                | +034
        move.b  0x4(a1),0x93(a6)                | +03a
        addq.l  #0x8,0x78(a6)                   | +040
        bra.w   .L08bff0                        | +044
.L08bfde:
        move.w  #0x0,0x76(a6)                   | +048
        move.b  #0x0,0x92(a6)                   | +04e
        move.b  #0x0,0x93(a6)                   | +054
.L08bff0:
        bra.w   .L08bffa                        | +05a
.L08bff4:
        move.b  #0x0,0x93(a6)                   | +05e
.L08bffa:
        andi.b  #0xee,ccr                       | +064
        bra.w   SetXNMid_08c006                 | +068

| ----------------------------------------------------------------------------
|  Icon_Base_08c2b8  @ $08C2B8  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Base_08c2b8, "ax", @progbits
        .global Icon_Base_08c2b8
Icon_Base_08c2b8:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xb1,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2e0a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0x40,0x22(a6)                  | +022
        move.w  #0x12b,0x24(a6)                 | +028
        clr.w   0x26(a6)                        | +02e
        lea     .L08c2f0(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08c2f0:
        movea.l 0xc(a6),a0                      | +038
        move.b  0x92(a0),d0                     | +03c
        andi.l  #0xf,d0                         | +040
        movea.l #0x2f2dca,a0                    | +046
        lsl.w   #0x2,d0                         | +04c
        movea.l (a0,d0.w),a0                    | +04e
        cmpa.l  #0xffffffff,a0                  | +052
        beq.w   JsrAbsThunk_08c31a              | +058
        jsr     0x28cd4.l                       | +05c

| ----------------------------------------------------------------------------
|  Icon_Slot1_08c322  @ $08C322  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot1_08c322, "ax", @progbits
        .global Icon_Slot1_08c322
Icon_Slot1_08c322:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xae,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2ed6.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0x70,0x22(a6)                  | +022
        move.w  #0x12b,0x24(a6)                 | +028
        clr.w   0x26(a6)                        | +02e
        lea     Icon_Slot1_Lit_08c4fa(pc),a1    | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        move.b  #0x10,0x8a(a6)                  | +042
        move.w  #0x0,0x76(a6)                   | +048
        lea     Icon_Slot_Run_08c4a0(pc),a1 | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  Icon_Slot2_08c37e  @ $08C37E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot2_08c37e, "ax", @progbits
        .global Icon_Slot2_08c37e
Icon_Slot2_08c37e:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xaf,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2ed6.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0xa0,0x22(a6)                  | +022
        move.w  #0x12b,0x24(a6)                 | +028
        clr.w   0x26(a6)                        | +02e
        lea     Icon_Slot2_Lit_08c526(pc),a1    | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        move.b  #0x20,0x8a(a6)                  | +042
        move.w  #0x0,0x76(a6)                   | +048
        lea     Icon_Slot_Run_08c4a0(pc),a1 | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  Icon_Slot3_08c3da  @ $08C3DA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot3_08c3da, "ax", @progbits
        .global Icon_Slot3_08c3da
Icon_Slot3_08c3da:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xb0,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2ed6.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.w  #0xd0,0x22(a6)                  | +022
        move.w  #0x12b,0x24(a6)                 | +028
        clr.w   0x26(a6)                        | +02e
        lea     Icon_Slot3_Lit_08c552(pc),a1    | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        move.b  #0x40,0x8a(a6)                  | +042
        move.w  #0x0,0x76(a6)                   | +048
        lea     Icon_Slot_Run_08c4a0(pc),a1 | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  Icon_Slot4_08c436  @ $08C436  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot4_08c436, "ax", @progbits
        .global Icon_Slot4_08c436
Icon_Slot4_08c436:
        movea.w #0x73d7,a1                      | +000
        move.w  #0x2e80,d0                      | +004
        move.w  #0x4,d1                         | +008
        move.w  #0x4,d2                         | +00c
        jsr     0x5da56.l                       | +010
        move.w  #0xe000,0x38(a6)                | +016
        bset    #0x6,0x12(a6)                   | +01c
        move.w  #0xb2,d1                        | +022
        jsr     0x236e.l                        | +026
        lea     0x2f2ed6.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0x100,0x22(a6)                 | +038
        move.w  #0x12b,0x24(a6)                 | +03e
        clr.w   0x26(a6)                        | +044
        lea     Icon_Slot4_Lit_08c57e(pc),a1    | +048
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd02.l                       | +052
        move.b  #0x80,0x8a(a6)                  | +058
        move.w  #0x0,0x76(a6)                   | +05e
        lea     Icon_Slot_Run_08c4a0(pc),a1 | +064
        move.l  a1,(a6)                         | +068
        .global Icon_Slot_Run_08c4a0
Icon_Slot_Run_08c4a0:
        movea.l 0xc(a6),a0                      | +06a
        move.b  0x93(a0),d0                     | +06e
        and.b   0x8a(a6),d0                     | +072
        beq.w   Icon_Slot_Tail_08c4ca           | +076
        move.w  #0x1,0x76(a6)                   | +07a
        lea     0x2f2ee2.l,a0                   | +080
        jsr     0x28cd4.l                       | +086

| ----------------------------------------------------------------------------
|  Icon_Slot_Tail_08c4ca  @ $08C4CA  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot_Tail_08c4ca, "ax", @progbits
        .global Icon_Slot_Tail_08c4ca
Icon_Slot_Tail_08c4ca:
        move.b  0x92(a0),d0                     | +000
        and.b   0x8a(a6),d0                     | +004
        bne.w   JsrAbsThunk_08c4f2              | +008
        cmpi.w  #0x1,0x76(a6)                   | +00c
        bne.w   JsrAbsThunk_08c4f2              | +012
        move.w  #0x0,0x76(a6)                   | +016
        lea     0x2f2f02.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022

| ----------------------------------------------------------------------------
|  Icon_Slot1_Lit_08c4fa  @ $08C4FA  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot1_Lit_08c4fa, "ax", @progbits
        .global Icon_Slot1_Lit_08c4fa
Icon_Slot1_Lit_08c4fa:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xae,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2f22.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     JsrAbsThunk_08c5aa(pc),a1       | +022
        move.l  a1,(a6)                         | +026
        bra.w   JsrAbsThunk_08c5aa              | +028

| ----------------------------------------------------------------------------
|  Icon_Slot2_Lit_08c526  @ $08C526  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot2_Lit_08c526, "ax", @progbits
        .global Icon_Slot2_Lit_08c526
Icon_Slot2_Lit_08c526:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xaf,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2f2e.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     JsrAbsThunk_08c5aa(pc),a1       | +022
        move.l  a1,(a6)                         | +026
        bra.w   JsrAbsThunk_08c5aa              | +028

| ----------------------------------------------------------------------------
|  Icon_Slot3_Lit_08c552  @ $08C552  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot3_Lit_08c552, "ax", @progbits
        .global Icon_Slot3_Lit_08c552
Icon_Slot3_Lit_08c552:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xb0,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2f3a.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     JsrAbsThunk_08c5aa(pc),a1       | +022
        move.l  a1,(a6)                         | +026
        bra.w   JsrAbsThunk_08c5aa              | +028

| ----------------------------------------------------------------------------
|  Icon_Slot4_Lit_08c57e  @ $08C57E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Slot4_Lit_08c57e, "ax", @progbits
        .global Icon_Slot4_Lit_08c57e
Icon_Slot4_Lit_08c57e:
        move.w  #0xe000,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0xb2,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f2f46.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     JsrAbsThunk_08c5aa(pc),a1       | +022
        move.l  a1,(a6)                         | +026
        bra.w   JsrAbsThunk_08c5aa              | +028

| ----------------------------------------------------------------------------
|  Icon_Anchor_Init_08c5b2  @ $08C5B2  (190 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Anchor_Init_08c5b2, "ax", @progbits
        .global Icon_Anchor_Init_08c5b2
Icon_Anchor_Init_08c5b2:
        move.w  #0x8000,0x38(a6)                | +000
        cmpi.b  #0x0,0x10fd83.l                 | +006
        bne.w   .L08c5ce                        | +00e
        move.b  #0x0,0x8a(a6)                   | +012
        bra.w   .L08c614                        | +018
.L08c5ce:
        cmpi.b  #0x2,0x10fd83.l                 | +01c
        bne.w   .L08c60e                        | +024
        move.b  0x10fd92.l,d0                   | +028
        cmpi.b  #0x2,d0                         | +02e
        bne.w   .L08c5ee                        | +032
        move.b  #0x3,0x8a(a6)                   | +036
.L08c5ee:
        cmpi.b  #0x1,d0                         | +03c
        bne.w   .L08c5fc                        | +040
        move.b  #0x2,0x8a(a6)                   | +044
.L08c5fc:
        cmpi.b  #0x0,d0                         | +04a
        bne.w   .L08c60a                        | +04e
        move.b  #0x1,0x8a(a6)                   | +052
.L08c60a:
        bra.w   .L08c614                        | +058
.L08c60e:
        move.b  #0x1,0x8a(a6)                   | +05c
.L08c614:
        move.w  #0x97,d1                        | +062
        jsr     0x236e.l                        | +066
        lea     0x2f2f52.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        move.w  #0xe0,0x22(a6)                  | +078
        move.w  #0x160,0x24(a6)                 | +07e
        clr.w   0x26(a6)                        | +084
        jsr     0x267e2.l                       | +088
        clr.l   0x2c(a6)                        | +08e
        move.l  #0x7084,0x86(a6)                | +092
        move.w  #0x0,0x70(a6)                   | +09a
        move.w  #0x0,0x80(a6)                   | +0a0
        move.l  #0x0,0x82(a6)                   | +0a6
        lea     0x2f0056.l,a1                   | +0ae
        move.l  a1,0x7c(a6)                     | +0b4
        move.w  #0x0,0x76(a6)                   | +0b8

| ----------------------------------------------------------------------------
|  Icon_Anchor_Run_08c678  @ $08C678  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Anchor_Run_08c678, "ax", @progbits
        .global Icon_Anchor_Run_08c678
Icon_Anchor_Run_08c678:
        movea.l 0xc(a6),a0                      | +000
        cmpi.w  #0x0,0x76(a0)                   | +004
        beq.w   .L08c70c                        | +00a
        clr.l   d0                              | +00e
        move.w  0x76(a0),d0                     | +010
        cmp.w   0x76(a6),d0                     | +014
        beq.w   .L08c6f6                        | +018
        move.w  d0,0x76(a6)                     | +01c
        cmpi.b  #0x0,0x8a(a6)                   | +020
        bne.w   .L08c6ac                        | +026
        lea     0x2efacc.l,a1                   | +02a
        bra.w   .L08c6dc                        | +030
.L08c6ac:
        cmpi.b  #0x3,0x8a(a6)                   | +034
        bne.w   .L08c6bc                        | +03a
        lea     0x2efeec.l,a1                   | +03e
.L08c6bc:
        cmpi.b  #0x2,0x8a(a6)                   | +044
        bne.w   .L08c6cc                        | +04a
        lea     0x2efd8c.l,a1                   | +04e
.L08c6cc:
        cmpi.b  #0x1,0x8a(a6)                   | +054
        bne.w   .L08c6dc                        | +05a
        lea     0x2efc2c.l,a1                   | +05e
.L08c6dc:
        lsl.l   #0x3,d0                         | +064
        adda.l  d0,a1                           | +066
        move.l  (a1)+,0x7c(a6)                  | +068
        move.l  (a1),0x86(a6)                   | +06c
        move.l  #0x0,0x82(a6)                   | +070
        move.w  #0x0,0x70(a6)                   | +078
.L08c6f6:
        cmpi.b  #0x0,0x8a(a6)                   | +07e
        bne.w   .L08c708                        | +084
        jsr     Anim_ScriptStepFix_A_08bce6(pc) | +088
        bra.w   .L08c70c                        | +08c
.L08c708:
        jsr     Anim_ScriptStepFix_B_08be28(pc) | +090
.L08c70c:
        jsr     0x27cee.l                       | +094
        jsr     0x28d70.l                       | +09a
        cmpi.w  #0x150,0x22(a6)                 | +0a0
        bls.w   Jsr5B6Rts_08c72e                | +0a6

| ----------------------------------------------------------------------------
|  Icon_Anchor_Drop_08c730  @ $08C730  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Icon_Anchor_Drop_08c730, "ax", @progbits
        .global Icon_Anchor_Drop_08c730
Icon_Anchor_Drop_08c730:
        move.w  #0x140,0x22(a6)                 | +000
        move.w  #0x180,0x24(a6)                 | +006
        move.w  #0x179,d1                       | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x8000,0x38(a6)                | +016
        jsr     0x267e2.l                       | +01c
        move.w  #0xf800,0x28(a6)                | +022
        lea     0x2ef80e.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     0x2ef84c.l,a0                   | +034
        move.l  a0,0x4c(a6)                     | +03a
        jsr     0x283ca.l                       | +03e
        lea     .L08c77a(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L08c77a:
        jsr     0x28d70.l                       | +04a
        jsr     0x27cee.l                       | +050
        jsr     0x283d8.l                       | +056
        btst    #0x1,0x13(a6)                   | +05c
        bne.w   .L08c7ac                        | +062
        movea.l #0xffffffff,a0                  | +066
        lea     0x2ef8a0.l,a0                   | +06c
        jsr     0x5dd56.l                       | +072
        bcc.w   .L08c7c4                        | +078
.L08c7ac:
        lea     0x77f6a.l,a1                    | +07c
        jsr     0x4ae.l                         | +082
        jsr     0x5dd02.l                       | +088
        jmp     0x518.l                         | +08e
.L08c7c4:
        rts                                     | +094

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08c7c6  @ $08C7C6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08c7c6, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08c7c6
Entity_CmpPrioWithSibling_08c7c6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08c7dc                    | +00c

| ----------------------------------------------------------------------------
|  Cut_SetMode2_08c7e2  @ $08C7E2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_SetMode2_08c7e2, "ax", @progbits
        .global Cut_SetMode2_08c7e2
Cut_SetMode2_08c7e2:
        move.b  #0x2,0x10e2ef.l                 | +000
        jmp     0x518.l                         | +008

| ----------------------------------------------------------------------------
|  Rts_08c7f0  @ $08C7F0  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08c7f0, "ax", @progbits
        .global Rts_08c7f0
Rts_08c7f0:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Cut_SetVariant1_08c800  @ $08C800  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_SetVariant1_08c800, "ax", @progbits
        .global Cut_SetVariant1_08c800
Cut_SetVariant1_08c800:
        move.b  #0x1,0x10e2ee.l                 | +000
        jmp     0x518.l                         | +008

| ----------------------------------------------------------------------------
|  Rts_08c80e  @ $08C80E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08c80e, "ax", @progbits
        .global Rts_08c80e
Rts_08c80e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Cut_SetVariant2_08c810  @ $08C810  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_SetVariant2_08c810, "ax", @progbits
        .global Cut_SetVariant2_08c810
Cut_SetVariant2_08c810:
        move.b  #0x2,0x10e2ee.l                 | +000
        jmp     0x518.l                         | +008

| ----------------------------------------------------------------------------
|  Rts_08c81e  @ $08C81E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08c81e, "ax", @progbits
        .global Rts_08c81e
Rts_08c81e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Cut_InstallListByVariant_08c820  @ $08C820  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_InstallListByVariant_08c820, "ax", @progbits
        .global Cut_InstallListByVariant_08c820
Cut_InstallListByVariant_08c820:
        cmpi.b  #0x2,0x10e2ee.l                 | +000
        bne.w   .L08c838                        | +008
        lea     0x2f3688.l,a0                   | +00c
        jmp     0x5db58.l                       | +012
.L08c838:
        cmpi.b  #0x1,0x10e2ee.l                 | +018
        bne.w   .L08c850                        | +020
        lea     0x2f35fe.l,a0                   | +024
        jmp     0x5db58.l                       | +02a
.L08c850:
        lea     0x2f3574.l,a0                   | +030
        jmp     0x5db58.l                       | +036

| ----------------------------------------------------------------------------
|  Cut_Watcher_Init_08c864  @ $08C864  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Watcher_Init_08c864, "ax", @progbits
        .global Cut_Watcher_Init_08c864
Cut_Watcher_Init_08c864:
        move.w  #0x7408,0x22(a6)                | +000
        clr.b   0x10e2ef.l                      | +006
        move.w  #0xffff,0x70(a6)                | +00c
        clr.w   0x30(a6)                        | +012
        lea     Cut_Watcher_Run_08c880(pc),a1   | +016
        move.l  a1,(a6)                         | +01a

| ----------------------------------------------------------------------------
|  Cut_Watcher_Run_08c880  @ $08C880  (122 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Watcher_Run_08c880, "ax", @progbits
        .global Cut_Watcher_Run_08c880
Cut_Watcher_Run_08c880:
        move.w  0x106f5c.l,d0                   | +000
        cmp.w   0x70(a6),d0                     | +006
        beq.w   .L08c89c                        | +00a
        move.w  d0,0x70(a6)                     | +00e
        move.w  #0x78,0x30(a6)                  | +012
        bra.w   .L08c8e8                        | +018
.L08c89c:
        tst.w   0x30(a6)                        | +01c
        beq.w   .L08c8ac                        | +020
        subq.w  #0x1,0x30(a6)                   | +024
        bra.w   .L08c8e8                        | +028
.L08c8ac:
        move.w  0x106e88.l,d0                   | +02c
        or.w    0x106e8a.l,d0                   | +032
        bne.w   .L08c8e8                        | +038
        jsr     0x5e1aa.l                       | +03c
        bcs.w   .L08c8e8                        | +042
        jsr     0x43ce2.l                       | +046
        bcs.w   .L08c8e8                        | +04c
        tst.b   0x106ed3.l                      | +050
        beq.w   .L08c8e8                        | +056
        move.w  #0x78,0x30(a6)                  | +05a
        move.b  #0x2,0x10e2ef.l                 | +060
.L08c8e8:
        tst.b   0x10e2ef.l                      | +068
        bne.w   .L08c8f4                        | +06e
        rts                                     | +072
.L08c8f4:
        lea     Cut_Watcher_Play_08c8fa(pc),a1  | +074
        move.l  a1,(a6)                         | +078

| ----------------------------------------------------------------------------
|  Cut_Watcher_Play_08c8fa  @ $08C8FA  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Watcher_Play_08c8fa, "ax", @progbits
        .global Cut_Watcher_Play_08c8fa
Cut_Watcher_Play_08c8fa:
        bsr.w   Cut_InstallListByVariant_08c820 | +000
        move.w  #0x10d5,d0                      | +004
        jsr     0x2352.l                        | +008
        lea     .L08c90e(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L08c90e:
        jsr     0x5db6a.l                       | +014
        bcc.w   SetHandlerRts_08c938            | +01a
        subq.b  #0x1,0x10e2ef.l                 | +01e
        bne.w   SetTaskHandler_08c932           | +024
        jsr     0x5dbdc.l                       | +028
        lea     Cut_Watcher_Run_08c880(pc),a1   | +02e
        move.l  a1,(a6)                         | +032
        bra.w   SetHandlerRts_08c938            | +034

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08c93a  @ $08C93A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08c93a, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08c93a
Entity_CmpPrioWithSibling_08c93a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08c950                    | +00c

| ----------------------------------------------------------------------------
|  Cut_Fade_08c956  @ $08C956  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Fade_08c956, "ax", @progbits
        .global Cut_Fade_08c956
Cut_Fade_08c956:
        move.w  #0x78,0x70(a6)                  | +000
        movea.w #0x70ae,a1                      | +006
        lea     0x2f4a12.l,a2                   | +00a
        move.w  #0x4,d1                         | +010
        jsr     0x4784c.l                       | +014
        lea     .L08c976(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L08c976:
        subq.w  #0x1,0x70(a6)                   | +020
        cmpi.w  #0x0,0x70(a6)                   | +024
        bge.w   .L08c9a4                        | +02a
        clr.b   0x106ed2.l                      | +02e
        movea.w #0x70ae,a1                      | +034
        lea     0x2f4a22.l,a2                   | +038
        move.w  #0x4,d1                         | +03e
        jsr     0x4784c.l                       | +042
        jmp     0x518.l                         | +048
.L08c9a4:
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Fix_TextRow_Draw_08c9a6  @ $08C9A6  (182 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_TextRow_Draw_08c9a6, "ax", @progbits
        .global Fix_TextRow_Draw_08c9a6
Fix_TextRow_Draw_08c9a6:
        move.w  #0x709a,d0                      | +000
        movea.l 0x80(a6),a1                     | +004
        move.l  a1,0x84(a6)                     | +008
        cmpa.l  #0x0,a1                         | +00c
        bne.w   .L08c9be                        | +012
        rts                                     | +016
.L08c9be:
        move.w  (a1)+,d1                        | +018
        cmpi.w  #0xfffe,d1                      | +01a
        bne.w   .L08c9ce                        | +01e
        move.w  #0x709c,d0                      | +022
        move.w  (a1)+,d1                        | +026
.L08c9ce:
        cmpi.w  #0x0,d1                         | +028
        bne.w   .L08c9dc                        | +02c
        move.l  a1,0x80(a6)                     | +030
        rts                                     | +034
.L08c9dc:
        cmpi.w  #0xffff,d1                      | +036
        bne.w   .L08c9ee                        | +03a
        clr.l   0x80(a6)                        | +03e
        clr.l   0x84(a6)                        | +042
        rts                                     | +046
.L08c9ee:
        cmpi.w  #0xfffd,d1                      | +048
        bne.w   .L08c9f8                        | +04c
        bra.b   .L08c9be                        | +050
.L08c9f8:
        move.w  d1,d2                           | +052
        andi.w  #0xf00,d2                       | +054
        move.w  #0x9000,d3                      | +058
        cmpi.w  #0xb00,d2                       | +05c
        bne.w   .L08ca0e                        | +060
        move.w  #0x4000,d3                      | +064
.L08ca0e:
        cmpi.w  #0xc00,d2                       | +068
        bne.w   .L08ca1a                        | +06c
        move.w  #0x4000,d3                      | +070
.L08ca1a:
        or.w    d3,d1                           | +074
        movem.w d0-d1,0x3c0000.l                | +076
        addi.w  #0x20,d0                        | +07e
        addq.w  #0x1,d1                         | +082
        movem.w d0-d1,0x3c0000.l                | +084
        subi.w  #0x1f,d0                        | +08c
        addi.w  #0xf,d1                         | +090
        movem.w d0-d1,0x3c0000.l                | +094
        addi.w  #0x20,d0                        | +09c
        addq.w  #0x1,d1                         | +0a0
        movem.w d0-d1,0x3c0000.l                | +0a2
        subi.w  #0x21,d0                        | +0aa
        addi.w  #0x40,d0                        | +0ae
        bra.w   .L08c9be                        | +0b2

| ----------------------------------------------------------------------------
|  Rts_08ca5c  @ $08CA5C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08ca5c, "ax", @progbits
        .global Rts_08ca5c
Rts_08ca5c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Fix_TextRow_Clear_08ca5e  @ $08CA5E  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_TextRow_Clear_08ca5e, "ax", @progbits
        .global Fix_TextRow_Clear_08ca5e
Fix_TextRow_Clear_08ca5e:
        movea.w #0x701a,a1                      | +000
        move.w  #0x20,d0                        | +004
        move.w  #0x28,d1                        | +008
        move.w  #0x2,d2                         | +00c
        jsr     0x5da9c.l                       | +010
        move.w  #0x709a,d0                      | +016
        movea.l 0x80(a6),a1                     | +01a
        cmpa.l  #0x0,a1                         | +01e
        bne.w   .L08ca88                        | +024
        rts                                     | +028
.L08ca88:
        move.w  (a1)+,d1                        | +02a
        cmpi.w  #0xfffe,d1                      | +02c
        bne.w   .L08ca94                        | +030
        rts                                     | +034
.L08ca94:
        cmpi.w  #0x0,d1                         | +036
        bne.w   .L08ca9e                        | +03a
        rts                                     | +03e
.L08ca9e:
        cmpi.w  #0xffff,d1                      | +040
        bne.w   .L08caa8                        | +044
        rts                                     | +048
.L08caa8:
        cmpi.w  #0xfffd,d1                      | +04a
        bne.w   .L08cab2                        | +04e
        rts                                     | +052
.L08cab2:
        move.w  d1,d2                           | +054
        andi.w  #0xf00,d2                       | +056
        move.w  #0x9000,d3                      | +05a
        cmpi.w  #0xb00,d2                       | +05e
        bne.w   .L08cac8                        | +062
        move.w  #0x4000,d3                      | +066
.L08cac8:
        cmpi.w  #0xc00,d2                       | +06a
        bne.w   .L08cad4                        | +06e
        move.w  #0x4000,d3                      | +072
.L08cad4:
        or.w    d3,d1                           | +076
        movem.w d0-d1,0x3c0000.l                | +078
        addi.w  #0x20,d0                        | +080
        addq.w  #0x1,d1                         | +084
        movem.w d0-d1,0x3c0000.l                | +086
        subi.w  #0x1f,d0                        | +08e
        addi.w  #0xf,d1                         | +092
        movem.w d0-d1,0x3c0000.l                | +096
        addi.w  #0x20,d0                        | +09e
        addq.w  #0x1,d1                         | +0a2
        movem.w d0-d1,0x3c0000.l                | +0a4
        subi.w  #0x21,d0                        | +0ac
        addi.w  #0x40,d0                        | +0b0
        bra.w   .L08ca88                        | +0b4

| ----------------------------------------------------------------------------
|  Rts_08cb16  @ $08CB16  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08cb16, "ax", @progbits
        .global Rts_08cb16
Rts_08cb16:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Fix_TextRow_DrawOne_08cb18  @ $08CB18  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Fix_TextRow_DrawOne_08cb18, "ax", @progbits
        .global Fix_TextRow_DrawOne_08cb18
Fix_TextRow_DrawOne_08cb18:
        cmpi.l  #0x0,0x84(a6)                   | +000
        bne.w   .L08cb26                        | +008
        rts                                     | +00c
.L08cb26:
        movea.l 0x84(a6),a1                     | +00e
        move.w  (a1),d1                         | +012
        cmpi.w  #0x0,d1                         | +014
        bne.w   .L08cb36                        | +018
        rts                                     | +01c
.L08cb36:
        cmpi.w  #0xffff,d1                      | +01e
        bne.w   .L08cb40                        | +022
        rts                                     | +026
.L08cb40:
        cmpi.w  #0xfffe,d1                      | +028
        bne.w   .L08cb5a                        | +02c
        move.w  #0x709c,d0                      | +030
        move.w  d0,0x90(a6)                     | +034
        addi.l  #0x2,0x84(a6)                   | +038
        bra.b   Fix_TextRow_DrawOne_08cb18      | +040
.L08cb5a:
        cmpi.w  #0xfffd,d1                      | +042
        bne.w   .L08cb70                        | +046
        jsr     Fix_TextRow_Clear_08ca5e(pc)    | +04a
        addi.l  #0x2,0x84(a6)                   | +04e
        bra.b   Fix_TextRow_DrawOne_08cb18      | +056
.L08cb70:
        movea.l 0x84(a6),a1                     | +058
        move.w  (a1),d1                         | +05c
        ori.w   #0xb000,d1                      | +05e
        move.w  0x90(a6),d0                     | +062
        movem.w d0-d1,0x3c0000.l                | +066
        addi.w  #0x20,d0                        | +06e
        addq.w  #0x1,d1                         | +072
        movem.w d0-d1,0x3c0000.l                | +074
        subi.w  #0x1f,d0                        | +07c
        addi.w  #0xf,d1                         | +080
        movem.w d0-d1,0x3c0000.l                | +084
        addi.w  #0x20,d0                        | +08c
        addq.w  #0x1,d1                         | +090
        movem.w d0-d1,0x3c0000.l                | +092
        subi.w  #0x21,d0                        | +09a
        addi.w  #0x40,0x90(a6)                  | +09e
        addi.l  #0x2,0x84(a6)                   | +0a4
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  Cut_TextPanel_08cbc6  @ $08CBC6  (120 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_TextPanel_08cbc6, "ax", @progbits
        .global Cut_TextPanel_08cbc6
Cut_TextPanel_08cbc6:
        lea     0x2f4712.l,a1                   | +000
        move.l  a1,0x80(a6)                     | +006
        move.w  #0xffff,0x38(a6)                | +00a
        bset    #0x6,0x12(a6)                   | +010
        move.w  #0x15e,d1                       | +016
        jsr     0x236e.l                        | +01a
        lea     0x2f3732.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        move.w  #0xa0,0x22(a6)                  | +02c
        move.w  #0x121,0x24(a6)                 | +032
        clr.w   0x26(a6)                        | +038
        move.w  #0x709a,d0                      | +03c
        move.w  d0,0x90(a6)                     | +040
        lea     .L08cc10(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L08cc10:
        jsr     0x28d70.l                       | +04a
        move.b  0x106f29.l,d0                   | +050
        bne.w   Cut_TextPanel_Type_08cc44       | +056
        movea.w #0x701a,a1                      | +05a
        move.w  #0x20,d0                        | +05e
        move.w  #0x28,d1                        | +062
        move.w  #0x4,d2                         | +066
        jsr     0x5da9c.l                       | +06a
        jsr     Fix_TextRow_Draw_08c9a6(pc)     | +070
        move.w  #0x709a,d0                      | +074

| ----------------------------------------------------------------------------
|  Cut_TextPanel_Type_08cc44  @ $08CC44  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_TextPanel_Type_08cc44, "ax", @progbits
        .global Cut_TextPanel_Type_08cc44
Cut_TextPanel_Type_08cc44:
        cmpi.b  #0x20,d0                        | +000
        bhi.w   Stub_0008CC64                   | +004
        move.w  d0,d4                           | +008
        move.w  #0x1,d5                         | +00a
        bra.b   .L08cc56                        | +00e
.L08cc54:
        addq.w  #0x1,d5                         | +010
.L08cc56:
        cmp.w   d4,d5                           | +012
        bgt.w   .L08cc62                        | +014
        jsr     Fix_TextRow_DrawOne_08cb18(pc)  | +018
        bra.b   .L08cc54                        | +01c
.L08cc62:
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Cut_Banner_08cc66  @ $08CC66  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Banner_08cc66, "ax", @progbits
        .global Cut_Banner_08cc66
Cut_Banner_08cc66:
        move.w  #0xf0,0x70(a6)                  | +000
        move.w  #0xffff,0x38(a6)                | +006
        bset    #0x6,0x12(a6)                   | +00c
        move.w  #0x15e,d1                       | +012
        jsr     0x236e.l                        | +016
        lea     0x2f373e.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        move.w  #0xa0,0x22(a6)                  | +028
        move.w  #0xf1,0x24(a6)                  | +02e
        clr.w   0x26(a6)                        | +034
        lea     .L08cca4(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L08cca4:
        subq.w  #0x1,0x70(a6)                   | +03e
        cmpi.w  #0x0,0x70(a6)                   | +042
        bgt.w   JsrAbsThunk_08ccbe              | +048
        jsr     0x46a96.l                       | +04c
        jmp     0x518.l                         | +052

| ----------------------------------------------------------------------------
|  Scroll_StepVelX_08ccc6  @ $08CCC6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Scroll_StepVelX_08ccc6, "ax", @progbits
        .global Scroll_StepVelX_08ccc6
Scroll_StepVelX_08ccc6:
        move.l  0x80(a6),0x106f60.l             | +000
        move.l  0x84(a6),d1                     | +008
        add.l   d1,0x80(a6)                     | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Scroll_StepVelY_08ccd8  @ $08CCD8  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Scroll_StepVelY_08ccd8, "ax", @progbits
        .global Scroll_StepVelY_08ccd8
Scroll_StepVelY_08ccd8:
        move.l  0x88(a6),0x106f64.l             | +000
        move.l  0x8c(a6),d1                     | +008
        add.l   d1,0x88(a6)                     | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Cut_Dropper_08ccea  @ $08CCEA  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Dropper_08ccea, "ax", @progbits
        .global Cut_Dropper_08ccea
Cut_Dropper_08ccea:
        move.w  #0xfffe,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        clr.w   d0                              | +00c
        move.b  0x99(a6),d0                     | +00e
        lsl.w   #0x5,d0                         | +012
        move.w  d0,0x2a(a6)                     | +014
        move.w  #0x136,d1                       | +018
        jsr     0x236e.l                        | +01c
        clr.l   d0                              | +022
        move.b  0x98(a6),d0                     | +024
        cmpi.l  #0xa,d0                         | +028
        bls.w   .L08cd1e                        | +02e
        clr.l   d0                              | +032
.L08cd1e:
        movea.l #0x2f455c,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L08cd3a                        | +046
        jsr     0x28cd4.l                       | +04a
.L08cd3a:
        lea     .L08cd40(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L08cd40:
        jsr     Pos_IntegrateY88_08d2d4(pc)                | +056  -> $08D2D4 (hueco futuro, defsym forward)
        cmpi.b  #0x0,0x98(a6)                   | +05a
        bne.w   .L08cd6a                        | +060
        cmpi.b  #0xb,0x106ece.l                 | +064
        bne.w   .L08cd6a                        | +06c
        cmpi.w  #0x170,0x24(a6)                 | +070
        blt.w   .L08cd6a                        | +076
        lea     Cut_Dropper_WaitScroll_08cda0(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L08cd6a:
        cmpi.b  #0xa,0x98(a6)                   | +080
        bne.w   .L08cd8a                        | +086
        cmpi.w  #0x170,0x24(a6)                 | +08a
        blt.w   .L08cd8a                        | +090
        move.w  #0x78,0x70(a6)                  | +094
        lea     Cut_Dropper_Finish_08cdba(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L08cd8a:
        jsr     Screen_InBoundsY_Latched_08d24c(pc)                | +0a0  -> $08D24C (hueco futuro, defsym forward)
        bcc.w   JsrAbsThunk_08cd98              | +0a4
        jmp     0x518.l                         | +0a8

| ----------------------------------------------------------------------------
|  Cut_Dropper_WaitScroll_08cda0  @ $08CDA0  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Dropper_WaitScroll_08cda0, "ax", @progbits
        .global Cut_Dropper_WaitScroll_08cda0
Cut_Dropper_WaitScroll_08cda0:
        jsr     0x28d70.l                       | +000
        cmpi.w  #0x1000,0x106f5c.l              | +006
        bcs.w   .L08cdb8                        | +00e
        jmp     0x518.l                         | +012
.L08cdb8:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Cut_Dropper_Finish_08cdba  @ $08CDBA  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Dropper_Finish_08cdba, "ax", @progbits
        .global Cut_Dropper_Finish_08cdba
Cut_Dropper_Finish_08cdba:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   JsrAbsThunk_08cdf4              | +00a
        cmpi.b  #0xc,0x106ece.l                 | +00e
        bne.w   .L08cdde                        | +016
        move.b  #0x60,d0                        | +01a
        jsr     0x2308.l                        | +01e
.L08cdde:
        move.w  #0x4,d0                         | +024
        jsr     0x5239e.l                       | +028
        move.w  #0xa0,0x70(a6)                  | +02e
        lea     Cut_Dropper_Exit_08cdfc(pc),a1  | +034
        move.l  a1,(a6)                         | +038

| ----------------------------------------------------------------------------
|  Cut_Dropper_Exit_08cdfc  @ $08CDFC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Dropper_Exit_08cdfc, "ax", @progbits
        .global Cut_Dropper_Exit_08cdfc
Cut_Dropper_Exit_08cdfc:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   JsrAbsThunk_08ce16              | +00a
        clr.b   0x106ed2.l                      | +00e
        jmp     0x518.l                         | +014

| ----------------------------------------------------------------------------
|  Cut_Item_08ce1e  @ $08CE1E  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Cut_Item_08ce1e, "ax", @progbits
        .global Cut_Item_08ce1e
Cut_Item_08ce1e:
        move.w  #0xffff,0x38(a6)                | +000
        bset    #0x6,0x12(a6)                   | +006
        move.w  #0x5f,d1                        | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f3b2c.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L08ce46(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L08ce46:
        jsr     Pos_IntegrateY88_08d2d4(pc)                | +028  -> $08D2D4 (hueco futuro, defsym forward)
        subq.b  #0x1,0x99(a6)                   | +02c
        bne.w   .L08ce58                        | +030
        jmp     0x518.l                         | +034
.L08ce58:
        jsr     Phys_GroundKill_08efb0(pc)        | +03a

| ----------------------------------------------------------------------------
|  SceneB_Init_08ce64  @ $08CE64  (182 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Init_08ce64, "ax", @progbits
        .global SceneB_Init_08ce64
SceneB_Init_08ce64:
        move.w  #0x29,d0                        | +000
        jsr     0x2352.l                        | +004
        clr.w   0x70(a6)                        | +00a
        move.b  #0x6,0x21(a6)                   | +00e
        move.b  #0xe,d0                         | +014
        jsr     0x43568.l                       | +018
        move.w  #0xffff,0x106f5e.l              | +01e
        cmpi.b  #0x1,0x106eae.l                 | +026
        bne.w   .L08cea4                        | +02e
        move.l  #0x1c000,0x106f60.l             | +032
        bra.w   .L08ceae                        | +03c
.L08cea4:
        move.l  #0x18000,0x106f60.l             | +040
.L08ceae:
        move.l  #0x0,0x106f64.l                 | +04a
        jsr     0x46a96.l                       | +054
        jsr     0x2230.l                        | +05a
        lea     Capsule_Fly_08d3b4(pc),a1             | +060  -> $08D3B4 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +064
        clr.b   0x20(a6)                        | +06a
        lea     .L08ced8(pc),a1                 | +06e
        move.l  a1,(a6)                         | +072
.L08ced8:
        jsr     0x437da.l                       | +074
        jsr     0x96a0e.l                       | +07a
        jsr     0x96a80.l                       | +080
        cmpi.w  #0x100,0x106f5c.l               | +086
        bcs.w   SetHandlerRts_08cf20            | +08e
        cmpi.b  #0x1,0x106eae.l                 | +092
        bne.w   .L08cf10                        | +09a
        move.l  #0x1a000,0x106f60.l             | +09e
        bra.w   SetTaskHandler_08cf1a           | +0a8
.L08cf10:
        move.l  #0x16000,0x106f60.l             | +0ac

| ----------------------------------------------------------------------------
|  SceneB_Stage2_08cf22  @ $08CF22  (66 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Stage2_08cf22, "ax", @progbits
        .global SceneB_Stage2_08cf22
SceneB_Stage2_08cf22:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006
        jsr     0x96a80.l                       | +00c
        cmpi.w  #0x1000,0x106f5c.l              | +012
        bcs.w   SetHandlerRts_08cf6a            | +01a
        cmpi.b  #0x1,0x106eae.l                 | +01e
        bne.w   .L08cf5a                        | +026
        move.l  #0x16000,0x106f60.l             | +02a
        bra.w   SetTaskHandler_08cf64           | +034
.L08cf5a:
        move.l  #0x12000,0x106f60.l             | +038

| ----------------------------------------------------------------------------
|  SceneB_Stage3_08cf6c  @ $08CF6C  (66 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Stage3_08cf6c, "ax", @progbits
        .global SceneB_Stage3_08cf6c
SceneB_Stage3_08cf6c:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006
        jsr     0x96a80.l                       | +00c
        cmpi.w  #0x2b94,0x106f5c.l              | +012
        bcs.w   SetHandlerRts_08cfb4            | +01a
        cmpi.b  #0x1,0x106eae.l                 | +01e
        bne.w   .L08cfa4                        | +026
        move.l  #0x16000,0x106f60.l             | +02a
        bra.w   SetTaskHandler_08cfae           | +034
.L08cfa4:
        move.l  #0x12000,0x106f60.l             | +038

| ----------------------------------------------------------------------------
|  SceneB_Stage4_08cfb6  @ $08CFB6  (66 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Stage4_08cfb6, "ax", @progbits
        .global SceneB_Stage4_08cfb6
SceneB_Stage4_08cfb6:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006
        jsr     0x96a80.l                       | +00c
        cmpi.w  #0x2d00,0x106f5c.l              | +012
        bcs.w   SetHandlerRts_08cffe            | +01a
        cmpi.b  #0x1,0x106eae.l                 | +01e
        bne.w   .L08cfee                        | +026
        move.l  #0x14000,0x106f60.l             | +02a
        bra.w   SetTaskHandler_08cff8           | +034
.L08cfee:
        move.l  #0x10000,0x106f60.l             | +038

| ----------------------------------------------------------------------------
|  SceneB_Stage5_08d000  @ $08D000  (66 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Stage5_08d000, "ax", @progbits
        .global SceneB_Stage5_08d000
SceneB_Stage5_08d000:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006
        jsr     0x96a80.l                       | +00c
        cmpi.w  #0x3000,0x106f5c.l              | +012
        bcs.w   SetHandlerRts_08d048            | +01a
        cmpi.b  #0x1,0x106eae.l                 | +01e
        bne.w   .L08d038                        | +026
        move.l  #0x26000,0x106f60.l             | +02a
        bra.w   SetTaskHandler_08d042           | +034
.L08d038:
        move.l  #0x20000,0x106f60.l             | +038

| ----------------------------------------------------------------------------
|  SceneB_Stage6_08d04a  @ $08D04A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Stage6_08d04a, "ax", @progbits
        .global SceneB_Stage6_08d04a
SceneB_Stage6_08d04a:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006
        jsr     0x96a80.l                       | +00c
        cmpi.w  #0x4000,0x106f5c.l              | +012
        bcs.w   SetHandlerRts_08d092            | +01a
        cmpi.b  #0x1,0x106eae.l                 | +01e
        bne.w   .L08d082                        | +026
        move.l  #0x26000,0x106f60.l             | +02a
        bra.w   SetTaskHandler_08d08c           | +034
.L08d082:
        move.l  #0x20000,0x106f60.l             | +038

| ----------------------------------------------------------------------------
|  SceneB_Tail_08d094  @ $08D094  (12 B)
| ----------------------------------------------------------------------------
        .section .text.SceneB_Tail_08d094, "ax", @progbits
        .global SceneB_Tail_08d094
SceneB_Tail_08d094:
        jsr     0x437da.l                       | +000
        jsr     0x96a0e.l                       | +006

| ----------------------------------------------------------------------------
|  SceneC_Init_08d0a8  @ $08D0A8  (210 B)
| ----------------------------------------------------------------------------
        .section .text.SceneC_Init_08d0a8, "ax", @progbits
        .global SceneC_Init_08d0a8
SceneC_Init_08d0a8:
        lea     Cut_Banner_08cc66(pc),a1        | +000
        jsr     0x4ae.l                         | +004
        lea     Cut_TextPanel_08cbc6(pc),a1     | +00a
        jsr     0x4ae.l                         | +00e
        move.w  #0x2a,d0                        | +014
        jsr     0x2352.l                        | +018
        clr.w   0x70(a6)                        | +01e
        move.b  #0x7,0x21(a6)                   | +022
        move.b  #0xf,d0                         | +028
        jsr     0x43568.l                       | +02c
        move.w  #0xffff,0x106f5e.l              | +032
        move.l  #0x18000,0x106f60.l             | +03a
        move.l  #0x0,0x106f64.l                 | +044
        bra.w   .L08d154                        | +04e
        jsr     0x46a96.l                       | +052
        move.w  #0x2a,d0                        | +058
        jsr     0x2352.l                        | +05c
        clr.w   0x70(a6)                        | +062
        move.b  #0x7,0x21(a6)                   | +066
        move.b  #0xf,d0                         | +06c
        jsr     0x43568.l                       | +070
        move.w  #0xffff,0x106f5e.l              | +076
        move.l  #0x0,0x106f64.l                 | +07e
        cmpi.b  #0x1,0x106eae.l                 | +088
        bne.w   .L08d14a                        | +090
        move.l  #0x1c000,0x106f60.l             | +094
        bra.w   .L08d154                        | +09e
.L08d14a:
        move.l  #0x18000,0x106f60.l             | +0a2
.L08d154:
        jsr     0x2230.l                        | +0ac
        lea     Capsule_Fly_08d3b4(pc),a1             | +0b2  -> $08D3B4 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +0b6
        clr.b   0x20(a6)                        | +0bc
        lea     .L08d16e(pc),a1                 | +0c0
        move.l  a1,(a6)                         | +0c4
.L08d16e:
        jsr     0x437da.l                       | +0c6
        jsr     0x96a0e.l                       | +0cc
