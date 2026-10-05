| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave TTT — Núcleo del jugador: armas/munición, input, spawn, idle
|  Región: $032A02..$0342C4  (5,346 B, 66 entradas, 38 huecos)
| ============================================================================
|
|  A) RESUMEN
|  ----------
|  Cluster del JUGADOR (entidad en a6 = slot $100440 P1 / $1004E0 P2, o su
|  padre +$C cuando la llamada viene de un hijo). Campos usados:
|   +$70 estado de anim, +$71 arma (0 = pistola, 4 = ?), +$72 modo, +$78/+$79
|   nibble de dirección/disparo, +$80 bombas, +$81/+$84/+$8B flags de evento
|   (consumidos a 0), +$82 munición (clamp 999 = $3E7), +$85, +$87 timer de
|   invulnerabilidad (parpadeo con tabla OpcodeOffsetTable_0329EE), +$88 bit0
|   (agachado), +$8C/+$8D bits de estado (bit2 = aire, bit4 = cambio de arma,
|   +$8D bit1 = en suelo), +$5C, +$60 = $32500.
|
|   1. Munición / armas ($32B58..$32C7E): Player_SetWeaponAndAmmo (d0 = jugador,
|      d1 = arma, d2 = munición; tabla Sub_000329D4 {$FFFF,10,999,10,999,15,
|      999,20,999,150} = munición por defecto por arma), Player_RefillAmmo
|      Clamp999, Item_GiveAmmo_ToPlayer (desde Item_GiveAmmoKind: +$98/+$99
|      del item, popup Score_Popup_ForPlayer_09b9ba; $FF = sin cambio),
|      Item_GiveBombs_ToPlayer (clamp 99). Player_GetAmmoOrFFFF/GetBombs/
|      DecBombs: lectores para el HUD.
|   2. Invulnerabilidad: Player_StartInvuln5 (+$87 = 5, +$8C bit4) y
|      Player_InvulnBlinkStep (decrementa +$87 y copia a +$14 el word indexado
|      por OpcodeOffsetTable_0329EE[+$87 & 15] -> paleta de parpadeo; al
|      acabar restaura +$16).
|   3. Música por arma: Player_WeaponMusicTable_032d28 {-1,$112F,$112C,$112E,
|      $112D} y Player_PlayWeaponMusicIfFlag (bit4 de +$8C -> $2352).
|      Player_PlayLifeMusic: $1123 (P1) / $1087 (P2).
|   4. Input (d0 = bits de pad): Input_ForwardByFacing/BackwardByFacing (bit0
|      de +$3A espeja JmpAbsThunk_032e3c=izq / Input_RightThunk=der),
|      Input_JumpOrFire ($5CF84 -> $5CEEC -> $5CF9C), Input_DownPressed,
|      Input_FireByMode/JumpByMode: según $106F2A (0/4 = layout alternativo,
|      $5CDB4) y devuelven d0 = campo+1 de InputLayout_ReadField2_05D5B6.
|      Player_ReadDirNibble/ReadFireNibble empaquetan el nibble previo << 4 |
|      actual en d0 (+$78).
|   5. Player_ActionSelect_0330d0 (y variante _B): decide la acción del
|      frame y devuelve C=1 + d1 = código: $FF = golpeado (+$13 bit1),
|      3 = disparo ($5CDC0), 4 = ..., 1/2 = por nibble (Player_DirNibble
|      ToAction), 0 = nada; carga el mapa de sprites Sub_000325E4 en +$4C.
|      Player_Idle_Tail_0341a4 lo consume: $FF -> Sub_00035D34 (hit),
|      3 -> Sub_000360BC (fire), 4 -> Sub_0003873C, 1 -> Sub_0003437E,
|      otro -> Sub_000342C4 (Wave siguiente).
|   6. Granadas: Player_ThrowGrenade_0332bc / _Back / _Down: si +$80 > 0 lo
|      decrementa, `lea JmpAbsThunk_033346(pc),a1; jsr $5EAB6` crea la
|      granada (Grenade_ThrowHeavy via jmp $28D876), copia pos ($5DD02) y
|      facing (espejado en _Back), $517FE, desplaza ±$10 en X y marca +$8C
|      bit2. Player_JmpGrenadeBounce/Down = `jmp $28D9DC/$28D7AA`.
|   7. Spawn: Player_SpawnStart_0336dc (pos $50/$1E0 + scroll, snd $177/
|      $190/$192, $138FE, PlayerEntity_InitAuxState, hijo $394E6) ->
|      Player_SpawnByMode (d2: 2 = Player_SpawnFall con paracaídas $279F8A,
|      1 -> Sub_00036C8C, otro -> Player_SpawnParachute $279B2C) ->
|      Player_SpawnLand/LandB -> Player_SpawnLand_Done (hijo $32112/$32142 por
|      jugador, +$45/+$59 = $50 de invulnerabilidad) -> Player_Idle.
|      Player_DeathGate_0334c6: si $106E92 == 0 marca muerte (+$13 bit0, HP
|      +$66 = 0); en la escena $106ECE == 1 con Y < $10C y $27DB2 -> d7 == $40
|      fuerza estado 5 (ahogado); cola jmp $28758.
|   8. Estados base: Player_Idle_033d64 (anim $279828 = $21, +$82 = 10 de
|      munición de pistola, decide Crouch (+$88 bit0), Reload (+$85/+$71),
|      CrouchB al azar ($5E9B6 & 7 == 4 con +$82 == 4), Sub_00034704 al
|      tocar suelo, Sub_00034B38/Sub_00034D32 por signo de vel X, Sub_000345B8,
|      TaskHandler_036d64 / Sub_00037018 por efecto $27EBA, Sub_00036914 por
|      disparo); Player_Crouch/CrouchB ($27973E/$27981E, anim 5),
|      Player_Reload ($2796F8/$279702, anim $33). Todos pasan por
|      Player_FrameCommon_032ff2 ($283CA, prio &= ~3, InvulnBlinkStep,
|      $2A720) y el suelo $5DD56 con hitbox Sub_000324C6/Sub_000324BC (escena 3).
|
|  B) EVIDENCIAS
|  -------------
|  * Player_StateTable68_03338a: 68 punteros -> 7 handlers en $37684..$37B00
|    (tabla de estados por +$70; PlayerStateLUT_03349A = {Player_*_03338a,
|    $033412} par P1/P2).
|  * Llamadas desde Item_GiveAmmoKind (RRR), Grenade_* (SSS) y
|    PlayerRoute_PublishState_033522 (NN).
|  * $033346..$033358 = tríada `jmp $28Dxxx.l` (ver player_grenade_18d1xx.s).
|
|  C) HIPÓTESIS / DUDAS
|  --------------------
|  * +$71: índice de arma (0 pistola, 1 HMG?, 2 shotgun?, 3 rocket?, 4 flame?)
|    — la tabla de munición {10,999,10,999,15,999,20,999,150} sugiere pares
|    {default, máximo}. Confirmar con los handlers $376xx.
|  * $106F2A: layout de botones (opción de servicio).
|  * Player_LinkRidePartner_032d6c: engancha un hijo (d2 = 1/2) a un jugador
|    que esté en el aire cayendo (+$2A < 0) — probable "rescate"/vehículo.
|
|  D) ESTRUCTURAS
|  --------------
|  Tabla de munición Sub_000329D4: words {arma0, (def,max) x4, 150}.
|  Player_WeaponMusicTable_032d28: 5 words (-1 = sin música).
|  Player_StateTable68_03338a: 68 x u32.
|
|  E) ISLAS ABSORBIDAS
|  -------------------
|  Ninguna; se corrigió el tamaño de PlayerRoute_PublishState_033522 (72 -> 80)
|  y el defsym Probe_Bit3At100001_End pasó a ser Player_DeathGate_0334c6.
|
|  F) ESTADO
|  ---------
|  66/66 entradas byte-exactas (5,354 B). 3 --data, 1 --entry, 2 labels __L.
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  Player_CheckDeathOrState21_032aa8  @ $032AA8  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CheckDeathOrState21_032aa8, "ax", @progbits
        .global Player_CheckDeathOrState21_032aa8
Player_CheckDeathOrState21_032aa8:
        jsr     0x28d70.l                       | +000
        bcs.w   SetXN_032ac2                    | +006
        cmpi.b  #0x0,0x21(a6)                   | +00a
        beq.w   SetXN_032ac2                    | +010

| ----------------------------------------------------------------------------
|  Player_PlaceAtScrollOffset_032ac8  @ $032AC8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Player_PlaceAtScrollOffset_032ac8, "ax", @progbits
        .global Player_PlaceAtScrollOffset_032ac8
Player_PlaceAtScrollOffset_032ac8:
        jsr     0x4cac4.l                       | +000
        cmpa.l  #0x100440,a6                    | +006
        bne.w   .L032ade                        | +00c
        add.w   d3,d0                           | +010
        bra.w   .L032ae0                        | +012
.L032ade:
        sub.w   d3,d0                           | +016
.L032ae0:
        move.w  d0,0x22(a6)                     | +018
        move.w  d1,0x24(a6)                     | +01c
        rts                                     | +020

| ----------------------------------------------------------------------------
|  Player_ConsumeFlag81_032aea  @ $032AEA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ConsumeFlag81_032aea, "ax", @progbits
        .global Player_ConsumeFlag81_032aea
Player_ConsumeFlag81_032aea:
        move.b  #0x1,d1                         | +000
        move.b  0x81(a6),d0                     | +004
        clr.b   0x81(a6)                        | +008
        cmp.b   d0,d1                           | +00c
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Player_ConsumeFireFlag84_032b1c  @ $032B1C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ConsumeFireFlag84_032b1c, "ax", @progbits
        .global Player_ConsumeFireFlag84_032b1c
Player_ConsumeFireFlag84_032b1c:
        lea     Sub_000329E8(pc),a1             | +000
        moveq   #0,d0                           | +004
        move.b  0x71(a6),d0                     | +006
        move.b  (a1,d0.w),d1                    | +00a
        move.b  0x84(a6),d0                     | +00e
        clr.b   0x84(a6)                        | +012
        cmp.b   d0,d1                           | +016
        rts                                     | +018

| ----------------------------------------------------------------------------
|  Player_SetWeaponAndAmmo_032b58  @ $032B58  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SetWeaponAndAmmo_032b58, "ax", @progbits
        .global Player_SetWeaponAndAmmo_032b58
Player_SetWeaponAndAmmo_032b58:
        movem.w d1,-(a7)                        | +000
        jsr     Player_SetAmmoForWeapon_032b6a(pc) | +004
        movem.w (a7)+,d1                        | +008
        move.b  d1,0x71(a1)                     | +00c
        rts                                     | +010

| ----------------------------------------------------------------------------
|  Player_SetAmmoForWeapon_032b6a  @ $032B6A  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SetAmmoForWeapon_032b6a, "ax", @progbits
        .global Player_SetAmmoForWeapon_032b6a
Player_SetAmmoForWeapon_032b6a:
        andi.w  #0xff,d2                        | +000
        cmpi.b  #0x0,d0                         | +004
        bne.w   .L032b80                        | +008
        lea     0x100440.l,a1                   | +00c
        bra.w   .L032b86                        | +012
.L032b80:
        lea     0x1004e0.l,a1                   | +016
.L032b86:
        andi.w  #0xff,d1                        | +01c
        lea     Sub_000329D4(pc),a2             | +020
        cmp.b   0x71(a1),d1                     | +024
        bne.w   .L032bb2                        | +028
        cmpi.b  #0x0,d2                         | +02c
        beq.w   .L032ba4                        | +030
        lsl.w   #0x2,d1                         | +034
        move.w  0x2(a2,d1.w),d2                 | +036
.L032ba4:
        move.w  0x82(a1),d0                     | +03a
        add.w   d2,d0                           | +03e
        move.w  d0,0x82(a1)                     | +040
        bra.w   .L032bc4                        | +044
.L032bb2:
        cmpi.b  #0x0,d2                         | +048
        bne.w   .L032bc0                        | +04c
        lsl.w   #0x2,d1                         | +050
        move.w  0x2(a2,d1.w),d2                 | +052
.L032bc0:
        move.w  d2,0x82(a1)                     | +056
.L032bc4:
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Player_RefillAmmoClamp999_032bc6  @ $032BC6  (76 B)
| ----------------------------------------------------------------------------
        .section .text.Player_RefillAmmoClamp999_032bc6, "ax", @progbits
        .global Player_RefillAmmoClamp999_032bc6
Player_RefillAmmoClamp999_032bc6:
        cmpi.b  #0x0,d0                         | +000
        bne.w   .L032bd8                        | +004
        lea     0x100440.l,a1                   | +008
        bra.w   .L032bde                        | +00e
.L032bd8:
        lea     0x1004e0.l,a1                   | +012
.L032bde:
        move.b  0x71(a1),d1                     | +018
        cmpi.b  #0x0,d1                         | +01c
        beq.w   .L032c10                        | +020
        andi.w  #0xff,d1                        | +024
        lea     Sub_000329D4(pc),a2             | +028
        lsl.w   #0x2,d1                         | +02c
        move.w  0x82(a1),d0                     | +02e
        add.w   0x2(a2,d1.w),d0                 | +032
        cmpi.w  #0x3e7,d0                       | +036
        ble.w   .L032c08                        | +03a
        move.w  #0x3e7,d0                       | +03e
.L032c08:
        move.w  d0,0x82(a1)                     | +042
        bra.w   .L032c10                        | +046
.L032c10:
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  Item_GiveAmmo_ToPlayer_032c12  @ $032C12  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Item_GiveAmmo_ToPlayer_032c12, "ax", @progbits
        .global Item_GiveAmmo_ToPlayer_032c12
Item_GiveAmmo_ToPlayer_032c12:
        cmpi.b  #0x0,d0                         | +000
        bne.w   .L032c24                        | +004
        lea     0x100440.l,a1                   | +008
        bra.w   .L032c2a                        | +00e
.L032c24:
        lea     0x1004e0.l,a1                   | +012
.L032c2a:
        move.b  0x71(a1),d1                     | +018
        cmpi.b  #0x4,d1                         | +01c
        bne.w   .L032c3e                        | +020
        move.b  0x99(a6),d2                     | +024
        bra.w   .L032c42                        | +028
.L032c3e:
        move.b  0x98(a6),d2                     | +02c
.L032c42:
        cmpi.b  #0xff,d2                        | +030
        beq.w   JsrAbsRts_032c7c                | +034
        cmpi.b  #0x0,d1                         | +038
        beq.w   JsrAbsThunk_032c76              | +03c
        move.w  0x82(a1),d0                     | +040
        moveq   #0,d1                           | +044
        move.b  d2,d1                           | +046
        add.w   d1,d0                           | +048
        cmpi.w  #0x3e7,d0                       | +04a
        ble.w   .L032c68                        | +04e
        move.w  #0x3e7,d0                       | +052
.L032c68:
        move.w  d0,0x82(a1)                     | +056
        jsr     0x9b9ba.l                       | +05a
        bra.w   JsrAbsRts_032c7c                | +060

| ----------------------------------------------------------------------------
|  Item_GiveBombs_ToPlayer_032c7e  @ $032C7E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Item_GiveBombs_ToPlayer_032c7e, "ax", @progbits
        .global Item_GiveBombs_ToPlayer_032c7e
Item_GiveBombs_ToPlayer_032c7e:
        cmpi.b  #0x0,d0                         | +000
        bne.w   .L032c90                        | +004
        lea     0x100440.l,a1                   | +008
        bra.w   .L032c96                        | +00e
.L032c90:
        lea     0x1004e0.l,a1                   | +012
.L032c96:
        cmpi.b  #0xff,0x98(a6)                  | +018
        beq.w   .L032cb8                        | +01e
        move.b  0x80(a1),d0                     | +022
        add.b   0x98(a6),d0                     | +026
        cmpi.w  #0x63,d0                        | +02a
        ble.w   .L032cb4                        | +02e
        move.w  #0x63,d0                        | +032
.L032cb4:
        move.b  d0,0x80(a1)                     | +036
.L032cb8:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Player_StartInvuln5_032cba  @ $032CBA  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Player_StartInvuln5_032cba, "ax", @progbits
        .global Player_StartInvuln5_032cba
Player_StartInvuln5_032cba:
        move.b  #0x5,0x87(a1)                   | +000
        bset    #0x4,0x8c(a1)                   | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Player_InvulnBlinkStep_032cc8  @ $032CC8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Player_InvulnBlinkStep_032cc8, "ax", @progbits
        .global Player_InvulnBlinkStep_032cc8
Player_InvulnBlinkStep_032cc8:
        cmpi.b  #0x0,0x87(a6)                   | +000
        beq.w   .L032cf8                        | +006
        subi.b  #0x1,0x87(a6)                   | +00a
        moveq   #0,d1                           | +010
        move.b  0x87(a6),d1                     | +012
        andi.w  #0xf,d1                         | +016
        lsl.w   #0x1,d1                         | +01a
        lea     OpcodeOffsetTable_0329EE(pc),a1 | +01c
        move.w  (a1,d1.w),d0                    | +020
        move.w  (a6,d0.w),d1                    | +024
        move.w  d1,0x14(a6)                     | +028
        bra.w   .L032cfe                        | +02c
.L032cf8:
        move.w  0x16(a6),0x14(a6)               | +030
.L032cfe:
        rts                                     | +036

| ----------------------------------------------------------------------------
|  Player_WeaponMusicTable_032d28  @ $032D28  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Player_WeaponMusicTable_032d28, "ax", @progbits
        .global Player_WeaponMusicTable_032d28
Player_WeaponMusicTable_032d28:
        .dc.w   0xffff                        | +000  (dato / opcode no decodificado)
        .dc.w   0x112f                        | +002  (dato / opcode no decodificado)
        .dc.w   0x112c                        | +004  (dato / opcode no decodificado)
        .dc.w   0x112e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x112d                        | +008  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_PlayWeaponMusicIfFlag_032d32  @ $032D32  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Player_PlayWeaponMusicIfFlag_032d32, "ax", @progbits
        .global Player_PlayWeaponMusicIfFlag_032d32
Player_PlayWeaponMusicIfFlag_032d32:
        btst    #0x4,0x8c(a6)                   | +000
        beq.w   ClearXN_032d66                  | +006
        bclr    #0x4,0x8c(a6)                   | +00a
        moveq   #0,d0                           | +010
        move.b  0x71(a6),d0                     | +012
        lsl.w   #0x1,d0                         | +016
        lea     Player_WeaponMusicTable_032d28(pc),a0 | +018
        move.w  (a0,d0.w),d0                    | +01c
        cmpi.w  #0xffff,d0                      | +020
        beq.w   SetXN_032d60                    | +024
        jsr     0x2352.l                        | +028

| ----------------------------------------------------------------------------
|  Player_LinkRidePartner_032d6c  @ $032D6C  (104 B)
| ----------------------------------------------------------------------------
        .section .text.Player_LinkRidePartner_032d6c, "ax", @progbits
        .global Player_LinkRidePartner_032d6c
Player_LinkRidePartner_032d6c:
        cmpi.b  #0x1,d2                         | +000
        bne.w   .L032d7e                        | +004
        lea     0x100440.l,a0                   | +008
        bra.w   .L032d94                        | +00e
.L032d7e:
        cmpi.b  #0x2,d2                         | +012
        bne.w   .L032d90                        | +016
        lea     0x1004e0.l,a0                   | +01a
        bra.w   .L032d94                        | +020
.L032d90:
        bra.w   ClearXN_032dda                  | +024
.L032d94:
        btst    #0x0,0x8c(a0)                   | +028
        bne.w   ClearXN_032dda                  | +02e
        btst    #0x5,0x8d(a0)                   | +032
        beq.w   ClearXN_032dda                  | +038
        btst    #0x1,0x8d(a0)                   | +03c
        beq.w   ClearXN_032dda                  | +042
        cmpi.w  #0x0,0x2a(a0)                   | +046
        bge.w   ClearXN_032dda                  | +04c
        move.b  d2,0x6d(a6)                     | +050
        bset    #0x0,0x8c(a0)                   | +054
        move.b  0x68(a0),d2                     | +05a
        move.b  d2,0x68(a6)                     | +05e
        jsr     0x2aca2.l                       | +062

| ----------------------------------------------------------------------------
|  Player_IsSlotRidingReady_032de0  @ $032DE0  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Player_IsSlotRidingReady_032de0, "ax", @progbits
        .global Player_IsSlotRidingReady_032de0
Player_IsSlotRidingReady_032de0:
        cmpa.l  #0x100440,a0                    | +000
        beq.b   .L032df2                        | +006
        cmpa.l  #0x1004e0,a0                    | +008
        bne.w   ClearXN_032e02                  | +00e
.L032df2:
        btst    #0x6,0x8d(a0)                   | +012
        beq.w   ClearXN_032e02                  | +018

| ----------------------------------------------------------------------------
|  Player_SlotBit1Test_032e08  @ $032E08  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SlotBit1Test_032e08, "ax", @progbits
        .global Player_SlotBit1Test_032e08
Player_SlotBit1Test_032e08:
        btst    #0x1,0x8d(a0)                   | +000
        beq.w   ClearXN_032e1a                  | +006
        ori.b   #0x11,ccr                       | +00a
        bra.w   ClearXNMid_032e1e               | +00e

| ----------------------------------------------------------------------------
|  Player_Consume8B_032e20  @ $032E20  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Consume8B_032e20, "ax", @progbits
        .global Player_Consume8B_032e20
Player_Consume8B_032e20:
        tst.b   0x8b(a6)                        | +000
        beq.w   .L032e2c                        | +004
        clr.b   0x8b(a6)                        | +008
.L032e2c:
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Player_Consume8B_Alt_032e2e  @ $032E2E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Consume8B_Alt_032e2e, "ax", @progbits
        .global Player_Consume8B_Alt_032e2e
Player_Consume8B_Alt_032e2e:
        tst.b   0x8b(a6)                        | +000
        beq.w   .L032e3a                        | +004
        clr.b   0x8b(a6)                        | +008
.L032e3a:
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Input_RightThunk_032e42  @ $032E42  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Input_RightThunk_032e42, "ax", @progbits
        .global Input_RightThunk_032e42
Input_RightThunk_032e42:
        jmp     0x5cf10.l                       | +000

| ----------------------------------------------------------------------------
|  Input_ForwardByFacing_032e48  @ $032E48  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Input_ForwardByFacing_032e48, "ax", @progbits
        .global Input_ForwardByFacing_032e48
Input_ForwardByFacing_032e48:
        btst    #0x0,0x3a(a6)                   | +000
        bne.w   .L032e56                        | +006
        jmp     JmpAbsThunk_032e3c(pc)          | +00a
.L032e56:
        jmp     Input_RightThunk_032e42(pc)     | +00e

| ----------------------------------------------------------------------------
|  Input_BackwardByFacing_032e5a  @ $032E5A  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Input_BackwardByFacing_032e5a, "ax", @progbits
        .global Input_BackwardByFacing_032e5a
Input_BackwardByFacing_032e5a:
        btst    #0x0,0x3a(a6)                   | +000
        bne.w   .L032e68                        | +006
        jmp     Input_RightThunk_032e42(pc)     | +00a
.L032e68:
        jmp     JmpAbsThunk_032e3c(pc)          | +00e

| ----------------------------------------------------------------------------
|  Input_JumpThunk_032e6c  @ $032E6C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Input_JumpThunk_032e6c, "ax", @progbits
        .global Input_JumpThunk_032e6c
Input_JumpThunk_032e6c:
        jmp     Input_JumpOrFire_032e70(pc)     | +000

| ----------------------------------------------------------------------------
|  Input_JumpOrFire_032e70  @ $032E70  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Input_JumpOrFire_032e70, "ax", @progbits
        .global Input_JumpOrFire_032e70
Input_JumpOrFire_032e70:
        jsr     0x5cf84.l                       | +000
        bcc.w   .L032e7c                        | +006
        rts                                     | +00a
.L032e7c:
        jsr     0x5ceec.l                       | +00c
        bcc.w   .L032e8c                        | +012
        jmp     0x5cf9c.l                       | +016
.L032e8c:
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  Input_DownPressed_032e90  @ $032E90  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Input_DownPressed_032e90, "ax", @progbits
        .global Input_DownPressed_032e90
Input_DownPressed_032e90:
        jsr     JmpAbsThunk_0330ca(pc)          | +000
        bcc.w   ClearXN_032e9e                  | +004

| ----------------------------------------------------------------------------
|  Player_ClearBoth8D_032ea4  @ $032EA4  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ClearBoth8D_032ea4, "ax", @progbits
        .global Player_ClearBoth8D_032ea4
Player_ClearBoth8D_032ea4:
        lea     0x100440.l,a0                   | +000
        clr.b   0x8d(a0)                        | +006
        lea     0x1004e0.l,a0                   | +00a
        clr.b   0x8d(a0)                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Player_ClearBoth88_032eba  @ $032EBA  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ClearBoth88_032eba, "ax", @progbits
        .global Player_ClearBoth88_032eba
Player_ClearBoth88_032eba:
        lea     0x100440.l,a0                   | +000
        clr.b   0x88(a0)                        | +006
        lea     0x1004e0.l,a0                   | +00a
        clr.b   0x88(a0)                        | +010
        rts                                     | +014

| ----------------------------------------------------------------------------
|  Player_PlayLifeMusic_032ed0  @ $032ED0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Player_PlayLifeMusic_032ed0, "ax", @progbits
        .global Player_PlayLifeMusic_032ed0
Player_PlayLifeMusic_032ed0:
        movem.l a0,-(a7)                        | +000
        movea.l 0xc(a6),a0                      | +004
        cmpa.l  #0x100440,a0                    | +008
        bne.w   .L032ef0                        | +00e
        move.w  #0x1123,d0                      | +012
        jsr     0x2352.l                        | +016
        bra.w   .L032efa                        | +01c
.L032ef0:
        move.w  #0x1087,d0                      | +020
        jsr     0x2352.l                        | +024
.L032efa:
        movem.l (a7)+,a0                        | +02a
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  Player_EffectHit40or80_032f00  @ $032F00  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Player_EffectHit40or80_032f00, "ax", @progbits
        .global Player_EffectHit40or80_032f00
Player_EffectHit40or80_032f00:
        jsr     0x27eba.l                       | +000
        andi.b  #0xc0,d0                        | +006
        cmpi.b  #0x40,d0                        | +00a
        beq.w   Stub_00032F1C                   | +00e
        cmpi.b  #0x80,d0                        | +012
        beq.w   Stub_00032F1C                   | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Player_EffectHit40or80_B_032f1e  @ $032F1E  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Player_EffectHit40or80_B_032f1e, "ax", @progbits
        .global Player_EffectHit40or80_B_032f1e
Player_EffectHit40or80_B_032f1e:
        jsr     0x27eba.l                       | +000
        andi.b  #0xc0,d0                        | +006
        cmpi.b  #0x40,d0                        | +00a
        beq.w   Stub_00032F3A                   | +00e
        cmpi.b  #0x80,d0                        | +012
        beq.w   Stub_00032F3A                   | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Player_ReadDirNibble_032f3c  @ $032F3C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ReadDirNibble_032f3c, "ax", @progbits
        .global Player_ReadDirNibble_032f3c
Player_ReadDirNibble_032f3c:
        move.b  0x78(a6),d0                     | +000
        movem.w d0,-(a7)                        | +004
        move.b  #0x1,0x78(a6)                   | +008
        jsr     Input_ForwardByFacing_032e48(pc) | +00e
        bcc.w   .L032f58                        | +012
        move.b  #0x1,0x78(a6)                   | +016
.L032f58:
        jsr     0x5cef8.l                       | +01c
        bcc.w   .L032f68                        | +022
        move.b  #0x8,0x78(a6)                   | +026
.L032f68:
        jsr     0x5ceec.l                       | +02c
        bcc.w   .L032f78                        | +032
        move.b  #0x4,0x78(a6)                   | +036
.L032f78:
        movem.w (a7)+,d0                        | +03c
        lsl.b   #0x4,d0                         | +040
        or.b    0x78(a6),d0                     | +042

| ----------------------------------------------------------------------------
|  Player_ReadFireNibble_032f88  @ $032F88  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ReadFireNibble_032f88, "ax", @progbits
        .global Player_ReadFireNibble_032f88
Player_ReadFireNibble_032f88:
        move.b  0x78(a6),d0                     | +000
        movem.w d0,-(a7)                        | +004
        jsr     0x5ceec.l                       | +008
        bcc.w   .L032fa4                        | +00e
        move.b  #0x4,0x78(a6)                   | +012
        bra.w   .L032faa                        | +018
.L032fa4:
        move.b  #0x1,0x78(a6)                   | +01c
.L032faa:
        movem.w (a7)+,d0                        | +022
        lsl.b   #0x4,d0                         | +026
        or.b    0x78(a6),d0                     | +028

| ----------------------------------------------------------------------------
|  Player_GetAmmoOrFFFF_032fba  @ $032FBA  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GetAmmoOrFFFF_032fba, "ax", @progbits
        .global Player_GetAmmoOrFFFF_032fba
Player_GetAmmoOrFFFF_032fba:
        move.w  0x82(a0),d0                     | +000
        cmpi.b  #0x0,0x71(a0)                   | +004
        bne.w   .L032fcc                        | +00a
        move.w  #0xffff,d0                      | +00e
.L032fcc:
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Player_GetBombs_032fce  @ $032FCE  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_GetBombs_032fce, "ax", @progbits
        .global Player_GetBombs_032fce
Player_GetBombs_032fce:
        move.b  0x80(a0),d0                     | +000
        rts                                     | +004

| ----------------------------------------------------------------------------
|  Player_DecBombs_032fd4  @ $032FD4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DecBombs_032fd4, "ax", @progbits
        .global Player_DecBombs_032fd4
Player_DecBombs_032fd4:
        moveq   #1,d1                           | +000
        move.b  0x80(a0),d0                     | +002
        beq.w   Player_DecBombs_Zero_032fe8     | +006
        sub.w   d1,0x80(a0)                     | +00a

| ----------------------------------------------------------------------------
|  Player_DecBombs_Zero_032fe8  @ $032FE8  (2 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DecBombs_Zero_032fe8, "ax", @progbits
        .global Player_DecBombs_Zero_032fe8
Player_DecBombs_Zero_032fe8:
        clr.b   d1                              | +000

| ----------------------------------------------------------------------------
|  Player_FrameCommon_032ff2  @ $032FF2  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Player_FrameCommon_032ff2, "ax", @progbits
        .global Player_FrameCommon_032ff2
Player_FrameCommon_032ff2:
        jsr     Stub_00032FF0(pc)               | +000
        jsr     0x283ca.l                       | +004
        andi.w  #0xfffc,0x38(a6)                | +00a
        jsr     Player_InvulnBlinkStep_032cc8(pc) | +010
        jsr     0x2a720.l                       | +014
        jsr     Stub_00032E8E(pc)               | +01a

| ----------------------------------------------------------------------------
|  Player_FrameCommon_NoPrio_033016  @ $033016  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Player_FrameCommon_NoPrio_033016, "ax", @progbits
        .global Player_FrameCommon_NoPrio_033016
Player_FrameCommon_NoPrio_033016:
        jsr     Stub_00032FF0(pc)               | +000
        jsr     0x283ca.l                       | +004
        jsr     Player_InvulnBlinkStep_032cc8(pc) | +00a
        jsr     0x2a720.l                       | +00e
        jsr     Stub_00032E8E(pc)               | +014

| ----------------------------------------------------------------------------
|  Input_FireByMode_033034  @ $033034  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Input_FireByMode_033034, "ax", @progbits
        .global Input_FireByMode_033034
Input_FireByMode_033034:
        cmpi.b  #0x0,0x106f2a.l                 | +000
        bne.w   .L03304a                        | +008
        jsr     0x5cdb4.l                       | +00c
        bra.w   .L033066                        | +012
.L03304a:
        cmpi.b  #0x4,0x106f2a.l                 | +016
        bne.w   .L033060                        | +01e
        jsr     0x5cdb4.l                       | +022
        bra.w   .L033066                        | +028
.L033060:
        jsr     0x5ceec.l                       | +02c
.L033066:
        bcs.w   Input_FireByMode_Hit_033072     | +032
        moveq   #0,d0                           | +036

| ----------------------------------------------------------------------------
|  Input_FireByMode_Hit_033072  @ $033072  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Input_FireByMode_Hit_033072, "ax", @progbits
        .global Input_FireByMode_Hit_033072
Input_FireByMode_Hit_033072:
        jsr     0x5d5b6.l                       | +000
        addq.w  #0x1,d0                         | +006

| ----------------------------------------------------------------------------
|  Input_JumpByMode_033080  @ $033080  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Input_JumpByMode_033080, "ax", @progbits
        .global Input_JumpByMode_033080
Input_JumpByMode_033080:
        cmpi.b  #0x0,0x106f2a.l                 | +000
        bne.w   .L033096                        | +008
        jsr     0x5cdb4.l                       | +00c
        bra.w   .L0330b0                        | +012
.L033096:
        cmpi.b  #0x4,0x106f2a.l                 | +016
        bne.w   .L0330ac                        | +01e
        jsr     0x5cdb4.l                       | +022
        bra.w   .L0330b0                        | +028
.L0330ac:
        jsr     Input_JumpThunk_032e6c(pc)      | +02c
.L0330b0:
        bcs.w   Input_JumpByMode_Hit_0330bc     | +030
        moveq   #0,d0                           | +034

| ----------------------------------------------------------------------------
|  Input_JumpByMode_Hit_0330bc  @ $0330BC  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Input_JumpByMode_Hit_0330bc, "ax", @progbits
        .global Input_JumpByMode_Hit_0330bc
Input_JumpByMode_Hit_0330bc:
        jsr     0x5d5b6.l                       | +000
        addq.w  #0x1,d0                         | +006

| ----------------------------------------------------------------------------
|  Player_ActionSelect_0330d0  @ $0330D0  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ActionSelect_0330d0, "ax", @progbits
        .global Player_ActionSelect_0330d0
Player_ActionSelect_0330d0:
        btst    #0x2,0x8c(a6)                   | +000
        beq.w   .L0330de                        | +006
        jmp     .L033162(pc)                    | +00a
.L0330de:
        jsr     0x5cef8.l                       | +00e
        bcs.w   Player_ActionSelect_B_Fire_0331ce | +014
        jsr     Player_ConsumeFireFlag84_032b1c(pc) | +018
        bcs.w   .L033148                        | +01c
        jsr     0x5cda8.l                       | +020
        bcc.w   .L033148                        | +026
        lea     Sub_000325E4(pc),a0             | +02a
        move.l  a0,0x4c(a6)                     | +02e
        jsr     0x283ca.l                       | +032
        jsr     0x283ca.l                       | +038
        jsr     0x283d8.l                       | +03e
        btst    #0x1,0x13(a6)                   | +044
        beq.w   .L033126                        | +04a
        move.b  #0xff,d1                        | +04e
        bra.w   SetXN_03316c                    | +052
.L033126:
        cmpi.w  #0x0,0x82(a6)                   | +056
        bne.b   .L033138                        | +05c
        cmpi.b  #0x0,0x71(a6)                   | +05e
        bne.w   .L033144                        | +064
.L033138:
        jsr     Player_DirNibbleToAction_033172(pc) | +068
        bra.w   SetXN_03316c                    | +06c
        bra.w   .L033148                        | +070
.L033144:
        bra.w   .L033162                        | +074
.L033148:
        jsr     Player_ConsumeFlag81_032aea(pc) | +078
        bcs.w   .L033162                        | +07c
        jsr     0x5cdc0.l                       | +080
        bcc.w   .L033162                        | +086
        move.b  #0x3,d1                         | +08a
        jmp     SetXN_03316c(pc)                | +08e
.L033162:
        move.b  #0x0,d1                         | +092

| ----------------------------------------------------------------------------
|  Player_DirNibbleToAction_033172  @ $033172  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DirNibbleToAction_033172, "ax", @progbits
        .global Player_DirNibbleToAction_033172
Player_DirNibbleToAction_033172:
        cmpi.b  #0x4,0x78(a6)                   | +000
        bne.w   .L033184                        | +006
        move.b  #0x1,d1                         | +00a
        bra.w   .L0331ac                        | +00e
.L033184:
        cmpi.b  #0x8,0x78(a6)                   | +012
        bne.w   .L033196                        | +018
        move.b  #0x4,d1                         | +01c
        bra.w   .L0331ac                        | +020
.L033196:
        cmpi.b  #0x2,0x78(a6)                   | +024
        bne.w   .L0331a8                        | +02a
        move.b  #0x2,d1                         | +02e
        bra.w   .L0331ac                        | +032
.L0331a8:
        move.b  #0x0,d1                         | +036
.L0331ac:
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  Player_ActionSelect_B_0331ae  @ $0331AE  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ActionSelect_B_0331ae, "ax", @progbits
        .global Player_ActionSelect_B_0331ae
Player_ActionSelect_B_0331ae:
        btst    #0x2,0x8c(a6)                   | +000
        beq.w   .L0331bc                        | +006
        jmp     .L0331c6(pc)                    | +00a
.L0331bc:
        jsr     0x5cef8.l                       | +00e
        bcs.w   Player_ActionSelect_B_Fire_0331ce | +014
.L0331c6:
        clr.b   d1                              | +018

| ----------------------------------------------------------------------------
|  Player_ActionSelect_B_Fire_0331ce  @ $0331CE  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ActionSelect_B_Fire_0331ce, "ax", @progbits
        .global Player_ActionSelect_B_Fire_0331ce
Player_ActionSelect_B_Fire_0331ce:
        jsr     Player_ConsumeFlag81_032aea(pc) | +000
        bcs.w   .L0331e8                        | +004
        jsr     0x5cdc0.l                       | +008
        bcc.w   .L0331e8                        | +00e
        move.b  #0x3,d1                         | +012
        bra.w   SetXN_033250                    | +016
.L0331e8:
        jsr     Player_ConsumeFireFlag84_032b1c(pc) | +01a
        bcs.w   .L033248                        | +01e
        jsr     0x5cda8.l                       | +022
        bcc.w   .L033248                        | +028
        lea     Sub_000325E4(pc),a0             | +02c
        move.l  a0,0x4c(a6)                     | +030
        jsr     0x283ca.l                       | +034
        jsr     0x283ca.l                       | +03a
        jsr     0x283d8.l                       | +040
        btst    #0x1,0x13(a6)                   | +046
        beq.w   .L033226                        | +04c
        move.b  #0xff,d1                        | +050
        bra.w   SetXN_033250                    | +054
.L033226:
        cmpi.w  #0x0,0x82(a6)                   | +058
        bne.b   .L033238                        | +05e
        cmpi.b  #0x0,0x71(a6)                   | +060
        bne.w   .L033244                        | +066
.L033238:
        move.b  #0x4,d1                         | +06a
        bra.w   SetXN_033250                    | +06e
        bra.w   .L033248                        | +072
.L033244:
        bra.w   .L033248                        | +076
.L033248:
        clr.b   d1                              | +07a

| ----------------------------------------------------------------------------
|  Player_ThrowGrenadeBack_033256  @ $033256  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenadeBack_033256, "ax", @progbits
        .global Player_ThrowGrenadeBack_033256
Player_ThrowGrenadeBack_033256:
        movem.l a0,-(a7)                        | +000
        cmpa.l  0x100440.l,a6                   | +004
        beq.b   .L03326c                        | +00a
        cmpa.l  0x1004e0.l,a6                   | +00c
        bne.w   .L033272                        | +012
.L03326c:
        movea.l a6,a0                           | +016
        bra.w   .L033276                        | +018
.L033272:
        movea.l 0xc(a6),a0                      | +01c
.L033276:
        cmpi.b  #0x0,0x80(a0)                   | +020
        beq.w   .L0332b4                        | +026
        subi.b  #0x1,0x80(a0)                   | +02a
        lea     JmpAbsThunk_033346(pc),a1       | +030
        jsr     0x5eab6.l                       | +034
        jsr     0x5dd02.l                       | +03a
        move.b  0x3a(a6),d0                     | +040
        eori.b  #0x1,d0                         | +044
        move.b  d0,0x3a(a0)                     | +048
        jsr     0x517fe.l                       | +04c
        addi.w  #0x10,0x22(a0)                  | +052
        bset    #0x2,0x8c(a6)                   | +058
.L0332b4:
        movem.l (a7)+,a0                        | +05e
        rts                                     | +062

| ----------------------------------------------------------------------------
|  Player_ThrowGrenade_0332bc  @ $0332BC  (96 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenade_0332bc, "ax", @progbits
        .global Player_ThrowGrenade_0332bc
Player_ThrowGrenade_0332bc:
        movem.l a0,-(a7)                        | +000
        cmpa.l  0x100440.l,a6                   | +004
        beq.b   .L0332d2                        | +00a
        cmpa.l  0x1004e0.l,a6                   | +00c
        bne.w   .L0332d8                        | +012
.L0332d2:
        movea.l a6,a0                           | +016
        bra.w   .L0332dc                        | +018
.L0332d8:
        movea.l 0xc(a6),a0                      | +01c
.L0332dc:
        cmpi.b  #0x0,0x80(a0)                   | +020
        beq.w   .L033316                        | +026
        subi.b  #0x1,0x80(a0)                   | +02a
        lea     JmpAbsThunk_033346(pc),a1       | +030
        jsr     0x5eab6.l                       | +034
        jsr     0x5dd02.l                       | +03a
        move.b  0x3a(a6),d0                     | +040
        move.b  d0,0x3a(a0)                     | +044
        jsr     0x517fe.l                       | +048
        subi.w  #0x10,0x22(a0)                  | +04e
        bset    #0x2,0x8c(a6)                   | +054
.L033316:
        movem.l (a7)+,a0                        | +05a
        rts                                     | +05e

| ----------------------------------------------------------------------------
|  Player_ThrowGrenadeDown_03331c  @ $03331C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Player_ThrowGrenadeDown_03331c, "ax", @progbits
        .global Player_ThrowGrenadeDown_03331c
Player_ThrowGrenadeDown_03331c:
        lea     Player_JmpGrenadeDown_033352(pc),a1 | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x3a(a6),d0                     | +010
        eori.b  #0x1,d0                         | +014
        move.b  d0,0x3a(a0)                     | +018
        jsr     0x517fe.l                       | +01c
        bset    #0x2,0x8c(a6)                   | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Player_JmpGrenadeBounce_03334c  @ $03334C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JmpGrenadeBounce_03334c, "ax", @progbits
        .global Player_JmpGrenadeBounce_03334c
Player_JmpGrenadeBounce_03334c:
        jmp     0x28d9dc.l                      | +000

| ----------------------------------------------------------------------------
|  Player_JmpGrenadeDown_033352  @ $033352  (6 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JmpGrenadeDown_033352, "ax", @progbits
        .global Player_JmpGrenadeDown_033352
Player_JmpGrenadeDown_033352:
        jmp     0x28d7aa.l                      | +000

| ----------------------------------------------------------------------------
|  Player_SpawnDebugTask_033358  @ $033358  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnDebugTask_033358, "ax", @progbits
        .global Player_SpawnDebugTask_033358
Player_SpawnDebugTask_033358:
        tst.b   0x10fd8f.l                      | +000
        beq.w   JsrAbsRts_033374                | +006
        lea     0x3d842.l,a1                    | +00a
        jsr     0x4ae.l                         | +010

| ----------------------------------------------------------------------------
|  Player_SpawnTask3D8FA_033376  @ $033376  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnTask3D8FA_033376, "ax", @progbits
        .global Player_SpawnTask3D8FA_033376
Player_SpawnTask3D8FA_033376:
        lea     0x3d8fa.l,a1                    | +000
        jsr     0x4ae.l                         | +006

| ----------------------------------------------------------------------------
|  Player_StateTable68_03338a  @ $03338A  (272 B)
| ----------------------------------------------------------------------------
        .section .text.Player_StateTable68_03338a, "ax", @progbits
        .global Player_StateTable68_03338a
Player_StateTable68_03338a:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +008  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +010  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +014  (dato / opcode no decodificado)
        .dc.w   0x7684                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +018  (dato / opcode no decodificado)
        .dc.w   0x78c6                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +020  (dato / opcode no decodificado)
        .dc.w   0x7b00                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +024  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +028  (dato / opcode no decodificado)
        .dc.w   0x7af2                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +030  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +034  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +038  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +040  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +044  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +048  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +050  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +054  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +058  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +060  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +064  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +068  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +070  (dato / opcode no decodificado)
        .dc.w   0x7a34                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +074  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +078  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +080  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +084  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +086  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +088  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +090  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +092  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +094  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +098  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +09a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +09c  (dato / opcode no decodificado)
        .dc.w   0x7684                        | +09e  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0a0  (dato / opcode no decodificado)
        .dc.w   0x78c6                        | +0a2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0a4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0a6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0a8  (dato / opcode no decodificado)
        .dc.w   0x78c6                        | +0aa  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0ac  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ae  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0b0  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0b2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0b4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0b6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0b8  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ba  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0bc  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0be  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0c0  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0c2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0c4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0c6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0c8  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ca  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0cc  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ce  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0d0  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0d2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0d4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0d6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0d8  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0da  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0dc  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0de  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0e0  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0e2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0e4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0e6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0e8  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ea  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0ec  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0ee  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0f0  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0f2  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0f4  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0f6  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0f8  (dato / opcode no decodificado)
        .dc.w   0x7a34                        | +0fa  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +0fc  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +0fe  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +100  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +102  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +104  (dato / opcode no decodificado)
        .dc.w   0x7778                        | +106  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +108  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +10a  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +10c  (dato / opcode no decodificado)
        .dc.w   0x7a3e                        | +10e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  PlayerStateLUT_03349A  @ $03349A  (8 B)
| ----------------------------------------------------------------------------
        .section .text.PlayerStateLUT_03349A, "ax", @progbits
        .global PlayerStateLUT_03349A
PlayerStateLUT_03349A:
        .dc.w   0x0003                        | +000  (dato / opcode no decodificado)
        .dc.w   0x338a                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0003                        | +004  (dato / opcode no decodificado)
        .dc.w   0x3412                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Player_DeathGate_0334c6  @ $0334C6  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Player_DeathGate_0334c6, "ax", @progbits
        .global Player_DeathGate_0334c6
Player_DeathGate_0334c6:
        tst.w   0x106e92.l                      | +000
        bne.w   .L0334da                        | +006
        bset    #0x0,0x13(a6)                   | +00a
        clr.w   0x66(a6)                        | +010
.L0334da:
        cmpi.b  #0x1,0x106ece.l                 | +014
        bne.w   .L03351c                        | +01c
        move.w  0x22(a6),d1                     | +020
        move.w  0x24(a6),d2                     | +024
        cmpi.w  #0x10c,d2                       | +028
        bge.w   .L03351c                        | +02c
        jsr     0x27db2.l                       | +030
        cmpi.b  #0x40,d7                        | +036
        bne.w   .L03351c                        | +03a
        bset    #0x3,0x13(a6)                   | +03e
        bset    #0x0,0x13(a6)                   | +044
        clr.w   0x66(a6)                        | +04a
        move.b  #0x5,d0                         | +04e
        move.b  d0,0x58(a6)                     | +052
.L03351c:
        jmp     0x28758.l                       | +056

| ----------------------------------------------------------------------------
|  Player_SpawnStart_0336dc  @ $0336DC  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnStart_0336dc, "ax", @progbits
        .global Player_SpawnStart_0336dc
Player_SpawnStart_0336dc:
        move.w  #0x50,0x22(a6)                  | +000
        move.w  #0x1e0,0x24(a6)                 | +006
        jsr     Player_PlaceAtScrollOffset_032ac8(pc) | +00c
        movem.w d2,-(a7)                        | +010
        move.w  #0x177,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  #0x190,d1                       | +01e
        jsr     0x236e.l                        | +022
        move.w  #0x192,d1                       | +028
        jsr     0x236e.l                        | +02c
        move.w  #0x1c,0x1c(a6)                  | +032
        jsr     0x138fe.l                       | +038
        jsr     PlayerEntity_InitAuxState_032A02(pc) | +03e
        bset    #0x4,0x12(a6)                   | +042
        ori.w   #0x0,0x38(a6)                   | +048
        lea     0x394e6.l,a1                    | +04e
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        jsr     0x517fe.l                       | +060

| ----------------------------------------------------------------------------
|  Player_SpawnByMode_033742  @ $033742  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnByMode_033742, "ax", @progbits
        .global Player_SpawnByMode_033742
Player_SpawnByMode_033742:
        clr.w   0x28(a6)                        | +000
        clr.w   0x2a(a6)                        | +004
        clr.w   0x2c(a6)                        | +008
        clr.w   0x2e(a6)                        | +00c
        clr.b   0x26(a6)                        | +010
        clr.b   0x27(a6)                        | +014
        move.w  0x38(a6),d1                     | +018
        move.w  #0x9fe0,d0                      | +01c
        andi.w  #0xffe3,d1                      | +020
        ori.w   #0x10,d0                        | +024
        or.w    d0,d1                           | +028
        move.w  d1,0x38(a6)                     | +02a
        movem.w (a7)+,d2                        | +02e
        cmpi.w  #0x2,d2                         | +032
        bne.w   .L033780                        | +036
        jmp     Player_SpawnFall_0337a8(pc)     | +03a
.L033780:
        cmpi.w  #0x1,d2                         | +03e
        bne.w   .L033798                        | +042
        move.b  #0x7f,0x59(a6)                  | +046
        move.b  #0x7f,0x45(a6)                  | +04c
        jmp     Player_JmpState36C8C_033952(pc) | +052
.L033798:
        move.b  #0x7f,0x59(a6)                  | +056
        move.b  #0x7f,0x45(a6)                  | +05c
        jmp     Player_SpawnParachute_033956(pc) | +062

| ----------------------------------------------------------------------------
|  Player_SpawnFall_0337a8  @ $0337A8  (192 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnFall_0337a8, "ax", @progbits
        .global Player_SpawnFall_0337a8
Player_SpawnFall_0337a8:
        addq.w  #0x3,0x24(a6)                   | +000
        move.b  #0x4,0x70(a6)                   | +004
        lea     0x279f8a.l,a0                   | +00a
        move.l  -0x4(a0),0x74(a6)               | +010
        move.b  #0xff,0x21(a6)                  | +016
        lea     0x279f8a.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        move.w  #0xfc00,0x2a(a6)                | +028
        lea     0xffff.w,a0                     | +02e
        move.l  a0,0x48(a6)                     | +032
        bset    #0x7,0x5b(a6)                   | +036
        lea     .L0337ea(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L0337ea:
        bset    #0x6,0x13(a6)                   | +042
        bset    #0x1,0x8d(a6)                   | +048
        jsr     Player_FrameCommon_032ff2(pc)   | +04e
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +052
        lea     Player_SpawnLand_03386e(pc),a1  | +056
        move.l  a1,(a6)                         | +05a
        bclr    #0x1,0x8d(a6)                   | +05c
        jsr     0x27bc8.l                       | +062
        bcs.w   .L03382a                        | +068
        jsr     0x27bc8.l                       | +06c
        bcs.w   .L03382a                        | +072
        lea     Player_SpawnLandB_0338fa(pc),a1 | +076
        move.l  a1,(a6)                         | +07a
        bset    #0x1,0x8d(a6)                   | +07c
.L03382a:
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +082
        cmpi.b  #0x3,0x106ece.l                 | +086
        beq.w   .L03384e                        | +08e
        movea.l #0xffffffff,a0                  | +092
        lea     Sub_000324C6(pc),a0             | +098
        jsr     0x5dd56.l                       | +09c
        bra.w   .L03385e                        | +0a2
.L03384e:
        movea.l #0xffffffff,a0                  | +0a6
        lea     Sub_000324BC(pc),a0             | +0ac
        jsr     0x5dd56.l                       | +0b0
.L03385e:
        bcc.w   JsrPcThunk_033868               | +0b6
        lea     TaskHandler_037b8e(pc),a1       | +0ba
        move.l  a1,(a6)                         | +0be

| ----------------------------------------------------------------------------
|  Player_SpawnLand_03386e  @ $03386E  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnLand_03386e, "ax", @progbits
        .global Player_SpawnLand_03386e
Player_SpawnLand_03386e:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bset    #0x7,0x5b(a6)                   | +00c
        move.l  #0x32500,0x60(a6)               | +012
        clr.w   0x28(a6)                        | +01a
        clr.w   0x2c(a6)                        | +01e
        lea     .L033896(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L033896:
        jsr     Player_FrameCommon_032ff2(pc)   | +028
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +02c
        jsr     0x27a92.l                       | +030
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +036
        bcc.w   .L0338e4                        | +03a
        lea     Sub_00034704(pc),a1             | +03e
        move.l  a1,(a6)                         | +042
        cmpa.l  #0x100440,a6                    | +044
        bne.w   .L0338cc                        | +04a
        lea     0x32112.l,a1                    | +04e
        jsr     0x4ae.l                         | +054
        bra.w   .L0338d8                        | +05a
.L0338cc:
        lea     0x32142.l,a1                    | +05e
        jsr     0x4ae.l                         | +064
.L0338d8:
        move.b  #0x50,0x45(a6)                  | +06a
        move.b  #0x50,0x59(a6)                  | +070
.L0338e4:
        jsr     0x27eba.l                       | +076
        bcc.w   JsrPcThunk_0338f4               | +07c
        lea     Sub_00036C8C(pc),a1             | +080
        move.l  a1,(a6)                         | +084

| ----------------------------------------------------------------------------
|  Player_SpawnLandB_0338fa  @ $0338FA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnLandB_0338fa, "ax", @progbits
        .global Player_SpawnLandB_0338fa
Player_SpawnLandB_0338fa:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bset    #0x7,0x5b(a6)                   | +00c
        move.l  #0x32500,0x60(a6)               | +012
        clr.w   0x28(a6)                        | +01a
        clr.w   0x2a(a6)                        | +01e
        clr.w   0x2c(a6)                        | +022
        clr.w   0x2e(a6)                        | +026
        lea     .L03392a(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L03392a:
        bset    #0x1,0x8d(a6)                   | +030
        jsr     Player_FrameCommon_032ff2(pc)   | +036
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +03a
        jsr     0x27bc8.l                       | +03e
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +044
        bcc.w   JsrPcThunk_03394c               | +048
        lea     Sub_00036C8C(pc),a1             | +04c
        move.l  a1,(a6)                         | +050

| ----------------------------------------------------------------------------
|  Player_JmpState36C8C_033952  @ $033952  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Player_JmpState36C8C_033952, "ax", @progbits
        .global Player_JmpState36C8C_033952
Player_JmpState36C8C_033952:
        jmp     Sub_00036C8C(pc)                | +000

| ----------------------------------------------------------------------------
|  Player_SpawnParachute_033956  @ $033956  (258 B)
| ----------------------------------------------------------------------------
        .section .text.Player_SpawnParachute_033956, "ax", @progbits
        .global Player_SpawnParachute_033956
Player_SpawnParachute_033956:
        lea     Sub_00038CF6(pc),a1             | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x0,0x7c(a6)                   | +010
        move.w  #0x0,0x7e(a6)                   | +016
        move.b  #0x4,0x70(a6)                   | +01c
        lea     0x279b2c.l,a0                   | +022
        move.l  -0x4(a0),0x74(a6)               | +028
        move.b  #0xff,0x21(a6)                  | +02e
        lea     0x279b2c.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        move.w  #0xff00,0x2a(a6)                | +040
        lea     0xffff.w,a0                     | +046
        move.l  a0,0x48(a6)                     | +04a
        lea     .L0339aa(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L0339aa:
        bset    #0x6,0x13(a6)                   | +054
        bset    #0x1,0x8d(a6)                   | +05a
        jsr     Player_FrameCommon_032ff2(pc)   | +060
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +064
        move.w  0x2a(a6),d0                     | +068
        move.w  #0x400,d1                       | +06c
        jsr     0x267f4.l                       | +070
        move.w  d0,0x2a(a6)                     | +076
        jsr     0x27bc8.l                       | +07a
        bcc.w   .L0339e6                        | +080
        lea     Player_SpawnLand_Done_033e8c(pc),a1 | +084
        move.l  a1,(a6)                         | +088
        bclr    #0x1,0x8d(a6)                   | +08a
.L0339e6:
        move.w  0x38(a6),d1                     | +090
        move.w  #0x9fe0,d0                      | +094
        andi.w  #0xffe3,d1                      | +098
        ori.w   #0x10,d0                        | +09c
        or.w    d0,d1                           | +0a0
        move.w  d1,0x38(a6)                     | +0a2
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0a6
        jsr     Input_FireByMode_033034(pc)     | +0aa
        bcc.w   .L033a0e                        | +0ae
        lea     Sub_00036C8C(pc),a1             | +0b2
        move.l  a1,(a6)                         | +0b6
.L033a0e:
        cmpi.w  #0x170,0x24(a6)                 | +0b8
        bgt.w   .L033a1e                        | +0be
        lea     Sub_00036C8C(pc),a1             | +0c2
        move.l  a1,(a6)                         | +0c6
.L033a1e:
        cmpi.b  #0x3,0x106ece.l                 | +0c8
        beq.w   .L033a3e                        | +0d0
        movea.l #0xffffffff,a0                  | +0d4
        lea     Sub_000324C6(pc),a0             | +0da
        jsr     0x5dd56.l                       | +0de
        bra.w   .L033a4e                        | +0e4
.L033a3e:
        movea.l #0xffffffff,a0                  | +0e8
        lea     Sub_000324BC(pc),a0             | +0ee
        jsr     0x5dd56.l                       | +0f2
.L033a4e:
        bcc.w   JsrPcThunk_033a58               | +0f8
        lea     TaskHandler_037b8e(pc),a1       | +0fc
        move.l  a1,(a6)                         | +100

| ----------------------------------------------------------------------------
|  Player_Crouch_033a5e  @ $033A5E  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Crouch_033a5e, "ax", @progbits
        .global Player_Crouch_033a5e
Player_Crouch_033a5e:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.b  #0x5,0x70(a6)                   | +01e
        lea     0x27973e.l,a0                   | +024
        move.l  -0x4(a0),0x74(a6)               | +02a
        move.b  #0xff,0x21(a6)                  | +030
        lea     0x27973e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     Sub_000326E0(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        move.l  #0x32500,0x60(a6)               | +04a
        jsr     0x267e6.l                       | +052
        clr.w   0x28(a6)                        | +058
        lea     .L033ac0(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L033ac0:
        jsr     Player_FrameCommon_032ff2(pc)   | +062
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +066
        jsr     0x27a92.l                       | +06a
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +070
        bcc.w   .L033af8                        | +074
        lea     Sub_00034704(pc),a1             | +078
        move.l  a1,(a6)                         | +07c
        tst.w   0x28(a6)                        | +07e
        beq.w   .L033af8                        | +082
        ble.w   .L033af2                        | +086
        lea     Sub_00034B38(pc),a1             | +08a
        move.l  a1,(a6)                         | +08e
        bra.w   .L033af8                        | +090
.L033af2:
        lea     Sub_00034D32(pc),a1             | +094
        move.l  a1,(a6)                         | +098
.L033af8:
        jmp     Player_Idle_Tail_0341a4(pc) | +09a

| ----------------------------------------------------------------------------
|  Player_CrouchB_033afc  @ $033AFC  (158 B)
| ----------------------------------------------------------------------------
        .section .text.Player_CrouchB_033afc, "ax", @progbits
        .global Player_CrouchB_033afc
Player_CrouchB_033afc:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bclr    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        move.b  #0x5,0x70(a6)                   | +01e
        lea     0x27981e.l,a0                   | +024
        move.l  -0x4(a0),0x74(a6)               | +02a
        move.b  #0xff,0x21(a6)                  | +030
        lea     0x27981e.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        lea     Sub_000326E0(pc),a0             | +042
        move.l  a0,0x48(a6)                     | +046
        move.l  #0x32500,0x60(a6)               | +04a
        jsr     0x267e6.l                       | +052
        clr.w   0x28(a6)                        | +058
        lea     .L033b5e(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L033b5e:
        jsr     Player_FrameCommon_032ff2(pc)   | +062
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +066
        jsr     0x27a92.l                       | +06a
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +070
        bcc.w   .L033b96                        | +074
        lea     Sub_00034704(pc),a1             | +078
        move.l  a1,(a6)                         | +07c
        tst.w   0x28(a6)                        | +07e
        beq.w   .L033b96                        | +082
        ble.w   .L033b90                        | +086
        lea     Sub_00034B38(pc),a1             | +08a
        move.l  a1,(a6)                         | +08e
        bra.w   .L033b96                        | +090
.L033b90:
        lea     Sub_00034D32(pc),a1             | +094
        move.l  a1,(a6)                         | +098
.L033b96:
        bra.w   Player_Idle_Tail_0341a4     | +09a

| ----------------------------------------------------------------------------
|  Player_Reload_033b9a  @ $033B9A  (458 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Reload_033b9a, "ax", @progbits
        .global Player_Reload_033b9a
Player_Reload_033b9a:
        cmpi.w  #0x1,0x72(a6)                   | +000
        bne.w   .L033ba8                        | +006
        jmp     .L033c8e(pc)                    | +00a
.L033ba8:
        move.w  #0x0,0x7c(a6)                   | +00e
        move.w  #0x0,0x7e(a6)                   | +014
        bset    #0x2,0x8c(a6)                   | +01a
        bclr    #0x1,0x8c(a6)                   | +020
        bclr    #0x3,0x8c(a6)                   | +026
        move.b  #0x33,0x70(a6)                  | +02c
        lea     0x2796f8.l,a0                   | +032
        move.l  -0x4(a0),0x74(a6)               | +038
        move.b  #0xff,0x21(a6)                  | +03e
        lea     0x2796f8.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     Sub_000326E0(pc),a0             | +050
        move.l  a0,0x48(a6)                     | +054
        move.l  #0x32500,0x60(a6)               | +058
        jsr     0x267e6.l                       | +060
        clr.w   0x28(a6)                        | +066
        lea     .L033c0a(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L033c0a:
        jsr     Player_FrameCommon_032ff2(pc)   | +070
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +074
        jsr     0x27a92.l                       | +078
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +07e
        bcc.w   .L033c6c                        | +082
        lea     Sub_00034704(pc),a1             | +086
        move.l  a1,(a6)                         | +08a
        tst.w   0x28(a6)                        | +08c
        beq.w   .L033c42                        | +090
        ble.w   .L033c3c                        | +094
        lea     Sub_00034B38(pc),a1             | +098
        move.l  a1,(a6)                         | +09c
        bra.w   .L033c42                        | +09e
.L033c3c:
        lea     Sub_00034D32(pc),a1             | +0a2
        move.l  a1,(a6)                         | +0a6
.L033c42:
        jsr     0x5e9b6.l                       | +0a8
        andi.w  #0x7,d0                         | +0ae
        moveq   #4,d0                           | +0b2
        cmp.w   0x82(a6),d0                     | +0b4
        bne.w   .L033c5c                        | +0b8
        lea     Player_CrouchB_033afc(pc),a1    | +0bc
        move.l  a1,(a6)                         | +0c0
.L033c5c:
        cmpi.w  #0x0,0x82(a6)                   | +0c2
        bne.w   .L033c6c                        | +0c8
        lea     Player_Idle_033d64(pc),a1       | +0cc
        move.l  a1,(a6)                         | +0d0
.L033c6c:
        btst    #0x2,0x8c(a6)                   | +0d2
        bne.w   .L033c7c                        | +0d8
        move.b  #0x1,0x85(a6)                   | +0dc
.L033c7c:
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +0e2
        bcc.w   .L033c8a                        | +0e6
        lea     Sub_000345B8(pc),a1             | +0ea
        move.l  a1,(a6)                         | +0ee
.L033c8a:
        bra.w   Player_Idle_Tail_0341a4     | +0f0
.L033c8e:
        move.w  #0x0,0x7c(a6)                   | +0f4
        move.w  #0x0,0x7e(a6)                   | +0fa
        bset    #0x2,0x8c(a6)                   | +100
        bclr    #0x1,0x8c(a6)                   | +106
        bclr    #0x3,0x8c(a6)                   | +10c
        move.b  #0x33,0x70(a6)                  | +112
        lea     0x279702.l,a0                   | +118
        move.l  -0x4(a0),0x74(a6)               | +11e
        move.b  #0xff,0x21(a6)                  | +124
        lea     0x279702.l,a0                   | +12a
        jsr     0x28cd4.l                       | +130
        lea     Sub_000326E0(pc),a0             | +136
        move.l  a0,0x48(a6)                     | +13a
        move.l  #0x32500,0x60(a6)               | +13e
        jsr     0x267e6.l                       | +146
        clr.w   0x28(a6)                        | +14c
        lea     .L033cf0(pc),a1                 | +150
        move.l  a1,(a6)                         | +154
.L033cf0:
        jsr     Player_FrameCommon_032ff2(pc)   | +156
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +15a
        jsr     0x27a92.l                       | +15e
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +164
        bcc.w   .L033d42                        | +168
        lea     Sub_00034704(pc),a1             | +16c
        move.l  a1,(a6)                         | +170
        tst.w   0x28(a6)                        | +172
        beq.w   .L033d28                        | +176
        ble.w   .L033d22                        | +17a
        lea     Sub_00034B38(pc),a1             | +17e
        move.l  a1,(a6)                         | +182
        bra.w   .L033d28                        | +184
.L033d22:
        lea     Sub_00034D32(pc),a1             | +188
        move.l  a1,(a6)                         | +18c
.L033d28:
        jsr     0x5e9b6.l                       | +18e
        andi.w  #0x7,d0                         | +194
        moveq   #4,d0                           | +198
        cmp.w   0x82(a6),d0                     | +19a
        bne.w   .L033d42                        | +19e
        lea     Player_CrouchB_033afc(pc),a1    | +1a2
        move.l  a1,(a6)                         | +1a6
.L033d42:
        btst    #0x2,0x8c(a6)                   | +1a8
        bne.w   .L033d52                        | +1ae
        move.b  #0x1,0x85(a6)                   | +1b2
.L033d52:
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +1b8
        bcc.w   .L033d60                        | +1bc
        lea     Sub_000345B8(pc),a1             | +1c0
        move.l  a1,(a6)                         | +1c4
.L033d60:
        bra.w   Player_Idle_Tail_0341a4     | +1c6

| ----------------------------------------------------------------------------
|  Player_Idle_033d64  @ $033D64  (1368 B)
| ----------------------------------------------------------------------------
        .section .text.Player_Idle_033d64, "ax", @progbits
        .global Player_Idle_033d64
Player_Idle_033d64:
        move.w  #0x0,0x7c(a6)                   | +000
        move.w  #0x0,0x7e(a6)                   | +006
        bset    #0x2,0x8c(a6)                   | +00c
        bclr    #0x1,0x8c(a6)                   | +012
        bclr    #0x3,0x8c(a6)                   | +018
        cmpi.b  #0x13,0x70(a6)                  | +01e
        bcs.w   .L033db6                        | +024
        cmpi.b  #0x24,0x70(a6)                  | +028
        bcc.w   .L033db6                        | +02e
        cmpi.b  #0x0,0x71(a6)                   | +032
        beq.w   .L033db6                        | +038
        move.b  #0x21,0x70(a6)                  | +03c
        lea     0x279828.l,a0                   | +042
        move.l  -0x4(a0),0x74(a6)               | +048
        bra.w   .L033dda                        | +04e
.L033db6:
        move.b  #0x21,0x70(a6)                  | +052
        lea     0x279828.l,a0                   | +058
        move.l  -0x4(a0),0x74(a6)               | +05e
        move.b  #0xff,0x21(a6)                  | +064
        lea     0x279828.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
.L033dda:
        lea     Sub_000326E0(pc),a0             | +076
        move.l  a0,0x48(a6)                     | +07a
        cmpi.b  #0x0,0x71(a6)                   | +07e
        beq.w   .L033e04                        | +084
        lea     0xffff.w,a0                     | +088
        move.l  a0,0x48(a6)                     | +08c
        lea     Sub_00038BE4(pc),a1             | +090
        jsr     0x4ae.l                         | +094
        jsr     0x5dd02.l                       | +09a
.L033e04:
        move.b  #0x0,0x71(a6)                   | +0a0
        move.w  #0xa,0x82(a6)                   | +0a6
        bset    #0x7,0x5b(a6)                   | +0ac
        move.l  #0x32500,0x60(a6)               | +0b2
        jsr     0x267e6.l                       | +0ba
        clr.w   0x28(a6)                        | +0c0
        lea     .L033e2e(pc),a1                 | +0c4
        move.l  a1,(a6)                         | +0c8
.L033e2e:
        btst    #0x4,0x8c(a6)                   | +0ca
        beq.w   .L033e46                        | +0d0
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +0d4
        lea     Sub_000345B8(pc),a1             | +0d8
        move.l  a1,(a6)                         | +0dc
        bra.w   Sub_000345B8                    | +0de
.L033e46:
        jsr     Player_FrameCommon_032ff2(pc)   | +0e2
        jsr     0x27a92.l                       | +0e6
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +0ec
        bcc.w   .L033e7a                        | +0f0
        lea     Sub_00034704(pc),a1             | +0f4
        move.l  a1,(a6)                         | +0f8
        tst.w   0x28(a6)                        | +0fa
        beq.w   .L033e7a                        | +0fe
        ble.w   .L033e74                        | +102
        lea     Sub_00034B38(pc),a1             | +106
        move.l  a1,(a6)                         | +10a
        bra.w   .L033e7a                        | +10c
.L033e74:
        lea     Sub_00034D32(pc),a1             | +110
        move.l  a1,(a6)                         | +114
.L033e7a:
        btst    #0x2,0x8c(a6)                   | +116
        bne.w   .L033e88                        | +11c
        bra.w   Player_Idle_Tail_0341a4     | +120
.L033e88:
        bra.w   Player_Idle_Tail_0341a4     | +124
        .global Player_SpawnLand_Done_033e8c
Player_SpawnLand_Done_033e8c:
        cmpa.l  #0x100440,a6                    | +128
        bne.w   .L033ea6                        | +12e
        lea     0x32112.l,a1                    | +132
        jsr     0x4ae.l                         | +138
        bra.w   .L033eb2                        | +13e
.L033ea6:
        lea     0x32142.l,a1                    | +142
        jsr     0x4ae.l                         | +148
.L033eb2:
        move.b  #0x50,0x45(a6)                  | +14e
        move.b  #0x50,0x59(a6)                  | +154
        jmp     .L033ede(pc)                    | +15a
        cmpi.b  #0x0,0x106ed3.l                 | +15e
        beq.w   .L033ed4                        | +166
        move.b  #0x1e,0x45(a6)                  | +16a
.L033ed4:
        move.b  #0x1e,0x59(a6)                  | +170
        jmp     .L033ede(pc)                    | +176
.L033ede:
        bclr    #0x0,0x8c(a6)                   | +17a
        cmpi.w  #0x2,0x72(a6)                   | +180
        bne.w   .L033ef4                        | +186
        move.w  #0x0,0x72(a6)                   | +18a
.L033ef4:
        move.w  #0x0,0x7c(a6)                   | +190
        move.w  #0x0,0x7e(a6)                   | +196
        bclr    #0x2,0x8c(a6)                   | +19c
        bclr    #0x1,0x8c(a6)                   | +1a2
        bclr    #0x3,0x8c(a6)                   | +1a8
        move.b  #0x1,0x70(a6)                   | +1ae
        lea     0x279a96.l,a0                   | +1b4
        move.l  -0x4(a0),0x74(a6)               | +1ba
        move.b  #0xff,0x21(a6)                  | +1c0
        lea     0x279a96.l,a0                   | +1c6
        jsr     0x28cd4.l                       | +1cc
        bset    #0x7,0x5b(a6)                   | +1d2
        jsr     Input_DownPressed_032e90(pc)    | +1d8
        bcc.w   .L033f48                        | +1dc
        jmp     Sub_0003827A(pc)                | +1e0
.L033f48:
        cmpi.w  #0x0,0x82(a6)                   | +1e4
        bne.w   .L033f5c                        | +1ea
        lea     Player_Idle_033d64(pc),a1       | +1ee
        move.l  a1,(a6)                         | +1f2
        bra.w   Player_Idle_033d64              | +1f4
.L033f5c:
        cmpi.b  #0x0,0x85(a6)                   | +1f8
        bne.w   .L033f7a                        | +1fe
        cmpi.b  #0x1,0x71(a6)                   | +202
        bne.w   .L033f7a                        | +208
        lea     Player_Reload_033b9a(pc),a1     | +20c
        move.l  a1,(a6)                         | +210
        bra.w   Player_Reload_033b9a            | +212
.L033f7a:
        btst    #0x0,0x88(a6)                   | +216
        beq.w   .L033f8e                        | +21c
        lea     Player_Crouch_033a5e(pc),a1     | +220
        move.l  a1,(a6)                         | +224
        bra.w   Player_Crouch_033a5e            | +226
.L033f8e:
        move.l  #0x32500,0x60(a6)               | +22a
        jsr     0x267e6.l                       | +232
        clr.w   0x28(a6)                        | +238
        lea     Sub_000326E0(pc),a0             | +23c
        move.l  a0,0x48(a6)                     | +240
        lea     .L033fae(pc),a1                 | +244
        move.l  a1,(a6)                         | +248
.L033fae:
        jsr     Player_FrameCommon_032ff2(pc)   | +24a
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +24e
        bcc.w   .L033fc0                        | +252
        lea     Sub_000345B8(pc),a1             | +256
        move.l  a1,(a6)                         | +25a
.L033fc0:
        jsr     0x27a92.l                       | +25c
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +262
        bcc.w   .L034042                        | +266
        lea     Sub_00034704(pc),a1             | +26a
        move.l  a1,(a6)                         | +26e
        move.b  #0x1,0x78(a6)                   | +270
        lsl.b   #0x4,d0                         | +276
        or.b    0x78(a6),d0                     | +278
        move.b  d0,0x79(a6)                     | +27c
        btst    #0x0,0x88(a6)                   | +280
        beq.w   .L033ff4                        | +286
        lea     Player_Crouch_033a5e(pc),a1     | +28a
        move.l  a1,(a6)                         | +28e
.L033ff4:
        cmpi.b  #0x0,0x85(a6)                   | +290
        bne.w   .L03400e                        | +296
        cmpi.b  #0x1,0x71(a6)                   | +29a
        bne.w   .L03400e                        | +2a0
        lea     Player_Reload_033b9a(pc),a1     | +2a4
        move.l  a1,(a6)                         | +2a8
.L03400e:
        jsr     0x5e9b6.l                       | +2aa
        andi.w  #0x7,d0                         | +2b0
        moveq   #4,d0                           | +2b4
        cmpi.b  #0x1,0x70(a6)                   | +2b6
        bne.w   .L034032                        | +2bc
        cmp.w   0x82(a6),d0                     | +2c0
        bne.w   .L034032                        | +2c4
        lea     Player_CrouchB_033afc(pc),a1    | +2c8
        move.l  a1,(a6)                         | +2cc
.L034032:
        cmpi.w  #0x0,0x82(a6)                   | +2ce
        bne.w   .L034042                        | +2d4
        lea     Player_Idle_033d64(pc),a1       | +2d8
        move.l  a1,(a6)                         | +2dc
.L034042:
        jmp     Player_Idle_Tail_0341a4(pc) | +2de
        cmpi.b  #0x13,0x70(a6)                  | +2e2
        bcs.w   .L03405e                        | +2e8
        cmpi.b  #0x24,0x70(a6)                  | +2ec
        bcc.w   .L03405e                        | +2f2
        jmp     Sub_00034704(pc)                | +2f6
.L03405e:
        cmpi.w  #0x1,0x72(a6)                   | +2fa
        bne.w   .L03406c                        | +300
        jmp     Sub_00034704(pc)                | +304
.L03406c:
        move.w  #0x0,0x7c(a6)                   | +308
        move.w  #0x0,0x7e(a6)                   | +30e
        move.b  #0x5,0x70(a6)                   | +314
        lea     0x279a96.l,a0                   | +31a
        move.l  -0x4(a0),0x74(a6)               | +320
        move.b  #0xff,0x21(a6)                  | +326
        lea     0x279a96.l,a0                   | +32c
        jsr     0x28cd4.l                       | +332
        bclr    #0x2,0x8c(a6)                   | +338
        bclr    #0x1,0x8c(a6)                   | +33e
        bclr    #0x3,0x8c(a6)                   | +344
        bset    #0x7,0x5b(a6)                   | +34a
        move.l  #0x32500,0x60(a6)               | +350
        jsr     0x267e6.l                       | +358
        clr.w   0x28(a6)                        | +35e
        lea     Sub_000326E0(pc),a0             | +362
        move.l  a0,0x48(a6)                     | +366
        cmpi.w  #0x0,0x82(a6)                   | +36a
        bne.w   .L0340e2                        | +370
        lea     Player_Idle_033d64(pc),a1       | +374
        move.l  a1,(a6)                         | +378
        bra.w   Player_Idle_033d64              | +37a
.L0340e2:
        cmpi.b  #0x0,0x85(a6)                   | +37e
        bne.w   .L034100                        | +384
        cmpi.b  #0x1,0x71(a6)                   | +388
        bne.w   .L034100                        | +38e
        lea     Player_Reload_033b9a(pc),a1     | +392
        move.l  a1,(a6)                         | +396
        bra.w   Player_Reload_033b9a            | +398
.L034100:
        btst    #0x0,0x88(a6)                   | +39c
        beq.w   .L034114                        | +3a2
        lea     Player_Crouch_033a5e(pc),a1     | +3a6
        move.l  a1,(a6)                         | +3aa
        bra.w   Player_Crouch_033a5e            | +3ac
.L034114:
        lea     .L03411a(pc),a1                 | +3b0
        move.l  a1,(a6)                         | +3b4
.L03411a:
        jsr     Player_FrameCommon_032ff2(pc)   | +3b6
        jsr     Player_PlayWeaponMusicIfFlag_032d32(pc) | +3ba
        jsr     0x27a92.l                       | +3be
        jsr     Player_CheckDeathOrState21_032aa8(pc) | +3c4
        bcc.w   Player_Idle_Tail_0341a4     | +3c8
        lea     Sub_00034704(pc),a1             | +3cc
        move.l  a1,(a6)                         | +3d0
        move.b  #0x1,0x78(a6)                   | +3d2
        lsl.b   #0x4,d0                         | +3d8
        or.b    0x78(a6),d0                     | +3da
        move.b  d0,0x79(a6)                     | +3de
        btst    #0x0,0x88(a6)                   | +3e2
        beq.w   .L034156                        | +3e8
        lea     Player_Crouch_033a5e(pc),a1     | +3ec
        move.l  a1,(a6)                         | +3f0
.L034156:
        cmpi.b  #0x0,0x85(a6)                   | +3f2
        bne.w   .L034170                        | +3f8
        cmpi.b  #0x1,0x71(a6)                   | +3fc
        bne.w   .L034170                        | +402
        lea     Player_Reload_033b9a(pc),a1     | +406
        move.l  a1,(a6)                         | +40a
.L034170:
        jsr     0x5e9b6.l                       | +40c
        andi.w  #0x7,d0                         | +412
        moveq   #4,d0                           | +416
        cmpi.b  #0x1,0x70(a6)                   | +418
        bne.w   .L034194                        | +41e
        cmp.w   0x82(a6),d0                     | +422
        bne.w   .L034194                        | +426
        lea     Player_CrouchB_033afc(pc),a1    | +42a
        move.l  a1,(a6)                         | +42e
.L034194:
        cmpi.w  #0x0,0x82(a6)                   | +430
        bne.w   Player_Idle_Tail_0341a4     | +436
        lea     Player_Idle_033d64(pc),a1       | +43a
        move.l  a1,(a6)                         | +43e
        .global Player_Idle_Tail_0341a4
Player_Idle_Tail_0341a4:
        btst    #0x2,0x8c(a6)                   | +440
        bne.w   .L03425c                        | +446
        btst    #0x0,0x3a(a6)                   | +44a
        bne.w   .L0341d8                        | +450
        jsr     JmpAbsThunk_032e3c(pc)          | +454
        bcc.w   .L0341c6                        | +458
        lea     Sub_00034B38(pc),a1             | +45c
        move.l  a1,(a6)                         | +460
.L0341c6:
        jsr     Input_RightThunk_032e42(pc)     | +462
        bcc.w   .L0341d4                        | +466
        lea     Sub_00035BF8(pc),a1             | +46a
        move.l  a1,(a6)                         | +46e
.L0341d4:
        bra.w   .L0341f4                        | +470
.L0341d8:
        jsr     JmpAbsThunk_032e3c(pc)          | +474
        bcc.w   .L0341e6                        | +478
        lea     Sub_00035ABA(pc),a1             | +47c
        move.l  a1,(a6)                         | +480
.L0341e6:
        jsr     Input_RightThunk_032e42(pc)     | +482
        bcc.w   .L0341f4                        | +486
        lea     Sub_00034D32(pc),a1             | +48a
        move.l  a1,(a6)                         | +48e
.L0341f4:
        jsr     Input_DownPressed_032e90(pc)    | +490
        bcc.w   .L034202                        | +494
        lea     Sub_00037C74(pc),a1             | +498
        move.l  a1,(a6)                         | +49c
.L034202:
        jsr     Player_ActionSelect_0330d0(pc)  | +49e
        bcc.w   .L03425c                        | +4a2
        cmpi.b  #0xff,d1                        | +4a6
        bne.w   .L03421c                        | +4aa
        lea     Sub_00035D34(pc),a1             | +4ae
        move.l  a1,(a6)                         | +4b2
        bra.w   .L03425c                        | +4b4
.L03421c:
        cmpi.b  #0x3,d1                         | +4b8
        bne.w   .L03422e                        | +4bc
        lea     Sub_000360BC(pc),a1             | +4c0
        move.l  a1,(a6)                         | +4c4
        bra.w   .L03425c                        | +4c6
.L03422e:
        cmpi.b  #0x4,d1                         | +4ca
        bne.w   .L034240                        | +4ce
        lea     Sub_0003873C(pc),a1             | +4d2
        move.l  a1,(a6)                         | +4d6
        bra.w   .L03425c                        | +4d8
.L034240:
        cmpi.b  #0x1,d1                         | +4dc
        bne.w   .L034252                        | +4e0
        lea     Sub_0003437E(pc),a1             | +4e4
        move.l  a1,(a6)                         | +4e8
        bra.w   .L03425c                        | +4ea
.L034252:
        lea     Sub_000342C4(pc),a1             | +4ee
        move.l  a1,(a6)                         | +4f2
        bra.w   .L03425c                        | +4f4
.L03425c:
        jsr     0x27eba.l                       | +4f8
        bcc.w   .L034276                        | +4fe
        lea     TaskHandler_036d64(pc),a1       | +502
        move.l  a1,(a6)                         | +506
        beq.w   .L034276                        | +508
        lea     Sub_00037018(pc),a1             | +50c
        move.l  a1,(a6)                         | +510
.L034276:
        jsr     Input_FireByMode_033034(pc)     | +512
        bcc.w   .L034284                        | +516
        lea     Sub_00036914(pc),a1             | +51a
        move.l  a1,(a6)                         | +51e
.L034284:
        jsr     PlayerRoute_PublishState_033522(pc) | +520
        cmpi.b  #0x3,0x106ece.l                 | +524
        beq.w   .L0342a8                        | +52c
        movea.l #0xffffffff,a0                  | +530
        lea     Sub_000324C6(pc),a0             | +536
        jsr     0x5dd56.l                       | +53a
        bra.w   .L0342b8                        | +540
.L0342a8:
        movea.l #0xffffffff,a0                  | +544
        lea     Sub_000324BC(pc),a0             | +54a
        jsr     0x5dd56.l                       | +54e
.L0342b8:
        bcc.w   SetHandlerRts_0342c2            | +554
