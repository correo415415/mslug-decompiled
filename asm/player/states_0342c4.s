| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave UUU — máquina de estados en suelo del jugador
|  Región: $0342C4..$036632  (9,070 B, 36 entradas, 1 huecos)
| ============================================================================
|
|  A. RESUMEN
|  Máquina de estados "en suelo" del jugador: Stand (de pie), Walk
|  (derecha/izquierda, con y sin disparo/disparo arriba), Turn (giro),
|  Melee (cuchillo), ThrowGrenade (lanzar granada) y RideSlug (subir al
|  tanque). Todos los handlers comparten la misma plantilla: (1) fijar
|  +$7C/+$7E (fase anim / modificador), limpiar +$8C bits 1-3, (2) elegir
|  tabla de sprites $2796xx..$279Fxx según +$72 (modo) y +$78 (dir) y
|  cargarla con $28CD4 (+$74 = -4(tabla) = "clave de anim"), (3) fijar
|  +$60 = $32500 (hitbox) y +$48 (callback de colisión Player_HitboxStand_0326e0 /
|  Player_HitboxMelee_032788 / Player_HitboxDeath_0328d8), (4) instalar en (a6) el bucle de frame
|  que llama a Player_FrameCommon, Player_PlayWeaponMusicIfFlag,
|  $27A92 (física) y Player_CheckDeathOrState21, y (5) despachar al
|  siguiente estado por input (Input_* y Player_ActionSelect: d1 = $FF
|  golpe -> Melee, 3 -> ThrowGrenade, 4 -> Player_CrouchShoot_03873c, 1 -> *ShootUp,
|  otro -> *Shoot).
|
|  B. ENTRADAS (36)
|   Player_ShootStand_0342c4 / Player_ShootStandUp_03437e: disparo de pie
|     (+$70 = $21, tablas $279716/$2796E4 o $279720/$2796EE); la copia
|     muerta $034438/$0344F2 (sin xrefs) repite el cuerpo con $27972A /
|     $279734 (versiones alternativas no enlazadas).
|   Player_ReenterByInput_0345b8: tras cambio de arma re-elige Walk/Turn
|     según facing (+$3A bit0) e input izq/der, si no -> Stand.
|   Player_StandFromWalk_0345fa: variante de Stand sin re-selección de anim.
|   Player_Stand_034704: estado base de pie (anim $10, $2796A4/$279926);
|     etiquetas Setup_0347e0, InputMove_034a00 (izq/der/abajo -> Walk*/
|     Player_CrouchEnter_037c74), Tail_034ada ($27EBA golpe -> Player_Knockback_036d64,
|     fuego -> Player_JumpStart_036914, PublishState, y test suelo $5DD56 con
|     Player_GroundTblB_0324c6/$324BC según escena $106ECE==3 -> Player_DeathPit_037b8e).
|   Player_WalkRight_034b38 / Player_WalkLeft_034d32: andar (anim $11,
|     $279882/$279878, vel X ±$300, +$3A facing). Sufijos _Shoot/_ShootUp
|     (+$7C/+$7E = 0/-1) usan los "pose" Player_WalkShootPose_034ede
|     (anim $22, $2798F4/$2798E0) y Player_WalkShootUpPose_035052
|     ($2798FE/$2798EA).
|   Player_WalkLoopRight_0351d8 / Player_WalkLoopLeft_0354f2 (+_Run_035590):
|     segunda fase de andar (anim $12, $279926/$27991C); al pulsar la
|     dirección contraria con +$8C bit2 hace eori +$3A (giro en sitio).
|     Pose variantes _Shoot/_ShootUp -> Player_WalkLoopShootPose_0357fa
|     (anim $23) / Player_WalkLoopShootUpPose_03595a; las _Alt_0358aa /
|     _Alt_035a0a ($279908/$279912) no tienen xrefs (copias muertas).
|   Player_TurnRight_035aba / Player_TurnLeft_035bf8 (+_Run_035c98): giro
|     (anim $31, $279A5A/$279A3C), etiqueta Turn_Actions_035b90.
|   Player_ActionDispatch_035cd8: sólo el switch de Player_ActionSelect.
|   Player_Melee_035d34 (+Run_035de0, Frame_035e54): cuchillo (anim $33,
|     $279D56; +$4C = Player_AttackTblB_032638 tabla de golpe, $283CA; vel según input
|     ±$300 con tablas $279D76/$279E30). Player_MeleeAlt_035ea8 /
|     _Walk_035f70: variante $279EFE.
|   Player_ThrowGrenade_Stand_0360bc / _Walk_036212 / _WalkLoop_036318:
|     anim $33, $279D2E/$279D38, +$3B = 0.
|   Player_RideSlug_0364a2: subir al Slug (anim 2, $279F08, +$48 =
|     Player_HitboxDeath_0328d8); copia pos del slot $100580 (+$22/+$24+1), propaga
|     +$68, y según input ($5CDC0 / $5CDB4 / $5D00E) salta a Player_RideSlug_Pose2_0366fe
|     o Player_SlugJumpOff_036796 (+$24 += $20, invuln +$45/+$59 = $3C).
|
|  C. CAMPOS DE ENTIDAD USADOS
|   +$00 handler, +$13 bit0, +$21 (muerte), +$22/+$24 pos, +$28/+$2A vel,
|   +$2C/+$2E accel, +$3A facing (bit0=izq), +$3B, +$45/+$59 invuln,
|   +$48 cb colisión, +$4C tabla ataque, +$60 hitbox ptr, +$68 idx jugador,
|   +$69 bit5 (flag anim espejada), +$70 anim id, +$71 arma, +$72 modo,
|   +$74 clave anim, +$78/+$79 dir nibble, +$7C/+$7E fase, +$82 munición,
|   +$85, +$88 bit0 agachado, +$8C bits1-3 (1=disparando, 2=aire,
|   3=bloqueo acción), bit4 cambio de arma.
|
|  D. HELPERS EXTERNOS
|   $2ABCC ClearXN (C=0 => rama "normal"), $28CD4 carga tabla sprite,
|   $267E6, $27A92 física+colisión, $27EBA golpe recibido, $283CA ataque,
|   $5DD56 test suelo, $5E9B6 RNG, $5CDB4/$5CDC0/$5CEF8/$5CF04/$5CF10/
|   $5D00E InputEvtThunk_* (máscaras $20/$40/$02/$08/$04/$30), $2ABF0 /
|   $2ABD2 / $2ACA2 / $2A25C helpers del slot Slug $100580.
|
|  E. HIPÓTESIS / DUDAS
|   - $5CF04 (mask $08) se trata como "izquierda" y $5CF10 (mask $04) como
|     "derecha" por el signo de la velocidad que fijan (-$300 / +$300 con
|     bset/bclr de +$3A). Confirmar con la tabla de botones.
|   - Cuatro cuerpos sin xrefs ($034438, $0344F2, Player_*_Alt_0358aa,
|     _Alt_035a0a): código muerto o alcanzado por tabla no decodificada.
|   - Player_JumpStart_036914 (fuego), Player_CrouchEnter_037c74 (abajo), Player_CrouchShoot_03873c (acción 4)
|     y Player_Knockback_036d64/037b8e se nombrarán en las waves siguientes.
|
|  F. ORIGEN
|   ASM a mano (no GCC): secuencias duplicadas con etiquetas de salto
|   cruzadas entre funciones (Stand_Tail, Walk_Tail), bra.w a bra.w,
|   `movea.l #-1,a0` seguido de `lea` que lo sobreescribe.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Player_ShootStand_0342c4  @ $0342C4  (186 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ShootStand_0342c4, "ax", @progbits
        .global Player_ShootStand_0342c4
Player_ShootStand_0342c4:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x1,0x72(a6)                   | +006
        bne.w   .L03432c                        | +00c
        move.w  #0x0,0x72(a6)                   | +010
        jsr     0x2abcc.l                       | +016
        bcs.w   .L034318                        | +01c
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x0,0x7e(a6)                   | +026
        move.b  #0x21,0x70(a6)                  | +02c
        lea     0x279716.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279716.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L034328                        | +050
.L034318:
        move.w  #0x1,0x7c(a6)                   | +054
        move.w  #0x3,0x7e(a6)                   | +05a
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +060
.L034328:
        bra.w   .L03437a                        | +064
.L03432c:
        jsr     0x2abcc.l                       | +068
        bcs.w   .L03436a                        | +06e
        move.w  #0x0,0x7c(a6)                   | +072
        move.w  #0x0,0x7e(a6)                   | +078
        move.b  #0x21,0x70(a6)                  | +07e
        lea     0x2796e4.l,a0                   | +084
        move.l  -0x4(a0),0x74(a6)               | +08a
        move.b  #0xff,0x21(a6)                  | +090
        lea     0x2796e4.l,a0                   | +096
        jsr     0x28cd4.l                       | +09c
        bra.w   .L03437a                        | +0a2
.L03436a:
        move.w  #0x1,0x7c(a6)                   | +0a6
        move.w  #0x3,0x7e(a6)                   | +0ac
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +0b2
.L03437a:
        bra.w   Player_Stand_Setup_0347e0    | +0b6

| ----------------------------------------------------------------------------
|  Player_ShootStandUp_03437e  @ $03437E  (570 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ShootStandUp_03437e, "ax", @progbits
        .global Player_ShootStandUp_03437e
Player_ShootStandUp_03437e:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        bne.w   .L0343e6                        | +00c
        move.w  #0x1,0x72(a6)                   | +010
        jsr     0x2abcc.l                       | +016
        bcs.w   .L0343d2                        | +01c
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x0,0x7e(a6)                   | +026
        move.b  #0x21,0x70(a6)                  | +02c
        lea     0x279720.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279720.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L0343e2                        | +050
.L0343d2:
        move.w  #0x0,0x7c(a6)                   | +054
        move.w  #0x4,0x7e(a6)                   | +05a
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +060
.L0343e2:
        bra.w   .L034434                        | +064
.L0343e6:
        jsr     0x2abcc.l                       | +068
        bcs.w   .L034424                        | +06e
        move.w  #0x0,0x7c(a6)                   | +072
        move.w  #0x0,0x7e(a6)                   | +078
        move.b  #0x21,0x70(a6)                  | +07e
        lea     0x2796ee.l,a0                   | +084
        move.l  -0x4(a0),0x74(a6)               | +08a
        move.b  #0xff,0x21(a6)                  | +090
        lea     0x2796ee.l,a0                   | +096
        jsr     0x28cd4.l                       | +09c
        bra.w   .L034434                        | +0a2
.L034424:
        move.w  #0x0,0x7c(a6)                   | +0a6
        move.w  #0x4,0x7e(a6)                   | +0ac
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +0b2
.L034434:
        bra.w   Player_Stand_Setup_0347e0    | +0b6
        bset    #0x1,0x8c(a6)                   | +0ba
        cmpi.w  #0x1,0x72(a6)                   | +0c0
        bne.w   .L0344a0                        | +0c6
        move.w  #0x0,0x72(a6)                   | +0ca
        jsr     0x2abcc.l                       | +0d0
        bcs.w   .L03448c                        | +0d6
        move.w  #0x0,0x7c(a6)                   | +0da
        move.w  #0x0,0x7e(a6)                   | +0e0
        move.b  #0x21,0x70(a6)                  | +0e6
        lea     0x279716.l,a0                   | +0ec
        move.l  -0x4(a0),0x74(a6)               | +0f2
        move.b  #0xff,0x21(a6)                  | +0f8
        lea     0x279716.l,a0                   | +0fe
        jsr     0x28cd4.l                       | +104
        bra.w   .L03449c                        | +10a
.L03448c:
        move.w  #0x1,0x7c(a6)                   | +10e
        move.w  #0x3,0x7e(a6)                   | +114
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +11a
.L03449c:
        bra.w   .L0344ee                        | +11e
.L0344a0:
        jsr     0x2abcc.l                       | +122
        bcs.w   .L0344de                        | +128
        move.w  #0x0,0x7c(a6)                   | +12c
        move.w  #0x0,0x7e(a6)                   | +132
        move.b  #0x21,0x70(a6)                  | +138
        lea     0x27972a.l,a0                   | +13e
        move.l  -0x4(a0),0x74(a6)               | +144
        move.b  #0xff,0x21(a6)                  | +14a
        lea     0x27972a.l,a0                   | +150
        jsr     0x28cd4.l                       | +156
        bra.w   .L0344ee                        | +15c
.L0344de:
        move.w  #0x1,0x7c(a6)                   | +160
        move.w  #0x3,0x7e(a6)                   | +166
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +16c
.L0344ee:
        bra.w   Player_Stand_Setup_0347e0    | +170
        move.w  #0x0,0x7c(a6)                   | +174
        move.w  #0x0,0x7e(a6)                   | +17a
        bset    #0x1,0x8c(a6)                   | +180
        cmpi.w  #0x0,0x72(a6)                   | +186
        bne.w   .L034566                        | +18c
        move.w  #0x1,0x72(a6)                   | +190
        jsr     0x2abcc.l                       | +196
        bcs.w   .L034552                        | +19c
        move.w  #0x0,0x7c(a6)                   | +1a0
        move.w  #0x0,0x7e(a6)                   | +1a6
        move.b  #0x21,0x70(a6)                  | +1ac
        lea     0x279720.l,a0                   | +1b2
        move.l  -0x4(a0),0x74(a6)               | +1b8
        move.b  #0xff,0x21(a6)                  | +1be
        lea     0x279720.l,a0                   | +1c4
        jsr     0x28cd4.l                       | +1ca
        bra.w   .L034562                        | +1d0
.L034552:
        move.w  #0x0,0x7c(a6)                   | +1d4
        move.w  #0x4,0x7e(a6)                   | +1da
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +1e0
.L034562:
        bra.w   .L0345b4                        | +1e4
.L034566:
        jsr     0x2abcc.l                       | +1e8
        bcs.w   .L0345a4                        | +1ee
        move.w  #0x0,0x7c(a6)                   | +1f2
        move.w  #0x0,0x7e(a6)                   | +1f8
        move.b  #0x21,0x70(a6)                  | +1fe
        lea     0x279734.l,a0                   | +204
        move.l  -0x4(a0),0x74(a6)               | +20a
        move.b  #0xff,0x21(a6)                  | +210
        lea     0x279734.l,a0                   | +216
        jsr     0x28cd4.l                       | +21c
        bra.w   .L0345b4                        | +222
.L0345a4:
        move.w  #0x0,0x7c(a6)                   | +226
        move.w  #0x4,0x7e(a6)                   | +22c
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +232
.L0345b4:
        bra.w   Player_Stand_Setup_0347e0    | +236

| ----------------------------------------------------------------------------
|  Player_ReenterByInput_0345b8  @ $0345B8  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ReenterByInput_0345b8, "ax", @progbits
        .global Player_ReenterByInput_0345b8
Player_ReenterByInput_0345b8:
        btst    #0x0,0x3a(a6)                   | +000
        bne.w   .L0345de                        | +006
        jsr     JmpAbsThunk_032e3c(pc)          | +00a
        bcc.w   .L0345ce                        | +00e
        jmp     Player_WalkRight_034b38(pc)     | +012
.L0345ce:
        jsr     Input_RightThunk_032e42(pc)     | +016
        bcc.w   .L0345da                        | +01a
        jmp     Player_TurnLeft_035bf8(pc)      | +01e
.L0345da:
        bra.w   .L0345f6                        | +022
.L0345de:
        jsr     Input_RightThunk_032e42(pc)     | +026
        bcc.w   .L0345ea                        | +02a
        jmp     Player_WalkLeft_034d32(pc)      | +02e
.L0345ea:
        jsr     JmpAbsThunk_032e3c(pc)          | +032
        bcc.w   .L0345f6                        | +036
        jmp     Player_TurnRight_035aba(pc)     | +03a
.L0345f6:
        jmp     Player_Stand_034704(pc)         | +03e

| ----------------------------------------------------------------------------
|  Player_StandFromWalk_0345fa  @ $0345FA  (266 B)
| ----------------------------------------------------------------------------
        .section .text.Player_StandFromWalk_0345fa, "ax", @progbits
        .global Player_StandFromWalk_0345fa
Player_StandFromWalk_0345fa:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        bset    #0x7,0x5b(a6)                   | +01e
        move.l  #0x32500,0x60(a6)               | +024
        jsr     0x267e6.l                       | +02c
        clr.w   0x28(a6)                        | +032
        lea     Player_HitboxStand_0326e0(pc),a0             | +036
        move.l  a0,0x48(a6)                     | +03a
        lea     .L03463e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L03463e:
        jsr     Player_FrameCommon_032ff2(pc)   | +044
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +048
        jsr     0x27a92.l                       | +04c
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +052
        bcc.w   .L03465a                        | +056
        lea     Player_Stand_034704(pc),a1      | +05a
        move.l  a1,(a6)                         | +05e
.L03465a:
        cmpi.b  #0x4,0x78(a6)                   | +060
        bne.w   .L0346ac                        | +066
        cmpi.l  #0x39e54,0x74(a6)               | +06a
        bne.w   .L0346a8                        | +072
        move.w  #0x1,0x72(a6)                   | +076
        jsr     0x2abcc.l                       | +07c
        bcs.w   .L034696                        | +082
        move.b  #0x10,0x70(a6)                  | +086
        lea     0x2796c4.l,a0                   | +08c
        move.l  -0x4(a0),0x74(a6)               | +092
        bra.w   .L0346a8                        | +098
.L034696:
        move.b  #0x10,0x70(a6)                  | +09c
        lea     0x27991c.l,a0                   | +0a2
        move.l  -0x4(a0),0x74(a6)               | +0a8
.L0346a8:
        bra.w   .L034700                        | +0ae
.L0346ac:
        cmpi.l  #0x39e54,0x74(a6)               | +0b2
        bne.w   .L0346f0                        | +0ba
        move.w  #0x0,0x72(a6)                   | +0be
        jsr     0x2abcc.l                       | +0c4
        bcs.w   .L0346de                        | +0ca
        move.b  #0x10,0x70(a6)                  | +0ce
        lea     0x2796a4.l,a0                   | +0d4
        move.l  -0x4(a0),0x74(a6)               | +0da
        bra.w   .L0346f0                        | +0e0
.L0346de:
        move.b  #0x10,0x70(a6)                  | +0e4
        lea     0x279926.l,a0                   | +0ea
        move.l  -0x4(a0),0x74(a6)               | +0f0
.L0346f0:
        btst    #0x0,0x88(a6)                   | +0f6
        beq.w   .L034700                        | +0fc
        lea     Player_Crouch_033a5e(pc),a1     | +100
        move.l  a1,(a6)                         | +104
.L034700:
        bra.w   Player_Stand_InputMove_034a00    | +106

| ----------------------------------------------------------------------------
|  Player_Stand_034704  @ $034704  (1076 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Stand_034704, "ax", @progbits
        .global Player_Stand_034704
Player_Stand_034704:
        cmpi.b  #0x13,0x70(a6)                  | +000
        bcs.w   .L034760                        | +006
        cmpi.b  #0x24,0x70(a6)                  | +00a
        bcc.w   .L034760                        | +010
        jsr     0x2abcc.l                       | +014
        bcs.w   .L03473e                        | +01a
        move.w  #0x0,0x7c(a6)                   | +01e
        move.w  #0x0,0x7e(a6)                   | +024
        lea     0x2796a4.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        bra.w   .L034756                        | +036
.L03473e:
        move.w  #0x0,0x7c(a6)                   | +03a
        move.w  #0x0,0x7e(a6)                   | +040
        lea     0x279926.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
.L034756:
        bclr    #0x5,0x69(a6)                   | +052
        bra.w   Player_Stand_Setup_0347e0    | +058
.L034760:
        bclr    #0x2,0x8c(a6)                   | +05c
        bclr    #0x1,0x8c(a6)                   | +062
        bclr    #0x3,0x8c(a6)                   | +068
        jsr     0x2abcc.l                       | +06e
        bcs.w   .L0347b0                        | +074
        move.w  #0x0,0x7c(a6)                   | +078
        move.w  #0x0,0x7e(a6)                   | +07e
        move.b  #0x10,0x70(a6)                  | +084
        lea     0x2796a4.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x2796a4.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
        bra.w   Player_Stand_Setup_0347e0    | +0a8
.L0347b0:
        move.w  #0x0,0x7c(a6)                   | +0ac
        move.w  #0x0,0x7e(a6)                   | +0b2
        move.b  #0x10,0x70(a6)                  | +0b8
        lea     0x279926.l,a0                   | +0be
        move.l  -0x4(a0),0x74(a6)               | +0c4
        move.b  #0xff,0x21(a6)                  | +0ca
        lea     0x279926.l,a0                   | +0d0
        jsr     0x28cd4.l                       | +0d6
        .global Player_Stand_Setup_0347e0
Player_Stand_Setup_0347e0:
        bset    #0x7,0x5b(a6)                   | +0dc
        move.l  #0x32500,0x60(a6)               | +0e2
        jsr     0x267e6.l                       | +0ea
        clr.w   0x28(a6)                        | +0f0
        lea     Player_HitboxStand_0326e0(pc),a0             | +0f4
        move.l  a0,0x48(a6)                     | +0f8
        lea     .L034806(pc),a1                 | +0fc
        move.l  a1,(a6)                         | +100
.L034806:
        jsr     Player_FrameCommon_032ff2(pc)   | +102
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +106
        bcc.w   .L034818                        | +10a
        lea     Player_ReenterByInput_0345b8(pc),a1 | +10e
        move.l  a1,(a6)                         | +112
.L034818:
        jsr     0x27a92.l                       | +114
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +11a
        bcc.w   .L034944                        | +11e
        move.w  #0x0,0x7c(a6)                   | +122
        move.w  #0x0,0x7e(a6)                   | +128
        bclr    #0x2,0x8c(a6)                   | +12e
        bclr    #0x1,0x8c(a6)                   | +134
        bclr    #0x3,0x8c(a6)                   | +13a
        cmpi.b  #0x4,0x78(a6)                   | +140
        bne.w   .L034890                        | +146
        move.w  #0x1,0x72(a6)                   | +14a
        jsr     0x2abcc.l                       | +150
        bcs.w   .L034874                        | +156
        move.b  #0x10,0x70(a6)                  | +15a
        lea     0x2796c4.l,a0                   | +160
        move.l  -0x4(a0),0x74(a6)               | +166
        bra.w   .L034886                        | +16c
.L034874:
        move.b  #0x10,0x70(a6)                  | +170
        lea     0x27991c.l,a0                   | +176
        move.l  -0x4(a0),0x74(a6)               | +17c
.L034886:
        bset    #0x5,0x69(a6)                   | +182
        bra.w   .L0348f0                        | +188
.L034890:
        move.w  #0x0,0x72(a6)                   | +18c
        jsr     0x2abcc.l                       | +192
        bcs.w   .L0348c8                        | +198
        move.b  #0x10,0x70(a6)                  | +19c
        lea     0x2796a4.l,a0                   | +1a2
        move.l  -0x4(a0),0x74(a6)               | +1a8
        move.b  #0xff,0x21(a6)                  | +1ae
        lea     0x2796a4.l,a0                   | +1b4
        jsr     0x28cd4.l                       | +1ba
        bra.w   .L0348ec                        | +1c0
.L0348c8:
        move.b  #0x10,0x70(a6)                  | +1c4
        lea     0x279926.l,a0                   | +1ca
        move.l  -0x4(a0),0x74(a6)               | +1d0
        move.b  #0xff,0x21(a6)                  | +1d6
        lea     0x279926.l,a0                   | +1dc
        jsr     0x28cd4.l                       | +1e2
.L0348ec:
        bra.w   .L0348f0                        | +1e8
.L0348f0:
        btst    #0x0,0x88(a6)                   | +1ec
        beq.w   .L034900                        | +1f2
        lea     Player_Crouch_033a5e(pc),a1     | +1f6
        move.l  a1,(a6)                         | +1fa
.L034900:
        cmpi.b  #0x0,0x85(a6)                   | +1fc
        bne.w   .L03491a                        | +202
        cmpi.b  #0x1,0x71(a6)                   | +206
        bne.w   .L03491a                        | +20c
        lea     Player_Reload_033b9a(pc),a1     | +210
        move.l  a1,(a6)                         | +214
.L03491a:
        jsr     0x5e9b6.l                       | +216
        andi.w  #0x7,d0                         | +21c
        moveq   #4,d0                           | +220
        cmp.w   0x82(a6),d0                     | +222
        bne.w   .L034934                        | +226
        lea     Player_CrouchB_033afc(pc),a1    | +22a
        move.l  a1,(a6)                         | +22e
.L034934:
        cmpi.w  #0x0,0x82(a6)                   | +230
        bne.w   .L034944                        | +236
        lea     Player_Idle_033d64(pc),a1       | +23a
        move.l  a1,(a6)                         | +23e
.L034944:
        cmpi.b  #0x10,0x70(a6)                  | +240
        bne.w   Player_Stand_InputMove_034a00    | +246
        cmpi.b  #0x4,0x78(a6)                   | +24a
        bne.w   .L0349ae                        | +250
        cmpi.w  #0x1,0x72(a6)                   | +254
        beq.w   .L0349aa                        | +25a
        move.w  #0x1,0x72(a6)                   | +25e
        jsr     0x2abcc.l                       | +264
        bcs.w   .L034988                        | +26a
        move.b  #0x10,0x70(a6)                  | +26e
        lea     0x2796c4.l,a0                   | +274
        move.l  -0x4(a0),0x74(a6)               | +27a
        bra.w   .L03499a                        | +280
.L034988:
        move.b  #0x10,0x70(a6)                  | +284
        lea     0x27991c.l,a0                   | +28a
        move.l  -0x4(a0),0x74(a6)               | +290
.L03499a:
        cmpi.b  #0x14,0x79(a6)                  | +296
        bne.w   .L0349aa                        | +29c
        bset    #0x5,0x69(a6)                   | +2a0
.L0349aa:
        bra.w   Player_Stand_InputMove_034a00    | +2a6
.L0349ae:
        cmpi.w  #0x0,0x72(a6)                   | +2aa
        beq.w   Player_Stand_InputMove_034a00    | +2b0
        move.w  #0x0,0x72(a6)                   | +2b4
        jsr     0x2abcc.l                       | +2ba
        bcs.w   .L0349de                        | +2c0
        move.b  #0x10,0x70(a6)                  | +2c4
        lea     0x2796a4.l,a0                   | +2ca
        move.l  -0x4(a0),0x74(a6)               | +2d0
        bra.w   .L0349f0                        | +2d6
.L0349de:
        move.b  #0x10,0x70(a6)                  | +2da
        lea     0x279926.l,a0                   | +2e0
        move.l  -0x4(a0),0x74(a6)               | +2e6
.L0349f0:
        cmpi.b  #0x41,0x79(a6)                  | +2ec
        bne.w   Player_Stand_InputMove_034a00    | +2f2
        bset    #0x5,0x69(a6)                   | +2f6
        .global Player_Stand_InputMove_034a00
Player_Stand_InputMove_034a00:
        btst    #0x2,0x8c(a6)                   | +2fc
        bne.w   .L034a5e                        | +302
        btst    #0x0,0x3a(a6)                   | +306
        bne.w   .L034a34                        | +30c
        jsr     JmpAbsThunk_032e3c(pc)          | +310
        bcc.w   .L034a22                        | +314
        lea     Player_WalkRight_034b38(pc),a1  | +318
        move.l  a1,(a6)                         | +31c
.L034a22:
        jsr     Input_RightThunk_032e42(pc)     | +31e
        bcc.w   .L034a30                        | +322
        lea     Player_TurnLeft_035bf8(pc),a1   | +326
        move.l  a1,(a6)                         | +32a
.L034a30:
        bra.w   .L034a50                        | +32c
.L034a34:
        jsr     JmpAbsThunk_032e3c(pc)          | +330
        bcc.w   .L034a42                        | +334
        lea     Player_TurnRight_035aba(pc),a1  | +338
        move.l  a1,(a6)                         | +33c
.L034a42:
        jsr     Input_RightThunk_032e42(pc)     | +33e
        bcc.w   .L034a50                        | +342
        lea     Player_WalkLeft_034d32(pc),a1   | +346
        move.l  a1,(a6)                         | +34a
.L034a50:
        jsr     Input_DownPressed_032e90(pc)    | +34c
        bcc.w   .L034a5e                        | +350
        lea     Player_CrouchEnter_037c74(pc),a1             | +354
        move.l  a1,(a6)                         | +358
.L034a5e:
        cmpi.b  #0x4,0x71(a6)                   | +35a
        bne.w   .L034a6c                        | +360
        bra.w   .L034a76                        | +364
.L034a6c:
        btst    #0x2,0x8c(a6)                   | +368
        bne.w   Player_Stand_Tail_034ada    | +36e
.L034a76:
        jsr     Player_ActionSelect_0330d0(pc)  | +372
        bcc.w   Player_Stand_Tail_034ada    | +376
        btst    #0x3,0x8c(a6)                   | +37a
        bne.w   .L034abe                        | +380
        cmpi.b  #0xff,d1                        | +384
        bne.w   .L034a9a                        | +388
        lea     Player_Melee_035d34(pc),a1      | +38c
        move.l  a1,(a6)                         | +390
        bra.w   Player_Stand_Tail_034ada    | +392
.L034a9a:
        cmpi.b  #0x3,d1                         | +396
        bne.w   .L034aac                        | +39a
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +39e
        move.l  a1,(a6)                         | +3a2
        bra.w   Player_Stand_Tail_034ada    | +3a4
.L034aac:
        cmpi.b  #0x4,d1                         | +3a8
        bne.w   .L034abe                        | +3ac
        lea     Player_CrouchShoot_03873c(pc),a1             | +3b0
        move.l  a1,(a6)                         | +3b4
        bra.w   Player_Stand_Tail_034ada    | +3b6
.L034abe:
        cmpi.b  #0x1,d1                         | +3ba
        bne.w   .L034ad0                        | +3be
        lea     Player_ShootStandUp_03437e(pc),a1 | +3c2
        move.l  a1,(a6)                         | +3c6
        bra.w   Player_Stand_Tail_034ada    | +3c8
.L034ad0:
        lea     Player_ShootStand_0342c4(pc),a1 | +3cc
        move.l  a1,(a6)                         | +3d0
        bra.w   Player_Stand_Tail_034ada    | +3d2
        .global Player_Stand_Tail_034ada
Player_Stand_Tail_034ada:
        jsr     0x27eba.l                       | +3d6
        bcc.w   .L034aea                        | +3dc
        lea     Player_Knockback_036d64(pc),a1       | +3e0
        move.l  a1,(a6)                         | +3e4
.L034aea:
        jsr     Input_FireByMode_033034(pc)     | +3e6
        bcc.w   .L034af8                        | +3ea
        lea     Player_JumpStart_036914(pc),a1             | +3ee
        move.l  a1,(a6)                         | +3f2
.L034af8:
        jsr     PlayerRoute_PublishState_033522(pc) | +3f4
        cmpi.b  #0x3,0x106ece.l                 | +3f8
        beq.w   .L034b1c                        | +400
        movea.l #0xffffffff,a0                  | +404
        lea     Player_GroundTblB_0324c6(pc),a0             | +40a
        jsr     0x5dd56.l                       | +40e
        bra.w   .L034b2c                        | +414
.L034b1c:
        movea.l #0xffffffff,a0                  | +418
        lea     Player_GroundTblA_0324bc(pc),a0             | +41e
        jsr     0x5dd56.l                       | +422
.L034b2c:
        bcc.w   .L034b36                        | +428
        lea     Player_DeathPit_037b8e(pc),a1       | +42c
        move.l  a1,(a6)                         | +430
.L034b36:
        rts                                     | +432

| ----------------------------------------------------------------------------
|  Player_WalkRight_034b38  @ $034B38  (506 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkRight_034b38, "ax", @progbits
        .global Player_WalkRight_034b38
Player_WalkRight_034b38:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        cmpi.b  #0x13,0x70(a6)                  | +00c
        bcs.w   .L034b6e                        | +012
        cmpi.b  #0x24,0x70(a6)                  | +016
        bcc.w   .L034b6e                        | +01c
        lea     0x279882.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        bclr    #0x5,0x69(a6)                   | +02c
        bra.w   Player_WalkRight_Setup_034ba4 | +032
.L034b6e:
        bclr    #0x2,0x8c(a6)                   | +036
        bclr    #0x1,0x8c(a6)                   | +03c
        bclr    #0x3,0x8c(a6)                   | +042
        move.b  #0x11,0x70(a6)                  | +048
        lea     0x279882.l,a0                   | +04e
        move.l  -0x4(a0),0x74(a6)               | +054
        move.b  #0xff,0x21(a6)                  | +05a
        lea     0x279882.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        .global Player_WalkRight_Setup_034ba4
Player_WalkRight_Setup_034ba4:
        move.w  #0x300,0x28(a6)                 | +06c
        clr.w   0x2c(a6)                        | +072
        move.l  #0x32500,0x60(a6)               | +076
        lea     Player_HitboxStand_0326e0(pc),a0             | +07e
        move.l  a0,0x48(a6)                     | +082
        bclr    #0x0,0x3a(a6)                   | +086
        lea     .L034bca(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L034bca:
        jsr     Player_FrameCommon_032ff2(pc)   | +092
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +096
        bcc.w   .L034bdc                        | +09a
        lea     Player_ReenterByInput_0345b8(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L034bdc:
        move.w  #0x300,0x28(a6)                 | +0a4
        jsr     0x27a92.l                       | +0aa
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0b0
        bcc.w   .L034bf6                        | +0b4
        lea     Player_WalkLoopRight_0351d8(pc),a1 | +0b8
        move.l  a1,(a6)                         | +0bc
.L034bf6:
        cmpi.b  #0x4,0x78(a6)                   | +0be
        bne.w   .L034c28                        | +0c4
        cmpi.l  #0x3a058,0x74(a6)               | +0c8
        bne.w   .L034c24                        | +0d0
        move.w  #0x1,0x72(a6)                   | +0d4
        move.b  #0x11,0x70(a6)                  | +0da
        lea     0x279878.l,a0                   | +0e0
        move.l  -0x4(a0),0x74(a6)               | +0e6
.L034c24:
        bra.w   .L034c4c                        | +0ec
.L034c28:
        cmpi.l  #0x3a058,0x74(a6)               | +0f0
        bne.w   .L034c4c                        | +0f8
        move.w  #0x0,0x72(a6)                   | +0fc
        move.b  #0x11,0x70(a6)                  | +102
        lea     0x279882.l,a0                   | +108
        move.l  -0x4(a0),0x74(a6)               | +10e
.L034c4c:
        btst    #0x2,0x8c(a6)                   | +114
        bne.w   Player_WalkRight_Tail_034ccc | +11a
        jsr     JmpAbsThunk_032e3c(pc)          | +11e
        bcs.w   .L034c64                        | +122
        lea     Player_StandFromWalk_0345fa(pc),a1 | +126
        move.l  a1,(a6)                         | +12a
.L034c64:
        jsr     Input_RightThunk_032e42(pc)     | +12c
        bcc.w   .L034c72                        | +130
        lea     Player_TurnLeft_035bf8(pc),a1   | +134
        move.l  a1,(a6)                         | +138
.L034c72:
        jsr     Player_ActionSelect_0330d0(pc)  | +13a
        bcc.w   Player_WalkRight_Tail_034ccc | +13e
        cmpi.b  #0xff,d1                        | +142
        bne.w   .L034c8c                        | +146
        lea     Player_Melee_035d34(pc),a1      | +14a
        move.l  a1,(a6)                         | +14e
        bra.w   Player_WalkRight_Tail_034ccc | +150
.L034c8c:
        cmpi.b  #0x3,d1                         | +154
        bne.w   .L034c9e                        | +158
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +15c
        move.l  a1,(a6)                         | +160
        bra.w   Player_WalkRight_Tail_034ccc | +162
.L034c9e:
        cmpi.b  #0x4,d1                         | +166
        bne.w   .L034cb0                        | +16a
        lea     Player_CrouchShoot_03873c(pc),a1             | +16e
        move.l  a1,(a6)                         | +172
        bra.w   Player_WalkRight_Tail_034ccc | +174
.L034cb0:
        cmpi.b  #0x1,d1                         | +178
        bne.w   .L034cc2                        | +17c
        lea     Player_WalkRight_ShootUp_03503e(pc),a1 | +180
        move.l  a1,(a6)                         | +184
        bra.w   Player_WalkRight_Tail_034ccc | +186
.L034cc2:
        lea     Player_WalkRight_Shoot_034eca(pc),a1 | +18a
        move.l  a1,(a6)                         | +18e
        bra.w   Player_WalkRight_Tail_034ccc | +190
        .global Player_WalkRight_Tail_034ccc
Player_WalkRight_Tail_034ccc:
        jsr     Input_DownPressed_032e90(pc)    | +194
        bcc.w   .L034cda                        | +198
        lea     Player_CrouchEnter_037c74(pc),a1             | +19c
        move.l  a1,(a6)                         | +1a0
.L034cda:
        cmpi.b  #0x0,0x85(a6)                   | +1a2
        bne.w   .L034cf4                        | +1a8
        cmpi.b  #0x1,0x71(a6)                   | +1ac
        bne.w   .L034cf4                        | +1b2
        lea     Player_Reload_033b9a(pc),a1     | +1b6
        move.l  a1,(a6)                         | +1ba
.L034cf4:
        cmpi.w  #0x0,0x82(a6)                   | +1bc
        bne.w   .L034d0e                        | +1c2
        cmpi.b  #0x0,0x71(a6)                   | +1c6
        beq.w   .L034d0e                        | +1cc
        lea     Player_Idle_033d64(pc),a1       | +1d0
        move.l  a1,(a6)                         | +1d4
.L034d0e:
        jsr     0x27eba.l                       | +1d6
        bcc.w   .L034d1e                        | +1dc
        lea     Player_Knockback_036d64(pc),a1       | +1e0
        move.l  a1,(a6)                         | +1e4
.L034d1e:
        jsr     Input_FireByMode_033034(pc)     | +1e6
        bcc.w   .L034d2c                        | +1ea
        lea     Player_JumpStart_036914(pc),a1             | +1ee
        move.l  a1,(a6)                         | +1f2
.L034d2c:
        jsr     PlayerRoute_PublishState_033522(pc) | +1f4
        rts                                     | +1f8

| ----------------------------------------------------------------------------
|  Player_WalkLeft_034d32  @ $034D32  (408 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLeft_034d32, "ax", @progbits
        .global Player_WalkLeft_034d32
Player_WalkLeft_034d32:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        cmpi.b  #0x13,0x70(a6)                  | +00c
        bcs.w   .L034d68                        | +012
        cmpi.b  #0x24,0x70(a6)                  | +016
        bcc.w   .L034d68                        | +01c
        lea     0x279882.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        bclr    #0x5,0x69(a6)                   | +02c
        bra.w   Player_WalkLeft_Setup_034d9e | +032
.L034d68:
        bclr    #0x2,0x8c(a6)                   | +036
        bclr    #0x1,0x8c(a6)                   | +03c
        bclr    #0x3,0x8c(a6)                   | +042
        move.b  #0x11,0x70(a6)                  | +048
        lea     0x279882.l,a0                   | +04e
        move.l  -0x4(a0),0x74(a6)               | +054
        move.b  #0xff,0x21(a6)                  | +05a
        lea     0x279882.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        .global Player_WalkLeft_Setup_034d9e
Player_WalkLeft_Setup_034d9e:
        move.w  #0xfd00,0x28(a6)                | +06c
        clr.w   0x2c(a6)                        | +072
        move.l  #0x32500,0x60(a6)               | +076
        lea     Player_HitboxStand_0326e0(pc),a0             | +07e
        move.l  a0,0x48(a6)                     | +082
        bset    #0x0,0x3a(a6)                   | +086
        lea     .L034dc4(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L034dc4:
        jsr     Player_FrameCommon_032ff2(pc)   | +092
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +096
        bcc.w   .L034dd6                        | +09a
        lea     Player_ReenterByInput_0345b8(pc),a1 | +09e
        move.l  a1,(a6)                         | +0a2
.L034dd6:
        move.w  #0xfd00,0x28(a6)                | +0a4
        jsr     0x27a92.l                       | +0aa
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0b0
        bcc.w   .L034df0                        | +0b4
        lea     Player_WalkLoopLeft_0354f2(pc),a1 | +0b8
        move.l  a1,(a6)                         | +0bc
.L034df0:
        cmpi.b  #0x4,0x78(a6)                   | +0be
        bne.w   .L034e22                        | +0c4
        cmpi.l  #0x3a058,0x74(a6)               | +0c8
        bne.w   .L034e1e                        | +0d0
        move.w  #0x1,0x72(a6)                   | +0d4
        move.b  #0x11,0x70(a6)                  | +0da
        lea     0x279878.l,a0                   | +0e0
        move.l  -0x4(a0),0x74(a6)               | +0e6
.L034e1e:
        bra.w   .L034e46                        | +0ec
.L034e22:
        cmpi.l  #0x3a058,0x74(a6)               | +0f0
        bne.w   .L034e46                        | +0f8
        move.w  #0x0,0x72(a6)                   | +0fc
        move.b  #0x11,0x70(a6)                  | +102
        lea     0x279882.l,a0                   | +108
        move.l  -0x4(a0),0x74(a6)               | +10e
.L034e46:
        btst    #0x2,0x8c(a6)                   | +114
        bne.w   .L034ec6                        | +11a
        jsr     Input_RightThunk_032e42(pc)     | +11e
        bcs.w   .L034e5e                        | +122
        lea     Player_StandFromWalk_0345fa(pc),a1 | +126
        move.l  a1,(a6)                         | +12a
.L034e5e:
        jsr     JmpAbsThunk_032e3c(pc)          | +12c
        bcc.w   .L034e6c                        | +130
        lea     Player_TurnRight_035aba(pc),a1  | +134
        move.l  a1,(a6)                         | +138
.L034e6c:
        jsr     Player_ActionSelect_0330d0(pc)  | +13a
        bcc.w   .L034ec6                        | +13e
        cmpi.b  #0xff,d1                        | +142
        bne.w   .L034e86                        | +146
        lea     Player_Melee_035d34(pc),a1      | +14a
        move.l  a1,(a6)                         | +14e
        bra.w   .L034ec6                        | +150
.L034e86:
        cmpi.b  #0x3,d1                         | +154
        bne.w   .L034e98                        | +158
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +15c
        move.l  a1,(a6)                         | +160
        bra.w   .L034ec6                        | +162
.L034e98:
        cmpi.b  #0x4,d1                         | +166
        bne.w   .L034eaa                        | +16a
        lea     Player_CrouchShoot_03873c(pc),a1             | +16e
        move.l  a1,(a6)                         | +172
        bra.w   Player_Walk_Tail_03548c | +174
.L034eaa:
        cmpi.b  #0x1,d1                         | +178
        bne.w   .L034ebc                        | +17c
        lea     Player_WalkLeft_ShootUp_0351c4(pc),a1 | +180
        move.l  a1,(a6)                         | +184
        bra.w   .L034ec6                        | +186
.L034ebc:
        lea     Player_WalkLeft_Shoot_0351b0(pc),a1 | +18a
        move.l  a1,(a6)                         | +18e
        bra.w   .L034ec6                        | +190
.L034ec6:
        bra.w   Player_WalkRight_Tail_034ccc | +194

| ----------------------------------------------------------------------------
|  Player_WalkRight_Shoot_034eca  @ $034ECA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkRight_Shoot_034eca, "ax", @progbits
        .global Player_WalkRight_Shoot_034eca
Player_WalkRight_Shoot_034eca:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        jsr     Player_WalkShootPose_034ede(pc) | +00c
        bra.w   Player_WalkRight_Setup_034ba4 | +010

| ----------------------------------------------------------------------------
|  Player_WalkShootPose_034ede  @ $034EDE  (352 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkShootPose_034ede, "ax", @progbits
        .global Player_WalkShootPose_034ede
Player_WalkShootPose_034ede:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x1,0x72(a6)                   | +006
        bne.w   .L034f3c                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L034f14                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x22,0x70(a6)                  | +020
        lea     0x2798f4.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L034f38                        | +032
.L034f14:
        move.b  #0x22,0x70(a6)                  | +036
        lea     0x2798f4.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798f4.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L034f38:
        bra.w   .L034f86                        | +05a
.L034f3c:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L034f62                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x22,0x70(a6)                  | +06e
        lea     0x2798e0.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L034f86                        | +080
.L034f62:
        move.b  #0x22,0x70(a6)                  | +084
        lea     0x2798e0.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x2798e0.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L034f86:
        move.w  #0x0,0x72(a6)                   | +0a8
        rts                                     | +0ae
        bset    #0x1,0x8c(a6)                   | +0b0
        cmpi.w  #0x1,0x72(a6)                   | +0b6
        bne.w   .L034fec                        | +0bc
        cmpi.b  #0x24,0x70(a6)                  | +0c0
        bcc.w   .L034fc4                        | +0c6
        bset    #0x5,0x69(a6)                   | +0ca
        move.b  #0x22,0x70(a6)                  | +0d0
        lea     0x2798f4.l,a0                   | +0d6
        move.l  -0x4(a0),0x74(a6)               | +0dc
        bra.w   .L034fe8                        | +0e2
.L034fc4:
        move.b  #0x22,0x70(a6)                  | +0e6
        lea     0x2798f4.l,a0                   | +0ec
        move.l  -0x4(a0),0x74(a6)               | +0f2
        move.b  #0xff,0x21(a6)                  | +0f8
        lea     0x2798f4.l,a0                   | +0fe
        jsr     0x28cd4.l                       | +104
.L034fe8:
        bra.w   .L035036                        | +10a
.L034fec:
        cmpi.b  #0x24,0x70(a6)                  | +10e
        bcc.w   .L035012                        | +114
        bset    #0x5,0x69(a6)                   | +118
        move.b  #0x22,0x70(a6)                  | +11e
        lea     0x279908.l,a0                   | +124
        move.l  -0x4(a0),0x74(a6)               | +12a
        bra.w   .L035036                        | +130
.L035012:
        move.b  #0x22,0x70(a6)                  | +134
        lea     0x279908.l,a0                   | +13a
        move.l  -0x4(a0),0x74(a6)               | +140
        move.b  #0xff,0x21(a6)                  | +146
        lea     0x279908.l,a0                   | +14c
        jsr     0x28cd4.l                       | +152
.L035036:
        move.w  #0x0,0x72(a6)                   | +158
        rts                                     | +15e

| ----------------------------------------------------------------------------
|  Player_WalkRight_ShootUp_03503e  @ $03503E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkRight_ShootUp_03503e, "ax", @progbits
        .global Player_WalkRight_ShootUp_03503e
Player_WalkRight_ShootUp_03503e:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        jsr     Player_WalkShootUpPose_035052(pc) | +00c
        bra.w   Player_WalkRight_Setup_034ba4 | +010

| ----------------------------------------------------------------------------
|  Player_WalkShootUpPose_035052  @ $035052  (350 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkShootUpPose_035052, "ax", @progbits
        .global Player_WalkShootUpPose_035052
Player_WalkShootUpPose_035052:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        bne.w   .L0350b0                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L035088                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x22,0x70(a6)                  | +020
        lea     0x2798fe.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L0350ac                        | +032
.L035088:
        move.b  #0x22,0x70(a6)                  | +036
        lea     0x2798fe.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798fe.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L0350ac:
        bra.w   .L0350fa                        | +05a
.L0350b0:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L0350d6                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x22,0x70(a6)                  | +06e
        lea     0x2798ea.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L0350fa                        | +080
.L0350d6:
        move.b  #0x22,0x70(a6)                  | +084
        lea     0x2798ea.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x2798ea.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L0350fa:
        move.w  #0x1,0x72(a6)                   | +0a8
        rts                                     | +0ae
        bset    #0x1,0x8c(a6)                   | +0b0
        cmpi.w  #0x0,0x72(a6)                   | +0b6
        bne.w   .L03515e                        | +0bc
        cmpi.b  #0x24,0x70(a6)                  | +0c0
        bcc.w   .L035138                        | +0c6
        bset    #0x5,0x69(a6)                   | +0ca
        move.b  #0x22,0x70(a6)                  | +0d0
        lea     0x2798fe.l,a0                   | +0d6
        move.l  -0x4(a0),0x74(a6)               | +0dc
        bra.w   .L03515c                        | +0e2
.L035138:
        move.b  #0x22,0x70(a6)                  | +0e6
        lea     0x2798fe.l,a0                   | +0ec
        move.l  -0x4(a0),0x74(a6)               | +0f2
        move.b  #0xff,0x21(a6)                  | +0f8
        lea     0x2798fe.l,a0                   | +0fe
        jsr     0x28cd4.l                       | +104
.L03515c:
        bra.b   .L0350fa                        | +10a
.L03515e:
        cmpi.b  #0x24,0x70(a6)                  | +10c
        bcc.w   .L035184                        | +112
        bset    #0x5,0x69(a6)                   | +116
        move.b  #0x22,0x70(a6)                  | +11c
        lea     0x279912.l,a0                   | +122
        move.l  -0x4(a0),0x74(a6)               | +128
        bra.w   .L0351a8                        | +12e
.L035184:
        move.b  #0x22,0x70(a6)                  | +132
        lea     0x279912.l,a0                   | +138
        move.l  -0x4(a0),0x74(a6)               | +13e
        move.b  #0xff,0x21(a6)                  | +144
        lea     0x279912.l,a0                   | +14a
        jsr     0x28cd4.l                       | +150
.L0351a8:
        move.w  #0x1,0x72(a6)                   | +156
        rts                                     | +15c

| ----------------------------------------------------------------------------
|  Player_WalkLeft_Shoot_0351b0  @ $0351B0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLeft_Shoot_0351b0, "ax", @progbits
        .global Player_WalkLeft_Shoot_0351b0
Player_WalkLeft_Shoot_0351b0:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        jsr     Player_WalkShootPose_034ede(pc) | +00c
        bra.w   Player_WalkLeft_Setup_034d9e | +010

| ----------------------------------------------------------------------------
|  Player_WalkLeft_ShootUp_0351c4  @ $0351C4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLeft_ShootUp_0351c4, "ax", @progbits
        .global Player_WalkLeft_ShootUp_0351c4
Player_WalkLeft_ShootUp_0351c4:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        jsr     Player_WalkShootUpPose_035052(pc) | +00c
        bra.w   Player_WalkLeft_Setup_034d9e | +010

| ----------------------------------------------------------------------------
|  Player_WalkLoopRight_0351d8  @ $0351D8  (794 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopRight_0351d8, "ax", @progbits
        .global Player_WalkLoopRight_0351d8
Player_WalkLoopRight_0351d8:
        cmpi.b  #0x13,0x70(a6)                  | +000
        bcs.w   .L03520e                        | +006
        cmpi.b  #0x24,0x70(a6)                  | +00a
        bcc.w   .L03520e                        | +010
        lea     0x279926.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x3,0x7e(a6)                   | +026
        bclr    #0x5,0x69(a6)                   | +02c
        bra.w   Player_WalkLoopRight_Setup_035250 | +032
.L03520e:
        move.w  #0x0,0x7c(a6)                   | +036
        move.w  #0xffff,0x7e(a6)                | +03c
        bclr    #0x2,0x8c(a6)                   | +042
        bclr    #0x1,0x8c(a6)                   | +048
        bclr    #0x3,0x8c(a6)                   | +04e
        move.b  #0x12,0x70(a6)                  | +054
        lea     0x279926.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
        move.b  #0xff,0x21(a6)                  | +066
        lea     0x279926.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        .global Player_WalkLoopRight_Setup_035250
Player_WalkLoopRight_Setup_035250:
        move.w  #0x300,0x28(a6)                 | +078
        clr.w   0x2c(a6)                        | +07e
        move.l  #0x32500,0x60(a6)               | +082
        lea     Player_HitboxStand_0326e0(pc),a0             | +08a
        move.l  a0,0x48(a6)                     | +08e
        bclr    #0x0,0x3a(a6)                   | +092
        lea     .L035276(pc),a1                 | +098
        move.l  a1,(a6)                         | +09c
.L035276:
        jsr     Player_FrameCommon_032ff2(pc)   | +09e
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +0a2
        bcc.w   .L035288                        | +0a6
        lea     Player_ReenterByInput_0345b8(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
.L035288:
        move.w  #0x300,0x28(a6)                 | +0b0
        jsr     0x27a92.l                       | +0b6
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0bc
        bcc.w   .L035308                        | +0c0
        move.w  #0x0,0x7c(a6)                   | +0c4
        move.w  #0xffff,0x7e(a6)                | +0ca
        bclr    #0x2,0x8c(a6)                   | +0d0
        bclr    #0x1,0x8c(a6)                   | +0d6
        bclr    #0x3,0x8c(a6)                   | +0dc
        cmpi.b  #0x13,0x70(a6)                  | +0e2
        bcs.w   .L0352e4                        | +0e8
        cmpi.b  #0x24,0x70(a6)                  | +0ec
        bcc.w   .L0352e4                        | +0f2
        move.b  #0x12,0x70(a6)                  | +0f6
        lea     0x279926.l,a0                   | +0fc
        move.l  -0x4(a0),0x74(a6)               | +102
        bra.w   .L035308                        | +108
.L0352e4:
        move.b  #0x12,0x70(a6)                  | +10c
        lea     0x279926.l,a0                   | +112
        move.l  -0x4(a0),0x74(a6)               | +118
        move.b  #0xff,0x21(a6)                  | +11e
        lea     0x279926.l,a0                   | +124
        jsr     0x28cd4.l                       | +12a
.L035308:
        cmpi.b  #0x12,0x70(a6)                  | +130
        bne.w   .L0353c4                        | +136
        cmpi.b  #0x4,0x78(a6)                   | +13a
        bne.w   .L035372                        | +140
        cmpi.w  #0x1,0x72(a6)                   | +144
        beq.w   .L03536e                        | +14a
        move.w  #0x1,0x72(a6)                   | +14e
        jsr     0x2abcc.l                       | +154
        bcs.w   .L03534c                        | +15a
        move.b  #0x12,0x70(a6)                  | +15e
        lea     0x27991c.l,a0                   | +164
        move.l  -0x4(a0),0x74(a6)               | +16a
        bra.w   .L03535e                        | +170
.L03534c:
        move.b  #0x12,0x70(a6)                  | +174
        lea     0x27991c.l,a0                   | +17a
        move.l  -0x4(a0),0x74(a6)               | +180
.L03535e:
        cmpi.b  #0x14,0x79(a6)                  | +186
        bne.w   .L03536e                        | +18c
        bset    #0x5,0x69(a6)                   | +190
.L03536e:
        bra.w   .L0353c4                        | +196
.L035372:
        cmpi.w  #0x0,0x72(a6)                   | +19a
        beq.w   .L0353c4                        | +1a0
        move.w  #0x0,0x72(a6)                   | +1a4
        jsr     0x2abcc.l                       | +1aa
        bcs.w   .L0353a2                        | +1b0
        move.b  #0x12,0x70(a6)                  | +1b4
        lea     0x279926.l,a0                   | +1ba
        move.l  -0x4(a0),0x74(a6)               | +1c0
        bra.w   .L0353b4                        | +1c6
.L0353a2:
        move.b  #0x12,0x70(a6)                  | +1ca
        lea     0x279926.l,a0                   | +1d0
        move.l  -0x4(a0),0x74(a6)               | +1d6
.L0353b4:
        cmpi.b  #0x41,0x79(a6)                  | +1dc
        bne.w   .L0353c4                        | +1e2
        bset    #0x5,0x69(a6)                   | +1e6
.L0353c4:
        btst    #0x2,0x8c(a6)                   | +1ec
        beq.w   .L035402                        | +1f2
        jsr     JmpAbsThunk_032e3c(pc)          | +1f6
        bcs.w   .L0353dc                        | +1fa
        lea     Player_Stand_034704(pc),a1      | +1fe
        move.l  a1,(a6)                         | +202
.L0353dc:
        jsr     Input_RightThunk_032e42(pc)     | +204
        bcc.w   .L0353fe                        | +208
        btst    #0x2,0x8c(a6)                   | +20c
        beq.w   .L0353f8                        | +212
        eori.b  #0x1,0x3a(a6)                   | +216
        bra.w   .L0353fe                        | +21c
.L0353f8:
        lea     Player_TurnLeft_035bf8(pc),a1   | +220
        move.l  a1,(a6)                         | +224
.L0353fe:
        bra.w   Player_Walk_Tail_03548c | +226
.L035402:
        jsr     JmpAbsThunk_032e3c(pc)          | +22a
        bcs.w   .L035410                        | +22e
        lea     Player_Idle_StateCheck_034046(pc),a1 | +232
        move.l  a1,(a6)                         | +236
.L035410:
        jsr     Input_RightThunk_032e42(pc)     | +238
        bcc.w   .L035432                        | +23c
        btst    #0x2,0x8c(a6)                   | +240
        beq.w   .L03542c                        | +246
        eori.b  #0x1,0x3a(a6)                   | +24a
        bra.w   .L035432                        | +250
.L03542c:
        lea     Player_TurnLeft_035bf8(pc),a1   | +254
        move.l  a1,(a6)                         | +258
.L035432:
        jsr     Player_ActionSelect_0330d0(pc)  | +25a
        bcc.w   Player_Walk_Tail_03548c | +25e
        cmpi.b  #0xff,d1                        | +262
        bne.w   .L03544c                        | +266
        lea     Player_Melee_035d34(pc),a1      | +26a
        move.l  a1,(a6)                         | +26e
        bra.w   Player_Walk_Tail_03548c | +270
.L03544c:
        cmpi.b  #0x3,d1                         | +274
        bne.w   .L03545e                        | +278
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +27c
        move.l  a1,(a6)                         | +280
        bra.w   Player_Walk_Tail_03548c | +282
.L03545e:
        cmpi.b  #0x4,d1                         | +286
        bne.w   .L035470                        | +28a
        lea     Player_CrouchShoot_03873c(pc),a1             | +28e
        move.l  a1,(a6)                         | +292
        bra.w   Player_Walk_Tail_03548c | +294
.L035470:
        cmpi.b  #0x1,d1                         | +298
        bne.w   .L035482                        | +29c
        lea     Player_WalkLoopRight_ShootUp_0357be(pc),a1 | +2a0
        move.l  a1,(a6)                         | +2a4
        bra.w   Player_Walk_Tail_03548c | +2a6
.L035482:
        lea     Player_WalkLoopRight_Shoot_0357aa(pc),a1 | +2aa
        move.l  a1,(a6)                         | +2ae
        bra.w   Player_Walk_Tail_03548c | +2b0
        .global Player_Walk_Tail_03548c
Player_Walk_Tail_03548c:
        jsr     Input_DownPressed_032e90(pc)    | +2b4
        bcc.w   .L03549a                        | +2b8
        lea     Player_CrouchEnter_037c74(pc),a1             | +2bc
        move.l  a1,(a6)                         | +2c0
.L03549a:
        cmpi.b  #0x0,0x85(a6)                   | +2c2
        bne.w   .L0354b4                        | +2c8
        cmpi.b  #0x1,0x71(a6)                   | +2cc
        bne.w   .L0354b4                        | +2d2
        lea     Player_Reload_033b9a(pc),a1     | +2d6
        move.l  a1,(a6)                         | +2da
.L0354b4:
        cmpi.w  #0x0,0x82(a6)                   | +2dc
        bne.w   .L0354ce                        | +2e2
        cmpi.b  #0x0,0x71(a6)                   | +2e6
        beq.w   .L0354ce                        | +2ec
        lea     Player_Idle_033d64(pc),a1       | +2f0
        move.l  a1,(a6)                         | +2f4
.L0354ce:
        jsr     0x27eba.l                       | +2f6
        bcc.w   .L0354de                        | +2fc
        lea     Player_Knockback_036d64(pc),a1       | +300
        move.l  a1,(a6)                         | +304
.L0354de:
        jsr     Input_FireByMode_033034(pc)     | +306
        bcc.w   .L0354ec                        | +30a
        lea     Player_JumpStart_036914(pc),a1             | +30e
        move.l  a1,(a6)                         | +312
.L0354ec:
        jsr     PlayerRoute_PublishState_033522(pc) | +314
        rts                                     | +318

| ----------------------------------------------------------------------------
|  Player_WalkLoopLeft_0354f2  @ $0354F2  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopLeft_0354f2, "ax", @progbits
        .global Player_WalkLoopLeft_0354f2
Player_WalkLoopLeft_0354f2:
        cmpi.b  #0x13,0x70(a6)                  | +000
        bcs.w   .L035528                        | +006
        cmpi.b  #0x24,0x70(a6)                  | +00a
        bcc.w   .L035528                        | +010
        lea     0x279926.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x3,0x7e(a6)                   | +026
        bclr    #0x5,0x69(a6)                   | +02c
        bra.w   Player_WalkLoopLeft_Setup_03556a | +032
.L035528:
        move.w  #0x0,0x7c(a6)                   | +036
        move.w  #0xffff,0x7e(a6)                | +03c
        bclr    #0x2,0x8c(a6)                   | +042
        bclr    #0x1,0x8c(a6)                   | +048
        bclr    #0x3,0x8c(a6)                   | +04e
        move.b  #0x12,0x70(a6)                  | +054
        lea     0x279926.l,a0                   | +05a
        move.l  -0x4(a0),0x74(a6)               | +060
        move.b  #0xff,0x21(a6)                  | +066
        lea     0x279926.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        .global Player_WalkLoopLeft_Setup_03556a
Player_WalkLoopLeft_Setup_03556a:
        move.w  #0xfd00,0x28(a6)                | +078
        clr.w   0x2c(a6)                        | +07e
        move.l  #0x32500,0x60(a6)               | +082
        lea     Player_HitboxStand_0326e0(pc),a0             | +08a
        move.l  a0,0x48(a6)                     | +08e
        lea     Player_WalkLoopLeft_Run_035590(pc),a1 | +092
        move.l  a1,(a6)                         | +096
        bset    #0x0,0x3a(a6)                   | +098

| ----------------------------------------------------------------------------
|  Player_WalkLoopLeft_Run_035590  @ $035590  (538 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopLeft_Run_035590, "ax", @progbits
        .global Player_WalkLoopLeft_Run_035590
Player_WalkLoopLeft_Run_035590:
        jsr     Player_FrameCommon_032ff2(pc)   | +000
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +004
        bcc.w   .L0355a2                        | +008
        lea     Player_ReenterByInput_0345b8(pc),a1 | +00c
        move.l  a1,(a6)                         | +010
.L0355a2:
        move.w  #0xfd00,0x28(a6)                | +012
        jsr     0x27a92.l                       | +018
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +01e
        bcc.w   .L035622                        | +022
        move.w  #0x0,0x7c(a6)                   | +026
        move.w  #0xffff,0x7e(a6)                | +02c
        bclr    #0x2,0x8c(a6)                   | +032
        bclr    #0x1,0x8c(a6)                   | +038
        bclr    #0x3,0x8c(a6)                   | +03e
        cmpi.b  #0x13,0x70(a6)                  | +044
        bcs.w   .L0355fe                        | +04a
        cmpi.b  #0x24,0x70(a6)                  | +04e
        bcc.w   .L0355fe                        | +054
        move.b  #0x12,0x70(a6)                  | +058
        lea     0x279926.l,a0                   | +05e
        move.l  -0x4(a0),0x74(a6)               | +064
        bra.w   .L035622                        | +06a
.L0355fe:
        move.b  #0x12,0x70(a6)                  | +06e
        lea     0x279926.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        move.b  #0xff,0x21(a6)                  | +080
        lea     0x279926.l,a0                   | +086
        jsr     0x28cd4.l                       | +08c
.L035622:
        cmpi.b  #0x12,0x70(a6)                  | +092
        bne.w   .L0356de                        | +098
        cmpi.b  #0x4,0x78(a6)                   | +09c
        bne.w   .L03568c                        | +0a2
        cmpi.w  #0x1,0x72(a6)                   | +0a6
        beq.w   .L035688                        | +0ac
        move.w  #0x1,0x72(a6)                   | +0b0
        jsr     0x2abcc.l                       | +0b6
        bcs.w   .L035666                        | +0bc
        move.b  #0x12,0x70(a6)                  | +0c0
        lea     0x27991c.l,a0                   | +0c6
        move.l  -0x4(a0),0x74(a6)               | +0cc
        bra.w   .L035678                        | +0d2
.L035666:
        move.b  #0x12,0x70(a6)                  | +0d6
        lea     0x27991c.l,a0                   | +0dc
        move.l  -0x4(a0),0x74(a6)               | +0e2
.L035678:
        cmpi.b  #0x14,0x79(a6)                  | +0e8
        bne.w   .L035688                        | +0ee
        bset    #0x5,0x69(a6)                   | +0f2
.L035688:
        bra.w   .L0356de                        | +0f8
.L03568c:
        cmpi.w  #0x0,0x72(a6)                   | +0fc
        beq.w   .L0356de                        | +102
        move.w  #0x0,0x72(a6)                   | +106
        jsr     0x2abcc.l                       | +10c
        bcs.w   .L0356bc                        | +112
        move.b  #0x12,0x70(a6)                  | +116
        lea     0x279926.l,a0                   | +11c
        move.l  -0x4(a0),0x74(a6)               | +122
        bra.w   .L0356ce                        | +128
.L0356bc:
        move.b  #0x12,0x70(a6)                  | +12c
        lea     0x279926.l,a0                   | +132
        move.l  -0x4(a0),0x74(a6)               | +138
.L0356ce:
        cmpi.b  #0x41,0x79(a6)                  | +13e
        bne.w   .L0356de                        | +144
        bset    #0x5,0x69(a6)                   | +148
.L0356de:
        btst    #0x2,0x8c(a6)                   | +14e
        beq.w   .L03571c                        | +154
        jsr     Input_RightThunk_032e42(pc)     | +158
        bcs.w   .L0356f6                        | +15c
        lea     Player_Stand_034704(pc),a1      | +160
        move.l  a1,(a6)                         | +164
.L0356f6:
        jsr     JmpAbsThunk_032e3c(pc)          | +166
        bcc.w   .L035718                        | +16a
        btst    #0x2,0x8c(a6)                   | +16e
        beq.w   .L035712                        | +174
        eori.b  #0x1,0x3a(a6)                   | +178
        bra.w   .L035718                        | +17e
.L035712:
        lea     Player_TurnRight_035aba(pc),a1  | +182
        move.l  a1,(a6)                         | +186
.L035718:
        bra.w   .L0357a6                        | +188
.L03571c:
        jsr     Input_RightThunk_032e42(pc)     | +18c
        bcs.w   .L03572a                        | +190
        lea     Player_Idle_StateCheck_034046(pc),a1 | +194
        move.l  a1,(a6)                         | +198
.L03572a:
        jsr     JmpAbsThunk_032e3c(pc)          | +19a
        bcc.w   .L03574c                        | +19e
        btst    #0x2,0x8c(a6)                   | +1a2
        beq.w   .L035746                        | +1a8
        eori.b  #0x1,0x3a(a6)                   | +1ac
        bra.w   .L03574c                        | +1b2
.L035746:
        lea     Player_TurnRight_035aba(pc),a1  | +1b6
        move.l  a1,(a6)                         | +1ba
.L03574c:
        jsr     Player_ActionSelect_0330d0(pc)  | +1bc
        bcc.w   .L0357a6                        | +1c0
        cmpi.b  #0xff,d1                        | +1c4
        bne.w   .L035766                        | +1c8
        lea     Player_Melee_035d34(pc),a1      | +1cc
        move.l  a1,(a6)                         | +1d0
        bra.w   .L0357a6                        | +1d2
.L035766:
        cmpi.b  #0x3,d1                         | +1d6
        bne.w   .L035778                        | +1da
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +1de
        move.l  a1,(a6)                         | +1e2
        bra.w   .L0357a6                        | +1e4
.L035778:
        cmpi.b  #0x4,d1                         | +1e8
        bne.w   .L03578a                        | +1ec
        lea     Player_CrouchShoot_03873c(pc),a1             | +1f0
        move.l  a1,(a6)                         | +1f4
        bra.w   .L0357a6                        | +1f6
.L03578a:
        cmpi.b  #0x1,d1                         | +1fa
        bne.w   .L03579c                        | +1fe
        lea     Player_WalkLoopLeft_ShootUp_0357e6(pc),a1 | +202
        move.l  a1,(a6)                         | +206
        bra.w   .L0357a6                        | +208
.L03579c:
        lea     Player_WalkLoopLeft_Shoot_0357d2(pc),a1 | +20c
        move.l  a1,(a6)                         | +210
        bra.w   .L0357a6                        | +212
.L0357a6:
        bra.w   Player_Walk_Tail_03548c | +216

| ----------------------------------------------------------------------------
|  Player_WalkLoopRight_Shoot_0357aa  @ $0357AA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopRight_Shoot_0357aa, "ax", @progbits
        .global Player_WalkLoopRight_Shoot_0357aa
Player_WalkLoopRight_Shoot_0357aa:
        move.w  #0x1,0x7c(a6)                   | +000
        move.w  #0x3,0x7e(a6)                   | +006
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +00c
        bra.w   Player_WalkLoopRight_Setup_035250 | +010

| ----------------------------------------------------------------------------
|  Player_WalkLoopRight_ShootUp_0357be  @ $0357BE  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopRight_ShootUp_0357be, "ax", @progbits
        .global Player_WalkLoopRight_ShootUp_0357be
Player_WalkLoopRight_ShootUp_0357be:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x4,0x7e(a6)                   | +006
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +00c
        bra.w   Player_WalkLoopRight_Setup_035250 | +010

| ----------------------------------------------------------------------------
|  Player_WalkLoopLeft_Shoot_0357d2  @ $0357D2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopLeft_Shoot_0357d2, "ax", @progbits
        .global Player_WalkLoopLeft_Shoot_0357d2
Player_WalkLoopLeft_Shoot_0357d2:
        move.w  #0x1,0x7c(a6)                   | +000
        move.w  #0x3,0x7e(a6)                   | +006
        jsr     Player_WalkLoopShootPose_0357fa(pc) | +00c
        bra.w   Player_WalkLoopLeft_Setup_03556a | +010

| ----------------------------------------------------------------------------
|  Player_WalkLoopLeft_ShootUp_0357e6  @ $0357E6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopLeft_ShootUp_0357e6, "ax", @progbits
        .global Player_WalkLoopLeft_ShootUp_0357e6
Player_WalkLoopLeft_ShootUp_0357e6:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x4,0x7e(a6)                   | +006
        jsr     Player_WalkLoopShootUpPose_03595a(pc) | +00c
        bra.w   Player_WalkLoopLeft_Setup_03556a | +010

| ----------------------------------------------------------------------------
|  Player_WalkLoopShootPose_0357fa  @ $0357FA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopShootPose_0357fa, "ax", @progbits
        .global Player_WalkLoopShootPose_0357fa
Player_WalkLoopShootPose_0357fa:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x1,0x72(a6)                   | +006
        bne.w   .L035858                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L035830                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x23,0x70(a6)                  | +020
        lea     0x2798f4.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L035854                        | +032
.L035830:
        move.b  #0x23,0x70(a6)                  | +036
        lea     0x2798f4.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798f4.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L035854:
        bra.w   .L0358a2                        | +05a
.L035858:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L03587e                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x23,0x70(a6)                  | +06e
        lea     0x2798e0.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L0358a2                        | +080
.L03587e:
        move.b  #0x23,0x70(a6)                  | +084
        lea     0x2798e0.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x2798e0.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L0358a2:
        move.w  #0x0,0x72(a6)                   | +0a8
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Player_WalkLoopShootPose_Alt_0358aa  @ $0358AA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopShootPose_Alt_0358aa, "ax", @progbits
        .global Player_WalkLoopShootPose_Alt_0358aa
Player_WalkLoopShootPose_Alt_0358aa:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x1,0x72(a6)                   | +006
        bne.w   .L035908                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L0358e0                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x23,0x70(a6)                  | +020
        lea     0x2798f4.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L035904                        | +032
.L0358e0:
        move.b  #0x23,0x70(a6)                  | +036
        lea     0x2798f4.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798f4.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L035904:
        bra.w   .L035952                        | +05a
.L035908:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L03592e                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x23,0x70(a6)                  | +06e
        lea     0x279908.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L035952                        | +080
.L03592e:
        move.b  #0x23,0x70(a6)                  | +084
        lea     0x279908.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x279908.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L035952:
        move.w  #0x0,0x72(a6)                   | +0a8
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Player_WalkLoopShootUpPose_03595a  @ $03595A  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopShootUpPose_03595a, "ax", @progbits
        .global Player_WalkLoopShootUpPose_03595a
Player_WalkLoopShootUpPose_03595a:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        bne.w   .L0359b8                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L035990                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x23,0x70(a6)                  | +020
        lea     0x2798fe.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L0359b4                        | +032
.L035990:
        move.b  #0x23,0x70(a6)                  | +036
        lea     0x2798fe.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798fe.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L0359b4:
        bra.w   .L035a02                        | +05a
.L0359b8:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L0359de                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x23,0x70(a6)                  | +06e
        lea     0x2798ea.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L035a02                        | +080
.L0359de:
        move.b  #0x23,0x70(a6)                  | +084
        lea     0x2798ea.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x2798ea.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L035a02:
        move.w  #0x1,0x72(a6)                   | +0a8
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Player_WalkLoopShootUpPose_Alt_035a0a  @ $035A0A  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WalkLoopShootUpPose_Alt_035a0a, "ax", @progbits
        .global Player_WalkLoopShootUpPose_Alt_035a0a
Player_WalkLoopShootUpPose_Alt_035a0a:
        bset    #0x1,0x8c(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +006
        bne.w   .L035a68                        | +00c
        cmpi.b  #0x24,0x70(a6)                  | +010
        bcc.w   .L035a40                        | +016
        bset    #0x5,0x69(a6)                   | +01a
        move.b  #0x23,0x70(a6)                  | +020
        lea     0x2798fe.l,a0                   | +026
        move.l  -0x4(a0),0x74(a6)               | +02c
        bra.w   .L035a64                        | +032
.L035a40:
        move.b  #0x23,0x70(a6)                  | +036
        lea     0x2798fe.l,a0                   | +03c
        move.l  -0x4(a0),0x74(a6)               | +042
        move.b  #0xff,0x21(a6)                  | +048
        lea     0x2798fe.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
.L035a64:
        bra.w   .L035ab2                        | +05a
.L035a68:
        cmpi.b  #0x24,0x70(a6)                  | +05e
        bcc.w   .L035a8e                        | +064
        bset    #0x5,0x69(a6)                   | +068
        move.b  #0x23,0x70(a6)                  | +06e
        lea     0x279912.l,a0                   | +074
        move.l  -0x4(a0),0x74(a6)               | +07a
        bra.w   .L035ab2                        | +080
.L035a8e:
        move.b  #0x23,0x70(a6)                  | +084
        lea     0x279912.l,a0                   | +08a
        move.l  -0x4(a0),0x74(a6)               | +090
        move.b  #0xff,0x21(a6)                  | +096
        lea     0x279912.l,a0                   | +09c
        jsr     0x28cd4.l                       | +0a2
.L035ab2:
        move.w  #0x1,0x72(a6)                   | +0a8
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  Player_TurnRight_035aba  @ $035ABA  (318 B)
| ----------------------------------------------------------------------------
        .section .text.Player_TurnRight_035aba, "ax", @progbits
        .global Player_TurnRight_035aba
Player_TurnRight_035aba:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        cmpi.w  #0x1,0x72(a6)                   | +01e
        bne.w   .L035b0a                        | +024
        move.b  #0x31,0x70(a6)                  | +028
        lea     0x279a5a.l,a0                   | +02e
        move.l  -0x4(a0),0x74(a6)               | +034
        move.b  #0xff,0x21(a6)                  | +03a
        lea     0x279a5a.l,a0                   | +040
        jsr     0x28cd4.l                       | +046
        bra.w   .L035b2e                        | +04c
.L035b0a:
        move.b  #0x31,0x70(a6)                  | +050
        lea     0x279a3c.l,a0                   | +056
        move.l  -0x4(a0),0x74(a6)               | +05c
        move.b  #0xff,0x21(a6)                  | +062
        lea     0x279a3c.l,a0                   | +068
        jsr     0x28cd4.l                       | +06e
.L035b2e:
        move.w  #0x300,0x28(a6)                 | +074
        clr.w   0x2c(a6)                        | +07a
        move.l  #0x32500,0x60(a6)               | +07e
        lea     Player_HitboxStand_0326e0(pc),a0             | +086
        move.l  a0,0x48(a6)                     | +08a
        bclr    #0x0,0x3a(a6)                   | +08e
        lea     .L035b54(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L035b54:
        jsr     Player_FrameCommon_032ff2(pc)   | +09a
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +09e
        bcc.w   .L035b66                        | +0a2
        lea     Player_ReenterByInput_0345b8(pc),a1 | +0a6
        move.l  a1,(a6)                         | +0aa
.L035b66:
        move.w  #0x300,0x28(a6)                 | +0ac
        jsr     0x27a92.l                       | +0b2
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0b8
        bcc.w   Player_Turn_Actions_035b90 | +0bc
        lea     Player_Stand_034704(pc),a1      | +0c0
        move.l  a1,(a6)                         | +0c4
        btst    #0x0,0x88(a6)                   | +0c6
        beq.w   Player_Turn_Actions_035b90 | +0cc
        lea     Player_Crouch_033a5e(pc),a1     | +0d0
        move.l  a1,(a6)                         | +0d4
        .global Player_Turn_Actions_035b90
Player_Turn_Actions_035b90:
        btst    #0x2,0x8c(a6)                   | +0d6
        bne.w   .L035bf4                        | +0dc
        jsr     Player_ActionSelect_0330d0(pc)  | +0e0
        bcc.w   .L035bf4                        | +0e4
        cmpi.b  #0xff,d1                        | +0e8
        bne.w   .L035bb4                        | +0ec
        lea     Player_Melee_035d34(pc),a1      | +0f0
        move.l  a1,(a6)                         | +0f4
        bra.w   .L035bf4                        | +0f6
.L035bb4:
        cmpi.b  #0x3,d1                         | +0fa
        bne.w   .L035bc6                        | +0fe
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +102
        move.l  a1,(a6)                         | +106
        bra.w   .L035bf4                        | +108
.L035bc6:
        cmpi.b  #0x4,d1                         | +10c
        bne.w   .L035bd8                        | +110
        lea     Player_CrouchShoot_03873c(pc),a1             | +114
        move.l  a1,(a6)                         | +118
        bra.w   .L035bf4                        | +11a
.L035bd8:
        cmpi.b  #0x1,d1                         | +11e
        bne.w   .L035bea                        | +122
        lea     Player_ShootStandUp_03437e(pc),a1 | +126
        move.l  a1,(a6)                         | +12a
        bra.w   .L035bf4                        | +12c
.L035bea:
        lea     Player_ShootStand_0342c4(pc),a1 | +130
        move.l  a1,(a6)                         | +134
        bra.w   .L035bf4                        | +136
.L035bf4:
        jmp     Player_Walk_Tail_03548c(pc) | +13a

| ----------------------------------------------------------------------------
|  Player_TurnLeft_035bf8  @ $035BF8  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Player_TurnLeft_035bf8, "ax", @progbits
        .global Player_TurnLeft_035bf8
Player_TurnLeft_035bf8:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        cmpi.w  #0x1,0x72(a6)                   | +01e
        bne.w   .L035c4e                        | +024
        jsr     0x2abcc.l                       | +028
        move.b  #0x31,0x70(a6)                  | +02e
        lea     0x279a5a.l,a0                   | +034
        move.l  -0x4(a0),0x74(a6)               | +03a
        move.b  #0xff,0x21(a6)                  | +040
        lea     0x279a5a.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        bra.w   .L035c72                        | +052
.L035c4e:
        move.b  #0x31,0x70(a6)                  | +056
        lea     0x279a3c.l,a0                   | +05c
        move.l  -0x4(a0),0x74(a6)               | +062
        move.b  #0xff,0x21(a6)                  | +068
        lea     0x279a3c.l,a0                   | +06e
        jsr     0x28cd4.l                       | +074
.L035c72:
        move.w  #0xfd00,0x28(a6)                | +07a
        clr.w   0x2c(a6)                        | +080
        move.l  #0x32500,0x60(a6)               | +084
        lea     Player_HitboxStand_0326e0(pc),a0             | +08c
        move.l  a0,0x48(a6)                     | +090
        lea     Player_TurnLeft_Run_035c98(pc),a1 | +094
        move.l  a1,(a6)                         | +098
        bset    #0x0,0x3a(a6)                   | +09a

| ----------------------------------------------------------------------------
|  Player_TurnLeft_Run_035c98  @ $035C98  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Player_TurnLeft_Run_035c98, "ax", @progbits
        .global Player_TurnLeft_Run_035c98
Player_TurnLeft_Run_035c98:
        jsr     Player_FrameCommon_032ff2(pc)   | +000
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +004
        bcc.w   .L035caa                        | +008
        lea     Player_ReenterByInput_0345b8(pc),a1 | +00c
        move.l  a1,(a6)                         | +010
.L035caa:
        move.w  #0xfd00,0x28(a6)                | +012
        jsr     0x27a92.l                       | +018
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +01e
        bcc.w   .L035cd4                        | +022
        lea     Player_Stand_034704(pc),a1      | +026
        move.l  a1,(a6)                         | +02a
        btst    #0x0,0x88(a6)                   | +02c
        beq.w   .L035cd4                        | +032
        lea     Player_Crouch_033a5e(pc),a1     | +036
        move.l  a1,(a6)                         | +03a
.L035cd4:
        bra.w   Player_Turn_Actions_035b90 | +03c

| ----------------------------------------------------------------------------
|  Player_ActionDispatch_035cd8  @ $035CD8  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ActionDispatch_035cd8, "ax", @progbits
        .global Player_ActionDispatch_035cd8
Player_ActionDispatch_035cd8:
        jsr     Player_ActionSelect_0330d0(pc)  | +000
        bcc.w   .L035d32                        | +004
        cmpi.b  #0xff,d1                        | +008
        bne.w   .L035cf2                        | +00c
        lea     Player_Melee_035d34(pc),a1      | +010
        move.l  a1,(a6)                         | +014
        bra.w   .L035d32                        | +016
.L035cf2:
        cmpi.b  #0x3,d1                         | +01a
        bne.w   .L035d04                        | +01e
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +022
        move.l  a1,(a6)                         | +026
        bra.w   .L035d32                        | +028
.L035d04:
        cmpi.b  #0x4,d1                         | +02c
        bne.w   .L035d16                        | +030
        lea     Player_CrouchShoot_03873c(pc),a1             | +034
        move.l  a1,(a6)                         | +038
        bra.w   .L035d32                        | +03a
.L035d16:
        cmpi.b  #0x1,d1                         | +03e
        bne.w   .L035d28                        | +042
        lea     Player_ShootStandUp_03437e(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
        bra.w   .L035d32                        | +04c
.L035d28:
        lea     Player_ShootStand_0342c4(pc),a1 | +050
        move.l  a1,(a6)                         | +054
        bra.w   .L035d32                        | +056
.L035d32:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Player_Melee_035d34  @ $035D34  (172 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Melee_035d34, "ax", @progbits
        .global Player_Melee_035d34
Player_Melee_035d34:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        lea     Player_AttackTblB_032638(pc),a0             | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        clr.w   0x28(a6)                        | +02c
        clr.w   0x2c(a6)                        | +030
        clr.w   0x2a(a6)                        | +034
        clr.w   0x2e(a6)                        | +038
        move.l  #0x32500,0x60(a6)               | +03c
        jsr     0x2abcc.l                       | +044
        bcs.w   .L035daa                        | +04a
        move.b  #0x33,0x70(a6)                  | +04e
        lea     0x279d56.l,a0                   | +054
        move.l  -0x4(a0),0x74(a6)               | +05a
        move.b  #0xff,0x21(a6)                  | +060
        lea     0x279d56.l,a0                   | +066
        jsr     0x28cd4.l                       | +06c
        bra.w   .L035dce                        | +072
.L035daa:
        move.b  #0x33,0x70(a6)                  | +076
        lea     0x279d56.l,a0                   | +07c
        move.l  -0x4(a0),0x74(a6)               | +082
        move.b  #0xff,0x21(a6)                  | +088
        lea     0x279d56.l,a0                   | +08e
        jsr     0x28cd4.l                       | +094
.L035dce:
        lea     Player_HitboxMelee_032788(pc),a0             | +09a
        move.l  a0,0x48(a6)                     | +09e
        lea     Player_Melee_Run_035de0(pc),a1  | +0a2
        move.l  a1,(a6)                         | +0a6
        bra.w   Player_Melee_Frame_035e54 | +0a8

| ----------------------------------------------------------------------------
|  Player_Melee_Run_035de0  @ $035DE0  (200 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Melee_Run_035de0, "ax", @progbits
        .global Player_Melee_Run_035de0
Player_Melee_Run_035de0:
        clr.w   d7                              | +000
        jsr     JmpAbsThunk_032e3c(pc)          | +002
        bcc.w   .L035dee                        | +006
        move.w  #0x300,d7                       | +00a
.L035dee:
        jsr     Input_RightThunk_032e42(pc)     | +00e
        bcc.w   .L035dfa                        | +012
        move.w  #0xfd00,d7                      | +016
.L035dfa:
        cmp.w   0x28(a6),d7                     | +01a
        beq.w   .L035e50                        | +01e
        cmpi.w  #0x0,d7                         | +022
        bne.w   .L035e20                        | +026
        lea     0x279d56.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        bclr    #0x5,0x69(a6)                   | +036
        bra.w   .L035e50                        | +03c
.L035e20:
        cmp.w   0x28(a6),d7                     | +040
        ble.w   .L035e3e                        | +044
        lea     0x279d76.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        bclr    #0x5,0x69(a6)                   | +054
        bra.w   .L035e50                        | +05a
.L035e3e:
        lea     0x279e30.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        bclr    #0x5,0x69(a6)                   | +06a
.L035e50:
        move.w  d7,0x28(a6)                     | +070
        .global Player_Melee_Frame_035e54
Player_Melee_Frame_035e54:
        jsr     Player_FrameCommon_032ff2(pc)   | +074
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +078
        jsr     0x27a92.l                       | +07c
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +082
        bcc.w   .L035ea0                        | +086
        lea     Player_Stand_034704(pc),a1      | +08a
        move.l  a1,(a6)                         | +08e
        move.b  #0x1,0x78(a6)                   | +090
        lsl.b   #0x4,d0                         | +096
        or.b    0x78(a6),d0                     | +098
        move.b  d0,0x79(a6)                     | +09c
        btst    #0x0,0x88(a6)                   | +0a0
        beq.w   .L035e90                        | +0a6
        lea     Player_Crouch_033a5e(pc),a1     | +0aa
        move.l  a1,(a6)                         | +0ae
.L035e90:
        cmpi.w  #0x0,0x82(a6)                   | +0b0
        bne.w   .L035ea0                        | +0b6
        lea     Player_Idle_033d64(pc),a1       | +0ba
        move.l  a1,(a6)                         | +0be
.L035ea0:
        jsr     Player_ActionDispatch_035cd8(pc) | +0c0
        jmp     Player_Stand_Tail_034ada(pc) | +0c4

| ----------------------------------------------------------------------------
|  Player_MeleeAlt_035ea8  @ $035EA8  (200 B)
| ----------------------------------------------------------------------------
        .section .text.Player_MeleeAlt_035ea8, "ax", @progbits
        .global Player_MeleeAlt_035ea8
Player_MeleeAlt_035ea8:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        lea     Player_AttackTblB_032638(pc),a0             | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        move.l  #0x32500,0x60(a6)               | +02c
        jsr     0x2abcc.l                       | +034
        bcs.w   .L035f0e                        | +03a
        move.b  #0x33,0x70(a6)                  | +03e
        lea     0x279efe.l,a0                   | +044
        move.l  -0x4(a0),0x74(a6)               | +04a
        move.b  #0xff,0x21(a6)                  | +050
        lea     0x279efe.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        bra.w   .L035f32                        | +062
.L035f0e:
        move.b  #0x33,0x70(a6)                  | +066
        lea     0x279efe.l,a0                   | +06c
        move.l  -0x4(a0),0x74(a6)               | +072
        move.b  #0xff,0x21(a6)                  | +078
        lea     0x279efe.l,a0                   | +07e
        jsr     0x28cd4.l                       | +084
.L035f32:
        lea     Player_HitboxMelee_032788(pc),a0             | +08a
        move.l  a0,0x48(a6)                     | +08e
        lea     .L035f40(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L035f40:
        jsr     Player_FrameCommon_032ff2(pc)   | +098
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +09c
        jsr     0x27a92.l                       | +0a0
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0a6
        bcc.w   .L035f6c                        | +0aa
        lea     Player_Stand_034704(pc),a1      | +0ae
        move.l  a1,(a6)                         | +0b2
        move.b  #0x1,0x78(a6)                   | +0b4
        lsl.b   #0x4,d0                         | +0ba
        or.b    0x78(a6),d0                     | +0bc
        move.b  d0,0x79(a6)                     | +0c0
.L035f6c:
        jmp     Player_WalkRight_Tail_034ccc(pc) | +0c4

| ----------------------------------------------------------------------------
|  Player_MeleeAlt_Walk_035f70  @ $035F70  (332 B)
| ----------------------------------------------------------------------------
        .section .text.Player_MeleeAlt_Walk_035f70, "ax", @progbits
        .global Player_MeleeAlt_Walk_035f70
Player_MeleeAlt_Walk_035f70:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        lea     Player_AttackTblB_032638(pc),a0             | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        move.l  #0x32500,0x60(a6)               | +02c
        jsr     0x2abcc.l                       | +034
        bcs.w   .L035fd6                        | +03a
        move.b  #0x33,0x70(a6)                  | +03e
        lea     0x279efe.l,a0                   | +044
        move.l  -0x4(a0),0x74(a6)               | +04a
        move.b  #0xff,0x21(a6)                  | +050
        lea     0x279efe.l,a0                   | +056
        jsr     0x28cd4.l                       | +05c
        bra.w   .L035ffa                        | +062
.L035fd6:
        move.b  #0x33,0x70(a6)                  | +066
        lea     0x279efe.l,a0                   | +06c
        move.l  -0x4(a0),0x74(a6)               | +072
        move.b  #0xff,0x21(a6)                  | +078
        lea     0x279efe.l,a0                   | +07e
        jsr     0x28cd4.l                       | +084
.L035ffa:
        lea     Player_HitboxMelee_032788(pc),a0             | +08a
        move.l  a0,0x48(a6)                     | +08e
        lea     .L036008(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L036008:
        jsr     Player_FrameCommon_032ff2(pc)   | +098
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +09c
        clr.w   d7                              | +0a0
        jsr     JmpAbsThunk_032e3c(pc)          | +0a2
        bcc.w   .L036024                        | +0a6
        bclr    #0x0,0x3a(a6)                   | +0aa
        move.w  #0x300,d7                       | +0b0
.L036024:
        bcc.w   .L036036                        | +0b4
        jsr     Input_RightThunk_032e42(pc)     | +0b8
        bset    #0x0,0x3a(a6)                   | +0bc
        move.w  #0xfd00,d7                      | +0c2
.L036036:
        move.w  d7,0x28(a6)                     | +0c6
        jsr     0x27a92.l                       | +0ca
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0d0
        bcc.w   .L03605e                        | +0d4
        lea     Player_Stand_034704(pc),a1      | +0d8
        move.l  a1,(a6)                         | +0dc
        move.b  #0x1,0x78(a6)                   | +0de
        lsl.b   #0x4,d0                         | +0e4
        or.b    0x78(a6),d0                     | +0e6
        move.b  d0,0x79(a6)                     | +0ea
.L03605e:
        jsr     Player_ActionSelect_0330d0(pc)  | +0ee
        bcc.w   .L0360b8                        | +0f2
        cmpi.b  #0xff,d1                        | +0f6
        bne.w   .L036078                        | +0fa
        lea     Player_Melee_035d34(pc),a1      | +0fe
        move.l  a1,(a6)                         | +102
        bra.w   .L0360b8                        | +104
.L036078:
        cmpi.b  #0x3,d1                         | +108
        bne.w   .L03608a                        | +10c
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +110
        move.l  a1,(a6)                         | +114
        bra.w   .L0360b8                        | +116
.L03608a:
        cmpi.b  #0x4,d1                         | +11a
        bne.w   .L03609c                        | +11e
        lea     Player_CrouchShoot_03873c(pc),a1             | +122
        move.l  a1,(a6)                         | +126
        bra.w   .L0360b8                        | +128
.L03609c:
        cmpi.b  #0x1,d1                         | +12c
        bne.w   .L0360ae                        | +130
        lea     Player_WalkLoopRight_ShootUp_0357be(pc),a1 | +134
        move.l  a1,(a6)                         | +138
        bra.w   .L0360b8                        | +13a
.L0360ae:
        lea     Player_WalkLoopRight_Shoot_0357aa(pc),a1 | +13e
        move.l  a1,(a6)                         | +142
        bra.w   .L0360b8                        | +144
.L0360b8:
        jmp     Player_Walk_Tail_03548c(pc) | +148

| ----------------------------------------------------------------------------
|  Player_ThrowGrenade_Stand_0360bc  @ $0360BC  (342 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenade_Stand_0360bc, "ax", @progbits
        .global Player_ThrowGrenade_Stand_0360bc
Player_ThrowGrenade_Stand_0360bc:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        clr.w   0x28(a6)                        | +01e
        clr.w   0x2c(a6)                        | +022
        clr.w   0x2a(a6)                        | +026
        clr.w   0x2e(a6)                        | +02a
        move.l  #0x32500,0x60(a6)               | +02e
        jsr     0x2abcc.l                       | +036
        bcs.w   .L036124                        | +03c
        move.b  #0x33,0x70(a6)                  | +040
        lea     0x279d2e.l,a0                   | +046
        move.l  -0x4(a0),0x74(a6)               | +04c
        move.b  #0xff,0x21(a6)                  | +052
        lea     0x279d2e.l,a0                   | +058
        jsr     0x28cd4.l                       | +05e
        bra.w   .L036148                        | +064
.L036124:
        move.b  #0x33,0x70(a6)                  | +068
        lea     0x279d2e.l,a0                   | +06e
        move.l  -0x4(a0),0x74(a6)               | +074
        move.b  #0xff,0x21(a6)                  | +07a
        lea     0x279d2e.l,a0                   | +080
        jsr     0x28cd4.l                       | +086
.L036148:
        clr.b   0x3b(a6)                        | +08c
        lea     .L036152(pc),a1                 | +090
        move.l  a1,(a6)                         | +094
.L036152:
        jsr     Player_FrameCommon_032ff2(pc)   | +096
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +09a
        jsr     0x27a92.l                       | +09e
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0a4
        bcc.w   .L03619e                        | +0a8
        lea     Player_Stand_034704(pc),a1      | +0ac
        move.l  a1,(a6)                         | +0b0
        move.b  #0x1,0x78(a6)                   | +0b2
        lsl.b   #0x4,d0                         | +0b8
        or.b    0x78(a6),d0                     | +0ba
        move.b  d0,0x79(a6)                     | +0be
        btst    #0x0,0x88(a6)                   | +0c2
        beq.w   .L03618e                        | +0c8
        lea     Player_Crouch_033a5e(pc),a1     | +0cc
        move.l  a1,(a6)                         | +0d0
.L03618e:
        cmpi.w  #0x0,0x82(a6)                   | +0d2
        bne.w   .L03619e                        | +0d8
        lea     Player_Idle_033d64(pc),a1       | +0dc
        move.l  a1,(a6)                         | +0e0
.L03619e:
        btst    #0x2,0x8c(a6)                   | +0e2
        bne.w   .L03620a                        | +0e8
        btst    #0x0,0x3a(a6)                   | +0ec
        bne.w   .L0361d2                        | +0f2
        jsr     JmpAbsThunk_032e3c(pc)          | +0f6
        bcc.w   .L0361c0                        | +0fa
        lea     Player_WalkRight_034b38(pc),a1  | +0fe
        move.l  a1,(a6)                         | +102
.L0361c0:
        jsr     Input_RightThunk_032e42(pc)     | +104
        bcc.w   .L0361ce                        | +108
        lea     Player_TurnLeft_035bf8(pc),a1   | +10c
        move.l  a1,(a6)                         | +110
.L0361ce:
        bra.w   .L0361ee                        | +112
.L0361d2:
        jsr     JmpAbsThunk_032e3c(pc)          | +116
        bcc.w   .L0361e0                        | +11a
        lea     Player_TurnRight_035aba(pc),a1  | +11e
        move.l  a1,(a6)                         | +122
.L0361e0:
        jsr     Input_RightThunk_032e42(pc)     | +124
        bcc.w   .L0361ee                        | +128
        lea     Player_WalkLeft_034d32(pc),a1   | +12c
        move.l  a1,(a6)                         | +130
.L0361ee:
        jsr     Input_DownPressed_032e90(pc)    | +132
        bcc.w   .L0361fc                        | +136
        lea     Player_CrouchEnter_037c74(pc),a1             | +13a
        move.l  a1,(a6)                         | +13e
.L0361fc:
        jsr     Input_FireByMode_033034(pc)     | +140
        bcc.w   .L03620a                        | +144
        lea     Player_JumpStart_036914(pc),a1             | +148
        move.l  a1,(a6)                         | +14c
.L03620a:
        jsr     Player_ActionDispatch_035cd8(pc) | +14e
        jmp     Player_Stand_Tail_034ada(pc) | +152

| ----------------------------------------------------------------------------
|  Player_ThrowGrenade_Walk_036212  @ $036212  (262 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenade_Walk_036212, "ax", @progbits
        .global Player_ThrowGrenade_Walk_036212
Player_ThrowGrenade_Walk_036212:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.l  #0x32500,0x60(a6)               | +01e
        jsr     0x2abcc.l                       | +026
        bcs.w   .L03626a                        | +02c
        move.b  #0x33,0x70(a6)                  | +030
        lea     0x279d38.l,a0                   | +036
        move.l  -0x4(a0),0x74(a6)               | +03c
        move.b  #0xff,0x21(a6)                  | +042
        lea     0x279d38.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        bra.w   .L03628e                        | +054
.L03626a:
        move.b  #0x33,0x70(a6)                  | +058
        lea     0x279d38.l,a0                   | +05e
        move.l  -0x4(a0),0x74(a6)               | +064
        move.b  #0xff,0x21(a6)                  | +06a
        lea     0x279d38.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
.L03628e:
        clr.b   0x3b(a6)                        | +07c
        lea     .L036298(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L036298:
        jsr     Player_FrameCommon_032ff2(pc)   | +086
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +08a
        jsr     0x27a92.l                       | +08e
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +094
        bcc.w   .L0362c4                        | +098
        lea     Player_Stand_034704(pc),a1      | +09c
        move.l  a1,(a6)                         | +0a0
        move.b  #0x1,0x78(a6)                   | +0a2
        lsl.b   #0x4,d0                         | +0a8
        or.b    0x78(a6),d0                     | +0aa
        move.b  d0,0x79(a6)                     | +0ae
.L0362c4:
        btst    #0x2,0x8c(a6)                   | +0b2
        bne.w   .L036314                        | +0b8
        btst    #0x0,0x3a(a6)                   | +0bc
        bne.w   .L0362ea                        | +0c2
        jsr     Input_RightThunk_032e42(pc)     | +0c6
        bcc.w   .L0362e6                        | +0ca
        lea     Player_TurnLeft_035bf8(pc),a1   | +0ce
        move.l  a1,(a6)                         | +0d2
.L0362e6:
        bra.w   .L0362f8                        | +0d4
.L0362ea:
        jsr     JmpAbsThunk_032e3c(pc)          | +0d8
        bcc.w   .L0362f8                        | +0dc
        lea     Player_TurnRight_035aba(pc),a1  | +0e0
        move.l  a1,(a6)                         | +0e4
.L0362f8:
        jsr     Input_DownPressed_032e90(pc)    | +0e6
        bcc.w   .L036306                        | +0ea
        lea     Player_CrouchEnter_037c74(pc),a1             | +0ee
        move.l  a1,(a6)                         | +0f2
.L036306:
        jsr     Input_FireByMode_033034(pc)     | +0f4
        bcc.w   .L036314                        | +0f8
        lea     Player_JumpStart_036914(pc),a1             | +0fc
        move.l  a1,(a6)                         | +100
.L036314:
        jmp     Player_WalkRight_Tail_034ccc(pc) | +102

| ----------------------------------------------------------------------------
|  Player_ThrowGrenade_WalkLoop_036318  @ $036318  (394 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenade_WalkLoop_036318, "ax", @progbits
        .global Player_ThrowGrenade_WalkLoop_036318
Player_ThrowGrenade_WalkLoop_036318:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0xffff,0x7e(a6)                | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.l  #0x32500,0x60(a6)               | +01e
        jsr     0x2abcc.l                       | +026
        bcs.w   .L036370                        | +02c
        move.b  #0x33,0x70(a6)                  | +030
        lea     0x279d38.l,a0                   | +036
        move.l  -0x4(a0),0x74(a6)               | +03c
        move.b  #0xff,0x21(a6)                  | +042
        lea     0x279d38.l,a0                   | +048
        jsr     0x28cd4.l                       | +04e
        bra.w   .L036394                        | +054
.L036370:
        move.b  #0x33,0x70(a6)                  | +058
        lea     0x279d38.l,a0                   | +05e
        move.l  -0x4(a0),0x74(a6)               | +064
        move.b  #0xff,0x21(a6)                  | +06a
        lea     0x279d38.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
.L036394:
        clr.b   0x3b(a6)                        | +07c
        lea     .L03639e(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L03639e:
        jsr     Player_FrameCommon_032ff2(pc)   | +086
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +08a
        clr.w   d7                              | +08e
        jsr     JmpAbsThunk_032e3c(pc)          | +090
        bcc.w   .L0363ba                        | +094
        bclr    #0x0,0x3a(a6)                   | +098
        move.w  #0x300,d7                       | +09e
.L0363ba:
        bcc.w   .L0363cc                        | +0a2
        jsr     Input_RightThunk_032e42(pc)     | +0a6
        bset    #0x0,0x3a(a6)                   | +0aa
        move.w  #0xfd00,d7                      | +0b0
.L0363cc:
        move.w  d7,0x28(a6)                     | +0b4
        jsr     0x27a92.l                       | +0b8
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0be
        bcc.w   .L0363f4                        | +0c2
        lea     Player_Stand_034704(pc),a1      | +0c6
        move.l  a1,(a6)                         | +0ca
        move.b  #0x1,0x78(a6)                   | +0cc
        lsl.b   #0x4,d0                         | +0d2
        or.b    0x78(a6),d0                     | +0d4
        move.b  d0,0x79(a6)                     | +0d8
.L0363f4:
        btst    #0x2,0x8c(a6)                   | +0dc
        bne.w   .L036444                        | +0e2
        btst    #0x0,0x3a(a6)                   | +0e6
        bne.w   .L03641a                        | +0ec
        jsr     Input_RightThunk_032e42(pc)     | +0f0
        bcc.w   .L036416                        | +0f4
        lea     Player_TurnLeft_035bf8(pc),a1   | +0f8
        move.l  a1,(a6)                         | +0fc
.L036416:
        bra.w   .L036428                        | +0fe
.L03641a:
        jsr     JmpAbsThunk_032e3c(pc)          | +102
        bcc.w   .L036428                        | +106
        lea     Player_TurnRight_035aba(pc),a1  | +10a
        move.l  a1,(a6)                         | +10e
.L036428:
        jsr     Input_DownPressed_032e90(pc)    | +110
        bcc.w   .L036436                        | +114
        lea     Player_CrouchEnter_037c74(pc),a1             | +118
        move.l  a1,(a6)                         | +11c
.L036436:
        jsr     Input_FireByMode_033034(pc)     | +11e
        bcc.w   .L036444                        | +122
        lea     Player_JumpStart_036914(pc),a1             | +126
        move.l  a1,(a6)                         | +12a
.L036444:
        jsr     Player_ActionSelect_0330d0(pc)  | +12c
        bcc.w   .L03649e                        | +130
        cmpi.b  #0xff,d1                        | +134
        bne.w   .L03645e                        | +138
        lea     Player_Melee_035d34(pc),a1      | +13c
        move.l  a1,(a6)                         | +140
        bra.w   .L03649e                        | +142
.L03645e:
        cmpi.b  #0x3,d1                         | +146
        bne.w   .L036470                        | +14a
        lea     Player_ThrowGrenade_Stand_0360bc(pc),a1 | +14e
        move.l  a1,(a6)                         | +152
        bra.w   .L03649e                        | +154
.L036470:
        cmpi.b  #0x4,d1                         | +158
        bne.w   .L036482                        | +15c
        lea     Player_CrouchShoot_03873c(pc),a1             | +160
        move.l  a1,(a6)                         | +164
        bra.w   .L03649e                        | +166
.L036482:
        cmpi.b  #0x1,d1                         | +16a
        bne.w   .L036494                        | +16e
        lea     Player_WalkLoopRight_ShootUp_0357be(pc),a1 | +172
        move.l  a1,(a6)                         | +176
        bra.w   .L03649e                        | +178
.L036494:
        lea     Player_WalkLoopRight_Shoot_0357aa(pc),a1 | +17c
        move.l  a1,(a6)                         | +180
        bra.w   .L03649e                        | +182
.L03649e:
        jmp     Player_Walk_Tail_03548c(pc) | +186

| ----------------------------------------------------------------------------
|  Player_RideSlug_0364a2  @ $0364A2  (400 B)
| ----------------------------------------------------------------------------
        .section .text.Player_RideSlug_0364a2, "ax", @progbits
        .global Player_RideSlug_0364a2
Player_RideSlug_0364a2:
        bclr    #0x2,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        bclr    #0x3,0x8c(a6)                   | +00c
        bclr    #0x0,0x3a(a6)                   | +012
        lea     Player_HitboxDeath_0328d8(pc),a0             | +018
        move.l  a0,0x48(a6)                     | +01c
        move.w  #0x0,0x7c(a6)                   | +020
        move.w  #0x10,0x7e(a6)                  | +026
        move.b  #0x2,0x70(a6)                   | +02c
        lea     0x279f08.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x279f08.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        clr.w   0x28(a6)                        | +050
        clr.w   0x2c(a6)                        | +054
        clr.w   0x2a(a6)                        | +058
        clr.w   0x2e(a6)                        | +05c
        lea     .L036508(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L036508:
        jsr     Player_FrameCommon_NoPrio_033016(pc) | +066
        jsr     0x2783a.l                       | +06a
        lea     0x100580.l,a0                   | +070
        move.w  0x22(a0),d0                     | +076
        move.w  0x24(a0),d1                     | +07a
        addq.w  #0x1,d1                         | +07e
        move.w  d0,0x22(a6)                     | +080
        move.w  d1,0x24(a6)                     | +084
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +088
        .global Player_RideSlug_Frame_03652e
Player_RideSlug_Frame_03652e:               | $03652E entrada secundaria (desde $0366xx)
        jsr     0x2aca2.l                       | +08c
        lea     0x100580.l,a0                   | +092
        move.b  0x68(a6),d2                     | +098
        move.b  d2,0x68(a0)                     | +09c
        jsr     0x2abf0.l                       | +0a0
        bcs.w   .L0365bc                        | +0a6
        jsr     0x5cef8.l                       | +0aa
        bcs.w   .L03656a                        | +0b0
        lea     0x100580.l,a0                   | +0b4
        btst    #0x0,0x13(a0)                   | +0ba
        bne.w   .L03656a                        | +0c0
        bra.w   .L0365bc                        | +0c4
.L03656a:
        btst    #0x2,0x8c(a6)                   | +0c8
        bne.w   .L036598                        | +0ce
        jsr     0x5cdc0.l                       | +0d2
        bcc.w   .L036598                        | +0d8
        jsr     0x2abd2.l                       | +0dc
        bcc.w   .L036598                        | +0e2
        btst    #0x0,0x13(a0)                   | +0e6
        bne.w   .L036598                        | +0ec
        lea     Player_RideSlug_Pose2_0366fe(pc),a1             | +0f0
        move.l  a1,(a6)                         | +0f4
.L036598:
        jsr     0x5cdb4.l                       | +0f6
        bcc.w   .L0365bc                        | +0fc
        jsr     0x2abd2.l                       | +100
        bcc.w   .L0365bc                        | +106
        lea     Player_SlugJumpOff_036796(pc),a1             | +10a
        move.l  a1,(a6)                         | +10e
        addi.w  #0x20,0x24(a6)                  | +110
        bra.w   .L0365f8                        | +116
.L0365bc:
        jsr     0x2abf0.l                       | +11a
        bcs.w   .L0365e8                        | +120
        jsr     0x5d00e.l                       | +124
        bcc.w   .L0365e8                        | +12a
        lea     Player_SlugJumpOff_036796(pc),a1             | +12e
        move.l  a1,(a6)                         | +132
        addi.w  #0x20,0x24(a6)                  | +134
        move.b  #0x3c,0x45(a6)                  | +13a
        move.b  #0x3c,0x59(a6)                  | +140
.L0365e8:
        jsr     0x2a25c.l                       | +146
        bcc.w   .L0365f8                        | +14c
        lea     Player_SlugJumpOff_036796(pc),a1             | +150
        move.l  a1,(a6)                         | +154
.L0365f8:
        cmpi.b  #0x3,0x106ece.l                 | +156
        beq.w   .L036618                        | +15e
        movea.l #0xffffffff,a0                  | +162
        lea     Player_GroundTblB_0324c6(pc),a0             | +168
        jsr     0x5dd56.l                       | +16c
        bra.w   .L036628                        | +172
.L036618:
        movea.l #0xffffffff,a0                  | +176
        lea     Player_GroundTblA_0324bc(pc),a0             | +17c
        jsr     0x5dd56.l                       | +180
.L036628:
        bcc.w   Player_RideSlug_Tail_036632     | +186
        lea     Player_DeathPit_037b8e(pc),a1       | +18a
        move.l  a1,(a6)                         | +18e
