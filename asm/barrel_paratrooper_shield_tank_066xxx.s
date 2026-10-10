| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave UUUU — barril flotante / mina acuática (cola de Barrel_Tmpl8D),
|  spawner de paracaidistas, soldado con escudo (dos variantes), escudo
|  desprendido, puerta de escena 5 y dirigible (partes, luces, escotilla,
|  gancho de cámara) y tanque enemigo (conductor, torreta, misil guiado)
|  (plantillas $E8000[53..55,94,95,108,109])
|  Región: $066000..$06A000  (15,488 B, 180 entradas, 79 huecos)
| ============================================================================
|
|  A) QUÉ ES
|  180 entradas en seis bloques (continúa sniper_camper_mortar_062xxx.s):
|  1. $066000..$0666AE — FloatBarrel_* / FloatMine_*: cola del barril de
|     tmpl 141 (Barrel_Tmpl8D_065f40): Submerge (vel +$2A=$A00, gravedad
|     negativa por nibble +$99), Surface/Drift/Bob/BobSettle (sondas $27CEE/
|     $27D50/$27EBA, target $5E086 con cajas $2C79AC/$2C79B6), Splash/Pop
|     (música $1052/$103E, hijo $788C0), Shard (fragmento con vel heredada
|     del abuelo +$50). FloatMine_Init/Armed: mina flotante (snd $179/$1DA,
|     HP $64, ataque $283CA, explosión $77F6A, WaveBob con Math_AbsW $8B58E).
|  2. $0666AE..$0670B6 — Paratrooper_*: Spawner_Tmpl5E (tmpl 94/95: hasta
|     +$98 paracaidistas, cadencia Tbl_Decode2D $2B889A, umbral de scroll
|     +$7E vs camera_x $106F50), Init (snd $183, prio $4000, X aleatoria
|     hacia el jugador $5E1EA) → Land → PickWalk → WalkA/WalkB (timers
|     $2B8A60/$2B89DE) → Leave; Die (snd $183/$1D7 según $10FD8F, música
|     $1043, flush $13600); Frag (vel por sin/cos $2C07AC/$2C072C, snd $17B)
|     → FragExplode ($77F6A); Smoke; SpawnPairA/B, SpawnChute ($788E4).
|     Paratrooper_Sprites_066cd8: 1,006 B de scripts de animación (frames
|     {dur,flags,ptr $2Cxxxx}, terminadores $0100 = loop).
|  3. $0670B6..$0682D0 — ShieldSoldier_* (tmpl 108, $670D2) y
|     ShieldSoldierB_* (tmpl 109, $676BA, también spawneado por
|     M4_PlatformSpawn_085134): Setup (snd $38/$1C7, HP 1, hijo Shield_Init
|     en +$90, anim $2C79E2/$2C7C82/$2C7D74) → Guard/Fall/Land/Crouch/
|     CrouchHold/Rise/Aim/Attack ($283CA/$283D8)/Recover → ShieldLost
|     (sprite $29BFC4, ToFlee = jmp $58F82) / Die (HumanDeath $4ABC0).
|     Predicados CanAttackA/B/C, PlayerFar/Near(+Edge), AtScreenEdge,
|     GuardTick, FallCheck, ShieldAlive, TakeShieldHit (+$5A/+$58 desde el
|     escudo), AbsDX, DropItem/B ($9A7AA), HitCheck.
|     Shield_*: Init (HP $2BF59A, sprites por +$80&7 en $2C7CFE, sigue al
|     padre $5E506), Break (música $10FF, Smoke+Debris), FlyOff/Bounce (RNG
|     $5DCA4, sondas $27C8C) → Blink (parpadeo con $106F28 bit0) → Free.
|  4. $0682D0..$0683F0 — S5Gate_*: tarea de puerta/compuerta de escena 5:
|     espera camera_x ≥ $E20/$1000 (nivel $10E39C=4), Begin (música $1032,
|     $106F5E=1, rampa $106F60 hasta $60000) → HoldOffset → Release
|     (Camera0_RelinkAndWrapScroll_06896A).
|  5. $0683F0..$068C1E — S5Airship_*: dirigible (Init: pos por
|     Coord_ScreenToLocalSecondary $440D0, hijos Part×3 con offset +$80,
|     Nose, prio $C000), Part/PartIdle/PartHold/PartSway/PartState1/3 (estado
|     +$20 del padre, jitter RNG $5E9B6, sello $106ED3), Light/LightBlink,
|     Hull, Nose (registro $43FAC con caja $2C84C2), Hatch/HatchClosed/
|     HatchOpen (música $10A8)/HatchAnim/HatchClose/HatchFinal (sprites por
|     +$3B&$F en $2C8626/$2C8686), CameraHook ($51B3E/$43DF4/$51B1C sobre
|     $106F6C), FollowParent/Grandparent, ShadowA/B/C, DropPow (spawnea
|     Pow_EntryB_03fec6 con probabilidad $2BA618), HitCheck.
|  6. $068C1E..$06A000 — Tank_*: tanque enemigo tmpl 53/54/55 (variantes
|     por +$7D/+$7E), Tmpl35: snd motor $1B/$1C, HP $64, hijos Driver
|     ($69880, en +$88) y Turret ($69A32, en +$8C), oruga $723D2 y sombra
|     $7773E, caja $2C89CC. Fall/Drive (música $109E, Player_Dispatch3Slots
|     $28998)/Idle/Turn/Brake/Resume/AimPlayer/FireBurst (RngReload
|     $2B807A, FireDone) → Die (explosión $77FD6, music $109E via $2222) →
|     Wreck/WreckBlast. Driver: Bail/Hit/Die/ToSoldier ($4A0D4 + jmp
|     $58F82/$5724E)/Wreck. Turret: HP $2B7DF0, estado +$20 $33/$77,
|     música $102D. Debris (jmp $6DBD4), Smoke/SmokeExplode ($77EFE),
|     Missile/MissileFly/MissileHome (ángulo +$82 hacia el jugador con
|     $5E070, vel por sin/cos) /MissileExplode. Helpers: SetSpriteByIndex
|     (con asserts trap #15 índice ≤ $E/$A), SetHandlerIfValid,
|     MirrorByFacing/Dir, PastRangeX, CooldownTick, AnimFrame, VelFromAngle,
|     RecoilTick, CopyParentAnim(+Prio), OffsetAbove, ItemFlag,
|     SpawnExplosion, DeathBlast (16× Sniper_SmokeA $62536), DropItem,
|     SpawnSmokeMissile/Debris, SetPrio4000.
|
|  B) CÓMO FUNCIONA
|  Protocolo a6 habitual: +$00 handler, +$0C padre, +$12/+$13 flags, +$20/
|  +$21 estado y orden del padre, +$22/+$24 pos, +$28/+$2A vel, +$2E grav,
|  +$38 prio, +$3A facing, +$3C plantilla, +$58 clase de ataque, +$66 HP,
|  +$70.. contadores; +$98..+$9C parámetros del Mission VM (cuenta, lado,
|  ítem). El soldado con escudo delega la colisión frontal en el hijo
|  Shield (bit0 de +$5A del hijo → TakeShieldHit copia +$58 y marca +$13
|  bit3); al perder el escudo pasa a soldado genérico. El tanque mantiene
|  punteros a sus hijos en +$88/+$8C y cambia sus sprites con
|  SetSpriteByIndex (tablas $2C8B96/$2C8BC2 con -1 = sin cambio). La
|  puerta y el dirigible de escena 5 usan las variables globales de
|  cámara $106F50/$106F5E/$106F60/$106F64 y el sello $106ED3 (misma
|  familia que cutscene_anim_08baxx.s). Los paracaidistas se auto-limitan
|  a la franja X $20..$120 (TimerInBounds).
|
|  C) INTERFAZ
|  Entradas externas: plantillas $E8000[53]=$68C1E, [54]=$68C2A, [55]=$68C38,
|  [94]=$666AE, [95]=$666B6, [108]=$670D2, [109]=$676BA; $85134
|  (M4_PlatformSpawn) -> $676BA; Barrel_Tmpl8D_065f40 (W TTTT) → $6600E/
|  $66622/$66644; mission_streams usa tmpl $037 (4×).
|  Salidas: $236E snd, $2352/$2222 música, $28CD4/$28D70 sprite/anim,
|  $2870A/$28758 daño/HP, $283CA/$283D8 ataque, $267E2/$2783A scroll,
|  $28134 prio, $27CEE/$27D50/$27AFC/$27C8C/$27EBA/$27A92/$27FAC sondas,
|  $43FAC registro, $4A0D4/$4ABC0 muerte humana, $58F82/$5724E soldier,
|  $5DCA4 RNG escalado, $5DD02 pos padre, $5DD56/$5DD5C offworld, $5E070
|  Atan2 delta, $5E086/$5E0D4/$5E1EA target, $5E45A padre liberado,
|  $5E506 copia pos/prio/facing, $5E5A8 probe 2 intentos, $5E766/$5E770
|  flash, $5E7C0 snap suelo, $5E844 ataque pesado, $5E9B6 RNG, $6DBD4
|  (jmp, Wave VVVV), $723D2/$7773E/$788C0/$788E4 hijos, $77EFE/$77F6A/
|  $77FD6 explosiones, $799DE Tbl_Decode2D, $8B58E abs, $9A7AA probe
|  move X, $13600 flush, $138FE timer, $3FEC6 POW, $440D0/$43DF4/$51B1C/
|  $51B3E cámara, $518 free, $4AE alloc. Datos: $2B7Dxx..$2BA6xx (tablas
|  por dificultad), $2C07AC/$2C072C sin/cos, sprites $2C76xx..$2C9Cxx.
|
|  D) EVIDENCIAS
|  - $E8000 apunta a 7 entradas de esta región (índices en C); Barrel_Tmpl8D
|    (TTTT) referenciaba $6600E/$66622/$66644 como defsym forward.
|  - m4_carrier_boss_helpers_0851xx.s: M4_PlatformSpawn_085134 hace
|    `lea $676BA,a1 ; jsr $4AE` → tmpl 109 es el soldado con escudo de la
|    plataforma del jefe de misión 4.
|  - Shield_Init: cuando su HP se agota escribe `move.l #$67656,(a0)` en el
|    padre (ShieldSoldier_ShieldLost) → relación padre/hijo confirmada.
|  - Tank: asserts `cmpi.w #$E,d0 / trap #15` antes de indexar $2C8BC2
|    (15 sprites de torreta) y `#$A` para $2C8B96 (11 de casco).
|  - S5Gate/S5Airship usan $106F5E/$106F60 igual que cutscene_anim_08baxx.s
|    ($1C000/$18000) y el sello $106ED3 de attract_cluster_batch_ff.s.
|  - Paratrooper_Die elige snd $183/$1D7 según $10FD8F (misma bandera que
|    otros gritos de muerte); Tank_DriverToSoldier salta a $58F82/$5724E.
|  - Tabla $66CD8..$670B6 identificada como datos por el patrón
|    {$0001,$0208,$0023,$4xxx,$FFFF} y el bucle $0100 $0006 ← code real
|    retoma en $670B6 (`movea.l 8(a6),a1`).
|
|  E) HIPÓTESIS / DUDAS
|  - "FloatBarrel"/"FloatMine": por gravedad negativa, flotación (WaveBob)
|    y mina con HP; el barril de TTTT podría ser otro objeto flotante.
|  - "Paratrooper": spawner con cadencia + hijo con paracaídas ($788E4,
|    nombre por comportamiento de caída lenta y SpawnChute); no verificado
|    visualmente.
|  - "ShieldSoldier": inferido por el hijo con HP propio que absorbe golpes
|    frontales y por la transición a soldado al perderlo.
|  - "S5Gate"/"S5Airship": por el uso de las globales de cámara de escena
|    5 y el gancho $106F6C; la identidad de "Part/Light/Hull/Nose/Hatch" es
|    aproximada (nombres por offsets y sprites).
|  - "Tank": vehículo con conductor que salta (Bail) y torreta con misil
|    guiado; podría ser otro vehículo blindado. $6DBD4 (Tank_Debris) y
|    $6A7D6 quedan para Wave VVVV.
|  - $66C06 (ApproachFail) y $67F7A (ProbeRevert) son colas de función
|    alcanzadas por fall-through; nombres por su efecto.
|
|  F) ESTADO
|  180/180 byte-exactas; 1 rango --data (docs/waves/uuuu_args.txt).
|  Nombres en docs/waves/uuuu_names.txt. Pendiente: $06A000..$083000.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  FloatBarrel_OffworldFree_066000  @ $066000  (14 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_OffworldFree_066000, "ax", @progbits
        .global FloatBarrel_OffworldFree_066000
FloatBarrel_OffworldFree_066000:
        add.w   d6,(a4)+                        | +000
        bcc.w   .L06600c                        | +002
        jmp     0x518.l                         | +006
.L06600c:
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  FloatBarrel_Submerge_06600e  @ $06600E  (190 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Submerge_06600e, "ax", @progbits
        .global FloatBarrel_Submerge_06600e
FloatBarrel_Submerge_06600e:
        move.w  #0x0,0x28(a6)                   | +000
        move.w  #0xa00,0x2a(a6)                 | +006
        move.w  #0x0,0x2c(a6)                   | +00c
        move.w  #0xff80,0x2e(a6)                | +012
        move.b  0x99(a6),d0                     | +018
        andi.b  #0x7,d0                         | +01c
.L06602e:
        cmpi.b  #0x0,d0                         | +020
        beq.w   .L066040                        | +024
        subq.b  #0x1,d0                         | +028
        addi.w  #0x8,0x2e(a6)                   | +02a
        bra.b   .L06602e                        | +030
.L066040:
        lea     0x2c7626.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     0x788c0.l,a1                    | +03e
        jsr     0x4ae.l                         | +044
        jsr     0x5dd02.l                       | +04a
        addq.w  #0x8,0x24(a0)                   | +050
        bset    #0x6,0x12(a0)                   | +054
        lea     .L06606e(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L06606e:
        jsr     0x27cee.l                       | +060
        jsr     0x28d70.l                       | +066
        clr.l   d0                              | +06c
        move.w  0x2a(a6),d0                     | +06e
        btst    #0xf,d0                         | +072
        beq.w   .L06608e                        | +076
        lea     FloatBarrel_Surface_0660cc(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L06608e:
        jsr     0x2870a.l                       | +080
        bcc.w   .L0660ae                        | +086
        lea     FloatBarrel_Shard_066528(pc),a1 | +08a
        jsr     0x4ae.l                         | +08e
        jsr     0x5dd02.l                       | +094
        lea     FloatBarrel_Pop_066228(pc),a1   | +09a
        move.l  a1,(a6)                         | +09e
.L0660ae:
        movea.l #0xffffffff,a0                  | +0a0
        lea     0x2c7998.l,a0                   | +0a6
        jsr     0x5dd5c.l                       | +0ac
        bcc.w   .L0660ca                        | +0b2
        jmp     0x518.l                         | +0b6
.L0660ca:
        rts                                     | +0bc

| ----------------------------------------------------------------------------
|  FloatBarrel_Surface_0660cc  @ $0660CC  (168 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Surface_0660cc, "ax", @progbits
        .global FloatBarrel_Surface_0660cc
FloatBarrel_Surface_0660cc:
        move.w  0x2e(a6),d0                     | +000
        asr.w   #0x1,d0                         | +004
        move.w  d0,0x2e(a6)                     | +006
        jsr     FloatBarrel_FaceTarget_066622(pc) | +00a
        lea     0x2c765c.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L0660ec(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L0660ec:
        jsr     0x27cee.l                       | +020
        jsr     0x28d70.l                       | +026
        lea     0x2c79ac.l,a0                   | +02c
        jsr     0x5e086.l                       | +032
        bcs.w   .L06611e                        | +038
        lea     0x2c79b6.l,a0                   | +03c
        jsr     0x5e086.l                       | +042
        bcc.w   .L06611e                        | +048
        lea     FloatBarrel_Drift_066174(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
.L06611e:
        jsr     0x27fac.l                       | +052
        bcs.w   .L066136                        | +058
        cmpi.b  #0x41,d7                        | +05c
        bne.w   .L066136                        | +060
        lea     FloatBarrel_Splash_0661d8(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L066136:
        jsr     0x2870a.l                       | +06a
        bcc.w   .L066156                        | +070
        lea     FloatBarrel_Shard_066528(pc),a1 | +074
        jsr     0x4ae.l                         | +078
        jsr     0x5dd02.l                       | +07e
        lea     FloatBarrel_Pop_066228(pc),a1   | +084
        move.l  a1,(a6)                         | +088
.L066156:
        movea.l #0xffffffff,a0                  | +08a
        lea     0x2c7998.l,a0                   | +090
        jsr     0x5dd5c.l                       | +096
        bcc.w   .L066172                        | +09c
        jmp     0x518.l                         | +0a0
.L066172:
        rts                                     | +0a6

| ----------------------------------------------------------------------------
|  FloatBarrel_Drift_066174  @ $066174  (100 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Drift_066174, "ax", @progbits
        .global FloatBarrel_Drift_066174
FloatBarrel_Drift_066174:
        lea     0x2c7688.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L066186(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L066186:
        jsr     0x27cee.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x27eba.l                       | +01e
        bcs.w   .L0661aa                        | +024
        cmpi.b  #0x41,d7                        | +028
        bne.w   .L0661aa                        | +02c
        lea     FloatBarrel_Splash_0661d8(pc),a1 | +030
        move.l  a1,(a6)                         | +034
.L0661aa:
        jsr     0x2870a.l                       | +036
        bcc.w   .L0661ba                        | +03c
        lea     FloatBarrel_Pop_066228(pc),a1   | +040
        move.l  a1,(a6)                         | +044
.L0661ba:
        movea.l #0xffffffff,a0                  | +046
        lea     0x2c7998.l,a0                   | +04c
        jsr     0x5dd5c.l                       | +052
        bcc.w   .L0661d6                        | +058
        jmp     0x518.l                         | +05c
.L0661d6:
        rts                                     | +062

| ----------------------------------------------------------------------------
|  FloatBarrel_Splash_0661d8  @ $0661D8  (78 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Splash_0661d8, "ax", @progbits
        .global FloatBarrel_Splash_0661d8
FloatBarrel_Splash_0661d8:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2c7998.l,a0                   | +006
        jsr     0x5dd56.l                       | +00c
        bcs.w   .L066214                        | +012
        move.w  #0x1052,d0                      | +016
        jsr     0x2352.l                        | +01a
        lea     0x788c0.l,a1                    | +020
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c
        addq.w  #0x8,0x24(a0)                   | +032
        bset    #0x6,0x12(a0)                   | +036
.L066214:
        lea     .L06621a(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L06621a:
        jsr     0x28d70.l                       | +042
        jmp     0x518.l                         | +048

| ----------------------------------------------------------------------------
|  Rts_066226  @ $066226  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_066226, "ax", @progbits
        .global Rts_066226
Rts_066226:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  FloatBarrel_Pop_066228  @ $066228  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Pop_066228, "ax", @progbits
        .global FloatBarrel_Pop_066228
FloatBarrel_Pop_066228:
        move.w  #0x103e,d0                      | +000
        jsr     0x2352.l                        | +004
        bclr    #0x6,0x12(a6)                   | +00a
        jsr     0x267e2.l                       | +010
        movea.l 0x50(a6),a0                     | +016
        move.w  0x28(a0),d0                     | +01a
        asr.w   #0x2,d0                         | +01e
        move.w  d0,0x28(a6)                     | +020
        lea     0x2c76e8.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a

| ----------------------------------------------------------------------------
|  FloatBarrel_Bob_066258  @ $066258  (76 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Bob_066258, "ax", @progbits
        .global FloatBarrel_Bob_066258
FloatBarrel_Bob_066258:
        move.w  #0x0,0x2a(a6)                   | +000
        move.w  #0xffd0,0x2e(a6)                | +006
        lea     .L06626a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06626a:
        jsr     0x27d50.l                       | +012
        jsr     0x27eba.l                       | +018
        bcs.w   .L066280                        | +01e
        lea     FloatBarrel_BobSettle_0662a4(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L066280:
        jsr     0x28d70.l                       | +028
        movea.l #0xffffffff,a0                  | +02e
        lea     0x2c7998.l,a0                   | +034
        jsr     0x5dd5c.l                       | +03a
        bcc.w   .L0662a2                        | +040
        jmp     0x518.l                         | +044
.L0662a2:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  FloatBarrel_BobSettle_0662a4  @ $0662A4  (214 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_BobSettle_0662a4, "ax", @progbits
        .global FloatBarrel_BobSettle_0662a4
FloatBarrel_BobSettle_0662a4:
        jsr     0x27eba.l                       | +000
        bcs.w   .L0662e2                        | +006
        cmpi.b  #0x41,d7                        | +00a
        bne.w   .L0662e2                        | +00e
        move.w  #0x1052,d0                      | +012
        jsr     0x2352.l                        | +016
        lea     0x788c0.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        addq.w  #0x8,0x24(a0)                   | +02e
        bset    #0x6,0x12(a0)                   | +032
        jmp     0x518.l                         | +038
.L0662e2:
        move.w  0x28(a6),d0                     | +03e
        neg.w   d0                              | +042
        asr.w   #0x5,d0                         | +044
        move.w  d0,0x2c(a6)                     | +046
        move.w  #0x0,0x2a(a6)                   | +04a
        move.w  #0x0,0x2e(a6)                   | +050
        lea     0x2c7744.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        lea     .L06630c(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L06630c:
        move.w  0x28(a6),d0                     | +068
        movem.w d0,-(a7)                        | +06c
        jsr     0x27d50.l                       | +070
        movem.w (a7)+,d0                        | +076
        move.w  0x28(a6),d1                     | +07a
        andi.w  #0x8000,d0                      | +07e
        andi.w  #0x8000,d1                      | +082
        cmp.w   d0,d1                           | +086
        beq.w   .L06633c                        | +088
        move.w  #0x0,0x28(a6)                   | +08c
        move.w  #0x0,0x2c(a6)                   | +092
.L06633c:
        jsr     0x27eba.l                       | +098
        bcc.w   .L06634c                        | +09e
        lea     FloatBarrel_Bob_066258(pc),a1   | +0a2
        move.l  a1,(a6)                         | +0a6
.L06634c:
        jsr     0x28d70.l                       | +0a8
        bcc.w   .L06635c                        | +0ae
        jmp     0x518.l                         | +0b2
.L06635c:
        movea.l #0xffffffff,a0                  | +0b8
        lea     0x2c7998.l,a0                   | +0be
        jsr     0x5dd5c.l                       | +0c4
        bcc.w   .L066378                        | +0ca
        jmp     0x518.l                         | +0ce
.L066378:
        rts                                     | +0d4

| ----------------------------------------------------------------------------
|  FloatMine_Init_06637a  @ $06637A  (278 B)
| ----------------------------------------------------------------------------
        .section .text.FloatMine_Init_06637a, "ax", @progbits
        .global FloatMine_Init_06637a
FloatMine_Init_06637a:
        move.w  #0x179,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x1da,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x8000,0x38(a6)                | +014
        jsr     0x267e2.l                       | +01a
        lea     0x2bdb84.l,a0                   | +020
        jsr     0x799de.l                       | +026
        move.w  d0,d0                           | +02c
        move.w  #0x18,d1                        | +02e
        btst    #0x0,0x3a(a6)                   | +032
        bne.w   .L0663ba                        | +038
        neg.w   d0                              | +03c
        neg.w   d1                              | +03e
.L0663ba:
        move.w  d0,0x28(a6)                     | +040
        add.w   d1,0x22(a6)                     | +044
        addi.w  #0x24,0x24(a6)                  | +048
        lea     0x2c7798.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     0x2c784c.l,a0                   | +05a
        move.l  a0,0x4c(a6)                     | +060
        jsr     0x283ca.l                       | +064
        lea     0x2c7944.l,a0                   | +06a
        move.l  a0,0x48(a6)                     | +070
        move.w  #0x64,0x66(a6)                  | +074
        move.w  #0xf400,0x76(a6)                | +07a
        lea     .L066400(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L066400:
        jsr     FloatBarrel_WaveBob_0665e8(pc)  | +086
        bcc.w   .L06640e                        | +08a
        lea     FloatMine_Armed_066490(pc),a1   | +08e
        move.l  a1,(a6)                         | +092
.L06640e:
        jsr     FloatBarrel_FlipByFrame_0665c6(pc) | +094
        jsr     0x28d70.l                       | +098
        jsr     0x2870a.l                       | +09e
        bcc.w   .L066432                        | +0a4
        bclr    #0x3,0x13(a6)                   | +0a8
        move.w  #0x108d,d0                      | +0ae
        jsr     0x2352.l                        | +0b2
.L066432:
        jsr     0x28758.l                       | +0b8
        bcs.w   .L06646c                        | +0be
        jsr     0x27fac.l                       | +0c2
        bcc.w   .L06646c                        | +0c8
        jsr     0x283d8.l                       | +0cc
        btst    #0x1,0x13(a6)                   | +0d2
        bne.w   .L06646c                        | +0d8
        movea.l #0xffffffff,a0                  | +0dc
        lea     0x2c79a2.l,a0                   | +0e2
        jsr     0x5dd56.l                       | +0e8
        bcc.w   .L06648e                        | +0ee
.L06646c:
        move.w  #0x1022,d0                      | +0f2
        jsr     0x2352.l                        | +0f6
        lea     0x77f6a.l,a1                    | +0fc
        jsr     0x4ae.l                         | +102
        jsr     0x5dd02.l                       | +108
        jmp     0x518.l                         | +10e
.L06648e:
        rts                                     | +114

| ----------------------------------------------------------------------------
|  FloatMine_Armed_066490  @ $066490  (152 B)
| ----------------------------------------------------------------------------
        .section .text.FloatMine_Armed_066490, "ax", @progbits
        .global FloatMine_Armed_066490
FloatMine_Armed_066490:
        lea     0x2c77a8.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0664a2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0664a2:
        jsr     FloatBarrel_WaveBob_0665e8(pc)  | +012
        jsr     FloatBarrel_FlipByFrame_0665c6(pc) | +016
        jsr     0x28d70.l                       | +01a
        jsr     0x2870a.l                       | +020
        bcc.w   .L0664ca                        | +026
        bclr    #0x3,0x13(a6)                   | +02a
        move.w  #0x108d,d0                      | +030
        jsr     0x2352.l                        | +034
.L0664ca:
        jsr     0x28758.l                       | +03a
        bcs.w   .L066504                        | +040
        jsr     0x27fac.l                       | +044
        bcc.w   .L066504                        | +04a
        jsr     0x283d8.l                       | +04e
        btst    #0x1,0x13(a6)                   | +054
        bne.w   .L066504                        | +05a
        movea.l #0xffffffff,a0                  | +05e
        lea     0x2c79a2.l,a0                   | +064
        jsr     0x5dd56.l                       | +06a
        bcc.w   .L066526                        | +070
.L066504:
        move.w  #0x1022,d0                      | +074
        jsr     0x2352.l                        | +078
        lea     0x77f6a.l,a1                    | +07e
        jsr     0x4ae.l                         | +084
        jsr     0x5dd02.l                       | +08a
        jmp     0x518.l                         | +090
.L066526:
        rts                                     | +096

| ----------------------------------------------------------------------------
|  FloatBarrel_Shard_066528  @ $066528  (158 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_Shard_066528, "ax", @progbits
        .global FloatBarrel_Shard_066528
FloatBarrel_Shard_066528:
        move.w  #0x179,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2c77e6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        jsr     0x267e2.l                       | +016
        move.w  #0x700,0x2a(a6)                 | +01c
        move.w  #0xff80,0x2e(a6)                | +022
        movea.l 0xc(a6),a0                      | +028
        movea.l 0x50(a0),a0                     | +02c
        move.w  0x28(a0),d0                     | +030
        cmpi.w  #0x0,d0                         | +034
        bne.w   .L066568                        | +038
        move.w  #0x800,d0                       | +03c
.L066568:
        asr.w   #0x1,d0                         | +040
        move.w  d0,0x28(a6)                     | +042
        lea     .L066574(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L066574:
        jsr     0x27cee.l                       | +04c
        jsr     0x27fac.l                       | +052
        bcs.w   .L0665a2                        | +058
        cmpi.b  #0x41,d7                        | +05c
        bne.w   .L0665be                        | +060
        lea     0x788c0.l,a1                    | +064
        jsr     0x4ae.l                         | +06a
        jsr     0x5dd02.l                       | +070
        bra.w   .L0665be                        | +076
.L0665a2:
        jsr     0x28d70.l                       | +07a
        movea.l #0xffffffff,a0                  | +080
        lea     0x2c79a2.l,a0                   | +086
        jsr     0x5dd56.l                       | +08c
        bcc.w   .L0665c4                        | +092
.L0665be:
        jmp     0x518.l                         | +096
.L0665c4:
        rts                                     | +09c

| ----------------------------------------------------------------------------
|  FloatBarrel_FlipByFrame_0665c6  @ $0665C6  (28 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_FlipByFrame_0665c6, "ax", @progbits
        .global FloatBarrel_FlipByFrame_0665c6
FloatBarrel_FlipByFrame_0665c6:
        move.b  0x106f28.l,d0                   | +000
        btst    #0x0,d0                         | +006
        bne.w   SetTaskWRts_0665e6              | +00a
        move.w  0x16(a6),d0                     | +00e
        move.w  0x18(a6),0x16(a6)               | +012
        move.w  d0,0x18(a6)                     | +018

| ----------------------------------------------------------------------------
|  FloatBarrel_WaveBob_0665e8  @ $0665E8  (46 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_WaveBob_0665e8, "ax", @progbits
        .global FloatBarrel_WaveBob_0665e8
FloatBarrel_WaveBob_0665e8:
        move.w  0x76(a6),d0                     | +000
        sub.w   0x2a(a6),d0                     | +004
        asr.w   #0x3,d0                         | +008
        move.w  d0,0x2e(a6)                     | +00a
        jsr     0x27cee.l                       | +00e
        move.w  0x2a(a6),d0                     | +014
        sub.w   d0,0x76(a6)                     | +018
        move.w  0x76(a6),d0                     | +01c
        jsr     0x8b58e.l                       | +020
        cmpi.w  #0x600,d0                       | +026
        bgt.w   ClearXN_06661c                  | +02a

| ----------------------------------------------------------------------------
|  FloatBarrel_FaceTarget_066622  @ $066622  (34 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_FaceTarget_066622, "ax", @progbits
        .global FloatBarrel_FaceTarget_066622
FloatBarrel_FaceTarget_066622:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a6),d0                     | +006
        sub.w   0x22(a0),d0                     | +00a
        bmi.w   .L06663c                        | +00e
        move.b  #0x0,0x3a(a6)                   | +012
        rts                                     | +018
.L06663c:
        move.b  #0x1,0x3a(a6)                   | +01a
        rts                                     | +020

| ----------------------------------------------------------------------------
|  FloatBarrel_TargetInRange_066644  @ $066644  (48 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_TargetInRange_066644, "ax", @progbits
        .global FloatBarrel_TargetInRange_066644
FloatBarrel_TargetInRange_066644:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a6),d0                     | +006
        sub.w   0x22(a0),d0                     | +00a
        jsr     0x8b58e.l                       | +00e
        clr.w   d2                              | +014
        move.b  0x9b(a6),d1                     | +016
.L06665e:
        cmpi.b  #0x0,d1                         | +01a
        beq.w   .L06666e                        | +01e
        subq.b  #0x1,d1                         | +022
        addi.w  #0x10,d2                        | +024
        bra.b   .L06665e                        | +028
.L06666e:
        cmp.w   d0,d2                           | +02a
        blt.w   ClearXN_06667a                  | +02c

| ----------------------------------------------------------------------------
|  FloatBarrel_SpawnMine_066680  @ $066680  (10 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_SpawnMine_066680, "ax", @progbits
        .global FloatBarrel_SpawnMine_066680
FloatBarrel_SpawnMine_066680:
        lea     FloatMine_Init_06637a(pc),a1    | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  FloatBarrel_HitCheck_066692  @ $066692  (16 B)
| ----------------------------------------------------------------------------
        .section .text.FloatBarrel_HitCheck_066692, "ax", @progbits
        .global FloatBarrel_HitCheck_066692
FloatBarrel_HitCheck_066692:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0666a8                    | +00c

| ----------------------------------------------------------------------------
|  Paratrooper_Spawner_Tmpl5E_0666ae  @ $0666AE  (230 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Spawner_Tmpl5E_0666ae, "ax", @progbits
        .global Paratrooper_Spawner_Tmpl5E_0666ae
Paratrooper_Spawner_Tmpl5E_0666ae:
        clr.b   0x80(a6)                        | +000
        bra.w   .L0666bc                        | +004
        move.b  #0x1,0x80(a6)                   | +008
.L0666bc:
        tst.b   0x98(a6)                        | +00e
        bne.w   .L0666ca                        | +012
        move.b  #0xff,0x98(a6)                  | +016
.L0666ca:
        tst.b   0x9b(a6)                        | +01c
        bne.w   .L0666d8                        | +020
        move.b  #0xff,0x9b(a6)                  | +024
.L0666d8:
        move.b  0x99(a6),d0                     | +02a
        andi.w  #0xff,d0                        | +02e
        lsl.w   #0x5,d0                         | +032
        move.w  d0,0x7e(a6)                     | +034
        clr.b   0x21(a6)                        | +038
        clr.b   0x20(a6)                        | +03c
        lea     .L0666f4(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L0666f4:
        jsr     0x2783a.l                       | +046
        move.w  0x7e(a6),d0                     | +04c
        cmp.w   0x106f50.l,d0                   | +050
        blt.w   SetTaskHandler_06679c           | +056
        tst.b   0x21(a6)                        | +05a
        bne.w   .L06676c                        | +05e
        tst.b   0x98(a6)                        | +062
        beq.w   SetTaskHandler_06679c           | +066
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L06676c                        | +074
        lea     0x2b889a.l,a0                   | +078
        jsr     0x799de.l                       | +07e
        move.w  d0,0x70(a6)                     | +084
        lea     Paratrooper_Init_0667c4(pc),a1  | +088
        jsr     0x4ae.l                         | +08c
        jsr     0x5dd02.l                       | +092
        move.b  0x80(a6),0x80(a0)               | +098
        move.b  0x9a(a6),0x9a(a0)               | +09e
        move.b  0x9b(a6),0x9b(a0)               | +0a4
        move.b  0x9c(a6),0x9c(a0)               | +0aa
        cmpi.b  #0xff,0x98(a6)                  | +0b0
        beq.w   .L06676c                        | +0b6
        subq.b  #0x1,0x98(a6)                   | +0ba
.L06676c:
        clr.b   0x21(a6)                        | +0be
        cmpi.w  #0x10,0x22(a6)                  | +0c2
        bgt.w   .L066780                        | +0c8
        lea     Paratrooper_Spawner_Done_0667a4(pc),a1 | +0cc
        move.l  a1,(a6)                         | +0d0
.L066780:
        movea.l #0xffffffff,a0                  | +0d2
        lea     Paratrooper_Sprites_066cd8__L066dd0(pc),a0 | +0d8
        jsr     0x5dd5c.l                       | +0dc
        bcc.w   SetHandlerRts_06679a            | +0e2

| ----------------------------------------------------------------------------
|  Paratrooper_Spawner_Done_0667a4  @ $0667A4  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Spawner_Done_0667a4, "ax", @progbits
        .global Paratrooper_Spawner_Done_0667a4
Paratrooper_Spawner_Done_0667a4:
        move.b  #0x1,0x20(a6)                   | +000
        lea     .L0667b0(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0667b0:
        tst.b   0x21(a6)                        | +00c
        bne.w   .L0667be                        | +010
        lea     JmpToScheduler_066a86(pc),a1    | +014
        move.l  a1,(a6)                         | +018
.L0667be:
        clr.b   0x21(a6)                        | +01a
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Paratrooper_Init_0667c4  @ $0667C4  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Init_0667c4, "ax", @progbits
        .global Paratrooper_Init_0667c4
Paratrooper_Init_0667c4:
        move.w  #0x183,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x14,0x38(a6)                  | +020
        move.b  0x9c(a6),d0                     | +026
        andi.w  #0xff,d0                        | +02a
        bne.w   .L066802                        | +02e
        lea     0x2b891c.l,a0                   | +032
        jsr     0x799de.l                       | +038
.L066802:
        move.w  d0,0x70(a6)                     | +03e
        tst.b   0x80(a6)                        | +042
        beq.w   .L066836                        | +046
        move.w  #0xa0,0x22(a6)                  | +04a
        jsr     0x5e1ea.l                       | +050
        bcs.w   .L066836                        | +056
        move.w  0x22(a0),d0                     | +05a
        cmpi.w  #0x20,d0                        | +05e
        blt.w   .L066836                        | +062
        cmpi.w  #0x120,d0                       | +066
        bgt.w   .L066836                        | +06a
        move.w  d0,0x22(a6)                     | +06e
.L066836:
        lea     Paratrooper_Sprites_066cd8__L066dd8(pc),a0 | +072
        jsr     0x28cd4.l                       | +076
        lea     .L066846(pc),a1                 | +07c
        move.l  a1,(a6)                         | +080
.L066846:
        jsr     0x2783a.l                       | +082
        jsr     0x28d70.l                       | +088
        bcc.w   .L06685c                        | +08e
        lea     Paratrooper_Land_066860(pc),a1  | +092
        move.l  a1,(a6)                         | +096
.L06685c:
        bra.w   Paratrooper_Die_0669ea__L066a6c | +098

| ----------------------------------------------------------------------------
|  Paratrooper_Land_066860  @ $066860  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Land_066860, "ax", @progbits
        .global Paratrooper_Land_066860
Paratrooper_Land_066860:
        move.l  #0x66d7c,0x48(a6)               | +000
        lea     Paratrooper_Sprites_066cd8__L066e48(pc),a0 | +008
        jsr     0x28cd4.l                       | +00c
        lea     .L066878(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L066878:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        jsr     Paratrooper_TimerInBounds_066c12(pc) | +024
        bcc.w   .L066892                        | +028
        lea     Paratrooper_PickWalk_066896(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L066892:
        bra.w   Paratrooper_Die_0669ea__L066a3c | +032

| ----------------------------------------------------------------------------
|  Paratrooper_PickWalk_066896  @ $066896  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_PickWalk_066896, "ax", @progbits
        .global Paratrooper_PickWalk_066896
Paratrooper_PickWalk_066896:
        cmpi.b  #0xff,0x9b(a6)                  | +000
        beq.w   .L0668a4                        | +006
        subq.b  #0x1,0x9b(a6)                   | +00a
.L0668a4:
        tst.b   0x9a(a6)                        | +00e
        beq.w   .L0668b4                        | +012
        jsr     Paratrooper_Approach_066bca(pc) | +016
        bcc.w   Paratrooper_WalkA_0668d4__L06693a | +01a
.L0668b4:
        lea     0x2b8a60.l,a0                   | +01e
        jsr     0x799de.l                       | +024
        move.w  d0,0x74(a6)                     | +02a
        lea     0x2b89de.l,a0                   | +02e
        jsr     0x799de.l                       | +034
        move.w  d0,0x72(a6)                     | +03a

| ----------------------------------------------------------------------------
|  Paratrooper_WalkA_0668d4  @ $0668D4  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_WalkA_0668d4, "ax", @progbits
        .global Paratrooper_WalkA_0668d4
Paratrooper_WalkA_0668d4:
        move.w  0x72(a6),0x70(a6)               | +000
        lea     Paratrooper_Sprites_066cd8__L066e8a(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L0668ea(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0668ea:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        bcc.w   .L066904                        | +022
        lea     Paratrooper_Sprites_066cd8__L066e48(pc),a0 | +026
        jsr     0x28cd4.l                       | +02a
.L066904:
        jsr     Paratrooper_TimerInBounds_066c12(pc) | +030
        bcc.w   .L066936                        | +034
        lea     Paratrooper_WalkA_0668d4(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        subq.w  #0x1,0x74(a6)                   | +03e
        cmpi.w  #0x0,0x74(a6)                   | +042
        bgt.w   .L066936                        | +048
        lea     Paratrooper_Land_066860(pc),a1  | +04c
        move.l  a1,(a6)                         | +050
        lea     0x2b891c.l,a0                   | +052
        jsr     0x799de.l                       | +058
        move.w  d0,0x70(a6)                     | +05e
.L066936:
        bra.w   Paratrooper_Die_0669ea__L066a3c | +062
        .global Paratrooper_WalkA_0668d4__L06693a
Paratrooper_WalkA_0668d4__L06693a:
.L06693a:
        lea     0x2b8a60.l,a0                   | +066
        jsr     0x799de.l                       | +06c
        move.w  d0,0x74(a6)                     | +072
        lea     0x2b89de.l,a0                   | +076
        jsr     0x799de.l                       | +07c
        move.w  d0,0x72(a6)                     | +082

| ----------------------------------------------------------------------------
|  Paratrooper_WalkB_06695a  @ $06695A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_WalkB_06695a, "ax", @progbits
        .global Paratrooper_WalkB_06695a
Paratrooper_WalkB_06695a:
        move.w  0x72(a6),0x70(a6)               | +000
        lea     Paratrooper_Sprites_066cd8__L066ec4(pc),a0 | +006
        jsr     0x28cd4.l                       | +00a
        lea     .L066970(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L066970:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        bcc.w   .L06698a                        | +022
        lea     Paratrooper_Sprites_066cd8__L066e48(pc),a0 | +026
        jsr     0x28cd4.l                       | +02a
.L06698a:
        jsr     Paratrooper_TimerInBounds_066c12(pc) | +030
        bcc.w   .L0669bc                        | +034
        lea     Paratrooper_WalkB_06695a(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
        subq.w  #0x1,0x74(a6)                   | +03e
        cmpi.w  #0x0,0x74(a6)                   | +042
        bgt.w   .L0669bc                        | +048
        lea     Paratrooper_Land_066860(pc),a1  | +04c
        move.l  a1,(a6)                         | +050
        lea     0x2b891c.l,a0                   | +052
        jsr     0x799de.l                       | +058
        move.w  d0,0x70(a6)                     | +05e
.L0669bc:
        bra.w   Paratrooper_Die_0669ea__L066a3c | +062

| ----------------------------------------------------------------------------
|  Paratrooper_Leave_0669c0  @ $0669C0  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Leave_0669c0, "ax", @progbits
        .global Paratrooper_Leave_0669c0
Paratrooper_Leave_0669c0:
        lea     Paratrooper_Sprites_066cd8__L066efe(pc),a0 | +000
        jsr     0x28cd4.l                       | +004
        lea     .L0669d0(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L0669d0:
        jsr     0x2783a.l                       | +010
        jsr     0x28d70.l                       | +016
        bcc.w   .L0669e6                        | +01c
        lea     JmpToScheduler_066a86(pc),a1    | +020
        move.l  a1,(a6)                         | +024
.L0669e6:
        bra.w   Paratrooper_Die_0669ea__L066a5c | +026

| ----------------------------------------------------------------------------
|  Paratrooper_Die_0669ea  @ $0669EA  (148 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Die_0669ea, "ax", @progbits
        .global Paratrooper_Die_0669ea
Paratrooper_Die_0669ea:
        jsr     0x13600.l                       | +000
        move.w  #0x183,d1                       | +006
        tst.b   0x10fd8f.l                      | +00a
        bne.w   .L066a02                        | +010
        move.w  #0x1d7,d1                       | +014
.L066a02:
        jsr     0x236e.l                        | +018
        move.w  #0x1043,d0                      | +01e
        jsr     0x2352.l                        | +022
        lea     Paratrooper_Sprites_066cd8__L066f6e(pc),a0 | +028
        jsr     0x28cd4.l                       | +02c
        lea     .L066a22(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L066a22:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L066a38                        | +044
        lea     JmpToScheduler_066a86(pc),a1    | +048
        move.l  a1,(a6)                         | +04c
.L066a38:
        bra.w   .L066a6c                        | +04e
        .global Paratrooper_Die_0669ea__L066a3c
Paratrooper_Die_0669ea__L066a3c:
.L066a3c:
        movea.l 0xc(a6),a0                      | +052
        tst.b   0x20(a0)                        | +056
        beq.w   .L066a4e                        | +05a
        lea     Paratrooper_Leave_0669c0(pc),a1 | +05e
        move.l  a1,(a6)                         | +062
.L066a4e:
        tst.b   0x9b(a6)                        | +064
        bne.w   .L066a5c                        | +068
        lea     Paratrooper_Leave_0669c0(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
        .global Paratrooper_Die_0669ea__L066a5c
Paratrooper_Die_0669ea__L066a5c:
.L066a5c:
        jsr     0x2870a.l                       | +072
        bcc.w   .L066a6c                        | +078
        lea     Paratrooper_Die_0669ea(pc),a1   | +07c
        move.l  a1,(a6)                         | +080
        .global Paratrooper_Die_0669ea__L066a6c
Paratrooper_Die_0669ea__L066a6c:
.L066a6c:
        movea.l 0xc(a6),a0                      | +082
        addq.b  #0x1,0x21(a0)                   | +086
        cmpi.w  #0x0,0x22(a6)                   | +08a
        bgt.w   SetHandlerRts_066a84            | +090

| ----------------------------------------------------------------------------
|  Paratrooper_FragExplode_066a9c  @ $066A9C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_FragExplode_066a9c, "ax", @progbits
        .global Paratrooper_FragExplode_066a9c
Paratrooper_FragExplode_066a9c:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        jmp     0x77f6a.l                       | +016

| ----------------------------------------------------------------------------
|  Paratrooper_Frag_066ab8  @ $066AB8  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Frag_066ab8, "ax", @progbits
        .global Paratrooper_Frag_066ab8
Paratrooper_Frag_066ab8:
        lea     0x2b8ae2.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  0x76(a6),d3                     | +00c
        andi.w  #0xff,d3                        | +010
        add.w   d3,d3                           | +014
        lea     0x2c07ac.l,a1                   | +016
        lea     0x2c072c.l,a2                   | +01c
        move.w  (a1,d3.w),d1                    | +022
        move.w  (a2,d3.w),d2                    | +026
        muls.w  d0,d1                           | +02a
        muls.w  d0,d2                           | +02c
        asr.l   #0x8,d1                         | +02e
        asr.l   #0x8,d2                         | +030
        move.w  d1,0x28(a6)                     | +032
        move.w  d2,0x2a(a6)                     | +036
        move.w  #0xd000,d0                      | +03a
        jsr     0x28134.l                       | +03e
        andi.w  #0xffe3,0x38(a6)                | +044
        ori.w   #0x14,0x38(a6)                  | +04a
        move.w  #0x17b,d1                       | +050
        jsr     0x236e.l                        | +054
        lea     Paratrooper_Sprites_066cd8__L066fee(pc),a0 | +05a
        jsr     0x28cd4.l                       | +05e
        lea     .L066b22(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L066b22:
        jsr     0x27cee.l                       | +06a
        jsr     0x28d70.l                       | +070
        jsr     0x283d8.l                       | +076
        btst    #0x1,0x13(a6)                   | +07c
        beq.w   .L066b44                        | +082
        lea     Paratrooper_FragExplode_066a9c(pc),a1 | +086
        move.l  a1,(a6)                         | +08a
.L066b44:
        cmpi.w  #0x1c8,0x24(a6)                 | +08c
        blt.w   .L066b54                        | +092
        lea     Paratrooper_FragExplode_066a9c(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L066b54:
        movea.l #0xffffffff,a0                  | +09c
        jsr     0x5dd56.l                       | +0a2
        bcc.w   SetHandlerRts_066b6a            | +0a8

| ----------------------------------------------------------------------------
|  Paratrooper_Smoke_066b6c  @ $066B6C  (86 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Smoke_066b6c, "ax", @progbits
        .global Paratrooper_Smoke_066b6c
Paratrooper_Smoke_066b6c:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        move.w  #0x4,d1                         | +016
        jsr     0x236e.l                        | +01a
        lea     Paratrooper_Sprites_066cd8__L067014(pc),a0 | +020
        jsr     0x28cd4.l                       | +024
        lea     .L066b9c(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L066b9c:
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        bcc.w   .L066bb2                        | +03c
        lea     Jsr5B6ThenJmpScheduler_066a8e(pc),a1 | +040
        move.l  a1,(a6)                         | +044
.L066bb2:
        movea.l #0xffffffff,a0                  | +046
        jsr     0x5dd56.l                       | +04c
        bcc.w   SetHandlerRts_066bc8            | +052

| ----------------------------------------------------------------------------
|  Paratrooper_Approach_066bca  @ $066BCA  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Approach_066bca, "ax", @progbits
        .global Paratrooper_Approach_066bca
Paratrooper_Approach_066bca:
        jsr     0x5e0d4.l                       | +000
        move.w  0x22(a0),d0                     | +006
        sub.w   0x22(a6),d0                     | +00a
        cmpi.w  #0x0,d0                         | +00e
        bgt.w   .L066be2                        | +012
        neg.w   d0                              | +016
.L066be2:
        cmpi.w  #0x20,d0                        | +018
        blt.w   SetXN_066c00                    | +01c
        subi.w  #0x18,0x22(a6)                  | +020
        jsr     0x5e5a8.l                       | +026
        bcc.w   Paratrooper_ApproachFail_066c06 | +02c
        addi.w  #0x18,0x22(a6)                  | +030

| ----------------------------------------------------------------------------
|  Paratrooper_ApproachFail_066c06  @ $066C06  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_ApproachFail_066c06, "ax", @progbits
        .global Paratrooper_ApproachFail_066c06
Paratrooper_ApproachFail_066c06:
        addi.w  #0x18,0x22(a6)                  | +000

| ----------------------------------------------------------------------------
|  Paratrooper_TimerInBounds_066c12  @ $066C12  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_TimerInBounds_066c12, "ax", @progbits
        .global Paratrooper_TimerInBounds_066c12
Paratrooper_TimerInBounds_066c12:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x20,0x22(a6)                  | +004
        blt.w   ClearXN_066c3a                  | +00a
        cmpi.w  #0x120,0x22(a6)                 | +00e
        bgt.w   ClearXN_066c3a                  | +014
        cmpi.w  #0x0,0x70(a6)                   | +018
        bgt.w   ClearXN_066c3a                  | +01e

| ----------------------------------------------------------------------------
|  Paratrooper_SpawnPairA_066c40  @ $066C40  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_SpawnPairA_066c40, "ax", @progbits
        .global Paratrooper_SpawnPairA_066c40
Paratrooper_SpawnPairA_066c40:
        lea     Paratrooper_Smoke_066b6c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0xfff8,0x22(a0)                | +010
        addi.w  #0x20,0x24(a0)                  | +016
        lea     Paratrooper_Frag_066ab8(pc),a1  | +01c
        jsr     0x4ae.l                         | +020
        jsr     0x5dd02.l                       | +026
        addi.w  #0xfff8,0x22(a0)                | +02c
        addi.w  #0x20,0x24(a0)                  | +032
        move.b  #0x40,0x76(a0)                  | +038
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Paratrooper_SpawnPairB_066c80  @ $066C80  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_SpawnPairB_066c80, "ax", @progbits
        .global Paratrooper_SpawnPairB_066c80
Paratrooper_SpawnPairB_066c80:
        lea     Paratrooper_Smoke_066b6c(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0xffec,0x22(a0)                | +010
        addi.w  #0x1c,0x24(a0)                  | +016
        lea     Paratrooper_Frag_066ab8(pc),a1  | +01c
        jsr     0x4ae.l                         | +020
        jsr     0x5dd02.l                       | +026
        addi.w  #0xffec,0x22(a0)                | +02c
        addi.w  #0x1c,0x24(a0)                  | +032
        move.b  #0x50,0x76(a0)                  | +038
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Paratrooper_SpawnChute_066cc0  @ $066CC0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_SpawnChute_066cc0, "ax", @progbits
        .global Paratrooper_SpawnChute_066cc0
Paratrooper_SpawnChute_066cc0:
        lea     0x788e4.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        subq.w  #0x1,0x38(a0)                   | +012
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Paratrooper_Sprites_066cd8  @ $066CD8  (1006 B)
| ----------------------------------------------------------------------------
        .section .text.Paratrooper_Sprites_066cd8, "ax", @progbits
        .global Paratrooper_Sprites_066cd8
Paratrooper_Sprites_066cd8:
        .dc.w   0x000f                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +014  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +016  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +020  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0402                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +054  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +064  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +066  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +070  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +072  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +074  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f6  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066dd0
Paratrooper_Sprites_066cd8__L066dd0:
.L066dd0:
        .dc.w   0xffe0                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0fe  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066dd8
Paratrooper_Sprites_066cd8__L066dd8:
.L066dd8:
        .dc.w   0x0001                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +104  (dato / opcode no decodificado)
        .dc.w   0x4666                        | +106  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x465a                        | +110  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +118  (dato / opcode no decodificado)
        .dc.w   0x464e                        | +11a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +122  (dato / opcode no decodificado)
        .dc.w   0x4640                        | +124  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x4632                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +136  (dato / opcode no decodificado)
        .dc.w   0x4624                        | +138  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +140  (dato / opcode no decodificado)
        .dc.w   0x460c                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x45f4                        | +14c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +154  (dato / opcode no decodificado)
        .dc.w   0x45e0                        | +156  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x45c4                        | +160  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +164  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +168  (dato / opcode no decodificado)
        .dc.w   0x45a6                        | +16a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +16e  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066e48
Paratrooper_Sprites_066cd8__L066e48:
.L066e48:
        .dc.w   0x0004                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +174  (dato / opcode no decodificado)
        .dc.w   0x43dc                        | +176  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x43fe                        | +180  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +188  (dato / opcode no decodificado)
        .dc.w   0x4420                        | +18a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +18c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +18e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +190  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +192  (dato / opcode no decodificado)
        .dc.w   0x4440                        | +194  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +196  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +198  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +19a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +19c  (dato / opcode no decodificado)
        .dc.w   0x4420                        | +19e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1a0  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1a2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1a4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1a6  (dato / opcode no decodificado)
        .dc.w   0x43fe                        | +1a8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1aa  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +1ac  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1ae  (dato / opcode no decodificado)
        .dc.w   0x6e48                        | +1b0  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066e8a
Paratrooper_Sprites_066cd8__L066e8a:
.L066e8a:
        .dc.w   0x0800                        | +1b2  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1b4  (dato / opcode no decodificado)
        .dc.w   0x6c40                        | +1b6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1b8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1ba  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1bc  (dato / opcode no decodificado)
        .dc.w   0x4460                        | +1be  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1c0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1c2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1c4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1c6  (dato / opcode no decodificado)
        .dc.w   0x447e                        | +1c8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1ca  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +1cc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1ce  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1d0  (dato / opcode no decodificado)
        .dc.w   0x449c                        | +1d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1d4  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +1d6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1d8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1da  (dato / opcode no decodificado)
        .dc.w   0x447e                        | +1dc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1de  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +1e0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1e2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1e4  (dato / opcode no decodificado)
        .dc.w   0x4460                        | +1e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1e8  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +1ea  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066ec4
Paratrooper_Sprites_066cd8__L066ec4:
.L066ec4:
        .dc.w   0x0800                        | +1ec  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +1ee  (dato / opcode no decodificado)
        .dc.w   0x6c80                        | +1f0  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1f2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1f4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +1f6  (dato / opcode no decodificado)
        .dc.w   0x454e                        | +1f8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +1fa  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +1fc  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +1fe  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +200  (dato / opcode no decodificado)
        .dc.w   0x456c                        | +202  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +204  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +206  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +208  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +20a  (dato / opcode no decodificado)
        .dc.w   0x458a                        | +20c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +20e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +210  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +212  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +214  (dato / opcode no decodificado)
        .dc.w   0x456c                        | +216  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +218  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +21a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +21c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +21e  (dato / opcode no decodificado)
        .dc.w   0x454e                        | +220  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +222  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +224  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066efe
Paratrooper_Sprites_066cd8__L066efe:
.L066efe:
        .dc.w   0x0003                        | +226  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +228  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +22a  (dato / opcode no decodificado)
        .dc.w   0x45a6                        | +22c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +22e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +230  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +232  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +234  (dato / opcode no decodificado)
        .dc.w   0x45c4                        | +236  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +238  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +23a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +23c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +23e  (dato / opcode no decodificado)
        .dc.w   0x45e0                        | +240  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +242  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +244  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +246  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +248  (dato / opcode no decodificado)
        .dc.w   0x45f4                        | +24a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +24c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +24e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +250  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +252  (dato / opcode no decodificado)
        .dc.w   0x460c                        | +254  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +256  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +258  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +25a  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +25c  (dato / opcode no decodificado)
        .dc.w   0x4624                        | +25e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +260  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +262  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +264  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +266  (dato / opcode no decodificado)
        .dc.w   0x4632                        | +268  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +26a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +26c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +26e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +270  (dato / opcode no decodificado)
        .dc.w   0x4640                        | +272  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +274  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +276  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +278  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +27a  (dato / opcode no decodificado)
        .dc.w   0x464e                        | +27c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +27e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +280  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +282  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +284  (dato / opcode no decodificado)
        .dc.w   0x465a                        | +286  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +288  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +28a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +28c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +28e  (dato / opcode no decodificado)
        .dc.w   0x4666                        | +290  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +292  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +294  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066f6e
Paratrooper_Sprites_066cd8__L066f6e:
.L066f6e:
        .dc.w   0x0800                        | +296  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +298  (dato / opcode no decodificado)
        .dc.w   0x6cc0                        | +29a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +29c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +29e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2a0  (dato / opcode no decodificado)
        .dc.w   0x4672                        | +2a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2a4  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2a6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2a8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2aa  (dato / opcode no decodificado)
        .dc.w   0x4694                        | +2ac  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ae  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2b0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2b2  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2b4  (dato / opcode no decodificado)
        .dc.w   0x46b2                        | +2b6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2b8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2ba  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2bc  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2be  (dato / opcode no decodificado)
        .dc.w   0x46cc                        | +2c0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2c2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2c4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2c6  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2c8  (dato / opcode no decodificado)
        .dc.w   0x46e6                        | +2ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2cc  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2ce  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2d0  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2d2  (dato / opcode no decodificado)
        .dc.w   0x4704                        | +2d4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2d6  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +2d8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2da  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2dc  (dato / opcode no decodificado)
        .dc.w   0x471e                        | +2de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2e0  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2e2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2e4  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2e6  (dato / opcode no decodificado)
        .dc.w   0x4738                        | +2e8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2ea  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2ec  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2ee  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2f0  (dato / opcode no decodificado)
        .dc.w   0x4752                        | +2f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2f4  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +2f6  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +2f8  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +2fa  (dato / opcode no decodificado)
        .dc.w   0x476c                        | +2fc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +2fe  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +300  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +302  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +304  (dato / opcode no decodificado)
        .dc.w   0x478a                        | +306  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +308  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +30a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +30c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +30e  (dato / opcode no decodificado)
        .dc.w   0x47a8                        | +310  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +312  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +314  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L066fee
Paratrooper_Sprites_066cd8__L066fee:
.L066fee:
        .dc.w   0x0a00                        | +316  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +318  (dato / opcode no decodificado)
        .dc.w   0x6cd8                        | +31a  (dato / opcode no decodificado)
        .dc.w   0x0800                        | +31c  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +31e  (dato / opcode no decodificado)
        .dc.w   0x83ca                        | +320  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +322  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +324  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +326  (dato / opcode no decodificado)
        .dc.w   0xff98                        | +328  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +32a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +32c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +32e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +330  (dato / opcode no decodificado)
        .dc.w   0xffa2                        | +332  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +334  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +336  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +338  (dato / opcode no decodificado)
        .dc.w   0x6ffa                        | +33a  (dato / opcode no decodificado)
        .global Paratrooper_Sprites_066cd8__L067014
Paratrooper_Sprites_066cd8__L067014:
.L067014:
        .dc.w   0x0001                        | +33c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +33e  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +340  (dato / opcode no decodificado)
        .dc.w   0xffac                        | +342  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +344  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +346  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +348  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +34a  (dato / opcode no decodificado)
        .dc.w   0xffbc                        | +34c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +34e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +350  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +352  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +354  (dato / opcode no decodificado)
        .dc.w   0xffcc                        | +356  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +358  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +35a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +35c  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +35e  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +360  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +362  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +364  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +366  (dato / opcode no decodificado)
        .dc.w   0x0023                        | +368  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +36a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +36c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +36e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +370  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +372  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +374  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +376  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +378  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +37a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +37c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +37e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +380  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +382  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +384  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +386  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +388  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +38a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +38c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +38e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +390  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +392  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +394  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +396  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +398  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +39a  (dato / opcode no decodificado)
        .dc.w   0x0048                        | +39c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +39e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3a0  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3a2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3a4  (dato / opcode no decodificado)
        .dc.w   0x0060                        | +3a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3a8  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3aa  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3ac  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3ae  (dato / opcode no decodificado)
        .dc.w   0x0078                        | +3b0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3b2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +3b4  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3b6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3b8  (dato / opcode no decodificado)
        .dc.w   0x0090                        | +3ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3bc  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3be  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3c0  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3c2  (dato / opcode no decodificado)
        .dc.w   0x00a4                        | +3c4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3c6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3c8  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3ca  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3cc  (dato / opcode no decodificado)
        .dc.w   0x00b4                        | +3ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3d0  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +3d2  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +3d4  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +3d6  (dato / opcode no decodificado)
        .dc.w   0x00c4                        | +3d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +3da  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +3dc  (dato / opcode no decodificado)
        movea.l 0x8(a6),a1                      | +3de
        move.b  0x10(a6),d0                     | +3e2
        cmp.b   0x10(a1),d0                     | +3e6
        bcs.w   SetXN_0670cc                    | +3ea

| ----------------------------------------------------------------------------
|  ShieldSoldier_Tmpl6C_0670d2  @ $0670D2  (258 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Tmpl6C_0670d2, "ax", @progbits
        .global ShieldSoldier_Tmpl6C_0670d2
ShieldSoldier_Tmpl6C_0670d2:
        jsr     ShieldSoldier_Setup_067e7e(pc)  | +000
        lea     0x2bf6de.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.w  d0,0x74(a6)                     | +010
        cmpi.b  #0x0,0x98(a6)                   | +014
        beq.w   .L0670f6                        | +01a
        move.w  #0x36,0x74(a6)                  | +01e
.L0670f6:
        tst.b   0x99(a6)                        | +024
        beq.w   ShieldSoldier_Fall_067220__L067254 | +028
        .global ShieldSoldier_Tmpl6C_0670d2__L0670fe
ShieldSoldier_Tmpl6C_0670d2__L0670fe:
.L0670fe:
        lea     0x2c7d1e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L067110(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L067110:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +03e
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +042
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +046
        jsr     0x28d70.l                       | +04a
        tst.b   0x99(a6)                        | +050
        bne.w   .L067146                        | +054
        jsr     ShieldSoldier_PlayerNearEdge_06815a(pc) | +058
        bcc.w   .L067138                        | +05c
        lea     ShieldSoldier_Land_0672e0__L06731e(pc),a1 | +060
        move.l  a1,(a6)                         | +064
.L067138:
        jsr     ShieldSoldier_PlayerNear_06810a(pc) | +066
        bcc.w   .L067146                        | +06a
        lea     ShieldSoldier_Fall_067220__L067254(pc),a1 | +06e
        move.l  a1,(a6)                         | +072
.L067146:
        jsr     ShieldSoldier_GuardTick_068186(pc) | +074
        bcc.w   .L067154                        | +078
        lea     ShieldSoldier_Guard_0671dc(pc),a1 | +07c
        move.l  a1,(a6)                         | +080
.L067154:
        jsr     ShieldSoldier_CanAttackA_068020(pc) | +082
        bcc.w   .L067162                        | +086
        lea     ShieldSoldier_Crouch_067394(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
.L067162:
        jsr     ShieldSoldier_CanAttackC_0680a4(pc) | +090
        bcc.w   .L067170                        | +094
        lea     ShieldSoldier_Aim_0674a2(pc),a1 | +098
        move.l  a1,(a6)                         | +09c
.L067170:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +09e
        bcc.w   .L06717e                        | +0a2
        lea     ShieldSoldier_Fall_067220(pc),a1 | +0a6
        move.l  a1,(a6)                         | +0aa
.L06717e:
        subq.w  #0x1,0x72(a6)                   | +0ac
        jsr     0x2870a.l                       | +0b0
        bcc.w   .L0671b4                        | +0b6
        bclr    #0x3,0x13(a6)                   | +0ba
        lea     0x5e766.l,a0                    | +0c0
        jsr     0x5e770.l                       | +0c6
        lea     ShieldSoldier_Recover_06756c__L0675bc(pc),a1 | +0cc
        move.l  a1,(a6)                         | +0d0
        cmpi.b  #0x1,0x58(a6)                   | +0d2
        beq.w   .L0671b4                        | +0d8
        lea     ShieldSoldier_Recover_06756c__L0675d2(pc),a1 | +0dc
        move.l  a1,(a6)                         | +0e0
.L0671b4:
        jsr     0x28758.l                       | +0e2
        bcs.w   ShieldSoldier_Die_067692        | +0e8
        movea.l #0xffffffff,a0                  | +0ec
        lea     0x2c7ce6.l,a0                   | +0f2
        jsr     0x5dd5c.l                       | +0f8
        bcc.w   SetHandlerRts_0671da            | +0fe

| ----------------------------------------------------------------------------
|  ShieldSoldier_Guard_0671dc  @ $0671DC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Guard_0671dc, "ax", @progbits
        .global ShieldSoldier_Guard_0671dc
ShieldSoldier_Guard_0671dc:
        clr.w   0x76(a6)                        | +000
        lea     0x2c7ee6.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L0671f2(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0671f2:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +016
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +01a
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L06720e                        | +028
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L06720e:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +032
        bcc.w   .L06721c                        | +036
        lea     ShieldSoldier_Fall_067220(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L06721c:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +040

| ----------------------------------------------------------------------------
|  ShieldSoldier_Fall_067220  @ $067220  (192 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Fall_067220, "ax", @progbits
        .global ShieldSoldier_Fall_067220
ShieldSoldier_Fall_067220:
        lea     0x2c7f54.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L067232(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L067232:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +012
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +016
        jsr     0x27c8c.l                       | +01a
        bcc.w   .L06724a                        | +020
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L06724a:
        jsr     0x28d70.l                       | +02a
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +030
        .global ShieldSoldier_Fall_067220__L067254
ShieldSoldier_Fall_067220__L067254:
.L067254:
        jsr     ShieldSoldier_RngRunSpeed_067f86(pc) | +034
        move.l  #0x2c7a8a,0x48(a6)              | +038
        lea     0x2c7da4.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        lea     .L067272(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L067272:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +052
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +056
        jsr     ShieldSoldier_ScrollProbeB_067f62(pc) | +05a
        jsr     0x28d70.l                       | +05e
        jsr     ShieldSoldier_CanAttackA_068020__L068034(pc) | +064
        bcc.w   .L067292                        | +068
        lea     ShieldSoldier_Land_0672e0(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L067292:
        jsr     ShieldSoldier_CanAttackC_0680a4__L0680b8(pc) | +072
        bcc.w   .L0672a0                        | +076
        lea     ShieldSoldier_Land_0672e0(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L0672a0:
        jsr     ShieldSoldier_GuardTick_068186(pc) | +080
        bcc.w   .L0672ae                        | +084
        lea     ShieldSoldier_Land_0672e0(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L0672ae:
        jsr     ShieldSoldier_PlayerFar_0680e8(pc) | +08e
        bcc.w   .L0672bc                        | +092
        lea     ShieldSoldier_Land_0672e0(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L0672bc:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +09c
        bcc.w   .L0672ca                        | +0a0
        lea     ShieldSoldier_Fall_067220(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L0672ca:
        cmpi.l  #0x67272,(a6)                   | +0aa
        beq.w   .L0672dc                        | +0b0
        move.l  #0x2c79e2,0x48(a6)              | +0b4
.L0672dc:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +0bc

| ----------------------------------------------------------------------------
|  ShieldSoldier_Land_0672e0  @ $0672E0  (180 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Land_0672e0, "ax", @progbits
        .global ShieldSoldier_Land_0672e0
ShieldSoldier_Land_0672e0:
        jsr     0x267e2.l                       | +000
        lea     0x2c7e2a.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0672f8(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0672f8:
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L06730c                        | +022
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L06730c:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +02c
        bcc.w   .L06731a                        | +030
        lea     ShieldSoldier_Fall_067220(pc),a1 | +034
        move.l  a1,(a6)                         | +038
.L06731a:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +03a
        .global ShieldSoldier_Land_0672e0__L06731e
ShieldSoldier_Land_0672e0__L06731e:
.L06731e:
        jsr     ShieldSoldier_RngRunSpeed_067f86(pc) | +03e
        neg.w   0x28(a6)                        | +042
        lea     0x2c7e92.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     .L067338(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L067338:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +058
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +05c
        jsr     ShieldSoldier_ScrollProbeB_067f62(pc) | +060
        jsr     0x28d70.l                       | +064
        jsr     ShieldSoldier_PlayerFarOrEdge_068130(pc) | +06a
        bcc.w   .L067358                        | +06e
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +072
        move.l  a1,(a6)                         | +076
.L067358:
        jsr     ShieldSoldier_CanAttackA_068020(pc) | +078
        bcc.w   .L067366                        | +07c
        lea     ShieldSoldier_Crouch_067394(pc),a1 | +080
        move.l  a1,(a6)                         | +084
.L067366:
        jsr     ShieldSoldier_CanAttackC_0680a4(pc) | +086
        bcc.w   .L067374                        | +08a
        lea     ShieldSoldier_Aim_0674a2(pc),a1 | +08e
        move.l  a1,(a6)                         | +092
.L067374:
        jsr     ShieldSoldier_GuardTick_068186(pc) | +094
        bcc.w   .L067382                        | +098
        lea     ShieldSoldier_Guard_0671dc(pc),a1 | +09c
        move.l  a1,(a6)                         | +0a0
.L067382:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +0a2
        bcc.w   .L067390                        | +0a6
        lea     ShieldSoldier_Fall_067220(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
.L067390:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +0b0

| ----------------------------------------------------------------------------
|  ShieldSoldier_Crouch_067394  @ $067394  (80 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Crouch_067394, "ax", @progbits
        .global ShieldSoldier_Crouch_067394
ShieldSoldier_Crouch_067394:
        lea     0x2bf822.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x82(a6)                     | +00c
        lea     0x2c7fb0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L0673b6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L0673b6:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0673d2                        | +034
        lea     ShieldSoldier_CrouchHold_0673e4(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0673d2:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L0673e0                        | +042
        lea     ShieldSoldier_Fall_067220(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L0673e0:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +04c

| ----------------------------------------------------------------------------
|  ShieldSoldier_CrouchHold_0673e4  @ $0673E4  (110 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_CrouchHold_0673e4, "ax", @progbits
        .global ShieldSoldier_CrouchHold_0673e4
ShieldSoldier_CrouchHold_0673e4:
        lea     0x2bf926.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c802a.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067406(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067406:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067440                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L067440                        | +03e
        lea     ShieldSoldier_CrouchHold_0673e4(pc),a1 | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L067440                        | +052
        lea     ShieldSoldier_Rise_067452(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        .global ShieldSoldier_CrouchHold_0673e4__L067440
ShieldSoldier_CrouchHold_0673e4__L067440:
.L067440:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +05c
        bcc.w   .L06744e                        | +060
        lea     ShieldSoldier_Fall_067220(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L06744e:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +06a

| ----------------------------------------------------------------------------
|  ShieldSoldier_Rise_067452  @ $067452  (80 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Rise_067452, "ax", @progbits
        .global ShieldSoldier_Rise_067452
ShieldSoldier_Rise_067452:
        lea     0x2bf8a4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c806e.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067474(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067474:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067490                        | +034
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L067490:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L06749e                        | +042
        lea     ShieldSoldier_Fall_067220(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L06749e:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +04c

| ----------------------------------------------------------------------------
|  ShieldSoldier_Aim_0674a2  @ $0674A2  (80 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Aim_0674a2, "ax", @progbits
        .global ShieldSoldier_Aim_0674a2
ShieldSoldier_Aim_0674a2:
        lea     0x2bf9a8.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x82(a6)                     | +00c
        lea     0x2c80e4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L0674c4(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L0674c4:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0674e0                        | +034
        lea     ShieldSoldier_Attack_0674f2(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0674e0:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L0674ee                        | +042
        lea     ShieldSoldier_Fall_067220(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L0674ee:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +04c

| ----------------------------------------------------------------------------
|  ShieldSoldier_Attack_0674f2  @ $0674F2  (122 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Attack_0674f2, "ax", @progbits
        .global ShieldSoldier_Attack_0674f2
ShieldSoldier_Attack_0674f2:
        lea     0x2bfaac.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c8146.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067514(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067514:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L06754e                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L06754e                        | +03e
        lea     ShieldSoldier_Attack_0674f2(pc),a1 | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L06754e                        | +052
        lea     ShieldSoldier_Recover_06756c(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
        .global ShieldSoldier_Attack_0674f2__L06754e
ShieldSoldier_Attack_0674f2__L06754e:
.L06754e:
        jsr     0x283ca.l                       | +05c
        jsr     0x283d8.l                       | +062
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +068
        bcc.w   .L067568                        | +06c
        lea     ShieldSoldier_Fall_067220(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L067568:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +076

| ----------------------------------------------------------------------------
|  ShieldSoldier_Recover_06756c  @ $06756C  (120 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Recover_06756c, "ax", @progbits
        .global ShieldSoldier_Recover_06756c
ShieldSoldier_Recover_06756c:
        lea     0x2bfa2a.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c82a0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L06758e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06758e:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0675aa                        | +034
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0675aa:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L0675b8                        | +042
        lea     ShieldSoldier_Fall_067220(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L0675b8:
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +04c
        .global ShieldSoldier_Recover_06756c__L0675bc
ShieldSoldier_Recover_06756c__L0675bc:
.L0675bc:
        lea     0x2c834c.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        lea     ShieldSoldier_Tail_0675e4(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
        bra.w   ShieldSoldier_Tail_0675e4       | +062
        .global ShieldSoldier_Recover_06756c__L0675d2
ShieldSoldier_Recover_06756c__L0675d2:
.L0675d2:
        lea     0x2c82ec.l,a0                   | +066
        jsr     0x28cd4.l                       | +06c
        lea     ShieldSoldier_Tail_0675e4(pc),a1 | +072
        move.l  a1,(a6)                         | +076

| ----------------------------------------------------------------------------
|  ShieldSoldier_Tail_0675e4  @ $0675E4  (106 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Tail_0675e4, "ax", @progbits
        .global ShieldSoldier_Tail_0675e4
ShieldSoldier_Tail_0675e4:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +000
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +004
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +008
        jsr     0x28d70.l                       | +00c
        bcc.w   .L067600                        | +012
        lea     ShieldSoldier_Tmpl6C_0670d2__L0670fe(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
.L067600:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +01c
        bcc.w   .L06760e                        | +020
        lea     ShieldSoldier_Fall_067220(pc),a1 | +024
        move.l  a1,(a6)                         | +028
        .global ShieldSoldier_Tail_0675e4__L06760e
ShieldSoldier_Tail_0675e4__L06760e:
.L06760e:
        subq.w  #0x1,0x72(a6)                   | +02a
        jsr     0x2870a.l                       | +02e
        bcc.w   .L06762e                        | +034
        bclr    #0x3,0x13(a6)                   | +038
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
.L06762e:
        jsr     0x28758.l                       | +04a
        bcs.w   ShieldSoldier_Die_067692        | +050
        movea.l #0xffffffff,a0                  | +054
        lea     0x2c7ce6.l,a0                   | +05a
        jsr     0x5dd5c.l                       | +060
        bcc.w   SetHandlerRts_067654            | +066

| ----------------------------------------------------------------------------
|  ShieldSoldier_ShieldLost_067656  @ $067656  (54 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ShieldLost_067656, "ax", @progbits
        .global ShieldSoldier_ShieldLost_067656
ShieldSoldier_ShieldLost_067656:
        jsr     ShieldSoldier_DropItemB_068286(pc) | +000
        lea     0x29bfc4.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L06766c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06766c:
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L067680                        | +020
        lea     ShieldSoldier_ToFlee_06768c(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L067680:
        move.l  #0xffffffff,0x48(a6)            | +02a
        bra.w   ShieldSoldier_Tail_0675e4__L06760e | +032

| ----------------------------------------------------------------------------
|  ShieldSoldier_ToFlee_06768c  @ $06768C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ToFlee_06768c, "ax", @progbits
        .global ShieldSoldier_ToFlee_06768c
ShieldSoldier_ToFlee_06768c:
        jmp     0x58f82.l                       | +000

| ----------------------------------------------------------------------------
|  ShieldSoldier_Die_067692  @ $067692  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Die_067692, "ax", @progbits
        .global ShieldSoldier_Die_067692
ShieldSoldier_Die_067692:
        clr.b   0x20(a6)                        | +000
        bset    #0x3,0x13(a6)                   | +004
        jsr     ShieldSoldier_DropItem_068260(pc) | +00a
        tst.b   0x9a(a6)                        | +00e
        beq.w   JsrAbsThunk_0676b2              | +012
        lea     0x4abc0.l,a1                    | +016
        move.l  a1,(a6)                         | +01c
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  ShieldSoldier_Tmpl6D_0676ba  @ $0676BA  (190 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Tmpl6D_0676ba, "ax", @progbits
        .global ShieldSoldier_Tmpl6D_0676ba
ShieldSoldier_Tmpl6D_0676ba:
        jsr     ShieldSoldier_Setup_067e7e(pc)  | +000
        tst.b   0x99(a6)                        | +004
        bne.w   ShieldSoldierB_Run_0677c4       | +008
        .global ShieldSoldier_Tmpl6D_0676ba__L0676c6
ShieldSoldier_Tmpl6D_0676ba__L0676c6:
.L0676c6:
        lea     0x2c7d1e.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L0676d8(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L0676d8:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +01e
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +022
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +026
        jsr     0x28d70.l                       | +02a
        jsr     ShieldSoldier_GuardTick_068186(pc) | +030
        bcc.w   .L0676f8                        | +034
        lea     ShieldSoldierB_Guard_067780(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0676f8:
        jsr     ShieldSoldier_CanAttackB_068070(pc) | +03e
        bcc.w   .L067706                        | +042
        lea     ShieldSoldierB_Land_067846__L067886(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L067706:
        jsr     ShieldSoldier_CanAttackC_0680a4(pc) | +04c
        bcc.w   .L067714                        | +050
        lea     ShieldSoldierB_Rise_067944__L067994(pc),a1 | +054
        move.l  a1,(a6)                         | +058
.L067714:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +05a
        bcc.w   .L067722                        | +05e
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +062
        move.l  a1,(a6)                         | +066
.L067722:
        subq.w  #0x1,0x72(a6)                   | +068
        jsr     0x2870a.l                       | +06c
        bcc.w   .L067758                        | +072
        bclr    #0x3,0x13(a6)                   | +076
        lea     0x5e766.l,a0                    | +07c
        jsr     0x5e770.l                       | +082
        lea     ShieldSoldierB_Fall_067aae__L067ae2(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
        cmpi.b  #0x1,0x58(a6)                   | +08e
        beq.w   .L067758                        | +094
        lea     ShieldSoldierB_Fall_067aae__L067af8(pc),a1 | +098
        move.l  a1,(a6)                         | +09c
.L067758:
        jsr     0x28758.l                       | +09e
        bcs.w   ShieldSoldier_Die_067692        | +0a4
        movea.l #0xffffffff,a0                  | +0a8
        lea     0x2c7ce6.l,a0                   | +0ae
        jsr     0x5dd5c.l                       | +0b4
        bcc.w   SetHandlerRts_06777e            | +0ba

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Guard_067780  @ $067780  (68 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Guard_067780, "ax", @progbits
        .global ShieldSoldierB_Guard_067780
ShieldSoldierB_Guard_067780:
        clr.w   0x76(a6)                        | +000
        lea     0x2c7ee6.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L067796(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L067796:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +016
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +01a
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +01e
        jsr     0x28d70.l                       | +022
        bcc.w   .L0677b2                        | +028
        lea     ShieldSoldier_Tmpl6D_0676ba__L0676c6(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L0677b2:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +032
        bcc.w   .L0677c0                        | +036
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L0677c0:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +040

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Run_0677c4  @ $0677C4  (130 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Run_0677c4, "ax", @progbits
        .global ShieldSoldierB_Run_0677c4
ShieldSoldierB_Run_0677c4:
        jsr     ShieldSoldier_RngRunSpeed_067f86(pc) | +000
        move.l  #0x2c7a8a,0x48(a6)              | +004
        lea     0x2c7da4.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L0677e2(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L0677e2:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +01e
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +022
        jsr     ShieldSoldier_ScrollProbeB_067f62(pc) | +026
        jsr     0x28d70.l                       | +02a
        move.b  0x99(a6),d0                     | +030
        andi.w  #0xff,d0                        | +034
        lsl.w   #0x4,d0                         | +038
        btst    #0x0,0x3a(a6)                   | +03a
        bne.w   .L067814                        | +040
        cmp.w   0x22(a6),d0                     | +044
        bgt.w   .L06781c                        | +048
        bra.w   .L067822                        | +04c
.L067814:
        cmp.w   0x22(a6),d0                     | +050
        bgt.w   .L067822                        | +054
.L06781c:
        lea     ShieldSoldierB_Land_067846(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L067822:
        cmpi.l  #0x677e2,(a6)                   | +05e
        beq.w   .L067834                        | +064
        move.l  #0x2c79e2,0x48(a6)              | +068
.L067834:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +070
        bcc.w   .L067842                        | +074
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L067842:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +07e

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Land_067846  @ $067846  (144 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Land_067846, "ax", @progbits
        .global ShieldSoldierB_Land_067846
ShieldSoldierB_Land_067846:
        jsr     0x267e2.l                       | +000
        lea     0x2c7e2a.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06785e(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06785e:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L067874                        | +024
        lea     ShieldSoldier_Tmpl6D_0676ba__L0676c6(pc),a1 | +028
        move.l  a1,(a6)                         | +02c
.L067874:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +02e
        bcc.w   .L067882                        | +032
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L067882:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +03c
        .global ShieldSoldierB_Land_067846__L067886
ShieldSoldierB_Land_067846__L067886:
.L067886:
        lea     0x2bf822.l,a0                   | +040
        jsr     0x799de.l                       | +046
        move.b  d0,0x82(a6)                     | +04c
        lea     0x2c7fb0.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        lea     .L0678a8(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L0678a8:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +062
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +066
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +06a
        jsr     0x28d70.l                       | +06e
        bcc.w   .L0678c4                        | +074
        lea     ShieldSoldierB_CrouchHold_0678d6(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L0678c4:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +07e
        bcc.w   .L0678d2                        | +082
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +086
        move.l  a1,(a6)                         | +08a
.L0678d2:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +08c

| ----------------------------------------------------------------------------
|  ShieldSoldierB_CrouchHold_0678d6  @ $0678D6  (110 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_CrouchHold_0678d6, "ax", @progbits
        .global ShieldSoldierB_CrouchHold_0678d6
ShieldSoldierB_CrouchHold_0678d6:
        lea     0x2bf926.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c802a.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L0678f8(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L0678f8:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067932                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   ShieldSoldier_CrouchHold_0673e4__L067440 | +03e
        lea     ShieldSoldierB_CrouchHold_0678d6(pc),a1 | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L067932                        | +052
        lea     ShieldSoldierB_Rise_067944(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L067932:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +05c
        bcc.w   .L067940                        | +060
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L067940:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +06a

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Rise_067944  @ $067944  (160 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Rise_067944, "ax", @progbits
        .global ShieldSoldierB_Rise_067944
ShieldSoldierB_Rise_067944:
        lea     0x2bf8a4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c806e.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067966(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067966:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067982                        | +034
        lea     ShieldSoldier_Tmpl6D_0676ba__L0676c6(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L067982:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L067990                        | +042
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L067990:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +04c
        .global ShieldSoldierB_Rise_067944__L067994
ShieldSoldierB_Rise_067944__L067994:
.L067994:
        lea     0x2bf9a8.l,a0                   | +050
        jsr     0x799de.l                       | +056
        move.b  d0,0x82(a6)                     | +05c
        lea     0x2c80e4.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        lea     .L0679b6(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L0679b6:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +072
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +076
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +07a
        jsr     0x28d70.l                       | +07e
        bcc.w   .L0679d2                        | +084
        lea     ShieldSoldierB_Attack_0679e4(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L0679d2:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +08e
        bcc.w   .L0679e0                        | +092
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L0679e0:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +09c

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Attack_0679e4  @ $0679E4  (122 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Attack_0679e4, "ax", @progbits
        .global ShieldSoldierB_Attack_0679e4
ShieldSoldierB_Attack_0679e4:
        lea     0x2bfaac.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c8146.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067a06(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067a06:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067a40                        | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   ShieldSoldier_Attack_0674f2__L06754e | +03e
        lea     ShieldSoldierB_Attack_0679e4(pc),a1 | +042
        move.l  a1,(a6)                         | +046
        subq.b  #0x1,0x82(a6)                   | +048
        cmpi.b  #0x0,0x82(a6)                   | +04c
        bgt.w   .L067a40                        | +052
        lea     ShieldSoldierB_Recover_067a5e(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L067a40:
        jsr     0x283ca.l                       | +05c
        jsr     0x283d8.l                       | +062
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +068
        bcc.w   .L067a5a                        | +06c
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L067a5a:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +076

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Recover_067a5e  @ $067A5E  (80 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Recover_067a5e, "ax", @progbits
        .global ShieldSoldierB_Recover_067a5e
ShieldSoldierB_Recover_067a5e:
        lea     0x2bfa2a.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2c82a0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L067a80(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L067a80:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +022
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +026
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   .L067a9c                        | +034
        lea     ShieldSoldier_Tmpl6D_0676ba__L0676c6(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L067a9c:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +03e
        bcc.w   .L067aaa                        | +042
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L067aaa:
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +04c

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Fall_067aae  @ $067AAE  (92 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Fall_067aae, "ax", @progbits
        .global ShieldSoldierB_Fall_067aae
ShieldSoldierB_Fall_067aae:
        lea     0x2c7f54.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L067ac0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L067ac0:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +012
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +016
        jsr     0x27c8c.l                       | +01a
        bcc.w   .L067ad8                        | +020
        lea     ShieldSoldierB_Run_0677c4(pc),a1 | +024
        move.l  a1,(a6)                         | +028
.L067ad8:
        jsr     0x28d70.l                       | +02a
        bra.w   ShieldSoldierB_Tail_067b0a__L067b34 | +030
        .global ShieldSoldierB_Fall_067aae__L067ae2
ShieldSoldierB_Fall_067aae__L067ae2:
.L067ae2:
        lea     0x2c834c.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     ShieldSoldierB_Tail_067b0a(pc),a1 | +040
        move.l  a1,(a6)                         | +044
        bra.w   ShieldSoldierB_Tail_067b0a      | +046
        .global ShieldSoldierB_Fall_067aae__L067af8
ShieldSoldierB_Fall_067aae__L067af8:
.L067af8:
        lea     0x2c82ec.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     ShieldSoldierB_Tail_067b0a(pc),a1 | +056
        move.l  a1,(a6)                         | +05a

| ----------------------------------------------------------------------------
|  ShieldSoldierB_Tail_067b0a  @ $067B0A  (106 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldierB_Tail_067b0a, "ax", @progbits
        .global ShieldSoldierB_Tail_067b0a
ShieldSoldierB_Tail_067b0a:
        jsr     ShieldSoldier_TakeShieldHit_067fb8(pc) | +000
        jsr     ShieldSoldier_ShieldAlive_0681ee(pc) | +004
        jsr     ShieldSoldier_ScrollProbe_067f46(pc) | +008
        jsr     0x28d70.l                       | +00c
        bcc.w   .L067b26                        | +012
        lea     ShieldSoldier_Tmpl6D_0676ba__L0676c6(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
.L067b26:
        jsr     ShieldSoldier_FallCheck_0681c6(pc) | +01c
        bcc.w   .L067b34                        | +020
        lea     ShieldSoldierB_Fall_067aae(pc),a1 | +024
        move.l  a1,(a6)                         | +028
        .global ShieldSoldierB_Tail_067b0a__L067b34
ShieldSoldierB_Tail_067b0a__L067b34:
.L067b34:
        subq.w  #0x1,0x72(a6)                   | +02a
        jsr     0x2870a.l                       | +02e
        bcc.w   .L067b54                        | +034
        bclr    #0x3,0x13(a6)                   | +038
        lea     0x5e766.l,a0                    | +03e
        jsr     0x5e770.l                       | +044
.L067b54:
        jsr     0x28758.l                       | +04a
        bcs.w   ShieldSoldier_Die_067692        | +050
        movea.l #0xffffffff,a0                  | +054
        lea     0x2c7ce6.l,a0                   | +05a
        jsr     0x5dd5c.l                       | +060
        bcc.w   SetHandlerRts_067b7a            | +066

| ----------------------------------------------------------------------------
|  Shield_Init_067b92  @ $067B92  (226 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Init_067b92, "ax", @progbits
        .global Shield_Init_067b92
Shield_Init_067b92:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x9a(a0)                        | +004
        bne.w   .L067bb8                        | +008
        move.w  #0x38,d1                        | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x1a,0x1c(a6)                  | +016
        jsr     0x138fe.l                       | +01c
        bra.w   .L067bce                        | +022
.L067bb8:
        move.w  #0x1c7,d1                       | +026
        jsr     0x236e.l                        | +02a
        move.w  #0x1f,0x1c(a6)                  | +030
        jsr     0x138fe.l                       | +036
.L067bce:
        lea     0x2bf59a.l,a0                   | +03c
        jsr     0x799de.l                       | +042
        move.w  d0,0x66(a6)                     | +048
        move.l  #0x2c7a36,0x48(a6)              | +04c
        lea     .L067bec(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L067bec:
        jsr     0x5e506.l                       | +05a
        move.l  0x94(a0),0x94(a6)               | +060
        move.b  0x80(a0),d0                     | +066
        cmp.b   0x80(a6),d0                     | +06a
        beq.w   .L067c28                        | +06e
        move.b  d0,0x80(a6)                     | +072
        andi.w  #0x7,d0                         | +076
        movea.l #0x2c7cfe,a0                    | +07a
        lsl.w   #0x2,d0                         | +080
        movea.l (a0,d0.w),a0                    | +082
        cmpa.l  #0xffffffff,a0                  | +086
        beq.w   .L067c28                        | +08c
        jsr     0x28cd4.l                       | +090
.L067c28:
        jsr     0x28d70.l                       | +096
        jsr     0x2870a.l                       | +09c
        bcc.w   .L067c3e                        | +0a2
        bclr    #0x3,0x13(a6)                   | +0a6
.L067c3e:
        jsr     0x28758.l                       | +0ac
        bcc.w   .L067c58                        | +0b2
        movea.l 0xc(a6),a0                      | +0b6
        move.l  #0x67656,(a0)                   | +0ba
        lea     Shield_FlyOff_067c7c(pc),a1     | +0c0
        move.l  a1,(a6)                         | +0c4
.L067c58:
        movea.l 0xc(a6),a0                      | +0c6
        tst.b   0x20(a0)                        | +0ca
        bne.w   .L067c6a                        | +0ce
        lea     Shield_FlyOff_067c7c(pc),a1     | +0d2
        move.l  a1,(a6)                         | +0d6
.L067c6a:
        jsr     0x5e45a.l                       | +0d8
        bcc.w   SetHandlerRts_067c7a            | +0de

| ----------------------------------------------------------------------------
|  Shield_FlyOff_067c7c  @ $067C7C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_FlyOff_067c7c, "ax", @progbits
        .global Shield_FlyOff_067c7c
Shield_FlyOff_067c7c:
        addi.w  #0xfff8,0x22(a6)                | +000
        addi.w  #0x12,0x24(a6)                  | +006
        move.w  #0xcc,d0                        | +00c
        jsr     0x5dca4.l                       | +010
        move.w  d0,0x28(a6)                     | +016
        move.w  #0x438,0x2a(a6)                 | +01a
        move.w  #0xffdc,0x2e(a6)                | +020
        move.w  #0x0,0x2c(a6)                   | +026
        lea     0x2c836c.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L067cba(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L067cba:
        jsr     0x27c8c.l                       | +03e
        bcc.w   .L067cca                        | +044
        lea     Shield_Bounce_067ce8(pc),a1     | +048
        move.l  a1,(a6)                         | +04c
.L067cca:
        jsr     0x28d70.l                       | +04e
        .global Shield_FlyOff_067c7c__L067cd0
Shield_FlyOff_067c7c__L067cd0:
.L067cd0:
        movea.l #0xffffffff,a0                  | +054
        jsr     0x5dd56.l                       | +05a
        bcc.w   SetHandlerRts_067ce6            | +060

| ----------------------------------------------------------------------------
|  Shield_Bounce_067ce8  @ $067CE8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Bounce_067ce8, "ax", @progbits
        .global Shield_Bounce_067ce8
Shield_Bounce_067ce8:
        move.w  #0xcc,d0                        | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x65e,0x2a(a6)                 | +00e
        move.w  #0xff5d,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        lea     0x2c836c.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L067d1a(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L067d1a:
        jsr     0x27c8c.l                       | +032
        bcc.w   .L067d2a                        | +038
        lea     Shield_Blink_067d32(pc),a1      | +03c
        move.l  a1,(a6)                         | +040
.L067d2a:
        jsr     0x28d70.l                       | +042
        bra.b   Shield_FlyOff_067c7c__L067cd0   | +048

| ----------------------------------------------------------------------------
|  Shield_Blink_067d32  @ $067D32  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Blink_067d32, "ax", @progbits
        .global Shield_Blink_067d32
Shield_Blink_067d32:
        move.w  #0x1e,0x70(a6)                  | +000
        lea     .L067d3e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L067d3e:
        jsr     0x2783a.l                       | +00c
        btst    #0x0,0x106f28.l                 | +012
        beq.w   .L067d56                        | +01a
        jsr     0x28d70.l                       | +01e
.L067d56:
        subq.w  #0x1,0x70(a6)                   | +024
        cmpi.w  #0x0,0x70(a6)                   | +028
        bgt.w   .L067d6a                        | +02e
        lea     Jsr5B6ThenJmpScheduler_067b84(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L067d6a:
        bra.w   Shield_FlyOff_067c7c__L067cd0   | +038

| ----------------------------------------------------------------------------
|  Shield_Debris_067d6e  @ $067D6E  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Debris_067d6e, "ax", @progbits
        .global Shield_Debris_067d6e
Shield_Debris_067d6e:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xd000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x14,0x38(a6)                  | +016
        lea     0x2bf7a0.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        btst    #0x0,0x3a(a6)                   | +028
        bne.w   .L067da2                        | +02e
        neg.w   d0                              | +032
.L067da2:
        move.w  d0,0x28(a6)                     | +034
        move.w  #0x157,d1                       | +038
        jsr     0x236e.l                        | +03c
        move.w  #0x158,d1                       | +042
        jsr     0x236e.l                        | +046
        move.w  #0x159,d1                       | +04c
        jsr     0x236e.l                        | +050
        lea     0x2c83f2.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        lea     .L067dd6(pc),a1                 | +062
        move.l  a1,(a6)                         | +066
.L067dd6:
        jsr     0x27cee.l                       | +068
        bcc.w   .L067de6                        | +06e
        lea     Shield_Free_067e7a(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L067de6:
        jsr     0x28d70.l                       | +078
        jsr     0x283d8.l                       | +07e
        btst    #0x1,0x13(a6)                   | +084
        beq.w   .L067e02                        | +08a
        lea     Shield_Free_067e7a(pc),a1       | +08e
        move.l  a1,(a6)                         | +092
.L067e02:
        movea.l #0xffffffff,a0                  | +094
        jsr     0x5dd56.l                       | +09a
        bcc.w   SetHandlerRts_067e18            | +0a0

| ----------------------------------------------------------------------------
|  Shield_Smoke_067e1a  @ $067E1A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Smoke_067e1a, "ax", @progbits
        .global Shield_Smoke_067e1a
Shield_Smoke_067e1a:
        move.w  #0x2000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        move.w  #0x17a,d1                       | +016
        jsr     0x236e.l                        | +01a
        lea     0x2c8428.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L067e4c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L067e4c:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L067e62                        | +03e
        lea     Jsr5B6ThenJmpScheduler_067b84(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L067e62:
        movea.l #0xffffffff,a0                  | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   SetHandlerRts_067e78            | +054

| ----------------------------------------------------------------------------
|  Shield_Free_067e7a  @ $067E7A  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Free_067e7a, "ax", @progbits
        .global Shield_Free_067e7a
Shield_Free_067e7a:
        bra.w   Jsr5B6ThenJmpScheduler_067b84   | +000

| ----------------------------------------------------------------------------
|  ShieldSoldier_Setup_067e7e  @ $067E7E  (194 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_Setup_067e7e, "ax", @progbits
        .global ShieldSoldier_Setup_067e7e
ShieldSoldier_Setup_067e7e:
        clr.b   0x84(a6)                        | +000
        cmpi.w  #0x100,0x22(a6)                 | +004
        bgt.w   .L067e92                        | +00a
        eori.b  #0x1,0x3a(a6)                   | +00e
.L067e92:
        jsr     0x5e7c0.l                       | +014
        cmpi.b  #0x2,0x98(a6)                   | +01a
        bne.w   .L067eb0                        | +020
        jsr     0x5e9b6.l                       | +024
        andi.b  #0x1,d0                         | +02a
        move.b  d0,0x98(a6)                     | +02e
.L067eb0:
        tst.b   0x9a(a6)                        | +032
        bne.w   .L067ed2                        | +036
        move.w  #0x38,d1                        | +03a
        jsr     0x236e.l                        | +03e
        move.w  #0x1a,0x1c(a6)                  | +044
        jsr     0x138fe.l                       | +04a
        bra.w   .L067ee8                        | +050
.L067ed2:
        move.w  #0x1c7,d1                       | +054
        jsr     0x236e.l                        | +058
        move.w  #0x1f,0x1c(a6)                  | +05e
        jsr     0x138fe.l                       | +064
.L067ee8:
        move.w  #0x8000,d0                      | +06a
        jsr     0x28134.l                       | +06e
        andi.w  #0xffe3,0x38(a6)                | +074
        ori.w   #0x18,0x38(a6)                  | +07a
        clr.b   0x81(a6)                        | +080
        move.w  #0x1,0x66(a6)                   | +084
        move.l  #0x2c79e2,0x48(a6)              | +08a
        move.l  #0x2c7c82,0x60(a6)              | +092
        move.l  #0x2c7d74,0x94(a6)              | +09a
        lea     Shield_Init_067b92(pc),a1       | +0a2
        jsr     0x4ae.l                         | +0a6
        jsr     0x5dd02.l                       | +0ac
        move.l  a0,0x90(a6)                     | +0b2
        lea     0x2bf8a4.l,a0                   | +0b6
        jsr     0x799de.l                       | +0bc

| ----------------------------------------------------------------------------
|  ShieldSoldier_ScrollProbe_067f46  @ $067F46  (22 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ScrollProbe_067f46, "ax", @progbits
        .global ShieldSoldier_ScrollProbe_067f46
ShieldSoldier_ScrollProbe_067f46:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   ClearXN_067f5c                  | +00c
        jsr     0x27c8c.l                       | +010

| ----------------------------------------------------------------------------
|  ShieldSoldier_ScrollProbeB_067f62  @ $067F62  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ScrollProbeB_067f62, "ax", @progbits
        .global ShieldSoldier_ScrollProbeB_067f62
ShieldSoldier_ScrollProbeB_067f62:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   ShieldSoldier_ProbeRevert_067f7a | +00c

| ----------------------------------------------------------------------------
|  ShieldSoldier_ProbeRevert_067f7a  @ $067F7A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ProbeRevert_067f7a, "ax", @progbits
        .global ShieldSoldier_ProbeRevert_067f7a
ShieldSoldier_ProbeRevert_067f7a:
        jsr     0x27a92.l                       | +000

| ----------------------------------------------------------------------------
|  ShieldSoldier_RngRunSpeed_067f86  @ $067F86  (50 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_RngRunSpeed_067f86, "ax", @progbits
        .global ShieldSoldier_RngRunSpeed_067f86
ShieldSoldier_RngRunSpeed_067f86:
        lea     0x2bf61c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,d1                           | +00c
        btst    #0x0,0x3a(a6)                   | +00e
        bne.w   .L067fa0                        | +014
        neg.w   d0                              | +018
.L067fa0:
        move.w  d0,0x28(a6)                     | +01a
        lsr.w   #0x7,d1                         | +01e
        andi.w  #0xc,d1                         | +020
        lea     0x2c7d64.l,a0                   | +024
        move.l  (a0,d1.w),0x94(a6)              | +02a
        rts                                     | +030

| ----------------------------------------------------------------------------
|  ShieldSoldier_TakeShieldHit_067fb8  @ $067FB8  (40 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_TakeShieldHit_067fb8, "ax", @progbits
        .global ShieldSoldier_TakeShieldHit_067fb8
ShieldSoldier_TakeShieldHit_067fb8:
        tst.b   0x81(a6)                        | +000
        bne.w   ClearXN_067fe6                  | +004
        movea.l 0x90(a6),a0                     | +008
        btst    #0x0,0x5a(a0)                   | +00c
        beq.w   ClearXN_067fe6                  | +012
        bset    #0x3,0x13(a6)                   | +016
        bset    #0x0,0x5a(a6)                   | +01c
        move.b  0x58(a0),0x58(a6)               | +022

| ----------------------------------------------------------------------------
|  ShieldSoldier_AtScreenEdge_067fec  @ $067FEC  (40 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_AtScreenEdge_067fec, "ax", @progbits
        .global ShieldSoldier_AtScreenEdge_067fec
ShieldSoldier_AtScreenEdge_067fec:
        cmpi.w  #0x20,0x22(a6)                  | +000
        bgt.w   .L068000                        | +006
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   SetXN_06801a                    | +010
.L068000:
        cmpi.w  #0x120,0x22(a6)                 | +014
        blt.w   ClearXN_068014                  | +01a
        btst    #0x0,0x3a(a6)                   | +01e
        beq.w   SetXN_06801a                    | +024

| ----------------------------------------------------------------------------
|  ShieldSoldier_CanAttackA_068020  @ $068020  (68 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_CanAttackA_068020, "ax", @progbits
        .global ShieldSoldier_CanAttackA_068020
ShieldSoldier_CanAttackA_068020:
        cmpi.w  #0x128,0x22(a6)                 | +000
        bgt.w   ClearXN_06806a                  | +006
        cmpi.w  #0x18,0x22(a6)                  | +00a
        blt.w   ClearXN_06806a                  | +010
        .global ShieldSoldier_CanAttackA_068020__L068034
ShieldSoldier_CanAttackA_068020__L068034:
.L068034:
        cmpi.b  #0x0,0x98(a6)                   | +014
        bne.w   ClearXN_06806a                  | +01a
        cmpi.w  #0x0,0x72(a6)                   | +01e
        bgt.w   ClearXN_06806a                  | +024
        lea     0x2c7cce.l,a0                   | +028
        jsr     0x5e086.l                       | +02e
        bcs.w   ClearXN_06806a                  | +034
        jsr     ShieldSoldier_AbsDX_06824c(pc)  | +038
        cmp.w   0x74(a6),d0                     | +03c
        bgt.w   ClearXN_06806a                  | +040

| ----------------------------------------------------------------------------
|  ShieldSoldier_CanAttackB_068070  @ $068070  (46 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_CanAttackB_068070, "ax", @progbits
        .global ShieldSoldier_CanAttackB_068070
ShieldSoldier_CanAttackB_068070:
        cmpi.w  #0x128,0x22(a6)                 | +000
        bgt.b   ClearXN_06806a                  | +006
        cmpi.w  #0x18,0x22(a6)                  | +008
        blt.b   ClearXN_06806a                  | +00e
        cmpi.b  #0x0,0x98(a6)                   | +010
        bne.b   ClearXN_06806a                  | +016
        cmpi.w  #0x0,0x72(a6)                   | +018
        bgt.b   ClearXN_06806a                  | +01e
        lea     0x2c7cce.l,a0                   | +020
        jsr     0x5e086.l                       | +026
        bcs.b   ClearXN_06806a                  | +02c

| ----------------------------------------------------------------------------
|  ShieldSoldier_CanAttackC_0680a4  @ $0680A4  (56 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_CanAttackC_0680a4, "ax", @progbits
        .global ShieldSoldier_CanAttackC_0680a4
ShieldSoldier_CanAttackC_0680a4:
        cmpi.w  #0x128,0x22(a6)                 | +000
        bgt.w   ClearXN_0680e2                  | +006
        cmpi.w  #0x18,0x22(a6)                  | +00a
        blt.w   ClearXN_0680e2                  | +010
        .global ShieldSoldier_CanAttackC_0680a4__L0680b8
ShieldSoldier_CanAttackC_0680a4__L0680b8:
.L0680b8:
        cmpi.b  #0x1,0x98(a6)                   | +014
        bne.w   ClearXN_0680e2                  | +01a
        cmpi.w  #0x0,0x72(a6)                   | +01e
        bgt.w   ClearXN_0680e2                  | +024
        lea     0x2c7cd6.l,a0                   | +028
        jsr     0x5e086.l                       | +02e
        bcs.w   ClearXN_0680e2                  | +034

| ----------------------------------------------------------------------------
|  ShieldSoldier_PlayerFar_0680e8  @ $0680E8  (22 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_PlayerFar_0680e8, "ax", @progbits
        .global ShieldSoldier_PlayerFar_0680e8
ShieldSoldier_PlayerFar_0680e8:
        jsr     0x5e0d4.l                       | +000
        bcs.w   SetXN_068104                    | +006
        jsr     ShieldSoldier_AbsDX_06824c(pc)  | +00a
        cmp.w   0x74(a6),d0                     | +00e
        ble.w   SetXN_068104                    | +012

| ----------------------------------------------------------------------------
|  ShieldSoldier_PlayerNear_06810a  @ $06810A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_PlayerNear_06810a, "ax", @progbits
        .global ShieldSoldier_PlayerNear_06810a
ShieldSoldier_PlayerNear_06810a:
        jsr     0x5e0d4.l                       | +000
        bcs.w   ClearXN_068124                  | +006
        jsr     ShieldSoldier_AbsDX_06824c(pc)  | +00a
        subi.w  #0x10,d0                        | +00e
        cmp.w   0x74(a6),d0                     | +012
        bgt.w   SetXN_06812a                    | +016

| ----------------------------------------------------------------------------
|  ShieldSoldier_PlayerFarOrEdge_068130  @ $068130  (30 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_PlayerFarOrEdge_068130, "ax", @progbits
        .global ShieldSoldier_PlayerFarOrEdge_068130
ShieldSoldier_PlayerFarOrEdge_068130:
        jsr     0x5e0d4.l                       | +000
        bcs.w   SetXN_068154                    | +006
        jsr     ShieldSoldier_AbsDX_06824c(pc)  | +00a
        cmp.w   0x74(a6),d0                     | +00e
        bgt.w   SetXN_068154                    | +012
        jsr     ShieldSoldier_AtScreenEdge_067fec(pc) | +016
        bcs.w   SetXN_068154                    | +01a

| ----------------------------------------------------------------------------
|  ShieldSoldier_PlayerNearEdge_06815a  @ $06815A  (32 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_PlayerNearEdge_06815a, "ax", @progbits
        .global ShieldSoldier_PlayerNearEdge_06815a
ShieldSoldier_PlayerNearEdge_06815a:
        jsr     0x5e0d4.l                       | +000
        bcs.w   ClearXN_068180                  | +006
        jsr     ShieldSoldier_AtScreenEdge_067fec(pc) | +00a
        bcs.w   ClearXN_068180                  | +00e
        jsr     ShieldSoldier_AbsDX_06824c(pc)  | +012
        addq.w  #0x8,d0                         | +016
        cmp.w   0x74(a6),d0                     | +018
        bgt.w   ClearXN_068180                  | +01c

| ----------------------------------------------------------------------------
|  ShieldSoldier_GuardTick_068186  @ $068186  (52 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_GuardTick_068186, "ax", @progbits
        .global ShieldSoldier_GuardTick_068186
ShieldSoldier_GuardTick_068186:
        addq.w  #0x1,0x76(a6)                   | +000
        cmpi.w  #0xf,0x76(a6)                   | +004
        blt.w   ClearXN_0681c0                  | +00a
        move.w  #0xf,0x76(a6)                   | +00e
        lea     0x2c7cee.l,a0                   | +014
        jsr     0x5e086.l                       | +01a
        bcc.w   ClearXN_0681c0                  | +020
        lea     0x2c7cf6.l,a0                   | +024
        jsr     0x5e086.l                       | +02a
        bcs.w   ClearXN_0681c0                  | +030

| ----------------------------------------------------------------------------
|  ShieldSoldier_FallCheck_0681c6  @ $0681C6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_FallCheck_0681c6, "ax", @progbits
        .global ShieldSoldier_FallCheck_0681c6
ShieldSoldier_FallCheck_0681c6:
        jsr     0x27eba.l                       | +000
        bcc.w   ShieldSoldier_FallReset_0681e4  | +006
        addq.b  #0x1,0x83(a6)                   | +00a
        cmpi.b  #0x3,0x83(a6)                   | +00e
        blt.w   ShieldSoldier_FallReset_0681e4  | +014

| ----------------------------------------------------------------------------
|  ShieldSoldier_FallReset_0681e4  @ $0681E4  (4 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_FallReset_0681e4, "ax", @progbits
        .global ShieldSoldier_FallReset_0681e4
ShieldSoldier_FallReset_0681e4:
        clr.b   0x83(a6)                        | +000

| ----------------------------------------------------------------------------
|  ShieldSoldier_ShieldAlive_0681ee  @ $0681EE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_ShieldAlive_0681ee, "ax", @progbits
        .global ShieldSoldier_ShieldAlive_0681ee
ShieldSoldier_ShieldAlive_0681ee:
        movea.l 0x90(a6),a0                     | +000
        cmpi.l  #0xffffffff,0x48(a0)            | +004
        beq.w   JsrAbsRts_068204                | +00c

| ----------------------------------------------------------------------------
|  Shield_Break_068206  @ $068206  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Shield_Break_068206, "ax", @progbits
        .global Shield_Break_068206
Shield_Break_068206:
        move.w  #0x10ff,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     Shield_Smoke_067e1a(pc),a1      | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     Shield_Debris_067d6e(pc),a1     | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        addi.w  #0x1c,0x24(a0)                  | +02a
        move.w  #0xffe2,d0                      | +030
        btst    #0x0,0x3a(a6)                   | +034
        beq.w   .L068246                        | +03a
        neg.w   d0                              | +03e
.L068246:
        add.w   d0,0x22(a0)                     | +040
        rts                                     | +044

| ----------------------------------------------------------------------------
|  ShieldSoldier_AbsDX_06824c  @ $06824C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_AbsDX_06824c, "ax", @progbits
        .global ShieldSoldier_AbsDX_06824c
ShieldSoldier_AbsDX_06824c:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x22(a0),d0                     | +004
        cmpi.w  #0x0,d0                         | +008
        bge.w   .L06825e                        | +00c
        neg.w   d0                              | +010
.L06825e:
        rts                                     | +012

| ----------------------------------------------------------------------------
|  ShieldSoldier_DropItem_068260  @ $068260  (38 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_DropItem_068260, "ax", @progbits
        .global ShieldSoldier_DropItem_068260
ShieldSoldier_DropItem_068260:
        tst.b   0x84(a6)                        | +000
        bne.w   .L068284                        | +004
        move.b  0x9a(a6),d0                     | +008
        move.w  #0x9b,d1                        | +00c
        jsr     0x9a7aa.l                       | +010
        bcs.w   .L068284                        | +016
        addq.w  #0x4,0x24(a0)                   | +01a
        move.b  #0x1,0x84(a6)                   | +01e
.L068284:
        rts                                     | +024

| ----------------------------------------------------------------------------
|  ShieldSoldier_DropItemB_068286  @ $068286  (46 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_DropItemB_068286, "ax", @progbits
        .global ShieldSoldier_DropItemB_068286
ShieldSoldier_DropItemB_068286:
        tst.b   0x84(a6)                        | +000
        bne.w   .L0682b2                        | +004
        move.b  0x9a(a6),d0                     | +008
        move.w  #0x9b,d1                        | +00c
        jsr     0x9a7aa.l                       | +010
        bcs.w   .L0682b2                        | +016
        addi.w  #0x10,0x24(a0)                  | +01a
        move.w  #0x800,0x2a(a0)                 | +020
        move.b  #0x1,0x84(a6)                   | +026
.L0682b2:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  ShieldSoldier_HitCheck_0682b4  @ $0682B4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.ShieldSoldier_HitCheck_0682b4, "ax", @progbits
        .global ShieldSoldier_HitCheck_0682b4
ShieldSoldier_HitCheck_0682b4:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0682ca                    | +00c

| ----------------------------------------------------------------------------
|  S5Gate_Init_0682d0  @ $0682D0  (56 B)
| ----------------------------------------------------------------------------
        .section .text.S5Gate_Init_0682d0, "ax", @progbits
        .global S5Gate_Init_0682d0
S5Gate_Init_0682d0:
        lea     0x2ba596.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x76(a6)                     | +00c
        move.w  #0xec0,0x72(a6)                 | +010
        lea     .L0682ec(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0682ec:
        cmpi.w  #0xe20,0x106f50.l               | +01c
        blt.w   .L068300                        | +024
        move.b  #0x4,0x10e39c.l                 | +028
.L068300:
        jsr     S5Airship_CameraHook_0689e6(pc) | +030
        bcc.w   SetHandlerRts_06830e            | +034

| ----------------------------------------------------------------------------
|  S5Gate_WaitScroll_068310  @ $068310  (46 B)
| ----------------------------------------------------------------------------
        .section .text.S5Gate_WaitScroll_068310, "ax", @progbits
        .global S5Gate_WaitScroll_068310
S5Gate_WaitScroll_068310:
        move.b  #0x4,0x10e39c.l                 | +000
        move.w  #0x1000,0x72(a6)                | +008
        lea     .L068324(pc),a1                 | +00e
        move.l  a1,(a6)                         | +012
.L068324:
        cmpi.w  #0x1000,0x106f50.l              | +014
        blt.w   .L068336                        | +01c
        lea     S5Gate_Begin_068346(pc),a1      | +020
        move.l  a1,(a6)                         | +024
.L068336:
        jsr     Camera0_RelinkAndWrapScroll_06896A(pc) | +026
        bcc.w   SetHandlerRts_068344            | +02a

| ----------------------------------------------------------------------------
|  S5Gate_Begin_068346  @ $068346  (82 B)
| ----------------------------------------------------------------------------
        .section .text.S5Gate_Begin_068346, "ax", @progbits
        .global S5Gate_Begin_068346
S5Gate_Begin_068346:
        move.w  #0x1032,d0                      | +000
        jsr     0x2352.l                        | +004
        movea.l 0xc(a6),a0                      | +00a
        move.b  #0x1,0x21(a0)                   | +00e
        move.w  #0x1,0x106f5e.l                 | +014
        clr.l   0x106f60.l                      | +01c
        clr.l   0x106f64.l                      | +022
        move.w  #0x1000,0x72(a6)                | +028
        lea     .L06837a(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L06837a:
        addi.l  #0x8000,0x106f60.l              | +034
        cmpi.l  #0x60000,0x106f60.l             | +03e
        blt.w   JsrPcThunk_068398               | +048
        lea     S5Gate_HoldOffset_06839e(pc),a1 | +04c
        move.l  a1,(a6)                         | +050

| ----------------------------------------------------------------------------
|  S5Gate_HoldOffset_06839e  @ $06839E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.S5Gate_HoldOffset_06839e, "ax", @progbits
        .global S5Gate_HoldOffset_06839e
S5Gate_HoldOffset_06839e:
        move.l  #0x60000,0x106f60.l             | +000
        lea     JsrPcThunk_0683ae(pc),a1        | +00a
        move.l  a1,(a6)                         | +00e

| ----------------------------------------------------------------------------
|  S5Gate_Release_0683b4  @ $0683B4  (52 B)
| ----------------------------------------------------------------------------
        .section .text.S5Gate_Release_0683b4, "ax", @progbits
        .global S5Gate_Release_0683b4
S5Gate_Release_0683b4:
        lea     .L0683ba(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0683ba:
        subi.l  #0x2000,0x106f60.l              | +006
        cmpi.l  #0x0,0x106f60.l                 | +010
        bgt.w   JsrPcThunk_0683ea               | +01a
        clr.l   0x106f60.l                      | +01e
        clr.l   0x106f64.l                      | +024
        jsr     Camera0_RelinkAndWrapScroll_06896A(pc) | +02a
        jmp     0x518.l                         | +02e

| ----------------------------------------------------------------------------
|  Rts_0683e8  @ $0683E8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0683e8, "ax", @progbits
        .global Rts_0683e8
Rts_0683e8:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  S5Airship_Init_0683f0  @ $0683F0  (196 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Init_0683f0, "ax", @progbits
        .global S5Airship_Init_0683f0
S5Airship_Init_0683f0:
        move.w  #0xf00,d0                       | +000
        move.w  #0xf8,d1                        | +004
        jsr     0x440d0.l                       | +008
        move.w  d0,0x22(a6)                     | +00e
        move.w  d1,0x24(a6)                     | +012
        jsr     0x5e7c0.l                       | +016
        addi.w  #0x130,0x22(a6)                 | +01c
        lea     S5Airship_Nose_06875e(pc),a1    | +022
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c
        clr.w   0x80(a0)                        | +032
        lea     S5Airship_Part_0684b4(pc),a1    | +036
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd02.l                       | +040
        clr.w   0x80(a0)                        | +046
        lea     S5Airship_Part_0684b4(pc),a1    | +04a
        jsr     0x4ae.l                         | +04e
        jsr     0x5dd02.l                       | +054
        move.w  #0x70,0x80(a0)                  | +05a
        lea     S5Airship_Part_0684b4(pc),a1    | +060
        jsr     0x4ae.l                         | +064
        jsr     0x5dd02.l                       | +06a
        move.w  #0xe0,0x80(a0)                  | +070
        move.w  #0xc000,d0                      | +076
        jsr     0x28134.l                       | +07a
        andi.w  #0xffe3,0x38(a6)                | +080
        ori.w   #0x8,0x38(a6)                   | +086
        lea     .L068482(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L068482:
        jsr     S5Airship_ActiveFlag_068a5a(pc) | +092
        jsr     S5Airship_ShadowC_068b64(pc)    | +096
        tst.w   0x106f5e.l                      | +09a
        beq.w   .L0684a2                        | +0a0
        cmpi.w  #0x30,0x22(a6)                  | +0a4
        ble.w   .L0684a2                        | +0aa
        subq.w  #0x1,0x22(a6)                   | +0ae
.L0684a2:
        cmpi.w  #0x30,0x22(a6)                  | +0b2
        bgt.w   .L0684b2                        | +0b8
        move.w  #0x30,0x22(a6)                  | +0bc
.L0684b2:
        rts                                     | +0c2

| ----------------------------------------------------------------------------
|  S5Airship_Part_0684b4  @ $0684B4  (90 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Part_0684b4, "ax", @progbits
        .global S5Airship_Part_0684b4
S5Airship_Part_0684b4:
        jsr     S5Airship_FollowParent_068a70(pc) | +000
        lea     S5Airship_Light_0686ce(pc),a1   | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        lea     S5Airship_Hull_06872a(pc),a1    | +014
        jsr     0x4ae.l                         | +018
        jsr     0x5dd02.l                       | +01e
        lea     S5Airship_Hatch_0687b4(pc),a1   | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        move.w  #0x1b7,d1                       | +034
        jsr     0x236e.l                        | +038
        clr.w   0x82(a6)                        | +03e
        clr.w   0x84(a6)                        | +042
        bset    #0x6,0x12(a6)                   | +046
        move.w  #0x7fff,0x66(a6)                | +04c
        move.l  #0x2c8466,0x48(a6)              | +052

| ----------------------------------------------------------------------------
|  S5Airship_PartIdle_06850e  @ $06850E  (112 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PartIdle_06850e, "ax", @progbits
        .global S5Airship_PartIdle_06850e
S5Airship_PartIdle_06850e:
        clr.b   0x20(a6)                        | +000
        lea     0x2c8536.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L068524(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L068524:
        jsr     S5Airship_Jitter_068ad0(pc)     | +016
        jsr     S5Airship_FollowParent_068a70(pc) | +01a
        jsr     Stub_00068B46(pc)               | +01e
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +022
        movea.l 0xc(a6),a0                      | +026
        movea.l 0xc(a0),a0                      | +02a
        btst    #0x0,0x13(a0)                   | +02e
        bne.w   .L068570                        | +034
        jsr     S5Airship_IsActive_068af4(pc)   | +038
        bcc.w   .L068570                        | +03c
        move.b  0x106f28.l,d0                   | +040
        andi.b  #0x1f,d0                        | +046
        bne.w   .L068570                        | +04a
        jsr     0x5e9b6.l                       | +04e
        andi.w  #0x3,d0                         | +054
        bne.w   .L068570                        | +058
        lea     S5Airship_PartHold_06857e(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L068570:
        bclr    #0x3,0x13(a6)                   | +062
        bclr    #0x0,0x13(a6)                   | +068
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  S5Airship_PartHold_06857e  @ $06857E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PartHold_06857e, "ax", @progbits
        .global S5Airship_PartHold_06857e
S5Airship_PartHold_06857e:
        move.w  #0x96,0x70(a6)                  | +000
        lea     0x2c8542.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L068596(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L068596:
        jsr     S5Airship_Jitter_068ad0(pc)     | +018
        jsr     S5Airship_FollowParent_068a70(pc) | +01c
        jsr     Stub_00068B46(pc)               | +020
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +024
        subq.w  #0x1,0x70(a6)                   | +028
        cmpi.w  #0x0,0x70(a6)                   | +02c
        bgt.w   .L0685ba                        | +032
        lea     S5Airship_PartState1_0685d8(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L0685ba:
        bclr    #0x3,0x13(a6)                   | +03c
        bclr    #0x0,0x13(a6)                   | +042
        tst.b   0x106ed3.l                      | +048
        bne.w   SetHandlerRts_0685d6            | +04e

| ----------------------------------------------------------------------------
|  S5Airship_PartState1_0685d8  @ $0685D8  (66 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PartState1_0685d8, "ax", @progbits
        .global S5Airship_PartState1_0685d8
S5Airship_PartState1_0685d8:
        move.b  #0x1,0x20(a6)                   | +000
        lea     .L0685e4(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L0685e4:
        jsr     S5Airship_Jitter_068ad0(pc)     | +00c
        jsr     S5Airship_FollowParent_068a70(pc) | +010
        jsr     Stub_00068B46(pc)               | +014
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +018
        cmpi.b  #0x1,0x20(a6)                   | +01c
        beq.w   .L068604                        | +022
        lea     S5Airship_PartSway_068622(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L068604:
        bclr    #0x3,0x13(a6)                   | +02c
        bclr    #0x0,0x13(a6)                   | +032
        tst.b   0x106ed3.l                      | +038
        bne.w   SetHandlerRts_068620            | +03e

| ----------------------------------------------------------------------------
|  S5Airship_PartSway_068622  @ $068622  (90 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PartSway_068622, "ax", @progbits
        .global S5Airship_PartSway_068622
S5Airship_PartSway_068622:
        lea     0x2c85a2.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L068634(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L068634:
        jsr     S5Airship_Jitter_068ad0(pc)     | +012
        jsr     S5Airship_FollowParent_068a70(pc) | +016
        jsr     Stub_00068B46(pc)               | +01a
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +01e
        move.b  0x106f28.l,d0                   | +022
        andi.b  #0x1f,d0                        | +028
        bne.w   .L068666                        | +02c
        jsr     0x5e9b6.l                       | +030
        andi.w  #0x7,d0                         | +036
        bne.w   .L068666                        | +03a
        lea     S5Airship_PartState3_068684(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
.L068666:
        bclr    #0x3,0x13(a6)                   | +044
        bclr    #0x0,0x13(a6)                   | +04a
        tst.b   0x106ed3.l                      | +050
        bne.w   SetHandlerRts_068682            | +056

| ----------------------------------------------------------------------------
|  S5Airship_PartState3_068684  @ $068684  (66 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PartState3_068684, "ax", @progbits
        .global S5Airship_PartState3_068684
S5Airship_PartState3_068684:
        move.b  #0x3,0x20(a6)                   | +000
        lea     .L068690(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L068690:
        jsr     S5Airship_Jitter_068ad0(pc)     | +00c
        jsr     S5Airship_FollowParent_068a70(pc) | +010
        jsr     Stub_00068B46(pc)               | +014
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +018
        cmpi.b  #0x3,0x20(a6)                   | +01c
        beq.w   .L0686b0                        | +022
        lea     S5Airship_PartIdle_06850e(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L0686b0:
        bclr    #0x3,0x13(a6)                   | +02c
        bclr    #0x0,0x13(a6)                   | +032
        tst.b   0x106ed3.l                      | +038
        bne.w   SetHandlerRts_0686cc            | +03e

| ----------------------------------------------------------------------------
|  S5Airship_Light_0686ce  @ $0686CE  (24 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Light_0686ce, "ax", @progbits
        .global S5Airship_Light_0686ce
S5Airship_Light_0686ce:
        clr.w   0x82(a6)                        | +000
        clr.w   0x84(a6)                        | +004
        move.w  #0x1b7,d1                       | +008
        jsr     0x236e.l                        | +00c
        bset    #0x6,0x12(a6)                   | +012

| ----------------------------------------------------------------------------
|  S5Airship_LightBlink_0686e6  @ $0686E6  (62 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_LightBlink_0686e6, "ax", @progbits
        .global S5Airship_LightBlink_0686e6
S5Airship_LightBlink_0686e6:
        lea     0x2c8506.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0686f8(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0686f8:
        jsr     S5Airship_IsActive_068af4(pc)   | +012
        bcc.w   .L068720                        | +016
        lea     0x2c8512.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L068712(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L068712:
        jsr     S5Airship_IsActive_068af4(pc)   | +02c
        bcs.w   .L068720                        | +030
        lea     S5Airship_LightBlink_0686e6(pc),a1 | +034
        move.l  a1,(a6)                         | +038
.L068720:
        jsr     S5Airship_FollowGrandparent_068a90(pc) | +03a

| ----------------------------------------------------------------------------
|  S5Airship_Hull_06872a  @ $06872A  (46 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Hull_06872a, "ax", @progbits
        .global S5Airship_Hull_06872a
S5Airship_Hull_06872a:
        clr.w   0x82(a6)                        | +000
        clr.w   0x84(a6)                        | +004
        move.w  #0x1b7,d1                       | +008
        jsr     0x236e.l                        | +00c
        bset    #0x6,0x12(a6)                   | +012
        lea     0x2c84fa.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L068754(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L068754:
        jsr     S5Airship_FollowParent_068a70(pc) | +02a

| ----------------------------------------------------------------------------
|  S5Airship_Nose_06875e  @ $06875E  (80 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Nose_06875e, "ax", @progbits
        .global S5Airship_Nose_06875e
S5Airship_Nose_06875e:
        clr.w   0x82(a6)                        | +000
        clr.w   0x84(a6)                        | +004
        move.w  #0x1b7,d1                       | +008
        jsr     0x236e.l                        | +00c
        bset    #0x6,0x12(a6)                   | +012
        lea     0x2c84ee.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L068788(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L068788:
        cmpi.w  #0x180,0x22(a6)                 | +02a
        bgt.w   .L0687aa                        | +030
        jsr     0x2783a.l                       | +034
        lea     0x2c84c2.l,a1                   | +03a
        jsr     0x43fac.l                       | +040
        lea     .L0687aa(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L0687aa:
        jsr     S5Airship_FollowParent_068a70(pc) | +04c

| ----------------------------------------------------------------------------
|  S5Airship_Hatch_0687b4  @ $0687B4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Hatch_0687b4, "ax", @progbits
        .global S5Airship_Hatch_0687b4
S5Airship_Hatch_0687b4:
        clr.w   0x86(a6)                        | +000
        move.w  #0xffff,0x84(a6)                | +004
        move.w  #0x1b7,d1                       | +00a
        jsr     0x236e.l                        | +00e

| ----------------------------------------------------------------------------
|  S5Airship_HatchClosed_0687c8  @ $0687C8  (60 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HatchClosed_0687c8, "ax", @progbits
        .global S5Airship_HatchClosed_0687c8
S5Airship_HatchClosed_0687c8:
        move.w  #0x8,0x82(a6)                   | +000
        lea     0x2c85e0.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0687e0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0687e0:
        jsr     S5Airship_FollowParent_068a70(pc) | +018
        jsr     S5Airship_ShadowA_068b14(pc)    | +01c
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +020
        movea.l 0xc(a6),a0                      | +024
        cmpi.b  #0x1,0x20(a0)                   | +028
        bne.w   .L068800                        | +02e
        lea     S5Airship_HatchOpen_068804(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L068800:
        bra.w   S5Airship_SealCheck_06894a      | +038

| ----------------------------------------------------------------------------
|  S5Airship_HatchOpen_068804  @ $068804  (64 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HatchOpen_068804, "ax", @progbits
        .global S5Airship_HatchOpen_068804
S5Airship_HatchOpen_068804:
        move.w  #0x10a8,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2c85e0.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L068820(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L068820:
        jsr     S5Airship_FollowParent_068a70(pc) | +01c
        jsr     S5Airship_ShadowA_068b14(pc)    | +020
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +024
        addq.w  #0x1,0x82(a6)                   | +028
        cmpi.w  #0x28,0x82(a6)                  | +02c
        bne.w   .L068840                        | +032
        lea     S5Airship_HatchAnim_068844(pc),a1 | +036
        move.l  a1,(a6)                         | +03a
.L068840:
        bra.w   S5Airship_SealCheck_06894a      | +03c

| ----------------------------------------------------------------------------
|  S5Airship_HatchAnim_068844  @ $068844  (96 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HatchAnim_068844, "ax", @progbits
        .global S5Airship_HatchAnim_068844
S5Airship_HatchAnim_068844:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x2,0x20(a0)                   | +004
        subq.w  #0x1,0x82(a6)                   | +00a
        move.b  0x3b(a6),d0                     | +00e
        andi.w  #0xf,d0                         | +012
        move.w  d0,0x86(a6)                     | +016
        movea.l #0x2c8626,a0                    | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L06887a                        | +02c
        jsr     0x28cd4.l                       | +030
.L06887a:
        lea     .L068880(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L068880:
        jsr     S5Airship_FollowParent_068a70(pc) | +03c
        jsr     S5Airship_ShadowA_068b14(pc)    | +040
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +044
        movea.l 0xc(a6),a0                      | +048
        cmpi.b  #0x3,0x20(a0)                   | +04c
        bne.w   .L0688a0                        | +052
        lea     S5Airship_HatchClose_0688a4(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L0688a0:
        bra.w   S5Airship_SealCheck_06894a      | +05c

| ----------------------------------------------------------------------------
|  S5Airship_HatchClose_0688a4  @ $0688A4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HatchClose_0688a4, "ax", @progbits
        .global S5Airship_HatchClose_0688a4
S5Airship_HatchClose_0688a4:
        move.w  0x86(a6),d0                     | +000
        movea.l #0x2c8686,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L0688c4                        | +016
        jsr     0x28cd4.l                       | +01a
.L0688c4:
        lea     .L0688ca(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L0688ca:
        jsr     S5Airship_FollowParent_068a70(pc) | +026
        jsr     S5Airship_ShadowA_068b14(pc)    | +02a
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +02e
        subq.w  #0x1,0x82(a6)                   | +032
        cmpi.w  #0x8,0x82(a6)                   | +036
        bne.w   .L0688f4                        | +03c
        movea.l 0xc(a6),a0                      | +040
        move.b  #0x4,0x20(a0)                   | +044
        lea     S5Airship_HatchClosed_0687c8(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L0688f4:
        bra.w   S5Airship_SealCheck_06894a      | +050

| ----------------------------------------------------------------------------
|  S5Airship_HatchFinal_0688f8  @ $0688F8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HatchFinal_0688f8, "ax", @progbits
        .global S5Airship_HatchFinal_0688f8
S5Airship_HatchFinal_0688f8:
        move.w  0x86(a6),d0                     | +000
        movea.l #0x2c8686,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L068918                        | +016
        jsr     0x28cd4.l                       | +01a
.L068918:
        lea     .L06891e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06891e:
        jsr     S5Airship_FollowParent_068a70(pc) | +026
        cmpi.w  #0x0,0x82(a6)                   | +02a
        blt.w   .L068930                        | +030
        jsr     S5Airship_ShadowA_068b14(pc)    | +034
.L068930:
        jsr     S5Airship_PastRightEdge_068ab8(pc) | +038
        subq.w  #0x1,0x82(a6)                   | +03c
        cmpi.w  #0xfff8,0x82(a6)                | +040
        bgt.w   SetHandlerRts_068948            | +046

| ----------------------------------------------------------------------------
|  S5Airship_SealCheck_06894a  @ $06894A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_SealCheck_06894a, "ax", @progbits
        .global S5Airship_SealCheck_06894a
S5Airship_SealCheck_06894a:
        tst.b   0x106ed3.l                      | +000
        bne.w   SetHandlerRts_06895a            | +006

| ----------------------------------------------------------------------------
|  S5Airship_CameraHook_0689e6  @ $0689E6  (104 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_CameraHook_0689e6, "ax", @progbits
        .global S5Airship_CameraHook_0689e6
S5Airship_CameraHook_0689e6:
        move.w  0x72(a6),d0                     | +000
        subq.w  #0x8,d0                         | +004
        cmp.w   0x106f50.l,d0                   | +006
        bgt.w   ClearXN_068a54                  | +00c
        lea     0x106f6c.l,a0                   | +010
        lea     0x2c84ba.l,a1                   | +016
        lea     0x20a000.l,a2                   | +01c
        move.l  a2,0x78(a0)                     | +022
        move.w  0x106f50.l,d0                   | +026
        sub.w   0x72(a6),d0                     | +02c
        addi.w  #0xfec0,d0                      | +030
        move.w  0x106f54.l,d1                   | +034
        addi.w  #0x30,d1                        | +03a
        addi.w  #0xffc0,d1                      | +03e
        jsr     0x51b3e.l                       | +042
        lea     0x106f6c.l,a0                   | +048
        jsr     0x43df4.l                       | +04e
        lea     0x106f6c.l,a0                   | +054
        move.b  #0xf,d0                         | +05a
        move.b  #0x0,d1                         | +05e
        jsr     0x51b1c.l                       | +062

| ----------------------------------------------------------------------------
|  S5Airship_ActiveFlag_068a5a  @ $068A5A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_ActiveFlag_068a5a, "ax", @progbits
        .global S5Airship_ActiveFlag_068a5a
S5Airship_ActiveFlag_068a5a:
        jsr     S5Airship_IsActive_068af4(pc)   | +000
        bcc.w   JsrAbsThunk_068a68              | +004
        bset    #0x6,0x13(a6)                   | +008

| ----------------------------------------------------------------------------
|  S5Airship_FollowParent_068a70  @ $068A70  (32 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_FollowParent_068a70, "ax", @progbits
        .global S5Airship_FollowParent_068a70
S5Airship_FollowParent_068a70:
        jsr     0x5e506.l                       | +000
        move.w  0x80(a6),d0                     | +006
        add.w   d0,0x22(a6)                     | +00a
        move.w  0x82(a6),d0                     | +00e
        add.w   d0,0x24(a6)                     | +012
        move.w  0x84(a6),d0                     | +016
        add.w   d0,0x38(a6)                     | +01a
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  S5Airship_FollowGrandparent_068a90  @ $068A90  (40 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_FollowGrandparent_068a90, "ax", @progbits
        .global S5Airship_FollowGrandparent_068a90
S5Airship_FollowGrandparent_068a90:
        jsr     0x5e506.l                       | +000
        move.w  0x80(a6),d0                     | +006
        add.w   d0,0x22(a6)                     | +00a
        movea.l 0xc(a0),a0                      | +00e
        move.w  0x24(a0),d0                     | +012
        add.w   0x82(a6),d0                     | +016
        move.w  d0,0x24(a6)                     | +01a
        move.w  0x84(a6),d0                     | +01e
        add.w   d0,0x38(a6)                     | +022
        rts                                     | +026

| ----------------------------------------------------------------------------
|  S5Airship_PastRightEdge_068ab8  @ $068AB8  (10 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_PastRightEdge_068ab8, "ax", @progbits
        .global S5Airship_PastRightEdge_068ab8
S5Airship_PastRightEdge_068ab8:
        cmpi.w  #0x1a0,0x22(a6)                 | +000
        bgt.w   ClearXN_068aca                  | +006

| ----------------------------------------------------------------------------
|  S5Airship_Jitter_068ad0  @ $068AD0  (36 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_Jitter_068ad0, "ax", @progbits
        .global S5Airship_Jitter_068ad0
S5Airship_Jitter_068ad0:
        jsr     S5Airship_IsActive_068af4(pc)   | +000
        bcs.w   .L068ada                        | +004
        rts                                     | +008
.L068ada:
        jsr     0x5e9b6.l                       | +00a
        move.w  #0xffff,d1                      | +010
        andi.w  #0x3,d0                         | +014
        bne.w   .L068aee                        | +018
        clr.w   d1                              | +01c
.L068aee:
        move.w  d1,0x82(a6)                     | +01e
        rts                                     | +022

| ----------------------------------------------------------------------------
|  S5Airship_IsActive_068af4  @ $068AF4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_IsActive_068af4, "ax", @progbits
        .global S5Airship_IsActive_068af4
S5Airship_IsActive_068af4:
        tst.w   0x106f5e.l                      | +000
        beq.w   ClearXN_068b0e                  | +006
        tst.l   0x106f60.l                      | +00a
        beq.w   ClearXN_068b0e                  | +010

| ----------------------------------------------------------------------------
|  S5Airship_ShadowA_068b14  @ $068B14  (42 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_ShadowA_068b14, "ax", @progbits
        .global S5Airship_ShadowA_068b14
S5Airship_ShadowA_068b14:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x8,0x38(a6)                   | +010
        move.w  0x22(a6),d0                     | +016
        move.w  0x24(a6),d1                     | +01a
        subi.w  #0x20,d0                        | +01e
        addi.w  #0x1f,d1                        | +022
        move.w  #0x40,d2                        | +026

| ----------------------------------------------------------------------------
|  S5Airship_ShadowB_068b48  @ $068B48  (20 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_ShadowB_068b48, "ax", @progbits
        .global S5Airship_ShadowB_068b48
S5Airship_ShadowB_068b48:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        subi.w  #0x38,d0                        | +008
        addi.w  #0x1f,d1                        | +00c
        move.w  #0x70,d2                        | +010

| ----------------------------------------------------------------------------
|  S5Airship_ShadowC_068b64  @ $068B64  (54 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_ShadowC_068b64, "ax", @progbits
        .global S5Airship_ShadowC_068b64
S5Airship_ShadowC_068b64:
        jsr     S5Airship_IsActive_068af4(pc)   | +000
        bcs.w   .L068b72                        | +004
        clr.w   d1                              | +008
        bra.w   .L068b86                        | +00a
.L068b72:
        jsr     0x5e9b6.l                       | +00e
        move.w  #0xffff,d1                      | +014
        andi.w  #0x3,d0                         | +018
        bne.w   .L068b86                        | +01c
        clr.w   d1                              | +020
.L068b86:
        add.w   0x24(a6),d1                     | +022
        addi.w  #0x1e,d1                        | +026
        move.w  0x22(a6),d0                     | +02a
        subi.w  #0x38,d0                        | +02e
        move.w  #0x140,d2                       | +032

| ----------------------------------------------------------------------------
|  S5Airship_DropPow_068ba2  @ $068BA2  (96 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_DropPow_068ba2, "ax", @progbits
        .global S5Airship_DropPow_068ba2
S5Airship_DropPow_068ba2:
        cmpi.w  #0x0,0x76(a6)                   | +000
        blt.w   .L068c00                        | +006
        lea     0x2ba618.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x78(a6)                     | +016
        jsr     0x5e9b6.l                       | +01a
        andi.w  #0xff,d0                        | +020
        cmp.w   0x78(a6),d0                     | +024
        bgt.w   .L068c00                        | +028
        lea     0x3fec6.l,a1                    | +02c
        jsr     0x4ae.l                         | +032
        jsr     0x5dd02.l                       | +038
        move.b  0x1.w,0x98(a0)                  | +03e
        clr.b   0x99(a0)                        | +044
        move.b  0x1.w,0x9a(a0)                  | +048
        move.w  #0x140,0x22(a0)                 | +04e
        move.w  #0x140,0x24(a0)                 | +054
        subq.w  #0x1,0x76(a6)                   | +05a
.L068c00:
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  S5Airship_HitCheck_068c02  @ $068C02  (16 B)
| ----------------------------------------------------------------------------
        .section .text.S5Airship_HitCheck_068c02, "ax", @progbits
        .global S5Airship_HitCheck_068c02
S5Airship_HitCheck_068c02:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_068c18                    | +00c

| ----------------------------------------------------------------------------
|  Tank_Tmpl35_068c1e  @ $068C1E  (302 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Tmpl35_068c1e, "ax", @progbits
        .global Tank_Tmpl35_068c1e
Tank_Tmpl35_068c1e:
        clr.b   0x7d(a6)                        | +000
        clr.b   0x7e(a6)                        | +004
        bra.w   .L068c42                        | +008
        clr.b   0x7d(a6)                        | +00c
        move.b  #0x1,0x7e(a6)                   | +010
        bra.w   .L068c42                        | +016
        move.b  #0x1,0x7d(a6)                   | +01a
        clr.b   0x7e(a6)                        | +020
.L068c42:
        jsr     Tank_EngineSound_069d5e(pc)     | +024
        jsr     0x5e7c0.l                       | +028
        jsr     0x267e2.l                       | +02e
        move.w  #0x3,0x1c(a6)                   | +034
        jsr     0x138fe.l                       | +03a
        lea     0x2b7f76.l,a0                   | +040
        jsr     0x799de.l                       | +046
        move.w  d0,0x76(a6)                     | +04c
        lea     0x2b7ff8.l,a0                   | +050
        jsr     0x799de.l                       | +056
        move.w  d0,0x78(a6)                     | +05c
        move.w  #0x8000,d0                      | +060
        jsr     0x28134.l                       | +064
        andi.w  #0xffe3,0x38(a6)                | +06a
        ori.w   #0x14,0x38(a6)                  | +070
        move.w  #0x64,0x66(a6)                  | +076
        move.l  #0x2c86e0,0x48(a6)              | +07c
        move.l  #0x2c8b18,0x60(a6)              | +084
        clr.b   0x7a(a6)                        | +08c
        lea     Tank_Driver_069880(pc),a1       | +090
        jsr     0x4ae.l                         | +094
        jsr     0x5dd02.l                       | +09a
        move.l  a0,0x88(a6)                     | +0a0
        move.b  0x98(a6),0x98(a0)               | +0a4
        tst.b   0x99(a6)                        | +0aa
        bne.w   .L068cea                        | +0ae
        lea     Tank_Turret_069a32(pc),a1       | +0b2
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd02.l                       | +0bc
        move.b  0x9b(a6),0x9b(a0)               | +0c2
        move.l  a0,0x8c(a6)                     | +0c8
.L068cea:
        move.w  #0x0,d0                         | +0cc
        move.w  #0xe0,d1                        | +0d0
        cmpi.w  #0x100,0x22(a6)                 | +0d4
        bgt.w   .L068d04                        | +0da
        move.w  #0x1,d0                         | +0de
        move.w  #0x60,d1                        | +0e2
.L068d04:
        move.b  d0,0x3a(a6)                     | +0e6
        move.b  d0,0x70(a6)                     | +0ea
        move.w  d1,0x72(a6)                     | +0ee
        lea     0x723d2.l,a1                    | +0f2
        jsr     0x4ae.l                         | +0f8
        jsr     0x5dd02.l                       | +0fe
        clr.w   0x98(a0)                        | +104
        lea     0x7773e.l,a1                    | +108
        jsr     0x4ae.l                         | +10e
        jsr     0x5dd02.l                       | +114
        move.l  #0x2c89cc,0x4c(a6)              | +11a
        tst.b   0x9a(a6)                        | +122
        beq.w   Tank_Drive_068ddc               | +126
        bra.w   Tank_Idle_068f3a                | +12a

| ----------------------------------------------------------------------------
|  Tank_Fall_068d4c  @ $068D4C  (144 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Fall_068d4c, "ax", @progbits
        .global Tank_Fall_068d4c
Tank_Fall_068d4c:
        jsr     0x267e2.l                       | +000
        cmpi.l  #0xffffffff,0x8c(a6)            | +006
        beq.w   .L068d84                        | +00e
        move.w  #0x0,d0                         | +012
        cmpi.w  #0xe,d0                         | +016
        ble.w   .L068d76                        | +01a
        nop                                     | +01e
        nop                                     | +020
        cmpi.w  #0xe,d0                         | +022
        nop                                     | +026
        trap    #0xf                            | +028
.L068d76:
        movea.l 0x8c(a6),a1                     | +02a
        lea     0x2c8bc2.l,a0                   | +02e
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +034
.L068d84:
        move.w  #0x0,d0                         | +038
        cmpi.w  #0xa,d0                         | +03c
        ble.w   .L068d9c                        | +040
        nop                                     | +044
        nop                                     | +046
        cmpi.w  #0xa,d0                         | +048
        nop                                     | +04c
        trap    #0xf                            | +04e
.L068d9c:
        movea.l 0x88(a6),a1                     | +050
        lea     0x2c8b96.l,a0                   | +054
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +05a
        lea     0x2c8cce.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L068dbc(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L068dbc:
        jsr     0x28998.l                       | +070
        jsr     0x27c8c.l                       | +076
        bcc.w   .L068dd2                        | +07c
        lea     Tank_Idle_068f3a(pc),a1         | +080
        move.l  a1,(a6)                         | +084
.L068dd2:
        jsr     0x28d70.l                       | +086
        bra.w   Tank_Tail_0697da__L0697ea       | +08c

| ----------------------------------------------------------------------------
|  Tank_Drive_068ddc  @ $068DDC  (350 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Drive_068ddc, "ax", @progbits
        .global Tank_Drive_068ddc
Tank_Drive_068ddc:
        tst.b   0x9a(a6)                        | +000
        bne.w   Tank_Idle_068f3a                | +004
        move.w  #0x109e,d0                      | +008
        jsr     0x2352.l                        | +00c
        jsr     0x267e2.l                       | +012
        move.w  #0x300,d0                       | +018
        jsr     Tank_MirrorByDir_069dd4(pc)     | +01c
        move.w  d0,0x28(a6)                     | +020
        clr.w   0x96(a6)                        | +024
        jsr     Tank_FacingMatchesDir_069ddc(pc) | +028
        bcs.w   .L068e7c                        | +02c
        cmpi.l  #0xffffffff,0x8c(a6)            | +030
        beq.w   .L068e3e                        | +038
        move.w  #0x1,d0                         | +03c
        cmpi.w  #0xe,d0                         | +040
        ble.w   .L068e30                        | +044
        nop                                     | +048
        nop                                     | +04a
        cmpi.w  #0xe,d0                         | +04c
        nop                                     | +050
        trap    #0xf                            | +052
.L068e30:
        movea.l 0x8c(a6),a1                     | +054
        lea     0x2c8bc2.l,a0                   | +058
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +05e
.L068e3e:
        move.w  #0x1,d0                         | +062
        cmpi.w  #0xa,d0                         | +066
        ble.w   .L068e56                        | +06a
        nop                                     | +06e
        nop                                     | +070
        cmpi.w  #0xa,d0                         | +072
        nop                                     | +076
        trap    #0xf                            | +078
.L068e56:
        movea.l 0x88(a6),a1                     | +07a
        lea     0x2c8b96.l,a0                   | +07e
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +084
        lea     0x2c8d6a.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
        move.l  #0x2c8e8a,0x90(a6)              | +094
        bra.w   .L068ee8                        | +09c
.L068e7c:
        cmpi.l  #0xffffffff,0x8c(a6)            | +0a0
        beq.w   .L068eae                        | +0a8
        move.w  #0x2,d0                         | +0ac
        cmpi.w  #0xe,d0                         | +0b0
        ble.w   .L068ea0                        | +0b4
        nop                                     | +0b8
        nop                                     | +0ba
        cmpi.w  #0xe,d0                         | +0bc
        nop                                     | +0c0
        trap    #0xf                            | +0c2
.L068ea0:
        movea.l 0x8c(a6),a1                     | +0c4
        lea     0x2c8bc2.l,a0                   | +0c8
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0ce
.L068eae:
        move.w  #0x1,d0                         | +0d2
        cmpi.w  #0xa,d0                         | +0d6
        ble.w   .L068ec6                        | +0da
        nop                                     | +0de
        nop                                     | +0e0
        cmpi.w  #0xa,d0                         | +0e2
        nop                                     | +0e6
        trap    #0xf                            | +0e8
.L068ec6:
        movea.l 0x88(a6),a1                     | +0ea
        lea     0x2c8b96.l,a0                   | +0ee
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0f4
        lea     0x2c8eba.l,a0                   | +0f8
        jsr     0x28cd4.l                       | +0fe
        move.l  #0x2c8f80,0x90(a6)              | +104
.L068ee8:
        jsr     Tank_AnimFrame_069e36(pc)       | +10c
        lea     .L068ef2(pc),a1                 | +110
        move.l  a1,(a6)                         | +114
.L068ef2:
        jsr     0x28998.l                       | +116
        jsr     0x27afc.l                       | +11c
        bcc.w   .L068f08                        | +122
        lea     Tank_Brake_06900a(pc),a1        | +126
        move.l  a1,(a6)                         | +12a
.L068f08:
        jsr     0x28d70.l                       | +12c
        jsr     Tank_PastRangeX_069df4(pc)      | +132
        bcc.w   .L068f1c                        | +136
        lea     Tank_Turn_068fe4(pc),a1         | +13a
        move.l  a1,(a6)                         | +13e
.L068f1c:
        jsr     Tank_CooldownTick_069e14(pc)    | +140
        bcc.w   .L068f2a                        | +144
        lea     Tank_Brake_06900a(pc),a1        | +148
        move.l  a1,(a6)                         | +14c
.L068f2a:
        jsr     0x283ca.l                       | +14e
        jsr     0x283d8.l                       | +154
        bra.w   Tank_Tail_0697da                | +15a

| ----------------------------------------------------------------------------
|  Tank_Idle_068f3a  @ $068F3A  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Idle_068f3a, "ax", @progbits
        .global Tank_Idle_068f3a
Tank_Idle_068f3a:
        move.w  #0xf,0x74(a6)                   | +000
        cmpi.l  #0xffffffff,0x8c(a6)            | +006
        beq.w   .L068f72                        | +00e
        move.w  #0x0,d0                         | +012
        cmpi.w  #0xe,d0                         | +016
        ble.w   .L068f64                        | +01a
        nop                                     | +01e
        nop                                     | +020
        cmpi.w  #0xe,d0                         | +022
        nop                                     | +026
        trap    #0xf                            | +028
.L068f64:
        movea.l 0x8c(a6),a1                     | +02a
        lea     0x2c8bc2.l,a0                   | +02e
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +034
.L068f72:
        move.w  #0x0,d0                         | +038
        cmpi.w  #0xa,d0                         | +03c
        ble.w   .L068f8a                        | +040
        nop                                     | +044
        nop                                     | +046
        cmpi.w  #0xa,d0                         | +048
        nop                                     | +04c
        trap    #0xf                            | +04e
.L068f8a:
        movea.l 0x88(a6),a1                     | +050
        lea     0x2c8b96.l,a0                   | +054
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +05a
        lea     0x2c8cce.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L068faa(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L068faa:
        jsr     0x28998.l                       | +070
        jsr     0x2783a.l                       | +076
        jsr     0x28d70.l                       | +07c
        tst.b   0x9a(a6)                        | +082
        bne.w   .L068fd2                        | +086
        subq.w  #0x1,0x74(a6)                   | +08a
        bne.w   .L068fd2                        | +08e
        lea     Tank_Resume_0690ea(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L068fd2:
        jsr     Tank_CooldownTick_069e14(pc)    | +098
        bcc.w   .L068fe0                        | +09c
        lea     Tank_AimPlayer_069190(pc),a1    | +0a0
        move.l  a1,(a6)                         | +0a4
.L068fe0:
        bra.w   Tank_Tail_0697da                | +0a6

| ----------------------------------------------------------------------------
|  Tank_Turn_068fe4  @ $068FE4  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Turn_068fe4, "ax", @progbits
        .global Tank_Turn_068fe4
Tank_Turn_068fe4:
        jsr     Tank_FacingMatchesDir_069ddc(pc) | +000
        bcs.w   .L068ff8                        | +004
        move.l  #0x2c8e8a,0x90(a6)              | +008
        bra.w   .L069000                        | +010
.L068ff8:
        move.l  #0x2c8f80,0x90(a6)              | +014
.L069000:
        eori.b  #0x1,0x70(a6)                   | +01c
        bra.w   Tank_Brake_06900a__L069026      | +022

| ----------------------------------------------------------------------------
|  Tank_Brake_06900a  @ $06900A  (224 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Brake_06900a, "ax", @progbits
        .global Tank_Brake_06900a
Tank_Brake_06900a:
        jsr     Tank_FacingMatchesDir_069ddc(pc) | +000
        bcs.w   .L06901e                        | +004
        move.l  #0x2c8e8a,0x90(a6)              | +008
        bra.w   .L069026                        | +010
.L06901e:
        move.l  #0x2c8f80,0x90(a6)              | +014
        .global Tank_Brake_06900a__L069026
Tank_Brake_06900a__L069026:
.L069026:
        move.w  0x28(a6),d0                     | +01c
        asr.w   #0x4,d0                         | +020
        clr.w   0x96(a6)                        | +022
        neg.w   d0                              | +026
        move.w  d0,0x2c(a6)                     | +028
        move.w  #0x109e,d0                      | +02c
        jsr     0x2222.l                        | +030
        lea     .L069046(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L069046:
        jsr     0x28998.l                       | +03c
        jsr     0x27afc.l                       | +042
        cmpi.w  #0x0,0x2c(a6)                   | +048
        beq.w   .L0690ce                        | +04e
        cmpi.w  #0x0,0x28(a6)                   | +052
        bne.w   .L0690ce                        | +058
        clr.w   0x2c(a6)                        | +05c
        cmpi.l  #0xffffffff,0x8c(a6)            | +060
        beq.w   .L06909c                        | +068
        move.w  #0x3,d0                         | +06c
        cmpi.w  #0xe,d0                         | +070
        ble.w   .L06908e                        | +074
        nop                                     | +078
        nop                                     | +07a
        cmpi.w  #0xe,d0                         | +07c
        nop                                     | +080
        trap    #0xf                            | +082
.L06908e:
        movea.l 0x8c(a6),a1                     | +084
        lea     0x2c8bc2.l,a0                   | +088
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +08e
.L06909c:
        move.w  #0x2,d0                         | +092
        cmpi.w  #0xa,d0                         | +096
        ble.w   .L0690b4                        | +09a
        nop                                     | +09e
        nop                                     | +0a0
        cmpi.w  #0xa,d0                         | +0a2
        nop                                     | +0a6
        trap    #0xf                            | +0a8
.L0690b4:
        movea.l 0x88(a6),a1                     | +0aa
        lea     0x2c8b96.l,a0                   | +0ae
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0b4
        lea     0x2c8fb0.l,a0                   | +0b8
        jsr     0x28cd4.l                       | +0be
.L0690ce:
        jsr     Tank_AnimFrame_069e36(pc)       | +0c4
        jsr     0x28d70.l                       | +0c8
        bcc.w   .L0690e2                        | +0ce
        lea     Tank_Idle_068f3a(pc),a1         | +0d2
        move.l  a1,(a6)                         | +0d6
.L0690e2:
        jsr     Tank_CooldownTick_069e14(pc)    | +0d8
        bra.w   Tank_Tail_0697da                | +0dc

| ----------------------------------------------------------------------------
|  Tank_Resume_0690ea  @ $0690EA  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Resume_0690ea, "ax", @progbits
        .global Tank_Resume_0690ea
Tank_Resume_0690ea:
        tst.b   0x9a(a6)                        | +000
        bne.w   Tank_Idle_068f3a                | +004
        jsr     Tank_CooldownTick_069e14(pc)    | +008
        bcs.w   Tank_AimPlayer_069190           | +00c
        cmpi.l  #0xffffffff,0x8c(a6)            | +010
        beq.w   .L06912c                        | +018
        move.w  #0x4,d0                         | +01c
        cmpi.w  #0xe,d0                         | +020
        ble.w   .L06911e                        | +024
        nop                                     | +028
        nop                                     | +02a
        cmpi.w  #0xe,d0                         | +02c
        nop                                     | +030
        trap    #0xf                            | +032
.L06911e:
        movea.l 0x8c(a6),a1                     | +034
        lea     0x2c8bc2.l,a0                   | +038
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +03e
.L06912c:
        move.w  #0x3,d0                         | +042
        cmpi.w  #0xa,d0                         | +046
        ble.w   .L069144                        | +04a
        nop                                     | +04e
        nop                                     | +050
        cmpi.w  #0xa,d0                         | +052
        nop                                     | +056
        trap    #0xf                            | +058
.L069144:
        movea.l 0x88(a6),a1                     | +05a
        lea     0x2c8b96.l,a0                   | +05e
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +064
        lea     0x2c90f4.l,a0                   | +068
        jsr     0x28cd4.l                       | +06e
        lea     .L069164(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L069164:
        jsr     0x28998.l                       | +07a
        jsr     0x2783a.l                       | +080
        jsr     0x28d70.l                       | +086
        bcc.w   .L069180                        | +08c
        lea     Tank_Drive_068ddc(pc),a1        | +090
        move.l  a1,(a6)                         | +094
.L069180:
        jsr     0x283ca.l                       | +096
        jsr     0x283d8.l                       | +09c
        bra.w   Tank_Tail_0697da                | +0a2

| ----------------------------------------------------------------------------
|  Tank_AimPlayer_069190  @ $069190  (326 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_AimPlayer_069190, "ax", @progbits
        .global Tank_AimPlayer_069190
Tank_AimPlayer_069190:
        cmpi.b  #0x0,0x7a(a6)                   | +000
        bne.w   Tank_FireBurst_0692e6           | +006
        move.b  #0x1,d0                         | +00a
        bra.w   .L0691cc                        | +00e
        lea     0x2c8b74.l,a0                   | +012
        jsr     0x5e0d4.l                       | +018
        bcs.w   .L0691ca                        | +01e
        move.w  0x22(a6),d0                     | +022
        sub.w   0x22(a0),d0                     | +026
        cmpi.w  #0xa0,d0                        | +02a
        bgt.w   .L0691ca                        | +02e
        move.b  #0x1,d0                         | +032
        bra.w   .L0691cc                        | +036
.L0691ca:
        clr.b   d0                              | +03a
.L0691cc:
        move.b  d0,0x7b(a6)                     | +03c
        beq.w   .L06923c                        | +040
        cmpi.l  #0xffffffff,0x8c(a6)            | +044
        beq.w   .L069206                        | +04c
        move.w  #0x5,d0                         | +050
        cmpi.w  #0xe,d0                         | +054
        ble.w   .L0691f8                        | +058
        nop                                     | +05c
        nop                                     | +05e
        cmpi.w  #0xe,d0                         | +060
        nop                                     | +064
        trap    #0xf                            | +066
.L0691f8:
        movea.l 0x8c(a6),a1                     | +068
        lea     0x2c8bc2.l,a0                   | +06c
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +072
.L069206:
        move.w  #0x5,d0                         | +076
        cmpi.w  #0xa,d0                         | +07a
        ble.w   .L06921e                        | +07e
        nop                                     | +082
        nop                                     | +084
        cmpi.w  #0xa,d0                         | +086
        nop                                     | +08a
        trap    #0xf                            | +08c
.L06921e:
        movea.l 0x88(a6),a1                     | +08e
        lea     0x2c8b96.l,a0                   | +092
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +098
        lea     0x2c922a.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
        bra.w   .L0692a0                        | +0a8
.L06923c:
        cmpi.l  #0xffffffff,0x8c(a6)            | +0ac
        beq.w   .L06926e                        | +0b4
        move.w  #0x6,d0                         | +0b8
        cmpi.w  #0xe,d0                         | +0bc
        ble.w   .L069260                        | +0c0
        nop                                     | +0c4
        nop                                     | +0c6
        cmpi.w  #0xe,d0                         | +0c8
        nop                                     | +0cc
        trap    #0xf                            | +0ce
.L069260:
        movea.l 0x8c(a6),a1                     | +0d0
        lea     0x2c8bc2.l,a0                   | +0d4
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0da
.L06926e:
        move.w  #0x5,d0                         | +0de
        cmpi.w  #0xa,d0                         | +0e2
        ble.w   .L069286                        | +0e6
        nop                                     | +0ea
        nop                                     | +0ec
        cmpi.w  #0xa,d0                         | +0ee
        nop                                     | +0f2
        trap    #0xf                            | +0f4
.L069286:
        movea.l 0x88(a6),a1                     | +0f6
        lea     0x2c8b96.l,a0                   | +0fa
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +100
        lea     0x2c92d8.l,a0                   | +104
        jsr     0x28cd4.l                       | +10a
.L0692a0:
        move.w  #0x32,0x74(a6)                  | +110
        lea     .L0692ac(pc),a1                 | +116
        move.l  a1,(a6)                         | +11a
.L0692ac:
        jsr     0x28998.l                       | +11c
        jsr     0x2783a.l                       | +122
        jsr     0x28d70.l                       | +128
        subq.w  #0x1,0x74(a6)                   | +12e
        bne.w   .L0692cc                        | +132
        lea     Tank_RngReload_0692d6(pc),a1    | +136
        move.l  a1,(a6)                         | +13a
.L0692cc:
        bclr    #0x3,0x13(a6)                   | +13c
        bra.w   Tank_Tail_0697da                | +142

| ----------------------------------------------------------------------------
|  Tank_RngReload_0692d6  @ $0692D6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_RngReload_0692d6, "ax", @progbits
        .global Tank_RngReload_0692d6
Tank_RngReload_0692d6:
        lea     0x2b807a.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x7a(a6)                     | +00c

| ----------------------------------------------------------------------------
|  Tank_FireBurst_0692e6  @ $0692E6  (328 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_FireBurst_0692e6, "ax", @progbits
        .global Tank_FireBurst_0692e6
Tank_FireBurst_0692e6:
        lea     0x2b7ff8.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x78(a6)                     | +00c
        cmpi.b  #0x0,0x7b(a6)                   | +010
        bne.w   .L069368                        | +016
        cmpi.l  #0xffffffff,0x8c(a6)            | +01a
        beq.w   .L069332                        | +022
        move.w  #0x8,d0                         | +026
        cmpi.w  #0xe,d0                         | +02a
        ble.w   .L069324                        | +02e
        nop                                     | +032
        nop                                     | +034
        cmpi.w  #0xe,d0                         | +036
        nop                                     | +03a
        trap    #0xf                            | +03c
.L069324:
        movea.l 0x8c(a6),a1                     | +03e
        lea     0x2c8bc2.l,a0                   | +042
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +048
.L069332:
        move.w  #0x6,d0                         | +04c
        cmpi.w  #0xa,d0                         | +050
        ble.w   .L06934a                        | +054
        nop                                     | +058
        nop                                     | +05a
        cmpi.w  #0xa,d0                         | +05c
        nop                                     | +060
        trap    #0xf                            | +062
.L06934a:
        movea.l 0x88(a6),a1                     | +064
        lea     0x2c8b96.l,a0                   | +068
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +06e
        lea     0x2c94a8.l,a0                   | +072
        jsr     0x28cd4.l                       | +078
        bra.w   .L0693cc                        | +07e
.L069368:
        cmpi.l  #0xffffffff,0x8c(a6)            | +082
        beq.w   .L06939a                        | +08a
        move.w  #0x9,d0                         | +08e
        cmpi.w  #0xe,d0                         | +092
        ble.w   .L06938c                        | +096
        nop                                     | +09a
        nop                                     | +09c
        cmpi.w  #0xe,d0                         | +09e
        nop                                     | +0a2
        trap    #0xf                            | +0a4
.L06938c:
        movea.l 0x8c(a6),a1                     | +0a6
        lea     0x2c8bc2.l,a0                   | +0aa
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0b0
.L06939a:
        move.w  #0x6,d0                         | +0b4
        cmpi.w  #0xa,d0                         | +0b8
        ble.w   .L0693b2                        | +0bc
        nop                                     | +0c0
        nop                                     | +0c2
        cmpi.w  #0xa,d0                         | +0c4
        nop                                     | +0c8
        trap    #0xf                            | +0ca
.L0693b2:
        movea.l 0x88(a6),a1                     | +0cc
        lea     0x2c8b96.l,a0                   | +0d0
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0d6
        lea     0x2c961e.l,a0                   | +0da
        jsr     0x28cd4.l                       | +0e0
.L0693cc:
        lea     .L0693d2(pc),a1                 | +0e6
        move.l  a1,(a6)                         | +0ea
.L0693d2:
        jsr     0x28998.l                       | +0ec
        jsr     0x2783a.l                       | +0f2
        jsr     0x28d70.l                       | +0f8
        bcc.w   .L0693f8                        | +0fe
        cmpi.b  #0x0,0x7a(a6)                   | +102
        bgt.w   .L0693f8                        | +108
        lea     Tank_FireDone_06942e(pc),a1     | +10c
        move.l  a1,(a6)                         | +110
.L0693f8:
        cmpi.b  #0x0,0x7a(a6)                   | +112
        ble.w   .L069424                        | +118
        subq.w  #0x1,0x78(a6)                   | +11c
        cmpi.w  #0x0,0x78(a6)                   | +120
        bgt.w   .L069424                        | +126
        subq.b  #0x1,0x7a(a6)                   | +12a
        cmpi.b  #0x0,0x7a(a6)                   | +12e
        ble.w   .L069424                        | +134
        lea     Tank_FireBurst_0692e6(pc),a1    | +138
        move.l  a1,(a6)                         | +13c
.L069424:
        bclr    #0x3,0x13(a6)                   | +13e
        bra.w   Tank_Tail_0697da                | +144

| ----------------------------------------------------------------------------
|  Tank_FireDone_06942e  @ $06942E  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_FireDone_06942e, "ax", @progbits
        .global Tank_FireDone_06942e
Tank_FireDone_06942e:
        move.b  #0x0,0x7a(a6)                   | +000
        lea     0x2b7f76.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x76(a6)                     | +012
        cmpi.b  #0x0,0x7b(a6)                   | +016
        beq.w   Tank_Idle_068f3a                | +01c
        clr.b   0x7b(a6)                        | +020
        cmpi.l  #0xffffffff,0x8c(a6)            | +024
        beq.w   .L069484                        | +02c
        move.w  #0x7,d0                         | +030
        cmpi.w  #0xe,d0                         | +034
        ble.w   .L069476                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.w  #0xe,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L069476:
        movea.l 0x8c(a6),a1                     | +048
        lea     0x2c8bc2.l,a0                   | +04c
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +052
.L069484:
        move.w  #0x0,d0                         | +056
        cmpi.w  #0xa,d0                         | +05a
        ble.w   .L06949c                        | +05e
        nop                                     | +062
        nop                                     | +064
        cmpi.w  #0xa,d0                         | +066
        nop                                     | +06a
        trap    #0xf                            | +06c
.L06949c:
        movea.l 0x88(a6),a1                     | +06e
        lea     0x2c8b96.l,a0                   | +072
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +078
        lea     0x2c93fa.l,a0                   | +07c
        jsr     0x28cd4.l                       | +082
        lea     .L0694bc(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L0694bc:
        jsr     0x28998.l                       | +08e
        jsr     0x2783a.l                       | +094
        jsr     0x28d70.l                       | +09a
        bcc.w   .L0694d8                        | +0a0
        lea     Tank_Idle_068f3a(pc),a1         | +0a4
        move.l  a1,(a6)                         | +0a8
.L0694d8:
        bra.w   Tank_Tail_0697da                | +0aa

| ----------------------------------------------------------------------------
|  Tank_Die_0694dc  @ $0694DC  (444 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Die_0694dc, "ax", @progbits
        .global Tank_Die_0694dc
Tank_Die_0694dc:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0x20,d0                        | +012
        jsr     Tank_MirrorByFacing_069dbc(pc)  | +016
        add.w   d0,0x22(a0)                     | +01a
        cmpi.l  #0xffffffff,0x8c(a6)            | +01e
        beq.w   .L06952c                        | +026
        move.w  #0xd,d0                         | +02a
        cmpi.w  #0xe,d0                         | +02e
        ble.w   .L06951e                        | +032
        nop                                     | +036
        nop                                     | +038
        cmpi.w  #0xe,d0                         | +03a
        nop                                     | +03e
        trap    #0xf                            | +040
.L06951e:
        movea.l 0x8c(a6),a1                     | +042
        lea     0x2c8bc2.l,a0                   | +046
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +04c
.L06952c:
        move.w  #0x8,d0                         | +050
        cmpi.w  #0xa,d0                         | +054
        ble.w   .L069544                        | +058
        nop                                     | +05c
        nop                                     | +05e
        cmpi.w  #0xa,d0                         | +060
        nop                                     | +064
        trap    #0xf                            | +066
.L069544:
        movea.l 0x88(a6),a1                     | +068
        lea     0x2c8b96.l,a0                   | +06c
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +072
        lea     0x2c97ca.l,a0                   | +076
        jsr     0x28cd4.l                       | +07c
        bra.w   .L06962e                        | +082
        .global Tank_Die_0694dc__L069562
Tank_Die_0694dc__L069562:
.L069562:
        cmpi.l  #0xffffffff,0x8c(a6)            | +086
        beq.w   .L069594                        | +08e
        move.w  #0xa,d0                         | +092
        cmpi.w  #0xe,d0                         | +096
        ble.w   .L069586                        | +09a
        nop                                     | +09e
        nop                                     | +0a0
        cmpi.w  #0xe,d0                         | +0a2
        nop                                     | +0a6
        trap    #0xf                            | +0a8
.L069586:
        movea.l 0x8c(a6),a1                     | +0aa
        lea     0x2c8bc2.l,a0                   | +0ae
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0b4
.L069594:
        move.w  #0x8,d0                         | +0b8
        cmpi.w  #0xa,d0                         | +0bc
        ble.w   .L0695ac                        | +0c0
        nop                                     | +0c4
        nop                                     | +0c6
        cmpi.w  #0xa,d0                         | +0c8
        nop                                     | +0cc
        trap    #0xf                            | +0ce
.L0695ac:
        movea.l 0x88(a6),a1                     | +0d0
        lea     0x2c8b96.l,a0                   | +0d4
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0da
        lea     0x2c9710.l,a0                   | +0de
        jsr     0x28cd4.l                       | +0e4
        bra.w   .L06962e                        | +0ea
        .global Tank_Die_0694dc__L0695ca
Tank_Die_0694dc__L0695ca:
.L0695ca:
        cmpi.l  #0xffffffff,0x8c(a6)            | +0ee
        beq.w   .L0695fc                        | +0f6
        move.w  #0xb,d0                         | +0fa
        cmpi.w  #0xe,d0                         | +0fe
        ble.w   .L0695ee                        | +102
        nop                                     | +106
        nop                                     | +108
        cmpi.w  #0xe,d0                         | +10a
        nop                                     | +10e
        trap    #0xf                            | +110
.L0695ee:
        movea.l 0x8c(a6),a1                     | +112
        lea     0x2c8bc2.l,a0                   | +116
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +11c
.L0695fc:
        move.w  #0x8,d0                         | +120
        cmpi.w  #0xa,d0                         | +124
        ble.w   .L069614                        | +128
        nop                                     | +12c
        nop                                     | +12e
        cmpi.w  #0xa,d0                         | +130
        nop                                     | +134
        trap    #0xf                            | +136
.L069614:
        movea.l 0x88(a6),a1                     | +138
        lea     0x2c8b96.l,a0                   | +13c
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +142
        lea     0x2c97ca.l,a0                   | +146
        jsr     0x28cd4.l                       | +14c
.L06962e:
        bclr    #0x3,0x13(a6)                   | +152
        move.w  #0x109e,d0                      | +158
        jsr     0x2222.l                        | +15c
        lea     .L069644(pc),a1                 | +162
        move.l  a1,(a6)                         | +166
.L069644:
        jsr     0x28998.l                       | +168
        jsr     0x2783a.l                       | +16e
        jsr     0x28d70.l                       | +174
        bcc.w   .L069660                        | +17a
        lea     Tank_Resume_0690ea(pc),a1       | +17e
        move.l  a1,(a6)                         | +182
.L069660:
        jsr     Tank_ItemFlag_069ee8(pc)        | +184
        jsr     Tank_CooldownTick_069e14(pc)    | +188
        jsr     0x27eba.l                       | +18c
        bcc.w   .L069678                        | +192
        lea     Tank_Fall_068d4c(pc),a1         | +196
        move.l  a1,(a6)                         | +19a
.L069678:
        jsr     0x2870a.l                       | +19c
        bcc.w   .L069694                        | +1a2
        bclr    #0x3,0x13(a6)                   | +1a6
        lea     0x5e766.l,a0                    | +1ac
        jsr     0x5e770.l                       | +1b2
.L069694:
        bra.w   Tank_Tail_0697da__L06982c       | +1b8

| ----------------------------------------------------------------------------
|  Tank_Wreck_069698  @ $069698  (302 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Wreck_069698, "ax", @progbits
        .global Tank_Wreck_069698
Tank_Wreck_069698:
        lea     Tank_DriverBail_0698aa(pc),a1   | +000
        move.l  0x88(a6),d0                     | +004
        jsr     Tank_SetHandlerIfValid_069da4(pc) | +008
        cmpi.l  #0xffffffff,0x8c(a6)            | +00c
        beq.w   .L0696d6                        | +014
        move.w  #0xc,d0                         | +018
        cmpi.w  #0xe,d0                         | +01c
        ble.w   .L0696c8                        | +020
        nop                                     | +024
        nop                                     | +026
        cmpi.w  #0xe,d0                         | +028
        nop                                     | +02c
        trap    #0xf                            | +02e
.L0696c8:
        movea.l 0x8c(a6),a1                     | +030
        lea     0x2c8bc2.l,a0                   | +034
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +03a
.L0696d6:
        move.w  #0xa,d0                         | +03e
        cmpi.w  #0xa,d0                         | +042
        ble.w   .L0696ee                        | +046
        nop                                     | +04a
        nop                                     | +04c
        cmpi.w  #0xa,d0                         | +04e
        nop                                     | +052
        trap    #0xf                            | +054
.L0696ee:
        movea.l 0x88(a6),a1                     | +056
        lea     0x2c8b96.l,a0                   | +05a
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +060
        lea     0x2c9938.l,a0                   | +064
        jsr     0x28cd4.l                       | +06a
        bra.w   .L06977c                        | +070
        .global Tank_Wreck_069698__L06970c
Tank_Wreck_069698__L06970c:
.L06970c:
        lea     Tank_DriverWreck_069a18(pc),a1  | +074
        move.l  0x88(a6),d0                     | +078
        jsr     Tank_SetHandlerIfValid_069da4(pc) | +07c
        cmpi.l  #0xffffffff,0x8c(a6)            | +080
        beq.w   .L06974a                        | +088
        move.w  #0xd,d0                         | +08c
        cmpi.w  #0xe,d0                         | +090
        ble.w   .L06973c                        | +094
        nop                                     | +098
        nop                                     | +09a
        cmpi.w  #0xe,d0                         | +09c
        nop                                     | +0a0
        trap    #0xf                            | +0a2
.L06973c:
        movea.l 0x8c(a6),a1                     | +0a4
        lea     0x2c8bc2.l,a0                   | +0a8
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0ae
.L06974a:
        move.w  #0xa,d0                         | +0b2
        cmpi.w  #0xa,d0                         | +0b6
        ble.w   .L069762                        | +0ba
        nop                                     | +0be
        nop                                     | +0c0
        cmpi.w  #0xa,d0                         | +0c2
        nop                                     | +0c6
        trap    #0xf                            | +0c8
.L069762:
        movea.l 0x88(a6),a1                     | +0ca
        lea     0x2c8b96.l,a0                   | +0ce
        jsr     Tank_SetSpriteByIndex_069d76(pc) | +0d4
        lea     0x2c99de.l,a0                   | +0d8
        jsr     0x28cd4.l                       | +0de
.L06977c:
        move.w  #0x0,0x94(a6)                   | +0e4
        bclr    #0x1,0x12(a6)                   | +0ea
        lea     .L06978e(pc),a1                 | +0f0
        move.l  a1,(a6)                         | +0f4
.L06978e:
        jsr     0x2783a.l                       | +0f6
        jsr     Tank_RecoilTick_069e90(pc)      | +0fc
        jsr     0x28d70.l                       | +100
        bcc.w   .L0697b0                        | +106
        tst.b   0x7d(a6)                        | +10a
        beq.w   .L0697b0                        | +10e
        lea     Tank_WreckBlast_0697ce(pc),a1   | +112
        move.l  a1,(a6)                         | +116
.L0697b0:
        movea.l #0xffffffff,a0                  | +118
        lea     0x2c8b6c.l,a0                   | +11e
        jsr     0x5dd5c.l                       | +124
        bcc.w   SetHandlerRts_0697cc            | +12a

| ----------------------------------------------------------------------------
|  Tank_WreckBlast_0697ce  @ $0697CE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_WreckBlast_0697ce, "ax", @progbits
        .global Tank_WreckBlast_0697ce
Tank_WreckBlast_0697ce:
        jsr     Tank_DeathBlast_069f12(pc)      | +000
        jmp     0x518.l                         | +004

| ----------------------------------------------------------------------------
|  Rts_0697d8  @ $0697D8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Rts_0697d8, "ax", @progbits
        .global Rts_0697d8
Rts_0697d8:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Tank_Tail_0697da  @ $0697DA  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Tail_0697da, "ax", @progbits
        .global Tank_Tail_0697da
Tank_Tail_0697da:
        jsr     0x27eba.l                       | +000
        bcc.w   .L0697ea                        | +006
        lea     Tank_Fall_068d4c(pc),a1         | +00a
        move.l  a1,(a6)                         | +00e
        .global Tank_Tail_0697da__L0697ea
Tank_Tail_0697da__L0697ea:
.L0697ea:
        jsr     Tank_ItemFlag_069ee8(pc)        | +010
        jsr     0x2870a.l                       | +014
        bcc.w   .L06982c                        | +01a
        lea     0x5e766.l,a0                    | +01e
        jsr     0x5e770.l                       | +024
        bclr    #0x3,0x13(a6)                   | +02a
        lea     0x5e766.l,a0                    | +030
        jsr     0x5e770.l                       | +036
        lea     Tank_Die_0694dc__L069562(pc),a1 | +03c
        move.l  a1,(a6)                         | +040
        jsr     0x5e844.l                       | +042
        bcc.w   .L06982c                        | +048
        lea     Tank_Die_0694dc__L0695ca(pc),a1 | +04c
        move.l  a1,(a6)                         | +050
        .global Tank_Tail_0697da__L06982c
Tank_Tail_0697da__L06982c:
.L06982c:
        jsr     0x28758.l                       | +052
        bcc.w   .L06984c                        | +058
        lea     Tank_Wreck_069698(pc),a1        | +05c
        move.l  a1,(a6)                         | +060
        jsr     0x5e844.l                       | +062
        bcc.w   .L06984c                        | +068
        lea     Tank_Wreck_069698__L06970c(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L06984c:
        movea.l #0xffffffff,a0                  | +072
        lea     0x2c8b6c.l,a0                   | +078
        jsr     0x5dd5c.l                       | +07e
        bcc.w   SetHandlerRts_069868            | +084

| ----------------------------------------------------------------------------
|  Tank_Driver_069880  @ $069880  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Driver_069880, "ax", @progbits
        .global Tank_Driver_069880
Tank_Driver_069880:
        move.w  #0x1d,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     .L069890(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L069890:
        jsr     Tank_CopyParentAnimPrio_069ebe(pc) | +010
        jsr     Tank_OffsetAbove_069ed4(pc)     | +014
        movea.l 0xc(a6),a0                      | +018
        move.l  0x5c(a0),0x5c(a6)               | +01c

| ----------------------------------------------------------------------------
|  Tank_DriverBail_0698aa  @ $0698AA  (114 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DriverBail_0698aa, "ax", @progbits
        .global Tank_DriverBail_0698aa
Tank_DriverBail_0698aa:
        move.b  0x98(a6),d0                     | +000
        cmpi.b  #0x2,d0                         | +004
        blt.w   .L0698bc                        | +008
        jsr     0x5e9b6.l                       | +00c
.L0698bc:
        andi.b  #0x1,d0                         | +012
        move.b  d0,0x98(a6)                     | +016
        cmpi.b  #0x0,d0                         | +01a
        bne.w   .L0698dc                        | +01e
        lea     0x2c9b2c.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L0698e8                        | +02e
.L0698dc:
        lea     0x2c9b88.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
.L0698e8:
        lea     .L0698ee(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0698ee:
        jsr     Tank_CopyParentAnimPrio_069ebe(pc) | +044
        jsr     Tank_OffsetAbove_069ed4(pc)     | +048
        move.b  #0x0,0x44(a6)                   | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   .L06990c                        | +058
        lea     Tank_DriverToSoldier_0699b2(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L06990c:
        jsr     0x2870a.l                       | +062
        bcc.w   JsrAbsRts_069922                | +068
        lea     Tank_DriverHit_069924(pc),a1    | +06c
        move.l  a1,(a6)                         | +070

| ----------------------------------------------------------------------------
|  Tank_DriverHit_069924  @ $069924  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DriverHit_069924, "ax", @progbits
        .global Tank_DriverHit_069924
Tank_DriverHit_069924:
        move.w  #0x0,0x72(a6)                   | +000
        lea     0x2c997c.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06993c(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06993c:
        jsr     Tank_CopyParentAnimPrio_069ebe(pc) | +018
        jsr     Tank_OffsetAbove_069ed4(pc)     | +01c
        movea.l 0xc(a6),a0                      | +020
        move.b  #0x0,0x44(a6)                   | +024
        cmpi.w  #0x0,0x94(a0)                   | +02a
        bne.w   JsrAbsThunk_06995e              | +030
        lea     Tank_DriverDie_069966(pc),a1    | +034
        move.l  a1,(a6)                         | +038

| ----------------------------------------------------------------------------
|  Tank_DriverDie_069966  @ $069966  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DriverDie_069966, "ax", @progbits
        .global Tank_DriverDie_069966
Tank_DriverDie_069966:
        jsr     0x4a0d4.l                       | +000
        jsr     0x267e2.l                       | +006
        lea     0x4acfe.l,a0                    | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L069984(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L069984:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        bcc.w   .L06999a                        | +02a
        lea     Jsr5B6ThenJmpScheduler_069872(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L06999a:
        movea.l #0xffffffff,a0                  | +034
        jsr     0x5dd56.l                       | +03a
        bcc.w   SetHandlerRts_0699b0            | +040

| ----------------------------------------------------------------------------
|  Tank_DriverToSoldier_0699b2  @ $0699B2  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DriverToSoldier_0699b2, "ax", @progbits
        .global Tank_DriverToSoldier_0699b2
Tank_DriverToSoldier_0699b2:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x18,0x38(a6)                  | +010
        addq.w  #0x8,0x24(a6)                   | +016
        cmpi.b  #0x0,0x98(a6)                   | +01a
        bne.w   .L069a08                        | +020
        eori.b  #0x1,0x3a(a6)                   | +024
        move.w  #0xff78,d0                      | +02a
        jsr     0x5dca4.l                       | +02e
        move.w  d0,0x28(a6)                     | +034
        move.w  #0x21c,0x2a(a6)                 | +038
        move.w  #0xffb8,0x2e(a6)                | +03e
        move.w  #0x0,0x2c(a6)                   | +044
        jsr     0x4a0d4.l                       | +04a
        jmp     0x58f82.l                       | +050
.L069a08:
        jsr     0x4a0d4.l                       | +056
        addq.w  #0x4,0x24(a6)                   | +05c
        jmp     0x5724e.l                       | +060

| ----------------------------------------------------------------------------
|  Tank_DriverWreck_069a18  @ $069A18  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DriverWreck_069a18, "ax", @progbits
        .global Tank_DriverWreck_069a18
Tank_DriverWreck_069a18:
        jsr     Tank_CopyParentAnimPrio_069ebe(pc) | +000
        jsr     Tank_OffsetAbove_069ed4(pc)     | +004
        jsr     0x28d70.l                       | +008
        jsr     0x4a166.l                       | +00e
        addq.w  #0x2,0x38(a6)                   | +014
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Tank_Turret_069a32  @ $069A32  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Turret_069a32, "ax", @progbits
        .global Tank_Turret_069a32
Tank_Turret_069a32:
        jsr     Tank_EngineSound_069d5e(pc)     | +000
        move.w  #0x3,0x1c(a6)                   | +004
        jsr     0x138fe.l                       | +00a
        lea     0x2b7df0.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.w  d0,0x66(a6)                     | +01c
        lea     .L069a58(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L069a58:
        jsr     Tank_CopyParentAnim_069eaa(pc)  | +026
        jsr     0x28d70.l                       | +02a
        jsr     0x283ca.l                       | +030
        jsr     0x283d8.l                       | +036
        jsr     0x2870a.l                       | +03c
        bcc.w   .L069a90                        | +042
        lea     0x5e766.l,a0                    | +046
        jsr     0x5e770.l                       | +04c
        bclr    #0x3,0x13(a6)                   | +052
        bclr    #0x0,0x13(a6)                   | +058
.L069a90:
        cmpi.b  #0x33,0x20(a6)                  | +05e
        beq.w   .L069ac6                        | +064
        cmpi.b  #0x77,0x20(a6)                  | +068
        beq.w   .L069ac6                        | +06e
        cmpi.w  #0x0,0x66(a6)                   | +072
        bgt.w   .L069ac6                        | +078
        move.b  #0x33,0x20(a6)                  | +07c
        move.w  #0x102d,d0                      | +082
        jsr     0x2352.l                        | +086
        lea     Tank_Die_0694dc(pc),a1          | +08c
        jsr     Tank_SetParentHandler_069db4(pc) | +090
.L069ac6:
        cmpi.b  #0x77,0x20(a6)                  | +094
        bne.w   .L069ad6                        | +09a
        lea     Tank_TurretFree_069ae8(pc),a1   | +09e
        move.l  a1,(a6)                         | +0a2
.L069ad6:
        jsr     0x5e45a.l                       | +0a4
        bcc.w   SetHandlerRts_069ae6            | +0aa

| ----------------------------------------------------------------------------
|  Tank_TurretFree_069ae8  @ $069AE8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_TurretFree_069ae8, "ax", @progbits
        .global Tank_TurretFree_069ae8
Tank_TurretFree_069ae8:
        movea.l 0xc(a6),a0                      | +000
        move.l  #0xffffffff,0x8c(a0)            | +004
        bra.w   Jsr5B6ThenJmpScheduler_069872   | +00c

| ----------------------------------------------------------------------------
|  Tank_Debris_069af8  @ $069AF8  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Debris_069af8, "ax", @progbits
        .global Tank_Debris_069af8
Tank_Debris_069af8:
        lea     0x2c8bfe.l,a0                   | +000
        bra.w   .L069b08                        | +006
        lea     0x2c8c16.l,a0                   | +00a
.L069b08:
        move.l  a0,0x5c(a6)                     | +010
        lea     0x2b7e72.l,a0                   | +014
        jsr     0x799de.l                       | +01a
        movea.l 0x5c(a6),a0                     | +020
        andi.w  #0x3,d0                         | +024
        move.w  d0,d1                           | +028
        add.w   d1,d1                           | +02a
        add.w   d1,d0                           | +02c
        lsl.w   #0x1,d0                         | +02e
        move.w  (a0,d0.w),0x28(a6)              | +030
        move.w  0x2(a0,d0.w),0x2e(a6)           | +036
        move.w  0x4(a0,d0.w),0x2a(a6)           | +03c
        jmp     0x6dbd4.l                       | +042

| ----------------------------------------------------------------------------
|  Tank_Smoke_069b40  @ $069B40  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Smoke_069b40, "ax", @progbits
        .global Tank_Smoke_069b40
Tank_Smoke_069b40:
        move.w  #0x8,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2c9bbe.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L069b76                        | +016
        .global Tank_Smoke_069b40__L069b5a
Tank_Smoke_069b40__L069b5a:
.L069b5a:
        eori.b  #0x1,0x3a(a6)                   | +01a
        move.w  #0x8,d1                         | +020
        jsr     0x236e.l                        | +024
        lea     0x2c9c60.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
.L069b76:
        move.w  #0x4000,d0                      | +036
        jsr     0x28134.l                       | +03a
        andi.w  #0xffe3,0x38(a6)                | +040
        ori.w   #0x1c,0x38(a6)                  | +046
        lea     .L069b92(pc),a1                 | +04c
        move.l  a1,(a6)                         | +050
.L069b92:
        jsr     0x2783a.l                       | +052
        jsr     0x28d70.l                       | +058
        bcc.w   SetHandlerRts_069ba8            | +05e

| ----------------------------------------------------------------------------
|  Tank_SmokeExplode_069baa  @ $069BAA  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SmokeExplode_069baa, "ax", @progbits
        .global Tank_SmokeExplode_069baa
Tank_SmokeExplode_069baa:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77efe.l                       | +01e

| ----------------------------------------------------------------------------
|  Tank_Missile_069bce  @ $069BCE  (142 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_Missile_069bce, "ax", @progbits
        .global Tank_Missile_069bce
Tank_Missile_069bce:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x23,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0xc,0x82(a6)                   | +010
        move.w  #0x5,0x66(a6)                   | +016
        move.w  #0xd000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x14,0x38(a6)                  | +02c
        lea     0x2b7ef4.l,a0                   | +032
        jsr     0x799de.l                       | +038
        move.w  d0,0x36(a6)                     | +03e
        lsr.w   #0x8,d0                         | +042
        lea     0x2c8c2e.l,a0                   | +044
        move.b  (a0,d0.w),0x7f(a6)              | +04a
        lea     0x2c8c3e.l,a0                   | +050
        add.w   d0,d0                           | +056
        move.w  (a0,d0.w),0x76(a6)              | +058
        move.w  #0x5a,0x74(a6)                  | +05e
        lea     0x2c9cf8.l,a0                   | +064
        jsr     0x28cd4.l                       | +06a
        jsr     Tank_VelFromAngle_069e5c(pc)    | +070
        lea     .L069c48(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L069c48:
        subq.w  #0x1,0x76(a6)                   | +07a
        cmpi.w  #0x0,0x76(a6)                   | +07e
        bgt.w   Tank_MissileFly_069c5c          | +084
        lea     Tank_MissileHome_069cd0(pc),a1  | +088
        move.l  a1,(a6)                         | +08c

| ----------------------------------------------------------------------------
|  Tank_MissileFly_069c5c  @ $069C5C  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_MissileFly_069c5c, "ax", @progbits
        .global Tank_MissileFly_069c5c
Tank_MissileFly_069c5c:
        subq.w  #0x1,0x74(a6)                   | +000
        cmpi.w  #0x0,0x74(a6)                   | +004
        bgt.w   .L069c70                        | +00a
        lea     Tank_MissileFly_069c5c(pc),a1   | +00e
        move.l  a1,(a6)                         | +012
.L069c70:
        jsr     0x27cee.l                       | +014
        bcc.w   .L069c80                        | +01a
        lea     Tank_MissileExplode_069d34(pc),a1 | +01e
        move.l  a1,(a6)                         | +022
.L069c80:
        jsr     0x28d70.l                       | +024
        jsr     0x283d8.l                       | +02a
        btst    #0x1,0x13(a6)                   | +030
        beq.w   .L069c9c                        | +036
        lea     Tank_MissileExplode_069d34(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L069c9c:
        bclr    #0x3,0x13(a6)                   | +040
        jsr     0x28758.l                       | +046
        bcc.w   .L069cb2                        | +04c
        lea     Tank_MissileExplode_069d34(pc),a1 | +050
        move.l  a1,(a6)                         | +054
.L069cb2:
        movea.l #0xffffffff,a0                  | +056
        lea     0x2c8b7c.l,a0                   | +05c
        jsr     0x5dd56.l                       | +062
        bcc.w   SetHandlerRts_069cce            | +068

| ----------------------------------------------------------------------------
|  Tank_MissileHome_069cd0  @ $069CD0  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_MissileHome_069cd0, "ax", @progbits
        .global Tank_MissileHome_069cd0
Tank_MissileHome_069cd0:
        move.w  #0x258,0x76(a6)                 | +000
        jsr     0x5e1ea.l                       | +006
        move.l  a0,0x84(a6)                     | +00c
        lea     .L069ce6(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L069ce6:
        addq.w  #0x1,0x76(a6)                   | +016
        move.w  0x76(a6),d0                     | +01a
        cmp.b   0x7f(a6),d0                     | +01e
        ble.w   .L069d30                        | +022
        clr.w   0x76(a6)                        | +026
        movea.l 0x84(a6),a0                     | +02a
        jsr     0x5e070.l                       | +02e
        addq.w  #0x8,d0                         | +034
        andi.w  #0xff,d0                        | +036
        lsr.w   #0x3,d0                         | +03a
        cmp.w   0x82(a6),d0                     | +03c
        beq.w   .L069d26                        | +040
        move.w  #0x1,d1                         | +044
        sub.w   0x82(a6),d0                     | +048
        bcc.w   .L069d22                        | +04c
        neg.w   d1                              | +050
.L069d22:
        add.w   d1,0x82(a6)                     | +052
.L069d26:
        andi.w  #0x1f,0x82(a6)                  | +056
        jsr     Tank_VelFromAngle_069e5c(pc)    | +05c
.L069d30:
        bra.w   Tank_MissileFly_069c5c          | +060

| ----------------------------------------------------------------------------
|  Tank_MissileExplode_069d34  @ $069D34  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_MissileExplode_069d34, "ax", @progbits
        .global Tank_MissileExplode_069d34
Tank_MissileExplode_069d34:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x0,0x38(a6)                   | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77f6a.l                       | +024

| ----------------------------------------------------------------------------
|  Tank_EngineSound_069d5e  @ $069D5E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_EngineSound_069d5e, "ax", @progbits
        .global Tank_EngineSound_069d5e
Tank_EngineSound_069d5e:
        move.w  #0x1b,d1                        | +000
        tst.b   0x9b(a6)                        | +004
        beq.w   JsrAbsThunk_069d6e              | +008
        move.w  #0x1c,d1                        | +00c

| ----------------------------------------------------------------------------
|  Tank_SetSpriteByIndex_069d76  @ $069D76  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetSpriteByIndex_069d76, "ax", @progbits
        .global Tank_SetSpriteByIndex_069d76
Tank_SetSpriteByIndex_069d76:
        move.l  a1,d2                           | +000
        cmpi.l  #0xffffffff,d2                  | +002
        beq.w   .L069da2                        | +008
        move.l  a6,-(a7)                        | +00c
        andi.w  #0xff,d0                        | +00e
        movea.l a1,a6                           | +012
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L069da0                        | +020
        jsr     0x28cd4.l                       | +024
.L069da0:
        movea.l (a7)+,a6                        | +02a
.L069da2:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  Tank_SetHandlerIfValid_069da4  @ $069DA4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetHandlerIfValid_069da4, "ax", @progbits
        .global Tank_SetHandlerIfValid_069da4
Tank_SetHandlerIfValid_069da4:
        cmpi.l  #0xffffffff,d0                  | +000
        beq.w   .L069db2                        | +006
        movea.l d0,a0                           | +00a
        move.l  a1,(a0)                         | +00c
.L069db2:
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Tank_SetParentHandler_069db4  @ $069DB4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetParentHandler_069db4, "ax", @progbits
        .global Tank_SetParentHandler_069db4
Tank_SetParentHandler_069db4:
        movea.l 0xc(a6),a0                      | +000
        move.l  a1,(a0)                         | +004
        rts                                     | +006

| ----------------------------------------------------------------------------
|  Tank_MirrorByFacing_069dbc  @ $069DBC  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_MirrorByFacing_069dbc, "ax", @progbits
        .global Tank_MirrorByFacing_069dbc
Tank_MirrorByFacing_069dbc:
        cmpi.b  #0x1,0x3a(a6)                   | +000
        .global Tank_MirrorByFacing_069dbc__L069dc2
Tank_MirrorByFacing_069dbc__L069dc2:
.L069dc2:
        beq.w   ClearXN_069dce                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  Tank_MirrorByDir_069dd4  @ $069DD4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_MirrorByDir_069dd4, "ax", @progbits
        .global Tank_MirrorByDir_069dd4
Tank_MirrorByDir_069dd4:
        cmpi.b  #0x1,0x70(a6)                   | +000
        bra.b   Tank_MirrorByFacing_069dbc__L069dc2 | +006

| ----------------------------------------------------------------------------
|  Tank_FacingMatchesDir_069ddc  @ $069DDC  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_FacingMatchesDir_069ddc, "ax", @progbits
        .global Tank_FacingMatchesDir_069ddc
Tank_FacingMatchesDir_069ddc:
        move.b  0x3a(a6),d0                     | +000
        cmp.b   0x70(a6),d0                     | +004
        bne.w   SetXN_069dee                    | +008

| ----------------------------------------------------------------------------
|  Tank_PastRangeX_069df4  @ $069DF4  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_PastRangeX_069df4, "ax", @progbits
        .global Tank_PastRangeX_069df4
Tank_PastRangeX_069df4:
        move.w  0x22(a6),d1                     | +000
        move.w  #0x0,d0                         | +004
        jsr     Tank_MirrorByDir_069dd4(pc)     | +008
        bcs.w   .L069e0c                        | +00c
        add.w   0x72(a6),d0                     | +010
        cmp.w   d1,d0                           | +014
        rts                                     | +016
.L069e0c:
        add.w   0x72(a6),d0                     | +018
        cmp.w   d0,d1                           | +01c
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Tank_CooldownTick_069e14  @ $069E14  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_CooldownTick_069e14, "ax", @progbits
        .global Tank_CooldownTick_069e14
Tank_CooldownTick_069e14:
        subq.w  #0x1,0x76(a6)                   | +000
        bgt.w   ClearXN_069e30                  | +004
        clr.w   0x76(a6)                        | +008
        cmpi.w  #0x120,0x22(a6)                 | +00c
        bgt.w   ClearXN_069e30                  | +012

| ----------------------------------------------------------------------------
|  Tank_AnimFrame_069e36  @ $069E36  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_AnimFrame_069e36, "ax", @progbits
        .global Tank_AnimFrame_069e36
Tank_AnimFrame_069e36:
        clr.l   d0                              | +000
        move.w  0x96(a6),d0                     | +002
        addq.w  #0x1,0x96(a6)                   | +006
        lsr.w   #0x4,d0                         | +00a
        cmpi.w  #0x3,d0                         | +00c
        ble.w   .L069e4e                        | +010
        move.w  #0x3,d0                         | +014
.L069e4e:
        lsl.w   #0x2,d0                         | +018
        movea.l 0x90(a6),a0                     | +01a
        move.l  (a0,d0.w),0x5c(a6)              | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Tank_VelFromAngle_069e5c  @ $069E5C  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_VelFromAngle_069e5c, "ax", @progbits
        .global Tank_VelFromAngle_069e5c
Tank_VelFromAngle_069e5c:
        move.w  0x82(a6),d0                     | +000
        andi.w  #0x1f,d0                        | +004
        lsl.w   #0x4,d0                         | +008
        lea     0x2c07ac.l,a1                   | +00a
        lea     0x2c072c.l,a2                   | +010
        move.w  (a1,d0.w),d1                    | +016
        move.w  (a2,d0.w),d2                    | +01a
        move.w  0x36(a6),d0                     | +01e
        muls.w  d0,d1                           | +022
        muls.w  d0,d2                           | +024
        asr.l   #0x8,d1                         | +026
        asr.l   #0x8,d2                         | +028
        move.w  d1,0x28(a6)                     | +02a
        move.w  d2,0x2a(a6)                     | +02e
        rts                                     | +032

| ----------------------------------------------------------------------------
|  Tank_RecoilTick_069e90  @ $069E90  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_RecoilTick_069e90, "ax", @progbits
        .global Tank_RecoilTick_069e90
Tank_RecoilTick_069e90:
        cmpi.w  #0x0,0x94(a6)                   | +000
        ble.w   SetXNMid_069ea8                 | +006
        subq.w  #0x1,0x94(a6)                   | +00a
        addi.b  #0x1,0x44(a6)                   | +00e

| ----------------------------------------------------------------------------
|  Tank_CopyParentAnim_069eaa  @ $069EAA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_CopyParentAnim_069eaa, "ax", @progbits
        .global Tank_CopyParentAnim_069eaa
Tank_CopyParentAnim_069eaa:
        jsr     0x5e506.l                       | +000
        move.b  0x44(a0),0x44(a6)               | +006
        move.l  0x5c(a0),0x5c(a6)               | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Tank_CopyParentAnimPrio_069ebe  @ $069EBE  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_CopyParentAnimPrio_069ebe, "ax", @progbits
        .global Tank_CopyParentAnimPrio_069ebe
Tank_CopyParentAnimPrio_069ebe:
        jsr     0x5e506.l                       | +000
        move.w  0x72(a6),d0                     | +006
        add.w   d0,0x38(a6)                     | +00a
        move.b  0x44(a0),0x44(a6)               | +00e
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Tank_OffsetAbove_069ed4  @ $069ED4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_OffsetAbove_069ed4, "ax", @progbits
        .global Tank_OffsetAbove_069ed4
Tank_OffsetAbove_069ed4:
        addi.w  #0x18,0x24(a6)                  | +000
        move.w  #0xfff0,d0                      | +006
        jsr     Tank_MirrorByFacing_069dbc(pc)  | +00a
        add.w   d0,0x22(a6)                     | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Tank_ItemFlag_069ee8  @ $069EE8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_ItemFlag_069ee8, "ax", @progbits
        .global Tank_ItemFlag_069ee8
Tank_ItemFlag_069ee8:
        tst.b   0x9c(a6)                        | +000
        beq.w   ClrRamWordRts_069ef6            | +004

| ----------------------------------------------------------------------------
|  Tank_SpawnExplosion_069ef8  @ $069EF8  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SpawnExplosion_069ef8, "ax", @progbits
        .global Tank_SpawnExplosion_069ef8
Tank_SpawnExplosion_069ef8:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0x4000,0x38(a0)                | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Tank_DeathBlast_069f12  @ $069F12  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DeathBlast_069f12, "ax", @progbits
        .global Tank_DeathBlast_069f12
Tank_DeathBlast_069f12:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  #0x10,d0                        | +01c
.L069f32:
        movem.w d0,-(a7)                        | +020
        lea     0x62536.l,a1                    | +024
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        addi.w  #0x18,0x24(a0)                  | +036
        movem.w (a7)+,d0                        | +03c
        subq.w  #0x1,d0                         | +040
        bcc.b   .L069f32                        | +042
        rts                                     | +044

| ----------------------------------------------------------------------------
|  Tank_DropItem_069f58  @ $069F58  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_DropItem_069f58, "ax", @progbits
        .global Tank_DropItem_069f58
Tank_DropItem_069f58:
        move.b  0x9d(a6),d0                     | +000
        move.w  #0x9e,d1                        | +004

| ----------------------------------------------------------------------------
|  Tank_SpawnSmokeMissile_069f68  @ $069F68  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SpawnSmokeMissile_069f68, "ax", @progbits
        .global Tank_SpawnSmokeMissile_069f68
Tank_SpawnSmokeMissile_069f68:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     Tank_Smoke_069b40__L069b5a(pc),a1 | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        addi.w  #0x30,0x24(a0)                  | +01a
        move.w  #0x28,d0                        | +020
        jsr     Tank_MirrorByFacing_069dbc(pc)  | +024
        add.w   d0,0x22(a0)                     | +028
        lea     Tank_Missile_069bce(pc),a1      | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        addi.w  #0x3e,0x24(a0)                  | +03c
        move.w  #0x28,d0                        | +042
        jsr     Tank_MirrorByFacing_069dbc(pc)  | +046
        add.w   d0,0x22(a0)                     | +04a
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  Tank_SpawnSmokeDebris_069fb8  @ $069FB8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SpawnSmokeDebris_069fb8, "ax", @progbits
        .global Tank_SpawnSmokeDebris_069fb8
Tank_SpawnSmokeDebris_069fb8:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     Tank_Smoke_069b40(pc),a1        | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     Tank_Debris_069af8(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        addi.w  #0x28,0x24(a0)                  | +02a
        move.w  #0x20,d0                        | +030
        jsr     Tank_MirrorByFacing_069dbc(pc)  | +034
        add.w   d0,0x22(a0)                     | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  Tank_SetPrio4000_069ff6  @ $069FF6  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Tank_SetPrio4000_069ff6, "ax", @progbits
        .global Tank_SetPrio4000_069ff6
Tank_SetPrio4000_069ff6:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
