| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave CCCC — vehículo SV-001: spawn/caída, estados en pendiente, disparo,
|             salto, caída, impacto y muerte (1a mitad de los estados)
|  Región: $02AE3E..$02DD20  (11,812 B, 70 entradas, 25 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 11,812 B: la primera mitad de la máquina de estados del Slug
|  (entidad en $100580, handler en (a6)). Completa a slug_vehicle_02ddxx.s
|  (Wave ZZZ) y usa los helpers de slug_helpers_0295xx.s (Wave BBBB). Todos
|  los estados comparten la cola estándar: Slug_GroundContact -> Slug_Fall,
|  Slug_InputDirByLayoutA/B + Slug_InputDirPtrTbl_02c9b0 ($772), Slug_CanFire
|  -> Slug_Fire*, Slug_ConsumeField89 -> Slug_IdleEnterB, $5D00E (daño) ->
|  Slug_Destroyed, Slug_TryStartDestroyed(B) + Slug_StateByAnglePtrTbl ($772)
|  y, si $5DD56 con $27964E devuelve C, salto a Slug_DeathFinishJmp.
|
|   1. Spawn por paracaídas: Slug_SpawnDrop_02ae50 (template $2AE50 creado
|      por cutscene_anim/anim_state_machine $8BD8E/$8BEE2/$8C088 con +$98 =
|      variante 0..4) y Slug_SpawnDropB_02aea4 (ref índice $E8490) eligen
|      en Slug_DropVariantPtrTbl(B)_02ae90/02aee4 uno de Slug_DropVariant0..4
|      _02aef8..02af88: Slug_Init con d1 = 3, prio |= 2, crean la torreta
|      Turret_InitDir0..4 ($459A8..$45ABE) + PlayerIcon_Pow + Slug_WheelAnim,
|      sprite $2792B0, hitbox $29698, vel Y -$400; al pasar Y > $180 ->
|      Slug_DropDescend_02b05e (anim $46, +$82 suelo += 8, clamp vel Y $400,
|      al tocar suelo Slug_IdleEnterA). Slug_DropBossInit_02b166 /
|      Slug_DropBossDescend_02b264: igual con Slug_InitBoss en ($68,$1D8) o
|      ($40,$1E0) y +$13 bit6 (no referenciado: variante sin usar).
|      Slug_DeadHandler_02ae3e: (a6) centinela "Slug muerto" (+$48/+$60 =
|      -1; Slug_IsAlive y turret_boss2 $45DEC lo comparan).
|   2. Parado: Slug_IdleFlat_02b38c ($267E6, anim por pendiente sobre
|      $2792F4, ataque Slug_AttackTbl10+$54, frame base $14) y
|      Slug_IdleSlope_02b4d2 (+Jmp_02b4ca; sprite por ángulo sobre $2B0C30,
|      frame base 0, empuje d7 = ±$2A0 según sondas; en escena $106F2A == 3
|      corrige con la pendiente); Slug_SlopeIdleEnter_02b6f4 (frame $41,
|      tabla $2B0CB8, ref Slug_Stall) y Slug_SlopeMount_02b7da (frame $82,
|      tabla $2B0CFC; destino de Slug_PlayerMount).
|   3. Movimiento: Slug_AccelRightB/LeftB_02b8d8/02ba3e (Fx_SpawnDustPair
|      $31E5E/$31E76, vel objetivo ±$2A0 en 8 frames (+$91), anim $27945C/
|      $279538, frame $19/$1E, música por índice en Slug_AccelRight/LeftMusic
|      Tbl $10AF/$10B0); Slug_BrakeRight/Left_02bd00/02be32 (decelera a 0,
|      frame $A/$F); Slug_CruiseRightB/LeftB_02bf64/02c07a (vel fija ±$2A0,
|      anim $2793D0/$279524, frame 0/5).
|   4. Disparo del cañón: Slug_FireIdle_02c190 y Slug_FireFlat_02c24a
|      (anim $279498, frame $14/$3C; PlayerSlot_TestMaskCur d1 = 4 -> crea el
|      proyectil VehicleLaunch_Init $311C0/$312B2 ó $31240 según +$84, humo
|      SlugFx_ExhaustOrDrop $319F0 / $31944; Slug_GaugeTick, +$8E = $1E
|      cadencia, +$8D bit0), Slug_FireRecoil_02c432 (anim $2794E8),
|      Slug_FireFlatResume_02c554, Slug_FireMoveRight/Left_02c572/02c648,
|      Slug_FireSlope_02c71e (igual sobre pendiente, humo $31922).
|   5. Salto: Slug_JumpCrouch_02c908 (+_Loop_02c95c; anim $279308, frame
|      $2D, ataque Slug_AttackPtrTbl+$14; destino de Slug_SetSpeed) ->
|      Slug_JumpLaunch_02ca0c (vel Y -$4A4 / grav -$63, anim $27931C, frame
|      $32, ataque +$28) -> Slug_JumpAir_02ca8a (dirección aérea $5D5B6 en
|      Slug_AirSteerAccelTbl_02c900 {0,-$80,+$80,0}, clamp ±$2A0/$400,
|      $5CEF8 -> despacho $2A08C) -> Slug_JumpLand_02cc1a (invierte vel X
|      *$80, anim $279330, frame $37, 17 frames) / Slug_JumpLandSlope_02ce1c.
|      Slug_JumpNeutral/Right/Left_02c9c4/02c9d4/02c9f0 fijan vel X 0/±$2AA
|      (tabla Slug_InputDirPtrTbl_02c9b0 indexada por input dirección).
|      Slug_JumpAirFire_02cc02 -> Slug_FireAirB.
|   6. Caída: Slug_FallStart_02cffa / Slug_Fall_02d02e (+_Loop_02d0c4,
|      grav -$20, +$82 += 4; ref Slug_Hunker, IdleAngledB, Stall) ->
|      Slug_JumpLand al tocar suelo; Slug_FallFire_02d26e.
|   7. Disparo en el aire: Slug_FireJumpCrouch_02d286 (anim $27931C),
|      Slug_FireAir_02d394 (+_Loop_02d49e, humo SlugFx_Exhaust $31AF6),
|      Slug_FireAirB_02d380 / Slug_FireAirResume_02d38a.
|   8. Impacto (desde Slug_StateByAnglePtrTbl[1,2,4,5]): Slug_HitReact_02d63e
|      (anims $27954C/$279560, frame $96, +$45 = $28, snd $5E722, cb
|      Slug_HitboxCbC), Slug_HitLaunchA/B_02d736/02d802 (vel Y -$3FC, anims
|      $279574/$279588) -> Slug_HitLandA/B_02d8c8/02d980 ($27959C/$2795B0).
|   9. Muerte: Slug_DeathStart_02dcc0 (destino de Slug_TryStartDestroyed(B):
|      hitbox $2973C + 2 ataques $283CA, Slug_ExplodeFx) ->
|      Slug_DeathExplode_02da38 (Slug_KillInit, +$13 bit0, elige anim por
|      lado del impacto (+$54 vs centro de la hitbox +$48), bset #4 +$8D,
|      Slug_UpdateDamageSprite) -> Slug_DeathFade_02dc5c ($28 frames,
|      +$48/+$60 = -1, $5B6 + $13600, handler $400 vía
|      Slug_DeathSetHandler400_02dcaa); Slug_DeathLaunch_02db52 (variante
|      con salto -$3FC, $3C frames). Slug_DestroyedSlideInit_02bbf2 /
|      Slug_DestroyedSlide_02bba4 (+_Loop_02bc78): música $10E9/$10B2
|      (Slug_DestroyedMusicTbl), vel $600, hitbox $2964C, +$92 = $30 ->
|      Slug_SelfDestructAttack cuando X >= $117 o expira.
|
|  B. EVIDENCIAS
|  -------------
|   - Los 70 handlers sólo son alcanzables desde slug_vehicle/slug_helpers
|     o entre sí; todos operan sobre los campos del Slug (+$80 ángulo,
|     +$82 suelo, +$8C/+$8D bits, +$90 gauge, +$94 anim idx).
|   - $2AE50 lo crean los scripts de intro de misión ($8BD8E..$8C088) en
|     ($A0,$1FF) con +$98 = 3: es el Slug que cae en paracaídas al inicio.
|   - Turret_InitDir0..4 ($459A8..$45ABE) son las 5 torretas del cañón;
|     una por variante de caída.
|   - Slug_IsAlive_02acfc compara (a6) con $2AE3E: el centinela de muerte.
|   - Las tablas de 5 words $2B8CE/$2BA34/$2BB9A sólo contienen ids de
|     música ($10AF/$10B0/$10B2) y -1; se indexan con anim idx >> 1.
|
|  C. CAMPOS (a6 = Slug)
|  ---------------------
|   +$00 handler, +$13 bits 0/3/6, +$20 frame, +$22/+$24 pos, +$28/+$2A vel,
|   +$2C/+$2E aceleración, +$36 impulso, +$45 anim timer, +$48 cb colisión,
|   +$4C tabla de ataque, +$54, +$5B bits 3/7, +$60 hitbox, +$80 ángulo,
|   +$82 Y de suelo, +$84 lado, +$8B, +$8C bit5 aire, +$8D bits 0/1/2/4,
|   +$8E cadencia, +$90 gauge, +$91 temporizador, +$92 timer destrucción,
|   +$94 anim idx, +$98 variante de caída.
|
|  D. HELPERS EXTERNOS
|  -------------------
|   $4AE/$6FE alloc, $518 free, $5B6, $772 Table_LookupPointerBounded,
|   $2352 music, $13600, $267E2/$267E6, $267F4 clamp, $283CA/$283D8 ataque,
|   $28CD4 sprite, $28D70 anim, $5CEF8, $5D5B6 input dir, $5DD02 copia,
|   $5DD56 suelo, $5E722 snd, $8F714 PlayerSlot_TestMaskCur, $517FE,
|   $311C0/$31240/$312B2 proyectiles, $31922/$31944/$319F0/$31AF6 humo,
|   $31E5E/$31E76 polvo, $31FCA icono POW, $459A8..$45ABE torretas.
|
|  E. HIPÓTESIS ABIERTAS
|  ---------------------
|   - Slug_DropBossInit/Descend no tienen referencias: posible variante
|     recortada o entrada vía tabla de datos no identificada.
|   - "Fire*" asume que PlayerSlot_TestMaskCur(d1 = 4) distingue el botón
|     de cañón del de vulcan; nombres provisionales hasta trazar $8F714.
|   - El criterio lado-del-impacto de Slug_DeathExplode (+$54 vs centro
|     hitbox) podría ser "lado del atacante" en vez de "lado del Slug".
|
|  F. SIGUIENTE
|  ------------
|   `$05AA96..$05CA2A`, `$057D04..$059342`, `$0527BA..$0539E2`.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Slug_DeadHandler_02ae3e  @ $02AE3E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeadHandler_02ae3e, "ax", @progbits
        .global Slug_DeadHandler_02ae3e
Slug_DeadHandler_02ae3e:
        move.l  #0xffffffff,0x48(a6)            | +000
        move.l  #0xffffffff,0x60(a6)            | +008
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Slug_SpawnDrop_02ae50  @ $02AE50  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SpawnDrop_02ae50, "ax", @progbits
        .global Slug_SpawnDrop_02ae50
Slug_SpawnDrop_02ae50:
        jsr     Slug_IsAlive_02acfc(pc)         | +000
        bcs.w   .L02ae8e                        | +004
        lea     Slug_DropVariantPtrTbl_02ae90(pc),a0 | +008
        moveq   #0,d0                           | +00c
        move.b  0x98(a6),d0                     | +00e
        cmpi.b  #0x5,d0                         | +012
        bcs.w   .L02ae6e                        | +016
        move.b  #0x4,d0                         | +01a
.L02ae6e:
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a1                    | +020
        lea     0x100580.l,a0                   | +024
        move.l  a1,(a0)                         | +02a
        jsr     0x5dd02.l                       | +02c
        move.w  0x24(a6),0x82(a0)               | +032
        jmp     0x518.l                         | +038
.L02ae8e:
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  Slug_DropVariantPtrTbl_02ae90  @ $02AE90  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariantPtrTbl_02ae90, "ax", @progbits
        .global Slug_DropVariantPtrTbl_02ae90
Slug_DropVariantPtrTbl_02ae90:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xaf1c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xaf40                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xaf64                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0xaf88                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_SpawnDropB_02aea4  @ $02AEA4  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SpawnDropB_02aea4, "ax", @progbits
        .global Slug_SpawnDropB_02aea4
Slug_SpawnDropB_02aea4:
        jsr     Slug_IsAlive_02acfc(pc)         | +000
        bcs.w   .L02aedc                        | +004
        lea     Slug_DropVariantPtrTblB_02aee4(pc),a0 | +008
        moveq   #0,d0                           | +00c
        move.b  0x98(a6),d0                     | +00e
        cmpi.b  #0x5,d0                         | +012
        bcs.w   .L02aec2                        | +016
        move.b  #0x4,d0                         | +01a
.L02aec2:
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a1                    | +020
        lea     0x100580.l,a0                   | +024
        move.l  a1,(a0)                         | +02a
        jsr     0x5dd02.l                       | +02c
        move.w  0x24(a6),0x82(a0)               | +032
.L02aedc:
        jmp     0x518.l                         | +038

| ----------------------------------------------------------------------------
|  Slug_SpawnDropRts_02aee2  @ $02AEE2  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SpawnDropRts_02aee2, "ax", @progbits
        .global Slug_SpawnDropRts_02aee2
Slug_SpawnDropRts_02aee2:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Slug_DropVariantPtrTblB_02aee4  @ $02AEE4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariantPtrTblB_02aee4, "ax", @progbits
        .global Slug_DropVariantPtrTblB_02aee4
Slug_DropVariantPtrTblB_02aee4:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xaef8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xaf1c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xaf40                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xaf64                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0xaf88                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DropVariant0_02aef8  @ $02AEF8  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariant0_02aef8, "ax", @progbits
        .global Slug_DropVariant0_02aef8
Slug_DropVariant0_02aef8:
        move.w  #0x3,d1                         | +000
        jsr     Slug_Init_02a0f8(pc)            | +004
        ori.w   #0x2,0x38(a6)                   | +008
        lea     0x459a8.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        bra.w   Slug_DropVariant4_02af88__L02afac | +020

| ----------------------------------------------------------------------------
|  Slug_DropVariant1_02af1c  @ $02AF1C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariant1_02af1c, "ax", @progbits
        .global Slug_DropVariant1_02af1c
Slug_DropVariant1_02af1c:
        move.w  #0x3,d1                         | +000
        jsr     Slug_Init_02a0f8(pc)            | +004
        ori.w   #0x2,0x38(a6)                   | +008
        lea     0x459c0.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        bra.w   Slug_DropVariant4_02af88__L02afac | +020

| ----------------------------------------------------------------------------
|  Slug_DropVariant2_02af40  @ $02AF40  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariant2_02af40, "ax", @progbits
        .global Slug_DropVariant2_02af40
Slug_DropVariant2_02af40:
        move.w  #0x3,d1                         | +000
        jsr     Slug_Init_02a0f8(pc)            | +004
        ori.w   #0x2,0x38(a6)                   | +008
        lea     0x459d8.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        bra.w   Slug_DropVariant4_02af88__L02afac | +020

| ----------------------------------------------------------------------------
|  Slug_DropVariant3_02af64  @ $02AF64  (36 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariant3_02af64, "ax", @progbits
        .global Slug_DropVariant3_02af64
Slug_DropVariant3_02af64:
        move.w  #0x3,d1                         | +000
        jsr     Slug_Init_02a0f8(pc)            | +004
        ori.w   #0x2,0x38(a6)                   | +008
        lea     0x459f0.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        bra.w   Slug_DropVariant4_02af88__L02afac | +020

| ----------------------------------------------------------------------------
|  Slug_DropVariant4_02af88  @ $02AF88  (206 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropVariant4_02af88, "ax", @progbits
        .global Slug_DropVariant4_02af88
Slug_DropVariant4_02af88:
        move.w  #0x3,d1                         | +000
        jsr     Slug_Init_02a0f8(pc)            | +004
        ori.w   #0x2,0x38(a6)                   | +008
        lea     0x45abe.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        bra.w   Slug_DropVariant4_02af88__L02afac | +020
        .global Slug_DropVariant4_02af88__L02afac
Slug_DropVariant4_02af88__L02afac:
        lea     0x31fca.l,a1                    | +024
        jsr     0x4ae.l                         | +02a
        lea     Slug_WheelAnim_02fb92__L02fb9a(pc),a1 | +030
        jsr     0x4ae.l                         | +034
        jsr     0x5dd02.l                       | +03a
        lea     0x2792b0.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        clr.w   0x28(a6)                        | +04c
        clr.w   0x2a(a6)                        | +050
        clr.w   0x2c(a6)                        | +054
        clr.w   0x2e(a6)                        | +058
        clr.b   0x26(a6)                        | +05c
        clr.b   0x27(a6)                        | +060
        clr.b   0x8e(a6)                        | +064
        lea     Slug_HitboxDestroyed_02964c__L029698(pc),a0 | +068
        move.l  a0,0x4c(a6)                     | +06c
        jsr     0x283ca.l                       | +070
        lea     .L02b004(pc),a1                 | +076
        move.l  a1,(a6)                         | +07a
.L02b004:
        move.b  #0x5a,0x45(a6)                  | +07c
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +082
        move.w  #0xfc00,0x2a(a6)                | +086
        jsr     Slug_PhysicsAir_02a878(pc)      | +08c
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +090
        move.w  0x38(a6),d1                     | +094
        move.w  #0x9fe0,d0                      | +098
        andi.w  #0xffe3,d1                      | +09c
        ori.w   #0xc,d0                         | +0a0
        or.w    d0,d1                           | +0a4
        move.w  d1,0x38(a6)                     | +0a6
        jsr     0x28d70.l                       | +0aa
        jsr     0x283d8.l                       | +0b0
        cmpi.w  #0x180,0x24(a6)                 | +0b6
        bgt.w   .L02b04e                        | +0bc
        lea     Slug_DropDescend_02b05e(pc),a1  | +0c0
        move.l  a1,(a6)                         | +0c4
.L02b04e:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0c6
        bcs.w   SetHandlerRts_02b262            | +0ca

| ----------------------------------------------------------------------------
|  Slug_DropDescend_02b05e  @ $02B05E  (264 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropDescend_02b05e, "ax", @progbits
        .global Slug_DropDescend_02b05e
Slug_DropDescend_02b05e:
        lea     0x2792b0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        addq.w  #0x8,0x82(a6)                   | +00c
        move.w  0x24(a6),d0                     | +010
        cmp.w   0x82(a6),d0                     | +014
        ble.w   .L02b07e                        | +018
        move.w  d0,0x82(a6)                     | +01c
.L02b07e:
        jsr     Slug_TerrainSlope_02a958(pc)    | +020
        cmp.w   0x80(a6),d2                     | +024
        beq.w   .L02b08e                        | +028
        move.w  d2,0x80(a6)                     | +02c
.L02b08e:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +030
        move.w  #0xfc00,0x2a(a6)                | +034
        lea     .L02b09e(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L02b09e:
        move.b  #0x46,0x45(a6)                  | +040
        bset    #0x1,0x8d(a6)                   | +046
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +04c
        move.w  0x2a(a6),d0                     | +050
        move.w  #0x400,d1                       | +054
        jsr     0x267f4.l                       | +058
        move.w  d0,0x2a(a6)                     | +05e
        jsr     Slug_PhysicsAir_02a878(pc)      | +062
        bcc.w   .L02b0ce                        | +066
        lea     Slug_IdleEnterA_02dd20(pc),a1   | +06a
        move.l  a1,(a6)                         | +06e
.L02b0ce:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +070
        move.w  0x38(a6),d1                     | +074
        move.w  #0x9fe0,d0                      | +078
        andi.w  #0xffe3,d1                      | +07c
        ori.w   #0xc,d0                         | +080
        or.w    d0,d1                           | +084
        move.w  d1,0x38(a6)                     | +086
        jsr     0x28d70.l                       | +08a
        jsr     0x283d8.l                       | +090
        tst.w   0x2a(a6)                        | +096
        bgt.w   .L02b164                        | +09a
        jsr     Slug_GroundContact_02a8c0(pc)   | +09e
        cmpi.b  #0x0,d1                         | +0a2
        ble.w   .L02b110                        | +0a6
        clr.w   0x28(a6)                        | +0aa
        clr.w   0x2c(a6)                        | +0ae
.L02b110:
        jsr     Slug_TerrainSlope_02a958(pc)    | +0b2
        cmp.w   0x80(a6),d2                     | +0b6
        beq.w   .L02b164                        | +0ba
        move.w  d2,0x80(a6)                     | +0be
        move.w  d2,d0                           | +0c2
        asr.w   #0x1,d0                         | +0c4
        neg.w   d0                              | +0c6
        addi.w  #0x20,d0                        | +0c8
        cmpi.w  #0x40,d0                        | +0cc
        bls.w   .L02b136                        | +0d0
        move.w  #0x40,d0                        | +0d4
.L02b136:
        movem.w d0,-(a7)                        | +0d8
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +0dc
        movea.l #0x2b0c30,a0                    | +0e0
        lsl.w   #0x2,d0                         | +0e6
        movea.l (a0,d0.w),a0                    | +0e8
        cmpa.l  #0xffffffff,a0                  | +0ec
        beq.w   .L02b15a                        | +0f2
        jsr     0x28cd4.l                       | +0f6
.L02b15a:
        movem.w (a7)+,d0                        | +0fc
        move.b  #0x14,0x20(a6)                  | +100
.L02b164:
        rts                                     | +106

| ----------------------------------------------------------------------------
|  Slug_DropBossInit_02b166  @ $02B166  (246 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropBossInit_02b166, "ax", @progbits
        .global Slug_DropBossInit_02b166
Slug_DropBossInit_02b166:
        move.w  #0x68,0x22(a6)                  | +000
        move.w  #0x1d8,0x24(a6)                 | +006
        move.w  #0x1d8,0x82(a6)                 | +00c
        move.w  #0x3,d1                         | +012
        jsr     Slug_InitBoss_02a1aa(pc)        | +016
        ori.w   #0x2,0x38(a6)                   | +01a
        bra.w   .L02b1c2                        | +020
        move.l  #0x2ae3e,(a6)                   | +024
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +02a
        rts                                     | +02e
        move.l  #0x2ae3e,(a6)                   | +030
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +036
        rts                                     | +03a
        move.w  #0x40,0x22(a6)                  | +03c
        move.w  #0x1e0,0x24(a6)                 | +042
        move.w  #0x1e0,0x82(a6)                 | +048
        move.w  #0x4d,d1                        | +04e
        jsr     Slug_InitBoss_02a1aa(pc)        | +052
        ori.w   #0x0,0x38(a6)                   | +056
.L02b1c2:
        lea     0x2792b0.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        clr.w   0x28(a6)                        | +068
        clr.w   0x2a(a6)                        | +06c
        clr.w   0x2c(a6)                        | +070
        clr.w   0x2e(a6)                        | +074
        clr.b   0x26(a6)                        | +078
        clr.b   0x27(a6)                        | +07c
        clr.b   0x8e(a6)                        | +080
        move.w  #0x200,0x2a(a6)                 | +084
        move.w  #0xfe00,0x2e(a6)                | +08a
        lea     Slug_HitboxDestroyed_02964c__L029698(pc),a0 | +090
        move.l  a0,0x4c(a6)                     | +094
        jsr     0x283ca.l                       | +098
        lea     .L02b20a(pc),a1                 | +09e
        move.l  a1,(a6)                         | +0a2
.L02b20a:
        bset    #0x6,0x13(a6)                   | +0a4
        move.b  #0x5a,0x45(a6)                  | +0aa
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +0b0
        jsr     Slug_PhysicsAir_02a878(pc)      | +0b4
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0b8
        move.w  0x38(a6),d1                     | +0bc
        move.w  #0x9fe0,d0                      | +0c0
        andi.w  #0xffe3,d1                      | +0c4
        ori.w   #0xc,d0                         | +0c8
        or.w    d0,d1                           | +0cc
        move.w  d1,0x38(a6)                     | +0ce
        jsr     0x28d70.l                       | +0d2
        jsr     0x283d8.l                       | +0d8
        cmpi.w  #0x180,0x24(a6)                 | +0de
        bgt.w   .L02b254                        | +0e4
        lea     Slug_DropBossDescend_02b264(pc),a1 | +0e8
        move.l  a1,(a6)                         | +0ec
.L02b254:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0ee
        bcs.w   SetHandlerRts_02b262            | +0f2

| ----------------------------------------------------------------------------
|  Slug_DropBossDescend_02b264  @ $02B264  (296 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DropBossDescend_02b264, "ax", @progbits
        .global Slug_DropBossDescend_02b264
Slug_DropBossDescend_02b264:
        lea     0x2792b0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        addq.w  #0x8,0x82(a6)                   | +00c
        move.w  #0x200,0x2a(a6)                 | +010
        move.w  #0xfe00,0x2e(a6)                | +016
        move.w  0x24(a6),d0                     | +01c
        cmp.w   0x82(a6),d0                     | +020
        ble.w   .L02b290                        | +024
        move.w  d0,0x82(a6)                     | +028
.L02b290:
        jsr     Slug_TerrainSlope_02a958(pc)    | +02c
        cmp.w   0x80(a6),d2                     | +030
        beq.w   .L02b2a0                        | +034
        move.w  d2,0x80(a6)                     | +038
.L02b2a0:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +03c
        lea     .L02b2aa(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L02b2aa:
        cmpi.w  #0x180,0x24(a6)                 | +046
        bgt.w   .L02b2be                        | +04c
        clr.w   0x2a(a6)                        | +050
        lea     .L02b2be(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L02b2be:
        bset    #0x6,0x13(a6)                   | +05a
        move.b  #0x46,0x45(a6)                  | +060
        bset    #0x1,0x8d(a6)                   | +066
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +06c
        move.w  0x2a(a6),d0                     | +070
        move.w  #0x400,d1                       | +074
        jsr     0x267f4.l                       | +078
        move.w  d0,0x2a(a6)                     | +07e
        jsr     Slug_PhysicsAir_02a878(pc)      | +082
        bcc.w   .L02b2f4                        | +086
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +08a
        move.l  a1,(a6)                         | +08e
.L02b2f4:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +090
        move.w  0x38(a6),d1                     | +094
        move.w  #0x9fe0,d0                      | +098
        andi.w  #0xffe3,d1                      | +09c
        ori.w   #0xc,d0                         | +0a0
        or.w    d0,d1                           | +0a4
        move.w  d1,0x38(a6)                     | +0a6
        jsr     0x28d70.l                       | +0aa
        jsr     0x283d8.l                       | +0b0
        tst.w   0x2a(a6)                        | +0b6
        bgt.w   .L02b38a                        | +0ba
        jsr     Slug_GroundContact_02a8c0(pc)   | +0be
        cmpi.b  #0x0,d1                         | +0c2
        ble.w   .L02b336                        | +0c6
        clr.w   0x28(a6)                        | +0ca
        clr.w   0x2c(a6)                        | +0ce
.L02b336:
        jsr     Slug_TerrainSlope_02a958(pc)    | +0d2
        cmp.w   0x80(a6),d2                     | +0d6
        beq.w   .L02b38a                        | +0da
        move.w  d2,0x80(a6)                     | +0de
        move.w  d2,d0                           | +0e2
        asr.w   #0x1,d0                         | +0e4
        neg.w   d0                              | +0e6
        addi.w  #0x20,d0                        | +0e8
        cmpi.w  #0x40,d0                        | +0ec
        bls.w   .L02b35c                        | +0f0
        move.w  #0x40,d0                        | +0f4
.L02b35c:
        movem.w d0,-(a7)                        | +0f8
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +0fc
        movea.l #0x2b0c30,a0                    | +100
        lsl.w   #0x2,d0                         | +106
        movea.l (a0,d0.w),a0                    | +108
        cmpa.l  #0xffffffff,a0                  | +10c
        beq.w   .L02b380                        | +112
        jsr     0x28cd4.l                       | +116
.L02b380:
        movem.w (a7)+,d0                        | +11c
        move.b  #0x14,0x20(a6)                  | +120
.L02b38a:
        rts                                     | +126

| ----------------------------------------------------------------------------
|  Slug_IdleFlat_02b38c  @ $02B38C  (318 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleFlat_02b38c, "ax", @progbits
        .global Slug_IdleFlat_02b38c
Slug_IdleFlat_02b38c:
        jsr     0x267e6.l                       | +000
        clr.w   0x28(a6)                        | +006
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +00a
        movea.l #0x2792f4,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02b3b6                        | +020
        jsr     0x28cd4.l                       | +024
.L02b3b6:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +02a
        movea.l (a1,d0.w),a0                    | +02e
        move.l  a0,0x48(a6)                     | +032
        move.b  #0x14,0x20(a6)                  | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        move.l  #0x295b4,0x60(a6)               | +046
        lea     .L02b3e0(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L02b3e0:
        lea     Slug_HitboxDestroyed_02964c__L029698(pc),a0 | +054
        move.l  a0,0x4c(a6)                     | +058
        jsr     0x283ca.l                       | +05c
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +062
        jsr     Slug_PhysicsE_02a7d8(pc)        | +066
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +06a
        bcc.w   .L02b404                        | +06e
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +072
        move.l  a1,(a6)                         | +076
.L02b404:
        jsr     ClearXN_02abc0(pc)              | +078
        bcs.w   .L02b428                        | +07c
        jsr     Slug_CallGroundProbeA_02a328(pc) | +080
        bcc.w   .L02b41a                        | +084
        lea     Slug_AccelRightB_02b8d8__L02b8de(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L02b41a:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +08e
        bcc.w   .L02b428                        | +092
        lea     Slug_AccelLeftB_02ba3e__L02ba44(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L02b428:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +09c
        bcc.w   .L02b448                        | +0a0
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0a4
        andi.w  #0xff,d0                        | +0a8
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0ac
        movea.l #0xffffffff,a1                  | +0b0
        jsr     0x772.l                         | +0b6
.L02b448:
        jsr     0x28d70.l                       | +0bc
        jsr     Slug_GroundContact_02a8c0(pc)   | +0c2
        bcc.w   .L02b45c                        | +0c6
        lea     Slug_Fall_02d02e(pc),a1         | +0ca
        move.l  a1,(a6)                         | +0ce
.L02b45c:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0d0
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0d4
        movea.l #0xffffffff,a1                  | +0d8
        jsr     0x772.l                         | +0de
        jsr     Slug_CanFire_02aac0(pc)         | +0e4
        bcc.w   .L02b47e                        | +0e8
        lea     Slug_FireFlat_02c24a(pc),a1     | +0ec
        move.l  a1,(a6)                         | +0f0
.L02b47e:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0f2
        bcc.w   .L02b48c                        | +0f6
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0fa
        move.l  a1,(a6)                         | +0fe
.L02b48c:
        jsr     JsrAbsThunk_02a5cc(pc)          | +100
        bcc.w   .L02b49a                        | +104
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +108
        move.l  a1,(a6)                         | +10c
.L02b49a:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +10e
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +112
        movea.l #0xffffffff,a1                  | +116
        jsr     0x772.l                         | +11c
        movea.l #0xffffffff,a0                  | +122
        lea     0x27964e.l,a0                   | +128
        jsr     0x5dd56.l                       | +12e
        bcc.w   .L02b4c8                        | +134
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +138
.L02b4c8:
        rts                                     | +13c

| ----------------------------------------------------------------------------
|  Slug_IdleSlopeJmp_02b4ca  @ $02B4CA  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleSlopeJmp_02b4ca, "ax", @progbits
        .global Slug_IdleSlopeJmp_02b4ca
Slug_IdleSlopeJmp_02b4ca:
        bra.w   Slug_IdleSlope_02b4d2           | +000
        .global Slug_IdleSlopeJmp_02b4ca__L02b4ce
Slug_IdleSlopeJmp_02b4ca__L02b4ce:
        bra.w   Slug_IdleSlope_02b4d2           | +004

| ----------------------------------------------------------------------------
|  Slug_IdleSlope_02b4d2  @ $02B4D2  (546 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleSlope_02b4d2, "ax", @progbits
        .global Slug_IdleSlope_02b4d2
Slug_IdleSlope_02b4d2:
        clr.w   0x36(a6)                        | +000
        bclr    #0x5,0x8c(a6)                   | +004
        clr.w   0x2c(a6)                        | +00a
        jsr     Slug_TerrainSlope_02a958(pc)    | +00e
        move.w  d2,0x80(a6)                     | +012
        move.w  d2,d0                           | +016
        asr.w   #0x1,d0                         | +018
        neg.w   d0                              | +01a
        addi.w  #0x20,d0                        | +01c
        cmpi.w  #0x40,d0                        | +020
        bls.w   .L02b4fe                        | +024
        move.w  #0x40,d0                        | +028
.L02b4fe:
        movem.w d0,-(a7)                        | +02c
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +030
        movea.l #0x2b0c30,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L02b522                        | +046
        jsr     0x28cd4.l                       | +04a
.L02b522:
        movem.w (a7)+,d0                        | +050
        move.b  #0x0,0x20(a6)                   | +054
        lsr.w   #0x2,d0                         | +05a
        andi.w  #0xff,d0                        | +05c
        add.b   d0,0x20(a6)                     | +060
        move.l  #0x295b4,0x60(a6)               | +064
        lea     .L02b544(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L02b544:
        lea     Slug_HitboxDestroyed_02964c__L029698(pc),a0 | +072
        move.l  a0,0x4c(a6)                     | +076
        jsr     0x283ca.l                       | +07a
        bset    #0x2,0x8d(a6)                   | +080
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +086
        move.w  0x36(a6),d7                     | +08a
        jsr     ClearXN_02abc0(pc)              | +08e
        bcs.w   .L02b59c                        | +092
        tst.b   0x92(a6)                        | +096
        bne.w   .L02b59c                        | +09a
        jsr     Slug_CallGroundProbeA_02a328(pc) | +09e
        bcc.w   .L02b588                        | +0a2
        tst.w   0x36(a6)                        | +0a6
        bne.w   .L02b584                        | +0aa
        move.w  #0x2a0,d7                       | +0ae
.L02b584:
        bra.w   .L02b59c                        | +0b2
.L02b588:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +0b6
        bcc.w   .L02b59c                        | +0ba
        tst.w   0x36(a6)                        | +0be
        bne.w   .L02b59c                        | +0c2
        move.w  #0xfd60,d7                      | +0c6
.L02b59c:
        cmpi.b  #0x3,0x106f2a.l                 | +0ca
        bne.w   .L02b5be                        | +0d2
        jsr     Slug_TerrainSlope_02a958(pc)    | +0d6
        sub.w   0x80(a6),d2                     | +0da
        eor.w   d7,d2                           | +0de
        asl.w   #0x1,d2                         | +0e0
        bcs.w   .L02b5be                        | +0e2
        move.w  d7,d2                           | +0e6
        asr.w   #0x1,d2                         | +0e8
        add.w   d2,d7                           | +0ea
.L02b5be:
        move.w  d7,0x28(a6)                     | +0ec
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0f0
        jsr     Slug_PhysicsB_02a760(pc)        | +0f4
        jsr     Slug_TerrainSlope_02a958(pc)    | +0f8
        cmp.w   0x80(a6),d2                     | +0fc
        beq.w   .L02b628                        | +100
        move.w  d2,0x80(a6)                     | +104
        move.w  d2,d0                           | +108
        asr.w   #0x1,d0                         | +10a
        neg.w   d0                              | +10c
        addi.w  #0x20,d0                        | +10e
        cmpi.w  #0x40,d0                        | +112
        bls.w   .L02b5f0                        | +116
        move.w  #0x40,d0                        | +11a
.L02b5f0:
        movem.w d0,-(a7)                        | +11e
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +122
        movea.l #0x2b0c30,a0                    | +126
        lsl.w   #0x2,d0                         | +12c
        movea.l (a0,d0.w),a0                    | +12e
        cmpa.l  #0xffffffff,a0                  | +132
        beq.w   .L02b614                        | +138
        jsr     0x28cd4.l                       | +13c
.L02b614:
        movem.w (a7)+,d0                        | +142
        move.b  #0x0,0x20(a6)                   | +146
        lsr.w   #0x2,d0                         | +14c
        andi.w  #0xff,d0                        | +14e
        add.b   d0,0x20(a6)                     | +152
.L02b628:
        jsr     0x28d70.l                       | +156
        jsr     Slug_TerrainIsSlope_02a4ec(pc)  | +15c
        bcs.w   .L02b65c                        | +160
        bra.w   .L02b63a                        | +164
.L02b63a:
        lea     Slug_IdleFlat_02b38c(pc),a1     | +168
        move.l  a1,(a6)                         | +16c
        jsr     Slug_CallGroundProbeA_02a328(pc) | +16e
        bcc.w   .L02b64e                        | +172
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +176
        move.l  a1,(a6)                         | +17a
.L02b64e:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +17c
        bcc.w   .L02b65c                        | +180
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +184
        move.l  a1,(a6)                         | +188
.L02b65c:
        tst.b   0x3b(a6)                        | +18a
        bne.w   .L02b66a                        | +18e
        jsr     0x283d8.l                       | +192
.L02b66a:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +198
        bcc.w   .L02b678                        | +19c
        lea     Slug_SlopeIdleEnter_02b6f4(pc),a1 | +1a0
        move.l  a1,(a6)                         | +1a4
.L02b678:
        jsr     Slug_GroundContact_02a8c0(pc)   | +1a6
        bcc.w   .L02b686                        | +1aa
        lea     Slug_FallStart_02cffa(pc),a1    | +1ae
        move.l  a1,(a6)                         | +1b2
.L02b686:
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +1b4
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +1b8
        movea.l #0xffffffff,a1                  | +1bc
        jsr     0x772.l                         | +1c2
        jsr     Slug_CanFire_02aac0(pc)         | +1c8
        bcc.w   .L02b6a8                        | +1cc
        lea     Slug_FireSlope_02c71e(pc),a1    | +1d0
        move.l  a1,(a6)                         | +1d4
.L02b6a8:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +1d6
        bcc.w   .L02b6b6                        | +1da
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1de
        move.l  a1,(a6)                         | +1e2
.L02b6b6:
        jsr     JsrAbsThunk_02a5cc(pc)          | +1e4
        bcc.w   .L02b6c4                        | +1e8
        lea     Slug_Destroyed_02fc70(pc),a1    | +1ec
        move.l  a1,(a6)                         | +1f0
.L02b6c4:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +1f2
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +1f6
        movea.l #0xffffffff,a1                  | +1fa
        jsr     0x772.l                         | +200
        movea.l #0xffffffff,a0                  | +206
        lea     0x27964e.l,a0                   | +20c
        jsr     0x5dd56.l                       | +212
        bcc.w   .L02b6f2                        | +218
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +21c
.L02b6f2:
        rts                                     | +220

| ----------------------------------------------------------------------------
|  Slug_SlopeIdleEnter_02b6f4  @ $02B6F4  (222 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SlopeIdleEnter_02b6f4, "ax", @progbits
        .global Slug_SlopeIdleEnter_02b6f4
Slug_SlopeIdleEnter_02b6f4:
        bclr    #0x5,0x8c(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        clr.w   0x2c(a6)                        | +00a
        jsr     Slug_TerrainSlope_02a958(pc)    | +00e
        move.w  0x80(a6),d2                     | +012
        move.w  d2,d0                           | +016
        asr.w   #0x1,d0                         | +018
        neg.w   d0                              | +01a
        addi.w  #0x20,d0                        | +01c
        cmpi.w  #0x40,d0                        | +020
        bls.w   .L02b720                        | +024
        move.w  #0x40,d0                        | +028
.L02b720:
        movem.w d0,-(a7)                        | +02c
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +030
        movea.l #0x2b0cb8,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L02b744                        | +046
        jsr     0x28cd4.l                       | +04a
.L02b744:
        movem.w (a7)+,d0                        | +050
        move.b  #0x41,0x20(a6)                  | +054
        lsr.w   #0x2,d0                         | +05a
        andi.w  #0xff,d0                        | +05c
        add.b   d0,0x20(a6)                     | +060
        move.b  #0xff,0x8b(a6)                  | +064
        lea     .L02b764(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L02b764:
        bset    #0x2,0x8d(a6)                   | +070
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +076
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +07a
        jsr     Slug_PhysicsB_02a760(pc)        | +07e
        jsr     0x28d70.l                       | +082
        bcc.w   .L02b786                        | +088
        lea     Slug_Stall_02eca6__L02ecc2(pc),a1 | +08c
        move.l  a1,(a6)                         | +090
.L02b786:
        jsr     Slug_GroundContact_02a8c0(pc)   | +092
        bcc.w   .L02b794                        | +096
        lea     Slug_FallStart_02cffa(pc),a1    | +09a
        move.l  a1,(a6)                         | +09e
.L02b794:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0a0
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0a4
        movea.l #0xffffffff,a1                  | +0a8
        jsr     0x772.l                         | +0ae
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0b4
        bcc.w   .L02b7b6                        | +0b8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0bc
        move.l  a1,(a6)                         | +0c0
.L02b7b6:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0c2
        bcc.w   .L02b7c4                        | +0c6
        lea     Slug_Destroyed_02fc70(pc),a1    | +0ca
        move.l  a1,(a6)                         | +0ce
.L02b7c4:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +0d0
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +0d4
        movea.l #0xffffffff,a1                  | +0d8

| ----------------------------------------------------------------------------
|  Slug_SlopeMount_02b7da  @ $02B7DA  (236 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SlopeMount_02b7da, "ax", @progbits
        .global Slug_SlopeMount_02b7da
Slug_SlopeMount_02b7da:
        bclr    #0x5,0x8c(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        clr.w   0x2c(a6)                        | +00a
        jsr     Slug_TerrainSlope_02a958(pc)    | +00e
        move.w  0x80(a6),d2                     | +012
        move.w  d2,d0                           | +016
        asr.w   #0x1,d0                         | +018
        neg.w   d0                              | +01a
        addi.w  #0x20,d0                        | +01c
        cmpi.w  #0x40,d0                        | +020
        bls.w   .L02b806                        | +024
        move.w  #0x40,d0                        | +028
.L02b806:
        movem.w d0,-(a7)                        | +02c
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +030
        movea.l #0x2b0cfc,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L02b82a                        | +046
        jsr     0x28cd4.l                       | +04a
.L02b82a:
        movem.w (a7)+,d0                        | +050
        move.b  #0x82,0x20(a6)                  | +054
        lsr.w   #0x2,d0                         | +05a
        andi.w  #0xff,d0                        | +05c
        add.b   d0,0x20(a6)                     | +060
        move.b  #0xff,0x8b(a6)                  | +064
        lea     .L02b84a(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L02b84a:
        bset    #0x2,0x8d(a6)                   | +070
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +076
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +07a
        jsr     Slug_PhysicsB_02a760(pc)        | +07e
        jsr     0x28d70.l                       | +082
        bcc.w   .L02b86c                        | +088
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +08c
        move.l  a1,(a6)                         | +090
.L02b86c:
        jsr     Slug_GroundContact_02a8c0(pc)   | +092
        bcc.w   .L02b87a                        | +096
        lea     Slug_FallStart_02cffa(pc),a1    | +09a
        move.l  a1,(a6)                         | +09e
.L02b87a:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0a0
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0a4
        movea.l #0xffffffff,a1                  | +0a8
        jsr     0x772.l                         | +0ae
        jsr     Slug_CanFire_02aac0(pc)         | +0b4
        bcc.w   .L02b89c                        | +0b8
        lea     Slug_FireSlope_02c71e(pc),a1    | +0bc
        move.l  a1,(a6)                         | +0c0
.L02b89c:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0c2
        bcc.w   .L02b8aa                        | +0c6
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0ca
        move.l  a1,(a6)                         | +0ce
.L02b8aa:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0d0
        bcc.w   .L02b8b8                        | +0d4
        lea     Slug_Destroyed_02fc70(pc),a1    | +0d8
        move.l  a1,(a6)                         | +0dc
.L02b8b8:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +0de
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +0e2
        movea.l #0xffffffff,a1                  | +0e6

| ----------------------------------------------------------------------------
|  Slug_AccelRightMusicTbl_02b8ce  @ $02B8CE  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelRightMusicTbl_02b8ce, "ax", @progbits
        .global Slug_AccelRightMusicTbl_02b8ce
Slug_AccelRightMusicTbl_02b8ce:
        .dc.w   0x10af                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AccelRightB_02b8d8  @ $02B8D8  (340 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelRightB_02b8d8, "ax", @progbits
        .global Slug_AccelRightB_02b8d8
Slug_AccelRightB_02b8d8:
        jsr     0x31e5e.l                       | +000
        .global Slug_AccelRightB_02b8d8__L02b8de
Slug_AccelRightB_02b8d8__L02b8de:
        move.w  #0x2a0,d0                       | +006
        sub.w   0x28(a6),d0                     | +00a
        asr.w   #0x3,d0                         | +00e
        move.w  d0,0x2c(a6)                     | +010
        move.b  #0x8,0x91(a6)                   | +014
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +01a
        movea.l #0x27945c,a0                    | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L02b912                        | +030
        jsr     0x28cd4.l                       | +034
.L02b912:
        movem.w d0,-(a7)                        | +03a
        lsr.w   #0x1,d0                         | +03e
        lea     Slug_AccelRightMusicTbl_02b8ce(pc),a0 | +040
        move.w  (a0,d0.w),d0                    | +044
        cmpi.w  #0xffff,d0                      | +048
        beq.w   .L02b92e                        | +04c
        jsr     0x2352.l                        | +050
.L02b92e:
        movem.w (a7)+,d0                        | +056
        move.b  #0x19,0x20(a6)                  | +05a
        andi.w  #0xff,d0                        | +060
        lsr.w   #0x2,d0                         | +064
        add.b   d0,0x20(a6)                     | +066
        lea     .L02b948(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L02b948:
        subq.b  #0x1,0x91(a6)                   | +070
        bne.w   .L02b960                        | +074
        move.w  #0x2a0,0x28(a6)                 | +078
        clr.w   0x2c(a6)                        | +07e
        lea     .L02b960(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L02b960:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +088
        jsr     Slug_PhysicsE_02a7d8(pc)        | +08c
        bcc.w   .L02b972                        | +090
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +094
        move.l  a1,(a6)                         | +098
.L02b972:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +09a
        bcc.w   .L02b980                        | +09e
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L02b980:
        jsr     0x28d70.l                       | +0a8
        bcc.w   .L02b990                        | +0ae
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +0b2
        move.l  a1,(a6)                         | +0b6
.L02b990:
        jsr     0x283d8.l                       | +0b8
        jsr     Slug_CallGroundProbeA_02a328(pc) | +0be
        bcs.w   .L02b9a4                        | +0c2
        lea     Slug_BrakeRight_02bd00(pc),a1   | +0c6
        move.l  a1,(a6)                         | +0ca
.L02b9a4:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +0cc
        bcc.w   .L02b9b2                        | +0d0
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +0d4
        move.l  a1,(a6)                         | +0d8
.L02b9b2:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +0da
        bcc.w   .L02b9d2                        | +0de
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0e2
        andi.w  #0xff,d0                        | +0e6
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0ea
        movea.l #0xffffffff,a1                  | +0ee
        jsr     0x772.l                         | +0f4
.L02b9d2:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0fa
        bcc.w   .L02b9e0                        | +0fe
        lea     Slug_Fall_02d02e(pc),a1         | +102
        move.l  a1,(a6)                         | +106
.L02b9e0:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +108
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +10c
        movea.l #0xffffffff,a1                  | +110
        jsr     0x772.l                         | +116
        jsr     Slug_CanFire_02aac0(pc)         | +11c
        bcc.w   .L02ba02                        | +120
        lea     Slug_FireFlat_02c24a(pc),a1     | +124
        move.l  a1,(a6)                         | +128
.L02ba02:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +12a
        bcc.w   .L02ba10                        | +12e
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +132
        move.l  a1,(a6)                         | +136
.L02ba10:
        jsr     JsrAbsThunk_02a5cc(pc)          | +138
        bcc.w   .L02ba1e                        | +13c
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +140
        move.l  a1,(a6)                         | +144
.L02ba1e:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +146
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +14a
        movea.l #0xffffffff,a1                  | +14e

| ----------------------------------------------------------------------------
|  Slug_AccelLeftMusicTbl_02ba34  @ $02BA34  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelLeftMusicTbl_02ba34, "ax", @progbits
        .global Slug_AccelLeftMusicTbl_02ba34
Slug_AccelLeftMusicTbl_02ba34:
        .dc.w   0x10b0                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_AccelLeftB_02ba3e  @ $02BA3E  (340 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelLeftB_02ba3e, "ax", @progbits
        .global Slug_AccelLeftB_02ba3e
Slug_AccelLeftB_02ba3e:
        jsr     0x31e76.l                       | +000
        .global Slug_AccelLeftB_02ba3e__L02ba44
Slug_AccelLeftB_02ba3e__L02ba44:
        move.w  #0xfd60,d0                      | +006
        sub.w   0x28(a6),d0                     | +00a
        asr.w   #0x3,d0                         | +00e
        move.w  d0,0x2c(a6)                     | +010
        move.b  #0x8,0x91(a6)                   | +014
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +01a
        movea.l #0x279538,a0                    | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L02ba78                        | +030
        jsr     0x28cd4.l                       | +034
.L02ba78:
        movem.w d0,-(a7)                        | +03a
        lsr.w   #0x1,d0                         | +03e
        lea     Slug_AccelLeftMusicTbl_02ba34(pc),a0 | +040
        move.w  (a0,d0.w),d0                    | +044
        cmpi.w  #0xffff,d0                      | +048
        beq.w   .L02ba94                        | +04c
        jsr     0x2352.l                        | +050
.L02ba94:
        movem.w (a7)+,d0                        | +056
        move.b  #0x1e,0x20(a6)                  | +05a
        andi.w  #0xff,d0                        | +060
        lsr.w   #0x2,d0                         | +064
        add.b   d0,0x20(a6)                     | +066
        lea     .L02baae(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L02baae:
        subq.b  #0x1,0x91(a6)                   | +070
        bne.w   .L02bac6                        | +074
        move.w  #0xfd60,0x28(a6)                | +078
        clr.w   0x2c(a6)                        | +07e
        lea     .L02bac6(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L02bac6:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +088
        jsr     Slug_PhysicsE_02a7d8(pc)        | +08c
        bcc.w   .L02bad8                        | +090
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +094
        move.l  a1,(a6)                         | +098
.L02bad8:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +09a
        bcc.w   .L02bae6                        | +09e
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L02bae6:
        jsr     0x28d70.l                       | +0a8
        bcc.w   .L02baf6                        | +0ae
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +0b2
        move.l  a1,(a6)                         | +0b6
.L02baf6:
        jsr     0x283d8.l                       | +0b8
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +0be
        bcs.w   .L02bb0a                        | +0c2
        lea     Slug_BrakeLeft_02be32(pc),a1    | +0c6
        move.l  a1,(a6)                         | +0ca
.L02bb0a:
        jsr     Slug_CallGroundProbeA_02a328(pc) | +0cc
        bcc.w   .L02bb18                        | +0d0
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +0d4
        move.l  a1,(a6)                         | +0d8
.L02bb18:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +0da
        bcc.w   .L02bb38                        | +0de
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0e2
        andi.w  #0xff,d0                        | +0e6
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0ea
        movea.l #0xffffffff,a1                  | +0ee
        jsr     0x772.l                         | +0f4
.L02bb38:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0fa
        bcc.w   .L02bb46                        | +0fe
        lea     Slug_Fall_02d02e(pc),a1         | +102
        move.l  a1,(a6)                         | +106
.L02bb46:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +108
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +10c
        movea.l #0xffffffff,a1                  | +110
        jsr     0x772.l                         | +116
        jsr     Slug_CanFire_02aac0(pc)         | +11c
        bcc.w   .L02bb68                        | +120
        lea     Slug_FireFlat_02c24a(pc),a1     | +124
        move.l  a1,(a6)                         | +128
.L02bb68:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +12a
        bcc.w   .L02bb76                        | +12e
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +132
        move.l  a1,(a6)                         | +136
.L02bb76:
        jsr     JsrAbsThunk_02a5cc(pc)          | +138
        bcc.w   .L02bb84                        | +13c
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +140
        move.l  a1,(a6)                         | +144
.L02bb84:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +146
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +14a
        movea.l #0xffffffff,a1                  | +14e

| ----------------------------------------------------------------------------
|  Slug_DestroyedMusicTbl_02bb9a  @ $02BB9A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DestroyedMusicTbl_02bb9a, "ax", @progbits
        .global Slug_DestroyedMusicTbl_02bb9a
Slug_DestroyedMusicTbl_02bb9a:
        .dc.w   0x10b2                        | +000  (dato / opcode no decodificado)
        .dc.w   0x10b2                        | +002  (dato / opcode no decodificado)
        .dc.w   0x10b2                        | +004  (dato / opcode no decodificado)
        .dc.w   0x10b2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x10b2                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DestroyedSlide_02bba4  @ $02BBA4  (78 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DestroyedSlide_02bba4, "ax", @progbits
        .global Slug_DestroyedSlide_02bba4
Slug_DestroyedSlide_02bba4:
        move.w  #0x600,0x28(a6)                 | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +00a
        movea.l #0x27936c,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02bbce                        | +020
        jsr     0x28cd4.l                       | +024
.L02bbce:
        move.b  #0x19,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        move.w  #0x10b2,d0                      | +03a
        jsr     0x2352.l                        | +03e
        lea     Slug_DestroyedSlide_Loop_02bc78(pc),a1 | +044
        move.l  a1,(a6)                         | +048
        bra.w   Slug_DestroyedSlide_Loop_02bc78 | +04a

| ----------------------------------------------------------------------------
|  Slug_DestroyedSlideInit_02bbf2  @ $02BBF2  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DestroyedSlideInit_02bbf2, "ax", @progbits
        .global Slug_DestroyedSlideInit_02bbf2
Slug_DestroyedSlideInit_02bbf2:
        move.w  #0x10e9,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     Slug_KillInit_02feda(pc)        | +00a
        move.w  #0x600,d0                       | +00e
        move.w  d0,0x28(a6)                     | +012
        move.b  #0x30,0x92(a6)                  | +016
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +01c
        movea.l #0x27936c,a0                    | +020
        lsl.w   #0x2,d0                         | +026
        movea.l (a0,d0.w),a0                    | +028
        cmpa.l  #0xffffffff,a0                  | +02c
        beq.w   .L02bc2e                        | +032
        jsr     0x28cd4.l                       | +036
.L02bc2e:
        move.b  #0x19,0x20(a6)                  | +03c
        andi.w  #0xff,d0                        | +042
        lsr.w   #0x2,d0                         | +046
        add.b   d0,0x20(a6)                     | +048
        move.w  #0x10b2,d0                      | +04c
        jsr     0x2352.l                        | +050
        move.l  #0x2964c,0x60(a6)               | +056
        lea     .L02bc56(pc),a1                 | +05e
        move.l  a1,(a6)                         | +062
.L02bc56:
        lea     0xffff.w,a0                     | +064
        move.l  a0,0x48(a6)                     | +068
        cmpi.b  #0x29,0x92(a6)                  | +06c
        bhi.w   Slug_DestroyedSlide_Loop_02bc78 | +072
        move.w  #0x600,0x28(a6)                 | +076
        clr.w   0x2c(a6)                        | +07c
        lea     Slug_DestroyedSlide_Loop_02bc78(pc),a1 | +080
        move.l  a1,(a6)                         | +084

| ----------------------------------------------------------------------------
|  Slug_DestroyedSlide_Loop_02bc78  @ $02BC78  (130 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DestroyedSlide_Loop_02bc78, "ax", @progbits
        .global Slug_DestroyedSlide_Loop_02bc78
Slug_DestroyedSlide_Loop_02bc78:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +000
        jsr     Slug_PhysicsE_02a7d8(pc)        | +004
        bcc.w   .L02bc8a                        | +008
        lea     Slug_Destroyed_02fc70__L02fc8e(pc),a1 | +00c
        move.l  a1,(a6)                         | +010
.L02bc8a:
        jsr     0x28d70.l                       | +012
        move.b  0x106f28.l,d0                   | +018
        andi.b  #0x1,d0                         | +01e
        bne.w   .L02bcb2                        | +022
        lea     Slug_HitboxB_029834(pc),a0      | +026
        move.l  a0,0x4c(a6)                     | +02a
        jsr     0x283ca.l                       | +02e
        jsr     0x283ca.l                       | +034
.L02bcb2:
        jsr     0x283d8.l                       | +03a
        jsr     Slug_UpdateDamageSprite_02fae4__L02fb04(pc) | +040
        bcc.w   .L02bcc6                        | +044
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L02bcc6:
        jsr     Slug_ClampField92_02ffb0(pc)    | +04e
        tst.b   0x92(a6)                        | +052
        beq.w   .L02bcd8                        | +056
        subi.b  #0x1,0x92(a6)                   | +05a
.L02bcd8:
        bhi.w   .L02bce2                        | +060
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L02bce2:
        cmpi.w  #0x117,0x22(a6)                 | +06a
        blt.w   .L02bcf2                        | +070
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +074
        move.l  a1,(a6)                         | +078
.L02bcf2:
        clr.w   0x2a(a6)                        | +07a
        clr.w   0x2e(a6)                        | +07e

| ----------------------------------------------------------------------------
|  Slug_BrakeRight_02bd00  @ $02BD00  (298 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_BrakeRight_02bd00, "ax", @progbits
        .global Slug_BrakeRight_02bd00
Slug_BrakeRight_02bd00:
        clr.w   d0                              | +000
        sub.w   0x28(a6),d0                     | +002
        asr.w   #0x3,d0                         | +006
        move.w  d0,0x2c(a6)                     | +008
        move.b  #0x8,0x91(a6)                   | +00c
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +012
        movea.l #0x279358,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02bd32                        | +028
        jsr     0x28cd4.l                       | +02c
.L02bd32:
        move.b  #0xa,0x20(a6)                   | +032
        andi.w  #0xff,d0                        | +038
        lsr.w   #0x2,d0                         | +03c
        add.b   d0,0x20(a6)                     | +03e
        lea     .L02bd48(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L02bd48:
        subq.b  #0x1,0x91(a6)                   | +048
        bne.w   .L02bd5e                        | +04c
        clr.w   0x28(a6)                        | +050
        clr.w   0x2c(a6)                        | +054
        lea     .L02bd5e(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L02bd5e:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +05e
        jsr     Slug_PhysicsE_02a7d8(pc)        | +062
        bcc.w   .L02bd70                        | +066
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L02bd70:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +070
        bcc.w   .L02bd7e                        | +074
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L02bd7e:
        jsr     0x28d70.l                       | +07e
        bcc.w   .L02bd8e                        | +084
        lea     Slug_IdleFlat_02b38c(pc),a1     | +088
        move.l  a1,(a6)                         | +08c
.L02bd8e:
        jsr     0x283d8.l                       | +08e
        jsr     Slug_CallGroundProbeA_02a328(pc) | +094
        bcc.w   .L02bda2                        | +098
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +09c
        move.l  a1,(a6)                         | +0a0
.L02bda2:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +0a2
        bcc.w   .L02bdb0                        | +0a6
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +0aa
        move.l  a1,(a6)                         | +0ae
.L02bdb0:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +0b0
        bcc.w   .L02bdd0                        | +0b4
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0b8
        andi.w  #0xff,d0                        | +0bc
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0c0
        movea.l #0xffffffff,a1                  | +0c4
        jsr     0x772.l                         | +0ca
.L02bdd0:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0d0
        bcc.w   .L02bdde                        | +0d4
        lea     Slug_Fall_02d02e(pc),a1         | +0d8
        move.l  a1,(a6)                         | +0dc
.L02bdde:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0de
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0e2
        movea.l #0xffffffff,a1                  | +0e6
        jsr     0x772.l                         | +0ec
        jsr     Slug_CanFire_02aac0(pc)         | +0f2
        bcc.w   .L02be00                        | +0f6
        lea     Slug_FireFlat_02c24a(pc),a1     | +0fa
        move.l  a1,(a6)                         | +0fe
.L02be00:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +100
        bcc.w   .L02be0e                        | +104
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +108
        move.l  a1,(a6)                         | +10c
.L02be0e:
        jsr     JsrAbsThunk_02a5cc(pc)          | +10e
        bcc.w   .L02be1c                        | +112
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L02be1c:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +11c
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +120
        movea.l #0xffffffff,a1                  | +124

| ----------------------------------------------------------------------------
|  Slug_BrakeLeft_02be32  @ $02BE32  (298 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_BrakeLeft_02be32, "ax", @progbits
        .global Slug_BrakeLeft_02be32
Slug_BrakeLeft_02be32:
        clr.w   d0                              | +000
        sub.w   0x28(a6),d0                     | +002
        asr.w   #0x3,d0                         | +006
        move.w  d0,0x2c(a6)                     | +008
        move.b  #0x8,0x91(a6)                   | +00c
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +012
        movea.l #0x2794fc,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02be64                        | +028
        jsr     0x28cd4.l                       | +02c
.L02be64:
        move.b  #0xf,0x20(a6)                   | +032
        andi.w  #0xff,d0                        | +038
        lsr.w   #0x2,d0                         | +03c
        add.b   d0,0x20(a6)                     | +03e
        lea     .L02be7a(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L02be7a:
        subq.b  #0x1,0x91(a6)                   | +048
        bne.w   .L02be90                        | +04c
        clr.w   0x28(a6)                        | +050
        clr.w   0x2c(a6)                        | +054
        lea     .L02be90(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L02be90:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +05e
        jsr     Slug_PhysicsE_02a7d8(pc)        | +062
        bcc.w   .L02bea2                        | +066
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L02bea2:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +070
        bcc.w   .L02beb0                        | +074
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +078
        move.l  a1,(a6)                         | +07c
.L02beb0:
        jsr     0x28d70.l                       | +07e
        bcc.w   .L02bec0                        | +084
        lea     Slug_IdleFlat_02b38c(pc),a1     | +088
        move.l  a1,(a6)                         | +08c
.L02bec0:
        jsr     0x283d8.l                       | +08e
        jsr     Slug_CallGroundProbeA_02a328(pc) | +094
        bcc.w   .L02bed4                        | +098
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +09c
        move.l  a1,(a6)                         | +0a0
.L02bed4:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +0a2
        bcc.w   .L02bee2                        | +0a6
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +0aa
        move.l  a1,(a6)                         | +0ae
.L02bee2:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +0b0
        bcc.w   .L02bf02                        | +0b4
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0b8
        andi.w  #0xff,d0                        | +0bc
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0c0
        movea.l #0xffffffff,a1                  | +0c4
        jsr     0x772.l                         | +0ca
.L02bf02:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0d0
        bcc.w   .L02bf10                        | +0d4
        lea     Slug_Fall_02d02e(pc),a1         | +0d8
        move.l  a1,(a6)                         | +0dc
.L02bf10:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0de
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0e2
        movea.l #0xffffffff,a1                  | +0e6
        jsr     0x772.l                         | +0ec
        jsr     Slug_CanFire_02aac0(pc)         | +0f2
        bcc.w   .L02bf32                        | +0f6
        lea     Slug_FireFlat_02c24a(pc),a1     | +0fa
        move.l  a1,(a6)                         | +0fe
.L02bf32:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +100
        bcc.w   .L02bf40                        | +104
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +108
        move.l  a1,(a6)                         | +10c
.L02bf40:
        jsr     JsrAbsThunk_02a5cc(pc)          | +10e
        bcc.w   .L02bf4e                        | +112
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L02bf4e:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +11c
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +120
        movea.l #0xffffffff,a1                  | +124

| ----------------------------------------------------------------------------
|  Slug_CruiseRightB_02bf64  @ $02BF64  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CruiseRightB_02bf64, "ax", @progbits
        .global Slug_CruiseRightB_02bf64
Slug_CruiseRightB_02bf64:
        move.w  #0x2a0,0x28(a6)                 | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +00a
        movea.l #0x2793d0,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02bf8e                        | +020
        jsr     0x28cd4.l                       | +024
.L02bf8e:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +02a
        movea.l (a1,d0.w),a0                    | +02e
        move.l  a0,0x48(a6)                     | +032
        move.b  #0x0,0x20(a6)                   | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        lea     .L02bfb0(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L02bfb0:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +04c
        jsr     Slug_PhysicsE_02a7d8(pc)        | +050
        bcc.w   .L02bfc2                        | +054
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L02bfc2:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +05e
        bcc.w   .L02bfd0                        | +062
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L02bfd0:
        jsr     Slug_CallGroundProbeA_02a328(pc) | +06c
        bcs.w   .L02bfde                        | +070
        lea     Slug_BrakeRight_02bd00(pc),a1   | +074
        move.l  a1,(a6)                         | +078
.L02bfde:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +07a
        bcc.w   .L02bfec                        | +07e
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +082
        move.l  a1,(a6)                         | +086
.L02bfec:
        jsr     0x28d70.l                       | +088
        jsr     0x283d8.l                       | +08e
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +094
        bcc.w   .L02c018                        | +098
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +09c
        andi.w  #0xff,d0                        | +0a0
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0a4
        movea.l #0xffffffff,a1                  | +0a8
        jsr     0x772.l                         | +0ae
.L02c018:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0b4
        bcc.w   .L02c026                        | +0b8
        lea     Slug_Fall_02d02e(pc),a1         | +0bc
        move.l  a1,(a6)                         | +0c0
.L02c026:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0c2
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0c6
        movea.l #0xffffffff,a1                  | +0ca
        jsr     0x772.l                         | +0d0
        jsr     Slug_CanFire_02aac0(pc)         | +0d6
        bcc.w   .L02c048                        | +0da
        lea     Slug_FireFlat_02c24a(pc),a1     | +0de
        move.l  a1,(a6)                         | +0e2
.L02c048:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0e4
        bcc.w   .L02c056                        | +0e8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0ec
        move.l  a1,(a6)                         | +0f0
.L02c056:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0f2
        bcc.w   .L02c064                        | +0f6
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +0fa
        move.l  a1,(a6)                         | +0fe
.L02c064:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +100
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +104
        movea.l #0xffffffff,a1                  | +108

| ----------------------------------------------------------------------------
|  Slug_CruiseLeftB_02c07a  @ $02C07A  (270 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CruiseLeftB_02c07a, "ax", @progbits
        .global Slug_CruiseLeftB_02c07a
Slug_CruiseLeftB_02c07a:
        move.w  #0xfd60,0x28(a6)                | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +00a
        movea.l #0x279524,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02c0a4                        | +020
        jsr     0x28cd4.l                       | +024
.L02c0a4:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +02a
        movea.l (a1,d0.w),a0                    | +02e
        move.l  a0,0x48(a6)                     | +032
        move.b  #0x5,0x20(a6)                   | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        lea     .L02c0c6(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L02c0c6:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +04c
        jsr     Slug_PhysicsE_02a7d8(pc)        | +050
        bcc.w   .L02c0d8                        | +054
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L02c0d8:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +05e
        bcc.w   .L02c0e6                        | +062
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L02c0e6:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +06c
        bcs.w   .L02c0f4                        | +070
        lea     Slug_BrakeLeft_02be32(pc),a1    | +074
        move.l  a1,(a6)                         | +078
.L02c0f4:
        jsr     Slug_CallGroundProbeA_02a328(pc) | +07a
        bcc.w   .L02c102                        | +07e
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +082
        move.l  a1,(a6)                         | +086
.L02c102:
        jsr     0x28d70.l                       | +088
        jsr     0x283d8.l                       | +08e
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +094
        bcc.w   .L02c12e                        | +098
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +09c
        andi.w  #0xff,d0                        | +0a0
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0a4
        movea.l #0xffffffff,a1                  | +0a8
        jsr     0x772.l                         | +0ae
.L02c12e:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0b4
        bcc.w   .L02c13c                        | +0b8
        lea     Slug_Fall_02d02e(pc),a1         | +0bc
        move.l  a1,(a6)                         | +0c0
.L02c13c:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0c2
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0c6
        movea.l #0xffffffff,a1                  | +0ca
        jsr     0x772.l                         | +0d0
        jsr     Slug_CanFire_02aac0(pc)         | +0d6
        bcc.w   .L02c15e                        | +0da
        lea     Slug_FireFlat_02c24a(pc),a1     | +0de
        move.l  a1,(a6)                         | +0e2
.L02c15e:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0e4
        bcc.w   .L02c16c                        | +0e8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0ec
        move.l  a1,(a6)                         | +0f0
.L02c16c:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0f2
        bcc.w   .L02c17a                        | +0f6
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +0fa
        move.l  a1,(a6)                         | +0fe
.L02c17a:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +100
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +104
        movea.l #0xffffffff,a1                  | +108

| ----------------------------------------------------------------------------
|  Slug_FireIdle_02c190  @ $02C190  (186 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireIdle_02c190, "ax", @progbits
        .global Slug_FireIdle_02c190
Slug_FireIdle_02c190:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x279498,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02c1b6                        | +01c
        jsr     0x28cd4.l                       | +020
.L02c1b6:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +026
        movea.l (a1,d0.w),a0                    | +02a
        move.l  a0,0x48(a6)                     | +02e
        move.b  #0x14,0x20(a6)                  | +032
        andi.w  #0xff,d0                        | +038
        lsr.w   #0x2,d0                         | +03c
        add.b   d0,0x20(a6)                     | +03e
        move.b  #0x4,d1                         | +042
        jsr     0x8f714.l                       | +046
        bcs.w   .L02c22e                        | +04c
        cmpi.b  #0xff,0x84(a6)                  | +050
        beq.w   .L02c200                        | +056
        lea     0x311c0.l,a1                    | +05a
        jsr     0x6fe.l                         | +060
        jsr     0x5dd02.l                       | +066
        bra.w   .L02c212                        | +06c
.L02c200:
        lea     0x31240.l,a1                    | +070
        jsr     0x6fe.l                         | +076
        jsr     0x5dd02.l                       | +07c
.L02c212:
        jsr     0x517fe.l                       | +082
        lea     0x319f0.l,a1                    | +088
        jsr     0x6fe.l                         | +08e
        jsr     0x5dd02.l                       | +094
        bra.w   .L02c246                        | +09a
.L02c22e:
        lea     0x31944.l,a1                    | +09e
        jsr     0x6fe.l                         | +0a4
        jsr     0x5dd02.l                       | +0aa
        jsr     0x517fe.l                       | +0b0
.L02c246:
        bra.w   Slug_FireFlat_02c24a__L02c300   | +0b6

| ----------------------------------------------------------------------------
|  Slug_FireFlat_02c24a  @ $02C24A  (480 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireFlat_02c24a, "ax", @progbits
        .global Slug_FireFlat_02c24a
Slug_FireFlat_02c24a:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x279498,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02c270                        | +01c
        jsr     0x28cd4.l                       | +020
.L02c270:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +026
        movea.l (a1,d0.w),a0                    | +02a
        move.l  a0,0x48(a6)                     | +02e
        move.b  #0x3c,0x20(a6)                  | +032
        andi.w  #0xff,d0                        | +038
        lsr.w   #0x2,d0                         | +03c
        add.b   d0,0x20(a6)                     | +03e
        move.b  #0x4,d1                         | +042
        jsr     0x8f714.l                       | +046
        bcs.w   .L02c2e8                        | +04c
        cmpi.b  #0xff,0x84(a6)                  | +050
        beq.w   .L02c2ba                        | +056
        lea     0x312b2.l,a1                    | +05a
        jsr     0x6fe.l                         | +060
        jsr     0x5dd02.l                       | +066
        bra.w   .L02c2cc                        | +06c
.L02c2ba:
        lea     0x31240.l,a1                    | +070
        jsr     0x6fe.l                         | +076
        jsr     0x5dd02.l                       | +07c
.L02c2cc:
        jsr     0x517fe.l                       | +082
        lea     0x319f0.l,a1                    | +088
        jsr     0x6fe.l                         | +08e
        jsr     0x5dd02.l                       | +094
        bra.w   Slug_FireFlat_02c24a__L02c300   | +09a
.L02c2e8:
        lea     0x31944.l,a1                    | +09e
        jsr     0x6fe.l                         | +0a4
        jsr     0x5dd02.l                       | +0aa
        jsr     0x517fe.l                       | +0b0
        .global Slug_FireFlat_02c24a__L02c300
Slug_FireFlat_02c24a__L02c300:
        cmpi.b  #0x0,0x90(a6)                   | +0b6
        beq.w   .L02c30e                        | +0bc
        jsr     Slug_GaugeTick_02aab0(pc)       | +0c0
.L02c30e:
        move.b  #0x1e,0x8e(a6)                  | +0c4
        bset    #0x0,0x8d(a6)                   | +0ca
        move.l  #0x295b4,0x60(a6)               | +0d0
        jsr     0x5cf1c.l                       | +0d8
        bcs.w   .L02c334                        | +0de
        clr.w   0x28(a6)                        | +0e2
        clr.w   0x2c(a6)                        | +0e6
.L02c334:
        lea     Slug_FireFlat_02c24a__L02c33a(pc),a1 | +0ea
        move.l  a1,(a6)                         | +0ee
        .global Slug_FireFlat_02c24a__L02c33a
Slug_FireFlat_02c24a__L02c33a:
        jsr     Slug_CanFire_02aac0(pc)         | +0f0
        bcc.w   .L02c348                        | +0f4
        lea     Slug_FireFlat_02c24a(pc),a1     | +0f8
        move.l  a1,(a6)                         | +0fc
.L02c348:
        jsr     ClearXN_02abc0(pc)              | +0fe
        bcs.w   .L02c36c                        | +102
        jsr     Slug_CallGroundProbeA_02a328(pc) | +106
        bcc.w   .L02c35e                        | +10a
        lea     Slug_FireMoveRight_02c572(pc),a1 | +10e
        move.l  a1,(a6)                         | +112
.L02c35e:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +114
        bcc.w   .L02c36c                        | +118
        lea     Slug_FireMoveLeft_02c648(pc),a1 | +11c
        move.l  a1,(a6)                         | +120
.L02c36c:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +122
        move.w  0x28(a6),d0                     | +126
        move.w  #0x540,d1                       | +12a
        jsr     0x267f4.l                       | +12e
        move.w  d0,0x28(a6)                     | +134
        jsr     ClearXN_02abc0(pc)              | +138
        bcc.w   .L02c392                        | +13c
        clr.w   0x28(a6)                        | +140
        clr.w   0x2c(a6)                        | +144
.L02c392:
        jsr     Slug_PhysicsG_02a85a(pc)        | +148
        bcc.w   .L02c3a0                        | +14c
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +150
        move.l  a1,(a6)                         | +154
.L02c3a0:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +156
        jsr     0x28d70.l                       | +15a
        bcc.w   .L02c3b4                        | +160
        lea     Slug_FireRecoil_02c432(pc),a1   | +164
        move.l  a1,(a6)                         | +168
.L02c3b4:
        cmpi.b  #0x10,0x8e(a6)                  | +16a
        bcc.w   .L02c3f2                        | +170
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +174
        bcc.w   .L02c3de                        | +178
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +17c
        andi.w  #0xff,d0                        | +180
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +184
        movea.l #0xffffffff,a1                  | +188
        jsr     0x772.l                         | +18e
.L02c3de:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +194
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +198
        movea.l #0xffffffff,a1                  | +19c
        jsr     0x772.l                         | +1a2
.L02c3f2:
        jsr     Slug_GroundContact_02a8c0(pc)   | +1a8
        bcc.w   .L02c400                        | +1ac
        lea     Slug_Fall_02d02e(pc),a1         | +1b0
        move.l  a1,(a6)                         | +1b4
.L02c400:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +1b6
        bcc.w   .L02c40e                        | +1ba
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1be
        move.l  a1,(a6)                         | +1c2
.L02c40e:
        jsr     JsrAbsThunk_02a5cc(pc)          | +1c4
        bcc.w   .L02c41c                        | +1c8
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +1cc
        move.l  a1,(a6)                         | +1d0
.L02c41c:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +1d2
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +1d6
        movea.l #0xffffffff,a1                  | +1da

| ----------------------------------------------------------------------------
|  Slug_FireRecoil_02c432  @ $02C432  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireRecoil_02c432, "ax", @progbits
        .global Slug_FireRecoil_02c432
Slug_FireRecoil_02c432:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x2794e8,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02c458                        | +01c
        jsr     0x28cd4.l                       | +020
.L02c458:
        move.b  #0x14,0x20(a6)                  | +026
        andi.w  #0xff,d0                        | +02c
        lsr.w   #0x2,d0                         | +030
        add.b   d0,0x20(a6)                     | +032
        lea     .L02c46e(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L02c46e:
        clr.w   0x28(a6)                        | +03c
        clr.w   0x2c(a6)                        | +040
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +044
        jsr     ClearXN_02abc0(pc)              | +048
        bcs.w   .L02c49e                        | +04c
        jsr     Slug_CallGroundProbeA_02a328(pc) | +050
        bcc.w   .L02c490                        | +054
        lea     Slug_AccelRightB_02b8d8__L02b8de(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L02c490:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +05e
        bcc.w   .L02c49e                        | +062
        lea     Slug_AccelLeftB_02ba3e__L02ba44(pc),a1 | +066
        move.l  a1,(a6)                         | +06a
.L02c49e:
        move.w  0x28(a6),d0                     | +06c
        move.w  #0x540,d1                       | +070
        jsr     0x267f4.l                       | +074
        move.w  d0,0x28(a6)                     | +07a
        jsr     Slug_PhysicsE_02a7d8(pc)        | +07e
        bcc.w   .L02c4be                        | +082
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +086
        move.l  a1,(a6)                         | +08a
.L02c4be:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +08c
        jsr     0x28d70.l                       | +090
        bcc.w   .L02c4d2                        | +096
        lea     Slug_IdleFlat_02b38c(pc),a1     | +09a
        move.l  a1,(a6)                         | +09e
.L02c4d2:
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +0a0
        bcc.w   .L02c4f2                        | +0a4
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0a8
        andi.w  #0xff,d0                        | +0ac
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +0b0
        movea.l #0xffffffff,a1                  | +0b4
        jsr     0x772.l                         | +0ba
.L02c4f2:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0c0
        bcc.w   .L02c500                        | +0c4
        lea     Slug_Fall_02d02e(pc),a1         | +0c8
        move.l  a1,(a6)                         | +0cc
.L02c500:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +0ce
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0d2
        movea.l #0xffffffff,a1                  | +0d6
        jsr     0x772.l                         | +0dc
        jsr     Slug_CanFire_02aac0(pc)         | +0e2
        bcc.w   .L02c522                        | +0e6
        lea     Slug_FireFlat_02c24a(pc),a1     | +0ea
        move.l  a1,(a6)                         | +0ee
.L02c522:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0f0
        bcc.w   .L02c530                        | +0f4
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0f8
        move.l  a1,(a6)                         | +0fc
.L02c530:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0fe
        bcc.w   .L02c53e                        | +102
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +106
        move.l  a1,(a6)                         | +10a
.L02c53e:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +10c
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +110
        movea.l #0xffffffff,a1                  | +114

| ----------------------------------------------------------------------------
|  Slug_FireFlatResume_02c554  @ $02C554  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireFlatResume_02c554, "ax", @progbits
        .global Slug_FireFlatResume_02c554
Slug_FireFlatResume_02c554:
        jsr     0x267e6.l                       | +000
        move.w  #0x0,0x28(a6)                   | +006
        move.l  #0x295b4,0x60(a6)               | +00c
        lea     Slug_FireFlat_02c24a__L02c33a(pc),a1 | +014
        move.l  a1,(a6)                         | +018
        jmp     Slug_FireFlat_02c24a__L02c33a(pc) | +01a

| ----------------------------------------------------------------------------
|  Slug_FireMoveRight_02c572  @ $02C572  (206 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireMoveRight_02c572, "ax", @progbits
        .global Slug_FireMoveRight_02c572
Slug_FireMoveRight_02c572:
        move.w  #0x2a0,0x28(a6)                 | +000
        clr.w   0x2c(a6)                        | +006
        lea     .L02c582(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L02c582:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +010
        jsr     Slug_PhysicsE_02a7d8(pc)        | +014
        bcc.w   .L02c594                        | +018
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +01c
        move.l  a1,(a6)                         | +020
.L02c594:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +022
        bcc.w   .L02c5a2                        | +026
        lea     Slug_IdleSlopeJmp_02b4ca(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L02c5a2:
        jsr     Slug_CallGroundProbeA_02a328(pc) | +030
        bcs.w   .L02c5b0                        | +034
        lea     Slug_FireFlatResume_02c554(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L02c5b0:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +03e
        bcc.w   .L02c5be                        | +042
        lea     Slug_FireMoveLeft_02c648(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L02c5be:
        jsr     0x28d70.l                       | +04c
        bcc.w   .L02c5ce                        | +052
        lea     Slug_CruiseRightB_02bf64(pc),a1 | +056
        move.l  a1,(a6)                         | +05a
.L02c5ce:
        jsr     0x283d8.l                       | +05c
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +062
        bcc.w   .L02c5f4                        | +066
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +06a
        andi.w  #0xff,d0                        | +06e
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +072
        movea.l #0xffffffff,a1                  | +076
        jsr     0x772.l                         | +07c
.L02c5f4:
        jsr     Slug_GroundContact_02a8c0(pc)   | +082
        bcc.w   .L02c602                        | +086
        lea     Slug_Fall_02d02e(pc),a1         | +08a
        move.l  a1,(a6)                         | +08e
.L02c602:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +090
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +094
        movea.l #0xffffffff,a1                  | +098
        jsr     0x772.l                         | +09e
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0a4
        bcc.w   .L02c624                        | +0a8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0ac
        move.l  a1,(a6)                         | +0b0
.L02c624:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0b2
        bcc.w   .L02c632                        | +0b6
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +0ba
        move.l  a1,(a6)                         | +0be
.L02c632:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +0c0
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +0c4
        movea.l #0xffffffff,a1                  | +0c8

| ----------------------------------------------------------------------------
|  Slug_FireMoveLeft_02c648  @ $02C648  (206 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireMoveLeft_02c648, "ax", @progbits
        .global Slug_FireMoveLeft_02c648
Slug_FireMoveLeft_02c648:
        move.w  #0xfd60,0x28(a6)                | +000
        clr.w   0x2c(a6)                        | +006
        lea     .L02c658(pc),a1                 | +00a
        move.l  a1,(a6)                         | +00e
.L02c658:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +010
        jsr     Slug_PhysicsE_02a7d8(pc)        | +014
        bcc.w   .L02c66a                        | +018
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +01c
        move.l  a1,(a6)                         | +020
.L02c66a:
        jsr     Slug_UpdateAngleIsSlope_02a4f0(pc) | +022
        bcc.w   .L02c678                        | +026
        lea     Slug_IdleSlopeJmp_02b4ca__L02b4ce(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L02c678:
        jsr     Slug_CallGroundProbeB_02a34e(pc) | +030
        bcs.w   .L02c686                        | +034
        lea     Slug_FireFlatResume_02c554(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L02c686:
        jsr     Slug_CallGroundProbeA_02a328(pc) | +03e
        bcc.w   .L02c694                        | +042
        lea     Slug_FireMoveRight_02c572(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L02c694:
        jsr     0x28d70.l                       | +04c
        bcc.w   .L02c6a4                        | +052
        lea     Slug_CruiseLeftB_02c07a(pc),a1  | +056
        move.l  a1,(a6)                         | +05a
.L02c6a4:
        jsr     0x283d8.l                       | +05c
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +062
        bcc.w   .L02c6ca                        | +066
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +06a
        andi.w  #0xff,d0                        | +06e
        lea     Slug_StateByAnglePtrTbl_02a060__L02a078(pc),a0 | +072
        movea.l #0xffffffff,a1                  | +076
        jsr     0x772.l                         | +07c
.L02c6ca:
        jsr     Slug_GroundContact_02a8c0(pc)   | +082
        bcc.w   .L02c6d8                        | +086
        lea     Slug_Fall_02d02e(pc),a1         | +08a
        move.l  a1,(a6)                         | +08e
.L02c6d8:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +090
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +094
        movea.l #0xffffffff,a1                  | +098
        jsr     0x772.l                         | +09e
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0a4
        bcc.w   .L02c6fa                        | +0a8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0ac
        move.l  a1,(a6)                         | +0b0
.L02c6fa:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0b2
        bcc.w   .L02c708                        | +0b6
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +0ba
        move.l  a1,(a6)                         | +0be
.L02c708:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +0c0
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +0c4
        movea.l #0xffffffff,a1                  | +0c8

| ----------------------------------------------------------------------------
|  Slug_FireSlope_02c71e  @ $02C71E  (474 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireSlope_02c71e, "ax", @progbits
        .global Slug_FireSlope_02c71e
Slug_FireSlope_02c71e:
        bclr    #0x5,0x8c(a6)                   | +000
        move.b  #0x4,d1                         | +006
        jsr     0x8f714.l                       | +00a
        bcs.w   .L02c780                        | +010
        cmpi.b  #0xff,0x84(a6)                  | +014
        beq.w   .L02c752                        | +01a
        lea     0x312b2.l,a1                    | +01e
        jsr     0x6fe.l                         | +024
        jsr     0x5dd02.l                       | +02a
        bra.w   .L02c764                        | +030
.L02c752:
        lea     0x31240.l,a1                    | +034
        jsr     0x6fe.l                         | +03a
        jsr     0x5dd02.l                       | +040
.L02c764:
        jsr     0x517fe.l                       | +046
        lea     0x319f0.l,a1                    | +04c
        jsr     0x6fe.l                         | +052
        jsr     0x5dd02.l                       | +058
        bra.w   .L02c798                        | +05e
.L02c780:
        lea     0x31922.l,a1                    | +062
        jsr     0x6fe.l                         | +068
        jsr     0x5dd02.l                       | +06e
        jsr     0x517fe.l                       | +074
.L02c798:
        cmpi.b  #0x0,0x90(a6)                   | +07a
        beq.w   .L02c7a6                        | +080
        jsr     Slug_GaugeTick_02aab0(pc)       | +084
.L02c7a6:
        move.b  #0x1e,0x8e(a6)                  | +088
        jsr     Slug_TerrainSlope_02a958(pc)    | +08e
        cmp.w   0x80(a6),d2                     | +092
        beq.w   .L02c80a                        | +096
        move.w  d2,0x80(a6)                     | +09a
        move.w  d2,d0                           | +09e
        asr.w   #0x1,d0                         | +0a0
        neg.w   d0                              | +0a2
        addi.w  #0x20,d0                        | +0a4
        cmpi.w  #0x40,d0                        | +0a8
        bls.w   .L02c7d2                        | +0ac
        move.w  #0x40,d0                        | +0b0
.L02c7d2:
        movem.w d0,-(a7)                        | +0b4
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +0b8
        movea.l #0x2b0c30,a0                    | +0bc
        lsl.w   #0x2,d0                         | +0c2
        movea.l (a0,d0.w),a0                    | +0c4
        cmpa.l  #0xffffffff,a0                  | +0c8
        beq.w   .L02c7f6                        | +0ce
        jsr     0x28cd4.l                       | +0d2
.L02c7f6:
        movem.w (a7)+,d0                        | +0d8
        move.b  #0x0,0x20(a6)                   | +0dc
        lsr.w   #0x2,d0                         | +0e2
        andi.w  #0xff,d0                        | +0e4
        add.b   d0,0x20(a6)                     | +0e8
.L02c80a:
        bset    #0x0,0x8d(a6)                   | +0ec
        move.l  #0x295b4,0x60(a6)               | +0f2
        lea     .L02c81e(pc),a1                 | +0fa
        move.l  a1,(a6)                         | +0fe
.L02c81e:
        bset    #0x2,0x8d(a6)                   | +100
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +106
        move.w  0x2a(a6),d0                     | +10a
        move.w  #0x400,d1                       | +10e
        jsr     0x267f4.l                       | +112
        move.w  d0,0x2a(a6)                     | +118
        jsr     Slug_PhysicsG_02a85a(pc)        | +11c
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +120
        jsr     0x28d70.l                       | +124
        jsr     Slug_TerrainSlope_02a958(pc)    | +12a
        cmp.w   0x80(a6),d2                     | +12e
        beq.w   .L02c8a6                        | +132
        move.w  d2,0x80(a6)                     | +136
        move.w  d2,d0                           | +13a
        asr.w   #0x1,d0                         | +13c
        neg.w   d0                              | +13e
        addi.w  #0x20,d0                        | +140
        cmpi.w  #0x40,d0                        | +144
        bls.w   .L02c86e                        | +148
        move.w  #0x40,d0                        | +14c
.L02c86e:
        movem.w d0,-(a7)                        | +150
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +154
        movea.l #0x2b0c30,a0                    | +158
        lsl.w   #0x2,d0                         | +15e
        movea.l (a0,d0.w),a0                    | +160
        cmpa.l  #0xffffffff,a0                  | +164
        beq.w   .L02c892                        | +16a
        jsr     0x28cd4.l                       | +16e
.L02c892:
        movem.w (a7)+,d0                        | +174
        move.b  #0x0,0x20(a6)                   | +178
        lsr.w   #0x2,d0                         | +17e
        andi.w  #0xff,d0                        | +180
        add.b   d0,0x20(a6)                     | +184
.L02c8a6:
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +188
        move.l  a1,(a6)                         | +18c
        jsr     Slug_GroundContact_02a8c0(pc)   | +18e
        bcc.w   .L02c8ba                        | +192
        lea     Slug_Fall_02d02e(pc),a1         | +196
        move.l  a1,(a6)                         | +19a
.L02c8ba:
        jsr     Slug_InputDirByLayoutA_02aaf0(pc) | +19c
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +1a0
        movea.l #0xffffffff,a1                  | +1a4
        jsr     0x772.l                         | +1aa
        jsr     Slug_CanFire_02aac0(pc)         | +1b0
        bcc.w   .L02c8dc                        | +1b4
        lea     Slug_FireSlope_02c71e(pc),a1    | +1b8
        move.l  a1,(a6)                         | +1bc
.L02c8dc:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +1be
        bcc.w   .L02c8ea                        | +1c2
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1c6
        move.l  a1,(a6)                         | +1ca
.L02c8ea:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +1cc
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +1d0
        movea.l #0xffffffff,a1                  | +1d4

| ----------------------------------------------------------------------------
|  Slug_AirSteerAccelTbl_02c900  @ $02C900  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AirSteerAccelTbl_02c900, "ax", @progbits
        .global Slug_AirSteerAccelTbl_02c900
Slug_AirSteerAccelTbl_02c900:
        .dc.w   0x0000                        | +000  (dato / opcode no decodificado)
        .dc.w   0xff80                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0080                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_JumpCrouch_02c908  @ $02C908  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpCrouch_02c908, "ax", @progbits
        .global Slug_JumpCrouch_02c908
Slug_JumpCrouch_02c908:
        bclr    #0x5,0x8c(a6)                   | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +00a
        movea.l #0x279308,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02c932                        | +020
        jsr     0x28cd4.l                       | +024
.L02c932:
        lea     Slug_AttackPtrTbl_02a024__L02a038(pc),a1 | +02a
        movea.l (a1,d0.w),a0                    | +02e
        move.l  a0,0x48(a6)                     | +032
        move.b  #0x2d,0x20(a6)                  | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        move.l  #0x295b4,0x60(a6)               | +046
        lea     Slug_JumpCrouch_Loop_02c95c(pc),a1 | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  Slug_JumpCrouch_Loop_02c95c  @ $02C95C  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpCrouch_Loop_02c95c, "ax", @progbits
        .global Slug_JumpCrouch_Loop_02c95c
Slug_JumpCrouch_Loop_02c95c:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +000
        jsr     Slug_PhysicsG_02a85a(pc)        | +004
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +008
        jsr     0x28d70.l                       | +00c
        bcc.w   .L02c978                        | +012
        lea     Slug_JumpLaunch_02ca0c(pc),a1   | +016
        move.l  a1,(a6)                         | +01a
.L02c978:
        jsr     0x283d8.l                       | +01c
        jsr     Slug_CanFire_02aac0(pc)         | +022
        bcc.w   .L02c98c                        | +026
        lea     Slug_FireJumpCrouch_02d286(pc),a1 | +02a
        move.l  a1,(a6)                         | +02e
.L02c98c:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +030
        bcc.w   .L02c99a                        | +034
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +038
        move.l  a1,(a6)                         | +03c
.L02c99a:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +03e
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +042
        movea.l #0xffffffff,a1                  | +046

| ----------------------------------------------------------------------------
|  Slug_InputDirPtrTbl_02c9b0  @ $02C9B0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_InputDirPtrTbl_02c9b0, "ax", @progbits
        .global Slug_InputDirPtrTbl_02c9b0
Slug_InputDirPtrTbl_02c9b0:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xc9c4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xc9f0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xc9d4                        | +00e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +010  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_JumpNeutral_02c9c4  @ $02C9C4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpNeutral_02c9c4, "ax", @progbits
        .global Slug_JumpNeutral_02c9c4
Slug_JumpNeutral_02c9c4:
        bclr    #0x5,0x8c(a6)                   | +000
        move.w  #0x0,0x28(a6)                   | +006
        bra.w   Slug_JumpCrouch_02c908          | +00c

| ----------------------------------------------------------------------------
|  Slug_JumpRight_02c9d4  @ $02C9D4  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpRight_02c9d4, "ax", @progbits
        .global Slug_JumpRight_02c9d4
Slug_JumpRight_02c9d4:
        cmpi.w  #0x3,0x94(a6)                   | +000
        bne.w   .L02c9e0                        | +006
        bra.b   Slug_JumpNeutral_02c9c4         | +00a
.L02c9e0:
        bclr    #0x5,0x8c(a6)                   | +00c
        move.w  #0x2aa,0x28(a6)                 | +012
        bra.w   Slug_JumpCrouch_02c908          | +018

| ----------------------------------------------------------------------------
|  Slug_JumpLeft_02c9f0  @ $02C9F0  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpLeft_02c9f0, "ax", @progbits
        .global Slug_JumpLeft_02c9f0
Slug_JumpLeft_02c9f0:
        cmpi.w  #0x4,0x94(a6)                   | +000
        bne.w   .L02c9fc                        | +006
        bra.b   Slug_JumpNeutral_02c9c4         | +00a
.L02c9fc:
        bclr    #0x5,0x8c(a6)                   | +00c
        move.w  #0xfd56,0x28(a6)                | +012
        bra.w   Slug_JumpCrouch_02c908          | +018

| ----------------------------------------------------------------------------
|  Slug_JumpLaunch_02ca0c  @ $02CA0C  (126 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpLaunch_02ca0c, "ax", @progbits
        .global Slug_JumpLaunch_02ca0c
Slug_JumpLaunch_02ca0c:
        bclr    #0x3,0x5b(a6)                   | +000
        bclr    #0x5,0x8c(a6)                   | +006
        move.w  #0x4a4,0x2a(a6)                 | +00c
        move.w  #0xff9d,0x2e(a6)                | +012
        move.w  0x24(a6),d0                     | +018
        cmp.w   0x82(a6),d0                     | +01c
        ble.w   .L02ca34                        | +020
        move.w  d0,0x82(a6)                     | +024
.L02ca34:
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +028
        movea.l #0x27931c,a0                    | +02c
        lsl.w   #0x2,d0                         | +032
        movea.l (a0,d0.w),a0                    | +034
        cmpa.l  #0xffffffff,a0                  | +038
        beq.w   .L02ca54                        | +03e
        jsr     0x28cd4.l                       | +042
.L02ca54:
        lea     Slug_AttackPtrTbl_02a024__L02a04c(pc),a1 | +048
        movea.l (a1,d0.w),a0                    | +04c
        move.l  a0,0x48(a6)                     | +050
        move.b  #0x32,0x20(a6)                  | +054
        andi.w  #0xff,d0                        | +05a
        lsr.w   #0x2,d0                         | +05e
        add.b   d0,0x20(a6)                     | +060
        lea     Slug_JumpAir_02ca8a(pc),a1      | +064
        move.l  a1,(a6)                         | +068
        jsr     Slug_TerrainSlope_02a958(pc)    | +06a
        cmp.w   0x80(a6),d2                     | +06e
        beq.w   .L02ca86                        | +072
        move.w  d2,0x80(a6)                     | +076
.L02ca86:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +07a

| ----------------------------------------------------------------------------
|  Slug_JumpAir_02ca8a  @ $02CA8A  (376 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpAir_02ca8a, "ax", @progbits
        .global Slug_JumpAir_02ca8a
Slug_JumpAir_02ca8a:
        jsr     Slug_CanFire_02aac0(pc)         | +000
        bcc.w   Slug_JumpAir_02ca8a__L02ca98    | +004
        lea     Slug_FireJumpCrouch_02d286__L02d298(pc),a1 | +008
        move.l  a1,(a6)                         | +00c
        .global Slug_JumpAir_02ca8a__L02ca98
Slug_JumpAir_02ca8a__L02ca98:
        jsr     0x5d5b6.l                       | +00e
        asl.w   #0x1,d0                         | +014
        lea     Slug_AirSteerAccelTbl_02c900(pc),a0 | +016
        move.w  (a0,d0.w),0x2c(a6)              | +01a
        move.w  0x28(a6),d0                     | +020
        move.w  #0x2a0,d1                       | +024
        jsr     0x267f4.l                       | +028
        move.w  d0,0x28(a6)                     | +02e
        move.w  0x2a(a6),d0                     | +032
        bgt.w   .L02cad2                        | +036
        move.w  #0x400,d1                       | +03a
        jsr     0x267f4.l                       | +03e
        move.w  d0,0x2a(a6)                     | +044
.L02cad2:
        bset    #0x1,0x8d(a6)                   | +048
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +04e
        jsr     Slug_PhysicsAir_02a878(pc)      | +052
        bcc.w   .L02cb1a                        | +056
        lea     Slug_JumpLand_02cc1a(pc),a1     | +05a
        move.l  a1,(a6)                         | +05e
        jsr     0x5cef8.l                       | +060
        bcc.w   .L02cb0c                        | +066
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +06a
        andi.w  #0xff,d0                        | +06e
        lea     Slug_StateByAnglePtrTbl_02a060__L02a08c(pc),a0 | +072
        movea.l #0xffffffff,a1                  | +076
        jsr     0x772.l                         | +07c
.L02cb0c:
        jsr     Slug_CanFire_02aac0(pc)         | +082
        bcc.w   .L02cb1a                        | +086
        lea     Slug_FireFlat_02c24a(pc),a1     | +08a
        move.l  a1,(a6)                         | +08e
.L02cb1a:
        jsr     0x28d70.l                       | +090
        tst.w   0x2a(a6)                        | +096
        bgt.w   .L02cbb0                        | +09a
        jsr     Slug_GroundContact_02a8c0(pc)   | +09e
        cmpi.b  #0x0,d1                         | +0a2
        ble.w   .L02cbb0                        | +0a6
        clr.w   0x28(a6)                        | +0aa
        clr.w   0x2c(a6)                        | +0ae
        jsr     Slug_TerrainSlope_02a958(pc)    | +0b2
        cmp.w   0x80(a6),d2                     | +0b6
        beq.w   .L02cbb0                        | +0ba
        move.w  d2,0x80(a6)                     | +0be
        move.w  d2,d0                           | +0c2
        asr.w   #0x1,d0                         | +0c4
        neg.w   d0                              | +0c6
        addi.w  #0x20,d0                        | +0c8
        cmpi.w  #0x40,d0                        | +0cc
        bls.w   .L02cb62                        | +0d0
        move.w  #0x40,d0                        | +0d4
.L02cb62:
        movem.w d0,-(a7)                        | +0d8
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +0dc
        movea.l #0x2b0c30,a0                    | +0e0
        lsl.w   #0x2,d0                         | +0e6
        movea.l (a0,d0.w),a0                    | +0e8
        cmpa.l  #0xffffffff,a0                  | +0ec
        beq.w   .L02cb86                        | +0f2
        jsr     0x28cd4.l                       | +0f6
.L02cb86:
        movem.w (a7)+,d0                        | +0fc
        move.b  #0x0,0x20(a6)                   | +100
        lsr.w   #0x2,d0                         | +106
        andi.w  #0xff,d0                        | +108
        add.b   d0,0x20(a6)                     | +10c
        bset    #0x2,0x8d(a6)                   | +110
        lea     Slug_JumpAir_02ca8a(pc),a0      | +116
        cmpa.l  (a6),a0                         | +11a
        bne.w   .L02cbb0                        | +11c
        lea     Slug_JumpAirFire_02cc02(pc),a1  | +120
        move.l  a1,(a6)                         | +124
.L02cbb0:
        btst    #0x1,0x8d(a6)                   | +126
        beq.w   .L02cbd2                        | +12c
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +130
        lea     Slug_StateByAnglePtrTbl_02a060__L02a06c(pc),a0 | +134
        movea.l #0xffffffff,a1                  | +138
        jsr     0x772.l                         | +13e
        bra.w   .L02cbe6                        | +144
.L02cbd2:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +148
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +14c
        movea.l #0xffffffff,a1                  | +150
        jsr     0x772.l                         | +156
.L02cbe6:
        movea.l #0xffffffff,a0                  | +15c
        lea     0x27964e.l,a0                   | +162
        jsr     0x5dd56.l                       | +168
        bcc.w   .L02cc00                        | +16e
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +172
.L02cc00:
        rts                                     | +176

| ----------------------------------------------------------------------------
|  Slug_JumpAirFire_02cc02  @ $02CC02  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpAirFire_02cc02, "ax", @progbits
        .global Slug_JumpAirFire_02cc02
Slug_JumpAirFire_02cc02:
        bset    #0x2,0x8d(a6)                   | +000
        jsr     Slug_CanFire_02aac0(pc)         | +006
        bcc.w   .L02cc16                        | +00a
        lea     Slug_FireAirB_02d380(pc),a1     | +00e
        move.l  a1,(a6)                         | +012
.L02cc16:
        bra.w   Slug_JumpAir_02ca8a__L02ca98    | +014

| ----------------------------------------------------------------------------
|  Slug_JumpLand_02cc1a  @ $02CC1A  (506 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpLand_02cc1a, "ax", @progbits
        .global Slug_JumpLand_02cc1a
Slug_JumpLand_02cc1a:
        bclr    #0x5,0x8c(a6)                   | +000
        bset    #0x7,0x5b(a6)                   | +006
        clr.w   0x2e(a6)                        | +00c
        clr.w   0x2a(a6)                        | +010
        tst.w   0x28(a6)                        | +014
        beq.w   .L02cc4a                        | +018
        move.w  0x28(a6),d1                     | +01c
        ext.l   d1                              | +020
        swap    d1                              | +022
        move.w  #0x80,d0                        | +024
        eor.w   d1,d0                           | +028
        sub.w   d1,d0                           | +02a
        move.w  d0,0x28(a6)                     | +02c
.L02cc4a:
        jsr     Slug_TerrainSlope_02a958(pc)    | +030
        move.w  d2,0x80(a6)                     | +034
        jsr     Slug_TerrainIsSlope_02a4ec(pc)  | +038
        bcs.w   .L02cc5e                        | +03c
        bra.w   .L02cc68                        | +040
.L02cc5e:
        lea     Slug_JumpLandSlope_02ce1c(pc),a1 | +044
        move.l  a1,(a6)                         | +048
        bra.w   Slug_JumpLandSlope_02ce1c       | +04a
.L02cc68:
        move.b  #0x11,0x91(a6)                  | +04e
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +054
        movea.l #0x279330,a0                    | +058
        lsl.w   #0x2,d0                         | +05e
        movea.l (a0,d0.w),a0                    | +060
        cmpa.l  #0xffffffff,a0                  | +064
        beq.w   .L02cc8e                        | +06a
        jsr     0x28cd4.l                       | +06e
.L02cc8e:
        lea     Slug_AttackTbl10_029fa8__L029ffc(pc),a1 | +074
        movea.l (a1,d0.w),a0                    | +078
        move.l  a0,0x48(a6)                     | +07c
        move.b  #0x37,0x20(a6)                  | +080
        andi.w  #0xff,d0                        | +086
        lsr.w   #0x2,d0                         | +08a
        add.b   d0,0x20(a6)                     | +08c
        lea     .L02ccb0(pc),a1                 | +090
        move.l  a1,(a6)                         | +094
.L02ccb0:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +096
        jsr     0x5cef8.l                       | +09a
        bcc.w   .L02ccd6                        | +0a0
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +0a4
        andi.w  #0xff,d0                        | +0a8
        lea     Slug_StateByAnglePtrTbl_02a060__L02a08c(pc),a0 | +0ac
        movea.l #0xffffffff,a1                  | +0b0
        jsr     0x772.l                         | +0b6
.L02ccd6:
        jsr     0x5d5b6.l                       | +0bc
        asl.w   #0x1,d0                         | +0c2
        lea     Slug_AirSteerAccelTbl_02c900(pc),a0 | +0c4
        move.w  (a0,d0.w),0x2c(a6)              | +0c8
        tst.w   0x2c(a6)                        | +0ce
        bne.w   .L02ccfc                        | +0d2
        move.w  0x28(a6),d0                     | +0d6
        neg.w   d0                              | +0da
        asr.w   #0x3,d0                         | +0dc
        move.w  d0,0x2c(a6)                     | +0de
.L02ccfc:
        move.w  0x28(a6),d0                     | +0e2
        move.w  #0x2a0,d1                       | +0e6
        jsr     0x267f4.l                       | +0ea
        move.w  d0,0x28(a6)                     | +0f0
        jsr     Slug_PhysicsE_02a7d8(pc)        | +0f4
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0f8
        subi.b  #0x1,0x91(a6)                   | +0fc
        jsr     0x28d70.l                       | +102
        bcc.w   .L02cd34                        | +108
        clr.w   0x28(a6)                        | +10c
        clr.w   0x2c(a6)                        | +110
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +114
        move.l  a1,(a6)                         | +118
.L02cd34:
        tst.b   0x91(a6)                        | +11a
        bne.w   .L02cd4a                        | +11e
        clr.w   0x28(a6)                        | +122
        clr.w   0x2c(a6)                        | +126
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +12a
        move.l  a1,(a6)                         | +12e
.L02cd4a:
        jsr     Slug_TerrainSlope_02a958(pc)    | +130
        cmp.w   0x80(a6),d2                     | +134
        beq.w   .L02cdb4                        | +138
        move.w  d2,0x80(a6)                     | +13c
        move.w  d2,d0                           | +140
        asr.w   #0x1,d0                         | +142
        neg.w   d0                              | +144
        addi.w  #0x20,d0                        | +146
        cmpi.w  #0x40,d0                        | +14a
        bls.w   .L02cd70                        | +14e
        move.w  #0x40,d0                        | +152
.L02cd70:
        movem.w d0,-(a7)                        | +156
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +15a
        movea.l #0x2b0c30,a0                    | +15e
        lsl.w   #0x2,d0                         | +164
        movea.l (a0,d0.w),a0                    | +166
        cmpa.l  #0xffffffff,a0                  | +16a
        beq.w   .L02cd94                        | +170
        jsr     0x28cd4.l                       | +174
.L02cd94:
        movem.w (a7)+,d0                        | +17a
        move.b  #0x0,0x20(a6)                   | +17e
        lsr.w   #0x2,d0                         | +184
        andi.w  #0xff,d0                        | +186
        add.b   d0,0x20(a6)                     | +18a
        bset    #0x2,0x8d(a6)                   | +18e
        lea     Slug_JumpLandSlope_02ce1c__L02ce7e(pc),a1 | +194
        move.l  a1,(a6)                         | +198
.L02cdb4:
        jsr     0x283d8.l                       | +19a
        jsr     Slug_GroundContact_02a8c0(pc)   | +1a0
        bcc.w   .L02cdc8                        | +1a4
        lea     Slug_Fall_02d02e(pc),a1         | +1a8
        move.l  a1,(a6)                         | +1ac
.L02cdc8:
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +1ae
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +1b2
        movea.l #0xffffffff,a1                  | +1b6
        jsr     0x772.l                         | +1bc
        jsr     Slug_CanFire_02aac0(pc)         | +1c2
        bcc.w   .L02cdea                        | +1c6
        lea     Slug_FireFlat_02c24a(pc),a1     | +1ca
        move.l  a1,(a6)                         | +1ce
.L02cdea:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +1d0
        bcc.w   .L02cdf8                        | +1d4
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1d8
        move.l  a1,(a6)                         | +1dc
.L02cdf8:
        jsr     JsrAbsThunk_02a5cc(pc)          | +1de
        bcc.w   .L02ce06                        | +1e2
        lea     Slug_DestroyedSlideInit_02bbf2(pc),a1 | +1e6
        move.l  a1,(a6)                         | +1ea
.L02ce06:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +1ec
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +1f0
        movea.l #0xffffffff,a1                  | +1f4

| ----------------------------------------------------------------------------
|  Slug_JumpLandSlope_02ce1c  @ $02CE1C  (470 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpLandSlope_02ce1c, "ax", @progbits
        .global Slug_JumpLandSlope_02ce1c
Slug_JumpLandSlope_02ce1c:
        move.b  #0x11,0x91(a6)                  | +000
        jsr     Slug_TerrainSlope_02a958(pc)    | +006
        move.w  d2,0x80(a6)                     | +00a
        move.w  d2,d0                           | +00e
        asr.w   #0x1,d0                         | +010
        neg.w   d0                              | +012
        addi.w  #0x20,d0                        | +014
        cmpi.w  #0x40,d0                        | +018
        bls.w   .L02ce40                        | +01c
        move.w  #0x40,d0                        | +020
.L02ce40:
        movem.w d0,-(a7)                        | +024
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +028
        movea.l #0x2b0c30,a0                    | +02c
        lsl.w   #0x2,d0                         | +032
        movea.l (a0,d0.w),a0                    | +034
        cmpa.l  #0xffffffff,a0                  | +038
        beq.w   .L02ce64                        | +03e
        jsr     0x28cd4.l                       | +042
.L02ce64:
        movem.w (a7)+,d0                        | +048
        move.b  #0x0,0x20(a6)                   | +04c
        lsr.w   #0x2,d0                         | +052
        andi.w  #0xff,d0                        | +054
        add.b   d0,0x20(a6)                     | +058
        lea     Slug_JumpLandSlope_02ce1c__L02ce7e(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
        .global Slug_JumpLandSlope_02ce1c__L02ce7e
Slug_JumpLandSlope_02ce1c__L02ce7e:
        bset    #0x2,0x8d(a6)                   | +062
        subi.b  #0x1,0x91(a6)                   | +068
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +06e
        jsr     Slug_CheckFreeThenC_02a59a(pc)  | +072
        bcc.w   .L02ce9c                        | +076
        lea     Slug_SlopeIdleEnter_02b6f4(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L02ce9c:
        jsr     0x5d5b6.l                       | +080
        asl.w   #0x1,d0                         | +086
        lea     Slug_AirSteerAccelTbl_02c900(pc),a0 | +088
        move.w  (a0,d0.w),0x2c(a6)              | +08c
        tst.w   0x2c(a6)                        | +092
        bne.w   .L02cec2                        | +096
        move.w  0x28(a6),d0                     | +09a
        neg.w   d0                              | +09e
        asr.w   #0x3,d0                         | +0a0
        move.w  d0,0x2c(a6)                     | +0a2
.L02cec2:
        move.w  0x28(a6),d0                     | +0a6
        move.w  #0x2a0,d1                       | +0aa
        jsr     0x267f4.l                       | +0ae
        move.w  d0,0x28(a6)                     | +0b4
        jsr     Slug_PhysicsE_02a7d8(pc)        | +0b8
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0bc
        jsr     0x28d70.l                       | +0c0
        jsr     Slug_GroundContact_02a8c0(pc)   | +0c6
        tst.b   0x91(a6)                        | +0ca
        beq.w   .L02cefa                        | +0ce
        cmpi.b  #0x3,d1                         | +0d2
        bge.w   .L02cefa                        | +0d6
        bra.w   .L02cf08                        | +0da
.L02cefa:
        clr.w   0x28(a6)                        | +0de
        clr.w   0x2c(a6)                        | +0e2
        lea     Slug_IdleSlope_02b4d2(pc),a1    | +0e6
        move.l  a1,(a6)                         | +0ea
.L02cf08:
        jsr     Slug_TerrainSlope_02a958(pc)    | +0ec
        cmp.w   0x80(a6),d2                     | +0f0
        beq.w   .L02cf66                        | +0f4
        move.w  d2,0x80(a6)                     | +0f8
        move.w  d2,d0                           | +0fc
        asr.w   #0x1,d0                         | +0fe
        neg.w   d0                              | +100
        addi.w  #0x20,d0                        | +102
        cmpi.w  #0x40,d0                        | +106
        bls.w   .L02cf2e                        | +10a
        move.w  #0x40,d0                        | +10e
.L02cf2e:
        movem.w d0,-(a7)                        | +112
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +116
        movea.l #0x2b0c30,a0                    | +11a
        lsl.w   #0x2,d0                         | +120
        movea.l (a0,d0.w),a0                    | +122
        cmpa.l  #0xffffffff,a0                  | +126
        beq.w   .L02cf52                        | +12c
        jsr     0x28cd4.l                       | +130
.L02cf52:
        movem.w (a7)+,d0                        | +136
        move.b  #0x0,0x20(a6)                   | +13a
        lsr.w   #0x2,d0                         | +140
        andi.w  #0xff,d0                        | +142
        add.b   d0,0x20(a6)                     | +146
.L02cf66:
        jsr     0x283d8.l                       | +14a
        jsr     Slug_GroundContact_02a8c0(pc)   | +150
        bcc.w   .L02cf7a                        | +154
        lea     Slug_Fall_02d02e(pc),a1         | +158
        move.l  a1,(a6)                         | +15c
.L02cf7a:
        cmpi.b  #0x3,0x106f2a.l                 | +15e
        bne.w   .L02cf9a                        | +166
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +16a
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +16e
        movea.l #0xffffffff,a1                  | +172
        jsr     0x772.l                         | +178
.L02cf9a:
        cmpi.b  #0x2,0x106f2a.l                 | +17e
        bne.w   .L02cfba                        | +186
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +18a
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +18e
        movea.l #0xffffffff,a1                  | +192
        jsr     0x772.l                         | +198
.L02cfba:
        jsr     Slug_CanFire_02aac0(pc)         | +19e
        bcc.w   .L02cfc8                        | +1a2
        lea     Slug_FireFlat_02c24a(pc),a1     | +1a6
        move.l  a1,(a6)                         | +1aa
.L02cfc8:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +1ac
        bcc.w   .L02cfd6                        | +1b0
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1b4
        move.l  a1,(a6)                         | +1b8
.L02cfd6:
        jsr     JsrAbsThunk_02a5cc(pc)          | +1ba
        bcc.w   .L02cfe4                        | +1be
        lea     Slug_Destroyed_02fc70(pc),a1    | +1c2
        move.l  a1,(a6)                         | +1c6
.L02cfe4:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +1c8
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +1cc
        movea.l #0xffffffff,a1                  | +1d0

| ----------------------------------------------------------------------------
|  Slug_FallStart_02cffa  @ $02CFFA  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FallStart_02cffa, "ax", @progbits
        .global Slug_FallStart_02cffa
Slug_FallStart_02cffa:
        bclr    #0x5,0x8c(a6)                   | +000
        move.w  #0xffe0,0x2e(a6)                | +006
        clr.w   0x2c(a6)                        | +00c
        addq.w  #0x4,0x82(a6)                   | +010
        move.w  0x24(a6),d0                     | +014
        cmp.w   0x82(a6),d0                     | +018
        ble.w   .L02d01e                        | +01c
        move.w  d0,0x82(a6)                     | +020
.L02d01e:
        lea     .L02d024(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L02d024:
        bset    #0x2,0x8d(a6)                   | +02a
        jmp     Slug_Fall_Loop_02d0c4(pc)       | +030

| ----------------------------------------------------------------------------
|  Slug_Fall_02d02e  @ $02D02E  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Fall_02d02e, "ax", @progbits
        .global Slug_Fall_02d02e
Slug_Fall_02d02e:
        bclr    #0x5,0x8c(a6)                   | +000
        move.w  #0xffe0,0x2e(a6)                | +006
        clr.w   0x2c(a6)                        | +00c
        addq.w  #0x4,0x82(a6)                   | +010
        move.w  0x24(a6),d0                     | +014
        cmp.w   0x82(a6),d0                     | +018
        ble.w   .L02d052                        | +01c
        move.w  d0,0x82(a6)                     | +020
.L02d052:
        jsr     Slug_TerrainSlope_02a958(pc)    | +024
        move.w  d2,0x80(a6)                     | +028
        move.w  d2,d0                           | +02c
        asr.w   #0x1,d0                         | +02e
        neg.w   d0                              | +030
        addi.w  #0x20,d0                        | +032
        cmpi.w  #0x40,d0                        | +036
        bls.w   .L02d070                        | +03a
        move.w  #0x40,d0                        | +03e
.L02d070:
        movem.w d0,-(a7)                        | +042
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +046
        movea.l #0x2b0c30,a0                    | +04a
        lsl.w   #0x2,d0                         | +050
        movea.l (a0,d0.w),a0                    | +052
        cmpa.l  #0xffffffff,a0                  | +056
        beq.w   .L02d094                        | +05c
        jsr     0x28cd4.l                       | +060
.L02d094:
        movem.w (a7)+,d0                        | +066
        move.b  #0x0,0x20(a6)                   | +06a
        lsr.w   #0x2,d0                         | +070
        andi.w  #0xff,d0                        | +072
        add.b   d0,0x20(a6)                     | +076
        bset    #0x2,0x8d(a6)                   | +07a
        lea     Slug_Fall_Loop_02d0c4(pc),a0    | +080
        cmpa.l  (a6),a0                         | +084
        bne.w   .L02d0be                        | +086
        lea     Slug_FallFire_02d26e(pc),a1     | +08a
        move.l  a1,(a6)                         | +08e
.L02d0be:
        lea     Slug_Fall_Loop_02d0c4(pc),a1    | +090
        move.l  a1,(a6)                         | +094

| ----------------------------------------------------------------------------
|  Slug_Fall_Loop_02d0c4  @ $02D0C4  (426 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Fall_Loop_02d0c4, "ax", @progbits
        .global Slug_Fall_Loop_02d0c4
Slug_Fall_Loop_02d0c4:
        jsr     0x5d5b6.l                       | +000
        asl.w   #0x1,d0                         | +006
        lea     Slug_AirSteerAccelTbl_02c900(pc),a0 | +008
        move.w  (a0,d0.w),0x2c(a6)              | +00c
        move.w  0x28(a6),d0                     | +012
        move.w  #0x2a0,d1                       | +016
        jsr     0x267f4.l                       | +01a
        move.w  d0,0x28(a6)                     | +020
        move.w  0x2a(a6),d0                     | +024
        move.w  #0x400,d1                       | +028
        jsr     0x267f4.l                       | +02c
        move.w  d0,0x2a(a6)                     | +032
        bset    #0x1,0x8d(a6)                   | +036
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +03c
        jsr     Slug_CanFire_02aac0(pc)         | +040
        bcc.w   .L02d112                        | +044
        lea     Slug_FireAir_02d394(pc),a1      | +048
        move.l  a1,(a6)                         | +04c
.L02d112:
        jsr     Slug_PhysicsAir_02a878(pc)      | +04e
        bcc.w   .L02d182                        | +052
        lea     Slug_JumpLand_02cc1a(pc),a1     | +056
        move.l  a1,(a6)                         | +05a
        jsr     0x5cef8.l                       | +05c
        bcc.w   .L02d142                        | +062
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +066
        andi.w  #0xff,d0                        | +06a
        lea     Slug_StateByAnglePtrTbl_02a060__L02a08c(pc),a0 | +06e
        movea.l #0xffffffff,a1                  | +072
        jsr     0x772.l                         | +078
.L02d142:
        cmpi.b  #0x3,0x106f2a.l                 | +07e
        bne.w   .L02d162                        | +086
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +08a
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +08e
        movea.l #0xffffffff,a1                  | +092
        jsr     0x772.l                         | +098
.L02d162:
        cmpi.b  #0x2,0x106f2a.l                 | +09e
        bne.w   .L02d182                        | +0a6
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +0aa
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0ae
        movea.l #0xffffffff,a1                  | +0b2
        jsr     0x772.l                         | +0b8
.L02d182:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0be
        jsr     0x28d70.l                       | +0c2
        tst.w   0x2a(a6)                        | +0c8
        bgt.w   .L02d21c                        | +0cc
        jsr     Slug_GroundContact_02a8c0(pc)   | +0d0
        cmpi.b  #0x0,d1                         | +0d4
        ble.w   .L02d1a8                        | +0d8
        clr.w   0x28(a6)                        | +0dc
        clr.w   0x2c(a6)                        | +0e0
.L02d1a8:
        jsr     Slug_TerrainSlope_02a958(pc)    | +0e4
        cmp.w   0x80(a6),d2                     | +0e8
        beq.w   .L02d21c                        | +0ec
        move.w  d2,0x80(a6)                     | +0f0
        move.w  d2,d0                           | +0f4
        asr.w   #0x1,d0                         | +0f6
        neg.w   d0                              | +0f8
        addi.w  #0x20,d0                        | +0fa
        cmpi.w  #0x40,d0                        | +0fe
        bls.w   .L02d1ce                        | +102
        move.w  #0x40,d0                        | +106
.L02d1ce:
        movem.w d0,-(a7)                        | +10a
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +10e
        movea.l #0x2b0c30,a0                    | +112
        lsl.w   #0x2,d0                         | +118
        movea.l (a0,d0.w),a0                    | +11a
        cmpa.l  #0xffffffff,a0                  | +11e
        beq.w   .L02d1f2                        | +124
        jsr     0x28cd4.l                       | +128
.L02d1f2:
        movem.w (a7)+,d0                        | +12e
        move.b  #0x0,0x20(a6)                   | +132
        lsr.w   #0x2,d0                         | +138
        andi.w  #0xff,d0                        | +13a
        add.b   d0,0x20(a6)                     | +13e
        bset    #0x2,0x8d(a6)                   | +142
        lea     Slug_Fall_Loop_02d0c4(pc),a0    | +148
        cmpa.l  (a6),a0                         | +14c
        bne.w   .L02d21c                        | +14e
        lea     Slug_FallFire_02d26e(pc),a1     | +152
        move.l  a1,(a6)                         | +156
.L02d21c:
        btst    #0x1,0x8d(a6)                   | +158
        beq.w   .L02d23e                        | +15e
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +162
        lea     Slug_StateByAnglePtrTbl_02a060__L02a06c(pc),a0 | +166
        movea.l #0xffffffff,a1                  | +16a
        jsr     0x772.l                         | +170
        bra.w   .L02d252                        | +176
.L02d23e:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +17a
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +17e
        movea.l #0xffffffff,a1                  | +182
        jsr     0x772.l                         | +188
.L02d252:
        movea.l #0xffffffff,a0                  | +18e
        lea     0x27964e.l,a0                   | +194
        jsr     0x5dd56.l                       | +19a
        bcc.w   .L02d26c                        | +1a0
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +1a4
.L02d26c:
        rts                                     | +1a8

| ----------------------------------------------------------------------------
|  Slug_FallFire_02d26e  @ $02D26E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FallFire_02d26e, "ax", @progbits
        .global Slug_FallFire_02d26e
Slug_FallFire_02d26e:
        bset    #0x2,0x8d(a6)                   | +000
        jsr     Slug_CanFire_02aac0(pc)         | +006
        bcc.w   .L02d282                        | +00a
        lea     Slug_FireAirB_02d380(pc),a1     | +00e
        move.l  a1,(a6)                         | +012
.L02d282:
        bra.w   Slug_Fall_Loop_02d0c4           | +014

| ----------------------------------------------------------------------------
|  Slug_FireJumpCrouch_02d286  @ $02D286  (250 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireJumpCrouch_02d286, "ax", @progbits
        .global Slug_FireJumpCrouch_02d286
Slug_FireJumpCrouch_02d286:
        bclr    #0x3,0x5b(a6)                   | +000
        move.w  #0x4a4,0x2a(a6)                 | +006
        move.w  #0xff9d,0x2e(a6)                | +00c
        .global Slug_FireJumpCrouch_02d286__L02d298
Slug_FireJumpCrouch_02d286__L02d298:
        bclr    #0x5,0x8c(a6)                   | +012
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +018
        movea.l #0x27931c,a0                    | +01c
        lsl.w   #0x2,d0                         | +022
        movea.l (a0,d0.w),a0                    | +024
        cmpa.l  #0xffffffff,a0                  | +028
        beq.w   .L02d2be                        | +02e
        jsr     0x28cd4.l                       | +032
.L02d2be:
        move.b  #0x2d,0x20(a6)                  | +038
        andi.w  #0xff,d0                        | +03e
        lsr.w   #0x2,d0                         | +042
        add.b   d0,0x20(a6)                     | +044
        bclr    #0x5,0x8c(a6)                   | +048
        move.b  #0x4,d1                         | +04e
        jsr     0x8f714.l                       | +052
        bcs.w   .L02d330                        | +058
        cmpi.b  #0xff,0x84(a6)                  | +05c
        beq.w   .L02d302                        | +062
        lea     0x312b2.l,a1                    | +066
        jsr     0x6fe.l                         | +06c
        jsr     0x5dd02.l                       | +072
        bra.w   .L02d314                        | +078
.L02d302:
        lea     0x31240.l,a1                    | +07c
        jsr     0x6fe.l                         | +082
        jsr     0x5dd02.l                       | +088
.L02d314:
        jsr     0x517fe.l                       | +08e
        lea     0x31af6.l,a1                    | +094
        jsr     0x6fe.l                         | +09a
        jsr     0x5dd02.l                       | +0a0
        bra.w   .L02d348                        | +0a6
.L02d330:
        lea     0x31922.l,a1                    | +0aa
        jsr     0x6fe.l                         | +0b0
        jsr     0x5dd02.l                       | +0b6
        jsr     0x517fe.l                       | +0bc
.L02d348:
        cmpi.b  #0x0,0x90(a6)                   | +0c2
        beq.w   .L02d356                        | +0c8
        jsr     Slug_GaugeTick_02aab0(pc)       | +0cc
.L02d356:
        move.b  #0x1e,0x8e(a6)                  | +0d0
        bset    #0x0,0x8d(a6)                   | +0d6
        btst    #0x2,0x8d(a6)                   | +0dc
        bne.w   .L02d376                        | +0e2
        lea     Slug_FireAir_Loop_02d49e(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
        bra.w   .L02d37c                        | +0ec
.L02d376:
        lea     Slug_FireAirResume_02d38a(pc),a1 | +0f0
        move.l  a1,(a6)                         | +0f4
.L02d37c:
        bra.w   Slug_FireAir_Loop_02d49e        | +0f6

| ----------------------------------------------------------------------------
|  Slug_FireAirB_02d380  @ $02D380  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireAirB_02d380, "ax", @progbits
        .global Slug_FireAirB_02d380
Slug_FireAirB_02d380:
        bset    #0x2,0x8d(a6)                   | +000
        bra.w   Slug_FireAir_02d394             | +006

| ----------------------------------------------------------------------------
|  Slug_FireAirResume_02d38a  @ $02D38A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireAirResume_02d38a, "ax", @progbits
        .global Slug_FireAirResume_02d38a
Slug_FireAirResume_02d38a:
        bset    #0x2,0x8d(a6)                   | +000
        bra.w   Slug_FireAir_Loop_02d49e        | +006

| ----------------------------------------------------------------------------
|  Slug_FireAir_02d394  @ $02D394  (266 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireAir_02d394, "ax", @progbits
        .global Slug_FireAir_02d394
Slug_FireAir_02d394:
        bclr    #0x5,0x8c(a6)                   | +000
        move.b  #0x4,d1                         | +006
        jsr     0x8f714.l                       | +00a
        bcs.w   .L02d3f6                        | +010
        cmpi.b  #0xff,0x84(a6)                  | +014
        beq.w   .L02d3c8                        | +01a
        lea     0x312b2.l,a1                    | +01e
        jsr     0x6fe.l                         | +024
        jsr     0x5dd02.l                       | +02a
        bra.w   .L02d3da                        | +030
.L02d3c8:
        lea     0x31240.l,a1                    | +034
        jsr     0x6fe.l                         | +03a
        jsr     0x5dd02.l                       | +040
.L02d3da:
        jsr     0x517fe.l                       | +046
        lea     0x31af6.l,a1                    | +04c
        jsr     0x6fe.l                         | +052
        jsr     0x5dd02.l                       | +058
        bra.w   .L02d40e                        | +05e
.L02d3f6:
        lea     0x31922.l,a1                    | +062
        jsr     0x6fe.l                         | +068
        jsr     0x5dd02.l                       | +06e
        jsr     0x517fe.l                       | +074
.L02d40e:
        cmpi.b  #0x0,0x90(a6)                   | +07a
        beq.w   .L02d41c                        | +080
        jsr     Slug_GaugeTick_02aab0(pc)       | +084
.L02d41c:
        move.b  #0x1e,0x8e(a6)                  | +088
        jsr     Slug_TerrainSlope_02a958(pc)    | +08e
        move.w  d2,0x80(a6)                     | +092
        move.w  d2,d0                           | +096
        asr.w   #0x1,d0                         | +098
        neg.w   d0                              | +09a
        addi.w  #0x20,d0                        | +09c
        cmpi.w  #0x40,d0                        | +0a0
        bls.w   .L02d440                        | +0a4
        move.w  #0x40,d0                        | +0a8
.L02d440:
        movem.w d0,-(a7)                        | +0ac
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +0b0
        movea.l #0x2b0c30,a0                    | +0b4
        lsl.w   #0x2,d0                         | +0ba
        movea.l (a0,d0.w),a0                    | +0bc
        cmpa.l  #0xffffffff,a0                  | +0c0
        beq.w   .L02d464                        | +0c6
        jsr     0x28cd4.l                       | +0ca
.L02d464:
        movem.w (a7)+,d0                        | +0d0
        move.b  #0x0,0x20(a6)                   | +0d4
        lsr.w   #0x2,d0                         | +0da
        andi.w  #0xff,d0                        | +0dc
        add.b   d0,0x20(a6)                     | +0e0
        bset    #0x2,0x8d(a6)                   | +0e4
        bset    #0x0,0x8d(a6)                   | +0ea
        btst    #0x2,0x8d(a6)                   | +0f0
        bne.w   .L02d498                        | +0f6
        lea     Slug_FireAir_Loop_02d49e(pc),a1 | +0fa
        move.l  a1,(a6)                         | +0fe
        bra.w   Slug_FireAir_Loop_02d49e        | +100
.L02d498:
        lea     Slug_FireAirResume_02d38a(pc),a1 | +104
        move.l  a1,(a6)                         | +108

| ----------------------------------------------------------------------------
|  Slug_FireAir_Loop_02d49e  @ $02D49E  (416 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_FireAir_Loop_02d49e, "ax", @progbits
        .global Slug_FireAir_Loop_02d49e
Slug_FireAir_Loop_02d49e:
        jsr     0x5d5b6.l                       | +000
        asl.w   #0x1,d0                         | +006
        lea     Slug_AirSteerAccelTbl_02c900(pc),a0 | +008
        move.w  (a0,d0.w),0x2c(a6)              | +00c
        move.w  0x28(a6),d0                     | +012
        move.w  #0x2a0,d1                       | +016
        jsr     0x267f4.l                       | +01a
        move.w  d0,0x28(a6)                     | +020
        move.w  0x2a(a6),d0                     | +024
        bgt.w   .L02d4d8                        | +028
        move.w  #0x400,d1                       | +02c
        jsr     0x267f4.l                       | +030
        move.w  d0,0x2a(a6)                     | +036
.L02d4d8:
        bset    #0x1,0x8d(a6)                   | +03a
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +040
        jsr     Slug_PhysicsAir_02a878(pc)      | +044
        bcc.w   .L02d552                        | +048
        lea     Slug_JumpLand_02cc1a(pc),a1     | +04c
        move.l  a1,(a6)                         | +050
        jsr     0x5cef8.l                       | +052
        bcc.w   .L02d512                        | +058
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +05c
        andi.w  #0xff,d0                        | +060
        lea     Slug_StateByAnglePtrTbl_02a060__L02a08c(pc),a0 | +064
        movea.l #0xffffffff,a1                  | +068
        jsr     0x772.l                         | +06e
.L02d512:
        cmpi.b  #0x3,0x106f2a.l                 | +074
        bne.w   .L02d532                        | +07c
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +080
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +084
        movea.l #0xffffffff,a1                  | +088
        jsr     0x772.l                         | +08e
.L02d532:
        cmpi.b  #0x2,0x106f2a.l                 | +094
        bne.w   .L02d552                        | +09c
        jsr     Slug_InputDirByLayoutB_02ab3c(pc) | +0a0
        lea     Slug_InputDirPtrTbl_02c9b0(pc),a0 | +0a4
        movea.l #0xffffffff,a1                  | +0a8
        jsr     0x772.l                         | +0ae
.L02d552:
        jsr     Slug_UpdateAirFlag_02a478(pc)   | +0b4
        jsr     0x28d70.l                       | +0b8
        tst.w   0x2a(a6)                        | +0be
        bgt.w   .L02d5ec                        | +0c2
        jsr     Slug_GroundContact_02a8c0(pc)   | +0c6
        cmpi.b  #0x0,d1                         | +0ca
        ble.w   .L02d5ec                        | +0ce
        clr.w   0x28(a6)                        | +0d2
        clr.w   0x2c(a6)                        | +0d6
        jsr     Slug_TerrainSlope_02a958(pc)    | +0da
        cmp.w   0x80(a6),d2                     | +0de
        beq.w   .L02d5ec                        | +0e2
        move.w  d2,0x80(a6)                     | +0e6
        move.w  d2,d0                           | +0ea
        asr.w   #0x1,d0                         | +0ec
        neg.w   d0                              | +0ee
        addi.w  #0x20,d0                        | +0f0
        cmpi.w  #0x40,d0                        | +0f4
        bls.w   .L02d59e                        | +0f8
        move.w  #0x40,d0                        | +0fc
.L02d59e:
        movem.w d0,-(a7)                        | +100
        jsr     Slug_AngleToSpriteIdx_0295a6(pc) | +104
        movea.l #0x2b0c30,a0                    | +108
        lsl.w   #0x2,d0                         | +10e
        movea.l (a0,d0.w),a0                    | +110
        cmpa.l  #0xffffffff,a0                  | +114
        beq.w   .L02d5c2                        | +11a
        jsr     0x28cd4.l                       | +11e
.L02d5c2:
        movem.w (a7)+,d0                        | +124
        move.b  #0x0,0x20(a6)                   | +128
        lsr.w   #0x2,d0                         | +12e
        andi.w  #0xff,d0                        | +130
        add.b   d0,0x20(a6)                     | +134
        bset    #0x2,0x8d(a6)                   | +138
        lea     Slug_FireAir_Loop_02d49e(pc),a0 | +13e
        cmpa.l  (a6),a0                         | +142
        bne.w   .L02d5ec                        | +144
        lea     Slug_FireAirResume_02d38a(pc),a1 | +148
        move.l  a1,(a6)                         | +14c
.L02d5ec:
        btst    #0x1,0x8d(a6)                   | +14e
        beq.w   .L02d60e                        | +154
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +158
        lea     Slug_StateByAnglePtrTbl_02a060__L02a06c(pc),a0 | +15c
        movea.l #0xffffffff,a1                  | +160
        jsr     0x772.l                         | +166
        bra.w   .L02d622                        | +16c
.L02d60e:
        jsr     Slug_TryStartDestroyedB_02a690(pc) | +170
        lea     Slug_StateByAnglePtrTbl_02a060(pc),a0 | +174
        movea.l #0xffffffff,a1                  | +178
        jsr     0x772.l                         | +17e
.L02d622:
        movea.l #0xffffffff,a0                  | +184
        lea     0x27964e.l,a0                   | +18a
        jsr     0x5dd56.l                       | +190
        bcc.w   .L02d63c                        | +196
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +19a
.L02d63c:
        rts                                     | +19e

| ----------------------------------------------------------------------------
|  Slug_HitReact_02d63e  @ $02D63E  (240 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitReact_02d63e, "ax", @progbits
        .global Slug_HitReact_02d63e
Slug_HitReact_02d63e:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x27954c,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02d664                        | +01c
        jsr     0x28cd4.l                       | +020
.L02d664:
        move.w  #0x0,d0                         | +026
        move.b  #0x96,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        bra.w   .L02d6ba                        | +03a
        bclr    #0x5,0x8c(a6)                   | +03e
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +044
        movea.l #0x279560,a0                    | +048
        lsl.w   #0x2,d0                         | +04e
        movea.l (a0,d0.w),a0                    | +050
        cmpa.l  #0xffffffff,a0                  | +054
        beq.w   .L02d6a2                        | +05a
        jsr     0x28cd4.l                       | +05e
.L02d6a2:
        move.w  #0x0,d0                         | +064
        move.b  #0x96,0x20(a6)                  | +068
        andi.w  #0xff,d0                        | +06e
        lsr.w   #0x2,d0                         | +072
        add.b   d0,0x20(a6)                     | +074
        bra.w   .L02d6ba                        | +078
.L02d6ba:
        jsr     0x267e2.l                       | +07c
        move.l  #0x295b4,0x60(a6)               | +082
        lea     Slug_HitReact_02d63e__L02d6ce(pc),a1 | +08a
        move.l  a1,(a6)                         | +08e
        .global Slug_HitReact_02d63e__L02d6ce
Slug_HitReact_02d63e__L02d6ce:
        move.b  #0x28,0x45(a6)                  | +090
        bclr    #0x3,0x13(a6)                   | +096
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +09c
        jsr     Slug_PhysicsE_02a7d8(pc)        | +0a0
        jsr     0x28d70.l                       | +0a4
        bcc.w   .L02d70a                        | +0aa
        lea     Slug_IdleFlat_02b38c(pc),a1     | +0ae
        move.l  a1,(a6)                         | +0b2
        move.b  #0x28,0x45(a6)                  | +0b4
        move.b  #0x28,d0                        | +0ba
        jsr     0x5e722.l                       | +0be
        lea     Slug_HitboxCbC_029a14(pc),a0    | +0c4
        move.l  a0,0x48(a6)                     | +0c8
.L02d70a:
        jsr     Slug_GroundContact_02a8c0(pc)   | +0cc
        bcc.w   .L02d718                        | +0d0
        lea     Slug_Fall_02d02e(pc),a1         | +0d4
        move.l  a1,(a6)                         | +0d8
.L02d718:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +0da
        bcc.w   .L02d726                        | +0de
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0e2
        move.l  a1,(a6)                         | +0e6
.L02d726:
        jsr     Slug_TryStartDestroyed_02a664(pc) | +0e8
        bcc.w   SetHandlerRts_02d734            | +0ec

| ----------------------------------------------------------------------------
|  Slug_HitLaunchA_02d736  @ $02D736  (204 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitLaunchA_02d736, "ax", @progbits
        .global Slug_HitLaunchA_02d736
Slug_HitLaunchA_02d736:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x279574,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02d75c                        | +01c
        jsr     0x28cd4.l                       | +020
.L02d75c:
        move.w  #0x0,d0                         | +026
        move.b  #0x96,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        move.w  #0x3fc,0x2a(a6)                 | +03a
        move.w  #0xffab,0x2e(a6)                | +040
        bclr    #0x5,0x8c(a6)                   | +046
        clr.w   0x2c(a6)                        | +04c
        lea     .L02d78c(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L02d78c:
        bset    #0x1,0x8d(a6)                   | +056
        move.b  #0x28,0x45(a6)                  | +05c
        move.b  #0x28,d0                        | +062
        jsr     0x5e722.l                       | +066
        bclr    #0x3,0x13(a6)                   | +06c
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +072
        move.w  0x2a(a6),d0                     | +076
        move.w  #0x400,d1                       | +07a
        jsr     0x267f4.l                       | +07e
        move.w  d0,0x2a(a6)                     | +084
        jsr     Slug_PhysicsAir_02a878(pc)      | +088
        bcc.w   .L02d7d2                        | +08c
        lea     Slug_HitLandA_02d8c8(pc),a1     | +090
        move.l  a1,(a6)                         | +094
        jsr     0x267e2.l                       | +096
.L02d7d2:
        jsr     0x28d70.l                       | +09c
        jsr     Slug_TryStartDestroyed_02a664(pc) | +0a2
        bcc.w   .L02d7e6                        | +0a6
        lea     Slug_DeathLaunch_02db52(pc),a1  | +0aa
        move.l  a1,(a6)                         | +0ae
.L02d7e6:
        movea.l #0xffffffff,a0                  | +0b0
        lea     0x27964e.l,a0                   | +0b6
        jsr     0x5dd56.l                       | +0bc
        bcc.w   .L02d800                        | +0c2
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +0c6
.L02d800:
        rts                                     | +0ca

| ----------------------------------------------------------------------------
|  Slug_HitLaunchB_02d802  @ $02D802  (198 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitLaunchB_02d802, "ax", @progbits
        .global Slug_HitLaunchB_02d802
Slug_HitLaunchB_02d802:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x279588,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02d828                        | +01c
        jsr     0x28cd4.l                       | +020
.L02d828:
        move.w  #0x0,d0                         | +026
        move.b  #0x96,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        move.w  #0x3fc,0x2a(a6)                 | +03a
        move.w  #0xffab,0x2e(a6)                | +040
        clr.w   0x2c(a6)                        | +046
        lea     .L02d852(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L02d852:
        bset    #0x1,0x8d(a6)                   | +050
        move.b  #0x28,0x45(a6)                  | +056
        move.b  #0x28,d0                        | +05c
        jsr     0x5e722.l                       | +060
        bclr    #0x3,0x13(a6)                   | +066
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +06c
        move.w  0x2a(a6),d0                     | +070
        move.w  #0x400,d1                       | +074
        jsr     0x267f4.l                       | +078
        move.w  d0,0x2a(a6)                     | +07e
        jsr     Slug_PhysicsAir_02a878(pc)      | +082
        bcc.w   .L02d898                        | +086
        lea     Slug_HitLandB_02d980(pc),a1     | +08a
        move.l  a1,(a6)                         | +08e
        jsr     0x267e2.l                       | +090
.L02d898:
        jsr     0x28d70.l                       | +096
        jsr     Slug_TryStartDestroyed_02a664(pc) | +09c
        bcc.w   .L02d8ac                        | +0a0
        lea     Slug_DeathLaunch_02db52(pc),a1  | +0a4
        move.l  a1,(a6)                         | +0a8
.L02d8ac:
        movea.l #0xffffffff,a0                  | +0aa
        lea     0x27964e.l,a0                   | +0b0
        jsr     0x5dd56.l                       | +0b6
        bcc.w   .L02d8c6                        | +0bc
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +0c0
.L02d8c6:
        rts                                     | +0c4

| ----------------------------------------------------------------------------
|  Slug_HitLandA_02d8c8  @ $02D8C8  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitLandA_02d8c8, "ax", @progbits
        .global Slug_HitLandA_02d8c8
Slug_HitLandA_02d8c8:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x27959c,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02d8ee                        | +01c
        jsr     0x28cd4.l                       | +020
.L02d8ee:
        move.w  #0x0,d0                         | +026
        move.b  #0x96,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        lea     .L02d908(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L02d908:
        move.b  #0x28,0x45(a6)                  | +040
        move.b  #0x28,d0                        | +046
        jsr     0x5e722.l                       | +04a
        bclr    #0x3,0x13(a6)                   | +050
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +056
        move.w  0x2a(a6),d0                     | +05a
        move.w  #0x400,d1                       | +05e
        jsr     0x267f4.l                       | +062
        move.w  d0,0x2a(a6)                     | +068
        jsr     Slug_PhysicsE_02a7d8(pc)        | +06c
        jsr     0x28d70.l                       | +070
        bcc.w   .L02d948                        | +076
        lea     Slug_HitReact_02d63e__L02d6ce(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L02d948:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +080
        bcc.w   .L02d956                        | +084
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +088
        move.l  a1,(a6)                         | +08c
.L02d956:
        jsr     Slug_TryStartDestroyed_02a664(pc) | +08e
        bcc.w   .L02d964                        | +092
        lea     Slug_DeathExplode_02da38(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L02d964:
        movea.l #0xffffffff,a0                  | +09c
        lea     0x27964e.l,a0                   | +0a2
        jsr     0x5dd56.l                       | +0a8
        bcc.w   .L02d97e                        | +0ae
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +0b2
.L02d97e:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  Slug_HitLandB_02d980  @ $02D980  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_HitLandB_02d980, "ax", @progbits
        .global Slug_HitLandB_02d980
Slug_HitLandB_02d980:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +006
        movea.l #0x2795b0,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02d9a6                        | +01c
        jsr     0x28cd4.l                       | +020
.L02d9a6:
        move.w  #0x0,d0                         | +026
        move.b  #0x96,0x20(a6)                  | +02a
        andi.w  #0xff,d0                        | +030
        lsr.w   #0x2,d0                         | +034
        add.b   d0,0x20(a6)                     | +036
        lea     .L02d9c0(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L02d9c0:
        move.b  #0x28,0x45(a6)                  | +040
        move.b  #0x28,d0                        | +046
        jsr     0x5e722.l                       | +04a
        bclr    #0x3,0x13(a6)                   | +050
        jsr     Slug_UpdateAnimKeepIdx_02aa0e(pc) | +056
        move.w  0x2a(a6),d0                     | +05a
        move.w  #0x400,d1                       | +05e
        jsr     0x267f4.l                       | +062
        move.w  d0,0x2a(a6)                     | +068
        jsr     Slug_PhysicsE_02a7d8(pc)        | +06c
        jsr     0x28d70.l                       | +070
        bcc.w   .L02da00                        | +076
        lea     Slug_HitReact_02d63e__L02d6ce(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L02da00:
        jsr     Slug_ConsumeField89_02ac80(pc)  | +080
        bcc.w   .L02da0e                        | +084
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +088
        move.l  a1,(a6)                         | +08c
.L02da0e:
        jsr     Slug_TryStartDestroyed_02a664(pc) | +08e
        bcc.w   .L02da1c                        | +092
        lea     Slug_DeathExplode_02da38(pc),a1 | +096
        move.l  a1,(a6)                         | +09a
.L02da1c:
        movea.l #0xffffffff,a0                  | +09c
        lea     0x27964e.l,a0                   | +0a2
        jsr     0x5dd56.l                       | +0a8
        bcc.w   .L02da36                        | +0ae
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +0b2
.L02da36:
        rts                                     | +0b6

| ----------------------------------------------------------------------------
|  Slug_DeathExplode_02da38  @ $02DA38  (276 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathExplode_02da38, "ax", @progbits
        .global Slug_DeathExplode_02da38
Slug_DeathExplode_02da38:
        jsr     Slug_KillInit_02feda(pc)        | +000
        bclr    #0x5,0x8c(a6)                   | +004
        bset    #0x0,0x13(a6)                   | +00a
        movea.l 0x48(a6),a2                     | +010
        move.w  #0x0,d0                         | +014
        cmpa.l  #0xffffffff,a2                  | +018
        beq.w   .L02da7a                        | +01e
        adda.l  #0xa,a2                         | +022
        move.w  0x54(a6),d0                     | +028
        move.w  (a2),d2                         | +02c
        move.w  0x2(a2),d3                      | +02e
        add.w   0x22(a6),d2                     | +032
        add.w   0x22(a6),d3                     | +036
        add.w   d2,d3                           | +03a
        asr.w   #0x1,d3                         | +03c
        sub.w   d3,d0                           | +03e
        subx.w  d0,d0                           | +040
.L02da7a:
        tst.w   d0                              | +042
        bne.w   .L02daa4                        | +044
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +048
        movea.l #0x27954c,a0                    | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a0                    | +054
        cmpa.l  #0xffffffff,a0                  | +058
        beq.w   .L02daa0                        | +05e
        jsr     0x28cd4.l                       | +062
.L02daa0:
        bra.w   .L02dac4                        | +068
.L02daa4:
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +06c
        movea.l #0x279560,a0                    | +070
        lsl.w   #0x2,d0                         | +076
        movea.l (a0,d0.w),a0                    | +078
        cmpa.l  #0xffffffff,a0                  | +07c
        beq.w   .L02dac4                        | +082
        jsr     0x28cd4.l                       | +086
.L02dac4:
        move.w  #0x0,d0                         | +08c
        move.b  #0x96,0x20(a6)                  | +090
        andi.w  #0xff,d0                        | +096
        lsr.w   #0x2,d0                         | +09a
        add.b   d0,0x20(a6)                     | +09c
        jsr     0x267e2.l                       | +0a0
        lea     .L02dae4(pc),a1                 | +0a6
        move.l  a1,(a6)                         | +0aa
.L02dae4:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +0ac
        bset    #0x4,0x8d(a6)                   | +0b0
        jsr     Slug_UpdateDamageSprite_02fae4__L02fafc(pc) | +0b6
        bcc.w   .L02db28                        | +0ba
        lea     Slug_DeathFade_02dc5c(pc),a1    | +0be
        move.l  a1,(a6)                         | +0c2
        lea     Slug_HitboxDestroyed_02964c__L02973c(pc),a0 | +0c4
        move.l  a0,0x4c(a6)                     | +0c8
        jsr     0x283ca.l                       | +0cc
        jsr     0x283ca.l                       | +0d2
        jsr     0x283d8.l                       | +0d8
        lea     0xffff.w,a0                     | +0de
        move.l  a0,0x4c(a6)                     | +0e2
        jsr     0x283ca.l                       | +0e6
        jsr     Slug_ExplodeFx_02ff22(pc)       | +0ec
.L02db28:
        jsr     Slug_PhysicsA_02a752(pc)        | +0f0
        jsr     0x28d70.l                       | +0f4
        movea.l #0xffffffff,a0                  | +0fa
        lea     0x27964e.l,a0                   | +100
        jsr     0x5dd56.l                       | +106
        bcc.w   JsrPcThunk_02db4c               | +10c
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +110

| ----------------------------------------------------------------------------
|  Slug_DeathLaunch_02db52  @ $02DB52  (260 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathLaunch_02db52, "ax", @progbits
        .global Slug_DeathLaunch_02db52
Slug_DeathLaunch_02db52:
        bclr    #0x5,0x8c(a6)                   | +000
        bset    #0x0,0x13(a6)                   | +006
        movea.l 0x48(a6),a2                     | +00c
        move.w  #0x0,d0                         | +010
        cmpa.l  #0xffffffff,a2                  | +014
        beq.w   .L02db90                        | +01a
        adda.l  #0xa,a2                         | +01e
        move.w  0x54(a6),d0                     | +024
        move.w  (a2),d2                         | +028
        move.w  0x2(a2),d3                      | +02a
        add.w   0x22(a6),d2                     | +02e
        add.w   0x22(a6),d3                     | +032
        add.w   d2,d3                           | +036
        asr.w   #0x1,d3                         | +038
        sub.w   d3,d0                           | +03a
        subx.w  d0,d0                           | +03c
.L02db90:
        tst.w   d0                              | +03e
        bne.w   .L02dbba                        | +040
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +044
        movea.l #0x27954c,a0                    | +048
        lsl.w   #0x2,d0                         | +04e
        movea.l (a0,d0.w),a0                    | +050
        cmpa.l  #0xffffffff,a0                  | +054
        beq.w   .L02dbb6                        | +05a
        jsr     0x28cd4.l                       | +05e
.L02dbb6:
        bra.w   .L02dbda                        | +064
.L02dbba:
        jsr     Slug_SlopeToAnimIdx_02a9a0(pc)  | +068
        movea.l #0x279560,a0                    | +06c
        lsl.w   #0x2,d0                         | +072
        movea.l (a0,d0.w),a0                    | +074
        cmpa.l  #0xffffffff,a0                  | +078
        beq.w   .L02dbda                        | +07e
        jsr     0x28cd4.l                       | +082
.L02dbda:
        move.w  #0x0,d0                         | +088
        move.b  #0x96,0x20(a6)                  | +08c
        andi.w  #0xff,d0                        | +092
        lsr.w   #0x2,d0                         | +096
        add.b   d0,0x20(a6)                     | +098
        jsr     0x267e2.l                       | +09c
        move.w  #0x3fc,0x2a(a6)                 | +0a2
        move.w  #0xffab,0x2e(a6)                | +0a8
        move.b  #0x3c,0x91(a6)                  | +0ae
        lea     .L02dc0c(pc),a1                 | +0b4
        move.l  a1,(a6)                         | +0b8
.L02dc0c:
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +0ba
        bset    #0x4,0x8d(a6)                   | +0be
        move.w  0x2a(a6),d0                     | +0c4
        move.w  #0x400,d1                       | +0c8
        jsr     0x267f4.l                       | +0cc
        move.w  d0,0x2a(a6)                     | +0d2
        jsr     Slug_PhysicsAir_02a878(pc)      | +0d6
        bcc.w   .L02dc36                        | +0da
        lea     Slug_DeathExplode_02da38(pc),a1 | +0de
        move.l  a1,(a6)                         | +0e2
.L02dc36:
        jsr     0x28d70.l                       | +0e4
        movea.l #0xffffffff,a0                  | +0ea
        lea     0x27964e.l,a0                   | +0f0
        jsr     0x5dd56.l                       | +0f6
        bcc.w   JsrPcThunk_02dc56               | +0fc
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +100

| ----------------------------------------------------------------------------
|  Slug_DeathFade_02dc5c  @ $02DC5C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathFade_02dc5c, "ax", @progbits
        .global Slug_DeathFade_02dc5c
Slug_DeathFade_02dc5c:
        bclr    #0x5,0x8c(a6)                   | +000
        move.b  #0x28,0x91(a6)                  | +006
        lea     0xffff.w,a0                     | +00c
        move.l  a0,0x48(a6)                     | +010
        move.l  #0xffffffff,0x60(a6)            | +014
        lea     .L02dc7e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L02dc7e:
        jsr     Slug_PhysicsA_02a752(pc)        | +022
        subi.b  #0x1,0x91(a6)                   | +026
        bne.w   JsrPcThunk_02dca4               | +02c
        .global Slug_DeathFade_02dc5c__L02dc8c
Slug_DeathFade_02dc5c__L02dc8c:
        jsr     0x5b6.l                         | +030
        bclr    #0x7,0x5b(a6)                   | +036
        jsr     0x13600.l                       | +03c
        lea     Slug_DeathSetHandler400_02dcaa(pc),a1 | +042
        move.l  a1,(a6)                         | +046

| ----------------------------------------------------------------------------
|  Slug_DeathSetHandler400_02dcaa  @ $02DCAA  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathSetHandler400_02dcaa, "ax", @progbits
        .global Slug_DeathSetHandler400_02dcaa
Slug_DeathSetHandler400_02dcaa:
        lea     0x400.l,a1                      | +000
        move.l  a1,(a6)                         | +006
        movea.l a6,a0                           | +008

| ----------------------------------------------------------------------------
|  Slug_DeathFinishJmp_02dcbc  @ $02DCBC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathFinishJmp_02dcbc, "ax", @progbits
        .global Slug_DeathFinishJmp_02dcbc
Slug_DeathFinishJmp_02dcbc:
        jmp     Slug_DeathFade_02dc5c__L02dc8c(pc) | +000

| ----------------------------------------------------------------------------
|  Slug_DeathStart_02dcc0  @ $02DCC0  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DeathStart_02dcc0, "ax", @progbits
        .global Slug_DeathStart_02dcc0
Slug_DeathStart_02dcc0:
        lea     Slug_HitboxDestroyed_02964c__L02973c(pc),a0 | +000
        move.l  a0,0x4c(a6)                     | +004
        jsr     0x283ca.l                       | +008
        jsr     0x283ca.l                       | +00e
        jsr     0x283d8.l                       | +014
        lea     0xffff.w,a0                     | +01a
        move.l  a0,0x4c(a6)                     | +01e
        jsr     0x283ca.l                       | +022
        jsr     Slug_ExplodeFx_02ff22(pc)       | +028
        lea     Slug_DeathFade_02dc5c(pc),a1    | +02c
        move.l  a1,(a6)                         | +030
        jsr     Slug_UpdateAnimAndChassis_02aa24(pc) | +032
        jsr     Slug_PhysicsA_02a752(pc)        | +036
        jsr     0x28d70.l                       | +03a
        movea.l #0xffffffff,a0                  | +040
        lea     0x27964e.l,a0                   | +046
        jsr     0x5dd56.l                       | +04c
        bcc.w   JsrPcThunk_02dd1a               | +052
        jmp     Slug_DeathFinishJmp_02dcbc(pc)  | +056
