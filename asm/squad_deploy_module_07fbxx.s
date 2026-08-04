| ============================================================================
|  Metal Slug 1 - asm/squad_deploy_module_07fbxx.s
|  ----------------------------------------------------------------------------
|  Wave CCC - modulo "Squad Deploy": comandante que despliega un escuadron
|  de hasta 8 soldados con gestion de slots por bitmask.
|  Region $07FBD2..$08072E, 23 entradas, 2 714 B (cierra los 23 huecos entre
|  las 26 islas SetTaskHandler_*/SetC_*/ClearC_*/SetTaskW_* ya matcheadas
|  en C - la cadena de handlers del modulo queda completa).
|
|  ARQUITECTURA DEL MODULO
|  -----------------------
|  1. Comandante (Squad_CmdrInit_08067e): suena $49/$170/$171, HP en +0x36
|     desde tabla 2D $2BEF82 ($799DE), tipo 6 + timer ($138FE), sprite
|     $2E3916. Bucle activo: probes $5E506/$808E8/$80932/$8094A + $28D70.
|  2. Manager de 8 slots (Squad_Mgr8Slots_0803e8): inicializa el array de
|     words +0x80..+0x8E a $FFFF (recompensas por slot) y +0x77/+0x78
|     como BITMASKS de ocupacion (alguna vez vivo / vivo ahora). Cadencias
|     +0x72/+0x74 y cupo +0x66 desde tablas 2D $2BEA3E/$2BEAC0/$2BEB42.
|     En cada tick busca el primer slot libre con btst y hace spawn del
|     hijo (Squad_ChildSpawnInit_080540) marcando bset en ambos masks;
|     el hijo hereda +0x84 (indice de slot), +0x78 (fila) y +0x66.
|  3. Hijos: dos plantillas de init - por tabla de offsets $2E3EBC
|     (pares ptr,ptr de 8 B: coords + timings por fila, indexadas
|     por +0x78*8, byte de timing en +0x32/+0x33) o posicion fija
|     desde $2BEED0 (x,y,flag por fila). Sprites $2E2FCA/$2E2FD6 (anim
|     alterna), salto con fisica $13C0E (vel $400 ang $88) y sombra
|     $723DA. Al morir notifican al padre (Squad_NotifyParent_0803d2
|     incrementa +0x20 del padre).
|  4. Sub-modulo "hatch" ($7FC1A): 3 variantes de fila que caen en
|     comun - spawn de escolta $77FD6 via $4AE, sprite de fila
|     ($2E1DCA/-$30 | $2E1E5C/+0 | $2E1EEE/+$40 en x), +0x70 guarda el
|     desplazamiento lateral (-$48/0/+$40). Cuando el padre llega a
|     estado 3 monta las piezas $2E2AE8/$2E2AFA ($77C7E) + escolta
|     $7F22A con flags $C000.
|  5. Seguimiento (Squad_TrackArc/FollowLeader/BobOscillate): oscilacion
|     vertical por tablas de curva $2E22FA/$2E2320 (indice = vel/4 con
|     clamps +-$90 -> rebote $B800/$4800) y onda seno $2C072C
|     (fase +0x74 mod $80, amplitud *4/256).
|
|  Hallazgos forenses:
|    - Squad_AIDecide ($7FEE8): unico uso del idiom `bpl.b` corto hacia
|      atras ($6AEC) en el modulo - GAS lo reproduce con bpl.b explicito.
|    - Squad_Mgr8Slots: doble pasada de btst sobre +0x77/+0x78 (primero
|      busca slot NUNCA usado, luego cualquier slot muerto) - respawn
|      con memoria de bajas.
|    - `lea $ffff.w, a0` + `move.l a0,$48(a6)` (=ENTITY_NIL corto, 4 B)
|      en $7FD2A/$7FD7A donde otros modulos usan `movea.l #-1` (6 B).
|    - Squad_RankFromDist ($7FFFC): divu encadenado (dist/(radio/4)) con
|      clamp 3..0 - rango de amenaza por cercania.
|    - Los `bcc.w` colgantes de cada estado saltan al RTS INTERNO (+6)
|      de la isla SetTaskHandler siguiente (defsyms SetHandlerRts_*),
|      mismo idiom que squad_children_handlers_0414xx.s.
|
|  Campos (a6): +0x0c padre  +0x20/+0x21 contadores de fase/muerte
|    +0x22/+0x24 x/y  +0x28/+0x2a/+0x2c velocidades  +0x32/+0x33 timing anim
|    +0x36 HP  +0x48 puntero recompensa  +0x5c variante aleatoria
|    +0x66 cupo/recompensa  +0x70 offset lateral / contador  +0x72 cadencia A
|    +0x74 cadencia B / fase seno  +0x76 num spawns hechos  +0x77/+0x78
|    bitmask slots (mgr) / fila (hijo)  +0x7c rango  +0x80..+0x8e
|    recompensa por slot  +0x84 indice slot  +0x90 ptr datos +0x94/+0x96
|    estado anim  +0x98/+0x99/+0x9a stats  +0x9d lado
| ============================================================================

| ----------------------------------------------------------------------------
|  Squad_EscortInit_07fbd2  @ $07FBD2  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_EscortInit_07fbd2, "ax", @progbits
        .global Squad_EscortInit_07fbd2
Squad_EscortInit_07fbd2:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        move.b  d0,0x5c(a6)                     | +00a
        lea     0x29ba70.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        lea     .L7fbf2(pc),a1                  | +01a
        move.l  a1,(a6)                         | +01e
.L7fbf2:
        jsr     Squad_FollowLeader_07fe66(pc)   | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L7fc06                         | +02a
        lea     TaskHandler_07fb28(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L7fc06:
        jsr     Squad_DeferredRelease_080054(pc) | +034
        jsr     Squad_LeaderDeadGate_07fde0(pc) | +038
        bcc.w   SetHandlerRts_07fc18            | +03c

| ----------------------------------------------------------------------------
|  Squad_HatchRow3Spawn_07fc1a  @ $07FC1A  (294 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_HatchRow3Spawn_07fc1a, "ax", @progbits
        .global Squad_HatchRow3Spawn_07fc1a
Squad_HatchRow3Spawn_07fc1a:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        addi.w  #0x28,0x38(a0)                  | +012
        subi.w  #0x30,0x22(a0)                  | +018
        move.w  #0xffb8,0x70(a6)                | +01e
        lea     0x2e1dca.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     .L7fcbe(pc),a1                  | +030
        move.l  a1,(a6)                         | +034
        bra.w   .L7fcbe                         | +036
        .global Squad_HatchRowB_07fc54
Squad_HatchRowB_07fc54:
        lea     0x77fd6.l,a1                    | +03a
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046
        addi.w  #0x28,0x38(a0)                  | +04c
        move.w  #0x0,0x70(a6)                   | +052
        lea     0x2e1e5c.l,a0                   | +058
        jsr     0x28cd4.l                       | +05e
        lea     .L7fcbe(pc),a1                  | +064
        move.l  a1,(a6)                         | +068
        bra.w   .L7fcbe                         | +06a
        .global Squad_HatchRowC_07fc88
Squad_HatchRowC_07fc88:
        lea     0x77fd6.l,a1                    | +06e
        jsr     0x4ae.l                         | +074
        jsr     0x5dd02.l                       | +07a
        addi.w  #0x28,0x38(a0)                  | +080
        addi.w  #0x40,0x22(a0)                  | +086
        move.w  #0x40,0x70(a6)                  | +08c
        lea     0x2e1eee.l,a0                   | +092
        jsr     0x28cd4.l                       | +098
        lea     .L7fcbe(pc),a1                  | +09e
        move.l  a1,(a6)                         | +0a2
.L7fcbe:
        movea.l 0xc(a6),a0                      | +0a4
        move.w  0x22(a0),0x22(a6)               | +0a8
        move.w  0x24(a0),0x24(a6)               | +0ae
        jsr     0x28d70.l                       | +0b4
        bclr    #0x3,0x13(a6)                   | +0ba
        movea.l 0xc(a6),a0                      | +0c0
        cmpi.b  #0x3,0x20(a0)                   | +0c4
        bne.w   .L7fd38                         | +0ca
        tst.w   0x70(a6)                        | +0ce
        bne.w   .L7fd2a                         | +0d2
        lea     0x2e2ae8.l,a1                   | +0d6
        jsr     0x77c7e.l                       | +0dc
        addi.w  #0x28,0x38(a0)                  | +0e2
        lea     0x2e2afa.l,a1                   | +0e8
        jsr     0x77c7e.l                       | +0ee
        addi.w  #0x28,0x38(a0)                  | +0f4
        lea     TaskHandler_07f22a(pc),a1       | +0fa
        jsr     0x4ae.l                         | +0fe
        jsr     0x5dd02.l                       | +104
        addi.w  #0xc000,0x38(a0)                | +10a
.L7fd2a:
        lea     0xffff.w,a0                     | +110
        move.l  a0,0x48(a6)                     | +114
        lea     Squad_HatchDeployed_07fd48(pc),a1 | +118
        move.l  a1,(a6)                         | +11c
.L7fd38:
        jsr     Squad_ProbeLeader_07fdc4(pc)    | +11e
        bcc.w   SetHandlerRts_07fd46            | +122

| ----------------------------------------------------------------------------
|  Squad_HatchDeployed_07fd48  @ $07FD48  (88 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_HatchDeployed_07fd48, "ax", @progbits
        .global Squad_HatchDeployed_07fd48
Squad_HatchDeployed_07fd48:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        jsr     0x28d70.l                       | +010
        movea.l 0xc(a6),a0                      | +016
        cmpi.b  #0x1,0x21(a0)                   | +01a
        bne.w   .L7fd98                         | +020
        move.w  0x70(a6),d0                     | +024
        add.w   d0,0x22(a6)                     | +028
        addi.w  #0x28,0x38(a6)                  | +02c
        lea     0xffff.w,a0                     | +032
        move.l  a0,0x48(a6)                     | +036
        lea     0x77fd6.l,a1                    | +03a
        move.l  a1,(a6)                         | +040
        tst.w   0x70(a6)                        | +042
        bne.w   .L7fd98                         | +046
        jsr     0x434dc.l                       | +04a
.L7fd98:
        jsr     Squad_ProbeLeader_07fdc4(pc)    | +050
        bcc.w   SetHandlerRts_07fda6            | +054

| ----------------------------------------------------------------------------
|  Squad_LeaderNotify_07fda8  @ $07FDA8  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaderNotify_07fda8, "ax", @progbits
        .global Squad_LeaderNotify_07fda8
Squad_LeaderNotify_07fda8:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x5,0x20(a0)                   | +004
        bra.w   Squad_KillSelf_07fdbc           | +00a
        .global Squad_MarkLeaderDead_07fdb6
Squad_MarkLeaderDead_07fdb6:
        move.b  #0xff,0x21(a6)                  | +00e
        .global Squad_KillSelf_07fdbc
Squad_KillSelf_07fdbc:
        jmp     0x518.l                         | +014
        rts                                     | +01a
        .global Squad_ProbeLeader_07fdc4
Squad_ProbeLeader_07fdc4:
        lea     0x2e2ad8.l,a0                   | +01c
        jsr     0x5dd5c.l                       | +022
        bcc.w   ClearC_07fdda                   | +028

| ----------------------------------------------------------------------------
|  Squad_LeaderDeadGate_07fde0  @ $07FDE0  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaderDeadGate_07fde0, "ax", @progbits
        .global Squad_LeaderDeadGate_07fde0
Squad_LeaderDeadGate_07fde0:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0xff,0x21(a0)                  | +004
        bne.w   ClearC_07fdf4                   | +00a

| ----------------------------------------------------------------------------
|  Squad_TrackArc_07fdfa  @ $07FDFA  (232 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_TrackArc_07fdfa, "ax", @progbits
        .global Squad_TrackArc_07fdfa
Squad_TrackArc_07fdfa:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.w  0x74(a6),d0                     | +010
        asr.w   #0x8,d0                         | +014
        add.w   0x22(a6),d0                     | +016
        move.w  d0,0x22(a6)                     | +01a
        addi.w  #0x8,0x24(a6)                   | +01e
        lea     0x2e22fa.l,a1                   | +024
        move.w  0x74(a6),d1                     | +02a
        asr.w   #0x8,d1                         | +02e
        addi.w  #0x50,d1                        | +030
        tst.w   d1                              | +034
        bpl.w   .L7fe3c                         | +036
        move.w  #0xb800,0x74(a6)                | +03a
        clr.w   d1                              | +040
.L7fe3c:
        cmpi.w  #0x90,d1                        | +042
        ble.w   .L7fe4e                         | +046
        move.w  #0x4800,0x74(a6)                | +04a
        move.w  #0x90,d1                        | +050
.L7fe4e:
        clr.w   d0                              | +054
        lsr.w   #0x2,d1                         | +056
        move.b  (a1,d1.w),d0                    | +058
        cmpi.w  #0x1,0x72(a6)                   | +05c
        beq.w   .L7fe64                         | +062
        add.w   d0,0x24(a6)                     | +066
.L7fe64:
        rts                                     | +06a
        .global Squad_FollowLeader_07fe66
Squad_FollowLeader_07fe66:
        movea.l 0xc(a6),a0                      | +06c
        movea.l 0xc(a0),a0                      | +070
        move.w  0x78(a0),0x22(a6)               | +074
        move.w  0x7a(a0),0x24(a6)               | +07a
        subq.w  #0x8,0x22(a6)                   | +080
        move.w  0x74(a6),d0                     | +084
        add.w   0x28(a6),d0                     | +088
        move.w  d0,0x74(a6)                     | +08c
        asr.w   #0x8,d0                         | +090
        add.w   d0,0x22(a6)                     | +092
        tst.w   0x76(a6)                        | +096
        beq.w   .L7feb2                         | +09a
        move.w  0x74(a6),d0                     | +09e
        lsr.w   #0x8,d0                         | +0a2
        lsr.w   #0x1,d0                         | +0a4
        lea     0x2e2320.l,a0                   | +0a6
        move.b  (a0,d0.w),d0                    | +0ac
        andi.w  #0xff,d0                        | +0b0
        add.w   d0,0x24(a6)                     | +0b4
.L7feb2:
        rts                                     | +0b8
        .global Squad_BobOscillate_07feb4
Squad_BobOscillate_07feb4:
        addi.w  #0x2,0x74(a6)                   | +0ba
        andi.w  #0x7f,0x74(a6)                  | +0c0
        move.w  0x76(a6),d0                     | +0c6
        sub.w   d0,0x24(a6)                     | +0ca
        move.w  0x74(a6),d0                     | +0ce
        add.w   d0,d0                           | +0d2
        lea     0x2c072c.l,a0                   | +0d4
        move.w  (a0,d0.w),d0                    | +0da
        muls.w  #0x4,d0                         | +0de
        asr.l   #0x8,d0                         | +0e2
        add.w   d0,0x24(a6)                     | +0e4

| ----------------------------------------------------------------------------
|  Squad_AIDecide_07fee8  @ $07FEE8  (326 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_AIDecide_07fee8, "ax", @progbits
        .global Squad_AIDecide_07fee8
Squad_AIDecide_07fee8:
        cmpi.w  #0xffff,0x80(a6)                | +000
        beq.w   .L7ff28                         | +006
        move.w  0x80(a6),0x66(a6)               | +00a
        tst.b   0x9d(a6)                        | +010
        beq.w   .L7ff0e                         | +014
        cmpi.w  #0x50,0x22(a6)                  | +018
        blt.w   .L7ff96                         | +01e
        bra.w   .L7ff18                         | +022
.L7ff0e:
        cmpi.w  #0xf0,0x22(a6)                  | +026
        bgt.w   .L7ff5e                         | +02c
.L7ff18:
        move.w  #0xffff,0x80(a6)                | +030
        lea     0x2e251c.l,a0                   | +036
        move.l  a0,0x48(a6)                     | +03c
.L7ff28:
        jsr     0x5e0d4.l                       | +040
        bcs.w   .L7ffbc                         | +046
        clr.b   d2                              | +04a
        move.w  0x22(a6),d0                     | +04c
        sub.w   0x22(a0),d0                     | +050
        cmpi.w  #0x60,d0                        | +054
        blt.w   .L7ff84                         | +058
        cmpi.w  #0x90,d0                        | +05c
        ble.w   .L7ffbc                         | +060
        moveq   #0x0,d0                         | +064
        move.b  0x98(a6),d0                     | +066
        asl.w   #0x4,d0                         | +06a
        cmp.w   0x22(a6),d0                     | +06c
        bge.w   .L7ffbc                         | +070
        moveq   #0x0,d0                         | +074
.L7ff5e:
        move.w  #0xfffe,d1                      | +076
        add.w   0x28(a6),d1                     | +07a
        cmpi.w  #0xfee0,d1                      | +07e
        bgt.w   .L7ff72                         | +082
        move.w  #0xfee0,d1                      | +086
.L7ff72:
        move.w  d1,0x28(a6)                     | +08a
        move.w  #0x18,0x70(a6)                  | +08e
        move.w  #0x1,0x7c(a6)                   | +094
        rts                                     | +09a
.L7ff84:
        moveq   #0x0,d0                         | +09c
        move.b  0x99(a6),d0                     | +09e
        asl.w   #0x4,d0                         | +0a2
        cmp.w   0x22(a6),d0                     | +0a4
        ble.w   .L7ffbc                         | +0a8
        moveq   #0x0,d0                         | +0ac
.L7ff96:
        move.w  #0x2,d1                         | +0ae
        add.w   0x28(a6),d1                     | +0b2
        cmpi.w  #0x120,d1                       | +0b6
        blt.w   .L7ffaa                         | +0ba
        move.w  #0x120,d1                       | +0be
.L7ffaa:
        move.w  d1,0x28(a6)                     | +0c2
        move.w  #0x30,0x70(a6)                  | +0c6
        move.w  #0x2,0x7c(a6)                   | +0cc
        rts                                     | +0d2
.L7ffbc:
        move.w  #0x24,0x70(a6)                  | +0d4
        move.w  0x28(a6),d1                     | +0da
        tst.w   d1                              | +0de
        bpl.w   .L7ffe0                         | +0e0
        move.w  #0x1,0x7c(a6)                   | +0e4
        addq.w  #0x6,d1                         | +0ea
        tst.w   d1                              | +0ec
        bpl.w   .L7ffee                         | +0ee
.L7ffda:
        move.w  d1,0x28(a6)                     | +0f2
        rts                                     | +0f6
.L7ffe0:
        move.w  #0x2,0x7c(a6)                   | +0f8
        addi.w  #0xfffa,d1                      | +0fe
        tst.w   d1                              | +102
        bpl.b   .L7ffda                         | +104
.L7ffee:
        jsr     0x267e2.l                       | +106
        move.w  #0x0,0x7c(a6)                   | +10c
        rts                                     | +112
        .global Squad_RankFromDist_07fffc
Squad_RankFromDist_07fffc:
        move.w  0x72(a6),d0                     | +114
        andi.l  #0xffff,d0                      | +118
        divu.w  #0x4,d0                         | +11e
        andi.l  #0xffff,d0                      | +122
        moveq   #0x0,d1                         | +128
        move.w  0x66(a6),d1                     | +12a
        divu.w  d0,d1                           | +12e
        move.w  #0x3,d0                         | +130
        sub.w   d1,d0                           | +134
        bpl.w   .L80024                         | +136
        clr.w   d0                              | +13a
.L80024:
        rts                                     | +13c
        .global Squad_ReadLeaderReward_080026
Squad_ReadLeaderReward_080026:
        movea.l 0xc(a6),a0                      | +13e
        move.w  0x7c(a0),d0                     | +142

| ----------------------------------------------------------------------------
|  Squad_LeaderGoneCheck_080034  @ $080034  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_LeaderGoneCheck_080034, "ax", @progbits
        .global Squad_LeaderGoneCheck_080034
Squad_LeaderGoneCheck_080034:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0xff,0x20(a0)                  | +004
        bne.w   ClearC_08004e                   | +00a
        jsr     0x4a166.l                       | +00e

| ----------------------------------------------------------------------------
|  Squad_DeferredRelease_080054  @ $080054  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_DeferredRelease_080054, "ax", @progbits
        .global Squad_DeferredRelease_080054
Squad_DeferredRelease_080054:
        jsr     Squad_LeaderGoneCheck_080034(pc) | +000
        tst.b   0x21(a6)                        | +004
        beq.w   JsrAbsThunk_080082              | +008
        jsr     0x2870a.l                       | +00c
        bcc.w   JsrAbsRts_080088                | +012
        movea.l 0xc(a6),a0                      | +016
        move.w  0x78(a6),d0                     | +01a
        bclr    d0,0x21(a0)                     | +01e
        tst.w   d0                              | +022
        bne.w   JsrAbsThunk_080082              | +024
        move.b  #0x1,0x78(a0)                   | +028

| ----------------------------------------------------------------------------
|  Squad_CompareDepth_08008a  @ $08008A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_CompareDepth_08008a, "ax", @progbits
        .global Squad_CompareDepth_08008a
Squad_CompareDepth_08008a:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0800a0                    | +00c

| ----------------------------------------------------------------------------
|  Squad_MgrInit_0800a6  @ $0800A6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_MgrInit_0800a6, "ax", @progbits
        .global Squad_MgrInit_0800a6
Squad_MgrInit_0800a6:
        moveq   #0x0,d0                         | +000
        move.b  d0,0x20(a6)                     | +002
        move.b  d0,0x21(a6)                     | +006
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0x3,d0                         | +010
        clr.w   d2                              | +014
        move.b  0x98(a6),d2                     | +016
        bne.w   Squad_MgrSpawnLoop_0800cc       | +01a

| ----------------------------------------------------------------------------
|  Squad_MgrSpawnLoop_0800cc  @ $0800CC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_MgrSpawnLoop_0800cc, "ax", @progbits
        .global Squad_MgrSpawnLoop_0800cc
Squad_MgrSpawnLoop_0800cc:
        subq.w  #0x1,d2                         | +000
        clr.w   d1                              | +002
.L800d0:
        movem.l d0-d2,-(a7)                     | +004
        lea     Squad_ChildInitTableA_080110(pc),a1 | +008
        jsr     0x4ae.l                         | +00c
        jsr     0x5dd02.l                       | +012
        movem.l (a7)+,d0-d2                     | +018
        move.w  d0,0x78(a0)                     | +01c
        move.w  d1,0x84(a0)                     | +020
        addq.w  #0x1,d1                         | +024
        dbra    d2,.L800d0                      | +026
        lea     .L800fc(pc),a1                  | +02a
        move.l  a1,(a6)                         | +02e
.L800fc:
        move.b  0x98(a6),d0                     | +030
        cmp.b   0x20(a6),d0                     | +034
        bne.w   SetHandlerRts_08010e            | +038

| ----------------------------------------------------------------------------
|  Squad_ChildInitTableA_080110  @ $080110  (214 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildInitTableA_080110, "ax", @progbits
        .global Squad_ChildInitTableA_080110
Squad_ChildInitTableA_080110:
        move.w  #0x132,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2e3ebc.l,a0                   | +00a
        move.w  0x78(a6),d0                     | +010
        lsl.w   #0x3,d0                         | +014
        movea.l (a0,d0.w),a1                    | +016
        movea.l 0x4(a0,d0.w),a2                 | +01a
        move.w  0x84(a6),d0                     | +01e
        move.b  (a2,d0.w),0x32(a6)              | +022
        move.b  (a2,d0.w),0x33(a6)              | +028
        add.w   d0,d0                           | +02e
        add.w   d0,d0                           | +030
        move.w  0x22(a6),d1                     | +032
        add.w   (a1,d0.w),d1                    | +036
        move.w  d1,0x22(a6)                     | +03a
        move.w  0x24(a6),d1                     | +03e
        add.w   0x2(a1,d0.w),d1                 | +042
        move.w  d1,0x24(a6)                     | +046
        clr.b   0x21(a6)                        | +04a
        jsr     0x267e2.l                       | +04e
        move.w  #0xfd00,0x28(a6)                | +054
        move.w  #0x1000,d0                      | +05a
        jsr     0x28134.l                       | +05e
        andi.w  #0xffe3,0x38(a6)                | +064
        ori.w   #0x8,0x38(a6)                   | +06a
        tst.w   0x84(a6)                        | +070
        bne.w   .L80192                         | +074
        move.w  #0x10ba,d0                      | +078
        jsr     0x2352.l                        | +07c
.L80192:
        lea     0x2e2fca.l,a0                   | +082
        jsr     0x28cd4.l                       | +088
        bclr    #0x3,0x13(a6)                   | +08e
        lea     TaskHandler_080996(pc),a1       | +094
        jsr     0x4ae.l                         | +098
        jsr     0x5dd02.l                       | +09e
        lea     .L801ba(pc),a1                  | +0a4
        move.l  a1,(a6)                         | +0a8
.L801ba:
        jsr     0x27cee.l                       | +0aa
        jsr     0x28d70.l                       | +0b0
        cmpi.w  #0xe0,0x22(a6)                  | +0b6
        bge.w   .L801d6                         | +0bc
        lea     Squad_ChildAltSprite_0801ee(pc),a1 | +0c0
        move.l  a1,(a6)                         | +0c4
.L801d6:
        movea.l #0xffffffff,a0                  | +0c6
        jsr     0x5dd5c.l                       | +0cc
        bcc.w   SetHandlerRts_0801ec            | +0d2

| ----------------------------------------------------------------------------
|  Squad_ChildAltSprite_0801ee  @ $0801EE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildAltSprite_0801ee, "ax", @progbits
        .global Squad_ChildAltSprite_0801ee
Squad_ChildAltSprite_0801ee:
        lea     0x2e2fd6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L80200(pc),a1                  | +00c
        move.l  a1,(a6)                         | +010
.L80200:
        jsr     0x27cee.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L80216                         | +01e
        lea     Squad_ChildLeap_08022e(pc),a1   | +022
        move.l  a1,(a6)                         | +026
.L80216:
        movea.l #0xffffffff,a0                  | +028
        jsr     0x5dd5c.l                       | +02e
        bcc.w   SetHandlerRts_08022c            | +034

| ----------------------------------------------------------------------------
|  Squad_ChildLeap_08022e  @ $08022E  (170 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildLeap_08022e, "ax", @progbits
        .global Squad_ChildLeap_08022e
Squad_ChildLeap_08022e:
        move.w  #0x400,d1                       | +000
        move.w  #0x88,d0                        | +004
        jsr     0x13c0e.l                       | +008
        move.w  d1,0x28(a6)                     | +00e
        move.w  d2,0x2a(a6)                     | +012
        clr.b   0x21(a6)                        | +016
        move.w  #0xffe0,0x2c(a6)                | +01a
        tst.w   0x84(a6)                        | +020
        bne.w   .L80260                         | +024
        move.w  #0x10b9,d0                      | +028
        jsr     0x2352.l                        | +02c
.L80260:
        lea     .L80266(pc),a1                  | +032
        move.l  a1,(a6)                         | +036
.L80266:
        jsr     0x27cee.l                       | +038
        tst.b   0x21(a6)                        | +03e
        bne.w   .L8029a                         | +042
        cmpi.b  #0x80,0x32(a6)                  | +046
        bcs.w   .L8029a                         | +04c
        move.b  #0xff,0x21(a6)                  | +050
        move.w  #0x1000,d0                      | +056
        jsr     0x28134.l                       | +05a
        andi.w  #0xffe3,0x38(a6)                | +060
        ori.w   #0xc,0x38(a6)                   | +066
.L8029a:
        jsr     0x28d70.l                       | +06c
        cmpi.b  #0xff,0x32(a6)                  | +072
        beq.w   .L802c8                         | +078
        addq.b  #0x3,0x32(a6)                   | +07c
        addq.b  #0x3,0x33(a6)                   | +080
        cmpi.b  #0x2,0x32(a6)                   | +084
        bhi.w   .L802c8                         | +08a
        move.b  #0xff,0x32(a6)                  | +08e
        move.b  #0xff,0x33(a6)                  | +094
.L802c8:
        movea.l #0xffffffff,a0                  | +09a
        jsr     0x5dd5c.l                       | +0a0
        bcc.w   SetHandlerRts_0802de            | +0a6

| ----------------------------------------------------------------------------
|  Squad_ChildInitFixed_0802e0  @ $0802E0  (154 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildInitFixed_0802e0, "ax", @progbits
        .global Squad_ChildInitFixed_0802e0
Squad_ChildInitFixed_0802e0:
        move.w  #0x133,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x267e2.l                       | +00a
        lea     0x2beed0.l,a0                   | +010
        move.w  0x78(a6),d0                     | +016
        lsl.w   #0x3,d0                         | +01a
        move.w  (a0,d0.w),0x22(a6)              | +01c
        move.w  0x2(a0,d0.w),0x24(a6)           | +022
        move.w  0x4(a0,d0.w),0x72(a6)           | +028
        move.b  #0x80,0x32(a6)                  | +02e
        move.b  #0x80,0x33(a6)                  | +034
        move.b  #0xff,0x21(a6)                  | +03a
        move.w  #0x1000,d0                      | +040
        jsr     0x28134.l                       | +044
        andi.w  #0xffe3,0x38(a6)                | +04a
        ori.w   #0xc,0x38(a6)                   | +050
        lea     0x2e3d68.l,a0                   | +056
        move.w  0x78(a6),d0                     | +05c
        add.w   d0,d0                           | +060
        add.w   d0,d0                           | +062
        move.l  (a0,d0.w),0x90(a6)              | +064
        moveq   #0x0,d1                         | +06a
        move.w  d1,0x94(a6)                     | +06c
        move.b  d1,0x96(a6)                     | +070
        clr.b   0x7e(a6)                        | +074
        move.w  #0x10ba,d0                      | +078
        jsr     0x2352.l                        | +07c
        lea     0x723da.l,a1                    | +082
        jsr     0x4ae.l                         | +088
        jsr     0x5dd02.l                       | +08e
        move.w  #0xfff0,0x98(a0)                | +094

| ----------------------------------------------------------------------------
|  Squad_ChildRun_080382  @ $080382  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildRun_080382, "ax", @progbits
        .global Squad_ChildRun_080382
Squad_ChildRun_080382:
        move.w  0x84(a6),d0                     | +000
        add.w   d0,d0                           | +004
        addi.w  #0x80,d0                        | +006
        movea.l 0xc(a6),a0                      | +00a
        movea.l 0xc(a0),a0                      | +00e
        move.w  (a0,d0.w),0x66(a6)              | +012
        lea     .L803a0(pc),a1                  | +018
        move.l  a1,(a6)                         | +01c
.L803a0:
        jsr     Sub_0008179E(pc)                | +01e
        jsr     0x78f8a.l                       | +022
        bcc.w   .L803b4                         | +028
        lea     Squad_NotifyParent_0803d2(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L803b4:
        jsr     0x28d70.l                       | +032
        movea.l #0xffffffff,a0                  | +038
        jsr     0x5dd5c.l                       | +03e
        bcc.w   SetHandlerRts_0803d0            | +044

| ----------------------------------------------------------------------------
|  Squad_NotifyParent_0803d2  @ $0803D2  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_NotifyParent_0803d2, "ax", @progbits
        .global Squad_NotifyParent_0803d2
Squad_NotifyParent_0803d2:
        movea.l 0xc(a6),a0                      | +000
        addq.b  #0x1,0x20(a0)                   | +004

| ----------------------------------------------------------------------------
|  Squad_Mgr8Slots_0803e8  @ $0803E8  (280 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_Mgr8Slots_0803e8, "ax", @progbits
        .global Squad_Mgr8Slots_0803e8
Squad_Mgr8Slots_0803e8:
        move.w  #0xffff,d0                      | +000
        move.w  d0,0x80(a6)                     | +004
        move.w  d0,0x82(a6)                     | +008
        move.w  d0,0x84(a6)                     | +00c
        move.w  d0,0x86(a6)                     | +010
        move.w  d0,0x88(a6)                     | +014
        move.w  d0,0x8a(a6)                     | +018
        move.w  d0,0x8c(a6)                     | +01c
        move.w  d0,0x8e(a6)                     | +020
        moveq   #0x0,d0                         | +024
        move.w  d0,0x70(a6)                     | +026
        move.b  d0,0x76(a6)                     | +02a
        move.b  d0,0x78(a6)                     | +02e
        move.b  d0,0x77(a6)                     | +032
        lea     0x2bea3e.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x72(a6)                     | +042
        lea     0x2beac0.l,a0                   | +046
        jsr     0x799de.l                       | +04c
        move.w  d0,0x74(a6)                     | +052
        lea     0x2beb42.l,a0                   | +056
        jsr     0x799de.l                       | +05c
        move.w  d0,0x66(a6)                     | +062
        lea     Squad_MgrWait_080454(pc),a1     | +066
        move.l  a1,(a6)                         | +06a
        .global Squad_MgrWait_080454
Squad_MgrWait_080454:
        tst.b   0x99(a6)                        | +06c
        bne.w   .L80466                         | +070
        bclr    #0x1,0x12(a6)                   | +074
        jmp     Sub_000811D6(pc)                | +07a
.L80466:
        addq.w  #0x1,0x70(a6)                   | +07e
        move.w  0x70(a6),d0                     | +082
        cmp.w   0x74(a6),d0                     | +086
        blt.w   SetHandlerRts_080506            | +08a
        move.w  #0x0,0x70(a6)                   | +08e
        clr.w   d0                              | +094
.L8047e:
        btst    d0,0x77(a6)                     | +096
        beq.w   .L8048e                         | +09a
        btst    d0,0x78(a6)                     | +09e
        beq.w   .L804ac                         | +0a2
.L8048e:
        addq.w  #0x1,d0                         | +0a6
        cmpi.w  #0x8,d0                         | +0a8
        blt.b   .L8047e                         | +0ac
        clr.w   d0                              | +0ae
.L80498:
        btst    d0,0x77(a6)                     | +0b0
        beq.w   .L804ac                         | +0b4
        addq.w  #0x1,d0                         | +0b8
        cmpi.w  #0x8,d0                         | +0ba
        blt.b   .L80498                         | +0be
        bra.w   SetTaskHandler_080500           | +0c0
.L804ac:
        move.l  d0,-(a7)                        | +0c4
        lea     Squad_ChildSpawnInit_080540(pc),a1 | +0c6
        jsr     0x4ae.l                         | +0ca
        jsr     0x5dd02.l                       | +0d0
        move.l  (a7)+,d0                        | +0d6
        move.w  d0,0x84(a0)                     | +0d8
        bset    d0,0x77(a6)                     | +0dc
        bset    d0,0x78(a6)                     | +0e0
        add.w   d0,d0                           | +0e4
        addi.w  #0x80,d0                        | +0e6
        cmpi.w  #0xffff,(a6,d0.w)               | +0ea
        bne.w   .L804e2                         | +0f0
        move.w  0x66(a6),(a6,d0.w)              | +0f4
.L804e2:
        move.w  0x66(a6),0x66(a0)               | +0fa
        addq.b  #0x1,0x76(a6)                   | +100
        move.b  0x76(a6),d0                     | +104
        cmp.b   0x99(a6),d0                     | +108
        bge.w   SetTaskHandler_080500           | +10c
        cmp.b   0x9a(a6),d0                     | +110
        blt.w   SetHandlerRts_080506            | +114

| ----------------------------------------------------------------------------
|  Squad_MgrIdle_080508  @ $080508  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_MgrIdle_080508, "ax", @progbits
        .global Squad_MgrIdle_080508
Squad_MgrIdle_080508:
        tst.b   0x99(a6)                        | +000
        bne.w   .L8051a                         | +004
        bclr    #0x1,0x12(a6)                   | +008
        jmp     Sub_000811D6(pc)                | +00e
.L8051a:
        tst.b   0x76(a6)                        | +012
        bne.w   SetHandlerRts_08053e            | +016
        addq.w  #0x1,0x70(a6)                   | +01a
        move.w  0x70(a6),d0                     | +01e
        cmp.w   0x72(a6),d0                     | +022
        blt.w   SetHandlerRts_08053e            | +026
        move.w  #0x0,0x70(a6)                   | +02a

| ----------------------------------------------------------------------------
|  Squad_ChildSpawnInit_080540  @ $080540  (166 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildSpawnInit_080540, "ax", @progbits
        .global Squad_ChildSpawnInit_080540
Squad_ChildSpawnInit_080540:
        move.w  #0x48,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        bset    #0x1,0x12(a6)                   | +016
        move.w  #0xc000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x14,0x38(a6)                  | +02c
        move.l  #0x2e3aa4,0x48(a6)              | +032
        clr.b   0x20(a6)                        | +03a
        jsr     Sub_0008169A(pc)                | +03e
        lea     Squad_ChildInitFixed_0802e0(pc),a1 | +042
        jsr     0x4ae.l                         | +046
        move.w  0x84(a6),0x84(a0)               | +04c
        move.w  0x78(a6),0x78(a0)               | +052
        move.w  0x66(a6),0x66(a0)               | +058
        lea     .L805a4(pc),a1                  | +05e
        move.l  a1,(a6)                         | +062
.L805a4:
        tst.b   0x20(a6)                        | +064
        bne.w   .L805ae                         | +068
        rts                                     | +06c
.L805ae:
        lea     0x723da.l,a1                    | +06e
        jsr     0x4ae.l                         | +074
        jsr     0x5dd02.l                       | +07a
        move.w  #0xffe0,0x98(a0)                | +080
        lea     TaskHandler_0809cc(pc),a1       | +086
        jsr     0x4ae.l                         | +08a
        jsr     0x5dd02.l                       | +090
        lea     Squad_CmdrInit_08067e(pc),a1    | +096
        jsr     0x4ae.l                         | +09a
        jsr     0x5dd02.l                       | +0a0

| ----------------------------------------------------------------------------
|  Squad_ChildActive_0805ee  @ $0805EE  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_ChildActive_0805ee, "ax", @progbits
        .global Squad_ChildActive_0805ee
Squad_ChildActive_0805ee:
        move.w  0x84(a6),d0                     | +000
        add.w   d0,d0                           | +004
        addi.w  #0x80,d0                        | +006
        movea.l 0xc(a6),a0                      | +00a
        move.w  (a0,d0.w),0x66(a6)              | +00e
        lea     .L80608(pc),a1                  | +014
        move.l  a1,(a6)                         | +018
.L80608:
        jsr     Sub_0008179E(pc)                | +01a
        jsr     0x78f8a.l                       | +01e
        bcc.w   .L8061c                         | +024
        lea     TaskHandler_081196(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L8061c:
        jsr     0x28d70.l                       | +02e
        jsr     0x2870a.l                       | +034
        bcc.w   .L80646                         | +03a
        bclr    #0x3,0x13(a6)                   | +03e
        tst.b   0x21(a6)                        | +044
        bne.w   .L80646                         | +048
        lea     0x2e3566.l,a0                   | +04c
        jsr     0x28cd4.l                       | +052
.L80646:
        lea     0x5e766.l,a0                    | +058
        jsr     0x5e770.l                       | +05e
        jsr     0x28758.l                       | +064
        bcc.w   .L80660                         | +06a
        jsr     Sub_00080AAA(pc)                | +06e
.L80660:
        movea.l #0xffffffff,a0                  | +072
        lea     0x2e3c44.l,a0                   | +078
        jsr     0x5dd5c.l                       | +07e
        bcc.w   SetHandlerRts_08067c            | +084

| ----------------------------------------------------------------------------
|  Squad_CmdrInit_08067e  @ $08067E  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Squad_CmdrInit_08067e, "ax", @progbits
        .global Squad_CmdrInit_08067e
Squad_CmdrInit_08067e:
        move.w  #0x49,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x170,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x171,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  #0x6,0x1c(a6)                   | +01e
        jsr     0x138fe.l                       | +024
        lea     0x2bef82.l,a0                   | +02a
        jsr     0x799de.l                       | +030
        move.w  d0,0x36(a6)                     | +036
        lea     0x2e3916.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L806ca(pc),a1                  | +046
        move.l  a1,(a6)                         | +04a
.L806ca:
        jsr     0x5e506.l                       | +04c
        jsr     Sub_000808E8(pc)                | +052
        bcs.w   .L806e8                         | +056
        jsr     Sub_000808A0(pc)                | +05a
        jsr     0x5e826.l                       | +05e
        jsr     0x28d70.l                       | +064
.L806e8:
        jsr     Sub_00080932(pc)                | +06a
        bcc.w   .L806f6                         | +06e
        lea     TaskHandler_080736(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L806f6:
        jsr     Sub_0008094A(pc)                | +078
        bcc.w   Squad_CmdrTick_080704           | +07c
        lea     TaskHandler_080736(pc),a1       | +080
        move.l  a1,(a6)                         | +084
        .global Squad_CmdrTick_080704
Squad_CmdrTick_080704:
        movea.l 0xc(a6),a0                      | +086
        cmpi.b  #0xff,0x20(a0)                  | +08a
        bne.w   .L80718                         | +090
        lea     JmpToScheduler_081214(pc),a1    | +094
        move.l  a1,(a6)                         | +098
.L80718:
        movea.l #0xffffffff,a0                  | +09a
        lea     0x2e3c44.l,a0                   | +0a0
        jsr     0x5dd5c.l                       | +0a6
        bcc.w   SetHandlerRts_080734            | +0ac
