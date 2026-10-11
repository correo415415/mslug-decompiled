| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave AAAA — granada del player, proyectiles, caída del vehículo, fx, iconos, tablas del player
|  Región: $030602..$032A02  (8,788 B, 96 entradas, 27 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 8,788 B entre el módulo del Slug (Wave ZZZ) y el player core
|  (Wave TTT). Mezcla cuatro grupos distintos más el bloque de tablas
|  estáticas del player:
|
|   1. Resto de Chain3 (continuación de ZZZ): Chain3_InitAlt_030696
|      (variante de Chain3_Init que marca +$98 = 1 y bset #3,+$5B; ref
|      $6BAD2) y los helpers Chain3_PickLink_030704 (elige el eslabón
|      +$74/+$78 cuyo +$5A bit5 esté activo; refs $2A76E..$2A862),
|      Chain3_LinkCmpField82_03073e, Chain3_LinksYDeltaIsStep_03076a
|      (|dY| entre eslabones ∈ {0,1,2,7,8,9,$1F..$22}; refs $2A798..),
|      Entity_CmpDepthWithLink8_0307e8 (+$10 vs link +$8 -> C).
|   2. Granada del player: PlayerGrenade_Spawn_0308c2 (snd $2, ref desde
|      PlayerFire_Flame/$44B68) y PlayerGrenade_SpawnB_03093a (refs
|      $3C74C/$8B66A/$9A260), PlayerGrenade_SpawnFromVehicle_0308b0 (snd
|      $1D6, jmp desde vehicle_deploy $45FE0): copian +$98/+$9A del
|      template a +$70/+$7A, crean el fx $77A96, eligen sprite por
|      +$70>>4 en $29D4D2/$29D452, calculan el ángulo de lanzamiento según
|      el layout $106F2A (0,1,2,3,5) o el bit1 de $100000 (debug) con
|      PlayerGrenade_AngleFromSpread_030bfe (+$9B = dispersión), velocidad
|      $800 (+$F800 vertical si +$7A bit0) vía $13C0E (sin/cos), tabla de
|      ataque PlayerGrenade_AttackTbl_030804 (2 x 80 B), rebote con
|      $27CEE; al tocar suelo (PlayerGrenade_GroundTbl_0308a8 / $5DD56)
|      -> PlayerGrenade_ExplodeGround_030bb6 (crea Fx_GroundBurst_031c72
|      via $6FE), si +$13 bit1 -> PlayerGrenade_ExplodeAir_030bd6
|      (Fx_AirBurst_031cca); +$76 = temporizador de mecha.
|   3. Proyectiles enemigos genéricos: EnemyShot_Straight_030c14 (snd $1D6,
|      ángulo +$70, sprite $29D7D2; refs mission_spawn_boss $44BEE,
|      turret_boss2 $458B0) y EnemyShot_Bounce_030c70 (snd $6, velX
|      aleatoria $5E9B6, sprite por RNG en $29D844, cae con $27D50, al
|      tocar suelo EnemyShot_BounceB_030d04 y luego EnemyShot_ExplodeAir
|      _030d5c; ref $44B4C).
|   4. Lanzamiento/caída del vehículo (VehicleLaunch_*): VehicleLaunch_Init
|      _0311c0 (ref $2C1EC) y VehicleLaunch_InitDrop_0318e6: usan +$94 del
|      padre como índice en VehicleLaunch_OffsetTblA/B (registros de 10 B
|      {?, dx, dy, ángulo}), sprite $29FA8A, tablas de ataque
|      VehicleLaunch_AttackTblA/B, música $1084, fx $77AB6, snd $58; según
|      $106F2A (2/3/5) consultan $5CD90 y Popcount4_0323b4 para elegir la
|      fase (+$72 = $7400/$6000/$4400/$26AA) y el impulso +$36. Vuelo:
|      VehicleLaunch_Fall_031594 -> VehicleLaunch_Glide_03168a (gravedad
|      $27D50, tope $700 vía $267F4, hitbox dinámico $5E018); crea un hijo
|      desde +$74 (= $31B78) cada N frames según +$20. Impacto:
|      VehicleLaunch_Crash_0317d2 ($10A2D1 = 1, música $10F2, $434DC,
|      cadena CrashA/B/C con tablas CrashTblA/B/C y $5E4B2),
|      VehicleLaunch_Despawn_0318b8 ($5B6) y _ReleaseParent_0318d4 (padre
|      +$8F--, $518).
|   5. Fx del Slug/explosiones: SlugFx_ExhaustOrDrop_0319f0 (refs $2C21A..;
|      si padre +$8D bit2 usa el ángulo +$80 y sprite $29E440, si no
|      VehicleDrop_SpriteTbl_0319dc por +$94), SlugFx_Exhaust_031af6 (refs
|      $2D31C/$2D3E2), Fx_Sparkle_031b52 (snd $58, $29FCCE), Fx_SmokePuff
|      _031bd0 (snd $178, $29E64A, prio $2000), Fx_DustCloud_031c20 (snd
|      $5, $29E568; refs $6E07E..), Fx_GroundBurst(_B) (snd $1D6/$4, par
|      de sprites por RNG; refs $460F6, $6DFC2/$82AEE), Fx_AirBurst_031cca
|      (snd $2, $2DD8B6), Fx_Spark_031d6a/_SparkAnim ($29E46A) con los
|      spawners condicionales Entity_SpawnSpark*IfBit1_031d90/dc2/dfe
|      (+$8C bit1, refs $1A3A14.., $1A682A.., $1A9472..), Fx_Dust_031e38/
|      _DustAnim ($2DD72A), Fx_SpawnDustPair_031e5e (jsr desde
|      Slug_AccelRight $2EE78 y $2B8DA; 10 variantes de offset ±$20/±$8/
|      ±$13 con eori #1,+$3A para espejar), Fx_SmokeAnim_031fa2 ($2DD944).
|   6. Iconos sobre el player: PlayerIcon_Pow_031fca (snd $1AD, anim
|      PlayerIcon_PowAnim/PowAltAnim según escena $106ECE == 5, +$5C = $28
|      de altura, sigue al padre con $5E4CA; ref $2AFAE),
|      PlayerIcon_Bubble_03207c (snd $1B0, refs $2A658/$30CA0/$462DA/$4EB6E),
|      PlayerIcon_SpawnFreeFallBoth_0320d4 (recorre slots con $5E3A2; ref
|      $84514) -> PlayerIcon_FreeFallP1/P2_032112/032142 (sólo si el padre
|      es $100440/$1004E0; snd $A2/$A3; refs Player_SpawnFreeFall_036c8c,
|      Player_SpawnLand_03386e).
|   7. Tablas estáticas del player (sólo datos, referenciadas por pc-rel
|      desde player_core/states/air/arm): Player_WeaponStateByteTbl_032412
|      (170 bytes 0/1), Player_GroundTblA/B_0324bc/c6 (sondas de suelo para
|      $5DD56 por escena), Player_VelYTbl/B, Player_VelXTbl (indexadas por
|      $5D5B6), Player_AttackTblA/B (+$4C), Player_Hitbox{Stand,Crouch,
|      Melee,Grenade,Air,Knockback,Death,Slug} (+$48, registros `02 00 00
|      24 36 xx ff ff` de 10 B + offsets), Player_WeaponAmmoTbl_0329d4
|      (5 words: $03E7/$03E7/$03E7/$03E7/$0096 = munición por arma),
|      Player_WeaponFlagTbl_0329e8 (6 bytes), OpcodeOffsetTable_0329EE y
|      Entity_ClearCollisionCb_0329f8 (+$48 = -1).
|
|  B. EVIDENCIAS
|  -------------
|   - PlayerFire_Pistol/Flame (Wave YYY) hacen `lea $3093A/$308C2,a1; jsr
|     $6FE` justo tras decrementar la munición +$82 -> son las granadas.
|   - vehicle_deploy_045f2c (Wave YY) salta a $308B0 y mission_spawn_boss
|     usa $30C14/$30C70/$308C2 como templates de proyectil.
|   - Player_SpawnFreeFall_036c8c elige $32112 o $32142 según a6 ==
|     $100440: iconos por jugador.
|   - Slug_AccelRight_02ee78 empieza con `jsr $31E5E` (polvo de las orugas).
|   - Player_WeaponAmmoTbl: 999/999/999/999/150 coincide con la munición
|     inicial de HMG/shotgun/rocket/flame en MS1 (pistola infinita).
|
|  C. CAMPOS (a6 = entidad, +$C = padre)
|  --------------------------------------
|   +$20 contador de hijos, +$22/+$24 pos, +$28/+$2A vel, +$2C/+$2E grav,
|   +$30 hitbox dinámico, +$34 ángulo, +$36 impulso, +$38 prio, +$3A
|   facing, +$46, +$48 cb colisión, +$4C tabla ataque, +$5C altura icono,
|   +$70 ángulo/contador, +$72 fase, +$74 template hijo, +$76 mecha,
|   +$7A flags granada, +$8C/+$8D bits, +$94 índice, +$98..+$9B params.
|
|  D. HELPERS EXTERNOS
|  -------------------
|   $4AE/$6FE alloc, $518 free, $5B6, $236E snd, $2352 music, $13C0E
|   sin/cos, $267E2, $267F4 clamp, $2783A física, $27CEE/$27D50 caída,
|   $27EBA, $283CA/$283D8 ataque, $28CD4 sprite, $28D70 paso anim,
|   $434DC, $5CD90, $5DD02 copia transform, $5DD56 suelo, $5E018 hitbox,
|   $5E3A2 slots, $5E4B2/$5E4CA seguir padre, $5E9B6 RNG.
|
|  E. HIPÓTESIS ABIERTAS
|  ---------------------
|   - "VehicleLaunch" = caída del vehículo lanzado desde el avión al inicio
|     de misión (usa $10A2D1 y música $10F2); podría ser también la caída
|     del Slug al pozo de la misión 2. Confirmar con $2C1EC.
|   - Nombres de los Fx por sonido/sprite; "Sparkle/SmokePuff/DustCloud"
|     son provisionales hasta ver los tiles $29Exxx.
|   - El orden de Player_Hitbox* (Stand/Crouch/...) se deduce de los
|     estados que los instalan en +$48 (Wave UUU/VVV/WWW).
|
|  F. SIGUIENTE
|  ------------
|   Helpers del Slug `$029xxx..$02DD20` (despachador $2A078, Slug_SlopeToAnimIdx_02a9a0),
|   después `$05AA96..$05CA2A`, `$057D04..$059342`.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Chain3_InitAlt_030696  @ $030696  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_InitAlt_030696, "ax", @progbits
        .global Chain3_InitAlt_030696
Chain3_InitAlt_030696:
        bset    #0x3,0x5b(a6)                   | +000
        lea     Chain3_TplC_03010c(pc),a1       | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        move.l  a0,0x7c(a6)                     | +016
        move.w  0x82(a6),0x24(a0)               | +01a
        move.b  #0x1,0x98(a0)                   | +020
        lea     Chain3_TplA_030068(pc),a1       | +026
        jsr     0x4ae.l                         | +02a
        jsr     0x5dd02.l                       | +030
        move.l  a0,0x74(a6)                     | +036
        move.w  0x82(a6),0x24(a0)               | +03a
        move.b  #0x1,0x98(a0)                   | +040
        lea     Chain3_TplB_0300ba(pc),a1       | +046
        jsr     0x4ae.l                         | +04a
        jsr     0x5dd02.l                       | +050
        move.l  a0,0x78(a6)                     | +056
        move.w  0x82(a6),0x24(a0)               | +05a
        move.b  #0x1,0x98(a0)                   | +060
        bra.w   Entity_Build3ChainCircular_03060A__L030664 | +066

| ----------------------------------------------------------------------------
|  Chain3_Nop_030700  @ $030700  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_Nop_030700, "ax", @progbits
        .global Chain3_Nop_030700
Chain3_Nop_030700:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Chain3_PickLink_030704  @ $030704  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_PickLink_030704, "ax", @progbits
        .global Chain3_PickLink_030704
Chain3_PickLink_030704:
        movea.l 0x74(a6),a1                     | +000
        movea.l 0x78(a6),a2                     | +004
        movea.l a1,a0                           | +008
        move.w  #0x1,d0                         | +00a
        move.b  0x5a(a1),d1                     | +00e
        andi.b  #0x20,d1                        | +012
        bne.w   .L030736                        | +016
        movea.l a2,a0                           | +01a
        move.w  #0xffff,d0                      | +01c
        move.b  0x5a(a2),d1                     | +020
        andi.b  #0x20,d1                        | +024
        bne.w   .L030736                        | +028
        movea.l a6,a0                           | +02c
        sub.w   d0,d0                           | +02e
        rts                                     | +030
.L030736:
        cmpa.l  #0xffffffff,a6                  | +032
        rts                                     | +038

| ----------------------------------------------------------------------------
|  Chain3_LinkCmpField82_03073e  @ $03073E  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_LinkCmpField82_03073e, "ax", @progbits
        .global Chain3_LinkCmpField82_03073e
Chain3_LinkCmpField82_03073e:
        jsr     Chain3_PickLink_030704(pc)      | +000
        bne.w   ClearXN_030764                  | +004
        move.w  0x82(a1),d1                     | +008
        move.w  #0x1,d0                         | +00c
        cmp.w   0x82(a2),d1                     | +010
        bge.w   SetXN_03075e                    | +014
        move.w  0x82(a2),d1                     | +018
        move.w  #0xffff,d0                      | +01c

| ----------------------------------------------------------------------------
|  Chain3_LinksYDeltaIsStep_03076a  @ $03076A  (126 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_LinksYDeltaIsStep_03076a, "ax", @progbits
        .global Chain3_LinksYDeltaIsStep_03076a
Chain3_LinksYDeltaIsStep_03076a:
        movea.l 0x74(a6),a1                     | +000
        movea.l 0x78(a6),a2                     | +004
        move.b  0x5a(a1),d1                     | +008
        or.b    0x5a(a2),d1                     | +00c
        andi.b  #0x10,d1                        | +010
        bne.w   .L0307e4                        | +014
        move.w  0x24(a1),d1                     | +018
        move.w  0x24(a2),d2                     | +01c
        sub.w   d2,d1                           | +020
        subx.w  d2,d2                           | +022
        add.w   d2,d1                           | +024
        eor.w   d2,d1                           | +026
        cmpi.w  #0x0,d1                         | +028
        beq.w   .L0307e4                        | +02c
        cmpi.w  #0x1,d1                         | +030
        beq.w   .L0307e4                        | +034
        cmpi.w  #0x2,d1                         | +038
        beq.w   .L0307e4                        | +03c
        cmpi.w  #0x8,d1                         | +040
        beq.w   .L0307e4                        | +044
        cmpi.w  #0x7,d1                         | +048
        beq.w   .L0307e4                        | +04c
        cmpi.w  #0x9,d1                         | +050
        beq.w   .L0307e4                        | +054
        cmpi.w  #0x20,d1                        | +058
        beq.w   .L0307e4                        | +05c
        cmpi.w  #0x1f,d1                        | +060
        beq.w   .L0307e4                        | +064
        cmpi.w  #0x21,d1                        | +068
        beq.w   .L0307e4                        | +06c
        cmpi.w  #0x22,d1                        | +070
        beq.w   .L0307e4                        | +074
        rts                                     | +078
.L0307e4:
        cmp.w   d1,d1                           | +07a
        rts                                     | +07c

| ----------------------------------------------------------------------------
|  Entity_CmpDepthWithLink8_0307e8  @ $0307E8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthWithLink8_0307e8, "ax", @progbits
        .global Entity_CmpDepthWithLink8_0307e8
Entity_CmpDepthWithLink8_0307e8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0307fe                    | +00c

| ----------------------------------------------------------------------------
|  PlayerGrenade_AttackTbl_030804  @ $030804  (164 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_AttackTbl_030804, "ax", @progbits
        .global PlayerGrenade_AttackTbl_030804
PlayerGrenade_AttackTbl_030804:
        .dc.w   0x0301                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfffd                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0401                        | +050  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xfffd                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfffd                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerGrenade_GroundTbl_0308a8  @ $0308A8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_GroundTbl_0308a8, "ax", @progbits
        .global PlayerGrenade_GroundTbl_0308a8
PlayerGrenade_GroundTbl_0308a8:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerGrenade_SpawnFromVehicle_0308b0  @ $0308B0  (18 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_SpawnFromVehicle_0308b0, "ax", @progbits
        .global PlayerGrenade_SpawnFromVehicle_0308b0
PlayerGrenade_SpawnFromVehicle_0308b0:
        move.w  #0x1d6,d1                       | +000
        move.w  d1,0x9e(a6)                     | +004
        jsr     0x236e.l                        | +008
        bra.w   PlayerGrenade_Spawn_0308c2__L0308d0 | +00e

| ----------------------------------------------------------------------------
|  PlayerGrenade_Spawn_0308c2  @ $0308C2  (120 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_Spawn_0308c2, "ax", @progbits
        .global PlayerGrenade_Spawn_0308c2
PlayerGrenade_Spawn_0308c2:
        move.w  #0x2,d1                         | +000
        move.w  d1,0x9e(a6)                     | +004
        jsr     0x236e.l                        | +008
        .global PlayerGrenade_Spawn_0308c2__L0308d0
PlayerGrenade_Spawn_0308c2__L0308d0:
        move.b  0x98(a6),0x70(a6)               | +00e
        move.b  0x9a(a6),0x7a(a6)               | +014
        bset    #0x4,0x6b(a6)                   | +01a
        move.w  #0xd000,0x38(a6)                | +020
        lea     0x77a96.l,a1                    | +026
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        move.b  0x70(a6),d0                     | +038
        andi.w  #0xf8,d0                        | +03c
        asr.w   #0x4,d0                         | +040
        movea.l 0xc(a6),a0                      | +042
        movea.l 0xc(a0),a0                      | +046
        movea.l #0x29d4d2,a0                    | +04a
        lsl.w   #0x2,d0                         | +050
        movea.l (a0,d0.w),a0                    | +052
        cmpa.l  #0xffffffff,a0                  | +056
        beq.w   .L030928                        | +05c
        jsr     0x28cd4.l                       | +060
.L030928:
        lea     PlayerGrenade_AttackTbl_030804(pc),a0 | +066
        move.l  a0,0x4c(a6)                     | +06a
        jsr     0x283ca.l                       | +06e
        jmp     PlayerGrenade_SpawnB_03093a__L0309ae(pc) | +074

| ----------------------------------------------------------------------------
|  PlayerGrenade_SpawnB_03093a  @ $03093A  (628 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_SpawnB_03093a, "ax", @progbits
        .global PlayerGrenade_SpawnB_03093a
PlayerGrenade_SpawnB_03093a:
        move.b  0x98(a6),0x70(a6)               | +000
        move.b  0x9a(a6),0x7a(a6)               | +006
        bset    #0x4,0x6b(a6)                   | +00c
        lea     PlayerGrenade_AttackTbl_030804(pc),a0 | +012
        move.l  a0,0x4c(a6)                     | +016
        jsr     0x283ca.l                       | +01a
        move.w  #0xd000,0x38(a6)                | +020
        lea     0x77a96.l,a1                    | +026
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        move.w  #0x2,d1                         | +038
        move.w  d1,0x9e(a6)                     | +03c
        jsr     0x236e.l                        | +040
        move.b  0x70(a6),d0                     | +046
        andi.w  #0xf8,d0                        | +04a
        asr.w   #0x4,d0                         | +04e
        movea.l 0xc(a6),a0                      | +050
        movea.l 0xc(a0),a0                      | +054
        movea.l #0x29d452,a0                    | +058
        lsl.w   #0x2,d0                         | +05e
        movea.l (a0,d0.w),a0                    | +060
        cmpa.l  #0xffffffff,a0                  | +064
        beq.w   PlayerGrenade_SpawnB_03093a__L0309ae | +06a
        jsr     0x28cd4.l                       | +06e
        .global PlayerGrenade_SpawnB_03093a__L0309ae
PlayerGrenade_SpawnB_03093a__L0309ae:
        jsr     0x267e2.l                       | +074
        move.b  0x70(a6),d0                     | +07a
        andi.w  #0xf8,d0                        | +07e
        btst    #0x1,0x100000.l                 | +082
        beq.w   .L0309f4                        | +08a
        move.w  d0,d1                           | +08e
        moveq   #0,d0                           | +090
        move.b  0x9b(a6),d0                     | +092
        move.b  d0,d2                           | +096
        andi.w  #0x8,d2                         | +098
        beq.w   .L0309e0                        | +09c
        not.w   d0                              | +0a0
        andi.w  #0x7,d0                         | +0a2
.L0309e0:
        addq.w  #0x4,d0                         | +0a6
        andi.w  #0x7,d0                         | +0a8
        subq.w  #0x4,d0                         | +0ac
        lsl.w   #0x1,d0                         | +0ae
        add.w   d1,d0                           | +0b0
        andi.w  #0xff,d0                        | +0b2
        bra.w   .L030aa4                        | +0b6
.L0309f4:
        cmpi.b  #0x5,0x106f2a.l                 | +0ba
        bne.w   .L030a2c                        | +0c2
        move.w  d0,d1                           | +0c6
        moveq   #0,d0                           | +0c8
        move.b  0x9b(a6),d0                     | +0ca
        move.b  d0,d2                           | +0ce
        andi.w  #0x8,d2                         | +0d0
        beq.w   .L030a18                        | +0d4
        not.w   d0                              | +0d8
        andi.w  #0x7,d0                         | +0da
.L030a18:
        addq.w  #0x4,d0                         | +0de
        andi.w  #0x7,d0                         | +0e0
        subq.w  #0x4,d0                         | +0e4
        lsl.w   #0x1,d0                         | +0e6
        add.w   d1,d0                           | +0e8
        andi.w  #0xff,d0                        | +0ea
        bra.w   .L030aa4                        | +0ee
.L030a2c:
        cmpi.b  #0x1,0x106f2a.l                 | +0f2
        bne.w   .L030a56                        | +0fa
        move.w  d0,d1                           | +0fe
        moveq   #0,d0                           | +100
        move.b  0x9b(a6),d0                     | +102
        andi.w  #0x1,d0                         | +106
        lsl.w   #0x1,d0                         | +10a
        subq.w  #0x1,d0                         | +10c
        add.w   d1,d0                           | +10e
        andi.w  #0xff,d0                        | +110
        jsr     PlayerGrenade_AngleFromSpread_030bfe(pc) | +114
        bra.w   .L030aa4                        | +118
.L030a56:
        cmpi.b  #0x3,0x106f2a.l                 | +11c
        bne.w   .L030a6a                        | +124
        jsr     PlayerGrenade_AngleFromSpread_030bfe(pc) | +128
        bra.w   .L030aa4                        | +12c
.L030a6a:
        cmpi.b  #0x2,0x106f2a.l                 | +130
        bne.w   .L030a7e                        | +138
        jsr     PlayerGrenade_AngleFromSpread_030bfe(pc) | +13c
        bra.w   .L030aa4                        | +140
.L030a7e:
        cmpi.b  #0x0,0x106f2a.l                 | +144
        bne.w   .L030aa4                        | +14c
        move.w  d0,d1                           | +150
        moveq   #0,d0                           | +152
        move.b  0x9b(a6),d0                     | +154
        addq.w  #0x4,d0                         | +158
        andi.w  #0x7,d0                         | +15a
        subq.w  #0x4,d0                         | +15e
        add.w   d1,d0                           | +160
        andi.w  #0xff,d0                        | +162
        bra.w   .L030aa4                        | +166
.L030aa4:
        movem.w d0,-(a7)                        | +16a
        move.b  0x7a(a6),d1                     | +16e
        andi.b  #0x1,d1                         | +172
        beq.w   .L030adc                        | +176
        move.w  #0xf800,d1                      | +17a
        jsr     0x13c0e.l                       | +17e
        movea.l 0xc(a6),a0                      | +184
        btst    #0x0,0x3a(a0)                   | +188
        beq.w   .L030ace                        | +18e
        neg.w   d1                              | +192
.L030ace:
        move.w  d1,0x28(a6)                     | +194
        move.w  d2,0x2a(a6)                     | +198
        jsr     0x27cee.l                       | +19c
.L030adc:
        movem.w (a7)+,d0                        | +1a2
        move.w  #0x800,d1                       | +1a6
        jsr     0x13c0e.l                       | +1aa
        move.w  d1,0x28(a6)                     | +1b0
        move.w  d2,0x2a(a6)                     | +1b4
        movea.l 0xc(a6),a0                      | +1b8
        movea.l 0xc(a0),a0                      | +1bc
        btst    #0x6,0x6b(a0)                   | +1c0
        beq.w   .L030b1c                        | +1c6
        movea.l 0xc(a6),a0                      | +1ca
        move.b  0x3a(a0),d1                     | +1ce
        move.b  d1,0x3a(a6)                     | +1d2
        andi.b  #0x1,d1                         | +1d6
        beq.w   .L030b1c                        | +1da
        neg.w   0x28(a6)                        | +1de
.L030b1c:
        jsr     0x283ca.l                       | +1e2
        lea     .L030b28(pc),a1                 | +1e8
        move.l  a1,(a6)                         | +1ec
.L030b28:
        bset    #0x6,0x13(a6)                   | +1ee
        jsr     0x27cee.l                       | +1f4
        bcs.w   .L030b42                        | +1fa
        jsr     0x27cee.l                       | +1fe
        bcc.w   .L030b6e                        | +204
.L030b42:
        lea     PlayerGrenade_ExplodeGround_030bb6(pc),a1 | +208
        move.l  a1,(a6)                         | +20c
        cmpi.b  #0x38,d0                        | +20e
        beq.w   .L030b64                        | +212
        andi.b  #0xc0,d7                        | +216
        cmpi.b  #0x0,d7                         | +21a
        beq.w   .L030b6e                        | +21e
        cmpi.b  #0xc0,d7                        | +222
        beq.w   .L030b6e                        | +226
.L030b64:
        lea     JmpToScheduler_030bf6(pc),a1    | +22a
        move.l  a1,(a6)                         | +22e
        bra.w   .L030b6e                        | +230
.L030b6e:
        jsr     0x28d70.l                       | +234
        subi.b  #0x1,0x76(a6)                   | +23a
        bne.w   .L030b84                        | +240
        lea     JmpToScheduler_030bf6(pc),a1    | +244
        move.l  a1,(a6)                         | +248
.L030b84:
        jsr     0x283d8.l                       | +24a
        btst    #0x1,0x13(a6)                   | +250
        beq.w   .L030b9a                        | +256
        lea     PlayerGrenade_ExplodeAir_030bd6(pc),a1 | +25a
        move.l  a1,(a6)                         | +25e
.L030b9a:
        movea.l #0xffffffff,a0                  | +260
        lea     PlayerGrenade_GroundTbl_0308a8(pc),a0 | +266
        jsr     0x5dd56.l                       | +26a
        bcc.w   SetHandlerRts_030bb4            | +270

| ----------------------------------------------------------------------------
|  PlayerGrenade_ExplodeGround_030bb6  @ $030BB6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_ExplodeGround_030bb6, "ax", @progbits
        .global PlayerGrenade_ExplodeGround_030bb6
PlayerGrenade_ExplodeGround_030bb6:
        lea     Fx_GroundBurst_031c72(pc),a1    | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x9e(a6),d1                     | +010
        move.w  d1,0x9e(a0)                     | +014

| ----------------------------------------------------------------------------
|  PlayerGrenade_ExplodeAir_030bd6  @ $030BD6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_ExplodeAir_030bd6, "ax", @progbits
        .global PlayerGrenade_ExplodeAir_030bd6
PlayerGrenade_ExplodeAir_030bd6:
        lea     Fx_AirBurst_031cca(pc),a1       | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x9e(a6),d1                     | +010
        move.w  d1,0x9e(a0)                     | +014

| ----------------------------------------------------------------------------
|  PlayerGrenade_AngleFromSpread_030bfe  @ $030BFE  (22 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerGrenade_AngleFromSpread_030bfe, "ax", @progbits
        .global PlayerGrenade_AngleFromSpread_030bfe
PlayerGrenade_AngleFromSpread_030bfe:
        move.w  d0,d1                           | +000
        moveq   #0,d0                           | +002
        move.b  0x9b(a6),d0                     | +004
        andi.w  #0x3,d0                         | +008
        subq.w  #0x2,d0                         | +00c
        add.w   d1,d0                           | +00e
        andi.w  #0xff,d0                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  EnemyShot_Straight_030c14  @ $030C14  (92 B)
| ----------------------------------------------------------------------------
        .section .text.EnemyShot_Straight_030c14, "ax", @progbits
        .global EnemyShot_Straight_030c14
EnemyShot_Straight_030c14:
        move.b  0x98(a6),0x70(a6)               | +000
        move.w  #0x1d6,d1                       | +006
        jsr     0x236e.l                        | +00a
        move.w  #0xd080,0x38(a6)                | +010
        move.b  0x70(a6),d0                     | +016
        andi.w  #0xff,d0                        | +01a
        lsl.w   #0x3,d0                         | +01e
        move.w  #0x14,d1                        | +020
        jsr     0x13c0e.l                       | +024
        add.w   d1,0x22(a6)                     | +02a
        add.w   d2,0x24(a6)                     | +02e
        lea     0x29d7d2.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L030c58(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L030c58:
        jsr     0x2783a.l                       | +044
        jsr     0x28d70.l                       | +04a
        bcc.w   .L030c6e                        | +050
        jmp     0x518.l                         | +054
.L030c6e:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  EnemyShot_Bounce_030c70  @ $030C70  (140 B)
| ----------------------------------------------------------------------------
        .section .text.EnemyShot_Bounce_030c70, "ax", @progbits
        .global EnemyShot_Bounce_030c70
EnemyShot_Bounce_030c70:
        move.w  #0x6,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x400,0x2a(a6)                 | +00a
        move.w  #0xff00,0x2e(a6)                | +010
        jsr     0x5e9b6.l                       | +016
        move.w  d0,d1                           | +01c
        andi.w  #0x140,d1                       | +01e
        addi.w  #0x80,d1                        | +022
        neg.w   d1                              | +026
        asl.w   #0x1,d1                         | +028
        move.w  d1,0x28(a6)                     | +02a
        andi.w  #0x3,d0                         | +02e
        movea.l #0x29d844,a0                    | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L030cbe                        | +044
        jsr     0x28cd4.l                       | +048
.L030cbe:
        lea     .L030cc4(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L030cc4:
        move.w  0x2a(a6),d0                     | +054
        move.w  #0x400,d1                       | +058
        jsr     0x267f4.l                       | +05c
        move.w  d0,0x2a(a6)                     | +062
        jsr     0x27d50.l                       | +066
        bcc.w   .L030ce6                        | +06c
        lea     EnemyShot_BounceB_030d04(pc),a1 | +070
        move.l  a1,(a6)                         | +074
.L030ce6:
        jsr     0x28d70.l                       | +076
        movea.l #0xffffffff,a0                  | +07c
        jsr     0x5dd56.l                       | +082
        bcc.w   SetHandlerRts_030d02            | +088

| ----------------------------------------------------------------------------
|  EnemyShot_BounceB_030d04  @ $030D04  (80 B)
| ----------------------------------------------------------------------------
        .section .text.EnemyShot_BounceB_030d04, "ax", @progbits
        .global EnemyShot_BounceB_030d04
EnemyShot_BounceB_030d04:
        move.w  #0x4c4,0x2a(a6)                 | +000
        move.w  #0xff86,0x2e(a6)                | +006
        move.b  #0xfe,0x59(a6)                  | +00c
        lea     .L030d1c(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L030d1c:
        move.w  0x2a(a6),d0                     | +018
        move.w  #0x400,d1                       | +01c
        jsr     0x267f4.l                       | +020
        move.w  d0,0x2a(a6)                     | +026
        jsr     0x27d50.l                       | +02a
        bcc.w   .L030d3e                        | +030
        lea     JmpToScheduler_030d74(pc),a1    | +034
        move.l  a1,(a6)                         | +038
.L030d3e:
        jsr     0x28d70.l                       | +03a
        movea.l #0xffffffff,a0                  | +040
        jsr     0x5dd56.l                       | +046
        bcc.w   SetHandlerRts_030d5a            | +04c

| ----------------------------------------------------------------------------
|  EnemyShot_ExplodeAir_030d5c  @ $030D5C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.EnemyShot_ExplodeAir_030d5c, "ax", @progbits
        .global EnemyShot_ExplodeAir_030d5c
EnemyShot_ExplodeAir_030d5c:
        lea     Fx_AirBurst_031cca(pc),a1       | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a

| ----------------------------------------------------------------------------
|  VehicleLaunch_AttackTblA_030d7c  @ $030D7C  (164 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_AttackTblA_030d7c, "ax", @progbits
        .global VehicleLaunch_AttackTblA_030d7c
VehicleLaunch_AttackTblA_030d7c:
        .dc.w   0x0302                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0402                        | +050  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_AttackTblB_030e20  @ $030E20  (328 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_AttackTblB_030e20, "ax", @progbits
        .global VehicleLaunch_AttackTblB_030e20
VehicleLaunch_AttackTblB_030e20:
        .dc.w   0x0302                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0402                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff9                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +0de  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0403                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +102  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +108  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +10a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +114  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +116  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +118  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +120  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +122  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +124  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +12e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +132  (dato / opcode no decodificado)
        .dc.w   0x000e                        | +134  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +138  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +140  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +142  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +144  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +146  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashTblA_030f68  @ $030F68  (164 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashTblA_030f68, "ax", @progbits
        .global VehicleLaunch_CrashTblA_030f68
VehicleLaunch_CrashTblA_030f68:
        .dc.w   0x0319                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xffe0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0402                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0064                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xffe0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashTblB_03100c  @ $03100C  (164 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashTblB_03100c, "ax", @progbits
        .global VehicleLaunch_CrashTblB_03100c
VehicleLaunch_CrashTblB_03100c:
        .dc.w   0x031b                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xffd0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0406                        | +050  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xffd0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashTblC_0310b0  @ $0310B0  (164 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashTblC_0310b0, "ax", @progbits
        .global VehicleLaunch_CrashTblC_0310b0
VehicleLaunch_CrashTblC_0310b0:
        .dc.w   0x031a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0032                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xffd0                        | +026  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +032  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0408                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0032                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +060  (dato / opcode no decodificado)
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
        .dc.w   0xffd0                        | +076  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +088  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x00a0                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +094  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +096  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffd0                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +09e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_GroundTbl_031154  @ $031154  (8 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_GroundTbl_031154, "ax", @progbits
        .global VehicleLaunch_GroundTbl_031154
VehicleLaunch_GroundTbl_031154:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffc0                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_OffsetTblA_03115c  @ $03115C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_OffsetTblA_03115c, "ax", @progbits
        .global VehicleLaunch_OffsetTblA_03115c
VehicleLaunch_OffsetTblA_03115c:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +004  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +014  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +018  (dato / opcode no decodificado)
        .dc.w   0x001b                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +022  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +028  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0009                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x00f8                        | +030  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_OffsetTblB_03118e  @ $03118E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_OffsetTblB_03118e, "ax", @progbits
        .global VehicleLaunch_OffsetTblB_03118e
VehicleLaunch_OffsetTblB_03118e:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +014  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +020  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +022  (dato / opcode no decodificado)
        .dc.w   0x003a                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0030                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +028  (dato / opcode no decodificado)
        .dc.w   0xfa8a                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x00f0                        | +030  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleLaunch_Init_0311c0  @ $0311C0  (980 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Init_0311c0, "ax", @progbits
        .global VehicleLaunch_Init_0311c0
VehicleLaunch_Init_0311c0:
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x94(a0),d3                     | +012
        mulu.w  #0xa,d3                         | +016
        lea     VehicleLaunch_OffsetTblA_03115c(pc),a1 | +01a
        lea     0x29fa8a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        move.w  0x4(a1,d3.w),d1                 | +02a
        add.w   d1,0x22(a6)                     | +02e
        move.w  0x6(a1,d3.w),d2                 | +032
        add.w   d2,0x24(a6)                     | +036
        move.w  0x8(a1,d3.w),d0                 | +03a
        move.w  d0,0x34(a6)                     | +03e
        lsr.w   #0x3,d0                         | +042
        move.w  d0,0x30(a6)                     | +044
        move.w  #0x4400,0x72(a6)                | +048
        move.w  #0x600,0x36(a6)                 | +04e
        move.w  #0x700,d1                       | +054
        move.w  0x34(a6),d0                     | +058
        jsr     0x13c0e.l                       | +05c
        move.w  d1,0x28(a6)                     | +062
        move.w  d2,0x2a(a6)                     | +066
        bra.w   .L0314f0                        | +06a
        lea     VehicleLaunch_AttackTblA_030d7c(pc),a0 | +06e
        move.l  a0,0x4c(a6)                     | +072
        jsr     0x283ca.l                       | +076
        bra.w   .L0312c0                        | +07c
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +080
        move.l  a0,0x4c(a6)                     | +084
        jsr     0x283ca.l                       | +088
        lea     0x29fa8a.l,a0                   | +08e
        jsr     0x28cd4.l                       | +094
        movea.l 0xc(a6),a0                      | +09a
        moveq   #0,d0                           | +09e
        move.w  0x80(a0),d0                     | +0a0
        bgt.w   .L031270                        | +0a4
        addi.w  #0x10,d0                        | +0a8
        bra.w   .L031274                        | +0ac
.L031270:
        addi.w  #0x8,d0                         | +0b0
.L031274:
        andi.w  #0xff,d0                        | +0b4
        move.w  d0,0x34(a6)                     | +0b8
        movea.l 0xc(a6),a1                      | +0bc
        movea.l 0x70(a1),a1                     | +0c0
        movea.l (a1),a1                         | +0c4
        move.w  0x8(a1),d1                      | +0c6
        move.w  0xa(a1),d2                      | +0ca
        add.w   d1,0x22(a6)                     | +0ce
        add.w   d2,0x24(a6)                     | +0d2
        move.w  #0x600,0x36(a6)                 | +0d6
        move.w  #0x6000,0x72(a6)                | +0dc
        clr.b   0x20(a6)                        | +0e2
        move.l  #0x31b78,0x74(a6)               | +0e6
        bra.w   .L0314d0                        | +0ee
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +0f2
        move.l  a0,0x4c(a6)                     | +0f6
        jsr     0x283ca.l                       | +0fa
.L0312c0:
        lea     0x29fa8a.l,a0                   | +100
        jsr     0x28cd4.l                       | +106
        move.b  #0xff,0x20(a6)                  | +10c
        move.l  #0x31b78,0x74(a6)               | +112
        move.w  #0x600,0x36(a6)                 | +11a
        move.w  #0x1000,0x72(a6)                | +120
        movea.l 0xc(a6),a0                      | +126
        moveq   #0,d0                           | +12a
        move.w  0x80(a0),d0                     | +12c
        bgt.w   .L0312fc                        | +130
        addi.w  #0x10,d0                        | +134
        bra.w   .L031300                        | +138
.L0312fc:
        addi.w  #0x8,d0                         | +13c
.L031300:
        move.b  0x106f2a.l,d1                   | +140
        cmpi.b  #0x3,d1                         | +146
        bne.w   .L031312                        | +14a
        addi.w  #0x0,d0                         | +14e
.L031312:
        andi.w  #0xff,d0                        | +152
        move.w  d0,0x34(a6)                     | +156
        move.b  0x106f2a.l,d1                   | +15a
        cmpi.b  #0x2,d1                         | +160
        bne.w   .L0313a6                        | +164
        bra.w   .L0313a6                        | +168
        movem.l a6,-(a7)                        | +16c
        movea.l 0xc(a6),a6                      | +170
        jsr     0x5cd90.l                       | +174
        movem.l (a7)+,a6                        | +17a
        bcc.w   .L0313a6                        | +17e
        jsr     Popcount4_0323b4(pc)            | +182
        cmpi.b  #0x4,d0                         | +186
        bne.w   .L031362                        | +18a
        move.w  #0xc00,0x36(a6)                 | +18e
        move.w  #0x7400,0x72(a6)                | +194
        move.w  #0xfff8,d2                      | +19a
        bra.w   .L031394                        | +19e
.L031362:
        cmpi.b  #0x3,d0                         | +1a2
        bne.w   .L03137c                        | +1a6
        move.w  #0xc00,0x36(a6)                 | +1aa
        move.w  #0x7400,0x72(a6)                | +1b0
        clr.w   d2                              | +1b6
        bra.w   .L031394                        | +1b8
.L03137c:
        cmpi.b  #0x2,d0                         | +1bc
        bne.w   .L0313a2                        | +1c0
        move.w  #0xc00,0x36(a6)                 | +1c4
        move.w  #0x7400,0x72(a6)                | +1ca
        move.w  #0x8,d2                         | +1d0
.L031394:
        move.w  0x34(a6),d0                     | +1d4
        sub.w   d2,d0                           | +1d8
        andi.w  #0xf8,d0                        | +1da
        move.w  d0,0x34(a6)                     | +1de
.L0313a2:
        bra.w   .L0314ac                        | +1e2
.L0313a6:
        cmpi.b  #0x5,d1                         | +1e6
        bne.w   .L031428                        | +1ea
        movem.l a6,-(a7)                        | +1ee
        movea.l 0xc(a6),a6                      | +1f2
        jsr     0x5cd90.l                       | +1f6
        movem.l (a7)+,a6                        | +1fc
        bcc.w   .L031428                        | +200
        jsr     Popcount4_0323b4(pc)            | +204
        cmpi.b  #0x4,d0                         | +208
        bne.w   .L0313e4                        | +20c
        move.w  #0xc00,0x36(a6)                 | +210
        move.w  #0x7400,0x72(a6)                | +216
        move.w  #0xfff8,d2                      | +21c
        bra.w   .L031416                        | +220
.L0313e4:
        cmpi.b  #0x3,d0                         | +224
        bne.w   .L0313fe                        | +228
        move.w  #0xc00,0x36(a6)                 | +22c
        move.w  #0x7400,0x72(a6)                | +232
        clr.w   d2                              | +238
        bra.w   .L031416                        | +23a
.L0313fe:
        cmpi.b  #0x2,d0                         | +23e
        bne.w   .L031424                        | +242
        move.w  #0xc00,0x36(a6)                 | +246
        move.w  #0x7400,0x72(a6)                | +24c
        move.w  #0x8,d2                         | +252
.L031416:
        move.w  0x34(a6),d0                     | +256
        sub.w   d2,d0                           | +25a
        andi.w  #0xf8,d0                        | +25c
        move.w  d0,0x34(a6)                     | +260
.L031424:
        bra.w   .L0314ac                        | +264
.L031428:
        cmpi.b  #0x3,d1                         | +268
        bne.w   .L0314ac                        | +26c
        movem.l a6,-(a7)                        | +270
        movea.l 0xc(a6),a6                      | +274
        jsr     0x5cd90.l                       | +278
        movem.l (a7)+,a6                        | +27e
        bcc.w   .L0314ac                        | +282
        jsr     Popcount4_0323b4(pc)            | +286
        bra.w   .L031452                        | +28a
        bra.w   .L0314ac                        | +28e
.L031452:
        movem.l a6,-(a7)                        | +292
        movea.l 0xc(a6),a1                      | +296
        movea.l a1,a6                           | +29a
        jsr     0x2a328.l                       | +29c
        movem.l (a7)+,a6                        | +2a2
        bcc.w   .L0314a8                        | +2a6
        move.w  #0xc00,0x36(a6)                 | +2aa
        move.w  #0x26aa,0x72(a6)                | +2b0
        move.w  0x34(a6),d0                     | +2b6
        subq.w  #0x8,d0                         | +2ba
        andi.w  #0xf8,d0                        | +2bc
        move.w  d0,0x34(a6)                     | +2c0
        move.b  0x106f2a.l,d1                   | +2c4
        cmpi.b  #0x3,d1                         | +2ca
        bne.w   .L031496                        | +2ce
        clr.b   0x20(a6)                        | +2d2
.L031496:
        move.b  0x106f2a.l,d1                   | +2d6
        cmpi.b  #0x2,d1                         | +2dc
        bne.w   .L0314a8                        | +2e0
        clr.b   0x20(a6)                        | +2e4
.L0314a8:
        bra.w   .L0314ac                        | +2e8
.L0314ac:
        movea.l 0xc(a6),a1                      | +2ec
        movea.l 0x70(a1),a1                     | +2f0
        movea.l (a1),a1                         | +2f4
        move.w  0x8(a1),d1                      | +2f6
        move.w  0xa(a1),d2                      | +2fa
        add.w   d1,0x22(a6)                     | +2fe
        addi.w  #0x0,0x22(a6)                   | +302
        add.w   d2,0x24(a6)                     | +308
        addq.w  #0x8,0x24(a6)                   | +30c
.L0314d0:
        move.w  0x34(a6),d1                     | +310
        lsr.w   #0x3,d1                         | +314
        move.w  d1,0x30(a6)                     | +316
        move.w  #0x700,d1                       | +31a
        move.w  0x34(a6),d0                     | +31e
        jsr     0x13c0e.l                       | +322
        move.w  d1,0x28(a6)                     | +328
        move.w  d2,0x2a(a6)                     | +32c
.L0314f0:
        move.w  #0x1084,d0                      | +330
        jsr     0x2352.l                        | +334
        move.w  #0xd000,0x38(a6)                | +33a
        lea     0x77ab6.l,a1                    | +340
        jsr     0x4ae.l                         | +346
        jsr     0x5dd02.l                       | +34c
        move.w  #0x58,d1                        | +352
        jsr     0x236e.l                        | +356
        clr.w   0x70(a6)                        | +35c
        movea.l 0xc(a6),a0                      | +360
        addi.b  #0x1,0x8f(a0)                   | +364
        andi.w  #0xffe3,0x38(a6)                | +36a
        ori.w   #0x0,0x38(a6)                   | +370
        jsr     0x283ca.l                       | +376
        lea     .L031542(pc),a1                 | +37c
        move.l  a1,(a6)                         | +380
.L031542:
        move.w  0x72(a6),d0                     | +382
        move.w  0x70(a6),d1                     | +386
        cmp.w   d0,d1                           | +38a
        bcc.w   .L03156c                        | +38c
        move.w  0x34(a6),d0                     | +390
        move.w  0x36(a6),d1                     | +394
        jsr     0x13c0e.l                       | +398
        move.w  d1,0x28(a6)                     | +39e
        move.w  d2,0x2a(a6)                     | +3a2
        lea     .L03156c(pc),a1                 | +3a6
        move.l  a1,(a6)                         | +3aa
.L03156c:
        move.w  0x28(a6),d0                     | +3ac
        add.w   d0,0x70(a6)                     | +3b0
        move.w  0x72(a6),d0                     | +3b4
        move.w  0x70(a6),d1                     | +3b8
        cmp.w   d0,d1                           | +3bc
        bcs.w   VehicleLaunch_Fall_031594__L0315a2 | +3be
        lea     VehicleLaunch_Fall_031594(pc),a1 | +3c2
        move.l  a1,(a6)                         | +3c6
        move.w  0x36(a6),d0                     | +3c8
        asr.w   #0x4,d0                         | +3cc
        neg.w   d0                              | +3ce
        move.w  d0,0x2e(a6)                     | +3d0

| ----------------------------------------------------------------------------
|  VehicleLaunch_Fall_031594  @ $031594  (238 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Fall_031594, "ax", @progbits
        .global VehicleLaunch_Fall_031594
VehicleLaunch_Fall_031594:
        tst.w   0x2a(a6)                        | +000
        bgt.w   VehicleLaunch_Fall_031594__L0315a2 | +004
        lea     VehicleLaunch_Glide_03168a(pc),a1 | +008
        move.l  a1,(a6)                         | +00c
        .global VehicleLaunch_Fall_031594__L0315a2
VehicleLaunch_Fall_031594__L0315a2:
        move.w  0x28(a6),d0                     | +00e
        move.w  0x2a(a6),d1                     | +012
        asr.w   #0x4,d0                         | +016
        asr.w   #0x4,d1                         | +018
        jsr     0x5e018.l                       | +01a
        lsr.w   #0x3,d0                         | +020
        move.w  d0,0x30(a6)                     | +022
        cmpi.b  #0xff,0x20(a6)                  | +026
        beq.w   .L03161e                        | +02c
        addq.b  #0x1,0x20(a6)                   | +030
        move.b  0x20(a6),d0                     | +034
        cmpi.b  #0x3,0x106f2a.l                 | +038
        bne.w   .L0315e0                        | +040
        andi.b  #0x0,d0                         | +044
        bra.w   .L0315f8                        | +048
.L0315e0:
        cmpi.b  #0x2,0x106f2a.l                 | +04c
        bne.w   .L0315f4                        | +054
        andi.b  #0x0,d0                         | +058
        bra.w   .L0315f8                        | +05c
.L0315f4:
        andi.b  #0xb,d0                         | +060
.L0315f8:
        bne.w   .L03161e                        | +064
        movea.l 0x74(a6),a1                     | +068
        jsr     0x4ae.l                         | +06c
        jsr     0x5dd02.l                       | +072
        move.w  0x34(a6),0x34(a0)               | +078
        move.w  0x30(a6),0x30(a0)               | +07e
        move.w  0x38(a6),0x38(a0)               | +084
.L03161e:
        bset    #0x6,0x13(a6)                   | +08a
        jsr     0x27cee.l                       | +090
        bcc.w   .L031634                        | +096
        lea     VehicleLaunch_JmpCrash_0318b4(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L031634:
        jsr     0x28d70.l                       | +0a0
        jsr     0x283d8.l                       | +0a6
        btst    #0x5,0x5a(a6)                   | +0ac
        beq.w   .L031650                        | +0b2
        lea     VehicleLaunch_Crash_0317d2(pc),a1 | +0b6
        move.l  a1,(a6)                         | +0ba
.L031650:
        btst    #0x1,0x13(a6)                   | +0bc
        beq.w   .L03166e                        | +0c2
        lea     VehicleLaunch_JmpCrash_0318b4(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
        cmpi.b  #0x6,d1                         | +0cc
        bne.w   .L03166e                        | +0d0
        lea     VehicleLaunch_Despawn_0318b8(pc),a1 | +0d4
        move.l  a1,(a6)                         | +0d8
.L03166e:
        movea.l #0xffffffff,a0                  | +0da
        lea     VehicleLaunch_GroundTbl_031154(pc),a0 | +0e0
        jsr     0x5dd56.l                       | +0e4
        bcc.w   SetHandlerRts_031688            | +0ea

| ----------------------------------------------------------------------------
|  VehicleLaunch_Glide_03168a  @ $03168A  (320 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Glide_03168a, "ax", @progbits
        .global VehicleLaunch_Glide_03168a
VehicleLaunch_Glide_03168a:
        cmpi.w  #0x7400,0x72(a6)                | +000
        bne.w   .L0316ac                        | +006
        move.w  #0x600,0x36(a6)                 | +00a
        move.w  0x34(a6),d0                     | +010
        move.w  0x36(a6),d1                     | +014
        jsr     0x13c0e.l                       | +018
        move.w  d1,0x28(a6)                     | +01e
.L0316ac:
        move.w  #0xffe0,0x2c(a6)                | +022
        move.w  #0xfeea,0x2e(a6)                | +028
        lea     .L0316be(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L0316be:
        cmpi.b  #0xff,0x20(a6)                  | +034
        beq.w   .L03171c                        | +03a
        addq.b  #0x1,0x20(a6)                   | +03e
        move.b  0x20(a6),d0                     | +042
        cmpi.b  #0x3,0x106f2a.l                 | +046
        bne.w   .L0316e4                        | +04e
        andi.b  #0x0,d0                         | +052
        bra.w   .L0316fc                        | +056
.L0316e4:
        cmpi.b  #0x2,0x106f2a.l                 | +05a
        bne.w   .L0316f8                        | +062
        andi.b  #0x0,d0                         | +066
        bra.w   .L0316fc                        | +06a
.L0316f8:
        andi.b  #0xb,d0                         | +06e
.L0316fc:
        bne.w   .L03171c                        | +072
        movea.l 0x74(a6),a1                     | +076
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        move.w  0x30(a6),0x30(a0)               | +086
        move.w  0x38(a6),0x38(a0)               | +08c
.L03171c:
        move.w  0x2a(a6),d0                     | +092
        move.w  #0x700,d1                       | +096
        jsr     0x267f4.l                       | +09a
        move.w  d0,0x2a(a6)                     | +0a0
        tst.w   0x28(a6)                        | +0a4
        bgt.w   .L03173e                        | +0a8
        clr.w   0x28(a6)                        | +0ac
        clr.w   0x2c(a6)                        | +0b0
.L03173e:
        bset    #0x6,0x13(a6)                   | +0b4
        jsr     0x27d50.l                       | +0ba
        bcc.w   .L031764                        | +0c0
        lea     VehicleLaunch_JmpCrash_0318b4(pc),a1 | +0c4
        move.l  a1,(a6)                         | +0c8
        jsr     0x27eba.l                       | +0ca
        bcs.w   .L031764                        | +0d0
        lea     VehicleLaunch_Crash_0317d2(pc),a1 | +0d4
        move.l  a1,(a6)                         | +0d8
.L031764:
        move.w  0x28(a6),d0                     | +0da
        move.w  0x2a(a6),d1                     | +0de
        asr.w   #0x4,d0                         | +0e2
        asr.w   #0x4,d1                         | +0e4
        jsr     0x5e018.l                       | +0e6
        lsr.w   #0x3,d0                         | +0ec
        move.w  d0,0x30(a6)                     | +0ee
        jsr     0x28d70.l                       | +0f2
        jsr     0x283d8.l                       | +0f8
        btst    #0x5,0x5a(a6)                   | +0fe
        beq.w   .L031798                        | +104
        lea     VehicleLaunch_Crash_0317d2(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
.L031798:
        btst    #0x1,0x13(a6)                   | +10e
        beq.w   .L0317b6                        | +114
        lea     VehicleLaunch_JmpCrash_0318b4(pc),a1 | +118
        move.l  a1,(a6)                         | +11c
        cmpi.b  #0x6,d1                         | +11e
        bne.w   .L0317b6                        | +122
        lea     VehicleLaunch_Despawn_0318b8(pc),a1 | +126
        move.l  a1,(a6)                         | +12a
.L0317b6:
        movea.l #0xffffffff,a0                  | +12c
        lea     VehicleLaunch_GroundTbl_031154(pc),a0 | +132
        jsr     0x5dd56.l                       | +136
        bcc.w   SetHandlerRts_0317d0            | +13c

| ----------------------------------------------------------------------------
|  VehicleLaunch_Crash_0317d2  @ $0317D2  (76 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Crash_0317d2, "ax", @progbits
        .global VehicleLaunch_Crash_0317d2
VehicleLaunch_Crash_0317d2:
        move.b  #0x1,0x10a2d1.l                 | +000
        move.w  #0x10f2,d0                      | +008
        jsr     0x2352.l                        | +00c
        move.w  #0xd000,0x38(a6)                | +012
        jsr     0x434dc.l                       | +018
        lea     0xffff.w,a0                     | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        movea.l 0xc(a6),a0                      | +02c
        subi.b  #0x1,0x8f(a0)                   | +030
        lea     Fx_SmokePuff_031bd0(pc),a1      | +036
        move.l  a1,(a6)                         | +03a
        lea     VehicleLaunch_CrashA_031826(pc),a1 | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashA_031826  @ $031826  (32 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashA_031826, "ax", @progbits
        .global VehicleLaunch_CrashA_031826
VehicleLaunch_CrashA_031826:
        lea     VehicleLaunch_CrashB_03184e(pc),a1 | +000
        move.l  a1,(a6)                         | +004
        jsr     0x283ca.l                       | +006
        lea     VehicleLaunch_CrashTblA_030f68(pc),a0 | +00c
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        jsr     0x283d8.l                       | +01a

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashB_03184e  @ $03184E  (32 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashB_03184e, "ax", @progbits
        .global VehicleLaunch_CrashB_03184e
VehicleLaunch_CrashB_03184e:
        jsr     0x283ca.l                       | +000
        lea     VehicleLaunch_CrashTblB_03100c(pc),a0 | +006
        move.l  a0,0x4c(a6)                     | +00a
        jsr     0x283ca.l                       | +00e
        jsr     0x283d8.l                       | +014
        lea     VehicleLaunch_CrashC_031876(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e

| ----------------------------------------------------------------------------
|  VehicleLaunch_CrashC_031876  @ $031876  (46 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_CrashC_031876, "ax", @progbits
        .global VehicleLaunch_CrashC_031876
VehicleLaunch_CrashC_031876:
        jsr     0x283ca.l                       | +000
        lea     VehicleLaunch_CrashTblC_0310b0(pc),a0 | +006
        move.l  a0,0x4c(a6)                     | +00a
        jsr     0x283ca.l                       | +00e
        jsr     0x283d8.l                       | +014
        jsr     0x5e4b2.l                       | +01a
        lea     0xffff.w,a0                     | +020
        move.l  a0,0x4c(a6)                     | +024
        jsr     0x283ca.l                       | +028

| ----------------------------------------------------------------------------
|  VehicleLaunch_JmpCrash_0318b4  @ $0318B4  (4 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_JmpCrash_0318b4, "ax", @progbits
        .global VehicleLaunch_JmpCrash_0318b4
VehicleLaunch_JmpCrash_0318b4:
        jmp     VehicleLaunch_Crash_0317d2(pc)  | +000

| ----------------------------------------------------------------------------
|  VehicleLaunch_Despawn_0318b8  @ $0318B8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Despawn_0318b8, "ax", @progbits
        .global VehicleLaunch_Despawn_0318b8
VehicleLaunch_Despawn_0318b8:
        jsr     0x5b6.l                         | +000
        lea     0xffff.w,a0                     | +006
        move.l  a0,0x4c(a6)                     | +00a
        jsr     0x283ca.l                       | +00e

| ----------------------------------------------------------------------------
|  VehicleLaunch_ReleaseParent_0318d4  @ $0318D4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_ReleaseParent_0318d4, "ax", @progbits
        .global VehicleLaunch_ReleaseParent_0318d4
VehicleLaunch_ReleaseParent_0318d4:
        movea.l 0xc(a6),a0                      | +000
        subi.b  #0x1,0x8f(a0)                   | +004
        jmp     0x518.l                         | +00a

| ----------------------------------------------------------------------------
|  VehicleLaunch_Nop_0318e4  @ $0318E4  (2 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_Nop_0318e4, "ax", @progbits
        .global VehicleLaunch_Nop_0318e4
VehicleLaunch_Nop_0318e4:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  VehicleLaunch_InitDrop_0318e6  @ $0318E6  (226 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleLaunch_InitDrop_0318e6, "ax", @progbits
        .global VehicleLaunch_InitDrop_0318e6
VehicleLaunch_InitDrop_0318e6:
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x94(a0),d3                     | +012
        mulu.w  #0xa,d3                         | +016
        lea     VehicleLaunch_OffsetTblA_03115c(pc),a1 | +01a
        lea     VehicleDrop_Template_0319d0(pc),a0 | +01e
        jsr     0x28cd4.l                       | +022
        move.w  0x4(a1,d3.w),d1                 | +028
        add.w   d1,0x22(a6)                     | +02c
        move.w  0x6(a1,d3.w),d2                 | +030
        add.w   d2,0x24(a6)                     | +034
        bra.w   .L03197c                        | +038
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +03c
        move.l  a0,0x4c(a6)                     | +040
        jsr     0x283ca.l                       | +044
        movea.l 0xc(a6),a0                      | +04a
        move.w  0x94(a0),d3                     | +04e
        mulu.w  #0xa,d3                         | +052
        lea     VehicleLaunch_OffsetTblB_03118e(pc),a1 | +056
        bra.w   .L031962                        | +05a
        lea     VehicleLaunch_AttackTblB_030e20(pc),a0 | +05e
        move.l  a0,0x4c(a6)                     | +062
        jsr     0x283ca.l                       | +066
        movea.l 0xc(a6),a0                      | +06c
        move.w  0x94(a0),d3                     | +070
        mulu.w  #0xa,d3                         | +074
        lea     VehicleLaunch_OffsetTblA_03115c(pc),a1 | +078
.L031962:
        lea     VehicleDrop_Template_0319d0(pc),a0 | +07c
        jsr     0x28cd4.l                       | +080
        move.w  0x4(a1,d3.w),d1                 | +086
        add.w   d1,0x22(a6)                     | +08a
        move.w  0x6(a1,d3.w),d2                 | +08e
        add.w   d2,0x24(a6)                     | +092
.L03197c:
        move.w  #0x1084,d0                      | +096
        jsr     0x2352.l                        | +09a
        movea.l 0xc(a6),a0                      | +0a0
        addi.b  #0x1,0x8f(a0)                   | +0a4
        jsr     0x283ca.l                       | +0aa
        lea     .L03199c(pc),a1                 | +0b0
        move.l  a1,(a6)                         | +0b4
.L03199c:
        jsr     0x2783a.l                       | +0b6
        jsr     0x28d70.l                       | +0bc
        bcc.w   .L0319b2                        | +0c2
        lea     VehicleLaunch_ReleaseParent_0318d4(pc),a1 | +0c6
        move.l  a1,(a6)                         | +0ca
.L0319b2:
        jsr     0x283d8.l                       | +0cc
        movea.l #0xffffffff,a0                  | +0d2
        jsr     0x5dd56.l                       | +0d8
        bcc.w   SetHandlerRts_0319ce            | +0de

| ----------------------------------------------------------------------------
|  VehicleDrop_Template_0319d0  @ $0319D0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleDrop_Template_0319d0, "ax", @progbits
        .global VehicleDrop_Template_0319d0
VehicleDrop_Template_0319d0:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  VehicleDrop_SpriteTbl_0319dc  @ $0319DC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.VehicleDrop_SpriteTbl_0319dc, "ax", @progbits
        .global VehicleDrop_SpriteTbl_0319dc
VehicleDrop_SpriteTbl_0319dc:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdca4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd84                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +008  (dato / opcode no decodificado)
        .dc.w   0xde68                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xdf4c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe030                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  SlugFx_ExhaustOrDrop_0319f0  @ $0319F0  (188 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_ExhaustOrDrop_0319f0, "ax", @progbits
        .global SlugFx_ExhaustOrDrop_0319f0
SlugFx_ExhaustOrDrop_0319f0:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x2,0x8d(a0)                   | +004
        beq.w   .L031a56                        | +00a
        movea.l 0x70(a0),a1                     | +00e
        movea.l (a1),a1                         | +012
        move.w  0x8(a1),d1                      | +014
        move.w  0xa(a1),d2                      | +018
        add.w   d1,0x22(a6)                     | +01c
        add.w   d2,0x24(a6)                     | +020
        move.w  0x80(a0),d0                     | +024
        cmpi.w  #0x20,d0                        | +028
        ble.w   .L031a24                        | +02c
        move.w  #0x40,d0                        | +030
.L031a24:
        cmpi.w  #0xffe0,d0                      | +034
        bge.w   .L031a30                        | +038
        move.w  #0xffe0,d0                      | +03c
.L031a30:
        andi.w  #0xff,d0                        | +040
        move.w  #0x10,d1                        | +044
        jsr     0x13c0e.l                       | +048
        add.w   d1,0x22(a6)                     | +04e
        add.w   d2,0x24(a6)                     | +052
        lea     0x29e440.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a86 | +062
.L031a56:
        movea.l 0xc(a6),a1                      | +066
        move.w  0x94(a1),0x34(a6)               | +06a
        move.w  0x94(a1),0x64(a6)               | +070
        move.w  0x94(a1),d0                     | +076
        movea.l #0x319dc,a0                     | +07a
        lsl.w   #0x2,d0                         | +080
        movea.l (a0,d0.w),a0                    | +082
        cmpa.l  #0xffffffff,a0                  | +086
        beq.w   SlugFx_ExhaustOrDrop_0319f0__L031a86 | +08c
        jsr     0x28cd4.l                       | +090
        .global SlugFx_ExhaustOrDrop_0319f0__L031a86
SlugFx_ExhaustOrDrop_0319f0__L031a86:
        move.w  #0x8,d1                         | +096
        jsr     0x236e.l                        | +09a
        subi.w  #0x4,0x22(a6)                   | +0a0
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +0a6
        move.l  a1,(a6)                         | +0aa
        .global SlugFx_ExhaustOrDrop_0319f0__L031a9c
SlugFx_ExhaustOrDrop_0319f0__L031a9c:
        jsr     0x2783a.l                       | +0ac
        jsr     0x28d70.l                       | +0b2
        bcc.w   Jsr5B6Rts_031ab8                | +0b8

| ----------------------------------------------------------------------------
|  SlugFx_ExhaustSpriteTbl_031aba  @ $031ABA  (60 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_ExhaustSpriteTbl_031aba, "ax", @progbits
        .global SlugFx_ExhaustSpriteTbl_031aba
SlugFx_ExhaustSpriteTbl_031aba:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe11c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe1c0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +010  (dato / opcode no decodificado)
        .dc.w   0xe244                        | +012  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +018  (dato / opcode no decodificado)
        .dc.w   0xe440                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xfffb                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x003a                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +020  (dato / opcode no decodificado)
        .dc.w   0xe440                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0019                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +028  (dato / opcode no decodificado)
        .dc.w   0xe11c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xe1c0                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +030  (dato / opcode no decodificado)
        .dc.w   0xe230                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +034  (dato / opcode no decodificado)
        .dc.w   0xe2b4                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +038  (dato / opcode no decodificado)
        .dc.w   0xe37a                        | +03a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  SlugFx_Exhaust_031af6  @ $031AF6  (92 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_Exhaust_031af6, "ax", @progbits
        .global SlugFx_Exhaust_031af6
SlugFx_Exhaust_031af6:
        movea.l 0xc(a6),a0                      | +000
        movea.l 0x70(a0),a1                     | +004
        movea.l (a1),a1                         | +008
        move.w  0x8(a1),d1                      | +00a
        move.w  0xa(a1),d2                      | +00e
        add.w   d1,0x22(a6)                     | +012
        add.w   d2,0x24(a6)                     | +016
        move.w  0x80(a0),d0                     | +01a
        cmpi.w  #0x20,d0                        | +01e
        ble.w   .L031b20                        | +022
        move.w  #0x40,d0                        | +026
.L031b20:
        cmpi.w  #0xffe0,d0                      | +02a
        bge.w   .L031b2c                        | +02e
        move.w  #0xffe0,d0                      | +032
.L031b2c:
        andi.w  #0xff,d0                        | +036
        move.w  #0x10,d1                        | +03a
        jsr     0x13c0e.l                       | +03e
        add.w   d1,0x22(a6)                     | +044
        add.w   d2,0x24(a6)                     | +048
        lea     0x29e440.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a86 | +058

| ----------------------------------------------------------------------------
|  Fx_Sparkle_031b52  @ $031B52  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_Sparkle_031b52, "ax", @progbits
        .global Fx_Sparkle_031b52
Fx_Sparkle_031b52:
        move.w  #0x58,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x29fcce.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x2,0x70(a6)                   | +016
        lea     Fx_Sparkle_Tick_031b98(pc),a1   | +01c
        move.l  a1,(a6)                         | +020
        bra.w   Fx_Sparkle_Tick_031b98          | +022
        move.w  #0x4,d1                         | +026
        jsr     0x236e.l                        | +02a
        lea     Fx_SparkleAnim_0323cc(pc),a0    | +030
        jsr     0x28cd4.l                       | +034
        move.w  #0x2,0x70(a6)                   | +03a
        lea     Fx_Sparkle_Tick_031b98(pc),a1   | +040
        move.l  a1,(a6)                         | +044

| ----------------------------------------------------------------------------
|  Fx_Sparkle_Tick_031b98  @ $031B98  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_Sparkle_Tick_031b98, "ax", @progbits
        .global Fx_Sparkle_Tick_031b98
Fx_Sparkle_Tick_031b98:
        subi.w  #0x1,0x38(a6)                   | +000
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        tst.b   0x46(a6)                        | +012
        bne.w   .L031bb8                        | +016
        subi.b  #0x1,0x46(a6)                   | +01a
.L031bb8:
        subi.w  #0x1,0x70(a6)                   | +020
        bgt.w   Jsr5B6Rts_031bce                | +026

| ----------------------------------------------------------------------------
|  Fx_SmokePuff_031bd0  @ $031BD0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_SmokePuff_031bd0, "ax", @progbits
        .global Fx_SmokePuff_031bd0
Fx_SmokePuff_031bd0:
        move.w  #0x178,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x29e64a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x2000,0x38(a6)                | +016
        jsr     0x5e9b6.l                       | +01c
        andi.w  #0xf,d0                         | +022
        subq.w  #0x8,d0                         | +026
        add.w   d0,0x22(a6)                     | +028
        lea     .L031c02(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L031c02:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   Jsr5B6Rts_031c1e                | +03e

| ----------------------------------------------------------------------------
|  Fx_DustCloud_031c20  @ $031C20  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_DustCloud_031c20, "ax", @progbits
        .global Fx_DustCloud_031c20
Fx_DustCloud_031c20:
        move.w  #0x5,d1                         | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0xf,d0                         | +010
        subq.w  #0x8,d0                         | +014
        add.w   d0,0x22(a6)                     | +016
        lea     0x29e568.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L031c4c(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L031c4c:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   Jsr5B6Rts_031c68                | +038

| ----------------------------------------------------------------------------
|  Fx_GroundBurst_SpriteTbl_031c6a  @ $031C6A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_GroundBurst_SpriteTbl_031c6a, "ax", @progbits
        .global Fx_GroundBurst_SpriteTbl_031c6a
Fx_GroundBurst_SpriteTbl_031c6a:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xda80                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdb0e                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Fx_GroundBurst_031c72  @ $031C72  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_GroundBurst_031c72, "ax", @progbits
        .global Fx_GroundBurst_031c72
Fx_GroundBurst_031c72:
        move.w  0x9e(a6),d1                     | +000
        cmpi.w  #0x1d6,d1                       | +004
        bne.w   .L031c86                        | +008
        move.w  #0x1d6,d1                       | +00c
        bra.w   .L031c8a                        | +010
.L031c86:
        move.w  #0x4,d1                         | +014
.L031c8a:
        jsr     0x236e.l                        | +018
        jsr     0x5e9b6.l                       | +01e
        andi.w  #0xf,d0                         | +024
        subq.w  #0x8,d0                         | +028
        add.w   d0,0x22(a6)                     | +02a
        andi.w  #0x1,d0                         | +02e
        movea.l #0x31c6a,a0                     | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L031cc0                        | +044
        jsr     0x28cd4.l                       | +048
.L031cc0:
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +04e
        move.l  a1,(a6)                         | +052
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +054

| ----------------------------------------------------------------------------
|  Fx_AirBurst_031cca  @ $031CCA  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_AirBurst_031cca, "ax", @progbits
        .global Fx_AirBurst_031cca
Fx_AirBurst_031cca:
        move.w  0x9e(a6),d1                     | +000
        cmpi.w  #0x1d6,d1                       | +004
        bne.w   .L031cde                        | +008
        move.w  #0x1d6,d1                       | +00c
        bra.w   .L031ce2                        | +010
.L031cde:
        move.w  #0x2,d1                         | +014
.L031ce2:
        jsr     0x236e.l                        | +018
        jsr     0x5e9b6.l                       | +01e
        andi.w  #0x1f,d0                        | +024
        subq.w  #0x8,d0                         | +028
        add.w   d0,0x22(a6)                     | +02a
        jsr     0x5e9b6.l                       | +02e
        andi.w  #0x1f,d0                        | +034
        subq.w  #0x8,d0                         | +038
        add.w   d0,0x24(a6)                     | +03a
        lea     0x2dd8b6.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +050

| ----------------------------------------------------------------------------
|  Fx_GroundBurstB_SpriteTbl_031d1e  @ $031D1E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_GroundBurstB_SpriteTbl_031d1e, "ax", @progbits
        .global Fx_GroundBurstB_SpriteTbl_031d1e
Fx_GroundBurstB_SpriteTbl_031d1e:
        .dc.w   0x0029                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdb88                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0029                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdc16                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Fx_GroundBurstB_031d26  @ $031D26  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_GroundBurstB_031d26, "ax", @progbits
        .global Fx_GroundBurstB_031d26
Fx_GroundBurstB_031d26:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0xf,d0                         | +010
        subq.w  #0x8,d0                         | +014
        add.w   d0,0x22(a6)                     | +016
        andi.w  #0x1,d0                         | +01a
        movea.l #0x31d1e,a0                     | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L031d60                        | +030
        jsr     0x28cd4.l                       | +034
.L031d60:
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +040

| ----------------------------------------------------------------------------
|  Fx_Spark_031d6a  @ $031D6A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_Spark_031d6a, "ax", @progbits
        .global Fx_Spark_031d6a
Fx_Spark_031d6a:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  Fx_SparkAnim_031d70  @ $031D70  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_SparkAnim_031d70, "ax", @progbits
        .global Fx_SparkAnim_031d70
Fx_SparkAnim_031d70:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x29e46a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +01c

| ----------------------------------------------------------------------------
|  Entity_SpawnSparkIfBit1_031d90  @ $031D90  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SpawnSparkIfBit1_031d90, "ax", @progbits
        .global Entity_SpawnSparkIfBit1_031d90
Entity_SpawnSparkIfBit1_031d90:
        btst    #0x1,0x8c(a6)                   | +000
        beq.w   .L031da8                        | +006
        bclr    #0x1,0x8c(a6)                   | +00a
        ori.b   #0x11,ccr                       | +010
        bra.w   .L031dac                        | +014
.L031da8:
        andi.b  #0xee,ccr                       | +018
.L031dac:
        bcc.w   JsrAbsRts_031dc0                | +01c
        lea     Fx_Spark_031d6a(pc),a1          | +020
        jsr     0x4ae.l                         | +024

| ----------------------------------------------------------------------------
|  Entity_SpawnSparkLeftIfBit1_031dc2  @ $031DC2  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SpawnSparkLeftIfBit1_031dc2, "ax", @progbits
        .global Entity_SpawnSparkLeftIfBit1_031dc2
Entity_SpawnSparkLeftIfBit1_031dc2:
        btst    #0x1,0x8c(a6)                   | +000
        beq.w   .L031dda                        | +006
        bclr    #0x1,0x8c(a6)                   | +00a
        ori.b   #0x11,ccr                       | +010
        bra.w   .L031dde                        | +014
.L031dda:
        andi.b  #0xee,ccr                       | +018
.L031dde:
        bcc.b   JsrAbsRts_031dc0                | +01c
        lea     Fx_Spark_031d6a(pc),a1          | +01e
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        addi.w  #0xffff,0x22(a0)                | +02e
        addi.w  #0xfffd,0x24(a0)                | +034
        bra.b   JsrAbsRts_031dc0                | +03a

| ----------------------------------------------------------------------------
|  Entity_SpawnSparkRightIfBit1_031dfe  @ $031DFE  (58 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_SpawnSparkRightIfBit1_031dfe, "ax", @progbits
        .global Entity_SpawnSparkRightIfBit1_031dfe
Entity_SpawnSparkRightIfBit1_031dfe:
        btst    #0x1,0x8c(a6)                   | +000
        beq.w   .L031e16                        | +006
        bclr    #0x1,0x8c(a6)                   | +00a
        ori.b   #0x11,ccr                       | +010
        bra.w   .L031e1a                        | +014
.L031e16:
        andi.b  #0xee,ccr                       | +018
.L031e1a:
        bcc.b   JsrAbsRts_031dc0                | +01c
        lea     Fx_Spark_031d6a(pc),a1          | +01e
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        addq.w  #0x1,0x22(a0)                   | +02e
        addi.w  #0xfffd,0x24(a0)                | +032
        bra.b   JsrAbsRts_031dc0                | +038

| ----------------------------------------------------------------------------
|  Fx_Dust_031e38  @ $031E38  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_Dust_031e38, "ax", @progbits
        .global Fx_Dust_031e38
Fx_Dust_031e38:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  Fx_DustAnim_031e3e  @ $031E3E  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_DustAnim_031e3e, "ax", @progbits
        .global Fx_DustAnim_031e3e
Fx_DustAnim_031e3e:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd72a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +01c

| ----------------------------------------------------------------------------
|  Fx_SpawnDustPair_031e5e  @ $031E5E  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_SpawnDustPair_031e5e, "ax", @progbits
        .global Fx_SpawnDustPair_031e5e
Fx_SpawnDustPair_031e5e:
        lea     Fx_Dust_031e38(pc),a1           | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0xffe0,0x22(a0)                | +010
.L031e74:
        rts                                     | +016
        lea     Fx_Dust_031e38(pc),a1           | +018
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        addi.w  #0x20,0x22(a0)                  | +028
        eori.b  #0x1,0x3a(a0)                   | +02e
        bra.b   .L031e74                        | +034
        lea     Fx_Dust_031e38(pc),a1           | +036
        jsr     0x4ae.l                         | +03a
        jsr     0x5dd02.l                       | +040
        addi.w  #0xffe0,0x22(a0)                | +046
        addi.w  #0xfff8,0x24(a0)                | +04c
        bra.b   .L031e74                        | +052
        lea     Fx_Dust_031e38(pc),a1           | +054
        jsr     0x4ae.l                         | +058
        jsr     0x5dd02.l                       | +05e
        addi.w  #0x20,0x22(a0)                  | +064
        addi.w  #0xfff8,0x24(a0)                | +06a
        eori.b  #0x1,0x3a(a0)                   | +070
        bra.b   .L031e74                        | +076
        lea     Fx_Dust_031e38(pc),a1           | +078
        jsr     0x4ae.l                         | +07c
        jsr     0x5dd02.l                       | +082
        addi.w  #0xffe0,0x22(a0)                | +088
        addq.w  #0x8,0x24(a0)                   | +08e
        bra.b   .L031e74                        | +092
        lea     Fx_Dust_031e38(pc),a1           | +094
        jsr     0x4ae.l                         | +098
        jsr     0x5dd02.l                       | +09e
        addi.w  #0x20,0x22(a0)                  | +0a4
        addq.w  #0x8,0x24(a0)                   | +0aa
        eori.b  #0x1,0x3a(a0)                   | +0ae
        bra.w   .L031e74                        | +0b4
        lea     Fx_Dust_031e38(pc),a1           | +0b8
        jsr     0x4ae.l                         | +0bc
        jsr     0x5dd02.l                       | +0c2
        addi.w  #0xffed,0x22(a0)                | +0c8
        addi.w  #0xfff4,0x24(a0)                | +0ce
        bra.w   .L031e74                        | +0d4
        lea     Fx_Dust_031e38(pc),a1           | +0d8
        jsr     0x4ae.l                         | +0dc
        jsr     0x5dd02.l                       | +0e2
        addi.w  #0x13,0x22(a0)                  | +0e8
        addi.w  #0xfff4,0x24(a0)                | +0ee
        eori.b  #0x1,0x3a(a0)                   | +0f4
        bra.w   .L031e74                        | +0fa
        lea     Fx_Dust_031e38(pc),a1           | +0fe
        jsr     0x4ae.l                         | +102
        jsr     0x5dd02.l                       | +108
        addi.w  #0xffed,0x22(a0)                | +10e
        addi.w  #0xc,0x24(a0)                   | +114
        bra.w   .L031e74                        | +11a
        lea     Fx_Dust_031e38(pc),a1           | +11e
        jsr     0x4ae.l                         | +122
        jsr     0x5dd02.l                       | +128
        addi.w  #0x13,0x22(a0)                  | +12e
        addi.w  #0xc,0x24(a0)                   | +134
        eori.b  #0x1,0x3a(a0)                   | +13a
        bra.w   .L031e74                        | +140

| ----------------------------------------------------------------------------
|  Fx_SmokeAnim_031fa2  @ $031FA2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_SmokeAnim_031fa2, "ax", @progbits
        .global Fx_SmokeAnim_031fa2
Fx_SmokeAnim_031fa2:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd944.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     SlugFx_ExhaustOrDrop_0319f0__L031a9c(pc),a1 | +016
        move.l  a1,(a6)                         | +01a
        bra.w   SlugFx_ExhaustOrDrop_0319f0__L031a9c | +01c

| ----------------------------------------------------------------------------
|  Entity_TestBit2Field8C_031fc2  @ $031FC2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_TestBit2Field8C_031fc2, "ax", @progbits
        .global Entity_TestBit2Field8C_031fc2
Entity_TestBit2Field8C_031fc2:
        btst    #0x2,0x8c(a0)                   | +000
        rts                                     | +006

| ----------------------------------------------------------------------------
|  PlayerIcon_Pow_031fca  @ $031FCA  (158 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_Pow_031fca, "ax", @progbits
        .global PlayerIcon_Pow_031fca
PlayerIcon_Pow_031fca:
        move.w  #0x1ad,d1                       | +000
        jsr     0x236e.l                        | +004
        cmpi.b  #0x5,0x106ece.l                 | +00a
        bne.w   .L031fee                        | +012
        lea     PlayerIcon_PowAltAnim_032286(pc),a0 | +016
        jsr     0x28cd4.l                       | +01a
        bra.w   .L031ff8                        | +020
.L031fee:
        lea     PlayerIcon_PowAnim_032292(pc),a0 | +024
        jsr     0x28cd4.l                       | +028
.L031ff8:
        move.w  #0x28,0x5c(a6)                  | +02e
        move.w  #0xffff,0x38(a6)                | +034
        bset    #0x6,0x12(a6)                   | +03a
        lea     .L032010(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L032010:
        jsr     0x5e4ca.l                       | +046
        move.w  d1,0x22(a6)                     | +04c
        add.w   0x5c(a6),d2                     | +050
        move.w  d2,0x24(a6)                     | +054
        move.w  d0,0x38(a6)                     | +058
        movea.l 0xc(a6),a0                      | +05c
        cmpi.b  #0x0,0x106ed3.l                 | +060
        beq.w   .L03205a                        | +068
        btst    #0x5,0x8d(a0)                   | +06c
        beq.w   .L03205a                        | +072
        movea.l #0xffffffff,a0                  | +076
        lea     PlayerIcon_GroundTbl_032070(pc),a0 | +07c
        jsr     0x5dd56.l                       | +080
        bcs.w   .L03205a                        | +086
        jsr     0x28d70.l                       | +08a
.L03205a:
        movea.l 0xc(a6),a0                      | +090
        btst    #0x0,0x13(a0)                   | +094
        beq.w   SetHandlerRts_03206e            | +09a

| ----------------------------------------------------------------------------
|  PlayerIcon_GroundTbl_032070  @ $032070  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_GroundTbl_032070, "ax", @progbits
        .global PlayerIcon_GroundTbl_032070
PlayerIcon_GroundTbl_032070:
        .dc.w   0xfff0                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +002  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffc8                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerIcon_Bubble_03207c  @ $03207C  (80 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_Bubble_03207c, "ax", @progbits
        .global PlayerIcon_Bubble_03207c
PlayerIcon_Bubble_03207c:
        move.w  #0x1b0,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     PlayerIcon_BubbleAnim_03232a(pc),a0 | +00a
        jsr     0x28cd4.l                       | +00e
        move.w  #0x38,0x5c(a6)                  | +014
        bra.w   .L03209a                        | +01a
.L03209a:
        move.w  #0xffff,0x38(a6)                | +01e
        bset    #0x6,0x12(a6)                   | +024
        lea     .L0320ac(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L0320ac:
        jsr     0x5e4ca.l                       | +030
        move.w  d1,0x22(a6)                     | +036
        add.w   0x5c(a6),d2                     | +03a
        move.w  d2,0x24(a6)                     | +03e
        move.w  d0,0x38(a6)                     | +042
        jsr     0x28d70.l                       | +046
        bcc.w   SetHandlerRts_0320d2            | +04c

| ----------------------------------------------------------------------------
|  PlayerIcon_SpawnFreeFallBoth_0320d4  @ $0320D4  (62 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_SpawnFreeFallBoth_0320d4, "ax", @progbits
        .global PlayerIcon_SpawnFreeFallBoth_0320d4
PlayerIcon_SpawnFreeFallBoth_0320d4:
        movem.l a6,-(a7)                        | +000
        move.w  #0x0,d0                         | +004
        jsr     0x5e3a2.l                       | +008
        bcc.w   .L0320f2                        | +00e
        movea.l a0,a6                           | +012
        lea     PlayerIcon_FreeFallP1_032112(pc),a1 | +014
        jsr     0x4ae.l                         | +018
.L0320f2:
        move.w  #0x1,d0                         | +01e
        jsr     0x5e3a2.l                       | +022
        bcc.w   .L03210c                        | +028
        movea.l a0,a6                           | +02c
        lea     PlayerIcon_FreeFallP2_032142(pc),a1 | +02e
        jsr     0x4ae.l                         | +032
.L03210c:
        movem.l (a7)+,a6                        | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  PlayerIcon_FreeFallP1_032112  @ $032112  (48 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_FreeFallP1_032112, "ax", @progbits
        .global PlayerIcon_FreeFallP1_032112
PlayerIcon_FreeFallP1_032112:
        cmpi.l  #0x100440,0xc(a6)               | +000
        beq.w   .L032124                        | +008
        jmp     0x518.l                         | +00c
.L032124:
        move.w  #0xa2,d1                        | +012
        jsr     0x236e.l                        | +016
        lea     PlayerIcon_FreeFallP1_Anim_0321c2(pc),a0 | +01c
        jsr     0x28cd4.l                       | +020
        move.w  #0x30,0x5c(a6)                  | +026
        bra.w   PlayerIcon_FreeFallP2_032142__L03216e | +02c

| ----------------------------------------------------------------------------
|  PlayerIcon_FreeFallP2_032142  @ $032142  (114 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_FreeFallP2_032142, "ax", @progbits
        .global PlayerIcon_FreeFallP2_032142
PlayerIcon_FreeFallP2_032142:
        cmpi.l  #0x1004e0,0xc(a6)               | +000
        beq.w   .L032154                        | +008
        jmp     0x518.l                         | +00c
.L032154:
        move.w  #0xa3,d1                        | +012
        jsr     0x236e.l                        | +016
        lea     PlayerIcon_FreeFallP2_Anim_032224(pc),a0 | +01c
        jsr     0x28cd4.l                       | +020
        move.w  #0x38,0x5c(a6)                  | +026
        .global PlayerIcon_FreeFallP2_032142__L03216e
PlayerIcon_FreeFallP2_032142__L03216e:
        move.w  #0xffff,0x38(a6)                | +02c
        bset    #0x6,0x12(a6)                   | +032
        lea     .L032180(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L032180:
        jsr     0x5e4ca.l                       | +03e
        move.w  d1,0x22(a6)                     | +044
        add.w   0x5c(a6),d2                     | +048
        move.w  d2,0x24(a6)                     | +04c
        move.w  d0,0x38(a6)                     | +050
        jsr     0x28d70.l                       | +054
        bcc.w   .L0321a6                        | +05a
        lea     JmpAbsThunk_0321bc(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L0321a6:
        movea.l 0xc(a6),a0                      | +064
        btst    #0x0,0x13(a0)                   | +068
        beq.w   SetHandlerRts_0321ba            | +06e

| ----------------------------------------------------------------------------
|  PlayerIcon_FreeFallP1_Anim_0321c2  @ $0321C2  (98 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_FreeFallP1_Anim_0321c2, "ax", @progbits
        .global PlayerIcon_FreeFallP1_Anim_0321c2
PlayerIcon_FreeFallP1_Anim_0321c2:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0988                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0998                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x09a8                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x09b8                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x09c8                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0x09d8                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0x09e8                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x09f8                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +052  (dato / opcode no decodificado)
        .dc.w   0x21c2                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +060  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerIcon_FreeFallP2_Anim_032224  @ $032224  (98 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_FreeFallP2_Anim_032224, "ax", @progbits
        .global PlayerIcon_FreeFallP2_Anim_032224
PlayerIcon_FreeFallP2_Anim_032224:
        .dc.w   0x0004                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0a08                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0a18                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a28                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0a38                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0a48                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0a58                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0a68                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0a78                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +052  (dato / opcode no decodificado)
        .dc.w   0x2224                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +058  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +060  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerIcon_PowAltAnim_032286  @ $032286  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_PowAltAnim_032286, "ax", @progbits
        .global PlayerIcon_PowAltAnim_032286
PlayerIcon_PowAltAnim_032286:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +00a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerIcon_PowAnim_032292  @ $032292  (152 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_PowAnim_032292, "ax", @progbits
        .global PlayerIcon_PowAnim_032292
PlayerIcon_PowAnim_032292:
        .dc.w   0x000a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0a88                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0a94                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0aa0                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0aac                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0abc                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0acc                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0adc                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0aec                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0afc                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0b10                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0b1c                        | +07e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0303                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +084  (dato / opcode no decodificado)
        .dc.w   0x229c                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +08a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +094  (dato / opcode no decodificado)
        .dc.w   0x2292                        | +096  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerIcon_BubbleAnim_03232a  @ $03232A  (138 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerIcon_BubbleAnim_03232a, "ax", @progbits
        .global PlayerIcon_BubbleAnim_03232a
PlayerIcon_BubbleAnim_03232a:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0b28                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0b34                        | +01a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0b40                        | +024  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0b4c                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0b5c                        | +038  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0b6c                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0b7c                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0b8c                        | +056  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0ba0                        | +060  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0bb4                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0025                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0bc0                        | +074  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0302                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x232a                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0208                        | +080  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +086  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +088  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Popcount4_0323b4  @ $0323B4  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Popcount4_0323b4, "ax", @progbits
        .global Popcount4_0323b4
Popcount4_0323b4:
        moveq   #0,d1                           | +000
        moveq   #0,d2                           | +002
        lsl.b   #0x1,d0                         | +004
        addx.b  d2,d1                           | +006
        lsl.b   #0x1,d0                         | +008
        addx.b  d2,d1                           | +00a
        lsl.b   #0x1,d0                         | +00c
        addx.b  d2,d1                           | +00e
        lsl.b   #0x1,d0                         | +010
        addx.b  d2,d1                           | +012
        move.b  d1,d0                           | +014
        rts                                     | +016

| ----------------------------------------------------------------------------
|  Fx_SparkleAnim_0323cc  @ $0323CC  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Fx_SparkleAnim_0323cc, "ax", @progbits
        .global Fx_SparkleAnim_0323cc
Fx_SparkleAnim_0323cc:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +004  (dato / opcode no decodificado)
        .dc.w   0x5c28                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +012  (dato / opcode no decodificado)
        .dc.w   0x5c46                        | +014  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_CmpDepthWithLink8B_0323e8  @ $0323E8  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpDepthWithLink8B_0323e8, "ax", @progbits
        .global Entity_CmpDepthWithLink8B_0323e8
Entity_CmpDepthWithLink8B_0323e8:
        move.b  d0,d3                           | +000
        movea.l 0x8(a6),a1                      | +002
        move.b  0x10(a6),d0                     | +006
        cmp.b   0x10(a1),d0                     | +00a
        bcs.w   SetXN_032400                    | +00e

| ----------------------------------------------------------------------------
|  Player_WeaponStateByteTbl_032412  @ $032412  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WeaponStateByteTbl_032412, "ax", @progbits
        .global Player_WeaponStateByteTbl_032412
Player_WeaponStateByteTbl_032412:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0101                        | +0a8  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_GroundTblA_0324bc  @ $0324BC  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GroundTblA_0324bc, "ax", @progbits
        .global Player_GroundTblA_0324bc
Player_GroundTblA_0324bc:
        .dc.w   0xffe0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +002  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_GroundTblB_0324c6  @ $0324C6  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GroundTblB_0324c6, "ax", @progbits
        .global Player_GroundTblB_0324c6
Player_GroundTblB_0324c6:
        .dc.w   0xffe0                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +002  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0040                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_VelYTbl_0324d0  @ $0324D0  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Player_VelYTbl_0324d0, "ax", @progbits
        .global Player_VelYTbl_0324d0
Player_VelYTbl_0324d0:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfe80                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0180                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_VelYTblB_0324d8  @ $0324D8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Player_VelYTblB_0324d8, "ax", @progbits
        .global Player_VelYTblB_0324d8
Player_VelYTblB_0324d8:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xff40                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x00c0                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_VelXTbl_0324e8  @ $0324E8  (252 B)
| ----------------------------------------------------------------------------
        .section .text.Player_VelXTbl_0324e8, "ax", @progbits
        .global Player_VelXTbl_0324e8
Player_VelXTbl_0324e8:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfe00                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfd00                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0300                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0154                        | +012  (dato / opcode no decodificado)
        .dc.w   0xfeac                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01c  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +058  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +062  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +098  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +0da  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0fa  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_AttackTblA_0325e4  @ $0325E4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_AttackTblA_0325e4, "ax", @progbits
        .global Player_AttackTblA_0325e4
Player_AttackTblA_0325e4:
        .dc.w   0x030b                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_AttackTblB_032638  @ $032638  (168 B)
| ----------------------------------------------------------------------------
        .section .text.Player_AttackTblB_032638, "ax", @progbits
        .global Player_AttackTblB_032638
Player_AttackTblB_032638:
        .dc.w   0x030a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0032                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0404                        | +004  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0028                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0320                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0304                        | +058  (dato / opcode no decodificado)
        .dc.w   0xff00                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +068  (dato / opcode no decodificado)
        .dc.w   0x363a                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x3644                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x364e                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x3658                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +098  (dato / opcode no decodificado)
        .dc.w   0x3662                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x1d00                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxStand_0326e0  @ $0326E0  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxStand_0326e0, "ax", @progbits
        .global Player_HitboxStand_0326e0
Player_HitboxStand_0326e0:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2412                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxCrouch_032734  @ $032734  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxCrouch_032734, "ax", @progbits
        .global Player_HitboxCrouch_032734
Player_HitboxCrouch_032734:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2412                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxMelee_032788  @ $032788  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxMelee_032788, "ax", @progbits
        .global Player_HitboxMelee_032788
Player_HitboxMelee_032788:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x249a                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff8                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff8                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxGrenade_0327dc  @ $0327DC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxGrenade_0327dc, "ax", @progbits
        .global Player_HitboxGrenade_0327dc
Player_HitboxGrenade_0327dc:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x249a                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff4                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff4                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxAir_032830  @ $032830  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxAir_032830, "ax", @progbits
        .global Player_HitboxAir_032830
Player_HitboxAir_032830:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2434                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff6                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0026                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0007                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0012                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxKnockback_032884  @ $032884  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxKnockback_032884, "ax", @progbits
        .global Player_HitboxKnockback_032884
Player_HitboxKnockback_032884:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2478                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0xfff6                        | +026  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxDeath_0328d8  @ $0328D8  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxDeath_0328d8, "ax", @progbits
        .global Player_HitboxDeath_0328d8
Player_HitboxDeath_0328d8:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2456                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +010  (dato / opcode no decodificado)
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
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +032  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0006                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xfff6                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_HitboxSlug_03292c  @ $03292C  (168 B)
| ----------------------------------------------------------------------------
        .section .text.Player_HitboxSlug_03292c, "ax", @progbits
        .global Player_HitboxSlug_03292c
Player_HitboxSlug_03292c:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +002  (dato / opcode no decodificado)
        .dc.w   0x2412                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +00e  (dato / opcode no decodificado)
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
        .dc.w   0xfffc                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +02e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +030  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +038  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +03a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +044  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +046  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0008                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +050  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +056  (dato / opcode no decodificado)
        .dc.w   0x2412                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +060  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +068  (dato / opcode no decodificado)
        .dc.w   0x366c                        | +06a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +074  (dato / opcode no decodificado)
        .dc.w   0x3676                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +080  (dato / opcode no decodificado)
        .dc.w   0x3680                        | +082  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +086  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x368a                        | +08e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +090  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +098  (dato / opcode no decodificado)
        .dc.w   0x3694                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x000c                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a6  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_WeaponAmmoTbl_0329d4  @ $0329D4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WeaponAmmoTbl_0329d4, "ax", @progbits
        .global Player_WeaponAmmoTbl_0329d4
Player_WeaponAmmoTbl_0329d4:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x03e7                        | +004  (dato / opcode no decodificado)
        .dc.w   0x000a                        | +006  (dato / opcode no decodificado)
        .dc.w   0x03e7                        | +008  (dato / opcode no decodificado)
        .dc.w   0x000f                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x03e7                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0014                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x03e7                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0096                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_WeaponFlagTbl_0329e8  @ $0329E8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WeaponFlagTbl_0329e8, "ax", @progbits
        .global Player_WeaponFlagTbl_0329e8
Player_WeaponFlagTbl_0329e8:
        .dc.w   0x1400                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +002  (dato / opcode no decodificado)
        .dc.w   0x14ff                        | +004  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  OpcodeOffsetTable_0329EE  @ $0329EE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.OpcodeOffsetTable_0329EE, "ax", @progbits
        .global OpcodeOffsetTable_0329EE
OpcodeOffsetTable_0329EE:
        .dc.w   0x0016                        | +000  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Entity_ClearCollisionCb_0329f8  @ $0329F8  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_ClearCollisionCb_0329f8, "ax", @progbits
        .global Entity_ClearCollisionCb_0329f8
Entity_ClearCollisionCb_0329f8:
        move.l  #0xffffffff,0x48(a6)            | +000
        rts                                     | +008
