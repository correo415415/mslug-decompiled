| ============================================================================
|  Metal Slug 1 - asm/mission_spawn_boss_0448a6.s
|  ----------------------------------------------------------------------------
|  Wave XX (2/3) - Spawn_FromStream + state-machine del jefe. Cluster
|  $0448A6..$044F82, 20 entradas, 1,684 B.
|
|  Spawn_FromStream_0448A6: materializa UN enemigo desde un registro de
|  18 B del stream de mision:
|    +0  guard 0 (trap #15 assert)   +1  flags: b0-1 gate dificultad
|        (3 = check $2AC6A, else cmp $106ED1), b2-3 subtipo, b4 no-count,
|        b6/b7 XY absolutos (sin restar scroll $106F50/$106F54)
|    +4  indice en la tabla de plantillas $E8000 (handler de tarea)
|    +6/+8  XY                        +A..+11  8 B copiados a $98(a0)
|  Crea la tarea via scheduler $4AE sobre el pool $100800, fija pos
|  ($22/$24, Y invertida 512-y), marca b1 de $12(a0) si cuenta para el
|  contador de vivos $106E8A y aplica el delta de camara ($4405E) si no
|  es contexto $100800. Spawn_MarkPending ($04498E) es el fallback si el
|  gate de dificultad rechaza: crea la tarea marcada como ya-muerta.
|
|  STATE-MACHINE DEL JEFE (cadena de handlers self-replace `lea (pc),a1;
|  move.l a1,(a6)`):
|    Boss_Intro_0449C2    warp a (160,328) + anim $2891EA + reset fisica
|    Boss_WaitOne_044A12  espera evento d1=1 via $5E912
|    Boss_EngageA_044A2C  musica ($236E d1=2 o d1=$4E segun $7A)
|    Boss_Rearm_044A50    recarga: timer $7C=$6000, anim $29F840
|    Boss_Active_044A7E   nucleo: AimInit+FaceTarget+AimUpdate cada frame,
|                         gate de fase via FireGate $45648 y tick padre
|                         $8F714 (d1=8); transita a PhaseFire o Descend
|    Boss_PhaseFire_044AFE fuego: hasta 2 rondas segun $5E3A2(0/1),
|                         spawnea protos $30C70/$30C14 + sfx $108C/$1022
|    Boss_Descend_044C28  descenso: baja $80(a6) += $106F6C hasta 480px,
|                         explosion $77F6A + anim $29D3A8 al tocar suelo
|  BossShot_Init_044D40 / BossShot_Fly_044DA4: proyectil con vel $D000,
|  rand $5DCA4(#$38) en X, timeout $45(a6)=60 frames y estela $1022.
|
|  MINIBOSS (montura): Miniboss_SpawnP1/P2/Alt eligen contexto de player
|  ($100440/$1004E0/$100580) + musica; Attach ($044E42) espera b0 de
|  $6B(a0) y b3 de $8C(a0) para montar (vel $C000 via $28134, anim
|  $29D090); Ride ($044EA2) sigue al player con colision rect $28C20
|  contra Boss_HitboxTable_044D30; Dead/Hop gestionan el desmonte.
|
|  Todo byte-exacto contra la ROM (verificado por match_batch).
| ============================================================================

        .globl  Spawn_FromStream_0448A6
        .type   Spawn_FromStream_0448A6, @function
        .section .text.Spawn_FromStream_0448A6, "ax", @progbits
Spawn_FromStream_0448A6:
        cmpi.b  #0x0, (a1)                             | +000
        beq.w   .L448ba                                | +004
        nop                                            | +008
        nop                                            | +00a
        cmpi.b  #0x0, (a1)                             | +00c
        nop                                            | +010
        trap    #15                                    | +012
.L448ba:
        move.b  0x1(a1), d7                            | +014
        andi.b  #0x3, d7                               | +018
        beq.w   .L448e6                                | +01c
        cmpi.b  #0x3, d7                               | +020
        bne.w   .L448dc                                | +024
        jsr     0x2ac6a.l                              | +028
        bcc.w   .L4498c                                | +02e
        bra.w   .L448e6                                | +032
.L448dc:
        cmp.b   0x106ed1.l, d7                         | +036
        bne.w   .L4498c                                | +03c
.L448e6:
        moveq   #0x0, d0                               | +040
        move.w  0x4(a1), d0                            | +042
        add.w   d0, d0                                 | +046
        add.w   d0, d0                                 | +048
        lea     0xe8000.l, a0                          | +04a
        movem.l a1/a6, -(a7)                           | +050
        movea.l (a0, d0.w), a1                         | +054
        lea     0x100800.l, a6                         | +058
        jsr     0x4ae.l                                | +05e
        movem.l (a7)+, a1/a6                           | +064
        move.b  0x1(a1), d7                            | +068
        btst    #0x4, d7                               | +06c
        bne.w   .L44920                                | +070
        bset    #0x1, 0x12(a0)                         | +074
.L44920:
        move.w  0x6(a1), d0                            | +07a
        btst    #0x7, d7                               | +07e
        bne.w   .L44932                                | +082
        sub.w   0x106f50.l, d0                         | +086
.L44932:
        move.w  d0, 0x22(a0)                           | +08c
        move.w  0x8(a1), d0                            | +090
        btst    #0x6, d7                               | +094
        bne.w   .L44948                                | +098
        sub.w   0x106f54.l, d0                         | +09c
.L44948:
        neg.w   d0                                     | +0a2
        addi.w  #0x200, d0                             | +0a4
        move.w  d0, 0x24(a0)                           | +0a8
        cmpa.l  #0x100800, a6                          | +0ac
        beq.w   .L44972                                | +0b2
        btst    #0x1, 0x12(a0)                         | +0b6
        beq.w   .L4496c                                | +0bc
        addq.w  #0x1, 0x106e8a.l                       | +0c0
.L4496c:
        jsr     0x4405e.l                              | +0c6
.L44972:
        andi.b  #0xc, d7                               | +0cc
        lsr.w   #0x2, d7                               | +0d0
        move.b  d7, 0x11(a0)                           | +0d2
        lea     0xa(a1), a1                            | +0d6
        lea     0x98(a0), a2                           | +0da
        moveq   #0x7, d7                               | +0de
.L44986:
        move.b  (a1)+, (a2)+                           | +0e0
        dbra    d7, .L44986                            | +0e2
.L4498c:
        rts                                            | +0e6
        .size   Spawn_FromStream_0448A6, .-Spawn_FromStream_0448A6

        .globl  Spawn_MarkPending_04498E
        .type   Spawn_MarkPending_04498E, @function
        .section .text.Spawn_MarkPending_04498E, "ax", @progbits
Spawn_MarkPending_04498E:
        move.l  a6, -(a7)                              | +000
        lea     0x100800.l, a6                         | +002
        jsr     0x4ae.l                                | +008
        bset    #0x1, 0x12(a0)                         | +00e
        movea.l (a7)+, a6                              | +014
        rts                                            | +016
        .size   Spawn_MarkPending_04498E, .-Spawn_MarkPending_04498E

        .globl  Entity_CmpDepthToParent_0449A6
        .type   Entity_CmpDepthToParent_0449A6, @function
        .section .text.Entity_CmpDepthToParent_0449A6, "ax", @progbits
Entity_CmpDepthToParent_0449A6:
        movea.l 0x8(a6), a1                            | +000
        move.b  0x10(a6), d0                           | +004
        cmp.b   0x10(a1), d0                           | +008
        bcs.w   SetXN_0449bc                           | +00c
        .size   Entity_CmpDepthToParent_0449A6, .-Entity_CmpDepthToParent_0449A6

        .globl  Boss_Intro_0449C2
        .type   Boss_Intro_0449C2, @function
        .section .text.Boss_Intro_0449C2, "ax", @progbits
Boss_Intro_0449C2:
        move.w  #0xa0, 0x22(a6)                        | +000
        move.w  #0x148, 0x24(a6)                       | +006
        move.w  #0x3, d1                               | +00c
        jsr     0x2a1aa.l                              | +010
        lea     Boss_EngageA_044A2C(pc), a1            | +016
        jsr     0x4ae.l                                | +01a
        jsr     0x5dd02.l                              | +020
        lea     0x2891ea.l, a0                         | +026
        jsr     0x28cd4.l                              | +02c
        clr.w   0x28(a6)                               | +032
        clr.w   0x2a(a6)                               | +036
        clr.w   0x2c(a6)                               | +03a
        clr.w   0x2e(a6)                               | +03e
        clr.b   0x26(a6)                               | +042
        clr.b   0x27(a6)                               | +046
        lea     Boss_WaitOne_044A12(pc), a1            | +04a
        move.l  a1, (a6)                               | +04e
        .size   Boss_Intro_0449C2, .-Boss_Intro_0449C2

        .globl  Boss_WaitOne_044A12
        .type   Boss_WaitOne_044A12, @function
        .section .text.Boss_WaitOne_044A12, "ax", @progbits
Boss_WaitOne_044A12:
        move.w  #0x1, d1                               | +000
        cmpi.w  #0x1, d1                               | +004
        beq.w   JsrAbsThunk_044a24                     | +008
        jsr     0x5e912.l                              | +00c
        .size   Boss_WaitOne_044A12, .-Boss_WaitOne_044A12

        .globl  Boss_EngageA_044A2C
        .type   Boss_EngageA_044A2C, @function
        .section .text.Boss_EngageA_044A2C, "ax", @progbits
Boss_EngageA_044A2C:
        move.b  #0x0, 0x7a(a6)                         | +000
        move.w  #0x2, d1                               | +006
        jsr     0x236e.l                               | +00a
        bra.w   Boss_Rearm_044A50                      | +010
        move.b  #0x1, 0x7a(a6)                         | +014
        move.w  #0x4e, d1                              | +01a
        jsr     0x236e.l                               | +01e
        .size   Boss_EngageA_044A2C, .-Boss_EngageA_044A2C

        .globl  Boss_Rearm_044A50
        .type   Boss_Rearm_044A50, @function
        .section .text.Boss_Rearm_044A50, "ax", @progbits
Boss_Rearm_044A50:
        bsr.w   Ent_AimInit_045412                     | +000
        move.w  #0x6000, d0                            | +004
        move.w  d0, 0x7c(a6)                           | +008
        move.b  0x7c(a6), d0                           | +00c
        addq.b  #0x4, d0                               | +010
        lsr.b   #0x3, d0                               | +012
        andi.w  #0x1f, d0                              | +014
        move.w  d0, 0x34(a6)                           | +018
        lea     0x29f840.l, a0                         | +01c
        jsr     0x28cd4.l                              | +022
        lea     Boss_Active_044A7E(pc), a1             | +028
        move.l  a1, (a6)                               | +02c
        .size   Boss_Rearm_044A50, .-Boss_Rearm_044A50

        .globl  Boss_Active_044A7E
        .type   Boss_Active_044A7E, @function
        .section .text.Boss_Active_044A7E, "ax", @progbits
Boss_Active_044A7E:
        bsr.w   Ent_InputSample_04546E                 | +000
        bsr.w   Ent_FaceTarget_044FA0                  | +004
        movea.l 0xc(a6), a0                            | +008
        bclr    #0x2, 0x8c(a0)                         | +00c
        jsr     Ent_AimUpdate_045022(pc)               | +012
        addq.b  #0x4, d0                               | +016
        lsr.b   #0x3, d0                               | +018
        andi.w  #0x1f, d0                              | +01a
        move.w  d0, 0x34(a6)                           | +01e
        jsr     0x28d70.l                              | +022
        .globl  Boss_ActiveWait_044AA6
Boss_ActiveWait_044AA6:
        movea.l 0xc(a6), a0                            | +028
        bclr    #0x3, 0x8c(a0)                         | +02c
        jsr     Ent_FireGate_045648(pc)                | +032
        bcc.w   .L44abe                                | +036
        lea     Boss_PhaseFire_044AFE(pc), a1          | +03a
        move.l  a1, (a6)                               | +03e
.L44abe:
        movem.l a6, -(a7)                              | +040
        movea.l 0xc(a6), a6                            | +044
        move.b  #0x8, d1                               | +048
        jsr     0x8f714.l                              | +04c
        movem.l (a7)+, a6                              | +052
        bcc.w   .L44ade                                | +056
        lea     Boss_Descend_044C28(pc), a1            | +05a
        move.l  a1, (a6)                               | +05e
.L44ade:
        jsr     0x5e452.l                              | +060
        bcc.w   SetTaskHandler_044af6                  | +066
        movea.l 0xc(a6), a0                            | +06a
        btst    #0x0, 0x13(a0)                         | +06e
        beq.w   SetHandlerRts_044afc                   | +074
        .size   Boss_Active_044A7E, .-Boss_Active_044A7E

        .globl  Boss_PhaseFire_044AFE
        .type   Boss_PhaseFire_044AFE, @function
        .section .text.Boss_PhaseFire_044AFE, "ax", @progbits
Boss_PhaseFire_044AFE:
        jsr     Ent_GroundProbe_04572C(pc)             | +000
        bcc.w   .L44b12                                | +004
        lea     0x29e864.l, a0                         | +008
        jsr     0x28cd4.l                              | +00e
.L44b12:
        addq.b  #0x1, 0x78(a6)                         | +014
        move.w  #0x0, d0                               | +018
        jsr     0x5e3a2.l                              | +01c
        bcc.w   .L44b3c                                | +022
        move.w  #0x1, d0                               | +026
        jsr     0x5e3a2.l                              | +02a
        bcc.w   .L44b3c                                | +030
        andi.b  #0x3, 0x78(a6)                         | +034
        bra.w   .L44b42                                | +03a
.L44b3c:
        andi.b  #0x1, 0x78(a6)                         | +03e
.L44b42:
        tst.b   0x78(a6)                               | +044
        bne.w   .L44b5c                                | +048
        lea     0x30c70.l, a1                          | +04c
        jsr     0x4ae.l                                | +052
        jsr     0x5dd02.l                              | +058
.L44b5c:
        move.w  #0x108c, d0                            | +05e
        jsr     0x2352.l                               | +062
        lea     0x308c2.l, a1                          | +068
        jsr     0x6fe.l                                | +06e
        jsr     0x5dd02.l                              | +074
        jsr     0x517fe.l                              | +07a
        move.b  0x35(a6), d0                           | +080
        lsl.b   #0x3, d0                               | +084
        move.b  d0, 0x98(a0)                           | +086
        move.b  0x79(a6), 0x9a(a0)                     | +08a
        move.b  0x82(a6), 0x9b(a0)                     | +090
        addi.b  #0x1, 0x82(a6)                         | +096
        movea.l 0xc(a6), a0                            | +09c
        bset    #0x2, 0x8c(a0)                         | +0a0
        lea     .L44bae(pc), a1                        | +0a6
        move.l  a1, (a6)                               | +0aa
        bra.w   .L44bbe                                | +0ac
.L44bae:
        movea.l 0xc(a6), a0                            | +0b0
        bclr    #0x2, 0x8c(a0)                         | +0b4
        lea     .L44bbe(pc), a1                        | +0ba
        move.l  a1, (a6)                               | +0be
.L44bbe:
        jsr     Ent_InputSample_04546E(pc)             | +0c0
        bsr.w   Ent_FaceTarget_044FA0                  | +0c4
        jsr     Ent_AimUpdate_045022(pc)               | +0c8
        move.w  0x34(a6), d1                           | +0cc
        addq.b  #0x4, d0                               | +0d0
        lsr.b   #0x3, d0                               | +0d2
        andi.w  #0x1f, d0                              | +0d4
        move.w  d0, 0x34(a6)                           | +0d8
        jsr     Ent_GroundProbe_04572C(pc)             | +0dc
        bcs.w   .L44c10                                | +0e0
        cmpi.w  #0x0, 0x88(a6)                         | +0e4
        bne.w   .L44c10                                | +0ea
        lea     0x30c14.l, a1                          | +0ee
        jsr     0x4ae.l                                | +0f4
        jsr     0x5dd02.l                              | +0fa
        move.b  0x35(a6), 0x98(a0)                     | +100
        lea     0x29f840.l, a0                         | +106
        jsr     0x28cd4.l                              | +10c
.L44c10:
        jsr     0x28d70.l                              | +112
        bra.w   Boss_ActiveWait_044AA6                 | +118
        .size   Boss_PhaseFire_044AFE, .-Boss_PhaseFire_044AFE

        .globl  Boss_Descend_044C28
        .type   Boss_Descend_044C28, @function
        .section .text.Boss_Descend_044C28, "ax", @progbits
Boss_Descend_044C28:
        move.w  #0x106a, d0                            | +000
        jsr     0x2352.l                               | +004
        clr.w   0x80(a6)                               | +00a
        move.l  a6, -(a7)                              | +00e
        lea     0x100800.l, a6                         | +010
        lea     BossShot_Init_044D40(pc), a1           | +016
        jsr     0x4ae.l                                | +01a
        movea.l (a7)+, a6                              | +020
        move.w  0x22(a6), 0x22(a0)                     | +022
        move.w  0x24(a6), 0x24(a0)                     | +028
        move.b  0x7a(a6), 0x98(a0)                     | +02e
        lea     0x77f6a.l, a1                          | +034
        jsr     0x6fe.l                                | +03a
        jsr     0x5dd02.l                              | +040
        lea     .L44c74(pc), a1                        | +046
        move.l  a1, (a6)                               | +04a
.L44c74:
        move.w  0x80(a6), d0                           | +04c
        add.w   0x106f6c.l, d0                         | +050
        move.w  d0, 0x80(a6)                           | +056
        cmpi.w  #0x1e0, d0                             | +05a
        blt.w   .L44c90                                | +05e
        lea     .L44ca8(pc), a1                        | +062
        move.l  a1, (a6)                               | +066
.L44c90:
        movem.l a6, -(a7)                              | +068
        movea.l 0xc(a6), a6                            | +06c
        move.b  #0x1, d1                               | +070
        jsr     0x8f6f2.l                              | +074
        movem.l (a7)+, a6                              | +07a
        rts                                            | +07e
.L44ca8:
        movea.l 0xc(a6), a0                            | +080
        bset    #0x3, 0x8c(a0)                         | +084
        bclr    #0x4, 0x8c(a0)                         | +08a
        lea     .L44cbe(pc), a1                        | +090
        move.l  a1, (a6)                               | +094
.L44cbe:
        movea.l 0xc(a6), a0                            | +096
        btst    #0x4, 0x8c(a0)                         | +09a
        beq.w   .L44cd8                                | +0a0
        bclr    #0x3, 0x8c(a0)                         | +0a4
        lea     .L44cda(pc), a1                        | +0aa
        move.l  a1, (a6)                               | +0ae
.L44cd8:
        bra.b   .L44c90                                | +0b0
.L44cda:
        move.w  #0x106a, d0                            | +0b2
        jsr     0x2352.l                               | +0b6
        lea     0x29d3a8.l, a0                         | +0bc
        jsr     0x28cd4.l                              | +0c2
        lea     .L44cf6(pc), a1                        | +0c8
        move.l  a1, (a6)                               | +0cc
.L44cf6:
        bsr.w   Ent_FaceTarget_044FA0                  | +0ce
        jsr     0x28d70.l                              | +0d2
        bcc.w   .L44d0a                                | +0d8
        lea     Boss_Rearm_044A50(pc), a1              | +0dc
        move.l  a1, (a6)                               | +0e0
.L44d0a:
        jsr     0x5e452.l                              | +0e2
        bcs.w   .L44d1a                                | +0e8
        lea     Jsr5B6ThenJmpScheduler_044c1a(pc), a1  | +0ec
        move.l  a1, (a6)                               | +0f0
.L44d1a:
        movea.l 0xc(a6), a0                            | +0f2
        cmpi.l  #0x2dc5c, (a0)                         | +0f6
        bne.w   SetHandlerRts_044d2e                   | +0fc
        .size   Boss_Descend_044C28, .-Boss_Descend_044C28

        .globl  Boss_HitboxTable_044D30
        .section .text.Boss_HitboxTable_044D30, "ax", @progbits
Boss_HitboxTable_044D30:
        .word   0xFFE8, 0x0018, 0xFFFC, 0x0008   | rect A
        .globl  Boss_HitboxB_044D38
Boss_HitboxB_044D38:
        .word   0x0000, 0x0001, 0x0000, 0x0001   | rect B
        .size   Boss_HitboxTable_044D30, .-Boss_HitboxTable_044D30

        .globl  BossShot_Init_044D40
        .type   BossShot_Init_044D40, @function
        .section .text.BossShot_Init_044D40, "ax", @progbits
BossShot_Init_044D40:
        move.w  #0xd000, d0                            | +000
        jsr     0x28134.l                              | +004
        andi.w  #0xffe3, 0x38(a6)                      | +00a
        ori.w   #0x10, 0x38(a6)                        | +010
        move.w  #0x2, d1                               | +016
        tst.b   0x98(a6)                               | +01a
        beq.w   .L44d66                                | +01e
        move.w  #0x4e, d1                              | +022
.L44d66:
        jsr     0x236e.l                               | +026
        move.w  #0x38, d0                              | +02c
        jsr     0x5dca4.l                              | +030
        move.w  d0, 0x28(a6)                           | +036
        move.w  #0x384, 0x2a(a6)                       | +03a
        move.w  #0xffec, 0x2e(a6)                      | +040
        move.w  #0x0, 0x2c(a6)                         | +046
        move.b  #0x3c, 0x45(a6)                        | +04c
        lea     0x29d090.l, a0                         | +052
        jsr     0x28cd4.l                              | +058
        lea     BossShot_Fly_044DA4(pc), a1            | +05e
        move.l  a1, (a6)                               | +062
        .size   BossShot_Init_044D40, .-BossShot_Init_044D40

        .globl  BossShot_Fly_044DA4
        .type   BossShot_Fly_044DA4, @function
        .section .text.BossShot_Fly_044DA4, "ax", @progbits
BossShot_Fly_044DA4:
        jsr     0x27d50.l                              | +000
        jsr     0x28d70.l                              | +006
        tst.b   0x45(a6)                               | +00c
        bgt.w   .L44dda                                | +010
        move.w  #0x1022, d0                            | +014
        jsr     0x2352.l                               | +018
        lea     0x77f6a.l, a1                          | +01e
        jsr     0x6fe.l                                | +024
        jsr     0x5dd02.l                              | +02a
        lea     JmpAbsThunk_044df2(pc), a1             | +030
        move.l  a1, (a6)                               | +034
.L44dda:
        movea.l #0xffffffff, a0                        | +036
        jsr     0x5dd56.l                              | +03c
        bcc.w   SetHandlerRts_044df0                   | +042
        .size   BossShot_Fly_044DA4, .-BossShot_Fly_044DA4

        .globl  Miniboss_SpawnP1_044DF8
        .type   Miniboss_SpawnP1_044DF8, @function
        .section .text.Miniboss_SpawnP1_044DF8, "ax", @progbits
Miniboss_SpawnP1_044DF8:
        move.w  #0x2, d1                               | +000
        jsr     0x236e.l                               | +004
        move.l  #0x100440, 0x70(a6)                    | +00a
        move.w  #0x0, d0                               | +012
        bra.w   Miniboss_Attach_044E42                 | +016
        .size   Miniboss_SpawnP1_044DF8, .-Miniboss_SpawnP1_044DF8

        .globl  Miniboss_SpawnP2_044E12
        .type   Miniboss_SpawnP2_044E12, @function
        .section .text.Miniboss_SpawnP2_044E12, "ax", @progbits
Miniboss_SpawnP2_044E12:
        move.w  #0x4e, d1                              | +000
        jsr     0x236e.l                               | +004
        move.l  #0x1004e0, 0x70(a6)                    | +00a
        move.w  #0x1, d0                               | +012
        bra.w   Miniboss_Attach_044E42                 | +016
        .size   Miniboss_SpawnP2_044E12, .-Miniboss_SpawnP2_044E12

        .globl  Miniboss_SpawnAlt_044E2C
        .type   Miniboss_SpawnAlt_044E2C, @function
        .section .text.Miniboss_SpawnAlt_044E2C, "ax", @progbits
Miniboss_SpawnAlt_044E2C:
        move.w  #0x2, d1                               | +000
        jsr     0x236e.l                               | +004
        move.l  #0x100580, 0x70(a6)                    | +00a
        move.w  #0x0, d0                               | +012
        .size   Miniboss_SpawnAlt_044E2C, .-Miniboss_SpawnAlt_044E2C

        .globl  Miniboss_Attach_044E42
        .type   Miniboss_Attach_044E42, @function
        .section .text.Miniboss_Attach_044E42, "ax", @progbits
Miniboss_Attach_044E42:
        jsr     0x5e3a2.l                              | +000
        bcs.w   .L44e50                                | +006
        bra.w   Task_Kill_044F9A                       | +00a
.L44e50:
        btst    #0x0, 0x6b(a0)                         | +00e
        bne.w   .L44e5e                                | +014
        bra.w   Task_Kill_044F9A                       | +018
.L44e5e:
        btst    #0x3, 0x8c(a0)                         | +01c
        bne.w   .L44e6c                                | +022
        bra.w   Task_Kill_044F9A                       | +026
.L44e6c:
        tst.b   0x98(a6)                               | +02a
        bne.w   .L44e7a                                | +02e
        bset    #0x2, 0x5b(a6)                         | +032
.L44e7a:
        move.w  #0xc000, d0                            | +038
        jsr     0x28134.l                              | +03c
        andi.w  #0xffe3, 0x38(a6)                      | +042
        ori.w   #0x10, 0x38(a6)                        | +048
        lea     0x29d090.l, a0                         | +04e
        jsr     0x28cd4.l                              | +054
        lea     Miniboss_Ride_044EA2(pc), a1           | +05a
        move.l  a1, (a6)                               | +05e
        .size   Miniboss_Attach_044E42, .-Miniboss_Attach_044E42

        .globl  Miniboss_Ride_044EA2
        .type   Miniboss_Ride_044EA2, @function
        .section .text.Miniboss_Ride_044EA2, "ax", @progbits
Miniboss_Ride_044EA2:
        jsr     0x27c8c.l                              | +000
        bcc.w   .L44ed0                                | +006
        lea     Miniboss_Hop_044F04(pc), a1            | +00a
        move.l  a1, (a6)                               | +00e
        cmpi.b  #0x0, 0x106ece.l                       | +010
        bne.w   .L44ed0                                | +018
        andi.b  #0xc0, d7                              | +01c
        cmpi.b  #0x40, d7                              | +020
        bne.w   .L44ed0                                | +024
        lea     Miniboss_Dead_044EEE(pc), a1           | +028
        move.l  a1, (a6)                               | +02c
.L44ed0:
        jsr     0x28d70.l                              | +02e
        movea.l #0xffffffff, a0                        | +034
        jsr     0x5dd5c.l                              | +03a
        bcc.w   SetHandlerRts_044eec                   | +040
        .size   Miniboss_Ride_044EA2, .-Miniboss_Ride_044EA2

        .globl  Miniboss_Dead_044EEE
        .type   Miniboss_Dead_044EEE, @function
        .section .text.Miniboss_Dead_044EEE, "ax", @progbits
Miniboss_Dead_044EEE:
        lea     0x29d234.l, a0                         | +000
        jsr     0x28cd4.l                              | +006
        lea     Miniboss_Fall_044F4A(pc), a1           | +00c
        move.l  a1, (a6)                               | +010
        bra.w   Miniboss_Fall_044F4A                   | +012
        .size   Miniboss_Dead_044EEE, .-Miniboss_Dead_044EEE

        .globl  Miniboss_Hop_044F04
        .type   Miniboss_Hop_044F04, @function
        .section .text.Miniboss_Hop_044F04, "ax", @progbits
Miniboss_Hop_044F04:
        lea     0x29d178.l, a0                         | +000
        jsr     0x28cd4.l                              | +006
        move.w  #0x10, d0                              | +00c
        jsr     0x5dca4.l                              | +010
        move.w  d0, 0x28(a6)                           | +016
        move.w  #0x480, 0x2a(a6)                       | +01a
        move.w  #0xff70, 0x2e(a6)                      | +020
        move.w  #0x0, 0x2c(a6)                         | +026
        lea     .L44f36(pc), a1                        | +02c
        move.l  a1, (a6)                               | +030
.L44f36:
        jsr     0x27bc8.l                              | +032
        bcc.w   .L44f46                                | +038
        lea     Miniboss_Fall_044F4A(pc), a1           | +03c
        move.l  a1, (a6)                               | +040
.L44f46:
        bra.w   .L44f50                                | +042
        .globl  Miniboss_Fall_044F4A
Miniboss_Fall_044F4A:
        jsr     0x2783a.l                              | +046
.L44f50:
        jsr     0x28d70.l                              | +04c
        movea.l #0xffffffff, a0                        | +052
        jsr     0x5dd5c.l                              | +058
        bcc.w   .L44f6c                                | +05e
        lea     Task_Kill_044F9A(pc), a1               | +062
        move.l  a1, (a6)                               | +066
.L44f6c:
        movea.l 0x70(a6), a1                           | +068
        lea     Boss_HitboxTable_044D30(pc), a2        | +06c
        lea     Boss_HitboxB_044D38(pc), a0            | +070
        jsr     0x28c20.l                              | +074
        bcc.w   SetHandlerRts_044f88                   | +07a
        .size   Miniboss_Hop_044F04, .-Miniboss_Hop_044F04

