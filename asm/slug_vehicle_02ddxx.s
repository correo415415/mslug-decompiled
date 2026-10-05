| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ZZZ — vehículo Metal Slug SV-001: estados, daño, destrucción, Chain3
|  Región: $02DD20..$030602  (10,258 B, 63 entradas, 28 huecos)
| ============================================================================
|
|  A. RESUMEN
|  ----------
|  Región de 10,258 B: máquina de estados del vehículo SV-001 "Metal Slug"
|  (entidad en slot $100580) y las entidades "Chain3" (cadena circular de
|  tres eslabones) que le siguen. Es la segunda mitad del módulo del Slug;
|  los helpers que usa viven justo antes ($029xxx..$02DD20, aún sin wave).
|
|   1. Estados del Slug (handlers en (a6), todos del mismo patrón: ángulo
|      del terreno vía Sub_0002A958 -> +$80, índice sprite por ángulo
|      (Slug_AngleIndex_02ff8e / Sub_000295A6) sobre la tabla $2B0DC8 o
|      $2B0C30, anim por arma/dirección vía Sub_0002A9A0 sobre tablas
|      $2793xx/$2794xx/$2796xx, callback de colisión Sub_0002999E y tabla
|      de ataque Sub_0002A024):
|        Slug_IdleEnterA/B/C_02dd20/74/c8, Slug_Idle_02de18,
|        Slug_IdleAngled_02deb4/_B_02e244 (parado en pendiente),
|        Slug_Jump_02e092 / Slug_JumpB_02e04a (salto, snd $196..$199),
|        Slug_Hunker_02e3b0 (agacharse), Slug_PlayerMount_02e496
|        (recoge al player: recorre $100440/$1004E0 y lo engancha),
|        Slug_Drive_02e6c2/_B_02e7fa, Slug_TurnToDrive_02e924,
|        Slug_DriveAlt_02e9c8, Slug_TurnToDriveAlt_02eb14, Slug_Brake_02ebb0,
|        Slug_Stall_02eca6, Slug_AccelRight/Left_02ee78/02efaa,
|        Slug_Knocked_02f0dc (empujón por explosión), Slug_DecelA/B_02f202/
|        02f312, Slug_CruiseRight/Left_02f422/02f4fc, Slug_SetSpeed_02f5d6.
|      Slug_StatePtrTbl_02e582: 80 punteros a estos handlers, indexada
|      desde el despachador $2A078..$2A09C (helper previo) por
|      (modo +$72, arma, dirección).
|   2. Daño acumulado: Slug_DamagePtrTbl_02fac8 -> Slug_DamageData_02faca,
|      Slug_DamageSpriteTblA/B/C_02f886/02f8ae/02fa3c (registros de hitbox
|      y sprites de abolladuras), Slug_ResetDamageIdx_02fada (destino del
|      thunk JsrPcThunk_02ff1c) y Slug_UpdateDamageSprite_02fae4 (elige
|      sprite por HP +$66). Slug_WheelAnim_02fb92 (+_Entry_02fc6a) anima
|      las orugas con velocidad +$28.
|   3. Destrucción: Slug_Destroyed_02fc70 (música $10E9 -> $10AF, +$92 =
|      $30 frames, +$36 = $600 de impulso, hitbox $295B4/$2964C, bset #2,
|      +$8D; en escena $106F2A == 3 corrige la velocidad con el ángulo),
|      Slug_KillInit_02feda, Slug_SelfDestructAttack_02fe6a y
|      Slug_BlastAttack_02ff4a (ataques $283CA/$283D8 con la tabla de
|      explosión), Slug_ExplodeFx_02ff22 (snd $19A..$19E), SlugFx_Smoke_02f64c,
|      SlugFx_Fall_02f692 y la tabla de 28 templates
|      SlugFx_ExplosionAnim_02f6c0 (anims $2A0xxx..).
|   4. Pequeños helpers: Slug_TypeIfAir_02f84a (ref $28F3A), Players_AnyFlag8D3
|      _02f85a (+_SetC): ¿algún player con +$8D bit3? (ref $1B94),
|      Slug_ClampField92_02ffb0 (+_ClearXN), Entity_CmpField10WithLink8_02ffe6
|      (+_SetXN; ref $18A07E; gemelo de la versión $039234).
|   5. Chain3_*: Chain3_Init_030002 crea 3 entidades enlazadas circularmente
|      (Entity_Build3ChainCircular_03060A en la región siguiente) con los
|      templates Chain3_TplA/B/C_030068/0300ba/03010c (anims $279668/
|      $27966E/$274674); Chain3_Follow_0301f0 sigue al padre (+$C) usando
|      Chain3_YDelta_030392, Chain3_Step_0303ee, Chain3_VelY_030416,
|      Chain3_VelX_030462 y Chain3_CheckSyncA/B_0304c4/03050c; Chain3_DebugHud
|      _030554 pinta ids $7412/$7413/$7415 con $5D6C2 si $100001 bit4.
|      Refs: $2A768..$2A826 (helper previo), $38FBA, $74874, $77390.
|
|  B. EVIDENCIAS
|  -------------
|   - Slot $100580 (lea directo en Slug_PlayerMount) = entidad del Slug en
|     el resto del proyecto; $100440/$1004E0 = players.
|   - Tablas de sprites $2B0DC8/$2B0C30 indexadas por ángulo (0..$40) son
|     las del casco del Slug; $2793xx..$2796xx = anims del Slug por arma.
|   - Música $10E9/$10AF/$10B0/$1034 y snd $196..$19E, $163 sólo aparecen
|     en el módulo del Slug (explosión/motor).
|   - Slug_StatePtrTbl: los 80 punteros caen todos en esta región y la
|     única referencia es el despachador $2A078.
|
|  C. CAMPOS (a6 = Slug)
|  ---------------------
|   +$00 handler, +$0C player montado, +$20 frame sprite, +$28/+$2A vel,
|   +$2C, +$36 impulso, +$48 cb colisión, +$60 hitbox, +$66 HP, +$72 modo,
|   +$80 ángulo terreno, +$8C bit5 (en el aire), +$8D bit2 (destruido) /
|   bit3 (player montado), +$92 temporizador, +$9x índice de daño.
|
|  D. HELPERS EXTERNOS
|  -------------------
|   $236E snd, $2352 music, $283CA/$283D8 ataque, $27EBA, $28992, $28CD4
|   sprite, $28D70 paso anim, $4AE alloc, $5CEF8 input, $5DD56 suelo,
|   $5D6C2 debug print; helpers del módulo Slug aún sin nombre:
|   Sub_000295A6 (sprite por ángulo), Sub_0002A328/34E/478/4EC/4F0/664/
|   690/766/7D8/824/8C0/958 (física del Slug), Sub_0002A9A0 (índice anim
|   por arma/dir), Sub_0002AA24, JsrAbsThunk_02a5cc, PcThunkTarget_02ac80.
|
|  E. HIPÓTESIS ABIERTAS
|  ---------------------
|   - Los nombres de estado (Drive/Cruise/Accel/Decel/Brake/Stall/Hunker)
|     se deducen de qué campos de velocidad tocan y de la tabla de 80
|     punteros; confirmar con el despachador $2A078 al hacer su wave.
|   - "Chain3" es provisional: podría ser la cadena/antena del Slug o el
|     humo en tres segmentos; depende de los templates $279668..
|   - Slug_DamageSpriteTbl* se trata como datos porque $02F886+0A no
|     decodifica como código; el formato exacto de registro está abierto.
|
|  F. SIGUIENTE
|  ------------
|   `$030602..$032A02` (segunda mitad del player core), después los helpers
|   del Slug `$029xxx..$02DD20` (Sub_0002A9A0 y compañía).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Slug_IdleEnterA_02dd20  @ $02DD20  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleEnterA_02dd20, "ax", @progbits
        .global Slug_IdleEnterA_02dd20
Slug_IdleEnterA_02dd20:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Sub_0002A9A0(pc)                | +006
        movea.l #0x279614,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02dd46                        | +01c
        jsr     0x28cd4.l                       | +020
.L02dd46:
        move.b  #0x41,0x20(a6)                  | +026
        andi.w  #0xff,d0                        | +02c
        lsr.w   #0x2,d0                         | +030
        add.b   d0,0x20(a6)                     | +032
        clr.w   0x2c(a6)                        | +036
        move.l  #0xffffffff,0x60(a6)            | +03a
        lea     Sub_0002999E(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        lea     Slug_Idle_02de18(pc),a1         | +04a
        move.l  a1,(a6)                         | +04e
        jmp     Slug_Idle_02de18(pc)            | +050

| ----------------------------------------------------------------------------
|  Slug_IdleEnterB_02dd74  @ $02DD74  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleEnterB_02dd74, "ax", @progbits
        .global Slug_IdleEnterB_02dd74
Slug_IdleEnterB_02dd74:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Sub_0002A9A0(pc)                | +006
        movea.l #0x279628,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02dd9a                        | +01c
        jsr     0x28cd4.l                       | +020
.L02dd9a:
        move.b  #0x41,0x20(a6)                  | +026
        andi.w  #0xff,d0                        | +02c
        lsr.w   #0x2,d0                         | +030
        add.b   d0,0x20(a6)                     | +032
        clr.w   0x2c(a6)                        | +036
        move.l  #0xffffffff,0x60(a6)            | +03a
        lea     Sub_0002999E(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        lea     Slug_Idle_02de18(pc),a1         | +04a
        move.l  a1,(a6)                         | +04e
        bra.w   Slug_Idle_02de18                | +050

| ----------------------------------------------------------------------------
|  Slug_IdleEnterC_02ddc8  @ $02DDC8  (80 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleEnterC_02ddc8, "ax", @progbits
        .global Slug_IdleEnterC_02ddc8
Slug_IdleEnterC_02ddc8:
        bclr    #0x5,0x8c(a6)                   | +000
        jsr     Sub_0002A9A0(pc)                | +006
        movea.l #0x279614,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L02ddee                        | +01c
        jsr     0x28cd4.l                       | +020
.L02ddee:
        move.b  #0x41,0x20(a6)                  | +026
        andi.w  #0xff,d0                        | +02c
        lsr.w   #0x2,d0                         | +030
        add.b   d0,0x20(a6)                     | +032
        clr.w   0x2c(a6)                        | +036
        move.l  #0xffffffff,0x60(a6)            | +03a
        lea     Sub_0002999E(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        lea     Slug_Idle_02de18(pc),a1         | +04a
        move.l  a1,(a6)                         | +04e

| ----------------------------------------------------------------------------
|  Slug_Idle_02de18  @ $02DE18  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Idle_02de18, "ax", @progbits
        .global Slug_Idle_02de18
Slug_Idle_02de18:
        jsr     Sub_0002AA24(pc)                | +000
        bset    #0x4,0x8d(a6)                   | +004
        bset    #0x5,0x8d(a6)                   | +00a
        clr.w   0x28(a6)                        | +010
        jsr     Sub_0002A824(pc)                | +014
        bcc.w   .L02de3a                        | +018
        lea     Slug_IdleAngled_02deb4(pc),a1   | +01c
        move.l  a1,(a6)                         | +020
.L02de3a:
        jsr     Sub_0002A478(pc)                | +022
        jsr     0x28d70.l                       | +026
        jsr     Sub_0002A8C0(pc)                | +02c
        bcc.w   .L02de52                        | +030
        lea     Slug_Jump_02e092(pc),a1         | +034
        move.l  a1,(a6)                         | +038
.L02de52:
        jsr     Sub_0002ACB8(pc)                | +03a
        bcc.w   .L02de60                        | +03e
        lea     Slug_Hunker_02e3b0(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L02de60:
        jsr     Sub_0002A664(pc)                | +048
        bcc.w   .L02de6e                        | +04c
        lea     TaskHandler_02da38(pc),a1       | +050
        move.l  a1,(a6)                         | +054
.L02de6e:
        cmpi.b  #0x0,0x106f4a.l                 | +056
        beq.w   .L02de98                        | +05e
        movea.l #0xffffffff,a0                  | +062
        lea     0x27964e.l,a0                   | +068
        jsr     0x5dd56.l                       | +06e
        bcc.w   .L02de94                        | +074
        jmp     Sub_0002DCBC(pc)                | +078
.L02de94:
        bra.w   .L02deb2                        | +07c
.L02de98:
        movea.l #0xffffffff,a0                  | +080
        lea     0x27965a.l,a0                   | +086
        jsr     0x5dd56.l                       | +08c
        bcc.w   .L02deb2                        | +092
        jmp     Sub_0002DCBC(pc)                | +096
.L02deb2:
        rts                                     | +09a

| ----------------------------------------------------------------------------
|  Slug_IdleAngled_02deb4  @ $02DEB4  (406 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleAngled_02deb4, "ax", @progbits
        .global Slug_IdleAngled_02deb4
Slug_IdleAngled_02deb4:
        clr.w   0x2c(a6)                        | +000
        jsr     Sub_0002A958(pc)                | +004
        move.w  d2,0x80(a6)                     | +008
        move.w  d2,d0                           | +00c
        asr.w   #0x1,d0                         | +00e
        neg.w   d0                              | +010
        addi.w  #0x20,d0                        | +012
        cmpi.w  #0x40,d0                        | +016
        bls.w   .L02ded6                        | +01a
        move.w  #0x40,d0                        | +01e
.L02ded6:
        movem.w d0,-(a7)                        | +022
        jsr     Sub_000295A6(pc)                | +026
        movea.l #0x2b0dc8,a0                    | +02a
        lsl.w   #0x2,d0                         | +030
        movea.l (a0,d0.w),a0                    | +032
        cmpa.l  #0xffffffff,a0                  | +036
        beq.w   .L02defa                        | +03c
        jsr     0x28cd4.l                       | +040
.L02defa:
        movem.w (a7)+,d0                        | +046
        move.w  #0x0,d0                         | +04a
        move.b  #0xc3,0x20(a6)                  | +04e
        move.l  #0xffffffff,0x60(a6)            | +054
        lea     Sub_0002999E(pc),a0             | +05c
        move.l  a0,0x48(a6)                     | +060
        lea     .L02df1e(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L02df1e:
        bset    #0x2,0x8d(a6)                   | +06a
        bset    #0x4,0x8d(a6)                   | +070
        bset    #0x5,0x8d(a6)                   | +076
        jsr     Sub_0002AA24(pc)                | +07c
        clr.w   0x28(a6)                        | +080
        cmpi.b  #0x3,0x106f2a.l                 | +084
        bne.w   .L02df62                        | +08c
        move.w  0x28(a6),d7                     | +090
        jsr     Sub_0002A958(pc)                | +094
        sub.w   0x80(a6),d2                     | +098
        eor.w   d7,d2                           | +09c
        asl.w   #0x1,d2                         | +09e
        bcs.w   .L02df5e                        | +0a0
        move.w  d7,d2                           | +0a4
        asr.w   #0x1,d2                         | +0a6
        add.w   d2,d7                           | +0a8
.L02df5e:
        move.w  d7,0x28(a6)                     | +0aa
.L02df62:
        jsr     Sub_0002A478(pc)                | +0ae
        jsr     Sub_0002A766(pc)                | +0b2
        jsr     Sub_0002A958(pc)                | +0b6
        cmp.w   0x80(a6),d2                     | +0ba
        beq.w   .L02dfc2                        | +0be
        move.w  d2,0x80(a6)                     | +0c2
        move.w  d2,d0                           | +0c6
        asr.w   #0x1,d0                         | +0c8
        neg.w   d0                              | +0ca
        addi.w  #0x20,d0                        | +0cc
        cmpi.w  #0x40,d0                        | +0d0
        bls.w   .L02df90                        | +0d4
        move.w  #0x40,d0                        | +0d8
.L02df90:
        movem.w d0,-(a7)                        | +0dc
        jsr     Sub_000295A6(pc)                | +0e0
        movea.l #0x2b0dc8,a0                    | +0e4
        lsl.w   #0x2,d0                         | +0ea
        movea.l (a0,d0.w),a0                    | +0ec
        cmpa.l  #0xffffffff,a0                  | +0f0
        beq.w   .L02dfb4                        | +0f6
        jsr     0x28cd4.l                       | +0fa
.L02dfb4:
        movem.w (a7)+,d0                        | +100
        move.w  #0x0,d0                         | +104
        move.b  #0xc3,0x20(a6)                  | +108
.L02dfc2:
        jsr     0x28d70.l                       | +10e
        jsr     Sub_0002A4EC(pc)                | +114
        bcs.w   .L02dfda                        | +118
        bra.w   .L02dfd4                        | +11c
.L02dfd4:
        lea     Slug_IdleEnterC_02ddc8(pc),a1   | +120
        move.l  a1,(a6)                         | +124
.L02dfda:
        jsr     Sub_0002A8C0(pc)                | +126
        bcc.w   .L02dfe8                        | +12a
        lea     Slug_JumpB_02e04a(pc),a1        | +12e
        move.l  a1,(a6)                         | +132
.L02dfe8:
        jsr     Sub_0002ACB8(pc)                | +134
        bcc.w   .L02dff6                        | +138
        lea     Slug_PlayerMount_02e496(pc),a1  | +13c
        move.l  a1,(a6)                         | +140
.L02dff6:
        jsr     Sub_0002A664(pc)                | +142
        bcc.w   .L02e004                        | +146
        lea     TaskHandler_02da38(pc),a1       | +14a
        move.l  a1,(a6)                         | +14e
.L02e004:
        cmpi.b  #0x0,0x106f4a.l                 | +150
        beq.w   .L02e02e                        | +158
        movea.l #0xffffffff,a0                  | +15c
        lea     0x27964e.l,a0                   | +162
        jsr     0x5dd56.l                       | +168
        bcc.w   .L02e02a                        | +16e
        jmp     Sub_0002DCBC(pc)                | +172
.L02e02a:
        bra.w   .L02e048                        | +176
.L02e02e:
        movea.l #0xffffffff,a0                  | +17a
        lea     0x27965a.l,a0                   | +180
        jsr     0x5dd56.l                       | +186
        bcc.w   .L02e048                        | +18c
        jmp     Sub_0002DCBC(pc)                | +190
.L02e048:
        rts                                     | +194

| ----------------------------------------------------------------------------
|  Slug_JumpB_02e04a  @ $02E04A  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_JumpB_02e04a, "ax", @progbits
        .global Slug_JumpB_02e04a
Slug_JumpB_02e04a:
        bclr    #0x5,0x8c(a6)                   | +000
        move.w  #0xffe0,0x2e(a6)                | +006
        clr.w   0x28(a6)                        | +00c
        clr.w   0x2c(a6)                        | +010
        addq.w  #0x4,0x82(a6)                   | +014
        move.w  0x24(a6),d0                     | +018
        cmp.w   0x82(a6),d0                     | +01c
        ble.w   .L02e072                        | +020
        move.w  d0,0x82(a6)                     | +024
.L02e072:
        move.l  #0xffffffff,0x60(a6)            | +028
        lea     Sub_0002999E(pc),a0             | +030
        move.l  a0,0x48(a6)                     | +034
        lea     .L02e088(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L02e088:
        bset    #0x2,0x8d(a6)                   | +03e
        jmp     Slug_Jump_02e092__L02e12c(pc)   | +044

| ----------------------------------------------------------------------------
|  Slug_Jump_02e092  @ $02E092  (434 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Jump_02e092, "ax", @progbits
        .global Slug_Jump_02e092
Slug_Jump_02e092:
        bclr    #0x5,0x8c(a6)                   | +000
        move.w  #0xffe0,0x2e(a6)                | +006
        clr.w   0x28(a6)                        | +00c
        clr.w   0x2c(a6)                        | +010
        addq.w  #0x4,0x82(a6)                   | +014
        move.w  0x24(a6),d0                     | +018
        cmp.w   0x82(a6),d0                     | +01c
        ble.w   .L02e0ba                        | +020
        move.w  d0,0x82(a6)                     | +024
.L02e0ba:
        jsr     Sub_0002A958(pc)                | +028
        move.w  d2,0x80(a6)                     | +02c
        move.w  d2,d0                           | +030
        asr.w   #0x1,d0                         | +032
        neg.w   d0                              | +034
        addi.w  #0x20,d0                        | +036
        cmpi.w  #0x40,d0                        | +03a
        bls.w   .L02e0d8                        | +03e
        move.w  #0x40,d0                        | +042
.L02e0d8:
        movem.w d0,-(a7)                        | +046
        jsr     Sub_000295A6(pc)                | +04a
        movea.l #0x2b0dc8,a0                    | +04e
        lsl.w   #0x2,d0                         | +054
        movea.l (a0,d0.w),a0                    | +056
        cmpa.l  #0xffffffff,a0                  | +05a
        beq.w   .L02e0fc                        | +060
        jsr     0x28cd4.l                       | +064
.L02e0fc:
        movem.w (a7)+,d0                        | +06a
        move.b  #0x0,0x20(a6)                   | +06e
        lsr.w   #0x2,d0                         | +074
        andi.w  #0xff,d0                        | +076
        add.b   d0,0x20(a6)                     | +07a
        bset    #0x2,0x8d(a6)                   | +07e
        move.l  #0xffffffff,0x60(a6)            | +084
        lea     Sub_0002999E(pc),a0             | +08c
        move.l  a0,0x48(a6)                     | +090
        lea     Slug_Jump_02e092__L02e12c(pc),a1 | +094
        move.l  a1,(a6)                         | +098
        .global Slug_Jump_02e092__L02e12c
Slug_Jump_02e092__L02e12c:
        move.w  0x2a(a6),d0                     | +09a
        move.w  #0x400,d1                       | +09e
        jsr     0x267f4.l                       | +0a2
        move.w  d0,0x2a(a6)                     | +0a8
        bset    #0x1,0x8d(a6)                   | +0ac
        bset    #0x5,0x8d(a6)                   | +0b2
        jsr     Sub_0002AA0E(pc)                | +0b8
        jsr     Sub_0002A878(pc)                | +0bc
        bcc.w   .L02e15c                        | +0c0
        lea     Slug_IdleEnterC_02ddc8(pc),a1   | +0c4
        move.l  a1,(a6)                         | +0c8
.L02e15c:
        jsr     Sub_0002A478(pc)                | +0ca
        jsr     0x28d70.l                       | +0ce
        tst.w   0x2a(a6)                        | +0d4
        bgt.w   .L02e1e6                        | +0d8
        jsr     Sub_0002A8C0(pc)                | +0dc
        cmpi.b  #0x0,d1                         | +0e0
        ble.w   .L02e182                        | +0e4
        clr.w   0x28(a6)                        | +0e8
        clr.w   0x2c(a6)                        | +0ec
.L02e182:
        jsr     Sub_0002A958(pc)                | +0f0
        cmp.w   0x80(a6),d2                     | +0f4
        beq.w   .L02e1e6                        | +0f8
        move.w  d2,0x80(a6)                     | +0fc
        move.w  d2,d0                           | +100
        asr.w   #0x1,d0                         | +102
        neg.w   d0                              | +104
        addi.w  #0x20,d0                        | +106
        cmpi.w  #0x40,d0                        | +10a
        bls.w   .L02e1a8                        | +10e
        move.w  #0x40,d0                        | +112
.L02e1a8:
        movem.w d0,-(a7)                        | +116
        jsr     Sub_000295A6(pc)                | +11a
        movea.l #0x2b0dc8,a0                    | +11e
        lsl.w   #0x2,d0                         | +124
        movea.l (a0,d0.w),a0                    | +126
        cmpa.l  #0xffffffff,a0                  | +12a
        beq.w   .L02e1cc                        | +130
        jsr     0x28cd4.l                       | +134
.L02e1cc:
        movem.w (a7)+,d0                        | +13a
        move.b  #0x0,0x20(a6)                   | +13e
        lsr.w   #0x2,d0                         | +144
        andi.w  #0xff,d0                        | +146
        add.b   d0,0x20(a6)                     | +14a
        bset    #0x2,0x8d(a6)                   | +14e
.L02e1e6:
        btst    #0x1,0x8d(a6)                   | +154
        bne.w   .L02e1fe                        | +15a
        jsr     Sub_0002A664(pc)                | +15e
        bcc.w   .L02e1fe                        | +162
        lea     TaskHandler_02da38(pc),a1       | +166
        move.l  a1,(a6)                         | +16a
.L02e1fe:
        cmpi.b  #0x0,0x106f4a.l                 | +16c
        beq.w   .L02e228                        | +174
        movea.l #0xffffffff,a0                  | +178
        lea     0x27964e.l,a0                   | +17e
        jsr     0x5dd56.l                       | +184
        bcc.w   .L02e224                        | +18a
        jmp     Sub_0002DCBC(pc)                | +18e
.L02e224:
        bra.w   .L02e242                        | +192
.L02e228:
        movea.l #0xffffffff,a0                  | +196
        lea     0x27965a.l,a0                   | +19c
        jsr     0x5dd56.l                       | +1a2
        bcc.w   .L02e242                        | +1a8
        jmp     Sub_0002DCBC(pc)                | +1ac
.L02e242:
        rts                                     | +1b0

| ----------------------------------------------------------------------------
|  Slug_IdleAngledB_02e244  @ $02E244  (356 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_IdleAngledB_02e244, "ax", @progbits
        .global Slug_IdleAngledB_02e244
Slug_IdleAngledB_02e244:
        clr.w   0x2c(a6)                        | +000
        jsr     Sub_0002A958(pc)                | +004
        move.w  d2,0x80(a6)                     | +008
        move.w  d2,d0                           | +00c
        asr.w   #0x1,d0                         | +00e
        neg.w   d0                              | +010
        addi.w  #0x20,d0                        | +012
        cmpi.w  #0x40,d0                        | +016
        bls.w   .L02e266                        | +01a
        move.w  #0x40,d0                        | +01e
.L02e266:
        movem.w d0,-(a7)                        | +022
        jsr     Sub_000295A6(pc)                | +026
        movea.l #0x2b0dc8,a0                    | +02a
        lsl.w   #0x2,d0                         | +030
        movea.l (a0,d0.w),a0                    | +032
        cmpa.l  #0xffffffff,a0                  | +036
        beq.w   .L02e28a                        | +03c
        jsr     0x28cd4.l                       | +040
.L02e28a:
        movem.w (a7)+,d0                        | +046
        move.w  #0x0,d0                         | +04a
        move.b  #0xc3,0x20(a6)                  | +04e
        move.l  #0xffffffff,0x60(a6)            | +054
        lea     Sub_0002999E(pc),a0             | +05c
        move.l  a0,0x48(a6)                     | +060
        lea     .L02e2ae(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L02e2ae:
        bset    #0x2,0x8d(a6)                   | +06a
        bset    #0x4,0x8d(a6)                   | +070
        bset    #0x5,0x8d(a6)                   | +076
        jsr     Sub_0002AA24(pc)                | +07c
        clr.w   0x28(a6)                        | +080
        cmpi.b  #0x3,0x106f2a.l                 | +084
        bne.w   .L02e2f2                        | +08c
        move.w  0x28(a6),d7                     | +090
        jsr     Sub_0002A958(pc)                | +094
        sub.w   0x80(a6),d2                     | +098
        eor.w   d7,d2                           | +09c
        asl.w   #0x1,d2                         | +09e
        bcs.w   .L02e2ee                        | +0a0
        move.w  d7,d2                           | +0a4
        asr.w   #0x1,d2                         | +0a6
        add.w   d2,d7                           | +0a8
.L02e2ee:
        move.w  d7,0x28(a6)                     | +0aa
.L02e2f2:
        jsr     Sub_0002A478(pc)                | +0ae
        jsr     0x28992.l                       | +0b2
        jsr     Sub_0002A766(pc)                | +0b8
        jsr     Sub_0002A958(pc)                | +0bc
        cmp.w   0x80(a6),d2                     | +0c0
        beq.w   .L02e358                        | +0c4
        move.w  d2,0x80(a6)                     | +0c8
        move.w  d2,d0                           | +0cc
        asr.w   #0x1,d0                         | +0ce
        neg.w   d0                              | +0d0
        addi.w  #0x20,d0                        | +0d2
        cmpi.w  #0x40,d0                        | +0d6
        bls.w   .L02e326                        | +0da
        move.w  #0x40,d0                        | +0de
.L02e326:
        movem.w d0,-(a7)                        | +0e2
        jsr     Sub_000295A6(pc)                | +0e6
        movea.l #0x2b0dc8,a0                    | +0ea
        lsl.w   #0x2,d0                         | +0f0
        movea.l (a0,d0.w),a0                    | +0f2
        cmpa.l  #0xffffffff,a0                  | +0f6
        beq.w   .L02e34a                        | +0fc
        jsr     0x28cd4.l                       | +100
.L02e34a:
        movem.w (a7)+,d0                        | +106
        move.w  #0x0,d0                         | +10a
        move.b  #0xc3,0x20(a6)                  | +10e
.L02e358:
        jsr     0x28d70.l                       | +114
        jsr     Sub_0002A4EC(pc)                | +11a
        bcs.w   .L02e370                        | +11e
        bra.w   .L02e36a                        | +122
.L02e36a:
        lea     Slug_IdleEnterC_02ddc8(pc),a1   | +126
        move.l  a1,(a6)                         | +12a
.L02e370:
        tst.b   0x3b(a6)                        | +12c
        bne.w   .L02e37e                        | +130
        jsr     0x283d8.l                       | +134
.L02e37e:
        jsr     Sub_0002A59A(pc)                | +13a
        bcs.w   .L02e38c                        | +13e
        lea     Slug_Hunker_02e3b0(pc),a1       | +142
        move.l  a1,(a6)                         | +146
.L02e38c:
        jsr     Sub_0002A8C0(pc)                | +148
        bcc.w   .L02e39a                        | +14c
        lea     Sub_0002CFFA(pc),a1             | +150
        move.l  a1,(a6)                         | +154
.L02e39a:
        jsr     Sub_0002A690(pc)                | +156
        lea     Sub_0002A060(pc),a0             | +15a
        movea.l #0xffffffff,a1                  | +15e

| ----------------------------------------------------------------------------
|  Slug_Hunker_02e3b0  @ $02E3B0  (222 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Hunker_02e3b0, "ax", @progbits
        .global Slug_Hunker_02e3b0
Slug_Hunker_02e3b0:
        bset    #0x0,0x6b(a6)                   | +000
        bclr    #0x5,0x8c(a6)                   | +006
        bclr    #0x1,0x8c(a6)                   | +00c
        jsr     Sub_0002A9A0(pc)                | +012
        movea.l #0x279600,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02e3e2                        | +028
        jsr     0x28cd4.l                       | +02c
.L02e3e2:
        lea     Sub_0002A024(pc),a1             | +032
        movea.l (a1,d0.w),a0                    | +036
        move.l  a0,0x48(a6)                     | +03a
        move.b  #0x28,0x20(a6)                  | +03e
        andi.w  #0xff,d0                        | +044
        lsr.w   #0x2,d0                         | +048
        add.b   d0,0x20(a6)                     | +04a
        lea     0x3d944.l,a1                    | +04e
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        move.l  #0xffffffff,0x60(a6)            | +060
        lea     Sub_0002999E(pc),a0             | +068
        move.l  a0,0x48(a6)                     | +06c
        clr.w   0x2c(a6)                        | +070
        lea     .L02e42a(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L02e42a:
        jsr     Sub_0002AA24(pc)                | +07a
        clr.w   0x28(a6)                        | +07e
        jsr     0x28992.l                       | +082
        jsr     Sub_0002A7D8(pc)                | +088
        clr.w   0x28(a6)                        | +08c
        jsr     Sub_0002A478(pc)                | +090
        jsr     0x28d70.l                       | +094
        bcc.w   .L02e464                        | +09a
        lea     Sub_00029A14(pc),a0             | +09e
        move.l  a0,0x48(a6)                     | +0a2
        move.l  #0x29600,0x60(a6)               | +0a6
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +0ae
        move.l  a1,(a6)                         | +0b2
.L02e464:
        jsr     Sub_0002A8C0(pc)                | +0b4
        bcc.w   .L02e472                        | +0b8
        lea     TaskHandler_02d02e(pc),a1       | +0bc
        move.l  a1,(a6)                         | +0c0
.L02e472:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0c2
        bcc.w   .L02e480                        | +0c6
        lea     Sub_0002BBF2(pc),a1             | +0ca
        move.l  a1,(a6)                         | +0ce
.L02e480:
        jsr     Sub_0002A690(pc)                | +0d0
        lea     Sub_0002A060(pc),a0             | +0d4
        movea.l #0xffffffff,a1                  | +0d8

| ----------------------------------------------------------------------------
|  Slug_PlayerMount_02e496  @ $02E496  (228 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_PlayerMount_02e496, "ax", @progbits
        .global Slug_PlayerMount_02e496
Slug_PlayerMount_02e496:
        bset    #0x0,0x6b(a6)                   | +000
        lea     0x3d944.l,a1                    | +006
        jsr     0x4ae.l                         | +00c
        jsr     0x5dd02.l                       | +012
        move.l  #0xffffffff,0x60(a6)            | +018
        bclr    #0x5,0x8c(a6)                   | +020
        clr.w   0x28(a6)                        | +026
        clr.w   0x2c(a6)                        | +02a
        jsr     Sub_0002A958(pc)                | +02e
        move.w  0x80(a6),d2                     | +032
        move.w  d2,d0                           | +036
        asr.w   #0x1,d0                         | +038
        neg.w   d0                              | +03a
        addi.w  #0x20,d0                        | +03c
        cmpi.w  #0x40,d0                        | +040
        bls.w   .L02e4e2                        | +044
        move.w  #0x40,d0                        | +048
.L02e4e2:
        movem.w d0,-(a7)                        | +04c
        jsr     Sub_000295A6(pc)                | +050
        movea.l #0x2b0d40,a0                    | +054
        lsl.w   #0x2,d0                         | +05a
        movea.l (a0,d0.w),a0                    | +05c
        cmpa.l  #0xffffffff,a0                  | +060
        beq.w   .L02e506                        | +066
        jsr     0x28cd4.l                       | +06a
.L02e506:
        movem.w (a7)+,d0                        | +070
        move.b  #0x82,0x20(a6)                  | +074
        lsr.w   #0x2,d0                         | +07a
        andi.w  #0xff,d0                        | +07c
        add.b   d0,0x20(a6)                     | +080
        move.b  #0xff,0x8b(a6)                  | +084
        lea     Sub_0002999E(pc),a0             | +08a
        move.l  a0,0x48(a6)                     | +08e
        lea     .L02e52e(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L02e52e:
        bset    #0x2,0x8d(a6)                   | +098
        jsr     Sub_0002AA24(pc)                | +09e
        jsr     Sub_0002A478(pc)                | +0a2
        jsr     Sub_0002A760(pc)                | +0a6
        jsr     0x28d70.l                       | +0aa
        bcc.w   .L02e550                        | +0b0
        lea     Sub_0002B7DA(pc),a1             | +0b4
        move.l  a1,(a6)                         | +0b8
.L02e550:
        jsr     Sub_0002A8C0(pc)                | +0ba
        bcc.w   .L02e55e                        | +0be
        lea     Sub_0002CFFA(pc),a1             | +0c2
        move.l  a1,(a6)                         | +0c6
.L02e55e:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0c8
        bcc.w   .L02e56c                        | +0cc
        lea     Slug_Destroyed_02fc70(pc),a1    | +0d0
        move.l  a1,(a6)                         | +0d4
.L02e56c:
        jsr     Sub_0002A690(pc)                | +0d6
        lea     Sub_0002A060(pc),a0             | +0da
        movea.l #0xffffffff,a1                  | +0de

| ----------------------------------------------------------------------------
|  Slug_StatePtrTbl_02e582  @ $02E582  (320 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_StatePtrTbl_02e582, "ax", @progbits
        .global Slug_StatePtrTbl_02e582
Slug_StatePtrTbl_02e582:
        .dc.w   0x002a                        | +000  (dato / opcode no decodificado)
        .dc.w   0x2850                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2826                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +008  (dato / opcode no decodificado)
        .dc.w   0x2808                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x2666                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +010  (dato / opcode no decodificado)
        .dc.w   0x264e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +014  (dato / opcode no decodificado)
        .dc.w   0x2636                        | +016  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +018  (dato / opcode no decodificado)
        .dc.w   0x261e                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x2606                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +020  (dato / opcode no decodificado)
        .dc.w   0x582c                        | +022  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +024  (dato / opcode no decodificado)
        .dc.w   0x5808                        | +026  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +028  (dato / opcode no decodificado)
        .dc.w   0x57d0                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x565a                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +030  (dato / opcode no decodificado)
        .dc.w   0x5636                        | +032  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +034  (dato / opcode no decodificado)
        .dc.w   0x561e                        | +036  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +038  (dato / opcode no decodificado)
        .dc.w   0x5606                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x55ee                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +040  (dato / opcode no decodificado)
        .dc.w   0x8468                        | +042  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +044  (dato / opcode no decodificado)
        .dc.w   0x8444                        | +046  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +048  (dato / opcode no decodificado)
        .dc.w   0x842c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x8296                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +050  (dato / opcode no decodificado)
        .dc.w   0x8272                        | +052  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +054  (dato / opcode no decodificado)
        .dc.w   0x825a                        | +056  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +058  (dato / opcode no decodificado)
        .dc.w   0x8242                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x822a                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +060  (dato / opcode no decodificado)
        .dc.w   0xd0ec                        | +062  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +064  (dato / opcode no decodificado)
        .dc.w   0xd0c8                        | +066  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +068  (dato / opcode no decodificado)
        .dc.w   0xd090                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +06c  (dato / opcode no decodificado)
        .dc.w   0xcf1a                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +070  (dato / opcode no decodificado)
        .dc.w   0xcef6                        | +072  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +074  (dato / opcode no decodificado)
        .dc.w   0xcede                        | +076  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +078  (dato / opcode no decodificado)
        .dc.w   0xcec6                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xceae                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +080  (dato / opcode no decodificado)
        .dc.w   0xfefa                        | +082  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +084  (dato / opcode no decodificado)
        .dc.w   0xfed6                        | +086  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +088  (dato / opcode no decodificado)
        .dc.w   0xfe9e                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +08c  (dato / opcode no decodificado)
        .dc.w   0xfd28                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +090  (dato / opcode no decodificado)
        .dc.w   0xfd04                        | +092  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +094  (dato / opcode no decodificado)
        .dc.w   0xfcec                        | +096  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +098  (dato / opcode no decodificado)
        .dc.w   0xfcd4                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xfcbc                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x39ee                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x39ee                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x39ee                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x39ee                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x39c4                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x39a6                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x397c                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x395e                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x67f6                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x67f6                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x67f6                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x67f6                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x67de                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x67c6                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x67ae                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x6796                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x943e                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x943e                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x943e                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x943e                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x9426                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x940e                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x93f6                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x93de                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +100  (dato / opcode no decodificado)
        .dc.w   0xc058                        | +102  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +104  (dato / opcode no decodificado)
        .dc.w   0xc058                        | +106  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +108  (dato / opcode no decodificado)
        .dc.w   0xc058                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +10c  (dato / opcode no decodificado)
        .dc.w   0xc058                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +110  (dato / opcode no decodificado)
        .dc.w   0xc040                        | +112  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +114  (dato / opcode no decodificado)
        .dc.w   0xc028                        | +116  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +118  (dato / opcode no decodificado)
        .dc.w   0xc010                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +11c  (dato / opcode no decodificado)
        .dc.w   0xbff8                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +120  (dato / opcode no decodificado)
        .dc.w   0xee66                        | +122  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +124  (dato / opcode no decodificado)
        .dc.w   0xee66                        | +126  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +128  (dato / opcode no decodificado)
        .dc.w   0xee66                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +12c  (dato / opcode no decodificado)
        .dc.w   0xee66                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +130  (dato / opcode no decodificado)
        .dc.w   0xee4e                        | +132  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +134  (dato / opcode no decodificado)
        .dc.w   0xee36                        | +136  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +138  (dato / opcode no decodificado)
        .dc.w   0xee1e                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x002a                        | +13c  (dato / opcode no decodificado)
        .dc.w   0xee06                        | +13e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_Drive_02e6c2  @ $02E6C2  (304 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Drive_02e6c2, "ax", @progbits
        .global Slug_Drive_02e6c2
Slug_Drive_02e6c2:
        bclr    #0x5,0x8c(a6)                   | +000
        bset    #0x1,0x8c(a6)                   | +006
        jsr     Sub_0002A9A0(pc)                | +00c
        movea.l #0x27940c,a0                    | +010
        lsl.w   #0x2,d0                         | +016
        movea.l (a0,d0.w),a0                    | +018
        cmpa.l  #0xffffffff,a0                  | +01c
        beq.w   .L02e6ee                        | +022
        jsr     0x28cd4.l                       | +026
.L02e6ee:
        lea     Sub_0002A024(pc),a1             | +02c
        movea.l (a1,d0.w),a0                    | +030
        move.l  a0,0x48(a6)                     | +034
        move.b  #0x3,0x8b(a6)                   | +038
        move.b  #0x23,0x20(a6)                  | +03e
        andi.w  #0xff,d0                        | +044
        lsr.w   #0x2,d0                         | +048
        add.b   d0,0x20(a6)                     | +04a
        .global Slug_Drive_02e6c2__L02e710
Slug_Drive_02e6c2__L02e710:
        move.l  #0x29600,0x60(a6)               | +04e
        clr.w   0x2c(a6)                        | +056
        lea     .L02e722(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L02e722:
        jsr     Sub_0002AA24(pc)                | +060
        bset    #0x4,0x8d(a6)                   | +064
        clr.w   0x28(a6)                        | +06a
        jsr     ClearXN_02abc0(pc)              | +06e
        bcs.w   .L02e754                        | +072
        jsr     Sub_0002A328(pc)                | +076
        bcc.w   .L02e746                        | +07a
        move.w  #0x2a0,0x28(a6)                 | +07e
.L02e746:
        jsr     Sub_0002A34E(pc)                | +084
        bcc.w   .L02e754                        | +088
        move.w  #0xfd60,0x28(a6)                | +08c
.L02e754:
        jsr     0x28992.l                       | +092
        jsr     Sub_0002A824(pc)                | +098
        bcc.w   .L02e768                        | +09c
        lea     Slug_Stall_02eca6__L02ecc2(pc),a1 | +0a0
        move.l  a1,(a6)                         | +0a4
.L02e768:
        clr.w   0x28(a6)                        | +0a6
        jsr     Sub_0002A478(pc)                | +0aa
        jsr     0x28d70.l                       | +0ae
        bcc.w   .L02e79c                        | +0b4
        lea     Slug_Brake_02ebb0(pc),a1        | +0b8
        move.l  a1,(a6)                         | +0bc
        jsr     Sub_0002A328(pc)                | +0be
        bcc.w   .L02e78e                        | +0c2
        lea     Slug_CruiseRight_02f422(pc),a1  | +0c6
        move.l  a1,(a6)                         | +0ca
.L02e78e:
        jsr     Sub_0002A34E(pc)                | +0cc
        bcc.w   .L02e79c                        | +0d0
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0d4
        move.l  a1,(a6)                         | +0d8
.L02e79c:
        jsr     0x5cef8.l                       | +0da
        bcs.w   .L02e7ac                        | +0e0
        lea     Slug_TurnToDriveAlt_02eb14(pc),a1 | +0e4
        move.l  a1,(a6)                         | +0e8
.L02e7ac:
        jsr     Sub_0002A8C0(pc)                | +0ea
        bcc.w   .L02e7ba                        | +0ee
        lea     TaskHandler_02d02e(pc),a1       | +0f2
        move.l  a1,(a6)                         | +0f6
.L02e7ba:
        jsr     Sub_0002AAF0(pc)                | +0f8
        bcc.w   .L02e7c8                        | +0fc
        lea     Slug_SetSpeed_02f5d6__L02f614(pc),a1 | +100
        move.l  a1,(a6)                         | +104
.L02e7c8:
        jsr     PcThunkTarget_02ac80(pc)        | +106
        bcc.w   .L02e7d6                        | +10a
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +10e
        move.l  a1,(a6)                         | +112
.L02e7d6:
        jsr     JsrAbsThunk_02a5cc(pc)          | +114
        bcc.w   .L02e7e4                        | +118
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +11c
        move.l  a1,(a6)                         | +120
.L02e7e4:
        jsr     Sub_0002A690(pc)                | +122
        lea     Sub_0002A060(pc),a0             | +126
        movea.l #0xffffffff,a1                  | +12a

| ----------------------------------------------------------------------------
|  Slug_DriveB_02e7fa  @ $02E7FA  (290 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DriveB_02e7fa, "ax", @progbits
        .global Slug_DriveB_02e7fa
Slug_DriveB_02e7fa:
        bclr    #0x5,0x8c(a6)                   | +000
        bset    #0x7,0x5b(a6)                   | +006
        lea     Sub_00029A68(pc),a0             | +00c
        move.l  a0,0x48(a6)                     | +010
        move.b  #0x3,0x8b(a6)                   | +014
        bset    #0x1,0x8c(a6)                   | +01a
        jsr     Sub_0002A9A0(pc)                | +020
        movea.l #0x279420,a0                    | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L02e83a                        | +036
        jsr     0x28cd4.l                       | +03a
.L02e83a:
        move.b  #0x41,0x20(a6)                  | +040
        andi.w  #0xff,d0                        | +046
        lsr.w   #0x2,d0                         | +04a
        add.b   d0,0x20(a6)                     | +04c
        move.l  #0x29600,0x60(a6)               | +050
        clr.w   0x2c(a6)                        | +058
        lea     .L02e85c(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L02e85c:
        jsr     Sub_0002AA24(pc)                | +062
        clr.w   0x28(a6)                        | +066
        bset    #0x4,0x8d(a6)                   | +06a
        jsr     ClearXN_02abc0(pc)              | +070
        bcs.w   .L02e88e                        | +074
        jsr     Sub_0002A328(pc)                | +078
        bcc.w   .L02e880                        | +07c
        move.w  #0x2a0,0x28(a6)                 | +080
.L02e880:
        jsr     Sub_0002A34E(pc)                | +086
        bcc.w   .L02e88e                        | +08a
        move.w  #0xfd60,0x28(a6)                | +08e
.L02e88e:
        jsr     0x28992.l                       | +094
        jsr     Sub_0002A824(pc)                | +09a
        bcc.w   .L02e8a2                        | +09e
        lea     Slug_Stall_02eca6__L02ecc2(pc),a1 | +0a2
        move.l  a1,(a6)                         | +0a6
.L02e8a2:
        clr.w   0x28(a6)                        | +0a8
        jsr     Sub_0002A478(pc)                | +0ac
        jsr     0x28d70.l                       | +0b0
        bcc.w   .L02e8d6                        | +0b6
        lea     Slug_Brake_02ebb0(pc),a1        | +0ba
        move.l  a1,(a6)                         | +0be
        jsr     Sub_0002A328(pc)                | +0c0
        bcc.w   .L02e8c8                        | +0c4
        lea     Slug_CruiseRight_02f422(pc),a1  | +0c8
        move.l  a1,(a6)                         | +0cc
.L02e8c8:
        jsr     Sub_0002A34E(pc)                | +0ce
        bcc.w   .L02e8d6                        | +0d2
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0d6
        move.l  a1,(a6)                         | +0da
.L02e8d6:
        jsr     Sub_0002A8C0(pc)                | +0dc
        bcc.w   .L02e8e4                        | +0e0
        lea     TaskHandler_02d02e(pc),a1       | +0e4
        move.l  a1,(a6)                         | +0e8
.L02e8e4:
        jsr     Sub_0002AB3C(pc)                | +0ea
        bcc.w   .L02e8f2                        | +0ee
        lea     Slug_SetSpeed_02f5d6__L02f614(pc),a1 | +0f2
        move.l  a1,(a6)                         | +0f6
.L02e8f2:
        jsr     PcThunkTarget_02ac80(pc)        | +0f8
        bcc.w   .L02e900                        | +0fc
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +100
        move.l  a1,(a6)                         | +104
.L02e900:
        jsr     JsrAbsThunk_02a5cc(pc)          | +106
        bcc.w   .L02e90e                        | +10a
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +10e
        move.l  a1,(a6)                         | +112
.L02e90e:
        jsr     Sub_0002A690(pc)                | +114
        lea     Sub_0002A060(pc),a0             | +118
        movea.l #0xffffffff,a1                  | +11c

| ----------------------------------------------------------------------------
|  Slug_TurnToDrive_02e924  @ $02E924  (164 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TurnToDrive_02e924, "ax", @progbits
        .global Slug_TurnToDrive_02e924
Slug_TurnToDrive_02e924:
        bclr    #0x5,0x8c(a6)                   | +000
        clr.b   0x8b(a6)                        | +006
        bclr    #0x1,0x8c(a6)                   | +00a
        moveq   #0,d0                           | +010
        moveq   #0,d1                           | +012
        move.b  0x3b(a6),d0                     | +014
        cmpi.l  #0x8,d0                         | +018
        bcc.w   Slug_Drive_02e6c2               | +01e
        movem.l d0,-(a7)                        | +022
        jsr     Sub_0002A9A0(pc)                | +026
        lea     0x2794d4.l,a0                   | +02a
        move.w  d0,d1                           | +030
        asl.w   #0x2,d0                         | +032
        movea.l (a0,d0.w),a0                    | +034
        movem.l (a7)+,d0                        | +038
        lsl.w   #0x2,d0                         | +03c
        movea.l (a0,d0.w),a0                    | +03e
        cmpa.l  #0xffffffff,a0                  | +042
        beq.w   .L02e976                        | +048
        jsr     0x28cd4.l                       | +04c
.L02e976:
        move.b  #0x46,0x20(a6)                  | +052
        lsr.w   #0x2,d0                         | +058
        andi.w  #0xff,d0                        | +05a
        cmpi.b  #0x8,d0                         | +05e
        bcs.w   .L02e996                        | +062
        nop                                     | +066
        nop                                     | +068
        cmpi.b  #0x8,d0                         | +06a
        nop                                     | +06e
        trap    #0xf                            | +070
.L02e996:
        mulu.w  #0x5,d0                         | +072
        add.b   d0,0x20(a6)                     | +076
        andi.l  #0xff,d1                        | +07a
        cmpi.b  #0x5,d1                         | +080
        bcs.w   .L02e9b8                        | +084
        nop                                     | +088
        nop                                     | +08a
        cmpi.b  #0x5,d1                         | +08c
        nop                                     | +090
        trap    #0xf                            | +092
.L02e9b8:
        add.b   d1,0x20(a6)                     | +094
        lea     Sub_00029A68(pc),a0             | +098
        move.l  a0,0x48(a6)                     | +09c
        bra.w   Slug_Drive_02e6c2__L02e710      | +0a0

| ----------------------------------------------------------------------------
|  Slug_DriveAlt_02e9c8  @ $02E9C8  (324 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DriveAlt_02e9c8, "ax", @progbits
        .global Slug_DriveAlt_02e9c8
Slug_DriveAlt_02e9c8:
        bset    #0x0,0x6b(a6)                   | +000
        bclr    #0x5,0x8c(a6)                   | +006
        bclr    #0x1,0x8c(a6)                   | +00c
        jsr     Sub_0002A9A0(pc)                | +012
        movea.l #0x279344,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02e9fa                        | +028
        jsr     0x28cd4.l                       | +02c
.L02e9fa:
        lea     Sub_0002A024(pc),a1             | +032
        movea.l (a1,d0.w),a0                    | +036
        move.l  a0,0x48(a6)                     | +03a
        move.b  #0x9,0x8b(a6)                   | +03e
        move.b  #0x28,0x20(a6)                  | +044
        andi.w  #0xff,d0                        | +04a
        lsr.w   #0x2,d0                         | +04e
        add.b   d0,0x20(a6)                     | +050
        .global Slug_DriveAlt_02e9c8__L02ea1c
Slug_DriveAlt_02e9c8__L02ea1c:
        clr.w   0x2c(a6)                        | +054
        lea     .L02ea26(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L02ea26:
        jsr     Sub_0002AA24(pc)                | +05e
        clr.w   0x28(a6)                        | +062
        jsr     ClearXN_02abc0(pc)              | +066
        bcs.w   .L02ea52                        | +06a
        jsr     Sub_0002A328(pc)                | +06e
        bcc.w   .L02ea44                        | +072
        move.w  #0x2a0,0x28(a6)                 | +076
.L02ea44:
        jsr     Sub_0002A34E(pc)                | +07c
        bcc.w   .L02ea52                        | +080
        move.w  #0xfd60,0x28(a6)                | +084
.L02ea52:
        jsr     0x28992.l                       | +08a
        jsr     Sub_0002A7D8(pc)                | +090
        bcc.w   .L02ea66                        | +094
        lea     Sub_0002B4D2(pc),a1             | +098
        move.l  a1,(a6)                         | +09c
.L02ea66:
        clr.w   0x28(a6)                        | +09e
        jsr     Sub_0002A478(pc)                | +0a2
        jsr     0x28d70.l                       | +0a6
        bcc.w   .L02eaaa                        | +0ac
        lea     Sub_00029A14(pc),a0             | +0b0
        move.l  a0,0x48(a6)                     | +0b4
        move.l  #0x29600,0x60(a6)               | +0b8
        lea     Sub_0002B38C(pc),a1             | +0c0
        move.l  a1,(a6)                         | +0c4
        jsr     Sub_0002A328(pc)                | +0c6
        bcc.w   .L02ea9c                        | +0ca
        lea     Sub_0002BF64(pc),a1             | +0ce
        move.l  a1,(a6)                         | +0d2
.L02ea9c:
        jsr     Sub_0002A34E(pc)                | +0d4
        bcc.w   .L02eaaa                        | +0d8
        lea     Sub_0002C07A(pc),a1             | +0dc
        move.l  a1,(a6)                         | +0e0
.L02eaaa:
        jsr     Sub_0002A59A(pc)                | +0e2
        bcc.w   .L02eab8                        | +0e6
        lea     Slug_TurnToDrive_02e924(pc),a1  | +0ea
        move.l  a1,(a6)                         | +0ee
.L02eab8:
        jsr     Sub_0002A8C0(pc)                | +0f0
        bcc.w   .L02eac6                        | +0f4
        lea     TaskHandler_02d02e(pc),a1       | +0f8
        move.l  a1,(a6)                         | +0fc
.L02eac6:
        jsr     Sub_0002AAF0(pc)                | +0fe
        bcc.w   .L02ead4                        | +102
        lea     Slug_SetSpeed_02f5d6__L02f614(pc),a1 | +106
        move.l  a1,(a6)                         | +10a
.L02ead4:
        jsr     Sub_0002AAC0(pc)                | +10c
        bcc.w   .L02eae2                        | +110
        lea     Sub_0002C24A(pc),a1             | +114
        move.l  a1,(a6)                         | +118
.L02eae2:
        jsr     PcThunkTarget_02ac80(pc)        | +11a
        bcc.w   .L02eaf0                        | +11e
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +122
        move.l  a1,(a6)                         | +126
.L02eaf0:
        jsr     JsrAbsThunk_02a5cc(pc)          | +128
        bcc.w   .L02eafe                        | +12c
        lea     Sub_0002BBF2(pc),a1             | +130
        move.l  a1,(a6)                         | +134
.L02eafe:
        jsr     Sub_0002A690(pc)                | +136
        lea     Sub_0002A060(pc),a0             | +13a
        movea.l #0xffffffff,a1                  | +13e

| ----------------------------------------------------------------------------
|  Slug_TurnToDriveAlt_02eb14  @ $02EB14  (156 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TurnToDriveAlt_02eb14, "ax", @progbits
        .global Slug_TurnToDriveAlt_02eb14
Slug_TurnToDriveAlt_02eb14:
        bclr    #0x5,0x8c(a6)                   | +000
        bclr    #0x1,0x8c(a6)                   | +006
        clr.b   0x8b(a6)                        | +00c
        moveq   #0,d0                           | +010
        moveq   #0,d1                           | +012
        move.b  0x3b(a6),d0                     | +014
        cmpi.l  #0x8,d0                         | +018
        bcc.w   Slug_DriveAlt_02e9c8            | +01e
        movem.l d0,-(a7)                        | +022
        jsr     Sub_0002A9A0(pc)                | +026
        lea     0x2794c0.l,a0                   | +02a
        move.w  d0,d1                           | +030
        asl.w   #0x2,d0                         | +032
        movea.l (a0,d0.w),a0                    | +034
        movem.l (a7)+,d0                        | +038
        lsl.w   #0x2,d0                         | +03c
        movea.l (a0,d0.w),a0                    | +03e
        cmpa.l  #0xffffffff,a0                  | +042
        beq.w   .L02eb66                        | +048
        jsr     0x28cd4.l                       | +04c
.L02eb66:
        move.b  #0x6e,0x20(a6)                  | +052
        lsr.w   #0x2,d0                         | +058
        andi.w  #0xff,d0                        | +05a
        cmpi.b  #0x8,d0                         | +05e
        bcs.w   .L02eb86                        | +062
        nop                                     | +066
        nop                                     | +068
        cmpi.b  #0x8,d0                         | +06a
        nop                                     | +06e
        trap    #0xf                            | +070
.L02eb86:
        mulu.w  #0x5,d0                         | +072
        add.b   d0,0x20(a6)                     | +076
        andi.l  #0xff,d1                        | +07a
        cmpi.b  #0x5,d1                         | +080
        bcs.w   .L02eba8                        | +084
        nop                                     | +088
        nop                                     | +08a
        cmpi.b  #0x5,d1                         | +08c
        nop                                     | +090
        trap    #0xf                            | +092
.L02eba8:
        add.b   d1,0x20(a6)                     | +094
        bra.w   Slug_DriveAlt_02e9c8__L02ea1c   | +098

| ----------------------------------------------------------------------------
|  Slug_Brake_02ebb0  @ $02EBB0  (238 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Brake_02ebb0, "ax", @progbits
        .global Slug_Brake_02ebb0
Slug_Brake_02ebb0:
        jsr     0x267e6.l                       | +000
        clr.w   0x28(a6)                        | +006
        clr.w   0x2c(a6)                        | +00a
        jsr     Sub_0002A9A0(pc)                | +00e
        movea.l #0x2793e4,a0                    | +012
        lsl.w   #0x2,d0                         | +018
        movea.l (a0,d0.w),a0                    | +01a
        cmpa.l  #0xffffffff,a0                  | +01e
        beq.w   .L02ebde                        | +024
        jsr     0x28cd4.l                       | +028
.L02ebde:
        move.w  #0x0,d0                         | +02e
        move.b  #0x96,0x20(a6)                  | +032
        andi.w  #0xff,d0                        | +038
        lsr.w   #0x2,d0                         | +03c
        add.b   d0,0x20(a6)                     | +03e
        move.l  #0x29600,0x60(a6)               | +042
        lea     .L02ec00(pc),a1                 | +04a
        move.l  a1,(a6)                         | +04e
.L02ec00:
        jsr     Sub_0002AA24(pc)                | +050
        bset    #0x4,0x8d(a6)                   | +054
        jsr     0x28992.l                       | +05a
        jsr     Sub_0002A824(pc)                | +060
        bcc.w   .L02ec1e                        | +064
        lea     Slug_Stall_02eca6__L02ecc2(pc),a1 | +068
        move.l  a1,(a6)                         | +06c
.L02ec1e:
        jsr     Sub_0002A4F0(pc)                | +06e
        bcc.w   .L02ec2c                        | +072
        lea     Slug_Stall_02eca6__L02ecc2(pc),a1 | +076
        move.l  a1,(a6)                         | +07a
.L02ec2c:
        jsr     ClearXN_02abc0(pc)              | +07c
        bcs.w   .L02ec50                        | +080
        jsr     Sub_0002A328(pc)                | +084
        bcc.w   .L02ec42                        | +088
        lea     Slug_AccelRight_02ee78__L02ee7e(pc),a1 | +08c
        move.l  a1,(a6)                         | +090
.L02ec42:
        jsr     Sub_0002A34E(pc)                | +092
        bcc.w   .L02ec50                        | +096
        lea     Slug_AccelLeft_02efaa__L02efb0(pc),a1 | +09a
        move.l  a1,(a6)                         | +09e
.L02ec50:
        jsr     0x28d70.l                       | +0a0
        jsr     0x5cef8.l                       | +0a6
        bcs.w   .L02ec66                        | +0ac
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +0b0
        move.l  a1,(a6)                         | +0b4
.L02ec66:
        jsr     Sub_0002A8C0(pc)                | +0b6
        bcc.w   .L02ec74                        | +0ba
        lea     TaskHandler_02d02e(pc),a1       | +0be
        move.l  a1,(a6)                         | +0c2
.L02ec74:
        jsr     PcThunkTarget_02ac80(pc)        | +0c4
        bcc.w   .L02ec82                        | +0c8
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0cc
        move.l  a1,(a6)                         | +0d0
.L02ec82:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0d2
        bcc.w   .L02ec90                        | +0d6
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +0da
        move.l  a1,(a6)                         | +0de
.L02ec90:
        jsr     Sub_0002A690(pc)                | +0e0
        lea     Sub_0002A060(pc),a0             | +0e4
        movea.l #0xffffffff,a1                  | +0e8

| ----------------------------------------------------------------------------
|  Slug_Stall_02eca6  @ $02ECA6  (458 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Stall_02eca6, "ax", @progbits
        .global Slug_Stall_02eca6
Slug_Stall_02eca6:
        move.w  #0x10af,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   Slug_Stall_02eca6__L02ecc2      | +00a
        .global Slug_Stall_02eca6__L02ecb4
Slug_Stall_02eca6__L02ecb4:
        move.w  #0x10b0,d0                      | +00e
        jsr     0x2352.l                        | +012
        bra.w   Slug_Stall_02eca6__L02ecc2      | +018
        .global Slug_Stall_02eca6__L02ecc2
Slug_Stall_02eca6__L02ecc2:
        clr.w   0x2c(a6)                        | +01c
        jsr     Sub_0002A958(pc)                | +020
        move.w  d2,0x80(a6)                     | +024
        move.w  d2,d0                           | +028
        asr.w   #0x1,d0                         | +02a
        neg.w   d0                              | +02c
        addi.w  #0x20,d0                        | +02e
        cmpi.w  #0x40,d0                        | +032
        bls.w   .L02ece4                        | +036
        move.w  #0x40,d0                        | +03a
.L02ece4:
        movem.w d0,-(a7)                        | +03e
        jsr     Sub_000295A6(pc)                | +042
        movea.l #0x2b0c74,a0                    | +046
        lsl.w   #0x2,d0                         | +04c
        movea.l (a0,d0.w),a0                    | +04e
        cmpa.l  #0xffffffff,a0                  | +052
        beq.w   .L02ed08                        | +058
        jsr     0x28cd4.l                       | +05c
.L02ed08:
        movem.w (a7)+,d0                        | +062
        move.w  #0x0,d0                         | +066
        move.b  #0xc3,0x20(a6)                  | +06a
        lea     .L02ed1c(pc),a1                 | +070
        move.l  a1,(a6)                         | +074
.L02ed1c:
        bset    #0x2,0x8d(a6)                   | +076
        jsr     Sub_0002AA24(pc)                | +07c
        clr.w   0x28(a6)                        | +080
        bset    #0x4,0x8d(a6)                   | +084
        jsr     ClearXN_02abc0(pc)              | +08a
        bcs.w   .L02ed58                        | +08e
        jsr     Sub_0002A328(pc)                | +092
        bcc.w   .L02ed4a                        | +096
        move.w  #0x2a0,0x28(a6)                 | +09a
        bra.w   .L02ed58                        | +0a0
.L02ed4a:
        jsr     Sub_0002A34E(pc)                | +0a4
        bcc.w   .L02ed58                        | +0a8
        move.w  #0xfd60,0x28(a6)                | +0ac
.L02ed58:
        cmpi.b  #0x3,0x106f2a.l                 | +0b2
        bne.w   .L02ed82                        | +0ba
        move.w  0x28(a6),d7                     | +0be
        jsr     Sub_0002A958(pc)                | +0c2
        sub.w   0x80(a6),d2                     | +0c6
        eor.w   d7,d2                           | +0ca
        asl.w   #0x1,d2                         | +0cc
        bcs.w   .L02ed7e                        | +0ce
        move.w  d7,d2                           | +0d2
        asr.w   #0x1,d2                         | +0d4
        add.w   d2,d7                           | +0d6
.L02ed7e:
        move.w  d7,0x28(a6)                     | +0d8
.L02ed82:
        jsr     Sub_0002A478(pc)                | +0dc
        jsr     0x28992.l                       | +0e0
        jsr     Sub_0002A766(pc)                | +0e6
        jsr     Sub_0002A958(pc)                | +0ea
        cmp.w   0x80(a6),d2                     | +0ee
        beq.w   .L02ede8                        | +0f2
        move.w  d2,0x80(a6)                     | +0f6
        move.w  d2,d0                           | +0fa
        asr.w   #0x1,d0                         | +0fc
        neg.w   d0                              | +0fe
        addi.w  #0x20,d0                        | +100
        cmpi.w  #0x40,d0                        | +104
        bls.w   .L02edb6                        | +108
        move.w  #0x40,d0                        | +10c
.L02edb6:
        movem.w d0,-(a7)                        | +110
        jsr     Sub_000295A6(pc)                | +114
        movea.l #0x2b0c74,a0                    | +118
        lsl.w   #0x2,d0                         | +11e
        movea.l (a0,d0.w),a0                    | +120
        cmpa.l  #0xffffffff,a0                  | +124
        beq.w   .L02edda                        | +12a
        jsr     0x28cd4.l                       | +12e
.L02edda:
        movem.w (a7)+,d0                        | +134
        move.w  #0x0,d0                         | +138
        move.b  #0xc3,0x20(a6)                  | +13c
.L02ede8:
        jsr     0x28d70.l                       | +142
        jsr     Sub_0002A4EC(pc)                | +148
        bcs.w   .L02ee1c                        | +14c
        bra.w   .L02edfa                        | +150
.L02edfa:
        lea     Slug_Brake_02ebb0(pc),a1        | +154
        move.l  a1,(a6)                         | +158
        jsr     Sub_0002A328(pc)                | +15a
        bcc.w   .L02ee0e                        | +15e
        lea     Slug_CruiseRight_02f422(pc),a1  | +162
        move.l  a1,(a6)                         | +166
.L02ee0e:
        jsr     Sub_0002A34E(pc)                | +168
        bcc.w   .L02ee1c                        | +16c
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +170
        move.l  a1,(a6)                         | +174
.L02ee1c:
        tst.b   0x3b(a6)                        | +176
        bne.w   .L02ee2a                        | +17a
        jsr     0x283d8.l                       | +17e
.L02ee2a:
        jsr     Sub_0002A59A(pc)                | +184
        bcs.w   .L02ee38                        | +188
        lea     Sub_0002B7DA(pc),a1             | +18c
        move.l  a1,(a6)                         | +190
.L02ee38:
        jsr     Sub_0002A8C0(pc)                | +192
        bcc.w   .L02ee46                        | +196
        lea     Sub_0002CFFA(pc),a1             | +19a
        move.l  a1,(a6)                         | +19e
.L02ee46:
        jsr     PcThunkTarget_02ac80(pc)        | +1a0
        bcc.w   .L02ee54                        | +1a4
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +1a8
        move.l  a1,(a6)                         | +1ac
.L02ee54:
        jsr     JsrAbsThunk_02a5cc(pc)          | +1ae
        bcc.w   .L02ee62                        | +1b2
        lea     Slug_Destroyed_02fc70(pc),a1    | +1b6
        move.l  a1,(a6)                         | +1ba
.L02ee62:
        jsr     Sub_0002A690(pc)                | +1bc
        lea     Sub_0002A060(pc),a0             | +1c0
        movea.l #0xffffffff,a1                  | +1c4

| ----------------------------------------------------------------------------
|  Slug_AccelRight_02ee78  @ $02EE78  (298 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelRight_02ee78, "ax", @progbits
        .global Slug_AccelRight_02ee78
Slug_AccelRight_02ee78:
        jsr     0x31e5e.l                       | +000
        .global Slug_AccelRight_02ee78__L02ee7e
Slug_AccelRight_02ee78__L02ee7e:
        move.w  #0x2a0,d0                       | +006
        sub.w   0x28(a6),d0                     | +00a
        asr.w   #0x3,d0                         | +00e
        move.w  d0,0x2c(a6)                     | +010
        move.b  #0x8,0x91(a6)                   | +014
        jsr     Sub_0002A9A0(pc)                | +01a
        movea.l #0x279470,a0                    | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L02eeb2                        | +030
        jsr     0x28cd4.l                       | +034
.L02eeb2:
        lsr.w   #0x1,d0                         | +03a
        lea     Sub_0002B8CE(pc),a0             | +03c
        move.w  (a0,d0.w),d0                    | +040
        cmpi.w  #0xffff,d0                      | +044
        beq.w   .L02eeca                        | +048
        jsr     0x2352.l                        | +04c
.L02eeca:
        move.w  #0x0,d0                         | +052
        move.b  #0x96,0x20(a6)                  | +056
        andi.w  #0xff,d0                        | +05c
        lsr.w   #0x2,d0                         | +060
        add.b   d0,0x20(a6)                     | +062
        lea     .L02eee4(pc),a1                 | +066
        move.l  a1,(a6)                         | +06a
.L02eee4:
        subq.b  #0x1,0x91(a6)                   | +06c
        bne.w   .L02eefc                        | +070
        move.w  #0x2a0,0x28(a6)                 | +074
        clr.w   0x2c(a6)                        | +07a
        lea     .L02eefc(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L02eefc:
        jsr     Sub_0002AA24(pc)                | +084
        bset    #0x4,0x8d(a6)                   | +088
        jsr     0x28992.l                       | +08e
        jsr     Sub_0002A824(pc)                | +094
        bcc.w   .L02ef1a                        | +098
        lea     Slug_Stall_02eca6(pc),a1        | +09c
        move.l  a1,(a6)                         | +0a0
.L02ef1a:
        jsr     Sub_0002A4F0(pc)                | +0a2
        bcc.w   .L02ef28                        | +0a6
        lea     Slug_Stall_02eca6(pc),a1        | +0aa
        move.l  a1,(a6)                         | +0ae
.L02ef28:
        jsr     0x5cef8.l                       | +0b0
        bcs.w   .L02ef38                        | +0b6
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +0ba
        move.l  a1,(a6)                         | +0be
.L02ef38:
        jsr     0x28d70.l                       | +0c0
        bcc.w   .L02ef48                        | +0c6
        lea     Slug_CruiseRight_02f422(pc),a1  | +0ca
        move.l  a1,(a6)                         | +0ce
.L02ef48:
        jsr     0x283d8.l                       | +0d0
        jsr     Sub_0002A328(pc)                | +0d6
        bcs.w   .L02ef5c                        | +0da
        lea     Slug_DecelA_02f202(pc),a1       | +0de
        move.l  a1,(a6)                         | +0e2
.L02ef5c:
        jsr     Sub_0002A34E(pc)                | +0e4
        bcc.w   .L02ef6a                        | +0e8
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0ec
        move.l  a1,(a6)                         | +0f0
.L02ef6a:
        jsr     Sub_0002A8C0(pc)                | +0f2
        bcc.w   .L02ef78                        | +0f6
        lea     TaskHandler_02d02e(pc),a1       | +0fa
        move.l  a1,(a6)                         | +0fe
.L02ef78:
        jsr     PcThunkTarget_02ac80(pc)        | +100
        bcc.w   .L02ef86                        | +104
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +108
        move.l  a1,(a6)                         | +10c
.L02ef86:
        jsr     JsrAbsThunk_02a5cc(pc)          | +10e
        bcc.w   .L02ef94                        | +112
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L02ef94:
        jsr     Sub_0002A690(pc)                | +11c
        lea     Sub_0002A060(pc),a0             | +120
        movea.l #0xffffffff,a1                  | +124

| ----------------------------------------------------------------------------
|  Slug_AccelLeft_02efaa  @ $02EFAA  (298 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AccelLeft_02efaa, "ax", @progbits
        .global Slug_AccelLeft_02efaa
Slug_AccelLeft_02efaa:
        jsr     0x31e76.l                       | +000
        .global Slug_AccelLeft_02efaa__L02efb0
Slug_AccelLeft_02efaa__L02efb0:
        move.w  #0xfd60,d0                      | +006
        sub.w   0x28(a6),d0                     | +00a
        asr.w   #0x3,d0                         | +00e
        move.w  d0,0x2c(a6)                     | +010
        move.b  #0x8,0x91(a6)                   | +014
        jsr     Sub_0002A9A0(pc)                | +01a
        movea.l #0x279484,a0                    | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L02efe4                        | +030
        jsr     0x28cd4.l                       | +034
.L02efe4:
        lsr.w   #0x1,d0                         | +03a
        lea     Sub_0002BA34(pc),a0             | +03c
        move.w  (a0,d0.w),d0                    | +040
        cmpi.w  #0xffff,d0                      | +044
        beq.w   .L02effc                        | +048
        jsr     0x2352.l                        | +04c
.L02effc:
        move.w  #0x0,d0                         | +052
        move.b  #0x96,0x20(a6)                  | +056
        andi.w  #0xff,d0                        | +05c
        lsr.w   #0x2,d0                         | +060
        add.b   d0,0x20(a6)                     | +062
        lea     .L02f016(pc),a1                 | +066
        move.l  a1,(a6)                         | +06a
.L02f016:
        subq.b  #0x1,0x91(a6)                   | +06c
        bne.w   .L02f02e                        | +070
        move.w  #0xfd60,0x28(a6)                | +074
        clr.w   0x2c(a6)                        | +07a
        lea     .L02f02e(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L02f02e:
        jsr     Sub_0002AA24(pc)                | +084
        bset    #0x4,0x8d(a6)                   | +088
        jsr     0x28992.l                       | +08e
        jsr     Sub_0002A824(pc)                | +094
        bcc.w   .L02f04c                        | +098
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +09c
        move.l  a1,(a6)                         | +0a0
.L02f04c:
        jsr     Sub_0002A4F0(pc)                | +0a2
        bcc.w   .L02f05a                        | +0a6
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
.L02f05a:
        jsr     0x5cef8.l                       | +0b0
        bcs.w   .L02f06a                        | +0b6
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +0ba
        move.l  a1,(a6)                         | +0be
.L02f06a:
        jsr     0x28d70.l                       | +0c0
        bcc.w   .L02f07a                        | +0c6
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0ca
        move.l  a1,(a6)                         | +0ce
.L02f07a:
        jsr     0x283d8.l                       | +0d0
        jsr     Sub_0002A34E(pc)                | +0d6
        bcs.w   .L02f08e                        | +0da
        lea     Slug_DecelB_02f312(pc),a1       | +0de
        move.l  a1,(a6)                         | +0e2
.L02f08e:
        jsr     Sub_0002A328(pc)                | +0e4
        bcc.w   .L02f09c                        | +0e8
        lea     Slug_CruiseRight_02f422(pc),a1  | +0ec
        move.l  a1,(a6)                         | +0f0
.L02f09c:
        jsr     Sub_0002A8C0(pc)                | +0f2
        bcc.w   .L02f0aa                        | +0f6
        lea     TaskHandler_02d02e(pc),a1       | +0fa
        move.l  a1,(a6)                         | +0fe
.L02f0aa:
        jsr     PcThunkTarget_02ac80(pc)        | +100
        bcc.w   .L02f0b8                        | +104
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +108
        move.l  a1,(a6)                         | +10c
.L02f0b8:
        jsr     JsrAbsThunk_02a5cc(pc)          | +10e
        bcc.w   .L02f0c6                        | +112
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L02f0c6:
        jsr     Sub_0002A690(pc)                | +11c
        lea     Sub_0002A060(pc),a0             | +120
        movea.l #0xffffffff,a1                  | +124

| ----------------------------------------------------------------------------
|  Slug_Knocked_02f0dc  @ $02F0DC  (288 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Knocked_02f0dc, "ax", @progbits
        .global Slug_Knocked_02f0dc
Slug_Knocked_02f0dc:
        move.w  #0x600,0x28(a6)                 | +000
        bra.w   .L02f102                        | +006
        .global Slug_Knocked_02f0dc__L02f0e6
Slug_Knocked_02f0dc__L02f0e6:
        move.w  #0x10e9,d0                      | +00a
        jsr     0x2352.l                        | +00e
        jsr     Slug_KillInit_02feda(pc)        | +014
        move.w  #0x600,d0                       | +018
        move.w  d0,0x28(a6)                     | +01c
        move.b  #0x30,0x92(a6)                  | +020
.L02f102:
        jsr     Sub_0002A9A0(pc)                | +026
        movea.l #0x279394,a0                    | +02a
        lsl.w   #0x2,d0                         | +030
        movea.l (a0,d0.w),a0                    | +032
        cmpa.l  #0xffffffff,a0                  | +036
        beq.w   .L02f122                        | +03c
        jsr     0x28cd4.l                       | +040
.L02f122:
        lsr.w   #0x1,d0                         | +046
        lea     Sub_0002BB9A(pc),a0             | +048
        move.w  (a0,d0.w),d0                    | +04c
        cmpi.w  #0xffff,d0                      | +050
        beq.w   .L02f13a                        | +054
        jsr     0x2352.l                        | +058
.L02f13a:
        move.w  #0x0,d0                         | +05e
        move.b  #0x96,0x20(a6)                  | +062
        andi.w  #0xff,d0                        | +068
        lsr.w   #0x2,d0                         | +06c
        add.b   d0,0x20(a6)                     | +06e
        lea     .L02f154(pc),a1                 | +072
        move.l  a1,(a6)                         | +076
.L02f154:
        cmpi.b  #0x29,0x92(a6)                  | +078
        bhi.w   .L02f16e                        | +07e
        move.w  #0x600,0x28(a6)                 | +082
        clr.w   0x2c(a6)                        | +088
        lea     .L02f16e(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L02f16e:
        jsr     Sub_0002AA24(pc)                | +092
        bset    #0x4,0x8d(a6)                   | +096
        jsr     0x28992.l                       | +09c
        jsr     Sub_0002A7D8(pc)                | +0a2
        bcc.w   .L02f18c                        | +0a6
        lea     Slug_Destroyed_02fc70__L02fc8e(pc),a1 | +0aa
        move.l  a1,(a6)                         | +0ae
.L02f18c:
        jsr     0x28d70.l                       | +0b0
        move.b  0x106f28.l,d0                   | +0b6
        andi.b  #0x1,d0                         | +0bc
        bne.w   .L02f1b4                        | +0c0
        lea     Sub_00029834(pc),a0             | +0c4
        move.l  a0,0x4c(a6)                     | +0c8
        jsr     0x283ca.l                       | +0cc
        jsr     0x283ca.l                       | +0d2
.L02f1b4:
        jsr     0x283d8.l                       | +0d8
        jsr     Slug_UpdateDamageSprite_02fae4__L02fb04(pc) | +0de
        bcc.w   .L02f1c8                        | +0e2
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +0e6
        move.l  a1,(a6)                         | +0ea
.L02f1c8:
        jsr     Slug_ClampField92_02ffb0(pc)    | +0ec
        tst.b   0x92(a6)                        | +0f0
        beq.w   .L02f1da                        | +0f4
        subi.b  #0x1,0x92(a6)                   | +0f8
.L02f1da:
        bhi.w   .L02f1e4                        | +0fe
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +102
        move.l  a1,(a6)                         | +106
.L02f1e4:
        cmpi.w  #0x117,0x22(a6)                 | +108
        blt.w   .L02f1f4                        | +10e
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +112
        move.l  a1,(a6)                         | +116
.L02f1f4:
        clr.w   0x2a(a6)                        | +118
        clr.w   0x2e(a6)                        | +11c

| ----------------------------------------------------------------------------
|  Slug_DecelA_02f202  @ $02F202  (264 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DecelA_02f202, "ax", @progbits
        .global Slug_DecelA_02f202
Slug_DecelA_02f202:
        clr.w   d0                              | +000
        sub.w   0x28(a6),d0                     | +002
        asr.w   #0x3,d0                         | +006
        move.w  d0,0x2c(a6)                     | +008
        move.b  #0x8,0x91(a6)                   | +00c
        jsr     Sub_0002A9A0(pc)                | +012
        movea.l #0x2793bc,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02f234                        | +028
        jsr     0x28cd4.l                       | +02c
.L02f234:
        move.w  #0x0,d0                         | +032
        move.b  #0x96,0x20(a6)                  | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        lea     .L02f24e(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L02f24e:
        subq.b  #0x1,0x91(a6)                   | +04c
        bne.w   .L02f264                        | +050
        clr.w   0x28(a6)                        | +054
        clr.w   0x2c(a6)                        | +058
        lea     .L02f264(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L02f264:
        jsr     Sub_0002AA24(pc)                | +062
        bset    #0x4,0x8d(a6)                   | +066
        jsr     0x28992.l                       | +06c
        jsr     Sub_0002A824(pc)                | +072
        bcc.w   .L02f282                        | +076
        lea     Slug_Stall_02eca6(pc),a1        | +07a
        move.l  a1,(a6)                         | +07e
.L02f282:
        jsr     Sub_0002A4F0(pc)                | +080
        bcc.w   .L02f290                        | +084
        lea     Slug_Stall_02eca6(pc),a1        | +088
        move.l  a1,(a6)                         | +08c
.L02f290:
        jsr     0x5cef8.l                       | +08e
        bcs.w   .L02f2a0                        | +094
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +098
        move.l  a1,(a6)                         | +09c
.L02f2a0:
        jsr     0x28d70.l                       | +09e
        bcc.w   .L02f2b0                        | +0a4
        lea     Slug_Brake_02ebb0(pc),a1        | +0a8
        move.l  a1,(a6)                         | +0ac
.L02f2b0:
        jsr     0x283d8.l                       | +0ae
        jsr     Sub_0002A328(pc)                | +0b4
        bcc.w   .L02f2c4                        | +0b8
        lea     Slug_CruiseRight_02f422(pc),a1  | +0bc
        move.l  a1,(a6)                         | +0c0
.L02f2c4:
        jsr     Sub_0002A34E(pc)                | +0c2
        bcc.w   .L02f2d2                        | +0c6
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0ca
        move.l  a1,(a6)                         | +0ce
.L02f2d2:
        jsr     Sub_0002A8C0(pc)                | +0d0
        bcc.w   .L02f2e0                        | +0d4
        lea     TaskHandler_02d02e(pc),a1       | +0d8
        move.l  a1,(a6)                         | +0dc
.L02f2e0:
        jsr     PcThunkTarget_02ac80(pc)        | +0de
        bcc.w   .L02f2ee                        | +0e2
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0e6
        move.l  a1,(a6)                         | +0ea
.L02f2ee:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0ec
        bcc.w   .L02f2fc                        | +0f0
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +0f4
        move.l  a1,(a6)                         | +0f8
.L02f2fc:
        jsr     Sub_0002A690(pc)                | +0fa
        lea     Sub_0002A060(pc),a0             | +0fe
        movea.l #0xffffffff,a1                  | +102

| ----------------------------------------------------------------------------
|  Slug_DecelB_02f312  @ $02F312  (264 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DecelB_02f312, "ax", @progbits
        .global Slug_DecelB_02f312
Slug_DecelB_02f312:
        clr.w   d0                              | +000
        sub.w   0x28(a6),d0                     | +002
        asr.w   #0x3,d0                         | +006
        move.w  d0,0x2c(a6)                     | +008
        move.b  #0x8,0x91(a6)                   | +00c
        jsr     Sub_0002A9A0(pc)                | +012
        movea.l #0x2793f8,a0                    | +016
        lsl.w   #0x2,d0                         | +01c
        movea.l (a0,d0.w),a0                    | +01e
        cmpa.l  #0xffffffff,a0                  | +022
        beq.w   .L02f344                        | +028
        jsr     0x28cd4.l                       | +02c
.L02f344:
        move.w  #0x0,d0                         | +032
        move.b  #0x96,0x20(a6)                  | +036
        andi.w  #0xff,d0                        | +03c
        lsr.w   #0x2,d0                         | +040
        add.b   d0,0x20(a6)                     | +042
        lea     .L02f35e(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L02f35e:
        subq.b  #0x1,0x91(a6)                   | +04c
        bne.w   .L02f374                        | +050
        clr.w   0x28(a6)                        | +054
        clr.w   0x2c(a6)                        | +058
        lea     .L02f374(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L02f374:
        jsr     Sub_0002AA24(pc)                | +062
        bset    #0x4,0x8d(a6)                   | +066
        jsr     0x28992.l                       | +06c
        jsr     Sub_0002A824(pc)                | +072
        bcc.w   .L02f392                        | +076
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +07a
        move.l  a1,(a6)                         | +07e
.L02f392:
        jsr     Sub_0002A4F0(pc)                | +080
        bcc.w   .L02f3a0                        | +084
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +088
        move.l  a1,(a6)                         | +08c
.L02f3a0:
        jsr     0x5cef8.l                       | +08e
        bcs.w   .L02f3b0                        | +094
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +098
        move.l  a1,(a6)                         | +09c
.L02f3b0:
        jsr     0x28d70.l                       | +09e
        bcc.w   .L02f3c0                        | +0a4
        lea     Slug_Brake_02ebb0(pc),a1        | +0a8
        move.l  a1,(a6)                         | +0ac
.L02f3c0:
        jsr     0x283d8.l                       | +0ae
        jsr     Sub_0002A328(pc)                | +0b4
        bcc.w   .L02f3d4                        | +0b8
        lea     Slug_CruiseRight_02f422(pc),a1  | +0bc
        move.l  a1,(a6)                         | +0c0
.L02f3d4:
        jsr     Sub_0002A34E(pc)                | +0c2
        bcc.w   .L02f3e2                        | +0c6
        lea     Slug_CruiseLeft_02f4fc(pc),a1   | +0ca
        move.l  a1,(a6)                         | +0ce
.L02f3e2:
        jsr     Sub_0002A8C0(pc)                | +0d0
        bcc.w   .L02f3f0                        | +0d4
        lea     TaskHandler_02d02e(pc),a1       | +0d8
        move.l  a1,(a6)                         | +0dc
.L02f3f0:
        jsr     PcThunkTarget_02ac80(pc)        | +0de
        bcc.w   .L02f3fe                        | +0e2
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0e6
        move.l  a1,(a6)                         | +0ea
.L02f3fe:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0ec
        bcc.w   .L02f40c                        | +0f0
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +0f4
        move.l  a1,(a6)                         | +0f8
.L02f40c:
        jsr     Sub_0002A690(pc)                | +0fa
        lea     Sub_0002A060(pc),a0             | +0fe
        movea.l #0xffffffff,a1                  | +102

| ----------------------------------------------------------------------------
|  Slug_CruiseRight_02f422  @ $02F422  (210 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CruiseRight_02f422, "ax", @progbits
        .global Slug_CruiseRight_02f422
Slug_CruiseRight_02f422:
        move.w  #0x2a0,0x28(a6)                 | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Sub_0002A9A0(pc)                | +00a
        movea.l #0x279434,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02f44c                        | +020
        jsr     0x28cd4.l                       | +024
.L02f44c:
        move.w  #0x0,d0                         | +02a
        move.b  #0x96,0x20(a6)                  | +02e
        andi.w  #0xff,d0                        | +034
        lsr.w   #0x2,d0                         | +038
        add.b   d0,0x20(a6)                     | +03a
        lea     .L02f466(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L02f466:
        jsr     Sub_0002AA24(pc)                | +044
        bset    #0x4,0x8d(a6)                   | +048
        jsr     0x28992.l                       | +04e
        jsr     Sub_0002A824(pc)                | +054
        bcc.w   .L02f484                        | +058
        lea     Slug_Stall_02eca6(pc),a1        | +05c
        move.l  a1,(a6)                         | +060
.L02f484:
        jsr     Sub_0002A4F0(pc)                | +062
        bcc.w   .L02f492                        | +066
        lea     Slug_Stall_02eca6(pc),a1        | +06a
        move.l  a1,(a6)                         | +06e
.L02f492:
        jsr     0x5cef8.l                       | +070
        bcs.w   .L02f4a2                        | +076
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +07a
        move.l  a1,(a6)                         | +07e
.L02f4a2:
        jsr     Sub_0002A328(pc)                | +080
        bcs.w   .L02f4b0                        | +084
        lea     Slug_DecelA_02f202(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L02f4b0:
        jsr     0x28d70.l                       | +08e
        jsr     0x283d8.l                       | +094
        jsr     Sub_0002A8C0(pc)                | +09a
        bcc.w   .L02f4ca                        | +09e
        lea     TaskHandler_02d02e(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6
.L02f4ca:
        jsr     PcThunkTarget_02ac80(pc)        | +0a8
        bcc.w   .L02f4d8                        | +0ac
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0b0
        move.l  a1,(a6)                         | +0b4
.L02f4d8:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0b6
        bcc.w   .L02f4e6                        | +0ba
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +0be
        move.l  a1,(a6)                         | +0c2
.L02f4e6:
        jsr     Sub_0002A690(pc)                | +0c4
        lea     Sub_0002A060(pc),a0             | +0c8
        movea.l #0xffffffff,a1                  | +0cc

| ----------------------------------------------------------------------------
|  Slug_CruiseLeft_02f4fc  @ $02F4FC  (210 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_CruiseLeft_02f4fc, "ax", @progbits
        .global Slug_CruiseLeft_02f4fc
Slug_CruiseLeft_02f4fc:
        move.w  #0xfd60,0x28(a6)                | +000
        clr.w   0x2c(a6)                        | +006
        jsr     Sub_0002A9A0(pc)                | +00a
        movea.l #0x279448,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L02f526                        | +020
        jsr     0x28cd4.l                       | +024
.L02f526:
        move.w  #0x0,d0                         | +02a
        move.b  #0x96,0x20(a6)                  | +02e
        andi.w  #0xff,d0                        | +034
        lsr.w   #0x2,d0                         | +038
        add.b   d0,0x20(a6)                     | +03a
        lea     .L02f540(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L02f540:
        jsr     Sub_0002AA24(pc)                | +044
        bset    #0x4,0x8d(a6)                   | +048
        jsr     0x28992.l                       | +04e
        jsr     Sub_0002A824(pc)                | +054
        bcc.w   .L02f55e                        | +058
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +05c
        move.l  a1,(a6)                         | +060
.L02f55e:
        jsr     Sub_0002A4F0(pc)                | +062
        bcc.w   .L02f56c                        | +066
        lea     Slug_Stall_02eca6__L02ecb4(pc),a1 | +06a
        move.l  a1,(a6)                         | +06e
.L02f56c:
        jsr     0x5cef8.l                       | +070
        bcs.w   .L02f57c                        | +076
        lea     Slug_DriveAlt_02e9c8(pc),a1     | +07a
        move.l  a1,(a6)                         | +07e
.L02f57c:
        jsr     Sub_0002A34E(pc)                | +080
        bcs.w   .L02f58a                        | +084
        lea     Slug_DecelB_02f312(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L02f58a:
        jsr     0x28d70.l                       | +08e
        jsr     0x283d8.l                       | +094
        jsr     Sub_0002A8C0(pc)                | +09a
        bcc.w   .L02f5a4                        | +09e
        lea     TaskHandler_02d02e(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6
.L02f5a4:
        jsr     PcThunkTarget_02ac80(pc)        | +0a8
        bcc.w   .L02f5b2                        | +0ac
        lea     Slug_IdleEnterB_02dd74(pc),a1   | +0b0
        move.l  a1,(a6)                         | +0b4
.L02f5b2:
        jsr     JsrAbsThunk_02a5cc(pc)          | +0b6
        bcc.w   .L02f5c0                        | +0ba
        lea     Slug_Knocked_02f0dc__L02f0e6(pc),a1 | +0be
        move.l  a1,(a6)                         | +0c2
.L02f5c0:
        jsr     Sub_0002A690(pc)                | +0c4
        lea     Sub_0002A060(pc),a0             | +0c8
        movea.l #0xffffffff,a1                  | +0cc

| ----------------------------------------------------------------------------
|  Slug_SetSpeed_02f5d6  @ $02F5D6  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SetSpeed_02f5d6, "ax", @progbits
        .global Slug_SetSpeed_02f5d6
Slug_SetSpeed_02f5d6:
        jsr     Sub_0002A9A0(pc)                | +000
        movea.l #0x279308,a0                    | +004
        lsl.w   #0x2,d0                         | +00a
        movea.l (a0,d0.w),a0                    | +00c
        cmpa.l  #0xffffffff,a0                  | +010
        beq.w   .L02f5f6                        | +016
        jsr     0x28cd4.l                       | +01a
.L02f5f6:
        move.w  #0x0,d0                         | +020
        move.b  #0x96,0x20(a6)                  | +024
        andi.w  #0xff,d0                        | +02a
        lsr.w   #0x2,d0                         | +02e
        add.b   d0,0x20(a6)                     | +030
        lea     Sub_0002C95C(pc),a1             | +034
        move.l  a1,(a6)                         | +038
        bra.w   Sub_0002C95C                    | +03a
        .global Slug_SetSpeed_02f5d6__L02f614
Slug_SetSpeed_02f5d6__L02f614:
        move.w  #0x0,0x28(a6)                   | +03e
        bra.b   Slug_SetSpeed_02f5d6            | +044
        move.w  #0x2aa,0x28(a6)                 | +046
        bra.b   Slug_SetSpeed_02f5d6            | +04c
        move.w  #0xfd56,0x28(a6)                | +04e
        bra.b   Slug_SetSpeed_02f5d6            | +054
        move.w  #0x155,0x28(a6)                 | +056
        bra.b   Slug_SetSpeed_02f5d6            | +05c
        move.w  #0xfeab,0x28(a6)                | +05e
        bra.b   Slug_SetSpeed_02f5d6            | +064
        move.w  #0xaa,0x28(a6)                  | +066
        bra.b   Slug_SetSpeed_02f5d6            | +06c
        move.w  #0xff56,0x28(a6)                | +06e
        bra.b   Slug_SetSpeed_02f5d6            | +074

| ----------------------------------------------------------------------------
|  SlugFx_Smoke_02f64c  @ $02F64C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_Smoke_02f64c, "ax", @progbits
        .global SlugFx_Smoke_02f64c
SlugFx_Smoke_02f64c:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd6a6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x2000,0x38(a6)                | +016
        lea     SlugFx_Fall_02f692(pc),a1       | +01c
        move.l  a1,(a6)                         | +020
        bra.w   SlugFx_Fall_02f692              | +022
        .global SlugFx_Smoke_02f64c__L02f672
SlugFx_Smoke_02f64c__L02f672:
        move.w  #0x163,d1                       | +026
        jsr     0x236e.l                        | +02a
        lea     SlugFx_ExplosionAnim_02f6c0(pc),a0 | +030
        jsr     0x28cd4.l                       | +034
        move.w  #0xd000,0x38(a6)                | +03a
        lea     SlugFx_Fall_02f692(pc),a1       | +040
        move.l  a1,(a6)                         | +044

| ----------------------------------------------------------------------------
|  SlugFx_Fall_02f692  @ $02F692  (46 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_Fall_02f692, "ax", @progbits
        .global SlugFx_Fall_02f692
SlugFx_Fall_02f692:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        bcc.w   .L02f6a8                        | +00c
        jmp     0x518.l                         | +010
.L02f6a8:
        movea.l #0xffffffff,a0                  | +016
        jsr     0x5dd56.l                       | +01c
        bcc.w   .L02f6be                        | +022
        jmp     0x518.l                         | +026
.L02f6be:
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  SlugFx_ExplosionAnim_02f6c0  @ $02F6C0  (394 B)
| ----------------------------------------------------------------------------
        .section .text.SlugFx_ExplosionAnim_02f6c0, "ax", @progbits
        .global SlugFx_ExplosionAnim_02f6c0
SlugFx_ExplosionAnim_02f6c0:
        .dc.w   0x0001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +004  (dato / opcode no decodificado)
        .dc.w   0x4f10                        | +006  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +012  (dato / opcode no decodificado)
        .dc.w   0x4f52                        | +014  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +020  (dato / opcode no decodificado)
        .dc.w   0x4f94                        | +022  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x4fc8                        | +030  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +038  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x4ff8                        | +03e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +046  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x5036                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +054  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +058  (dato / opcode no decodificado)
        .dc.w   0x5074                        | +05a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +062  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +066  (dato / opcode no decodificado)
        .dc.w   0x50b2                        | +068  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +070  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +074  (dato / opcode no decodificado)
        .dc.w   0x50ec                        | +076  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +082  (dato / opcode no decodificado)
        .dc.w   0x512a                        | +084  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +090  (dato / opcode no decodificado)
        .dc.w   0x5168                        | +092  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x51a2                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x51ea                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0001                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x5238                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x5286                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x52d2                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x5322                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x5360                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +100  (dato / opcode no decodificado)
        .dc.w   0x53ac                        | +102  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x53e8                        | +110  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +118  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x5426                        | +11e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +126  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x545e                        | +12c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +134  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +138  (dato / opcode no decodificado)
        .dc.w   0x549a                        | +13a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +142  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +144  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +146  (dato / opcode no decodificado)
        .dc.w   0x54d0                        | +148  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +150  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +154  (dato / opcode no decodificado)
        .dc.w   0x5506                        | +156  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +162  (dato / opcode no decodificado)
        .dc.w   0x5530                        | +164  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +170  (dato / opcode no decodificado)
        .dc.w   0x555c                        | +172  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +174  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x1e08                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x0022                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x557e                        | +180  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +184  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +186  (dato / opcode no decodificado)
        .dc.w   0x1600                        | +188  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_TypeIfAir_02f84a  @ $02F84A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_TypeIfAir_02f84a, "ax", @progbits
        .global Slug_TypeIfAir_02f84a
Slug_TypeIfAir_02f84a:
        btst    #0x2,0x8d(a6)                   | +000
        beq.w   .L02f858                        | +006
        move.w  0x1c(a6),d2                     | +00a
.L02f858:
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Players_AnyFlag8D3_02f85a  @ $02F85A  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AnyFlag8D3_02f85a, "ax", @progbits
        .global Players_AnyFlag8D3_02f85a
Players_AnyFlag8D3_02f85a:
        lea     0x100440.l,a0                   | +000
        btst    #0x3,0x8d(a0)                   | +006
        bne.w   Players_AnyFlag8D3_SetC_02f880  | +00c
        lea     0x1004e0.l,a0                   | +010
        btst    #0x3,0x8d(a0)                   | +016
        bne.w   Players_AnyFlag8D3_SetC_02f880  | +01c

| ----------------------------------------------------------------------------
|  Players_AnyFlag8D3_SetC_02f880  @ $02F880  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Players_AnyFlag8D3_SetC_02f880, "ax", @progbits
        .global Players_AnyFlag8D3_SetC_02f880
Players_AnyFlag8D3_SetC_02f880:
        ori.b   #0x11,ccr                       | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Slug_DamageSpriteTblA_02f886  @ $02F886  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamageSpriteTblA_02f886, "ax", @progbits
        .global Slug_DamageSpriteTblA_02f886
Slug_DamageSpriteTblA_02f886:
        .dc.w   0x0104                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1e00                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0400                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0300                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0204                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +026  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DamageSpriteTblB_02f8ae  @ $02F8AE  (398 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamageSpriteTblB_02f8ae, "ax", @progbits
        .global Slug_DamageSpriteTblB_02f8ae
Slug_DamageSpriteTblB_02f8ae:
        .dc.w   0x0104                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +048  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +084  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +090  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +098  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +100  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +104  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +108  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +10e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +110  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +112  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +114  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +116  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +118  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +11a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +11c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +11e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +120  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +122  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +124  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +126  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +128  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +12a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +12c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +12e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +130  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +132  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +134  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +136  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +138  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +13a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +13c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +13e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +140  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +142  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +144  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +146  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +148  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +14a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +14c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +14e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +150  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +152  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +154  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +156  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +158  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +15a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +15c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +15e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +160  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +162  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +164  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +166  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +168  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +16a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +16c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +16e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +170  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +172  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +174  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +176  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +178  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +17a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +17c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +17e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +180  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +182  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +184  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +186  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +188  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +18a  (dato / opcode no decodificado)
        .dc.w   0x0302                        | +18c  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DamageSpriteTblC_02fa3c  @ $02FA3C  (140 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamageSpriteTblC_02fa3c, "ax", @progbits
        .global Slug_DamageSpriteTblC_02fa3c
Slug_DamageSpriteTblC_02fa3c:
        .dc.w   0x0104                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0016                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +004  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +010  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +020  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +028  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +034  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +040  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +048  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +054  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +060  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0108                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +070  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +074  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +078  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x001a                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0104                        | +084  (dato / opcode no decodificado)
        .dc.w   0x001c                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0100                        | +08a  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DamagePtrTbl_02fac8  @ $02FAC8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamagePtrTbl_02fac8, "ax", @progbits
        .global Slug_DamagePtrTbl_02fac8
Slug_DamagePtrTbl_02fac8:
        .dc.w   0x0302                        | +000  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_DamageData_02faca  @ $02FACA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_DamageData_02faca, "ax", @progbits
        .global Slug_DamageData_02faca
Slug_DamageData_02faca:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfb3e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfb6e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0xfb82                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xfb8c                        | +00e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Slug_ResetDamageIdx_02fada  @ $02FADA  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ResetDamageIdx_02fada, "ax", @progbits
        .global Slug_ResetDamageIdx_02fada
Slug_ResetDamageIdx_02fada:
        clr.b   0x88(a6)                        | +000
        clr.w   0x86(a6)                        | +004
        rts                                     | +008

| ----------------------------------------------------------------------------
|  Slug_UpdateDamageSprite_02fae4  @ $02FAE4  (168 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_UpdateDamageSprite_02fae4, "ax", @progbits
        .global Slug_UpdateDamageSprite_02fae4
Slug_UpdateDamageSprite_02fae4:
        cmpi.w  #0x1,0x66(a6)                   | +000
        bhi.w   .L02faf6                        | +006
        lea     Slug_DamageSpriteTblA_02f886(pc),a1 | +00a
        bra.w   .L02fb0c                        | +00e
.L02faf6:
        andi.b  #0xee,ccr                       | +012
        rts                                     | +016
        lea     Slug_DamageSpriteTblB_02f8ae(pc),a1 | +018
        bra.w   .L02fb0c                        | +01c
        .global Slug_UpdateDamageSprite_02fae4__L02fb04
Slug_UpdateDamageSprite_02fae4__L02fb04:
        lea     Slug_DamageSpriteTblC_02fa3c(pc),a1 | +020
        bra.w   .L02fb0c                        | +024
.L02fb0c:
        moveq   #0,d0                           | +028
        moveq   #0,d7                           | +02a
        move.w  0x86(a6),d0                     | +02c
        move.b  (a1,d0.w),d2                    | +030
        cmpi.b  #0x4,d2                         | +034
        bcs.w   .L02fb2c                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x4,d2                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L02fb2c:
        andi.l  #0xff,d2                        | +048
        lsl.l   #0x2,d2                         | +04e
        lea     Slug_DamageData_02faca(pc),a2   | +050
        movea.l (a2,d2.w),a3                    | +054
        jmp     (a3)                            | +058
        andi.b  #0xf7,0x12(a6)                  | +05a
        move.b  0x3(a1,d0.w),d5                 | +060
        or.b    d5,0x12(a6)                     | +064
        addq.b  #0x1,0x88(a6)                   | +068
        move.b  0x88(a6),d4                     | +06c
        cmp.b   0x2(a1,d0.w),d4                 | +070
        blt.w   .L02fb68                        | +074
        clr.b   0x88(a6)                        | +078
        move.b  0x1(a1,d0.w),d7                 | +07c
        add.w   d7,0x86(a6)                     | +080
.L02fb68:
        andi.b  #0xee,ccr                       | +084
        rts                                     | +088
        move.w  0x2(a1,d0.w),d3                 | +08a
        move.w  (a6,d3.w),0x14(a6)              | +08e
        move.b  0x1(a1,d0.w),d7                 | +094
        add.w   d7,0x86(a6)                     | +098
        bra.b   .L02fb0c                        | +09c
        move.w  0x2(a1,d0.w),d3                 | +09e
        move.w  d3,0x86(a6)                     | +0a2
        bra.b   .L02fb0c                        | +0a6

| ----------------------------------------------------------------------------
|  Slug_WheelAnim_02fb92  @ $02FB92  (216 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_WheelAnim_02fb92, "ax", @progbits
        .global Slug_WheelAnim_02fb92
Slug_WheelAnim_02fb92:
        ori.b   #0x18,(a6)                      | +000
        ori.b   #0x1c,(a2)+                     | +004
        clr.w   0x70(a6)                        | +008
        move.w  #0x196,d1                       | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x197,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x198,d1                       | +020
        jsr     0x236e.l                        | +024
        move.w  #0x199,d1                       | +02a
        jsr     0x236e.l                        | +02e
        lea     .L02fbcc(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L02fbcc:
        movea.l 0xc(a6),a0                      | +03a
        btst    #0x0,0x13(a0)                   | +03e
        beq.w   .L02fbe8                        | +044
        lea     Slug_WheelAnim_Entry_02fc6a(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
        clr.w   0x70(a6)                        | +04e
        bra.w   Slug_WheelAnim_02fb92__L02fbfa  | +052
.L02fbe8:
        cmpi.w  #0x10,0x66(a0)                  | +056
        bhi.w   .L02fbf6                        | +05c
        bra.w   Slug_WheelAnim_02fb92__L02fbfa  | +060
.L02fbf6:
        jmp     .L02fc54(pc)                    | +064
        .global Slug_WheelAnim_02fb92__L02fbfa
Slug_WheelAnim_02fb92__L02fbfa:
        btst    #0x3,0x12(a0)                   | +068
        beq.w   .L02fc54                        | +06e
        addq.w  #0x2,0x70(a6)                   | +072
        andi.w  #0x6,0x70(a6)                   | +076
        moveq   #0,d0                           | +07c
        moveq   #0,d1                           | +07e
        move.w  0x70(a6),d0                     | +080
        lea     Slug_WheelAnim_02fb92(pc),a1    | +084
        move.w  (a1,d0.w),d1                    | +088
        move.w  (a6,d1.w),0x14(a6)              | +08c
        cmpi.w  #0xffff,0x40(a0)                | +092
        bne.w   .L02fc36                        | +098
        movea.l 0x3c(a0),a2                     | +09c
        bra.w   .L02fc3a                        | +0a0
.L02fc36:
        movea.l 0x40(a0),a2                     | +0a4
.L02fc3a:
        move.l  a2,0x3c(a6)                     | +0a8
        move.l  a2,0x40(a6)                     | +0ac
        jsr     0x5e4b2.l                       | +0b0
        subi.w  #0x2,0x38(a6)                   | +0b6
        jsr     0x28d70.l                       | +0bc
.L02fc54:
        movea.l 0xc(a6),a0                      | +0c2
        cmpi.l  #0x2dc5c,(a0)                   | +0c6
        bne.w   .L02fc68                        | +0cc
        jmp     0x518.l                         | +0d0
.L02fc68:
        rts                                     | +0d6

| ----------------------------------------------------------------------------
|  Slug_WheelAnim_Entry_02fc6a  @ $02FC6A  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_WheelAnim_Entry_02fc6a, "ax", @progbits
        .global Slug_WheelAnim_Entry_02fc6a
Slug_WheelAnim_Entry_02fc6a:
        movea.l 0xc(a6),a0                      | +000
        bra.b   Slug_WheelAnim_02fb92__L02fbfa  | +004

| ----------------------------------------------------------------------------
|  Slug_Destroyed_02fc70  @ $02FC70  (500 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_Destroyed_02fc70, "ax", @progbits
        .global Slug_Destroyed_02fc70
Slug_Destroyed_02fc70:
        move.w  #0x10e9,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     Slug_KillInit_02feda(pc)        | +00a
        move.b  #0x30,0x92(a6)                  | +00e
        move.w  #0x10af,d0                      | +014
        jsr     0x2352.l                        | +018
        .global Slug_Destroyed_02fc70__L02fc8e
Slug_Destroyed_02fc70__L02fc8e:
        move.w  #0x600,0x36(a6)                 | +01e
        bra.w   .L02fc9c                        | +024
        clr.w   0x36(a6)                        | +028
.L02fc9c:
        bclr    #0x5,0x8c(a6)                   | +02c
        clr.w   0x2c(a6)                        | +032
        jsr     Sub_0002A958(pc)                | +036
        move.w  d2,0x80(a6)                     | +03a
        move.w  d2,d0                           | +03e
        asr.w   #0x1,d0                         | +040
        neg.w   d0                              | +042
        addi.w  #0x20,d0                        | +044
        cmpi.w  #0x40,d0                        | +048
        bls.w   .L02fcc4                        | +04c
        move.w  #0x40,d0                        | +050
.L02fcc4:
        movem.w d0,-(a7)                        | +054
        jsr     Sub_000295A6(pc)                | +058
        movea.l #0x2b0c30,a0                    | +05c
        lsl.w   #0x2,d0                         | +062
        movea.l (a0,d0.w),a0                    | +064
        cmpa.l  #0xffffffff,a0                  | +068
        beq.w   .L02fce8                        | +06e
        jsr     0x28cd4.l                       | +072
.L02fce8:
        movem.w (a7)+,d0                        | +078
        move.b  #0x0,0x20(a6)                   | +07c
        lsr.w   #0x2,d0                         | +082
        andi.w  #0xff,d0                        | +084
        add.b   d0,0x20(a6)                     | +088
        lea     0xffff.w,a0                     | +08c
        move.l  a0,0x48(a6)                     | +090
        move.l  #0x295b4,0x60(a6)               | +094
        move.l  #0x2964c,0x60(a6)               | +09c
        lea     .L02fd1a(pc),a1                 | +0a4
        move.l  a1,(a6)                         | +0a8
.L02fd1a:
        bset    #0x2,0x8d(a6)                   | +0aa
        jsr     Sub_0002AA24(pc)                | +0b0
        move.w  0x36(a6),d7                     | +0b4
        cmpi.b  #0x3,0x106f2a.l                 | +0b8
        bne.w   .L02fd4a                        | +0c0
        jsr     Sub_0002A958(pc)                | +0c4
        sub.w   0x80(a6),d2                     | +0c8
        eor.w   d7,d2                           | +0cc
        asl.w   #0x1,d2                         | +0ce
        bcs.w   .L02fd4a                        | +0d0
        move.w  d7,d2                           | +0d4
        asr.w   #0x1,d2                         | +0d6
        add.w   d2,d7                           | +0d8
.L02fd4a:
        move.w  d7,0x28(a6)                     | +0da
        jsr     Sub_0002A478(pc)                | +0de
        jsr     Sub_0002A760(pc)                | +0e2
        jsr     Sub_0002A958(pc)                | +0e6
        cmp.w   0x80(a6),d2                     | +0ea
        beq.w   .L02fdbc                        | +0ee
        move.w  d2,0x80(a6)                     | +0f2
        move.w  d2,d0                           | +0f6
        asr.w   #0x1,d0                         | +0f8
        neg.w   d0                              | +0fa
        addi.w  #0x20,d0                        | +0fc
        cmpi.w  #0x40,d0                        | +100
        bls.w   .L02fd7c                        | +104
        move.w  #0x40,d0                        | +108
.L02fd7c:
        movem.w d0,-(a7)                        | +10c
        jsr     Sub_000295A6(pc)                | +110
        movea.l #0x2b0c30,a0                    | +114
        lsl.w   #0x2,d0                         | +11a
        movea.l (a0,d0.w),a0                    | +11c
        cmpa.l  #0xffffffff,a0                  | +120
        beq.w   .L02fda0                        | +126
        jsr     0x28cd4.l                       | +12a
.L02fda0:
        movem.w (a7)+,d0                        | +130
        move.b  #0x0,0x20(a6)                   | +134
        lsr.w   #0x2,d0                         | +13a
        andi.w  #0xff,d0                        | +13c
        add.b   d0,0x20(a6)                     | +140
        lea     0xffff.w,a0                     | +144
        move.l  a0,0x48(a6)                     | +148
.L02fdbc:
        jsr     0x28d70.l                       | +14c
        move.b  0x106f28.l,d0                   | +152
        andi.b  #0x1,d0                         | +158
        bne.w   .L02fdea                        | +15c
        lea     Sub_00029834(pc),a0             | +160
        move.l  a0,0x4c(a6)                     | +164
        jsr     0x283ca.l                       | +168
        jsr     0x283ca.l                       | +16e
        jsr     0x283d8.l                       | +174
.L02fdea:
        jsr     Sub_0002A4EC(pc)                | +17a
        bcs.w   .L02fe0c                        | +17e
        bra.w   .L02fdf6                        | +182
.L02fdf6:
        tst.b   0x92(a6)                        | +186
        beq.w   .L02fe0c                        | +18a
        tst.w   0x36(a6)                        | +18e
        ble.w   .L02fe0c                        | +192
        lea     Sub_0002BBA4(pc),a1             | +196
        move.l  a1,(a6)                         | +19a
.L02fe0c:
        tst.b   0x3b(a6)                        | +19c
        bne.w   .L02fe1a                        | +1a0
        jsr     0x283d8.l                       | +1a4
.L02fe1a:
        jsr     Slug_UpdateDamageSprite_02fae4__L02fb04(pc) | +1aa
        bcc.w   .L02fe28                        | +1ae
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +1b2
        move.l  a1,(a6)                         | +1b6
.L02fe28:
        jsr     Slug_ClampField92_02ffb0(pc)    | +1b8
        tst.b   0x92(a6)                        | +1bc
        bne.w   .L02fe46                        | +1c0
        clr.b   0x92(a6)                        | +1c4
        clr.w   0x36(a6)                        | +1c8
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +1cc
        move.l  a1,(a6)                         | +1d0
        bra.w   .L02fe4c                        | +1d2
.L02fe46:
        subi.b  #0x1,0x92(a6)                   | +1d6
.L02fe4c:
        cmpi.w  #0x117,0x22(a6)                 | +1dc
        blt.w   .L02fe5c                        | +1e2
        lea     Slug_SelfDestructAttack_02fe6a(pc),a1 | +1e6
        move.l  a1,(a6)                         | +1ea
.L02fe5c:
        clr.w   0x2a(a6)                        | +1ec
        clr.w   0x2e(a6)                        | +1f0

| ----------------------------------------------------------------------------
|  Slug_SelfDestructAttack_02fe6a  @ $02FE6A  (106 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_SelfDestructAttack_02fe6a, "ax", @progbits
        .global Slug_SelfDestructAttack_02fe6a
Slug_SelfDestructAttack_02fe6a:
        bclr    #0x5,0x8c(a6)                   | +000
        lea     .L02fe76(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L02fe76:
        jsr     Sub_0002AA24(pc)                | +00c
        bset    #0x4,0x8d(a6)                   | +010
        move.b  #0x1,0x10a2d1.l                 | +016
        lea     Sub_00029790(pc),a0             | +01e
        move.l  a0,0x4c(a6)                     | +022
        jsr     0x283ca.l                       | +026
        jsr     0x283ca.l                       | +02c
        jsr     0x283d8.l                       | +032
        lea     0xffff.w,a0                     | +038
        move.l  a0,0x4c(a6)                     | +03c
        jsr     0x283ca.l                       | +040
        jsr     Slug_ExplodeFx_02ff22(pc)       | +046
        lea     Slug_BlastAttack_02ff4a(pc),a1  | +04a
        jsr     0x4ae.l                         | +04e
        jsr     0x517fe.l                       | +054
        jsr     Sub_0002A752(pc)                | +05a
        jsr     0x28d70.l                       | +05e
        lea     Sub_0002DC5C(pc),a1             | +064
        move.l  a1,(a6)                         | +068

| ----------------------------------------------------------------------------
|  Slug_KillInit_02feda  @ $02FEDA  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_KillInit_02feda, "ax", @progbits
        .global Slug_KillInit_02feda
Slug_KillInit_02feda:
        clr.b   0x59(a6)                        | +000
        move.b  #0xfe,0x45(a6)                  | +004
        clr.w   0x66(a6)                        | +00a
        bset    #0x0,0x13(a6)                   | +00e
        jsr     0x13600.l                       | +014
        move.w  #0x19c,d1                       | +01a
        jsr     0x236e.l                        | +01e
        move.w  #0x19d,d1                       | +024
        jsr     0x236e.l                        | +028
        move.w  #0x19e,d1                       | +02e
        jsr     0x236e.l                        | +032
        move.w  #0x19a,d1                       | +038
        jsr     0x236e.l                        | +03c

| ----------------------------------------------------------------------------
|  Slug_ExplodeFx_02ff22  @ $02FF22  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ExplodeFx_02ff22, "ax", @progbits
        .global Slug_ExplodeFx_02ff22
Slug_ExplodeFx_02ff22:
        move.w  #0x1034,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     SlugFx_Smoke_02f64c__L02f672(pc),a1 | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        jsr     0x8189c.l                       | +01a

| ----------------------------------------------------------------------------
|  Slug_BlastAttack_02ff4a  @ $02FF4A  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_BlastAttack_02ff4a, "ax", @progbits
        .global Slug_BlastAttack_02ff4a
Slug_BlastAttack_02ff4a:
        move.w  #0xa0,0x22(a6)                  | +000
        move.w  #0x180,0x24(a6)                 | +006
        lea     Sub_000298D8(pc),a0             | +00c
        move.l  a0,0x4c(a6)                     | +010
        jsr     0x283ca.l                       | +014
        jsr     0x283ca.l                       | +01a
        jsr     0x283d8.l                       | +020
        lea     0xffff.w,a0                     | +026
        move.l  a0,0x4c(a6)                     | +02a
        jsr     0x283ca.l                       | +02e

| ----------------------------------------------------------------------------
|  Slug_AngleIndex_02ff8e  @ $02FF8E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_AngleIndex_02ff8e, "ax", @progbits
        .global Slug_AngleIndex_02ff8e
Slug_AngleIndex_02ff8e:
        lea     0x100580.l,a0                   | +000
        move.w  0x80(a0),d2                     | +006
        move.w  d2,d0                           | +00a
        asr.w   #0x1,d0                         | +00c
        neg.w   d0                              | +00e
        addi.w  #0x20,d0                        | +010
        cmpi.w  #0x40,d0                        | +014
        bls.w   .L02ffae                        | +018
        move.w  #0x40,d0                        | +01c
.L02ffae:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Slug_ClampField92_02ffb0  @ $02FFB0  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ClampField92_02ffb0, "ax", @progbits
        .global Slug_ClampField92_02ffb0
Slug_ClampField92_02ffb0:
        cmpi.w  #0xc0,0x22(a6)                  | +000
        blt.w   Slug_ClampField92_ClearXN_02ffe0 | +006
        movea.l #0xffffffff,a0                  | +00a
        jsr     0x5dd56.l                       | +010
        bcc.w   Slug_ClampField92_ClearXN_02ffe0 | +016
        cmpi.b  #0xa,0x92(a6)                   | +01a
        ble.w   Slug_ClampField92_ClearXN_02ffe0 | +020
        move.b  #0xa,0x92(a6)                   | +024

| ----------------------------------------------------------------------------
|  Slug_ClampField92_ClearXN_02ffe0  @ $02FFE0  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Slug_ClampField92_ClearXN_02ffe0, "ax", @progbits
        .global Slug_ClampField92_ClearXN_02ffe0
Slug_ClampField92_ClearXN_02ffe0:
        andi.b  #0xee,ccr                       | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Entity_CmpField10WithLink8_02ffe6  @ $02FFE6  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpField10WithLink8_02ffe6, "ax", @progbits
        .global Entity_CmpField10WithLink8_02ffe6
Entity_CmpField10WithLink8_02ffe6:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   Entity_CmpField10WithLink8_SetXN_02fffc | +00c

| ----------------------------------------------------------------------------
|  Entity_CmpField10WithLink8_SetXN_02fffc  @ $02FFFC  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Entity_CmpField10WithLink8_SetXN_02fffc, "ax", @progbits
        .global Entity_CmpField10WithLink8_SetXN_02fffc
Entity_CmpField10WithLink8_SetXN_02fffc:
        ori.b   #0x11,ccr                       | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Chain3_Init_030002  @ $030002  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_Init_030002, "ax", @progbits
        .global Chain3_Init_030002
Chain3_Init_030002:
        tst.b   0x98(a6)                        | +000
        bne.w   .L030010                        | +004
        bset    #0x2,0x5b(a6)                   | +008
.L030010:
        bset    #0x1,0x6b(a6)                   | +00e
        bset    #0x6,0x12(a6)                   | +014
        move.w  #0xf000,d0                      | +01a
        jsr     0x28134.l                       | +01e
        andi.w  #0xffe3,0x38(a6)                | +024
        ori.w   #0x10,0x38(a6)                  | +02a
        clr.w   0x2c(a6)                        | +030
        clr.w   0x2e(a6)                        | +034
        clr.w   0x8e(a6)                        | +038
        clr.b   0x5c(a6)                        | +03c
        jsr     0x27eba.l                       | +040
        bcc.w   .L030054                        | +046
        clr.b   0x84(a6)                        | +04a
        bra.w   .L03005a                        | +04e
.L030054:
        move.b  #0x1,0x84(a6)                   | +052
.L03005a:
        lea     0x29dc90.l,a0                   | +058

| ----------------------------------------------------------------------------
|  Chain3_TplA_030068  @ $030068  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_TplA_030068, "ax", @progbits
        .global Chain3_TplA_030068
Chain3_TplA_030068:
        move.w  #0x7,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,d1                         | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x8,d1                         | +014
        jsr     0x236e.l                        | +018
        move.w  #0x6,d1                         | +01e
        jsr     0x236e.l                        | +022
        jsr     Chain3_Init_030002(pc)          | +028
        lea     0x279668.l,a1                   | +02c
        move.w  0x2(a1),d0                      | +032
        move.w  d0,0x7c(a6)                     | +036
        add.w   0x22(a6),d0                     | +03a
        move.w  d0,0x22(a6)                     | +03e
        move.b  #0x7f,0x6a(a6)                  | +042
        move.w  #0x0,0x7e(a6)                   | +048
        bra.w   Chain3_Follow_0301f0            | +04e

| ----------------------------------------------------------------------------
|  Chain3_TplB_0300ba  @ $0300BA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_TplB_0300ba, "ax", @progbits
        .global Chain3_TplB_0300ba
Chain3_TplB_0300ba:
        move.w  #0x7,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,d1                         | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x8,d1                         | +014
        jsr     0x236e.l                        | +018
        move.w  #0x6,d1                         | +01e
        jsr     0x236e.l                        | +022
        jsr     Chain3_Init_030002(pc)          | +028
        lea     0x27966e.l,a1                   | +02c
        move.w  0x2(a1),d0                      | +032
        move.w  d0,0x7c(a6)                     | +036
        add.w   0x22(a6),d0                     | +03a
        move.w  d0,0x22(a6)                     | +03e
        move.b  #0xff,0x6a(a6)                  | +042
        move.w  #0x8000,0x7e(a6)                | +048
        bra.w   Chain3_Follow_0301f0            | +04e

| ----------------------------------------------------------------------------
|  Chain3_TplC_03010c  @ $03010C  (220 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_TplC_03010c, "ax", @progbits
        .global Chain3_TplC_03010c
Chain3_TplC_03010c:
        move.w  #0x7,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,d1                         | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x8,d1                         | +014
        jsr     0x236e.l                        | +018
        move.w  #0x6,d1                         | +01e
        jsr     0x236e.l                        | +022
        lea     0x279674.l,a1                   | +028
        move.w  0x2(a1),d0                      | +02e
        add.w   d0,0x22(a6)                     | +032
        move.w  d0,0x7c(a6)                     | +036
        jsr     Chain3_Init_030002(pc)          | +03a
        move.w  0x16(a6),0x14(a6)               | +03e
        lea     .L030156(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L030156:
        movea.l 0xc(a6),a0                      | +04a
        btst    #0x3,0x13(a0)                   | +04e
        beq.w   .L030168                        | +054
        clr.w   0x8e(a6)                        | +058
.L030168:
        move.w  0x22(a6),0x80(a6)               | +05c
        move.w  0x24(a6),0x82(a6)               | +062
        movea.l 0xc(a6),a2                      | +068
        move.w  0x82(a2),0x24(a6)               | +06c
        move.w  0x22(a2),0x22(a6)               | +072
        jsr     0x27eba.l                       | +078
        scc.b   d0                              | +07e
        andi.b  #0x1,d0                         | +080
        move.b  d0,0x84(a6)                     | +084
        move.w  0x24(a6),0x9a(a6)               | +088
        move.w  0x8e(a6),d0                     | +08e
        beq.w   .L0301ba                        | +092
        cmpi.w  #0x0,0x279666.l                 | +096
        bne.w   .L0301b6                        | +09e
        add.w   d0,0x9a(a6)                     | +0a2
        bra.w   .L0301ba                        | +0a6
.L0301b6:
        add.w   d0,0x24(a6)                     | +0aa
.L0301ba:
        movea.l 0x70(a6),a1                     | +0ae
        movea.l 0x74(a6),a2                     | +0b2
        move.w  0x24(a1),0x8a(a6)               | +0b6
        move.w  0x24(a2),0x8c(a6)               | +0bc
        jsr     Chain3_DebugHud_030554(pc)      | +0c2
        clr.w   0x8e(a6)                        | +0c6
        btst    #0x4,0x100001.l                 | +0ca
        beq.w   JsrAbsRts_0301ee                | +0d2
        move.w  #0xf000,0x38(a6)                | +0d6

| ----------------------------------------------------------------------------
|  Chain3_Follow_0301f0  @ $0301F0  (410 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_Follow_0301f0, "ax", @progbits
        .global Chain3_Follow_0301f0
Chain3_Follow_0301f0:
        lea     .L0301f6(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L0301f6:
        movea.l 0xc(a6),a0                      | +006
        btst    #0x3,0x13(a0)                   | +00a
        beq.w   .L030208                        | +010
        clr.w   0x8e(a6)                        | +014
.L030208:
        move.w  0x22(a6),0x80(a6)               | +018
        move.w  0x24(a6),0x82(a6)               | +01e
        jsr     0x2abcc.l                       | +024
        bcs.w   .L030224                        | +02a
        jsr     0x2783a.l                       | +02e
.L030224:
        jsr     Chain3_VelX_030462(pc)          | +034
        tst.b   0x84(a6)                        | +038
        beq.w   .L030294                        | +03c
        move.w  0x16(a6),0x14(a6)               | +040
        bsr.w   Chain3_CheckSyncB_03050c        | +046
        jsr     0x27902.l                       | +04a
        bcc.w   .L03024a                        | +050
        bset    #0x4,0x5a(a6)                   | +054
.L03024a:
        jsr     0x27eba.l                       | +05a
        bcc.w   .L030258                        | +060
        clr.b   0x84(a6)                        | +064
.L030258:
        movea.l 0xc(a6),a0                      | +068
        cmpi.w  #0xffff,0x34(a0)                | +06c
        bne.w   .L03026a                        | +072
        clr.b   0x84(a6)                        | +076
.L03026a:
        bsr.w   Chain3_YDelta_030392            | +07a
        bcc.w   .L030280                        | +07e
        clr.w   0x2a(a6)                        | +082
        clr.b   0x84(a6)                        | +086
        move.b  #0xff,0x5c(a6)                  | +08a
.L030280:
        tst.b   0x84(a6)                        | +090
        bne.w   .L030290                        | +094
        clr.w   0x28(a6)                        | +098
        bra.w   .L030294                        | +09c
.L030290:
        bra.w   .L03034e                        | +0a0
.L030294:
        movea.l 0xc(a6),a0                      | +0a4
        move.w  0x82(a0),d0                     | +0a8
        move.w  0x34(a0),d1                     | +0ac
        cmpi.w  #0xffff,d1                      | +0b0
        bne.w   .L0302e0                        | +0b4
        move.w  0x18(a6),0x14(a6)               | +0b8
        bsr.w   Chain3_VelY_030416              | +0be
        move.w  0x22(a6),d1                     | +0c2
        move.w  0x24(a6),d2                     | +0c6
        jsr     0x280c6.l                       | +0ca
        bcs.w   .L0302dc                        | +0d0
        move.w  0x22(a6),d1                     | +0d4
        move.w  0x24(a6),d2                     | +0d8
        subq.w  #0x4,d2                         | +0dc
        jsr     0x280c6.l                       | +0de
        bcs.w   .L0302dc                        | +0e4
        clr.b   0x5c(a6)                        | +0e8
.L0302dc:
        bra.w   .L030304                        | +0ec
.L0302e0:
        move.w  0x1a(a6),0x14(a6)               | +0f0
        bsr.w   Chain3_Step_0303ee              | +0f6
        sub.w   0x24(a6),d0                     | +0fa
        blt.w   .L0302f6                        | +0fe
        clr.b   0x5c(a6)                        | +102
.L0302f6:
        lsl.w   #0x6,d0                         | +106
        move.w  0x2a(a6),d1                     | +108
        asr.w   #0x1,d1                         | +10c
        sub.w   d1,d0                           | +10e
        add.w   d0,0x2a(a6)                     | +110
.L030304:
        tst.b   0x5c(a6)                        | +114
        beq.w   .L030316                        | +118
        jsr     0x27d50.l                       | +11c
        bra.w   .L03031c                        | +122
.L030316:
        jsr     0x27c2a.l                       | +126
.L03031c:
        bcc.w   .L03032e                        | +12c
        move.b  #0x1,0x84(a6)                   | +130
        clr.b   0x5c(a6)                        | +136
        bra.w   .L03034e                        | +13a
.L03032e:
        move.w  0x2a(a6),d0                     | +13e
        cmpi.w  #0x8,d0                         | +142
        bcc.w   .L03034e                        | +146
        jsr     0x27eba.l                       | +14a
        bcs.w   .L03034e                        | +150
        move.b  #0x1,0x84(a6)                   | +154
        clr.b   0x5c(a6)                        | +15a
.L03034e:
        move.w  0x24(a6),0x9a(a6)               | +15e
        move.w  0x8e(a6),d0                     | +164
        beq.w   .L030378                        | +168
        cmpi.w  #0x0,0x279666.l                 | +16c
        bne.w   .L030370                        | +174
        add.w   d0,0x9a(a6)                     | +178
        bra.w   .L030374                        | +17c
.L030370:
        add.w   d0,0x24(a6)                     | +180
.L030374:
        clr.w   0x8e(a6)                        | +184
.L030378:
        btst    #0x4,0x100001.l                 | +188
        beq.w   JsrAbsRts_030390                | +190
        move.w  #0xf000,0x38(a6)                | +194

| ----------------------------------------------------------------------------
|  Chain3_YDelta_030392  @ $030392  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_YDelta_030392, "ax", @progbits
        .global Chain3_YDelta_030392
Chain3_YDelta_030392:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x24(a6),d0                     | +004
        sub.w   0x82(a0),d0                     | +008
        cmpi.w  #0xffff,0x34(a0)                | +00c
        bne.w   .L0303d2                        | +012
        move.w  0x28(a6),d1                     | +016
        beq.w   .L0303c6                        | +01a
        move.w  0x7c(a6),d2                     | +01e
        eor.w   d1,d2                           | +022
        bpl.w   .L0303c6                        | +024
        cmpi.w  #0x0,d0                         | +028
        bge.w   .L0303e8                        | +02c
        bra.w   .L0303ce                        | +030
.L0303c6:
        cmpi.w  #0x18,d0                        | +034
        bge.w   .L0303e8                        | +038
.L0303ce:
        bra.w   .L0303e2                        | +03c
.L0303d2:
        cmpi.w  #0x12,d0                        | +040
        bgt.w   .L0303e8                        | +044
        cmpi.w  #0xffee,d0                      | +048
        blt.w   .L0303e8                        | +04c
.L0303e2:
        andi.b  #0xfe,ccr                       | +050
        rts                                     | +054
.L0303e8:
        ori.b   #0x1,ccr                        | +056
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Chain3_Step_0303ee  @ $0303EE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_Step_0303ee, "ax", @progbits
        .global Chain3_Step_0303ee
Chain3_Step_0303ee:
        tst.w   d1                              | +000
        beq.w   .L030414                        | +002
        move.w  0x7c(a6),d2                     | +006
        tst.w   d1                              | +00a
        bpl.w   .L030400                        | +00c
        neg.w   d2                              | +010
.L030400:
        subq.w  #0x1,d2                         | +012
        cmpi.w  #0x1000,d1                      | +014
        beq.b   .L030410                        | +018
        cmpi.w  #0xf000,d1                      | +01a
        bne.w   .L030412                        | +01e
.L030410:
        asr.w   #0x2,d2                         | +022
.L030412:
        add.w   d2,d0                           | +024
.L030414:
        rts                                     | +026

| ----------------------------------------------------------------------------
|  Chain3_VelY_030416  @ $030416  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_VelY_030416, "ax", @progbits
        .global Chain3_VelY_030416
Chain3_VelY_030416:
        movea.l 0xc(a6),a0                      | +000
        btst    #0x3,0x5b(a0)                   | +004
        beq.w   .L03042e                        | +00a
        subi.w  #0xc0,0x2a(a6)                  | +00e
        bra.w   .L030460                        | +014
.L03042e:
        move.w  0x2a(a0),0x2a(a6)               | +018
        movea.l 0x74(a6),a1                     | +01e
        movea.l 0x70(a6),a2                     | +022
        move.w  0x24(a1),d0                     | +026
        sub.w   0x24(a2),d0                     | +02a
        bpl.w   .L03044a                        | +02e
        neg.w   d0                              | +032
.L03044a:
        cmpi.w  #0x1c,d0                        | +034
        bcc.w   .L030460                        | +038
        move.w  0x82(a0),d0                     | +03c
        sub.w   0x24(a6),d0                     | +040
        lsl.w   #0x4,d0                         | +044
        add.w   d0,0x2a(a6)                     | +046
.L030460:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Chain3_VelX_030462  @ $030462  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_VelX_030462, "ax", @progbits
        .global Chain3_VelX_030462
Chain3_VelX_030462:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x22(a1),d1                     | +004
        add.w   0x7c(a6),d1                     | +008
        sub.w   0x22(a6),d1                     | +00c
        asl.w   #0x8,d1                         | +010
        move.w  d1,0x28(a6)                     | +012
        cmpi.w  #0xffff,0x7e(a6)                | +016
        beq.w   .L0304ae                        | +01c
        move.w  0x28(a1),d1                     | +020
        andi.w  #0x8000,d1                      | +024
        move.w  0x7e(a6),d2                     | +028
        eor.w   d2,d1                           | +02c
        bne.w   .L0304ae                        | +02e
        move.b  0x13(a1),d0                     | +032
        andi.b  #0x30,d0                        | +036
        move.b  #0x30,d1                        | +03a
        not.b   d1                              | +03e
        and.b   d1,0x13(a6)                     | +040
        or.b    d0,0x13(a6)                     | +044
        bra.w   .L0304c2                        | +048
.L0304ae:
        move.b  0x5b(a6),d1                     | +04c
        move.b  #0x30,d2                        | +050
        and.b   d2,d1                           | +054
        neg.b   d2                              | +056
        and.b   d2,0x13(a6)                     | +058
        or.b    d1,0x13(a6)                     | +05c
.L0304c2:
        rts                                     | +060

| ----------------------------------------------------------------------------
|  Chain3_CheckSyncA_0304c4  @ $0304C4  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_CheckSyncA_0304c4, "ax", @progbits
        .global Chain3_CheckSyncA_0304c4
Chain3_CheckSyncA_0304c4:
        movea.l 0x74(a6),a1                     | +000
        movea.l 0x78(a6),a2                     | +004
        move.w  0x24(a1),d1                     | +008
        cmp.w   0x24(a2),d1                     | +00c
        beq.w   .L0304ee                        | +010
        movea.l 0x78(a6),a3                     | +014
        move.w  0x8e(a3),d0                     | +018
        or.w    0x8e(a1),d0                     | +01c
        or.w    0x8e(a2),d0                     | +020
        tst.w   d0                              | +024
        beq.w   .L0304fe                        | +026
.L0304ee:
        bclr    #0x4,0x13(a6)                   | +02a
        bclr    #0x5,0x13(a6)                   | +030
        bra.w   .L03050a                        | +036
.L0304fe:
        bset    #0x4,0x13(a6)                   | +03a
        bset    #0x5,0x13(a6)                   | +040
.L03050a:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Chain3_CheckSyncB_03050c  @ $03050C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_CheckSyncB_03050c, "ax", @progbits
        .global Chain3_CheckSyncB_03050c
Chain3_CheckSyncB_03050c:
        movea.l 0x70(a6),a1                     | +000
        movea.l 0x74(a6),a2                     | +004
        move.w  0x82(a1),d1                     | +008
        cmp.w   0x82(a2),d1                     | +00c
        beq.w   .L030536                        | +010
        movea.l 0x78(a6),a3                     | +014
        move.w  0x8e(a3),d0                     | +018
        or.w    0x8e(a1),d0                     | +01c
        or.w    0x8e(a2),d0                     | +020
        tst.w   d0                              | +024
        beq.w   .L030546                        | +026
.L030536:
        bclr    #0x4,0x5b(a6)                   | +02a
        bclr    #0x5,0x5b(a6)                   | +030
        bra.w   .L030552                        | +036
.L030546:
        bset    #0x4,0x5b(a6)                   | +03a
        bset    #0x5,0x5b(a6)                   | +040
.L030552:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  Chain3_DebugHud_030554  @ $030554  (174 B)
| ----------------------------------------------------------------------------
        .section .text.Chain3_DebugHud_030554, "ax", @progbits
        .global Chain3_DebugHud_030554
Chain3_DebugHud_030554:
        btst    #0x4,0x100001.l                 | +000
        beq.w   JsrAbsRts_030608                | +008
        movea.l #0x7412,a1                      | +00c
        moveq   #0,d0                           | +012
        movea.l 0x70(a6),a2                     | +014
        move.w  0x8e(a2),d0                     | +018
        jsr     0x5d6c2.l                       | +01c
        movea.l #0x7413,a1                      | +022
        moveq   #0,d0                           | +028
        movea.l 0x74(a6),a2                     | +02a
        move.w  0x8e(a2),d0                     | +02e
        jsr     0x5d6c2.l                       | +032
        movea.l #0x7415,a1                      | +038
        moveq   #0,d0                           | +03e
        movea.l 0x70(a6),a2                     | +040
        move.w  0x22(a2),d0                     | +044
        jsr     0x5d6c2.l                       | +048
        movea.l #0x7416,a1                      | +04e
        moveq   #0,d0                           | +054
        move.w  0x22(a6),d0                     | +056
        jsr     0x5d6c2.l                       | +05a
        movea.l #0x7417,a1                      | +060
        moveq   #0,d0                           | +066
        movea.l 0x74(a6),a2                     | +068
        move.w  0x22(a2),d0                     | +06c
        jsr     0x5d6c2.l                       | +070
        movea.l #0x7419,a1                      | +076
        moveq   #0,d0                           | +07c
        movea.l 0x70(a6),a2                     | +07e
        move.w  0x24(a2),d0                     | +082
        jsr     0x5d6c2.l                       | +086
        movea.l #0x741a,a1                      | +08c
        moveq   #0,d0                           | +092
        move.w  0x24(a6),d0                     | +094
        jsr     0x5d6c2.l                       | +098
        movea.l #0x741b,a1                      | +09e
        moveq   #0,d0                           | +0a4
        movea.l 0x74(a6),a2                     | +0a6
        move.w  0x24(a2),d0                     | +0aa
