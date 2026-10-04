| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave LLL — Escena 5: dirigible de desembarco, torres, campamento y props
|  Región: $088A56..$08BA00  (10,690 B, 98 entradas, 43 huecos cerrados)
| ============================================================================
|
|  Cluster de entidades de la escena 5 del dispatcher de spawn (lista
|  $097422 = JumpTable_096B9C[5], registros de 20 B {$0100, x, y, handler.l,
|  0, 0, FFFF x3}, centinela $FFFF). Handlers referenciados desde esa lista:
|  $088F74, $088F8E, $0890CC, $089E8E, $08A006, $0893AC, $08A10C, $08A74E,
|  $089C44, $089E2C, $089C58. Plantillas del Mission VM (op $00, $E8000[i]):
|  [168..170] = Proj_Tmpl168/169/170 ($8B9A2/$8B9AA/$8B9B2), [254..257] =
|  Proj_Drop_V0..V3 ($8B34C/$8B366/$8B380/$8B39A), [258] = S5_Bunker_088ff4,
|  [270] = Proj_Thrown_08b258. La tabla de animación de sprites $1EE1xx
|  referencia los callbacks SprCb_Lamp2On/Off y SprCb_Lamp3On/Off (usan $2C26).
|
|  Convenciones (idénticas a Wave KKK): +$22/+$24 pos, +$28/+$2A vel, +$C
|  padre, +$20/+$21 estado, +$13 flags, +$3C registro de spawn ($2942A lee
|  x/y), +$48 hitbox, +$66 HP, +$70 margen de cull ($4FA70), +$72 timer,
|  +$74 tabla de blit ($5022A), $2870A daño (flash $5E766/$5E770), $28758
|  colisión con jugador, $28D70 slots enlazados, $4429E MissionWatch_Spawn.
|
|  A) $088A64..$088B30 — Fort_BlitWreck_V1..V4: cola del blit de la fortaleza
|     destruida de la Wave KKK (Fort_BlitWreck_088a28): según +$20 (1..4)
|     dibuja pares de tablas $2EDAF8.. con StateMachineRun $5022A.
|
|  B) $088F58..$089398 — CAMPAMENTO (props estáticos, búnker, tiendas, depósito)
|     Entity_CmpPrioWithSibling_088f58 / _08b928: compara prioridad con el
|       hermano enlazado (helper duplicado, dos copias idénticas).
|     S5_PropStatic_A/B ($88F74/$88F8E): leen spawn, mapa de sprites $2EDEA2
|       y convergen en S5_PropStatic_Common_088fa8 (snd $CA, +$70=$70, +$32=$FF).
|     S5_Bunker_088ff4 (plantilla 258): blit por índice S5_Bunker_BlitByIdx.
|     S5_Camp_Spawn_0890cc: crea las tiendas (S5_Tent_089160, blit por fila
|       S5_Tent_BlitRowByIdx_08b8f4) que al destruirse pasan a S5_Tent_Ruin.
|     S5_Depot_089290 → S5_Depot_Wreck_089374 → S5_Depot_Idle_089398 (rts
|       perpetuo; antes llamado TaskHandler_089398 en src/task_handlers.c).
|
|  C) $0893AC..$08983A — DIRIGIBLE DE DESEMBARCO (Airship_*)
|     Airship_Wait_0893ac: espera a $106F54 >= $280 (scroll), snd $CF, crea
|       la torreta Turret8 vía Airship_Attach_089688 (→ Turret8_Init_08989c)
|       y un Airship_PlayerTracker_0896de por jugador ($100440/$1004E0) que
|       suelta pares $78908 + Wreck_Spark; desciende con Airship_Steer /
|       Airship_SpiralDescent y sondea el suelo (Airship_ProbeGround_08b7b0,
|       Airship_CheckLanding_08b7f0).
|     Airship_Landed_089504: $10E39C=0, $106F5E=-1, $106F60=$8000, $106F64=0,
|       caja $2EEE24; suelta soldados con Airship_DropSoldier_08b82c (retroceso
|       Airship_Recoil_08b862) y publica el lock de cámara (Camera_PublishLockX).
|     Airship_Hover_0895cc: flota hasta cam x >= $680 → Airship_Depart_0895fe
|       (snd $1026, espiral $8B718, rastro Airship_TrailRecord[_Alt] → $99812).
|     Airship_Ground_0897d2 / _GroundB_08983a: sombra/base en el suelo.
|
|  D) $08989C..$089C08 — TORRETA DE 8 DIRECCIONES (Turret8_*)
|     Turret8_Init → Aim → Track (gira con Turret8_RotateStep_08b5c8, tablas
|       de sprites $2EE318/$2EE3F0/$2EE4C8) → Fire (Turret8_FireBullet_08b626,
|       helpers $8F3A6/$8F3BE/$8F69C) → Cooldown; Turret8_Barrel_089a30 es el
|       cañón hijo; Turret8_Casing_089ad0 expulsa casquillos con RNG ($799DE),
|       que reposan (Casing_Rest) y se desvanecen (Casing_Fade).
|
|  E) $089C08..$08A10C — PROPS DESTRUIBLES
|     S5_Crate_089c08 → S5_Crate_Broken_089d34; S5_PropSolid_089d8e;
|     S5_PropStatic_C_089e2c; S5_BarrelRow_Spawn_089e8e crea S5_Barrel_089f46
|     en fila; S5_RockRain_Spawn_08a006 crea S5_FallingRock_08a05e.
|
|  F) $08A10C..$08AE56 — TORRES A y B (S5_TowerA_* / S5_TowerB_*)
|     Init: snd $1DC + $D3, 3–4 hijos TowerPort_Init_08ae56 y un soldado
|       ($77228); HP aleatoria desde $2C06AA cuando cam x >= $7D0 (A) /
|       $950 (B). Hit: flash + bclr bit3 +$13. Stage2/3/4 al bajar HP de
|       $29A / $14D (blits de daño). Destroy: score $5000, 6 explosiones
|       $7808A, Wreck_FlagSet_08ae0c ($10E39E=1; Wreck_FlagClear lo borra),
|       humo Wreck_SmokeRise_08b07c, apaga lámparas (SprCb_Lamp*Off) y
|       termina en S5_TowerA_Rts_08a74c / S5_TowerB_Rts_08ae0a.
|
|  G) $08AE56..$08B258 — PORTILLAS DE TORRE Y RESTOS
|     TowerPort_Init → Active → Stage2 → Idle (sigue al padre con
|       Entity_FollowParent[Plus40]); Wreck_SparkBurst_08b10a lanza
|       Wreck_Spark_08b1f2 (chispas con gravedad).
|
|  H) $08B258..$08B45C — PROYECTILES
|     Proj_Thrown_08b258 (plantilla 270): parámetros +$9A..+$9D, física
|       $8F002/$8F010, Handler_ConditionalHitCounter_08B558.
|     Proj_Drop_V0..V3 (plantillas 254..257): mapa $2EE1B8, tabla +$70
|       $2EF7E4; convergen en Proj_Drop_Common_08b3b4 (snd $16D/$16F/$CE/$16E
|       según cam x vs $670).
|
|  I) $08B45C..$08B944 — CALLBACKS Y HELPERS
|     SprCb_Lamp2On/Off, SprCb_Lamp3On/Off: encienden/apagan lámparas de la
|       torre (slot de sprite vía $2C26). Math_AbsW_08b58e: |d0.w|.
|     Entity_FollowParent*_08b59e/_08b5b6, Airship_Steer*, Airship_Probe*,
|       Camera_PublishLockX_08b8e4, S5_*_BlitByIdx.
|
|  J) $08B944..$08BA00 — TABLA Y PLANTILLAS DE PROYECTIL
|     Proj_ScriptTable_08b944 (datos, 94 B, .dc.w) usada por Proj_Bounce_08b9ba
|       (`lea Proj_ScriptTable_08b944(pc),a0`); Proj_Tmpl168/169/170 son las
|       entradas de plantilla que saltan a Proj_Bounce.
|
|  Callees aún no emparejados (quedan en huecos futuros): $9A300, $38F14,
|  $997E2, $78908, $631D0, $8F3A6, $8F3BE, $8F69C, $8F002, $8F010, $280C6,
|  $5E3A2, $3093A, $5DD5C; refs pc-rel forward: $8BA0C/$8BA52/$8BB34/$8BB5E.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Fort_BlitWreck_V1_088a64  @ $088A64  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_BlitWreck_V1_088a64, "ax", @progbits
        .global Fort_BlitWreck_V1_088a64
Fort_BlitWreck_V1_088a64:
        cmpi.b  #0x1,0x20(a6)                   | +000
        bne.w   Fort_BlitWreck_V2_088a94        | +006
        lea     0x2edaf8.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edb0c.l,a2                   | +016
        jsr     0x5022a.l                       | +01c

| ----------------------------------------------------------------------------
|  Fort_BlitWreck_V2_088a94  @ $088A94  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_BlitWreck_V2_088a94, "ax", @progbits
        .global Fort_BlitWreck_V2_088a94
Fort_BlitWreck_V2_088a94:
        cmpi.b  #0x2,0x20(a6)                   | +000
        bne.w   Fort_BlitWreck_V3_088ad0        | +006
        lea     0x2edb34.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edb48.l,a2                   | +016
        jsr     0x5022a.l                       | +01c
        lea     0x2edb5c.l,a2                   | +022
        jsr     0x5022a.l                       | +028

| ----------------------------------------------------------------------------
|  Fort_BlitWreck_V3_088ad0  @ $088AD0  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_BlitWreck_V3_088ad0, "ax", @progbits
        .global Fort_BlitWreck_V3_088ad0
Fort_BlitWreck_V3_088ad0:
        cmpi.b  #0x3,0x20(a6)                   | +000
        bne.w   Fort_BlitWreck_V4_088b00        | +006
        lea     0x2edb84.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edb98.l,a2                   | +016
        jsr     0x5022a.l                       | +01c

| ----------------------------------------------------------------------------
|  Fort_BlitWreck_V4_088b00  @ $088B00  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Fort_BlitWreck_V4_088b00, "ax", @progbits
        .global Fort_BlitWreck_V4_088b00
Fort_BlitWreck_V4_088b00:
        cmpi.b  #0x4,0x20(a6)                   | +000
        bne.w   Stub_00088B3C                   | +006
        lea     0x2edbc0.l,a2                   | +00a
        jsr     0x5022a.l                       | +010
        lea     0x2edbd4.l,a2                   | +016
        jsr     0x5022a.l                       | +01c
        lea     0x2edbe8.l,a2                   | +022
        jsr     0x5022a.l                       | +028

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_088f58  @ $088F58  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_088f58, "ax", @progbits
        .global Entity_CmpPrioWithSibling_088f58
Entity_CmpPrioWithSibling_088f58:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_088f6e                    | +00c

| ----------------------------------------------------------------------------
|  S5_PropStatic_A_088f74  @ $088F74  (26 B)
| ----------------------------------------------------------------------------
        .section .text.S5_PropStatic_A_088f74, "ax", @progbits
        .global S5_PropStatic_A_088f74
S5_PropStatic_A_088f74:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2ede92.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   S5_PropStatic_Common_088fa8 | +016

| ----------------------------------------------------------------------------
|  S5_PropStatic_B_088f8e  @ $088F8E  (102 B)
| ----------------------------------------------------------------------------
        .section .text.S5_PropStatic_B_088f8e, "ax", @progbits
        .global S5_PropStatic_B_088f8e
S5_PropStatic_B_088f8e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2edea2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   S5_PropStatic_Common_088fa8 | +016
        .global S5_PropStatic_Common_088fa8
S5_PropStatic_Common_088fa8:
        move.w  #0xca,d1                        | +01a
        jsr     0x236e.l                        | +01e
        move.w  #0x70,0x70(a6)                  | +024
        move.b  #0xff,0x32(a6)                  | +02a
        move.b  #0xff,0x33(a6)                  | +030
        move.b  #0x0,0x3a(a6)                   | +036
        move.w  #0xc000,0x38(a6)                | +03c
        lea     .L088fd6(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L088fd6:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        jsr     0x4fa70.l                       | +054
        bcc.w   .L088ff2                        | +05a
        jmp     0x518.l                         | +05e
.L088ff2:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  S5_Bunker_088ff4  @ $088FF4  (216 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Bunker_088ff4, "ax", @progbits
        .global S5_Bunker_088ff4
S5_Bunker_088ff4:
        move.w  #0xcd,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  0x98(a6),d0                     | +00a
        move.b  d0,0x21(a6)                     | +00e
        move.w  #0x40,0x70(a6)                  | +012
        move.b  #0xff,0x32(a6)                  | +018
        move.b  #0xff,0x33(a6)                  | +01e
        move.b  #0x0,0x3a(a6)                   | +024
        move.w  #0x28,0x66(a6)                  | +02a
        move.w  #0x0,0x38(a6)                   | +030
        lea     0x2edeb2.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     0x2ee7b4.l,a0                   | +042
        move.l  a0,0x48(a6)                     | +048
        lea     .L089046(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L089046:
        jsr     0x2783a.l                       | +052
        jsr     0x28d70.l                       | +058
        jsr     0x2870a.l                       | +05e
        bcc.w   .L08907a                        | +064
        lea     0x5e766.l,a0                    | +068
        jsr     0x5e770.l                       | +06e
        lea     0x2edec2.l,a0                   | +074
        jsr     0x28cd4.l                       | +07a
        bclr    #0x3,0x13(a6)                   | +080
.L08907a:
        jsr     0x28758.l                       | +086
        bcc.w   .L0890ba                        | +08c
        move.w  #0x1023,d0                      | +090
        jsr     0x2352.l                        | +094
        lea     0x2ef1ac.l,a1                   | +09a
        jsr     0x77c7e.l                       | +0a0
        jsr     S5_Bunker_BlitByIdx_08b90e(pc)  | +0a6
        lea     0x2ef626.l,a1                   | +0aa
        jsr     0x43fac.l                       | +0b0
        lea     0x2ef298.l,a1                   | +0b6
        jsr     0x77c7e.l                       | +0bc
        bra.w   .L0890c4                        | +0c2
.L0890ba:
        jsr     0x4fa70.l                       | +0c6
        bcc.w   .L0890ca                        | +0cc
.L0890c4:
        jmp     0x518.l                         | +0d0
.L0890ca:
        rts                                     | +0d6

| ----------------------------------------------------------------------------
|  S5_Camp_Spawn_0890cc  @ $0890CC  (146 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Camp_Spawn_0890cc, "ax", @progbits
        .global S5_Camp_Spawn_0890cc
S5_Camp_Spawn_0890cc:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     S5_Depot_089290(pc),a1          | +00a
        jsr     0x6fe.l                         | +00e
        jsr     0x5dd02.l                       | +014
        addi.w  #0x60,0x22(a0)                  | +01a
        addi.w  #0x24,0x24(a0)                  | +020
        lea     S5_Tent_089160(pc),a1           | +026
        jsr     0x6fe.l                         | +02a
        jsr     0x5dd02.l                       | +030
        move.b  #0x0,0x21(a0)                   | +036
        addi.w  #0x20,0x24(a0)                  | +03c
        lea     S5_Tent_089160(pc),a1           | +042
        jsr     0x6fe.l                         | +046
        jsr     0x5dd02.l                       | +04c
        move.b  #0x1,0x21(a0)                   | +052
        addi.w  #0x38,0x22(a0)                  | +058
        addi.w  #0x20,0x24(a0)                  | +05e
        lea     S5_Tent_089160(pc),a1           | +064
        jsr     0x6fe.l                         | +068
        jsr     0x5dd02.l                       | +06e
        move.b  #0x2,0x21(a0)                   | +074
        addi.w  #0x80,0x22(a0)                  | +07a
        addi.w  #0x20,0x24(a0)                  | +080
        lea     .L089158(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L089158:
        jmp     0x518.l                         | +08c

| ----------------------------------------------------------------------------
|  Rts_08915e  @ $08915E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08915e, "ax", @progbits
        .global Rts_08915e
Rts_08915e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  S5_Tent_089160  @ $089160  (218 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Tent_089160, "ax", @progbits
        .global S5_Tent_089160
S5_Tent_089160:
        move.w  #0xcc,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x40,0x70(a6)                  | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        move.w  #0x0,0x38(a6)                   | +022
        move.w  #0x14,0x66(a6)                  | +028
        clr.l   d0                              | +02e
        move.b  0x21(a6),d0                     | +030
        asl.l   #0x1,d0                         | +034
        movea.l #0x2edf76,a0                    | +036
        lsl.w   #0x2,d0                         | +03c
        movea.l (a0,d0.w),a0                    | +03e
        cmpa.l  #0xffffffff,a0                  | +042
        beq.w   .L0891b2                        | +048
        jsr     0x28cd4.l                       | +04c
.L0891b2:
        lea     .L0891b8(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L0891b8:
        jsr     0x2783a.l                       | +058
        jsr     0x28d70.l                       | +05e
        jsr     0x2870a.l                       | +064
        bcc.w   .L0891e0                        | +06a
        lea     0x5e766.l,a0                    | +06e
        jsr     0x5e770.l                       | +074
        bclr    #0x3,0x13(a6)                   | +07a
.L0891e0:
        jsr     0x28758.l                       | +080
        bcc.w   .L089228                        | +086
        move.w  #0x1030,d0                      | +08a
        jsr     0x2352.l                        | +08e
        lea     0x2ef1d0.l,a1                   | +094
        jsr     0x77c7e.l                       | +09a
        lea     0x631d0.l,a1                    | +0a0
        jsr     0x4ae.l                         | +0a6
        jsr     0x5dd22.l                       | +0ac
        move.w  0x54(a6),0x22(a0)               | +0b2
        move.w  0x56(a6),0x24(a0)               | +0b8
        jsr     S5_Tent_BlitRowByIdx_08b8f4(pc) | +0be
        lea     S5_Tent_Ruin_08923a(pc),a1      | +0c2
        move.l  a1,(a6)                         | +0c6
.L089228:
        jsr     0x4fa70.l                       | +0c8
        bcc.w   .L089238                        | +0ce
        jmp     0x518.l                         | +0d2
.L089238:
        rts                                     | +0d8

| ----------------------------------------------------------------------------
|  S5_Tent_Ruin_08923a  @ $08923A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Tent_Ruin_08923a, "ax", @progbits
        .global S5_Tent_Ruin_08923a
S5_Tent_Ruin_08923a:
        move.l  #0x500,d0                       | +000
        jsr     0x51a28.l                       | +006
        clr.l   d0                              | +00c
        move.b  0x21(a6),d0                     | +00e
        asl.l   #0x1,d0                         | +012
        addq.l  #0x1,d0                         | +014
        movea.l #0x2edf76,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L08926c                        | +028
        jsr     0x28cd4.l                       | +02c
.L08926c:
        lea     .L089272(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L089272:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        jsr     0x4fa70.l                       | +044
        bcc.w   .L08928e                        | +04a
        jmp     0x518.l                         | +04e
.L08928e:
        rts                                     | +054

| ----------------------------------------------------------------------------
|  S5_Depot_089290  @ $089290  (228 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Depot_089290, "ax", @progbits
        .global S5_Depot_089290
S5_Depot_089290:
        move.w  #0xcb,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x30,0x70(a6)                  | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        move.w  #0x64,0x66(a6)                  | +022
        move.w  #0x0,0x38(a6)                   | +028
        lea     0x2ee2c6.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L0892d0(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L0892d0:
        jsr     0x2783a.l                       | +040
        jsr     0x28d70.l                       | +046
        jsr     0x2870a.l                       | +04c
        bcc.w   .L0892f8                        | +052
        lea     0x5e766.l,a0                    | +056
        jsr     0x5e770.l                       | +05c
        bclr    #0x3,0x13(a6)                   | +062
.L0892f8:
        jsr     0x28758.l                       | +068
        bcc.w   .L089362                        | +06e
        move.l  #0x500,d0                       | +072
        jsr     0x51a28.l                       | +078
        move.w  #0x1035,d0                      | +07e
        jsr     0x2352.l                        | +082
        lea     0x2ef2ea.l,a1                   | +088
        jsr     0x77c7e.l                       | +08e
        move.w  #0xc000,0x38(a0)                | +094
        lea     0x7808a.l,a1                    | +09a
        jsr     0x4ae.l                         | +0a0
        jsr     0x5dd22.l                       | +0a6
        move.w  #0xc000,0x38(a0)                | +0ac
        lea     0xf0742.l,a1                    | +0b2
        move.w  #0x82,d0                        | +0b8
        move.b  #0x0,0x82(a6)                   | +0bc
        jsr     0x4429e.l                       | +0c2
        lea     S5_Depot_Wreck_089374(pc),a1    | +0c8
        move.l  a1,(a6)                         | +0cc
        bra.w   .L089372                        | +0ce
.L089362:
        jsr     0x4fa70.l                       | +0d2
        bcc.w   .L089372                        | +0d8
        jmp     0x518.l                         | +0dc
.L089372:
        rts                                     | +0e2

| ----------------------------------------------------------------------------
|  S5_Depot_Wreck_089374  @ $089374  (28 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Depot_Wreck_089374, "ax", @progbits
        .global S5_Depot_Wreck_089374
S5_Depot_Wreck_089374:
        lea     0x2ef00c.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        jsr     0x283ca.l                       | +010
        jsr     0x283d8.l                       | +016

| ----------------------------------------------------------------------------
|  S5_Depot_Idle_089398  @ $089398  (18 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Depot_Idle_089398, "ax", @progbits
        .global S5_Depot_Idle_089398
S5_Depot_Idle_089398:
        jsr     0x283ca.l                       | +000
        jsr     0x283d8.l                       | +006
        jmp     0x518.l                         | +00c

| ----------------------------------------------------------------------------
|  Rts_0893aa  @ $0893AA  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0893aa, "ax", @progbits
        .global Rts_0893aa
Rts_0893aa:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Airship_Wait_0893ac  @ $0893AC  (336 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Wait_0893ac, "ax", @progbits
        .global Airship_Wait_0893ac
Airship_Wait_0893ac:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x68,0x22(a6)                  | +00a
        move.w  #0xb8,0x24(a6)                  | +010
        movea.l 0xc(a6),a0                      | +016
        cmpi.b  #0xc,0x106ece.l                 | +01a
        bne.w   .L0893de                        | +022
        move.w  #0x38,0x22(a6)                  | +026
        move.w  #0x110,0x24(a6)                 | +02c
.L0893de:
        lea     .L0893e4(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L0893e4:
        jsr     Airship_TrailRecord_08b8b0(pc)  | +038
        cmpi.w  #0x280,0x106f54.l               | +03c
        bge.w   .L0893f6                        | +044
        rts                                     | +048
.L0893f6:
        move.w  #0xcf,d1                        | +04a
        jsr     0x236e.l                        | +04e
        move.w  #0xa0,0x70(a6)                  | +054
        move.w  #0x0,0x84(a6)                   | +05a
        move.b  #0x0,0x83(a6)                   | +060
        move.b  #0xff,0x32(a6)                  | +066
        move.b  #0xff,0x33(a6)                  | +06c
        move.b  #0x0,0x3a(a6)                   | +072
        move.b  #0x0,0x20(a6)                   | +078
        move.w  #0x0,0x38(a6)                   | +07e
        jsr     0x267e2.l                       | +084
        move.w  #0xa0,0x2a(a6)                  | +08a
        lea     0x2edf8e.l,a0                   | +090
        jsr     0x28cd4.l                       | +096
        lea     Airship_Attach_089688(pc),a1    | +09c
        jsr     0x4ae.l                         | +0a0
        cmpi.b  #0xc,0x106ece.l                 | +0a6
        beq.w   .L089488                        | +0ae
        lea     Airship_Ground_0897d2(pc),a1    | +0b2
        jsr     0x4ae.l                         | +0b6
        lea     Airship_PlayerTracker_0896de(pc),a1 | +0bc
        jsr     0x4ae.l                         | +0c0
        move.b  #0x0,0x21(a0)                   | +0c6
        lea     Airship_PlayerTracker_0896de(pc),a1 | +0cc
        jsr     0x4ae.l                         | +0d0
        move.b  #0x1,0x21(a0)                   | +0d6
.L089488:
        lea     0x9a300.l,a1                    | +0dc
        jsr     0x4ae.l                         | +0e2
        move.w  #0x0,0x22(a0)                   | +0e8
        move.w  #0x40,0x24(a0)                  | +0ee
        move.l  0x106f50.l,d0                   | +0f4
        swap    d0                              | +0fa
        move.w  d0,0x7c(a6)                     | +0fc
        move.w  #0x1080,d0                      | +100
        jsr     0x2352.l                        | +104
        move.w  #0x107f,d0                      | +10a
        jsr     0x2352.l                        | +10e
        cmpi.b  #0xc,0x106ece.l                 | +114
        beq.w   Airship_Landed_089504           | +11c
        lea     .L0894d2(pc),a1                 | +120
        move.l  a1,(a6)                         | +124
.L0894d2:
        clr.b   0x10e39a.l                      | +126
        bset    #0x6,0x13(a6)                   | +12c
        jsr     0x27cee.l                       | +132
        jsr     Airship_TrailRecord_08b8b0(pc)  | +138
        jsr     Camera_PublishLockX_08b8e4(pc)  | +13c
        jsr     0x28d70.l                       | +140
        cmpi.w  #0x118,0x24(a6)                 | +146
        blt.w   SetHandlerRts_089502            | +14c

| ----------------------------------------------------------------------------
|  Airship_Landed_089504  @ $089504  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Landed_089504, "ax", @progbits
        .global Airship_Landed_089504
Airship_Landed_089504:
        lea     0x38f14.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        addi.w  #0x9e,0x22(a0)                  | +012
        addi.w  #0x40,0x24(a0)                  | +018
        move.b  #0x1,0x98(a0)                   | +01e
        cmpi.b  #0xc,0x106ece.l                 | +024
        beq.w   .L089556                        | +02c
        clr.b   0x10e39c.l                      | +030
        move.w  #0xffff,0x106f5e.l              | +036
        move.l  #0x8000,0x106f60.l              | +03e
        move.l  #0x0,0x106f64.l                 | +048
.L089556:
        jsr     0x267e2.l                       | +052
        move.w  #0x40,0x34(a6)                  | +058
        move.w  #0x3e,0x36(a6)                  | +05e
        move.w  #0x68,0x7e(a6)                  | +064
        move.w  #0x118,0x80(a6)                 | +06a
        lea     0x2eee24.l,a0                   | +070
        move.l  a0,0x4c(a6)                     | +076
        jsr     0x283ca.l                       | +07a
        move.b  #0xff,0x20(a6)                  | +080
        lea     .L089590(pc),a1                 | +086
        move.l  a1,(a6)                         | +08a
.L089590:
        bset    #0x6,0x13(a6)                   | +08c
        jsr     Airship_Steer_08b6a8(pc)        | +092
        jsr     Airship_TrailRecord_08b8b0(pc)  | +096
        jsr     Camera_PublishLockX_08b8e4(pc)  | +09a
        jsr     0x28d70.l                       | +09e
        jsr     0x283ca.l                       | +0a4
        jsr     0x283d8.l                       | +0aa
        move.l  0x106f50.l,d0                   | +0b0
        swap    d0                              | +0b6
        cmpi.w  #0x680,d0                       | +0b8
        blt.w   SetHandlerRts_0895ca            | +0bc

| ----------------------------------------------------------------------------
|  Airship_Hover_0895cc  @ $0895CC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Hover_0895cc, "ax", @progbits
        .global Airship_Hover_0895cc
Airship_Hover_0895cc:
        move.w  #0x100,0x80(a6)                 | +000
        lea     .L0895d8(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0895d8:
        bset    #0x6,0x13(a6)                   | +00c
        jsr     Airship_Steer_08b6a8(pc)        | +012
        jsr     Airship_TrailRecord_08b8b0(pc)  | +016
        jsr     Camera_PublishLockX_08b8e4(pc)  | +01a
        jsr     0x28d70.l                       | +01e
        jsr     0x283ca.l                       | +024

| ----------------------------------------------------------------------------
|  Airship_Depart_0895fe  @ $0895FE  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Depart_0895fe, "ax", @progbits
        .global Airship_Depart_0895fe
Airship_Depart_0895fe:
        move.w  #0x1026,d0                      | +000
        jsr     0x2352.l                        | +004
        cmpi.b  #0xc,0x106ece.l                 | +00a
        beq.w   .L08961a                        | +012
        clr.w   0x106f5e.l                      | +016
.L08961a:
        move.w  #0x160,0x36(a6)                 | +01c
        lea     0xffff.w,a0                     | +022
        move.l  a0,0x4c(a6)                     | +026
        jsr     0x283ca.l                       | +02a
        lea     .L089634(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L089634:
        jsr     0x2783a.l                       | +036
        jsr     Airship_SpiralDescent_08b718(pc) | +03c
        cmpi.b  #0xc,0x106ece.l                 | +040
        bne.w   .L089652                        | +048
        jsr     Airship_TrailRecord_08b8b0(pc)  | +04c
        bra.w   .L089656                        | +050
.L089652:
        jsr     Airship_TrailRecordAlt_08b8ca(pc) | +054
.L089656:
        jsr     0x28d70.l                       | +058
        cmpi.w  #0x3,0x36(a6)                   | +05e
        bhi.w   .L089670                        | +064
        move.w  #0x23,d0                        | +068
        jsr     0x2352.l                        | +06c
.L089670:
        jsr     0x4fa70.l                       | +072
        bcc.w   Jsr5B6Rts_089686                | +078

| ----------------------------------------------------------------------------
|  Airship_Attach_089688  @ $089688  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Attach_089688, "ax", @progbits
        .global Airship_Attach_089688
Airship_Attach_089688:
        move.w  #0xcf,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        lea     0x2edf9e.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L0896b6(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0896b6:
        movea.l 0xc(a6),a0                      | +02e
        move.w  0x22(a0),0x22(a6)               | +032
        move.w  0x24(a0),0x24(a6)               | +038
        move.w  0x38(a0),0x38(a6)               | +03e
        cmpi.w  #0x0,0x22(a6)                   | +044
        blt.w   JsrAbsRts_0896dc                | +04a

| ----------------------------------------------------------------------------
|  Airship_PlayerTracker_0896de  @ $0896DE  (244 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_PlayerTracker_0896de, "ax", @progbits
        .global Airship_PlayerTracker_0896de
Airship_PlayerTracker_0896de:
        lea     0x100440.l,a0                   | +000
        move.l  a0,0x70(a6)                     | +006
        move.w  #0x0,d0                         | +00a
        cmpi.b  #0x0,0x21(a6)                   | +00e
        beq.w   .L089704                        | +014
        lea     0x1004e0.l,a0                   | +018
        move.l  a0,0x70(a6)                     | +01e
        move.w  #0x1,d0                         | +022
.L089704:
        jsr     0x5e3a2.l                       | +026
        bcs.w   .L089716                        | +02c
        jmp     0x518.l                         | +030
        rts                                     | +036
.L089716:
        lea     .L08971c(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L08971c:
        move.l  0x106f50.l,d0                   | +03e
        swap    d0                              | +044
        cmpi.w  #0x678,d0                       | +046
        bgt.w   .L0897ca                        | +04a
        move.l  0x106f54.l,d0                   | +04e
        swap    d0                              | +054
        cmpi.w  #0x280,d0                       | +056
        blt.w   .L0897d0                        | +05a
        movea.l 0x70(a6),a0                     | +05e
        cmpi.w  #0x120,0x24(a0)                 | +062
        bgt.w   .L0897d0                        | +068
        move.w  #0x1053,d0                      | +06c
        jsr     0x2352.l                        | +070
        lea     0x78908.l,a1                    | +076
        jsr     0x4ae.l                         | +07c
        movea.l 0x70(a6),a1                     | +082
        move.w  0x22(a1),0x22(a0)               | +086
        subi.w  #0x18,0x22(a0)                  | +08c
        move.w  0x24(a1),0x24(a0)               | +092
        move.w  #0xc000,0x38(a0)                | +098
        lea     0x78908.l,a1                    | +09e
        jsr     0x4ae.l                         | +0a4
        movea.l 0x70(a6),a1                     | +0aa
        move.w  0x22(a1),0x22(a0)               | +0ae
        addi.w  #0x18,0x22(a0)                  | +0b4
        move.w  0x24(a1),0x24(a0)               | +0ba
        move.w  #0xc000,0x38(a0)                | +0c0
        lea     Wreck_Spark_08b1f2(pc),a1       | +0c6
        jsr     0x4ae.l                         | +0ca
        movea.l 0x70(a6),a1                     | +0d0
        move.w  0x22(a1),0x22(a0)               | +0d4
        move.w  0x24(a1),0x24(a0)               | +0da
        move.w  #0xc000,0x38(a0)                | +0e0
        move.w  #0x0,0x72(a0)                   | +0e6
.L0897ca:
        jmp     0x518.l                         | +0ec
.L0897d0:
        rts                                     | +0f2

| ----------------------------------------------------------------------------
|  Airship_Ground_0897d2  @ $0897D2  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Ground_0897d2, "ax", @progbits
        .global Airship_Ground_0897d2
Airship_Ground_0897d2:
        move.w  #0xd0,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x120,0x24(a6)                 | +01c
        bset    #0x6,0x12(a6)                   | +022
        move.w  #0xf000,0x38(a6)                | +028
        lea     0x2edfae.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     Airship_GroundB_08983a(pc),a1   | +03a
        jsr     0x4ae.l                         | +03e
        lea     .L08981c(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L08981c:
        jsr     0x2783a.l                       | +04a
        movea.l 0xc(a6),a0                      | +050
        move.w  0x22(a0),d0                     | +054
        subi.w  #0xa,d0                         | +058
        move.w  d0,0x22(a6)                     | +05c

| ----------------------------------------------------------------------------
|  Airship_GroundB_08983a  @ $08983A  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_GroundB_08983a, "ax", @progbits
        .global Airship_GroundB_08983a
Airship_GroundB_08983a:
        move.w  #0xd0,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        bset    #0x6,0x12(a6)                   | +01c
        move.w  #0xf000,0x38(a6)                | +022
        lea     0x2edfec.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L089874(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L089874:
        movea.l 0xc(a6),a0                      | +03a
        move.w  0x22(a0),0x22(a6)               | +03e
        move.w  0x24(a0),0x24(a6)               | +044
        move.w  0x38(a0),0x38(a6)               | +04a
        cmpi.w  #0x0,0x22(a6)                   | +050
        blt.w   JsrAbsRts_08989a                | +056

| ----------------------------------------------------------------------------
|  Turret8_Init_08989c  @ $08989C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Init_08989c, "ax", @progbits
        .global Turret8_Init_08989c
Turret8_Init_08989c:
        move.w  #0xd1,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x2,0x34(a6)                   | +01c
        move.w  #0x1,0x90(a6)                   | +022
        move.w  #0xf,0x92(a6)                   | +028
        jsr     0x8f3a6.l                       | +02e
        move.b  d0,0x95(a6)                     | +034

| ----------------------------------------------------------------------------
|  Turret8_Aim_0898d4  @ $0898D4  (132 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Aim_0898d4, "ax", @progbits
        .global Turret8_Aim_0898d4
Turret8_Aim_0898d4:
        move.w  0x34(a6),d0                     | +000
        andi.w  #0x7,d0                         | +004
        movea.l #0x2ee318,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L0898f8                        | +01a
        jsr     0x28cd4.l                       | +01e
.L0898f8:
        lea     .L0898fe(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L0898fe:
        jsr     Entity_FollowParentPlus40_08b59e(pc) | +02a
        move.b  #0x0,0x94(a6)                   | +02e
        jsr     0x28d70.l                       | +034
        movea.l 0xc(a6),a0                      | +03a
        cmpi.b  #0x0,0x20(a0)                   | +03e
        beq.w   SetHandlerRts_08995e            | +044
        move.w  0x22(a6),d0                     | +048
        cmpi.w  #0xfff0,d0                      | +04c
        blt.w   SetHandlerRts_08995e            | +050
        move.w  0x24(a6),d1                     | +054
        move.b  0x95(a6),d2                     | +058
        jsr     0x8f3be.l                       | +05c
        btst    #0x7,d4                         | +062
        bne.w   SetHandlerRts_08995e            | +066
        move.b  d4,0x21(a6)                     | +06a
        lea     Turret8_Barrel_089a30(pc),a1    | +06e
        jsr     0x6fe.l                         | +072
        jsr     0x5dd02.l                       | +078
        move.b  0x21(a6),0x21(a0)               | +07e

| ----------------------------------------------------------------------------
|  Turret8_Track_089960  @ $089960  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Track_089960, "ax", @progbits
        .global Turret8_Track_089960
Turret8_Track_089960:
        lea     .L089966(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L089966:
        jsr     Entity_FollowParentPlus40_08b59e(pc) | +006
        move.b  #0x1,0x94(a6)                   | +00a
        jsr     Turret8_RotateStep_08b5c8(pc)   | +010
        bcc.w   .L0899a2                        | +014
        move.b  #0x88,0x94(a6)                  | +018
        move.w  0x34(a6),d0                     | +01e
        andi.w  #0x7,d0                         | +022
        movea.l #0x2ee3f0,a0                    | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L0899a2                        | +038
        jsr     0x28cd4.l                       | +03c
.L0899a2:
        jsr     0x28d70.l                       | +042
        movea.l #0x100440,a0                    | +048
        cmpi.b  #0x0,0x21(a6)                   | +04e
        beq.w   .L0899be                        | +054
        movea.l #0x1004e0,a0                    | +058
.L0899be:
        btst    #0x0,0x13(a0)                   | +05e
        beq.w   Turret8_Fire_0899d0             | +064

| ----------------------------------------------------------------------------
|  Turret8_Fire_0899d0  @ $0899D0  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Fire_0899d0, "ax", @progbits
        .global Turret8_Fire_0899d0
Turret8_Fire_0899d0:
        jsr     Turret8_FireBullet_08b626(pc)   | +000
        move.w  0x22(a6),d0                     | +004
        cmpi.w  #0xfff0,d0                      | +008
        blt.w   .L0899f6                        | +00c
        move.w  0x24(a6),d1                     | +010
        move.b  0x95(a6),d2                     | +014
        jsr     0x8f3be.l                       | +018
        btst    #0x7,d4                         | +01e
        beq.w   SetHandlerRts_089a02            | +022
.L0899f6:
        move.b  #0xff,0x94(a6)                  | +026

| ----------------------------------------------------------------------------
|  Turret8_Cooldown_089a04  @ $089A04  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Cooldown_089a04, "ax", @progbits
        .global Turret8_Cooldown_089a04
Turret8_Cooldown_089a04:
        move.b  #0xff,0x94(a6)                  | +000
        move.w  #0xa,0x72(a6)                   | +006
        lea     .L089a16(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L089a16:
        jsr     Entity_FollowParentPlus40_08b59e(pc) | +012
        jsr     0x28d70.l                       | +016
        subq.w  #0x1,0x72(a6)                   | +01c
        bne.w   SetHandlerRts_089a2e            | +020

| ----------------------------------------------------------------------------
|  Turret8_Barrel_089a30  @ $089A30  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Barrel_089a30, "ax", @progbits
        .global Turret8_Barrel_089a30
Turret8_Barrel_089a30:
        move.b  0x21(a6),d4                     | +000
        jsr     0x8f69c.l                       | +004
        jsr     0x236e.l                        | +00a
        move.b  #0xff,0x32(a6)                  | +010
        move.b  #0xff,0x33(a6)                  | +016
        move.b  #0x0,0x3a(a6)                   | +01c
        movea.l 0xc(a6),a0                      | +022
        move.w  0x34(a0),d0                     | +026
        andi.w  #0x7,d0                         | +02a
        movea.l #0x2ee4c8,a0                    | +02e
        lsl.w   #0x2,d0                         | +034
        movea.l (a0,d0.w),a0                    | +036
        cmpa.l  #0xffffffff,a0                  | +03a
        beq.w   .L089a7a                        | +040
        jsr     0x28cd4.l                       | +044
.L089a7a:
        lea     .L089a80(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L089a80:
        jsr     Entity_FollowParent_08b5b6(pc)  | +050
        movea.l 0xc(a6),a0                      | +054
        cmpi.b  #0xff,0x94(a0)                  | +058
        beq.w   JmpToScheduler_089ac8           | +05e
        cmpi.b  #0x88,0x94(a0)                  | +062
        bne.w   JsrAbsThunk_089ac0              | +068
        move.w  0x34(a0),d0                     | +06c
        andi.w  #0x7,d0                         | +070
        movea.l #0x2ee4c8,a0                    | +074
        lsl.w   #0x2,d0                         | +07a
        movea.l (a0,d0.w),a0                    | +07c
        cmpa.l  #0xffffffff,a0                  | +080
        beq.w   JsrAbsThunk_089ac0              | +086
        jsr     0x28cd4.l                       | +08a

| ----------------------------------------------------------------------------
|  Turret8_Casing_089ad0  @ $089AD0  (222 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Casing_089ad0, "ax", @progbits
        .global Turret8_Casing_089ad0
Turret8_Casing_089ad0:
        move.w  #0xd2,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0xc000,0x38(a6)                | +01c
        lea     0x2ee4e8.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        jsr     0x267e2.l                       | +02e
        move.w  #0x380,0x2a(a6)                 | +034
        jsr     0x5e9b6.l                       | +03a
        andi.w  #0xff,d0                        | +040
        add.w   d0,0x2a(a6)                     | +044
        move.w  #0xffc0,0x2e(a6)                | +048
        move.w  #0x40,0x28(a6)                  | +04e
        add.w   d0,0x28(a6)                     | +054
        lea     .L089b2e(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L089b2e:
        jsr     0x27cee.l                       | +05e
        scs.b   0x5c(a6)                        | +064
        jsr     0x28d70.l                       | +068
        move.w  0x2a(a6),d0                     | +06e
        bpl.w   .L089b96                        | +072
        move.b  0x5c(a6),d0                     | +076
        beq.w   .L089b96                        | +07a
        jsr     0x5e9b6.l                       | +07e
        andi.b  #0xf,d0                         | +084
        bne.w   .L089b66                        | +088
        neg.w   0x28(a6)                        | +08c
        eori.b  #0x1,0x3a(a6)                   | +090
.L089b66:
        move.w  0x2a(a6),d0                     | +096
        jsr     Math_AbsW_08b58e(pc)            | +09a
        asr.w   #0x1,d0                         | +09e
        move.w  d0,0x2a(a6)                     | +0a0
        cmpi.w  #0x80,d0                        | +0a4
        bgt.w   .L089b96                        | +0a8
        lea     Turret8_Casing_Rest_089bae(pc),a1 | +0ac
        move.l  a1,(a6)                         | +0b0
        jsr     0x5e9b6.l                       | +0b2
        andi.w  #0x7f,d0                        | +0b8
        bne.w   .L089b96                        | +0bc
        lea     Turret8_Casing_Fade_089bd8(pc),a1 | +0c0
        move.l  a1,(a6)                         | +0c4
.L089b96:
        lea     0x2ef7b2.l,a0                   | +0c6
        jsr     0x5dd56.l                       | +0cc
        bcc.w   .L089bac                        | +0d2
        jmp     0x518.l                         | +0d6
.L089bac:
        rts                                     | +0dc

| ----------------------------------------------------------------------------
|  Turret8_Casing_Rest_089bae  @ $089BAE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Casing_Rest_089bae, "ax", @progbits
        .global Turret8_Casing_Rest_089bae
Turret8_Casing_Rest_089bae:
        lea     .L089bb4(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L089bb4:
        jsr     0x27cee.l                       | +006
        jsr     0x28d70.l                       | +00c
        lea     0x2ef7b2.l,a0                   | +012
        jsr     0x5dd56.l                       | +018
        bcc.w   .L089bd6                        | +01e
        jmp     0x518.l                         | +022
.L089bd6:
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Turret8_Casing_Fade_089bd8  @ $089BD8  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_Casing_Fade_089bd8, "ax", @progbits
        .global Turret8_Casing_Fade_089bd8
Turret8_Casing_Fade_089bd8:
        lea     .L089bde(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L089bde:
        jsr     0x2783a.l                       | +006
        move.b  #0x7f,0x44(a6)                  | +00c
        jsr     0x28d70.l                       | +012
        lea     0x2ef7b2.l,a0                   | +018
        jsr     0x5dd56.l                       | +01e
        bcc.w   .L089c06                        | +024
        jmp     0x518.l                         | +028
.L089c06:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  S5_Crate_089c08  @ $089C08  (300 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Crate_089c08, "ax", @progbits
        .global S5_Crate_089c08
S5_Crate_089c08:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x21(a6)                   | +00a
        bra.w   .L089c6c                        | +010
        movea.l 0x3c(a6),a1                     | +014
        jsr     0x2942a.l                       | +018
        move.b  #0x1,0x21(a6)                   | +01e
        bra.w   .L089c6c                        | +024
        movea.l 0x3c(a6),a1                     | +028
        jsr     0x2942a.l                       | +02c
        move.b  #0x2,0x21(a6)                   | +032
        bra.w   .L089c6c                        | +038
        movea.l 0x3c(a6),a1                     | +03c
        jsr     0x2942a.l                       | +040
        move.b  #0x3,0x21(a6)                   | +046
        bra.w   .L089c6c                        | +04c
        movea.l 0x3c(a6),a1                     | +050
        jsr     0x2942a.l                       | +054
        move.b  #0x4,0x21(a6)                   | +05a
        bra.w   .L089c6c                        | +060
.L089c6c:
        move.w  #0xd6,d1                        | +064
        jsr     0x236e.l                        | +068
        move.w  #0x40,0x70(a6)                  | +06e
        move.b  #0xff,0x32(a6)                  | +074
        move.b  #0xff,0x33(a6)                  | +07a
        move.b  #0x0,0x3a(a6)                   | +080
        move.w  #0x14,0x66(a6)                  | +086
        move.w  #0x8000,0x38(a6)                | +08c
        jsr     0x267e2.l                       | +092
        jsr     0x27cee.l                       | +098
        clr.l   d0                              | +09e
        move.b  0x21(a6),d0                     | +0a0
        movea.l #0x2ee13c,a0                    | +0a4
        lsl.w   #0x2,d0                         | +0aa
        movea.l (a0,d0.w),a0                    | +0ac
        cmpa.l  #0xffffffff,a0                  | +0b0
        beq.w   .L089cc8                        | +0b6
        jsr     0x28cd4.l                       | +0ba
.L089cc8:
        lea     .L089cce(pc),a1                 | +0c0
        move.l  a1,(a6)                         | +0c4
.L089cce:
        jsr     0x2783a.l                       | +0c6
        jsr     0x28d70.l                       | +0cc
        jsr     0x2870a.l                       | +0d2
        bcc.w   .L089cf6                        | +0d8
        lea     0x5e766.l,a0                    | +0dc
        jsr     0x5e770.l                       | +0e2
        bclr    #0x3,0x13(a6)                   | +0e8
.L089cf6:
        jsr     0x28758.l                       | +0ee
        bcc.w   .L089d22                        | +0f4
        move.w  #0x102e,d0                      | +0f8
        jsr     0x2352.l                        | +0fc
        lea     0x2ef1d0.l,a1                   | +102
        jsr     0x77c7e.l                       | +108
        move.w  #0xf000,0x38(a0)                | +10e
        lea     S5_Crate_Broken_089d34(pc),a1   | +114
        move.l  a1,(a6)                         | +118
.L089d22:
        jsr     0x4fa70.l                       | +11a
        bcc.w   .L089d32                        | +120
        jmp     0x518.l                         | +124
.L089d32:
        rts                                     | +12a

| ----------------------------------------------------------------------------
|  S5_Crate_Broken_089d34  @ $089D34  (90 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Crate_Broken_089d34, "ax", @progbits
        .global S5_Crate_Broken_089d34
S5_Crate_Broken_089d34:
        move.l  #0x500,d0                       | +000
        jsr     0x51a28.l                       | +006
        clr.l   d0                              | +00c
        move.b  0x21(a6),d0                     | +00e
        lea     0xffff.w,a0                     | +012
        move.l  a0,0x48(a6)                     | +016
        movea.l #0x2ee150,a0                    | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L089d6a                        | +02c
        jsr     0x28cd4.l                       | +030
.L089d6a:
        lea     .L089d70(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L089d70:
        jsr     0x2783a.l                       | +03c
        jsr     0x28d70.l                       | +042
        jsr     0x4fa70.l                       | +048
        bcc.w   .L089d8c                        | +04e
        jmp     0x518.l                         | +052
.L089d8c:
        rts                                     | +058

| ----------------------------------------------------------------------------
|  S5_PropSolid_089d8e  @ $089D8E  (158 B)
| ----------------------------------------------------------------------------
        .section .text.S5_PropSolid_089d8e, "ax", @progbits
        .global S5_PropSolid_089d8e
S5_PropSolid_089d8e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2ee164.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L089dc2                        | +016
        movea.l 0x3c(a6),a1                     | +01a
        jsr     0x2942a.l                       | +01e
        lea     0x2ee176.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L089dc2                        | +030
.L089dc2:
        move.w  #0xd7,d1                        | +034
        jsr     0x236e.l                        | +038
        move.w  #0x50,0x70(a6)                  | +03e
        move.b  #0xff,0x32(a6)                  | +044
        move.b  #0xff,0x33(a6)                  | +04a
        move.b  #0x0,0x3a(a6)                   | +050
        move.w  #0x8000,0x38(a6)                | +056
        jsr     0x267e2.l                       | +05c
        jsr     0x27cee.l                       | +062
        lea     .L089dfc(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L089dfc:
        jsr     0x2783a.l                       | +06e
        jsr     0x28d70.l                       | +074
        bclr    #0x3,0x13(a6)                   | +07a
        bclr    #0x0,0x13(a6)                   | +080
        move.w  #0x7fff,0x66(a6)                | +086
        jsr     0x4fa70.l                       | +08c
        bcc.w   .L089e2a                        | +092
        jmp     0x518.l                         | +096
.L089e2a:
        rts                                     | +09c

| ----------------------------------------------------------------------------
|  S5_PropStatic_C_089e2c  @ $089E2C  (98 B)
| ----------------------------------------------------------------------------
        .section .text.S5_PropStatic_C_089e2c, "ax", @progbits
        .global S5_PropStatic_C_089e2c
S5_PropStatic_C_089e2c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     0x2ee188.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0xd7,d1                        | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x90,0x70(a6)                  | +020
        move.b  #0xff,0x32(a6)                  | +026
        move.b  #0xff,0x33(a6)                  | +02c
        move.b  #0x0,0x3a(a6)                   | +032
        move.w  #0x0,0x38(a6)                   | +038
        lea     .L089e70(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L089e70:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        jsr     0x4fa70.l                       | +050
        bcc.w   .L089e8c                        | +056
        jmp     0x518.l                         | +05a
.L089e8c:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  S5_BarrelRow_Spawn_089e8e  @ $089E8E  (184 B)
| ----------------------------------------------------------------------------
        .section .text.S5_BarrelRow_Spawn_089e8e, "ax", @progbits
        .global S5_BarrelRow_Spawn_089e8e
S5_BarrelRow_Spawn_089e8e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.b  #0x0,0x21(a6)                   | +00a
        lea     S5_Barrel_089f46(pc),a1         | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd22.l                       | +01a
        addi.w  #0x40,0x22(a0)                  | +020
        addi.w  #0x80,0x24(a0)                  | +026
        lea     S5_Barrel_089f46(pc),a1         | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd22.l                       | +036
        addi.w  #0x90,0x22(a0)                  | +03c
        addi.w  #0x80,0x24(a0)                  | +042
        lea     S5_Barrel_089f46(pc),a1         | +048
        jsr     0x4ae.l                         | +04c
        jsr     0x5dd22.l                       | +052
        addi.w  #0x20,0x22(a0)                  | +058
        lea     S5_Barrel_089f46(pc),a1         | +05e
        jsr     0x4ae.l                         | +062
        jsr     0x5dd22.l                       | +068
        addi.w  #0x80,0x22(a0)                  | +06e
        lea     S5_Barrel_089f46(pc),a1         | +074
        jsr     0x4ae.l                         | +078
        jsr     0x5dd22.l                       | +07e
        addi.w  #0xe0,0x22(a0)                  | +084
        lea     .L089f1e(pc),a1                 | +08a
        move.l  a1,(a6)                         | +08e
.L089f1e:
        clr.b   0x10e39a.l                      | +090
        jsr     0x2783a.l                       | +096
        cmpi.b  #0xff,0x21(a6)                  | +09c
        bne.w   .L089f44                        | +0a2
        move.w  #0x1028,d0                      | +0a6
        jsr     0x2352.l                        | +0aa
        jmp     0x518.l                         | +0b0
.L089f44:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  S5_Barrel_089f46  @ $089F46  (192 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Barrel_089f46, "ax", @progbits
        .global S5_Barrel_089f46
S5_Barrel_089f46:
        jsr     0x2783a.l                       | +000
        cmpi.w  #0x140,0x22(a6)                 | +006
        ble.w   .L089f58                        | +00c
        rts                                     | +010
.L089f58:
        move.w  #0xcd,d1                        | +012
        jsr     0x236e.l                        | +016
        move.w  #0x20,0x70(a6)                  | +01c
        move.b  #0xff,0x32(a6)                  | +022
        move.b  #0xff,0x33(a6)                  | +028
        move.b  #0x0,0x3a(a6)                   | +02e
        move.w  #0x1,0x66(a6)                   | +034
        move.w  #0x0,0x38(a6)                   | +03a
        lea     0x2ee808.l,a0                   | +040
        move.l  a0,0x48(a6)                     | +046
        lea     .L089f96(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L089f96:
        jsr     0x2783a.l                       | +050
        movea.l 0xc(a6),a0                      | +056
        cmpi.b  #0xff,0x21(a0)                  | +05a
        beq.w   .L089fe6                        | +060
        jsr     0x2870a.l                       | +064
        bcc.w   .L089fd2                        | +06a
        lea     0x5e766.l,a0                    | +06e
        jsr     0x5e770.l                       | +074
        lea     0x2edec2.l,a0                   | +07a
        jsr     0x28cd4.l                       | +080
        bclr    #0x3,0x13(a6)                   | +086
.L089fd2:
        jsr     0x28758.l                       | +08c
        bcc.w   .L08a004                        | +092
        movea.l 0xc(a6),a0                      | +096
        move.b  #0xff,0x21(a0)                  | +09a
.L089fe6:
        lea     0x2ef1be.l,a1                   | +0a0
        jsr     0x77c7e.l                       | +0a6
        lea     0x2ef1e2.l,a1                   | +0ac
        jsr     0x77c7e.l                       | +0b2
        jmp     0x518.l                         | +0b8
.L08a004:
        rts                                     | +0be

| ----------------------------------------------------------------------------
|  S5_RockRain_Spawn_08a006  @ $08A006  (88 B)
| ----------------------------------------------------------------------------
        .section .text.S5_RockRain_Spawn_08a006, "ax", @progbits
        .global S5_RockRain_Spawn_08a006
S5_RockRain_Spawn_08a006:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        lea     .L08a016(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L08a016:
        cmpi.w  #0x40,0x106f54.l                | +010
        bgt.w   .L08a024                        | +018
        rts                                     | +01c
.L08a024:
        move.w  #0xf,0x72(a6)                   | +01e
        lea     .L08a030(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L08a030:
        subi.w  #0x1,0x72(a6)                   | +02a
        bne.w   .L08a04a                        | +030
        lea     S5_FallingRock_08a05e(pc),a1    | +034
        jsr     0x4ae.l                         | +038
        move.w  #0xf,0x72(a6)                   | +03e
.L08a04a:
        cmpi.w  #0x180,0x106f54.l               | +044
        ble.w   .L08a05c                        | +04c
        jmp     0x518.l                         | +050
.L08a05c:
        rts                                     | +056

| ----------------------------------------------------------------------------
|  S5_FallingRock_08a05e  @ $08A05E  (174 B)
| ----------------------------------------------------------------------------
        .section .text.S5_FallingRock_08a05e, "ax", @progbits
        .global S5_FallingRock_08a05e
S5_FallingRock_08a05e:
        move.w  #0xce,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0xff,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0xc000,0x38(a6)                | +01c
        move.w  #0x0,0x22(a6)                   | +022
        move.w  #0xa0,0x24(a6)                  | +028
        jsr     0x267e2.l                       | +02e
        move.w  #0xf000,0x38(a6)                | +034
        bset    #0x6,0x12(a6)                   | +03a
        jsr     0x5e9b6.l                       | +040
        andi.w  #0xff,d0                        | +046
        add.w   d0,0x22(a6)                     | +04a
        andi.w  #0x3,d0                         | +04e
        move.w  d0,0x2a(a6)                     | +052
        addi.w  #0x10,0x2a(a6)                  | +056
        movem.w d0,-(a7)                        | +05c
        movea.l #0x2ee1c4,a0                    | +060
        lsl.w   #0x2,d0                         | +066
        movea.l (a0,d0.w),a0                    | +068
        cmpa.l  #0xffffffff,a0                  | +06c
        beq.w   .L08a0da                        | +072
        jsr     0x28cd4.l                       | +076
.L08a0da:
        movem.w (a7)+,d0                        | +07c
        andi.b  #0x1,d0                         | +080
        or.b    d0,0x3a(a6)                     | +084
        lea     .L08a0ec(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L08a0ec:
        move.w  0x2a(a6),d0                     | +08e
        add.w   d0,0x24(a6)                     | +092
        jsr     0x28d70.l                       | +096
        cmpi.w  #0x200,0x24(a6)                 | +09c
        ble.w   .L08a10a                        | +0a2
        jmp     0x518.l                         | +0a6
.L08a10a:
        rts                                     | +0ac

| ----------------------------------------------------------------------------
|  S5_TowerA_Init_08a10c  @ $08A10C  (472 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Init_08a10c, "ax", @progbits
        .global S5_TowerA_Init_08a10c
S5_TowerA_Init_08a10c:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x1dc,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0xd3,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0xff,0x32(a6)                  | +01e
        move.b  #0xff,0x33(a6)                  | +024
        move.b  #0x0,0x3a(a6)                   | +02a
        move.b  #0x0,0x20(a6)                   | +030
        move.w  #0x7fff,0x66(a6)                | +036
        move.w  #0xc000,0x38(a6)                | +03c
        cmpi.b  #0xc,0x106ece.l                 | +042
        beq.w   .L08a170                        | +04a
        lea     0xf0612.l,a1                    | +04e
        move.w  #0x82,d0                        | +054
        move.b  #0x0,0x82(a6)                   | +058
        jsr     0x4429e.l                       | +05e
.L08a170:
        lea     TowerPort_Init_08ae56(pc),a1    | +064
        jsr     0x4ae.l                         | +068
        jsr     0x5dd22.l                       | +06e
        move.b  #0x0,0x21(a0)                   | +074
        addi.w  #0x20,0x22(a0)                  | +07a
        lea     0x2ef3ac.l,a1                   | +080
        move.l  a1,0x70(a0)                     | +086
        move.l  #0xffffffff,0x74(a0)            | +08a
        lea     TowerPort_Init_08ae56(pc),a1    | +092
        jsr     0x4ae.l                         | +096
        jsr     0x5dd22.l                       | +09c
        move.b  #0x0,0x21(a0)                   | +0a2
        addi.w  #0x60,0x22(a0)                  | +0a8
        lea     0x2ef3c0.l,a1                   | +0ae
        move.l  a1,0x70(a0)                     | +0b4
        move.l  #0xffffffff,0x74(a0)            | +0b8
        lea     TowerPort_Init_08ae56(pc),a1    | +0c0
        jsr     0x4ae.l                         | +0c4
        jsr     0x5dd22.l                       | +0ca
        move.b  #0x0,0x21(a0)                   | +0d0
        addi.w  #0xa0,0x22(a0)                  | +0d6
        lea     0x2ef3d4.l,a1                   | +0dc
        move.l  a1,0x70(a0)                     | +0e2
        move.l  #0xffffffff,0x74(a0)            | +0e6
        move.b  #0x0,0x86(a6)                   | +0ee
        lea     0x77228.l,a1                    | +0f4
        jsr     0x4ae.l                         | +0fa
        jsr     0x5dd22.l                       | +100
        addi.w  #0xec,0x22(a0)                  | +106
        subi.w  #0x80,0x24(a0)                  | +10c
        move.b  #0x80,0x98(a0)                  | +112
        move.b  #0x86,0x99(a0)                  | +118
        lea     0x2ee1e8.l,a0                   | +11e
        jsr     0x28cd4.l                       | +124
        lea     0x2eeb50.l,a0                   | +12a
        move.l  a0,0x48(a6)                     | +130
        lea     .L08a246(pc),a1                 | +134
        move.l  a1,(a6)                         | +138
.L08a246:
        clr.b   0x10e39a.l                      | +13a
        jsr     0x2783a.l                       | +140
        cmpi.w  #0xb0,0x22(a6)                  | +146
        bgt.w   .L08a282                        | +14c
        jsr     0x28d70.l                       | +150
        bcc.w   .L08a282                        | +156
        jsr     0x2870a.l                       | +15a
        bcc.w   .L08a282                        | +160
        lea     0x2ee1d4.l,a0                   | +164
        jsr     0x28cd4.l                       | +16a
        jsr     0x28d70.l                       | +170
.L08a282:
        jsr     0x2870a.l                       | +176
        bcc.w   SetHandlerRts_08a31a            | +17c
        bclr    #0x3,0x13(a6)                   | +180
        cmpi.b  #0x1,0x58(a6)                   | +186
        beq.w   SetHandlerRts_08a31a            | +18c
        cmpi.b  #0xc,0x106ece.l                 | +190
        bne.w   S5_TowerA_Hit_08a2ec            | +198
        lea     0x2ef6a4.l,a1                   | +19c
        jsr     0x43fac.l                       | +1a2
        lea     0x2ef6c4.l,a1                   | +1a8
        jsr     0x43fac.l                       | +1ae
        lea     0x2ef6e4.l,a1                   | +1b4
        jsr     0x43fac.l                       | +1ba
        lea     0x2ef410.l,a2                   | +1c0
        jsr     0x5022a.l                       | +1c6
        lea     0x2ef474.l,a2                   | +1cc
        jsr     0x5022a.l                       | +1d2

| ----------------------------------------------------------------------------
|  S5_TowerA_Hit_08a2ec  @ $08A2EC  (40 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Hit_08a2ec, "ax", @progbits
        .global S5_TowerA_Hit_08a2ec
S5_TowerA_Hit_08a2ec:
        move.w  #0x1035,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x7808a.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd22.l                       | +016
        addi.w  #0xc0,0x22(a0)                  | +01c
        subi.w  #0x80,0x24(a0)                  | +022

| ----------------------------------------------------------------------------
|  S5_TowerA_Stage2_08a31c  @ $08A31C  (296 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Stage2_08a31c, "ax", @progbits
        .global S5_TowerA_Stage2_08a31c
S5_TowerA_Stage2_08a31c:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef3e8.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef44c.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6a4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ef286.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        lea     0x2eeba4.l,a0                   | +03c
        move.l  a0,0x48(a6)                     | +042
        lea     .L08a368(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L08a368:
        move.l  0x106f50.l,d0                   | +04c
        swap    d0                              | +052
        cmpi.w  #0x7d0,d0                       | +054
        bge.w   .L08a3ae                        | +058
        clr.b   0x10e39a.l                      | +05c
        jsr     0x2783a.l                       | +062
        jsr     0x28d70.l                       | +068
        jsr     0x2870a.l                       | +06e
        bcc.w   .L08a3a0                        | +074
        lea     0x5e766.l,a0                    | +078
        jsr     0x5e770.l                       | +07e
.L08a3a0:
        bclr    #0x3,0x13(a6)                   | +084
        bclr    #0x0,0x13(a6)                   | +08a
        rts                                     | +090
.L08a3ae:
        lea     0x2c06aa.l,a0                   | +092
        jsr     0x799de.l                       | +098
        move.w  d0,0x66(a6)                     | +09e
        lea     .L08a3c4(pc),a1                 | +0a2
        move.l  a1,(a6)                         | +0a6
.L08a3c4:
        clr.b   0x10e39a.l                      | +0a8
        jsr     0x2783a.l                       | +0ae
        jsr     0x28d70.l                       | +0b4
        bcc.w   .L08a3f6                        | +0ba
        jsr     0x2870a.l                       | +0be
        bcc.w   .L08a3f6                        | +0c4
        lea     0x2ee1d4.l,a0                   | +0c8
        jsr     0x28cd4.l                       | +0ce
        jsr     0x28d70.l                       | +0d4
.L08a3f6:
        jsr     0x2870a.l                       | +0da
        bcc.w   .L08a412                        | +0e0
        lea     0x5e766.l,a0                    | +0e4
        jsr     0x5e770.l                       | +0ea
        bclr    #0x3,0x13(a6)                   | +0f0
.L08a412:
        cmpi.w  #0x29a,0x66(a6)                 | +0f6
        bgt.w   SetHandlerRts_08a44a            | +0fc
        move.w  #0x1035,d0                      | +100
        jsr     0x2352.l                        | +104
        lea     0x7808a.l,a1                    | +10a
        jsr     0x4ae.l                         | +110
        jsr     0x5dd22.l                       | +116
        addi.w  #0xc0,0x22(a0)                  | +11c
        subi.w  #0x80,0x24(a0)                  | +122

| ----------------------------------------------------------------------------
|  S5_TowerA_Stage3_08a44c  @ $08A44C  (194 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Stage3_08a44c, "ax", @progbits
        .global S5_TowerA_Stage3_08a44c
S5_TowerA_Stage3_08a44c:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef3fc.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef460.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6c4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ef286.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        lea     .L08a48e(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L08a48e:
        clr.b   0x10e39a.l                      | +042
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        bcc.w   .L08a4c0                        | +054
        jsr     0x2870a.l                       | +058
        bcc.w   .L08a4c0                        | +05e
        lea     0x2ee1d4.l,a0                   | +062
        jsr     0x28cd4.l                       | +068
        jsr     0x28d70.l                       | +06e
.L08a4c0:
        jsr     0x2870a.l                       | +074
        bcc.w   .L08a4dc                        | +07a
        lea     0x5e766.l,a0                    | +07e
        jsr     0x5e770.l                       | +084
        bclr    #0x3,0x13(a6)                   | +08a
.L08a4dc:
        cmpi.w  #0x14d,0x66(a6)                 | +090
        bgt.w   SetHandlerRts_08a44a            | +096
        move.w  #0x1035,d0                      | +09a
        jsr     0x2352.l                        | +09e
        lea     0x7808a.l,a1                    | +0a4
        jsr     0x4ae.l                         | +0aa
        jsr     0x5dd22.l                       | +0b0
        addi.w  #0xc0,0x22(a0)                  | +0b6
        subi.w  #0x80,0x24(a0)                  | +0bc

| ----------------------------------------------------------------------------
|  S5_TowerA_Stage4_08a516  @ $08A516  (178 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Stage4_08a516, "ax", @progbits
        .global S5_TowerA_Stage4_08a516
S5_TowerA_Stage4_08a516:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef410.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef474.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6e4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ee212.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
        lea     0x2ef286.l,a1                   | +03c
        jsr     0x77c7e.l                       | +042
        lea     .L08a564(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L08a564:
        clr.b   0x10e39a.l                      | +04e
        jsr     0x2783a.l                       | +054
        jsr     0x28d70.l                       | +05a
        bcc.w   .L08a596                        | +060
        jsr     0x2870a.l                       | +064
        bcc.w   .L08a596                        | +06a
        lea     0x2ee1fe.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
        jsr     0x28d70.l                       | +07a
.L08a596:
        jsr     0x2870a.l                       | +080
        bcc.w   .L08a5b2                        | +086
        lea     0x5e766.l,a0                    | +08a
        jsr     0x5e770.l                       | +090
        bclr    #0x3,0x13(a6)                   | +096
.L08a5b2:
        jsr     0x28758.l                       | +09c
        bcc.w   S5_TowerA_Rts_08a74c | +0a2
        move.l  #0x5000,d0                      | +0a6
        jsr     0x51a28.l                       | +0ac

| ----------------------------------------------------------------------------
|  S5_TowerA_Destroy_08a5c8  @ $08A5C8  (390 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerA_Destroy_08a5c8, "ax", @progbits
        .global S5_TowerA_Destroy_08a5c8
S5_TowerA_Destroy_08a5c8:
        jsr     0x434ce.l                       | +000
        move.w  #0x102f,d0                      | +006
        jsr     0x2352.l                        | +00a
        lea     0x7808a.l,a1                    | +010
        jsr     0x4ae.l                         | +016
        jsr     0x5dd22.l                       | +01c
        addi.w  #0xc0,0x22(a0)                  | +022
        subi.w  #0x80,0x24(a0)                  | +028
        lea     0x7808a.l,a1                    | +02e
        jsr     0x4ae.l                         | +034
        jsr     0x5dd22.l                       | +03a
        addi.w  #0x30,0x22(a0)                  | +040
        subi.w  #0x30,0x24(a0)                  | +046
        lea     0x7808a.l,a1                    | +04c
        jsr     0x4ae.l                         | +052
        jsr     0x5dd22.l                       | +058
        addi.w  #0x70,0x22(a0)                  | +05e
        subi.w  #0x10,0x24(a0)                  | +064
        lea     0x7808a.l,a1                    | +06a
        jsr     0x4ae.l                         | +070
        jsr     0x5dd22.l                       | +076
        addi.w  #0x90,0x22(a0)                  | +07c
        subi.w  #0x40,0x24(a0)                  | +082
        lea     0x7808a.l,a1                    | +088
        jsr     0x4ae.l                         | +08e
        jsr     0x5dd22.l                       | +094
        addi.w  #0xc0,0x22(a0)                  | +09a
        subi.w  #0x30,0x24(a0)                  | +0a0
        lea     0x7808a.l,a1                    | +0a6
        jsr     0x4ae.l                         | +0ac
        jsr     0x5dd22.l                       | +0b2
        addi.w  #0x100,0x22(a0)                 | +0b8
        subi.w  #0x40,0x24(a0)                  | +0be
        lea     0x2ef286.l,a1                   | +0c4
        jsr     0x77c7e.l                       | +0ca
        lea     0x2ef438.l,a2                   | +0d0
        jsr     0x5022a.l                       | +0d6
        lea     0x2ef424.l,a2                   | +0dc
        jsr     0x5022a.l                       | +0e2
        lea     0x2ef488.l,a2                   | +0e8
        jsr     0x5022a.l                       | +0ee
        lea     0x2ef63e.l,a1                   | +0f4
        jsr     0x43fac.l                       | +0fa
        lea     0x2ef66c.l,a1                   | +100
        jsr     0x43fac.l                       | +106
        lea     0x2ef688.l,a1                   | +10c
        jsr     0x43fac.l                       | +112
        lea     0x2ef696.l,a1                   | +118
        jsr     0x43fac.l                       | +11e
        lea     0x2ef704.l,a1                   | +124
        jsr     0x43fac.l                       | +12a
        move.b  #0xff,0x82(a6)                  | +130
        move.b  #0xff,0x86(a6)                  | +136
        lea     Wreck_FlagSet_08ae0c(pc),a1     | +13c
        jsr     0x4ae.l                         | +140
        jsr     0x5dd22.l                       | +146
        addi.w  #0x20,0x22(a0)                  | +14c
        lea     Wreck_SmokeRise_08b07c(pc),a1   | +152
        jsr     0x4ae.l                         | +156
        jsr     0x5dd22.l                       | +15c
        addi.w  #0x90,0x22(a0)                  | +162
        subi.w  #0x10,0x24(a0)                  | +168
        move.b  #0x0,0x21(a0)                   | +16e
        move.b  #0xff,0x20(a6)                  | +174
        jsr     SprCb_Lamp2Off_08b490(pc)       | +17a
        jmp     0x518.l                         | +17e
        .global S5_TowerA_Rts_08a74c
S5_TowerA_Rts_08a74c:
        rts                                     | +184

| ----------------------------------------------------------------------------
|  S5_TowerB_Init_08a74e  @ $08A74E  (554 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Init_08a74e, "ax", @progbits
        .global S5_TowerB_Init_08a74e
S5_TowerB_Init_08a74e:
        movea.l 0x3c(a6),a1                     | +000
        jsr     0x2942a.l                       | +004
        move.w  #0x1dc,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0xd3,d1                        | +014
        jsr     0x236e.l                        | +018
        move.b  #0xff,0x32(a6)                  | +01e
        move.b  #0xff,0x33(a6)                  | +024
        move.b  #0x0,0x3a(a6)                   | +02a
        move.b  #0x0,0x20(a6)                   | +030
        move.w  #0x7fff,0x66(a6)                | +036
        move.w  #0xc000,0x38(a6)                | +03c
        cmpi.b  #0xc,0x106ece.l                 | +042
        beq.w   .L08a7b2                        | +04a
        lea     0xf0638.l,a1                    | +04e
        move.w  #0x82,d0                        | +054
        move.b  #0x0,0x82(a6)                   | +058
        jsr     0x4429e.l                       | +05e
.L08a7b2:
        lea     TowerPort_Init_08ae56(pc),a1    | +064
        jsr     0x4ae.l                         | +068
        jsr     0x5dd22.l                       | +06e
        move.b  #0x0,0x21(a0)                   | +074
        subi.w  #0x20,0x22(a0)                  | +07a
        lea     0x2ef5c8.l,a1                   | +080
        move.l  a1,0x70(a0)                     | +086
        lea     0x2ef5dc.l,a1                   | +08a
        move.l  a1,0x74(a0)                     | +090
        lea     TowerPort_Init_08ae56(pc),a1    | +094
        jsr     0x4ae.l                         | +098
        jsr     0x5dd22.l                       | +09e
        move.b  #0x0,0x21(a0)                   | +0a4
        addi.w  #0x20,0x22(a0)                  | +0aa
        lea     0x2ef49c.l,a1                   | +0b0
        move.l  a1,0x70(a0)                     | +0b6
        lea     0x2ef4b0.l,a1                   | +0ba
        move.l  a1,0x74(a0)                     | +0c0
        lea     TowerPort_Init_08ae56(pc),a1    | +0c4
        jsr     0x4ae.l                         | +0c8
        jsr     0x5dd22.l                       | +0ce
        move.b  #0x0,0x21(a0)                   | +0d4
        addi.w  #0x60,0x22(a0)                  | +0da
        lea     0x2ef4c4.l,a1                   | +0e0
        move.l  a1,0x70(a0)                     | +0e6
        lea     0x2ef4d8.l,a1                   | +0ea
        move.l  a1,0x74(a0)                     | +0f0
        lea     TowerPort_Init_08ae56(pc),a1    | +0f4
        jsr     0x4ae.l                         | +0f8
        jsr     0x5dd22.l                       | +0fe
        move.b  #0x1,0x21(a0)                   | +104
        addi.w  #0xa0,0x22(a0)                  | +10a
        lea     0x2ef4ec.l,a1                   | +110
        move.l  a1,0x70(a0)                     | +116
        lea     0x2ef500.l,a1                   | +11a
        move.l  a1,0x74(a0)                     | +120
        move.b  #0x0,0x86(a6)                   | +124
        lea     0x77228.l,a1                    | +12a
        jsr     0x4ae.l                         | +130
        jsr     0x5dd22.l                       | +136
        addi.w  #0x110,0x22(a0)                 | +13c
        subi.w  #0x48,0x24(a0)                  | +142
        move.b  #0x49,0x98(a0)                  | +148
        move.b  #0x86,0x99(a0)                  | +14e
        lea     0x2ee23c.l,a0                   | +154
        jsr     0x28cd4.l                       | +15a
        lea     0x2eeb50.l,a0                   | +160
        move.l  a0,0x48(a6)                     | +166
        move.b  #0x0,0x87(a6)                   | +16a
        lea     .L08a8c4(pc),a1                 | +170
        move.l  a1,(a6)                         | +174
.L08a8c4:
        clr.b   0x10e39a.l                      | +176
        jsr     0x2783a.l                       | +17c
        cmpi.w  #0xb0,0x22(a6)                  | +182
        bgt.w   .L08a90c                        | +188
        move.b  #0x0,0x87(a6)                   | +18c
        jsr     0x28d70.l                       | +192
        bcc.w   .L08a90c                        | +198
        jsr     0x2870a.l                       | +19c
        bcc.w   .L08a90c                        | +1a2
        lea     0x2ee228.l,a0                   | +1a6
        jsr     0x28cd4.l                       | +1ac
        move.b  #0xff,0x87(a6)                  | +1b2
        jsr     0x28d70.l                       | +1b8
.L08a90c:
        jsr     0x2870a.l                       | +1be
        bcc.w   SetHandlerRts_08a9ae            | +1c4
        bclr    #0x3,0x13(a6)                   | +1c8
        cmpi.b  #0x1,0x58(a6)                   | +1ce
        beq.w   SetHandlerRts_08a9ae            | +1d4
        cmpi.b  #0x2,0x58(a6)                   | +1d8
        beq.w   SetHandlerRts_08a9ae            | +1de
        cmpi.b  #0xc,0x106ece.l                 | +1e2
        bne.w   S5_TowerB_Hit_08a980            | +1ea
        lea     0x2ef6a4.l,a1                   | +1ee
        jsr     0x43fac.l                       | +1f4
        lea     0x2ef6c4.l,a1                   | +1fa
        jsr     0x43fac.l                       | +200
        lea     0x2ef6e4.l,a1                   | +206
        jsr     0x43fac.l                       | +20c
        lea     0x2ef53c.l,a2                   | +212
        jsr     0x5022a.l                       | +218
        lea     0x2ef5a0.l,a2                   | +21e
        jsr     0x5022a.l                       | +224

| ----------------------------------------------------------------------------
|  S5_TowerB_Hit_08a980  @ $08A980  (40 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Hit_08a980, "ax", @progbits
        .global S5_TowerB_Hit_08a980
S5_TowerB_Hit_08a980:
        move.w  #0x1035,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x7808a.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd22.l                       | +016
        addi.w  #0xc0,0x22(a0)                  | +01c
        subi.w  #0x80,0x24(a0)                  | +022

| ----------------------------------------------------------------------------
|  S5_TowerB_Stage2_08a9b0  @ $08A9B0  (314 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Stage2_08a9b0, "ax", @progbits
        .global S5_TowerB_Stage2_08a9b0
S5_TowerB_Stage2_08a9b0:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef514.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef578.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6a4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ef286.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        lea     0x2eeba4.l,a0                   | +03c
        move.l  a0,0x48(a6)                     | +042
        move.b  #0x0,0x87(a6)                   | +046
        lea     .L08aa02(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L08aa02:
        move.l  0x106f50.l,d0                   | +052
        swap    d0                              | +058
        cmpi.w  #0x950,d0                       | +05a
        bge.w   .L08aa48                        | +05e
        clr.b   0x10e39a.l                      | +062
        jsr     0x2783a.l                       | +068
        jsr     0x28d70.l                       | +06e
        jsr     0x2870a.l                       | +074
        bcc.w   .L08aa3a                        | +07a
        lea     0x5e766.l,a0                    | +07e
        jsr     0x5e770.l                       | +084
.L08aa3a:
        bclr    #0x3,0x13(a6)                   | +08a
        bclr    #0x0,0x13(a6)                   | +090
        rts                                     | +096
.L08aa48:
        lea     0x2c06aa.l,a0                   | +098
        jsr     0x799de.l                       | +09e
        move.w  d0,0x66(a6)                     | +0a4
        lea     .L08aa5e(pc),a1                 | +0a8
        move.l  a1,(a6)                         | +0ac
.L08aa5e:
        clr.b   0x10e39a.l                      | +0ae
        jsr     0x2783a.l                       | +0b4
        move.b  #0x0,0x87(a6)                   | +0ba
        jsr     0x28d70.l                       | +0c0
        bcc.w   .L08aa9c                        | +0c6
        jsr     0x2870a.l                       | +0ca
        bcc.w   .L08aa9c                        | +0d0
        lea     0x2ee228.l,a0                   | +0d4
        jsr     0x28cd4.l                       | +0da
        move.b  #0xff,0x87(a6)                  | +0e0
        jsr     0x28d70.l                       | +0e6
.L08aa9c:
        jsr     0x2870a.l                       | +0ec
        bcc.w   .L08aab8                        | +0f2
        lea     0x5e766.l,a0                    | +0f6
        jsr     0x5e770.l                       | +0fc
        bclr    #0x3,0x13(a6)                   | +102
.L08aab8:
        cmpi.w  #0x29a,0x66(a6)                 | +108
        bgt.w   SetHandlerRts_08aaf0            | +10e
        move.w  #0x1035,d0                      | +112
        jsr     0x2352.l                        | +116
        lea     0x7808a.l,a1                    | +11c
        jsr     0x4ae.l                         | +122
        jsr     0x5dd22.l                       | +128
        addi.w  #0xc0,0x22(a0)                  | +12e
        subi.w  #0x80,0x24(a0)                  | +134

| ----------------------------------------------------------------------------
|  S5_TowerB_Stage3_08aaf2  @ $08AAF2  (218 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Stage3_08aaf2, "ax", @progbits
        .global S5_TowerB_Stage3_08aaf2
S5_TowerB_Stage3_08aaf2:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef528.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef58c.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6c4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ef286.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        lea     0x2ee266.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L08ab40(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L08ab40:
        clr.b   0x10e39a.l                      | +04e
        jsr     0x2783a.l                       | +054
        move.b  #0x0,0x87(a6)                   | +05a
        jsr     0x28d70.l                       | +060
        bcc.w   .L08ab7e                        | +066
        jsr     0x2870a.l                       | +06a
        bcc.w   .L08ab7e                        | +070
        lea     0x2ee252.l,a0                   | +074
        jsr     0x28cd4.l                       | +07a
        move.b  #0xff,0x87(a6)                  | +080
        jsr     0x28d70.l                       | +086
.L08ab7e:
        jsr     0x2870a.l                       | +08c
        bcc.w   .L08ab9a                        | +092
        lea     0x5e766.l,a0                    | +096
        jsr     0x5e770.l                       | +09c
        bclr    #0x3,0x13(a6)                   | +0a2
.L08ab9a:
        cmpi.w  #0x14d,0x66(a6)                 | +0a8
        bgt.w   SetHandlerRts_08aaf0            | +0ae
        move.w  #0x1035,d0                      | +0b2
        jsr     0x2352.l                        | +0b6
        lea     0x7808a.l,a1                    | +0bc
        jsr     0x4ae.l                         | +0c2
        jsr     0x5dd22.l                       | +0c8
        addi.w  #0xc0,0x22(a0)                  | +0ce
        subi.w  #0x80,0x24(a0)                  | +0d4

| ----------------------------------------------------------------------------
|  S5_TowerB_Stage4_08abd4  @ $08ABD4  (190 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Stage4_08abd4, "ax", @progbits
        .global S5_TowerB_Stage4_08abd4
S5_TowerB_Stage4_08abd4:
        jsr     0x434dc.l                       | +000
        jsr     0x2783a.l                       | +006
        lea     0x2ef53c.l,a2                   | +00c
        jsr     0x5022a.l                       | +012
        lea     0x2ef5a0.l,a2                   | +018
        jsr     0x5022a.l                       | +01e
        lea     0x2ef6e4.l,a1                   | +024
        jsr     0x43fac.l                       | +02a
        lea     0x2ef286.l,a1                   | +030
        jsr     0x77c7e.l                       | +036
        lea     0x2ee290.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L08ac22(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L08ac22:
        clr.b   0x10e39a.l                      | +04e
        jsr     0x2783a.l                       | +054
        move.b  #0x0,0x87(a6)                   | +05a
        jsr     0x28d70.l                       | +060
        bcc.w   .L08ac60                        | +066
        jsr     0x2870a.l                       | +06a
        bcc.w   .L08ac60                        | +070
        lea     0x2ee27c.l,a0                   | +074
        jsr     0x28cd4.l                       | +07a
        move.b  #0xff,0x87(a6)                  | +080
        jsr     0x28d70.l                       | +086
.L08ac60:
        jsr     0x2870a.l                       | +08c
        bcc.w   .L08ac7c                        | +092
        lea     0x5e766.l,a0                    | +096
        jsr     0x5e770.l                       | +09c
        bclr    #0x3,0x13(a6)                   | +0a2
.L08ac7c:
        jsr     0x28758.l                       | +0a8
        bcc.w   S5_TowerB_Rts_08ae0a | +0ae
        move.l  #0x5000,d0                      | +0b2
        jsr     0x51a28.l                       | +0b8

| ----------------------------------------------------------------------------
|  S5_TowerB_Destroy_08ac92  @ $08AC92  (378 B)
| ----------------------------------------------------------------------------
        .section .text.S5_TowerB_Destroy_08ac92, "ax", @progbits
        .global S5_TowerB_Destroy_08ac92
S5_TowerB_Destroy_08ac92:
        jsr     0x434ce.l                       | +000
        move.w  #0x102f,d0                      | +006
        jsr     0x2352.l                        | +00a
        lea     0x7808a.l,a1                    | +010
        jsr     0x4ae.l                         | +016
        jsr     0x5dd22.l                       | +01c
        addi.w  #0xc0,0x22(a0)                  | +022
        subi.w  #0x80,0x24(a0)                  | +028
        lea     0x7808a.l,a1                    | +02e
        jsr     0x4ae.l                         | +034
        jsr     0x5dd22.l                       | +03a
        addi.w  #0x30,0x22(a0)                  | +040
        subi.w  #0x30,0x24(a0)                  | +046
        lea     0x7808a.l,a1                    | +04c
        jsr     0x4ae.l                         | +052
        jsr     0x5dd22.l                       | +058
        addi.w  #0x70,0x22(a0)                  | +05e
        subi.w  #0x10,0x24(a0)                  | +064
        lea     0x7808a.l,a1                    | +06a
        jsr     0x4ae.l                         | +070
        jsr     0x5dd22.l                       | +076
        addi.w  #0x90,0x22(a0)                  | +07c
        subi.w  #0x40,0x24(a0)                  | +082
        lea     0x7808a.l,a1                    | +088
        jsr     0x4ae.l                         | +08e
        jsr     0x5dd22.l                       | +094
        addi.w  #0xc0,0x22(a0)                  | +09a
        subi.w  #0x30,0x24(a0)                  | +0a0
        lea     0x7808a.l,a1                    | +0a6
        jsr     0x4ae.l                         | +0ac
        jsr     0x5dd22.l                       | +0b2
        addi.w  #0x100,0x22(a0)                 | +0b8
        subi.w  #0x40,0x24(a0)                  | +0be
        lea     0x2ef286.l,a1                   | +0c4
        jsr     0x77c7e.l                       | +0ca
        lea     0x2ef564.l,a2                   | +0d0
        jsr     0x5022a.l                       | +0d6
        lea     0x2ef550.l,a2                   | +0dc
        jsr     0x5022a.l                       | +0e2
        lea     0x2ef5b4.l,a2                   | +0e8
        jsr     0x5022a.l                       | +0ee
        lea     0x2ef740.l,a1                   | +0f4
        jsr     0x43fac.l                       | +0fa
        lea     0x2ef77a.l,a1                   | +100
        jsr     0x43fac.l                       | +106
        lea     0x2ef79a.l,a1                   | +10c
        jsr     0x43fac.l                       | +112
        lea     0x2ef704.l,a1                   | +118
        jsr     0x43fac.l                       | +11e
        move.b  #0xff,0x82(a6)                  | +124
        move.b  #0xff,0x86(a6)                  | +12a
        lea     Wreck_FlagSet_08ae0c(pc),a1     | +130
        jsr     0x4ae.l                         | +134
        jsr     0x5dd22.l                       | +13a
        addi.w  #0x20,0x22(a0)                  | +140
        lea     Wreck_SmokeRise_08b07c(pc),a1   | +146
        jsr     0x4ae.l                         | +14a
        jsr     0x5dd22.l                       | +150
        addi.w  #0x90,0x22(a0)                  | +156
        subi.w  #0x10,0x24(a0)                  | +15c
        move.b  #0x1,0x21(a0)                   | +162
        move.b  #0xff,0x20(a6)                  | +168
        jsr     SprCb_Lamp3Off_08b50e(pc)       | +16e
        jmp     0x518.l                         | +172
        .global S5_TowerB_Rts_08ae0a
S5_TowerB_Rts_08ae0a:
        rts                                     | +178

| ----------------------------------------------------------------------------
|  Wreck_FlagSet_08ae0c  @ $08AE0C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Wreck_FlagSet_08ae0c, "ax", @progbits
        .global Wreck_FlagSet_08ae0c
Wreck_FlagSet_08ae0c:
        lea     0x2eeec8.l,a0                   | +000
        move.l  a0,0x4c(a6)                     | +006
        jsr     0x283ca.l                       | +00a
        jsr     0x283ca.l                       | +010
        jsr     0x283d8.l                       | +016
        move.b  #0x1,0x10e39e.l                 | +01c

| ----------------------------------------------------------------------------
|  Wreck_FlagClear_08ae38  @ $08AE38  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Wreck_FlagClear_08ae38, "ax", @progbits
        .global Wreck_FlagClear_08ae38
Wreck_FlagClear_08ae38:
        move.b  #0x0,0x10e39e.l                 | +000
        lea     0xffff.w,a0                     | +008
        move.l  a0,0x4c(a6)                     | +00c
        jsr     0x283ca.l                       | +010
        jmp     0x518.l                         | +016

| ----------------------------------------------------------------------------
|  Rts_08ae54  @ $08AE54  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08ae54, "ax", @progbits
        .global Rts_08ae54
Rts_08ae54:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TowerPort_Init_08ae56  @ $08AE56  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TowerPort_Init_08ae56, "ax", @progbits
        .global TowerPort_Init_08ae56
TowerPort_Init_08ae56:
        cmpi.w  #0x140,0x22(a6)                 | +000
        blt.w   TowerPort_Active_08ae68         | +006

| ----------------------------------------------------------------------------
|  TowerPort_Active_08ae68  @ $08AE68  (246 B)
| ----------------------------------------------------------------------------
        .section .text.TowerPort_Active_08ae68, "ax", @progbits
        .global TowerPort_Active_08ae68
TowerPort_Active_08ae68:
        move.w  #0x14,0x66(a6)                  | +000
        lea     0x2eebf8.l,a0                   | +006
        move.l  a0,0x48(a6)                     | +00c
        lea     0x2ee5be.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        cmpi.b  #0x0,0x21(a6)                   | +01c
        beq.w   .L08aec0                        | +022
        move.w  #0x1dd,d1                       | +026
        jsr     0x236e.l                        | +02a
        move.w  #0xd4,d1                        | +030
        jsr     0x236e.l                        | +034
        lea     0x2ee58e.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        move.w  #0x8000,0x38(a6)                | +046
        jsr     0x267e2.l                       | +04c
        jsr     0x27cee.l                       | +052
.L08aec0:
        lea     .L08aec6(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L08aec6:
        movea.l 0xc(a6),a0                      | +05e
        cmpi.b  #0xff,0x20(a0)                  | +062
        beq.w   .L08af58                        | +068
        move.w  0x18(a6),0x14(a6)               | +06c
        cmpi.b  #0x0,0x21(a6)                   | +072
        beq.w   .L08aef4                        | +078
        cmpi.b  #0xff,0x87(a0)                  | +07c
        bne.w   .L08aef4                        | +082
        move.w  0x16(a6),0x14(a6)               | +086
.L08aef4:
        jsr     0x2783a.l                       | +08c
        jsr     0x28d70.l                       | +092
        jsr     0x2870a.l                       | +098
        bcc.w   .L08af1c                        | +09e
        lea     0x5e766.l,a0                    | +0a2
        jsr     0x5e770.l                       | +0a8
        bclr    #0x3,0x13(a6)                   | +0ae
.L08af1c:
        jsr     0x28758.l                       | +0b4
        bcc.w   SetHandlerRts_08af66            | +0ba
        bclr    #0x0,0x13(a6)                   | +0be
        move.w  #0x1027,d0                      | +0c4
        jsr     0x2352.l                        | +0c8
        movea.l 0x70(a6),a2                     | +0ce
        jsr     0x5022a.l                       | +0d2
        lea     0x2ef274.l,a1                   | +0d8
        jsr     0x77c7e.l                       | +0de
        cmpi.l  #0xffffffff,0x74(a6)            | +0e4
        bne.w   SetTaskHandler_08af60           | +0ec
.L08af58:
        jmp     0x518.l                         | +0f0

| ----------------------------------------------------------------------------
|  Rts_08af5e  @ $08AF5E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_08af5e, "ax", @progbits
        .global Rts_08af5e
Rts_08af5e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TowerPort_Stage2_08af68  @ $08AF68  (196 B)
| ----------------------------------------------------------------------------
        .section .text.TowerPort_Stage2_08af68, "ax", @progbits
        .global TowerPort_Stage2_08af68
TowerPort_Stage2_08af68:
        move.w  #0x14,0x66(a6)                  | +000
        cmpi.b  #0x0,0x21(a6)                   | +006
        beq.w   .L08af84                        | +00c
        lea     0x2ee59e.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L08af84:
        lea     .L08af8a(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L08af8a:
        movea.l 0xc(a6),a0                      | +022
        cmpi.b  #0xff,0x20(a0)                  | +026
        beq.w   JmpToScheduler_08b034           | +02c
        move.w  0x18(a6),0x14(a6)               | +030
        cmpi.b  #0x0,0x21(a6)                   | +036
        beq.w   .L08afb8                        | +03c
        cmpi.b  #0xff,0x87(a0)                  | +040
        bne.w   .L08afb8                        | +046
        move.w  0x16(a6),0x14(a6)               | +04a
.L08afb8:
        jsr     0x2783a.l                       | +050
        jsr     0x28d70.l                       | +056
        jsr     0x2870a.l                       | +05c
        bcc.w   .L08afe0                        | +062
        lea     0x5e766.l,a0                    | +066
        jsr     0x5e770.l                       | +06c
        bclr    #0x3,0x13(a6)                   | +072
.L08afe0:
        jsr     0x28758.l                       | +078
        bcc.w   JmpSchedRts_08b03a              | +07e
        move.l  #0x1000,d0                      | +082
        jsr     0x51a28.l                       | +088
        move.w  #0x1027,d0                      | +08e
        jsr     0x2352.l                        | +092
        movea.l 0x74(a6),a2                     | +098
        jsr     0x5022a.l                       | +09c
        lea     0x2ef274.l,a1                   | +0a2
        jsr     0x77c7e.l                       | +0a8
        cmpi.b  #0x0,0x21(a6)                   | +0ae
        beq.w   JmpToScheduler_08b034           | +0b4
        lea     0x2ee5ae.l,a0                   | +0b8
        jsr     0x28cd4.l                       | +0be

| ----------------------------------------------------------------------------
|  TowerPort_Idle_08b03c  @ $08B03C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TowerPort_Idle_08b03c, "ax", @progbits
        .global TowerPort_Idle_08b03c
TowerPort_Idle_08b03c:
        lea     .L08b042(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L08b042:
        movea.l 0xc(a6),a0                      | +006
        cmpi.b  #0xff,0x20(a0)                  | +00a
        beq.w   JmpToScheduler_08b074           | +010
        move.w  0x18(a6),0x14(a6)               | +014
        cmpi.b  #0xff,0x87(a0)                  | +01a
        bne.w   .L08b066                        | +020
        move.w  0x16(a6),0x14(a6)               | +024
.L08b066:
        jsr     0x2783a.l                       | +02a

| ----------------------------------------------------------------------------
|  Wreck_SmokeRise_08b07c  @ $08B07C  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Wreck_SmokeRise_08b07c, "ax", @progbits
        .global Wreck_SmokeRise_08b07c
Wreck_SmokeRise_08b07c:
        move.w  #0xd5,d1                        | +000
        jsr     0x236e.l                        | +004
        move.b  #0xff,0x32(a6)                  | +00a
        move.b  #0x50,0x33(a6)                  | +010
        move.b  #0x0,0x3a(a6)                   | +016
        move.w  #0x0,0x38(a6)                   | +01c
        jsr     0x267e2.l                       | +022
        move.w  #0xfffa,0x2e(a6)                | +028
        clr.l   d0                              | +02e
        move.b  0x21(a6),d0                     | +030
        movea.l #0x2ee2be,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L08b0cc                        | +046
        jsr     0x28cd4.l                       | +04a
.L08b0cc:
        move.w  #0x1026,d0                      | +050
        jsr     0x2352.l                        | +054
        lea     .L08b0dc(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08b0dc:
        jsr     0x27cee.l                       | +060
        cmpi.b  #0xff,0x33(a6)                  | +066
        beq.w   .L08b0f2                        | +06c
        addi.b  #0x1,0x33(a6)                   | +070
.L08b0f2:
        jsr     0x28d70.l                       | +076
        cmpi.w  #0x170,0x24(a6)                 | +07c
        bgt.w   SetHandlerRts_08b108            | +082

| ----------------------------------------------------------------------------
|  Wreck_SparkBurst_08b10a  @ $08B10A  (232 B)
| ----------------------------------------------------------------------------
        .section .text.Wreck_SparkBurst_08b10a, "ax", @progbits
        .global Wreck_SparkBurst_08b10a
Wreck_SparkBurst_08b10a:
        move.w  #0x1051,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     Wreck_Spark_08b1f2(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd22.l                       | +014
        addi.w  #0x70,0x22(a0)                  | +01a
        move.w  #0x125,0x24(a0)                 | +020
        move.w  #0x0,0x72(a0)                   | +026
        lea     Wreck_Spark_08b1f2(pc),a1       | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd22.l                       | +036
        addi.w  #0x40,0x22(a0)                  | +03c
        move.w  #0x125,0x24(a0)                 | +042
        move.w  #0x6,0x72(a0)                   | +048
        lea     Wreck_Spark_08b1f2(pc),a1       | +04e
        jsr     0x4ae.l                         | +052
        jsr     0x5dd22.l                       | +058
        addi.w  #0x10,0x22(a0)                  | +05e
        move.w  #0x125,0x24(a0)                 | +064
        move.w  #0xc,0x72(a0)                   | +06a
        lea     Wreck_Spark_08b1f2(pc),a1       | +070
        jsr     0x4ae.l                         | +074
        jsr     0x5dd22.l                       | +07a
        subi.w  #0x20,0x22(a0)                  | +080
        move.w  #0x125,0x24(a0)                 | +086
        move.w  #0x12,0x72(a0)                  | +08c
        lea     Wreck_Spark_08b1f2(pc),a1       | +092
        jsr     0x4ae.l                         | +096
        jsr     0x5dd22.l                       | +09c
        subi.w  #0x50,0x22(a0)                  | +0a2
        move.w  #0x125,0x24(a0)                 | +0a8
        move.w  #0x18,0x72(a0)                  | +0ae
        lea     .L08b1c4(pc),a1                 | +0b4
        move.l  a1,(a6)                         | +0b8
.L08b1c4:
        jsr     0x27cee.l                       | +0ba
        cmpi.b  #0xff,0x33(a6)                  | +0c0
        beq.w   .L08b1da                        | +0c6
        addi.b  #0x1,0x33(a6)                   | +0ca
.L08b1da:
        jsr     0x28d70.l                       | +0d0
        cmpi.w  #0x80,0x24(a6)                  | +0d6
        bgt.w   .L08b1f0                        | +0dc
        jmp     0x518.l                         | +0e0
.L08b1f0:
        rts                                     | +0e6

| ----------------------------------------------------------------------------
|  Wreck_Spark_08b1f2  @ $08B1F2  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Wreck_Spark_08b1f2, "ax", @progbits
        .global Wreck_Spark_08b1f2
Wreck_Spark_08b1f2:
        jsr     0x2783a.l                       | +000
        subq.w  #0x1,0x72(a6)                   | +006
        cmpi.w  #0x0,0x72(a6)                   | +00a
        ble.w   .L08b208                        | +010
        rts                                     | +014
.L08b208:
        move.w  #0x139,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.b  #0xff,0x32(a6)                  | +020
        move.b  #0xff,0x33(a6)                  | +026
        lea     0x2ded26.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L08b230(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L08b230:
        jsr     0x44048.l                       | +03e
        jsr     0x28d70.l                       | +044
        bcs.w   .L08b250                        | +04a
        lea     0x2ef7a8.l,a0                   | +04e
        jsr     0x5dd56.l                       | +054
        bcc.w   .L08b256                        | +05a
.L08b250:
        jmp     0x518.l                         | +05e
.L08b256:
        rts                                     | +064

| ----------------------------------------------------------------------------
|  Proj_Thrown_08b258  @ $08B258  (244 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Thrown_08b258, "ax", @progbits
        .global Proj_Thrown_08b258
Proj_Thrown_08b258:
        cmpi.b  #0x0,0x9c(a6)                   | +000
        bne.w   .L08b270                        | +006
        move.w  #0x162,d1                       | +00a
        jsr     0x236e.l                        | +00e
        bra.w   .L08b27a                        | +014
.L08b270:
        move.w  #0xd8,d1                        | +018
        jsr     0x236e.l                        | +01c
.L08b27a:
        move.b  #0xff,0x32(a6)                  | +022
        move.b  #0xff,0x33(a6)                  | +028
        jsr     0x267e2.l                       | +02e
        jsr     0x8f002.l                       | +034
        jsr     0x8f010.l                       | +03a
        cmpi.b  #0x1,0x9a(a6)                   | +040
        bne.w   .L08b2b2                        | +046
        bset    #0x6,0x12(a6)                   | +04a
        move.w  #0xffff,0x38(a6)                | +050
        bra.w   .L08b2b8                        | +056
.L08b2b2:
        move.w  #0x8000,0x38(a6)                | +05a
.L08b2b8:
        clr.l   d0                              | +060
        move.b  0x9d(a6),d0                     | +062
        movea.l #0x2ee7a8,a0                    | +066
        lsl.w   #0x2,d0                         | +06c
        movea.l (a0,d0.w),a0                    | +06e
        cmpa.l  #0xffffffff,a0                  | +072
        beq.w   .L08b2da                        | +078
        jsr     0x28cd4.l                       | +07c
.L08b2da:
        lea     .L08b2e0(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L08b2e0:
        jsr     0x27cee.l                       | +088
        jsr     0x28d70.l                       | +08e
        cmpi.b  #0x0,0x9b(a6)                   | +094
        bne.w   .L08b2fe                        | +09a
        lea     0xffff.w,a0                     | +09e
        move.l  a0,0x48(a6)                     | +0a2
.L08b2fe:
        jsr     0x2870a.l                       | +0a6
        bcc.w   .L08b31e                        | +0ac
        lea     0x5e766.l,a0                    | +0b0
        jsr     0x5e770.l                       | +0b6
        bclr    #0x3,0x13(a6)                   | +0bc
        jsr     Handler_ConditionalHitCounter_08B558(pc) | +0c2
.L08b31e:
        cmpi.b  #0x0,0x9b(a6)                   | +0c6
        beq.w   .L08b334                        | +0cc
        jsr     0x283ca.l                       | +0d0
        jsr     0x283d8.l                       | +0d6
.L08b334:
        lea     0x2ef7bc.l,a0                   | +0dc
        jsr     0x5dd5c.l                       | +0e2
        bcc.w   .L08b34a                        | +0e8
        jmp     0x518.l                         | +0ec
.L08b34a:
        rts                                     | +0f2

| ----------------------------------------------------------------------------
|  Proj_Drop_V0_08b34c  @ $08B34C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Drop_V0_08b34c, "ax", @progbits
        .global Proj_Drop_V0_08b34c
Proj_Drop_V0_08b34c:
        lea     0x2ee194.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ef7c6.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        bra.w   Proj_Drop_Common_08b3b4    | +016

| ----------------------------------------------------------------------------
|  Proj_Drop_V1_08b366  @ $08B366  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Drop_V1_08b366, "ax", @progbits
        .global Proj_Drop_V1_08b366
Proj_Drop_V1_08b366:
        lea     0x2ee1a0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ef7d0.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        bra.w   Proj_Drop_Common_08b3b4    | +016

| ----------------------------------------------------------------------------
|  Proj_Drop_V2_08b380  @ $08B380  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Drop_V2_08b380, "ax", @progbits
        .global Proj_Drop_V2_08b380
Proj_Drop_V2_08b380:
        lea     0x2ee1ac.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ef7da.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        bra.w   Proj_Drop_Common_08b3b4    | +016

| ----------------------------------------------------------------------------
|  Proj_Drop_V3_08b39a  @ $08B39A  (194 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Drop_V3_08b39a, "ax", @progbits
        .global Proj_Drop_V3_08b39a
Proj_Drop_V3_08b39a:
        lea     0x2ee1b8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     0x2ef7e4.l,a0                   | +00c
        move.l  a0,0x70(a6)                     | +012
        bra.w   Proj_Drop_Common_08b3b4    | +016
        .global Proj_Drop_Common_08b3b4
Proj_Drop_Common_08b3b4:
        move.b  0x9a(a6),0x32(a6)               | +01a
        move.b  0x9a(a6),0x33(a6)               | +020
        jsr     0x267e2.l                       | +026
        jsr     0x8f002.l                       | +02c
        jsr     0x8f010.l                       | +032
        move.l  0x106f50.l,d0                   | +038
        swap    d0                              | +03e
        cmpi.b  #0x0,0x9b(a6)                   | +040
        bne.w   .L08b40e                        | +046
        move.w  #0x0,0x38(a6)                   | +04a
        cmpi.w  #0x670,d0                       | +050
        ble.w   .L08b400                        | +054
        move.w  #0x16d,d1                       | +058
        jsr     0x236e.l                        | +05c
        bra.w   .L08b40a                        | +062
.L08b400:
        move.w  #0x16f,d1                       | +066
        jsr     0x236e.l                        | +06a
.L08b40a:
        bra.w   .L08b434                        | +070
.L08b40e:
        move.w  #0x1fff,0x38(a6)                | +074
        cmpi.w  #0x670,d0                       | +07a
        ble.w   .L08b42a                        | +07e
        move.w  #0xce,d1                        | +082
        jsr     0x236e.l                        | +086
        bra.w   .L08b434                        | +08c
.L08b42a:
        move.w  #0x16e,d1                       | +090
        jsr     0x236e.l                        | +094
.L08b434:
        lea     .L08b43a(pc),a1                 | +09a
        move.l  a1,(a6)                         | +09e
.L08b43a:
        jsr     0x27cee.l                       | +0a0
        jsr     0x28d70.l                       | +0a6
        movea.l 0x70(a6),a0                     | +0ac
        jsr     0x5dd5c.l                       | +0b0
        bcc.w   .L08b45a                        | +0b6
        jmp     0x518.l                         | +0ba
.L08b45a:
        rts                                     | +0c0

| ----------------------------------------------------------------------------
|  SprCb_Lamp2On_08b45c  @ $08B45C  (52 B)
| ----------------------------------------------------------------------------
        .section .text.SprCb_Lamp2On_08b45c, "ax", @progbits
        .global SprCb_Lamp2On_08b45c
SprCb_Lamp2On_08b45c:
        move.w  #0x11,d1                        | +000
        move.w  #0x1ac,d2                       | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x12,d1                        | +016
        move.w  #0x1ad,d2                       | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x16(a6),0x14(a6)               | +02c
        rts                                     | +032

| ----------------------------------------------------------------------------
|  SprCb_Lamp2Off_08b490  @ $08B490  (52 B)
| ----------------------------------------------------------------------------
        .section .text.SprCb_Lamp2Off_08b490, "ax", @progbits
        .global SprCb_Lamp2Off_08b490
SprCb_Lamp2Off_08b490:
        move.w  #0x11,d1                        | +000
        move.w  #0x81,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x12,d1                        | +016
        move.w  #0x82,d2                        | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  0x18(a6),0x14(a6)               | +02c
        rts                                     | +032

| ----------------------------------------------------------------------------
|  SprCb_Lamp3On_08b4c4  @ $08B4C4  (74 B)
| ----------------------------------------------------------------------------
        .section .text.SprCb_Lamp3On_08b4c4, "ax", @progbits
        .global SprCb_Lamp3On_08b4c4
SprCb_Lamp3On_08b4c4:
        move.w  #0x10,d1                        | +000
        move.w  #0x1ab,d2                       | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x11,d1                        | +016
        move.w  #0x1ac,d2                       | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0x12,d1                        | +02c
        move.w  #0x1ad,d2                       | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038
        jsr     0x2c26.l                        | +03c
        move.w  0x16(a6),0x14(a6)               | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  SprCb_Lamp3Off_08b50e  @ $08B50E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.SprCb_Lamp3Off_08b50e, "ax", @progbits
        .global SprCb_Lamp3Off_08b50e
SprCb_Lamp3Off_08b50e:
        move.w  #0x10,d1                        | +000
        move.w  #0x80,d2                        | +004
        move.w  #0xffff,d3                      | +008
        move.w  #0x1,d4                         | +00c
        jsr     0x2c26.l                        | +010
        move.w  #0x11,d1                        | +016
        move.w  #0x81,d2                        | +01a
        move.w  #0xffff,d3                      | +01e
        move.w  #0x1,d4                         | +022
        jsr     0x2c26.l                        | +026
        move.w  #0x12,d1                        | +02c
        move.w  #0x82,d2                        | +030
        move.w  #0xffff,d3                      | +034
        move.w  #0x1,d4                         | +038
        jsr     0x2c26.l                        | +03c
        move.w  0x18(a6),0x14(a6)               | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  Math_AbsW_08b58e  @ $08B58E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Math_AbsW_08b58e, "ax", @progbits
        .global Math_AbsW_08b58e
Math_AbsW_08b58e:
        move.w  d0,d1                           | +000
        asr.w   #0x8,d1                         | +002
        btst    #0x7,d1                         | +004
        beq.w   .L08b59c                        | +008
        neg.w   d0                              | +00c
.L08b59c:
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Entity_FollowParentPlus40_08b59e  @ $08B59E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_FollowParentPlus40_08b59e, "ax", @progbits
        .global Entity_FollowParentPlus40_08b59e
Entity_FollowParentPlus40_08b59e:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        addi.w  #0x40,0x24(a6)                  | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Entity_FollowParent_08b5b6  @ $08B5B6  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_FollowParent_08b5b6, "ax", @progbits
        .global Entity_FollowParent_08b5b6
Entity_FollowParent_08b5b6:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Turret8_RotateStep_08b5c8  @ $08B5C8  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_RotateStep_08b5c8, "ax", @progbits
        .global Turret8_RotateStep_08b5c8
Turret8_RotateStep_08b5c8:
        subq.w  #0x1,0x92(a6)                   | +000
        bne.w   ClearXN_08b620                  | +004
        move.w  0x90(a6),d0                     | +008
        add.w   d0,0x34(a6)                     | +00c
        move.w  0x90(a6),d0                     | +010
        asr.w   #0x8,d0                         | +014
        btst    #0x7,d0                         | +016
        bne.w   .L08b5fa                        | +01a
        cmpi.w  #0x3,0x34(a6)                   | +01e
        ble.w   .L08b60e                        | +024
        move.w  #0xffff,0x90(a6)                | +028
        bra.w   .L08b60e                        | +02e
.L08b5fa:
        cmpi.w  #0x2,0x34(a6)                   | +032
        bge.w   .L08b60e                        | +038
        move.w  #0x1,0x90(a6)                   | +03c
        bra.w   .L08b60e                        | +042
.L08b60e:
        andi.w  #0x7,0x34(a6)                   | +046
        move.w  #0xf,0x92(a6)                   | +04c

| ----------------------------------------------------------------------------
|  Turret8_FireBullet_08b626  @ $08B626  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Turret8_FireBullet_08b626, "ax", @progbits
        .global Turret8_FireBullet_08b626
Turret8_FireBullet_08b626:
        move.b  0x106f28.l,d0                   | +000
        andi.b  #0x7,d0                         | +006
        bne.w   .L08b6a6                        | +00a
        move.b  0x106f28.l,d0                   | +00e
        btst    #0x3,d0                         | +014
        bne.w   .L08b65e                        | +018
        lea     Turret8_Casing_089ad0(pc),a1    | +01c
        jsr     0x6fe.l                         | +020
        jsr     0x5dd02.l                       | +026
        addi.w  #0x8,0x22(a0)                   | +02c
        addi.w  #0x20,0x24(a0)                  | +032
.L08b65e:
        move.w  #0x108c,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x3093a.l,a1                    | +042
        jsr     0x4ae.l                         | +048
        jsr     0x5dd22.l                       | +04e
        move.w  0x34(a6),d0                     | +054
        asl.w   #0x4,d0                         | +058
        andi.w  #0xff,d0                        | +05a
        move.b  d0,0x98(a0)                     | +05e
        clr.l   d0                              | +062
        move.w  0x34(a6),d0                     | +064
        asl.l   #0x2,d0                         | +068
        lea     0x2ef7ee.l,a1                   | +06a
        move.w  (a1,d0.w),d1                    | +070
        add.w   d1,0x22(a0)                     | +074
        move.w  0x2(a1,d0.w),d1                 | +078
        add.w   d1,0x24(a0)                     | +07c
.L08b6a6:
        rts                                     | +080

| ----------------------------------------------------------------------------
|  Airship_Steer_08b6a8  @ $08B6A8  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Steer_08b6a8, "ax", @progbits
        .global Airship_Steer_08b6a8
Airship_Steer_08b6a8:
        subi.w  #0x2,0x34(a6)                   | +000
        andi.w  #0xff,0x34(a6)                  | +006
        move.w  0x34(a6),d0                     | +00c
        move.w  #0x3e,d1                        | +010
        jsr     0x13c0e.l                       | +014
        asl.w   #0x1,d1                         | +01a
        jsr     Airship_SteerX_08b778(pc)       | +01c
        move.w  d1,0x28(a6)                     | +020
        jsr     Airship_SteerY_08b796(pc)       | +024
        move.w  d2,0x2a(a6)                     | +028
        jsr     Airship_CheckLanding_08b7f0(pc) | +02c
        bcc.w   .L08b6f6                        | +030
        bclr    #0x6,0x13(a6)                   | +034
        jsr     0x2783a.l                       | +03a
        cmpi.w  #0x0,0x28(a6)                   | +040
        ble.w   .L08b6f6                        | +046
        clr.w   0x28(a6)                        | +04a
.L08b6f6:
        jsr     0x27cee.l                       | +04e
        jsr     Airship_Recoil_08b862(pc)       | +054
        jsr     Airship_ProbeGround_08b7b0(pc)  | +058
        bcc.w   JsrPcRts_08b716                 | +05c
        move.w  #0x1081,d0                      | +060
        jsr     0x2352.l                        | +064

| ----------------------------------------------------------------------------
|  Airship_SpiralDescent_08b718  @ $08B718  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_SpiralDescent_08b718, "ax", @progbits
        .global Airship_SpiralDescent_08b718
Airship_SpiralDescent_08b718:
        move.w  0x36(a6),d1                     | +000
        btst    #0xf,d1                         | +004
        beq.w   .L08b72c                        | +008
        clr.w   d1                              | +00c
        clr.w   d2                              | +00e
        bra.w   .L08b768                        | +010
.L08b72c:
        move.w  0x36(a6),d1                     | +014
        asr.w   #0x5,d1                         | +018
        move.b  0x106f28.l,d0                   | +01a
        btst    #0x0,d0                         | +020
        beq.w   .L08b742                        | +024
        neg.w   d1                              | +028
.L08b742:
        asl.w   #0x8,d1                         | +02a
        move.w  d1,0x2a(a6)                     | +02c
        move.w  #0x0,0x28(a6)                   | +030
        jsr     0x27cee.l                       | +036
        subi.w  #0x3,0x36(a6)                   | +03c
        move.w  0x36(a6),d1                     | +042
        move.w  #0x1f,d0                        | +046
        jsr     0x13c0e.l                       | +04a
.L08b768:
        move.w  d1,0x28(a6)                     | +050
        move.w  d2,0x2a(a6)                     | +054

| ----------------------------------------------------------------------------
|  Airship_SteerX_08b778  @ $08B778  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_SteerX_08b778, "ax", @progbits
        .global Airship_SteerX_08b778
Airship_SteerX_08b778:
        move.w  0x7e(a6),d0                     | +000
        sub.w   0x22(a6),d0                     | +004
        beq.w   .L08b794                        | +008
        bpl.w   .L08b790                        | +00c
        subi.w  #0x12,d1                        | +010
        bra.w   .L08b794                        | +014
.L08b790:
        addi.w  #0x12,d1                        | +018
.L08b794:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Airship_SteerY_08b796  @ $08B796  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_SteerY_08b796, "ax", @progbits
        .global Airship_SteerY_08b796
Airship_SteerY_08b796:
        move.w  0x80(a6),d0                     | +000
        sub.w   0x24(a6),d0                     | +004
        beq.w   .L08b7ae                        | +008
        bpl.w   .L08b7ac                        | +00c
        subq.w  #0x4,d2                         | +010
        bra.w   .L08b7ae                        | +012
.L08b7ac:
        addq.w  #0x4,d2                         | +016
.L08b7ae:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Airship_ProbeGround_08b7b0  @ $08B7B0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_ProbeGround_08b7b0, "ax", @progbits
        .global Airship_ProbeGround_08b7b0
Airship_ProbeGround_08b7b0:
        move.w  0x22(a6),d1                     | +000
        move.w  0x24(a6),d2                     | +004
        addi.w  #0xa8,d1                        | +008
        addi.w  #0x10,d2                        | +00c
        jsr     0x280c6.l                       | +010
        bcc.w   Airship_ProbeGround_Tail_08b7e6 | +016
        cmpi.b  #0x25,d7                        | +01a
        bne.w   Airship_ProbeGround_Tail_08b7e6 | +01e
        cmpi.b  #0x0,0x83(a6)                   | +022
        bne.w   Airship_ProbeGround_Tail_08b7e6 | +028
        move.b  d0,0x83(a6)                     | +02c

| ----------------------------------------------------------------------------
|  Airship_ProbeGround_Tail_08b7e6  @ $08B7E6  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_ProbeGround_Tail_08b7e6, "ax", @progbits
        .global Airship_ProbeGround_Tail_08b7e6
Airship_ProbeGround_Tail_08b7e6:
        move.b  d0,0x83(a6)                     | +000

| ----------------------------------------------------------------------------
|  Airship_CheckLanding_08b7f0  @ $08B7F0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_CheckLanding_08b7f0, "ax", @progbits
        .global Airship_CheckLanding_08b7f0
Airship_CheckLanding_08b7f0:
        move.w  0x22(a6),d1                     | +000
        move.w  0x24(a6),d2                     | +004
        addi.w  #0xa8,d1                        | +008
        addi.w  #0x10,d2                        | +00c
        jsr     0x280c6.l                       | +010
        bcc.w   ClearXN_08b826                  | +016
        cmpi.b  #0x25,d7                        | +01a
        beq.w   SetXN_08b820                    | +01e
        cmpi.b  #0x1e,d7                        | +022
        bne.w   ClearXN_08b826                  | +026
        lea     Airship_Depart_0895fe(pc),a1    | +02a
        move.l  a1,(a6)                         | +02e

| ----------------------------------------------------------------------------
|  Airship_DropSoldier_08b82c  @ $08B82C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_DropSoldier_08b82c, "ax", @progbits
        .global Airship_DropSoldier_08b82c
Airship_DropSoldier_08b82c:
        move.w  #0xa,0x84(a6)                   | +000
        move.l  0x106f50.l,d0                   | +006
        swap    d0                              | +00c
        cmpi.w  #0x9a0,d0                       | +00e
        bge.w   .L08b860                        | +012
        lea     0x631d0.l,a1                    | +016
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd22.l                       | +022
        addi.w  #0xb8,0x22(a0)                  | +028
        addi.w  #0x50,0x24(a0)                  | +02e
.L08b860:
        rts                                     | +034

| ----------------------------------------------------------------------------
|  Airship_Recoil_08b862  @ $08B862  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_Recoil_08b862, "ax", @progbits
        .global Airship_Recoil_08b862
Airship_Recoil_08b862:
        cmpi.w  #0x0,0x84(a6)                   | +000
        beq.w   .L08b8ae                        | +006
        subq.w  #0x1,0x84(a6)                   | +00a
        move.w  #0x200,d1                       | +00e
        move.b  0x106f28.l,d0                   | +012
        btst    #0x0,d0                         | +018
        bne.w   .L08b884                        | +01c
        neg.w   d1                              | +020
.L08b884:
        move.w  0x28(a6),d2                     | +022
        move.w  0x2a(a6),d3                     | +026
        movem.w d2,-(a7)                        | +02a
        movem.w d3,-(a7)                        | +02e
        move.w  d1,0x2a(a6)                     | +032
        jsr     0x27cee.l                       | +036
        movem.w (a7)+,d3                        | +03c
        movem.w (a7)+,d2                        | +040
        move.w  d2,0x28(a6)                     | +044
        move.w  d3,0x2a(a6)                     | +048
.L08b8ae:
        rts                                     | +04c

| ----------------------------------------------------------------------------
|  Airship_TrailRecord_08b8b0  @ $08B8B0  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_TrailRecord_08b8b0, "ax", @progbits
        .global Airship_TrailRecord_08b8b0
Airship_TrailRecord_08b8b0:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        subi.w  #0xa0,d0                        | +008
        addi.w  #0x2e,d1                        | +00c
        move.w  #0x140,d2                       | +010
        jmp     0x99812.l                       | +014

| ----------------------------------------------------------------------------
|  Airship_TrailRecordAlt_08b8ca  @ $08B8CA  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Airship_TrailRecordAlt_08b8ca, "ax", @progbits
        .global Airship_TrailRecordAlt_08b8ca
Airship_TrailRecordAlt_08b8ca:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        subi.w  #0xa0,d0                        | +008
        addi.w  #0x2e,d1                        | +00c
        move.w  #0x140,d2                       | +010
        jmp     0x997e2.l                       | +014

| ----------------------------------------------------------------------------
|  Camera_PublishLockX_08b8e4  @ $08B8E4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Camera_PublishLockX_08b8e4, "ax", @progbits
        .global Camera_PublishLockX_08b8e4
Camera_PublishLockX_08b8e4:
        move.w  0x22(a6),d0                     | +000
        addi.w  #0x70,d0                        | +004
        move.w  d0,0x106f42.l                   | +008
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  S5_Tent_BlitRowByIdx_08b8f4  @ $08B8F4  (18 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Tent_BlitRowByIdx_08b8f4, "ax", @progbits
        .global S5_Tent_BlitRowByIdx_08b8f4
S5_Tent_BlitRowByIdx_08b8f4:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asl.l   #0x2,d0                         | +006
        lea     0x2ef61a.l,a0                   | +008
        movea.l (a0,d0.w),a1                    | +00e

| ----------------------------------------------------------------------------
|  S5_Bunker_BlitByIdx_08b90e  @ $08B90E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.S5_Bunker_BlitByIdx_08b90e, "ax", @progbits
        .global S5_Bunker_BlitByIdx_08b90e
S5_Bunker_BlitByIdx_08b90e:
        clr.l   d0                              | +000
        move.b  0x21(a6),d0                     | +002
        asl.l   #0x2,d0                         | +006
        lea     0x2ef39c.l,a2                   | +008
        movea.l (a2,d0.w),a2                    | +00e

| ----------------------------------------------------------------------------
|  Entity_CmpPrioWithSibling_08b928  @ $08B928  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpPrioWithSibling_08b928, "ax", @progbits
        .global Entity_CmpPrioWithSibling_08b928
Entity_CmpPrioWithSibling_08b928:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08b93e                    | +00c

| ----------------------------------------------------------------------------
|  Proj_ScriptTable_08b944  @ $08B944  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_ScriptTable_08b944, "ax", @progbits
        .global Proj_ScriptTable_08b944
Proj_ScriptTable_08b944:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +004  (dato / opcode no decodificado)
        .dc.w   0xbada                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +008  (dato / opcode no decodificado)
        .dc.w   0xbabe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +050  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +052  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Proj_Tmpl168_08b9a2  @ $08B9A2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Tmpl168_08b9a2, "ax", @progbits
        .global Proj_Tmpl168_08b9a2
Proj_Tmpl168_08b9a2:
        jsr     Sub_0008BB5E(pc)                | +000  -> $08BB5E (hueco futuro, defsym forward)
        bra.w   Sub_0008BA0C                    | +004  -> $08BA0C (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  Proj_Tmpl169_08b9aa  @ $08B9AA  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Tmpl169_08b9aa, "ax", @progbits
        .global Proj_Tmpl169_08b9aa
Proj_Tmpl169_08b9aa:
        jsr     Sub_0008BB34(pc)                | +000  -> $08BB34 (hueco futuro, defsym forward)
        bra.w   Proj_Bounce_08b9ba              | +004

| ----------------------------------------------------------------------------
|  Proj_Tmpl170_08b9b2  @ $08B9B2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Tmpl170_08b9b2, "ax", @progbits
        .global Proj_Tmpl170_08b9b2
Proj_Tmpl170_08b9b2:
        jsr     Sub_0008BB34(pc)                | +000  -> $08BB34 (hueco futuro, defsym forward)
        bra.w   Sub_0008BA52                    | +004  -> $08BA52 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  Proj_Bounce_08b9ba  @ $08B9BA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Proj_Bounce_08b9ba, "ax", @progbits
        .global Proj_Bounce_08b9ba
Proj_Bounce_08b9ba:
        move.w  #0x0,0x28(a6)                   | +000
        lea     0x29b744.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     0x29ca20.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L08b9de(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L08b9de:
        jsr     0x2783a.l                       | +024
        jsr     0x28d70.l                       | +02a
        bcc.w   .L08b9f4                        | +030
        lea     Proj_Bounce_08b9ba(pc),a1       | +034
        move.l  a1,(a6)                         | +038
.L08b9f4:
        jsr     0x2870a.l                       | +03a
        lea     Proj_ScriptTable_08b944(pc),a0  | +040
        .dc.w   0x227c                        | +044  (dato / opcode no decodificado)
