| ============================================================================
|  Metal Slug 1 - asm/mission_event_vm_04422a.s
|  ----------------------------------------------------------------------------
|  Wave XX (1/3) - LA VM DE EVENTOS DE MISION. Cluster $04422A..$044896,
|  33 entradas, 1,536 B: rellena los 14 primeros huecos entre las islas C
|  ya matcheadas de la zona.
|
|  ARQUITECTURA. Cada escena tiene un stream de bytecode de "eventos de
|  mision" (tabla de 14 punteros en MissionStreamPtrs_044266, indexada por
|  $106ECE). MissionDriver_Init ($0442E6) resuelve el stream (con override
|  a $F260E si $10FD83 esta armado), arranca dos tareas paralelas
|  ($8C864 y $79298) y cae en MissionDriver_Loop ($044368), que cada frame:
|    - si no esta pausado ($74(a6)=0): compara el umbral del registro
|      actual ($2(a1)) contra el progreso del scroll $106F5C; si se cruzo,
|      ejecuta el op via MissionVM_ExecOp ($0443C0, jump-table de 13 ops).
|    - si esta pausado: decrementa el timer $76(a6) y re-ejecuta al expirar.
|
|  OPCODES (tabla en $0443E4):
|    $00 spawn directo (Spawn_FromStream)     $01 spawner periodico
|    $02 skip hasta op $0D                    $03 select condicional (rand)
|    $04 espera players pasen umbral Y        $05 pausa + flag $108179
|    $06 pausa                                $07 espera enemigos < umbral
|    $08 reanuda + limpia $108179             $09 reanuda
|    $0A espera players pasen umbral X        $0B espera ambos ejes Y
|    $0C fin de stream (trap #15 guard)       -> LeaA1Plus4_*/task-kill islas
|
|  MissionVM_SkipOp ($0446B6) es el ITERADOR PARALELO: avanza el PC del
|  stream saltando registros SIN ejecutarlos (cada op tiene su stride en
|  la 2a jump-table de $0446DA); lo usan los ops de espera $04/$0A/$0B
|  para descartar los spawns que el jugador dejo atras, y op $03 para
|  elegir 1 de N ramas via $5E9E4 (rand) juntando punteros en la pila.
|
|  HALLAZGO: los guards `nop;nop;cmpi;nop;trap #15` tras cada fetch de op
|  son asserts de desarrollo (op > $0C = stream corrupto) que SNK dejo
|  compilados en la ROM final.
|
|  PeriodicSpawner ($0447C6/$04482A/$044850/$044860): tarea hija que
|  re-dispara Spawn_FromStream cada $80(a6) frames (cadencia en el stream,
|  x32 si flag), con limite de repeticiones $7C y gate por progreso $82
|  vs $106F5C y por conteo de enemigos vivos $84 vs $106E88.
|
|  Todo byte-exacto contra la ROM (verificado por match_batch).
| ============================================================================

        .globl  MissionBoot_SceneDesc_04422A
        .section .text.MissionBoot_SceneDesc_04422A, "ax", @progbits
MissionBoot_SceneDesc_04422A:
        .word   0x0200, 0x0002                         | $4422A par de words
        .globl  MissionBoot_SceneList_04422E
MissionBoot_SceneList_04422E:
        .long   MissionBoot_SceneDesc_04422A + 2       | -> word $0002
        .long   MissionBoot_SceneDesc_04422A           | -> word $0200
        .size   MissionBoot_SceneDesc_04422A, .-MissionBoot_SceneDesc_04422A

        .globl  MissionBoot_Run_044236
        .type   MissionBoot_Run_044236, @function
        .section .text.MissionBoot_Run_044236, "ax", @progbits
MissionBoot_Run_044236:
        lea     MissionBoot_SceneList_04422E(pc), a0   | +000
        jsr     SceneLoader_ByIndex_043562(pc)         | +004
        moveq   #0x0, d0                               | +008
        moveq   #0x0, d1                               | +00a
        jmp     SceneScriptVM_Frame_0437DA(pc)         | +00c
        .size   MissionBoot_Run_044236, .-MissionBoot_Run_044236

        .globl  Entity_CmpDepthToParent_044246
        .type   Entity_CmpDepthToParent_044246, @function
        .section .text.Entity_CmpDepthToParent_044246, "ax", @progbits
Entity_CmpDepthToParent_044246:
        movea.l 0x8(a6), a1                            | +000
        move.b  0x10(a6), d0                           | +004
        cmp.b   0x10(a1), d0                           | +008
        bcs.w   SetXN_04425c                           | +00c
        .size   Entity_CmpDepthToParent_044246, .-Entity_CmpDepthToParent_044246

        .globl  MissionStream_Table_044262
        .section .text.MissionStream_Table_044262, "ax", @progbits
MissionStream_Table_044262:
        .byte   0x0c, 0x00, 0xff, 0xff                 | stream nulo: op $0C = fin
        .globl  MissionStreamPtrs_044266
MissionStreamPtrs_044266:
        .long   0x000E8524
        .long   0x000E970A
        .long   0x000EA3FE
        .long   0x000EB6EA
        .long   0x000EC898
        .long   0x000EE200
        .long   0x000F0ABE
        .long   0x000F0FB0
        .long   0x000F1032
        .long   0x000F11D4
        .long   MissionStream_Table_044262             | mision vacia (op $0C)
        .long   0x000F11D8
        .long   0x000F1CD4
        .long   MissionStream_Table_044262             | mision vacia (op $0C)
        .size   MissionStream_Table_044262, .-MissionStream_Table_044262

        .globl  MissionWatch_Spawn_04429E
        .type   MissionWatch_Spawn_04429E, @function
        .section .text.MissionWatch_Spawn_04429E, "ax", @progbits
MissionWatch_Spawn_04429E:
        move.l  a1, -(a7)                              | +000
        move.w  d0, -(a7)                              | +002
        lea     MissionWatch_Handler_0442C4(pc), a1    | +004
        jsr     0x4ae.l                                | +008
        move.w  (a7)+, 0x78(a0)                        | +00e
        move.l  (a7)+, 0x70(a0)                        | +012
        move.b  #0x0, 0x74(a0)                         | +016
        clr.w   0x76(a0)                               | +01c
        clr.w   0x7a(a0)                               | +020
        rts                                            | +024
        .size   MissionWatch_Spawn_04429E, .-MissionWatch_Spawn_04429E

        .globl  MissionWatch_Handler_0442C4
        .type   MissionWatch_Handler_0442C4, @function
        .section .text.MissionWatch_Handler_0442C4, "ax", @progbits
MissionWatch_Handler_0442C4:
        movea.l 0xc(a6), a0                            | +000
        move.w  0x78(a6), d0                           | +004
        tst.b   (a0, d0.w)                             | +008
        beq.w   MissionWatch_Resume_0442E2             | +00c
        .size   MissionWatch_Handler_0442C4, .-MissionWatch_Handler_0442C4

        .globl  MissionWatch_Resume_0442E2
        .type   MissionWatch_Resume_0442E2, @function
        .section .text.MissionWatch_Resume_0442E2, "ax", @progbits
MissionWatch_Resume_0442E2:
        bra.w   MissionDriver_Loop_044368              | +000
        .size   MissionWatch_Resume_0442E2, .-MissionWatch_Resume_0442E2

        .globl  MissionDriver_Init_0442E6
        .type   MissionDriver_Init_0442E6, @function
        .section .text.MissionDriver_Init_0442E6, "ax", @progbits
MissionDriver_Init_0442E6:
        moveq   #0x0, d0                               | +000
        move.b  0x106ece.l, d0                         | +002
        bpl.w   .L44302                                | +008
        jsr     0x5b6.l                                | +00c
        lea     0x400.l, a1                            | +012
        move.l  a1, (a6)                               | +018
        rts                                            | +01a
.L44302:
        add.w   d0, d0                                 | +01c
        add.w   d0, d0                                 | +01e
        lea     MissionStreamPtrs_044266(pc), a0       | +020
        move.l  (a0, d0.w), d0                         | +024
        cmpi.l  #0xf1cd4, d0                           | +028
        bne.w   .L44328                                | +02e
        tst.b   0x10fd83.l                             | +032
        beq.w   .L44328                                | +038
        move.l  #0xf260e, d0                           | +03c
.L44328:
        move.l  d0, 0x70(a6)                           | +042
        move.l  a6, -(a7)                              | +046
        lea     0x1008a0.l, a6                         | +048
        jsr     0x8c85c.l                              | +04e
        lea     0x8c864.l, a1                          | +054
        jsr     0x4ae.l                                | +05a
        lea     0x79298.l, a1                          | +060
        jsr     0x4ae.l                                | +066
        movea.l (a7)+, a6                              | +06c
        move.b  #0x0, 0x74(a6)                         | +06e
        clr.w   0x76(a6)                               | +074
        clr.w   0x7a(a6)                               | +078
        lea     MissionDriver_Loop_044368(pc), a0      | +07c
        move.l  a0, (a6)                               | +080
        .size   MissionDriver_Init_0442E6, .-MissionDriver_Init_0442E6

        .globl  MissionDriver_Loop_044368
        .type   MissionDriver_Loop_044368, @function
        .section .text.MissionDriver_Loop_044368, "ax", @progbits
MissionDriver_Loop_044368:
        movea.l 0x70(a6), a1                           | +000
.L4436c:
        cmpi.b  #0x0, 0x74(a6)                         | +004
        bne.w   .L44396                                | +00a
        move.w  0x2(a1), d0                            | +00e
        cmpi.w  #0xffff, d0                            | +012
        beq.w   .L44390                                | +016
        move.w  d0, 0x7a(a6)                           | +01a
        cmp.w   0x106f5c.l, d0                         | +01e
        bls.w   .L443b6                                | +024
.L44390:
        rts                                            | +028
        bra.w   .L443b6                                | +02a
.L44396:
        move.w  0x76(a6), d0                           | +02e
        beq.w   .L443a8                                | +032
        subq.w  #0x1, d0                               | +036
        move.w  d0, 0x76(a6)                           | +038
        bra.w   .L443ac                                | +03c
.L443a8:
        move.w  0x2(a1), d0                            | +040
.L443ac:
        move.w  d0, 0x76(a6)                           | +044
        beq.w   .L443b6                                | +048
        rts                                            | +04c
.L443b6:
        bsr.w   MissionVM_ExecOp_0443C0                | +04e
        move.l  a1, 0x70(a6)                           | +052
        bra.b   .L4436c                                | +056
        .size   MissionDriver_Loop_044368, .-MissionDriver_Loop_044368

        .globl  MissionVM_ExecOp_0443C0
        .type   MissionVM_ExecOp_0443C0, @function
        .section .text.MissionVM_ExecOp_0443C0, "ax", @progbits
MissionVM_ExecOp_0443C0:
        moveq   #0x0, d0                               | +000
        move.b  (a1), d0                               | +002
        cmpi.b  #0xc, d0                               | +004
        bls.w   .L443d8                                | +008
        nop                                            | +00c
        nop                                            | +00e
        cmpi.b  #0xc, d0                               | +010
        nop                                            | +014
        trap    #15                                    | +016
.L443d8:
        add.w   d0, d0                                 | +018
        add.w   d0, d0                                 | +01a
        lea     .L443e4(pc), a0                        | +01c
        jmp     (a0, d0.w)                             | +020
.L443e4:
        bra.w   MissionOp00_SpawnDirect_0446A8         | +024
        bra.w   MissionOp01_PeriodicSpawn_04469E       | +028
        bra.w   MissionOp02_SkipTo0D_04460A            | +02c
        bra.w   MissionOp03_CondSelect_044638          | +030
        bra.w   MissionOp04_PlayersPastY_04446C        | +034
        bra.w   MissionOp05_PauseOn_044426             | +038
        bra.w   MissionOp06_Pause_04443A               | +03c
        bra.w   MissionOp07_WaitEnemies_044446         | +040
        bra.w   MissionOp08_Resume_04445A              | +044
        bra.w   LeaA1Plus4_044420                      | +048
        bra.w   MissionOp0A_PlayersPastX_0444E8        | +04c
        bra.w   MissionOp0B_BothPastY_04455A           | +050
        bra.w   .L44418                                | +054
.L44418:
        jmp     0x518.l                                | +058
        rts                                            | +05e
        .size   MissionVM_ExecOp_0443C0, .-MissionVM_ExecOp_0443C0

        .globl  MissionOp05_PauseOn_044426
        .type   MissionOp05_PauseOn_044426, @function
        .section .text.MissionOp05_PauseOn_044426, "ax", @progbits
MissionOp05_PauseOn_044426:
        move.b  #0x1, 0x74(a6)                         | +000
        move.b  #0x1, 0x108179.l                       | +006
        .size   MissionOp05_PauseOn_044426, .-MissionOp05_PauseOn_044426

        .globl  MissionOp06_Pause_04443A
        .type   MissionOp06_Pause_04443A, @function
        .section .text.MissionOp06_Pause_04443A, "ax", @progbits
MissionOp06_Pause_04443A:
        move.b  #0x1, 0x74(a6)                         | +000
        .size   MissionOp06_Pause_04443A, .-MissionOp06_Pause_04443A

        .globl  MissionOp07_WaitEnemies_044446
        .type   MissionOp07_WaitEnemies_044446, @function
        .section .text.MissionOp07_WaitEnemies_044446, "ax", @progbits
MissionOp07_WaitEnemies_044446:
        move.w  0x106e88.l, d0                         | +000
        cmp.b   0x1(a1), d0                            | +006
        bhi.w   LeaA1Plus4Rts_044458                   | +00a
        .size   MissionOp07_WaitEnemies_044446, .-MissionOp07_WaitEnemies_044446

        .globl  MissionOp08_Resume_04445A
        .type   MissionOp08_Resume_04445A, @function
        .section .text.MissionOp08_Resume_04445A, "ax", @progbits
MissionOp08_Resume_04445A:
        move.b  #0x0, 0x74(a6)                         | +000
        clr.b   0x108179.l                             | +006
        .size   MissionOp08_Resume_04445A, .-MissionOp08_Resume_04445A

        .globl  MissionOp04_PlayersPastY_04446C
        .type   MissionOp04_PlayersPastY_04446C, @function
        .section .text.MissionOp04_PlayersPastY_04446C, "ax", @progbits
MissionOp04_PlayersPastY_04446C:
        clr.w   d0                                     | +000
        clr.w   d1                                     | +002
        clr.w   d2                                     | +004
        clr.w   d3                                     | +006
        cmpi.b  #0x1, 0x10fdb6.l                       | +008
        bne.w   .L4448e                                | +010
        lea     0x100440.l, a0                         | +014
        move.w  0x22(a0), d0                           | +01a
        move.w  0x24(a0), d1                           | +01e
.L4448e:
        cmpi.b  #0x1, 0x10fdb7.l                       | +022
        bne.w   .L444a8                                | +02a
        lea     0x1004e0.l, a0                         | +02e
        move.w  0x22(a0), d2                           | +034
        move.w  0x24(a0), d3                           | +038
.L444a8:
        cmp.w   d2, d0                                 | +03c
        bge.w   .L444b0                                | +03e
        exg       d1, d3                                 | +042
.L444b0:
        subi.w  #0x200, d1                             | +044
        neg.w   d1                                     | +048
        cmpi.b  #0x0, 0x1(a1)                          | +04a
        bne.w   .L444c6                                | +050
        add.w   0x106f54.l, d1                         | +054
.L444c6:
        cmp.w   0x4(a1), d1                            | +05a
        lea     0x6(a1), a1                            | +05e
        bpl.w   .L444de                                | +062
        bsr.w   MissionVM_ExecOp_0443C0                | +066
        bsr.w   MissionVM_SkipOp_0446B6                | +06a
        bra.w   .L444e6                                | +06e
.L444de:
        bsr.w   MissionVM_SkipOp_0446B6                | +072
        bsr.w   MissionVM_ExecOp_0443C0                | +076
.L444e6:
        rts                                            | +07a
        .size   MissionOp04_PlayersPastY_04446C, .-MissionOp04_PlayersPastY_04446C

        .globl  MissionOp0A_PlayersPastX_0444E8
        .type   MissionOp0A_PlayersPastX_0444E8, @function
        .section .text.MissionOp0A_PlayersPastX_0444E8, "ax", @progbits
MissionOp0A_PlayersPastX_0444E8:
        move.w  #0xffff, d0                            | +000
        move.w  #0xffff, d1                            | +004
        cmpi.b  #0x1, 0x10fdb6.l                       | +008
        bne.w   .L44506                                | +010
        lea     0x100440.l, a0                         | +014
        move.w  0x22(a0), d0                           | +01a
.L44506:
        cmpi.b  #0x1, 0x10fdb7.l                       | +01e
        bne.w   .L4451c                                | +026
        lea     0x1004e0.l, a0                         | +02a
        move.w  0x22(a0), d1                           | +030
.L4451c:
        cmp.w   d1, d0                                 | +034
        bls.w   .L44524                                | +036
        exg       d1, d0                                 | +03a
.L44524:
        cmpi.b  #0x0, 0x1(a1)                          | +03c
        bne.w   .L44534                                | +042
        add.w   0x106f50.l, d0                         | +046
.L44534:
        cmp.w   0x4(a1), d0                            | +04c
        lea     0x6(a1), a1                            | +050
        bmi.w   .L4454e                                | +054
        bsr.w   MissionVM_SkipOp_0446B6                | +058
        bsr.w   MissionVM_ExecOp_0443C0                | +05c
        rts                                            | +060
        bra.w   Stub_00044558                          | +062
.L4454e:
        bsr.w   MissionVM_ExecOp_0443C0                | +066
        bsr.w   MissionVM_SkipOp_0446B6                | +06a
        rts                                            | +06e
        .size   MissionOp0A_PlayersPastX_0444E8, .-MissionOp0A_PlayersPastX_0444E8

        .globl  MissionOp0B_BothPastY_04455A
        .type   MissionOp0B_BothPastY_04455A, @function
        .section .text.MissionOp0B_BothPastY_04455A, "ax", @progbits
MissionOp0B_BothPastY_04455A:
        clr.w   d0                                     | +000
        clr.w   d1                                     | +002
        cmpi.b  #0x1, 0x10fdb6.l                       | +004
        bne.w   .L44574                                | +00c
        lea     0x100440.l, a0                         | +010
        move.w  0x24(a0), d0                           | +016
.L44574:
        cmpi.b  #0x1, 0x10fdb7.l                       | +01a
        bne.w   .L4458a                                | +022
        lea     0x1004e0.l, a0                         | +026
        move.w  0x24(a0), d1                           | +02c
.L4458a:
        tst.w   d0                                     | +030
        bne.w   .L44592                                | +032
        move.w  d1, d0                                 | +036
.L44592:
        tst.w   d1                                     | +038
        bne.w   .L4459a                                | +03a
        move.w  d0, d1                                 | +03e
.L4459a:
        subi.w  #0x200, d0                             | +040
        neg.w   d0                                     | +044
        subi.w  #0x200, d1                             | +046
        neg.w   d1                                     | +04a
        cmpi.b  #0x0, 0x1(a1)                          | +04c
        bne.w   .L445bc                                | +052
        add.w   0x106f54.l, d0                         | +056
        add.w   0x106f54.l, d1                         | +05c
.L445bc:
        cmp.w   0x4(a1), d0                            | +062
        smi     d0                                     | +066
        cmp.w   0x4(a1), d1                            | +068
        smi     d1                                     | +06c
        lea     0x6(a1), a1                            | +06e
        move.b  d0, d7                                 | +072
        and.b   d1, d7                                 | +074
        beq.w   .L445e4                                | +076
        bsr.w   MissionVM_ExecOp_0443C0                | +07a
        bsr.w   MissionVM_SkipOp_0446B6                | +07e
        bsr.w   MissionVM_SkipOp_0446B6                | +082
        bra.w   .L44608                                | +086
.L445e4:
        move.b  d0, d7                                 | +08a
        or.b    d0, d7                                 | +08c
        beq.w   .L445fc                                | +08e
        bsr.w   MissionVM_SkipOp_0446B6                | +092
        bsr.w   MissionVM_ExecOp_0443C0                | +096
        bsr.w   MissionVM_SkipOp_0446B6                | +09a
        bra.w   .L44608                                | +09e
.L445fc:
        bsr.w   MissionVM_SkipOp_0446B6                | +0a2
        bsr.w   MissionVM_SkipOp_0446B6                | +0a6
        bsr.w   MissionVM_ExecOp_0443C0                | +0aa
.L44608:
        rts                                            | +0ae
        .size   MissionOp0B_BothPastY_04455A, .-MissionOp0B_BothPastY_04455A

        .globl  MissionOp02_SkipTo0D_04460A
        .type   MissionOp02_SkipTo0D_04460A, @function
        .section .text.MissionOp02_SkipTo0D_04460A, "ax", @progbits
MissionOp02_SkipTo0D_04460A:
        lea     0x4(a1), a1                            | +000
.L4460e:
        move.b  (a1), d0                               | +004
        cmpi.b  #0xc, d0                               | +006
        bne.w   .L44624                                | +00a
        nop                                            | +00e
        nop                                            | +010
        cmpi.b  #0xc, d0                               | +012
        nop                                            | +016
        trap    #15                                    | +018
.L44624:
        cmpi.b  #0xd, d0                               | +01a
        beq.w   .L44632                                | +01e
        bsr.w   MissionVM_ExecOp_0443C0                | +022
        bra.b   .L4460e                                | +026
.L44632:
        lea     0x2(a1), a1                            | +028
        rts                                            | +02c
        .size   MissionOp02_SkipTo0D_04460A, .-MissionOp02_SkipTo0D_04460A

        .globl  MissionOp03_CondSelect_044638
        .type   MissionOp03_CondSelect_044638, @function
        .section .text.MissionOp03_CondSelect_044638, "ax", @progbits
MissionOp03_CondSelect_044638:
        moveq   #0x0, d7                               | +000
        lea     0x4(a1), a1                            | +002
.L4463e:
        move.b  (a1), d0                               | +006
        cmpi.b  #0xc, d0                               | +008
        bne.w   .L44654                                | +00c
        nop                                            | +010
        nop                                            | +012
        cmpi.b  #0xc, d0                               | +014
        nop                                            | +018
        trap    #15                                    | +01a
.L44654:
        cmpi.b  #0xe, d0                               | +01c
        beq.w   .L44666                                | +020
        move.l  a1, -(a7)                              | +024
        addq.w  #0x1, d7                               | +026
        bsr.w   MissionVM_SkipOp_0446B6                | +028
        bra.b   .L4463e                                | +02c
.L44666:
        lea     0x2(a1), a2                            | +02e
        move.w  d7, d0                                 | +032
        beq.w   .L4469a                                | +034
        move.l  a4, -(a7)                              | +038
        movem.w d6-d7, -(a7)                           | +03a
        jsr     0x5e9e4.l                              | +03e
        movem.w (a7)+, d6-d7                           | +044
        movea.l (a7)+, a4                              | +048
        add.w   d0, d0                                 | +04a
        add.w   d0, d0                                 | +04c
        add.w   d7, d7                                 | +04e
        add.w   d7, d7                                 | +050
        movea.l (a7, d0.w), a1                         | +052
        lea     (a7, d7.w), a7                         | +056
        move.l  a2, -(a7)                              | +05a
        bsr.w   MissionVM_ExecOp_0443C0                | +05c
        movea.l (a7)+, a2                              | +060
.L4469a:
        movea.l a2, a1                                 | +062
        rts                                            | +064
        .size   MissionOp03_CondSelect_044638, .-MissionOp03_CondSelect_044638

        .globl  MissionOp01_PeriodicSpawn_04469E
        .type   MissionOp01_PeriodicSpawn_04469E, @function
        .section .text.MissionOp01_PeriodicSpawn_04469E, "ax", @progbits
MissionOp01_PeriodicSpawn_04469E:
        bsr.w   PeriodicSpawner_Setup_0447C6           | +000
        bsr.w   MissionVM_SkipOp_0446B6                | +004
        rts                                            | +008
        .size   MissionOp01_PeriodicSpawn_04469E, .-MissionOp01_PeriodicSpawn_04469E

        .globl  MissionOp00_SpawnDirect_0446A8
        .type   MissionOp00_SpawnDirect_0446A8, @function
        .section .text.MissionOp00_SpawnDirect_0446A8, "ax", @progbits
MissionOp00_SpawnDirect_0446A8:
        move.l  a1, -(a7)                              | +000
        bsr.w   Spawn_FromStream_0448A6                | +002
        movea.l (a7)+, a1                              | +006
        lea     0x12(a1), a1                           | +008
        rts                                            | +00c
        .size   MissionOp00_SpawnDirect_0446A8, .-MissionOp00_SpawnDirect_0446A8

        .globl  MissionVM_SkipOp_0446B6
        .type   MissionVM_SkipOp_0446B6, @function
        .section .text.MissionVM_SkipOp_0446B6, "ax", @progbits
MissionVM_SkipOp_0446B6:
        moveq   #0x0, d0                               | +000
        move.b  (a1), d0                               | +002
        cmpi.b  #0xc, d0                               | +004
        bls.w   .L446ce                                | +008
        nop                                            | +00c
        nop                                            | +00e
        cmpi.b  #0xc, d0                               | +010
        nop                                            | +014
        trap    #15                                    | +016
.L446ce:
        add.w   d0, d0                                 | +018
        add.w   d0, d0                                 | +01a
        lea     .L446da(pc), a0                        | +01c
        jmp     (a0, d0.w)                             | +020
.L446da:
        bra.w   MissionSkip_Op00_04472C                | +024
        bra.w   MissionSkip_Op01_044732                | +028
        bra.w   MissionSkip_Op02_04473C                | +02c
        bra.w   MissionSkip_Op03_04476A                | +030
        bra.w   MissionSkip_Op04_044798                | +034
        bra.w   LeaA1Plus4_044720                      | +038
        bra.w   LeaA1Plus4_044726                      | +03c
        bra.w   LeaA1Plus4_04471a                      | +040
        bra.w   LeaA1Plus4_044714                      | +044
        bra.w   LeaA1Plus4_04470e                      | +048
        bra.w   MissionSkip_Op0A_0447A6                | +04c
        bra.w   MissionSkip_Op0B_0447B4                | +050
        bra.w   MissionSkip_Rts_0447C4                 | +054
        .size   MissionVM_SkipOp_0446B6, .-MissionVM_SkipOp_0446B6

        .globl  MissionSkip_Op00_04472C
        .type   MissionSkip_Op00_04472C, @function
        .section .text.MissionSkip_Op00_04472C, "ax", @progbits
MissionSkip_Op00_04472C:
        lea     0x12(a1), a1                           | +000
        rts                                            | +004
        .size   MissionSkip_Op00_04472C, .-MissionSkip_Op00_04472C

        .globl  MissionSkip_Op01_044732
        .type   MissionSkip_Op01_044732, @function
        .section .text.MissionSkip_Op01_044732, "ax", @progbits
MissionSkip_Op01_044732:
        lea     0xa(a1), a1                            | +000
        bsr.w   MissionVM_SkipOp_0446B6                | +004
        rts                                            | +008
        .size   MissionSkip_Op01_044732, .-MissionSkip_Op01_044732

        .globl  MissionSkip_Op02_04473C
        .type   MissionSkip_Op02_04473C, @function
        .section .text.MissionSkip_Op02_04473C, "ax", @progbits
MissionSkip_Op02_04473C:
        lea     0x4(a1), a1                            | +000
.L44740:
        move.b  (a1), d0                               | +004
        cmpi.b  #0xc, d0                               | +006
        bne.w   .L44756                                | +00a
        nop                                            | +00e
        nop                                            | +010
        cmpi.b  #0xc, d0                               | +012
        nop                                            | +016
        trap    #15                                    | +018
.L44756:
        cmpi.b  #0xd, d0                               | +01a
        beq.w   .L44764                                | +01e
        bsr.w   MissionVM_SkipOp_0446B6                | +022
        bra.b   .L44740                                | +026
.L44764:
        lea     0x2(a1), a1                            | +028
        rts                                            | +02c
        .size   MissionSkip_Op02_04473C, .-MissionSkip_Op02_04473C

        .globl  MissionSkip_Op03_04476A
        .type   MissionSkip_Op03_04476A, @function
        .section .text.MissionSkip_Op03_04476A, "ax", @progbits
MissionSkip_Op03_04476A:
        lea     0x4(a1), a1                            | +000
.L4476e:
        move.b  (a1), d0                               | +004
        cmpi.b  #0xc, d0                               | +006
        bne.w   .L44784                                | +00a
        nop                                            | +00e
        nop                                            | +010
        cmpi.b  #0xc, d0                               | +012
        nop                                            | +016
        trap    #15                                    | +018
.L44784:
        cmpi.b  #0xe, d0                               | +01a
        beq.w   .L44792                                | +01e
        bsr.w   MissionVM_SkipOp_0446B6                | +022
        bra.b   .L4476e                                | +026
.L44792:
        lea     0x2(a1), a1                            | +028
        rts                                            | +02c
        .size   MissionSkip_Op03_04476A, .-MissionSkip_Op03_04476A

        .globl  MissionSkip_Op04_044798
        .type   MissionSkip_Op04_044798, @function
        .section .text.MissionSkip_Op04_044798, "ax", @progbits
MissionSkip_Op04_044798:
        lea     0x6(a1), a1                            | +000
        bsr.w   MissionVM_SkipOp_0446B6                | +004
        bsr.w   MissionVM_SkipOp_0446B6                | +008
        rts                                            | +00c
        .size   MissionSkip_Op04_044798, .-MissionSkip_Op04_044798

        .globl  MissionSkip_Op0A_0447A6
        .type   MissionSkip_Op0A_0447A6, @function
        .section .text.MissionSkip_Op0A_0447A6, "ax", @progbits
MissionSkip_Op0A_0447A6:
        lea     0x6(a1), a1                            | +000
        bsr.w   MissionVM_SkipOp_0446B6                | +004
        bsr.w   MissionVM_SkipOp_0446B6                | +008
        rts                                            | +00c
        .size   MissionSkip_Op0A_0447A6, .-MissionSkip_Op0A_0447A6

        .globl  MissionSkip_Op0B_0447B4
        .type   MissionSkip_Op0B_0447B4, @function
        .section .text.MissionSkip_Op0B_0447B4, "ax", @progbits
MissionSkip_Op0B_0447B4:
        lea     0x6(a1), a1                            | +000
        bsr.w   MissionVM_SkipOp_0446B6                | +004
        bsr.w   MissionVM_SkipOp_0446B6                | +008
        bsr.w   MissionVM_SkipOp_0446B6                | +00c
        .globl  MissionSkip_Rts_0447C4
MissionSkip_Rts_0447C4:
        rts                                            | +010
        .size   MissionSkip_Op0B_0447B4, .-MissionSkip_Op0B_0447B4

        .globl  PeriodicSpawner_Setup_0447C6
        .type   PeriodicSpawner_Setup_0447C6, @function
        .section .text.PeriodicSpawner_Setup_0447C6, "ax", @progbits
PeriodicSpawner_Setup_0447C6:
        movem.l a1, -(a7)                              | +000
        lea     PeriodicSpawner_Task_04482A(pc), a1    | +004
        jsr     0x4ae.l                                | +008
        movem.l (a7)+, a1                              | +00e
        move.b  0x5(a1), d0                            | +012
        andi.w  #0xff, d0                              | +016
        tst.b   0x8(a1)                                | +01a
        beq.w   .L447ea                                | +01e
        lsl.w   #0x5, d0                               | +022
.L447ea:
        move.w  d0, 0x80(a0)                           | +024
        move.b  0x1(a1), 0x7c(a0)                      | +028
        moveq   #0x0, d0                               | +02e
        move.b  0x6(a1), d0                            | +030
        tst.b   0x9(a1)                                | +034
        beq.w   .L44804                                | +038
        lsl.w   #0x5, d0                               | +03c
.L44804:
        move.w  d0, 0x7e(a0)                           | +03e
        move.b  0x7(a1), 0x84(a0)                      | +042
        move.b  0x4(a1), d0                            | +048
        andi.w  #0xff, d0                              | +04c
        lsl.w   #0x4, d0                               | +050
        add.w   0x7a(a6), d0                           | +052
        move.w  d0, 0x82(a0)                           | +056
        lea     0xa(a1), a1                            | +05a
        move.l  a1, 0x70(a0)                           | +05e
        rts                                            | +062
        .size   PeriodicSpawner_Setup_0447C6, .-PeriodicSpawner_Setup_0447C6

        .globl  PeriodicSpawner_Task_04482A
        .type   PeriodicSpawner_Task_04482A, @function
        .section .text.PeriodicSpawner_Task_04482A, "ax", @progbits
PeriodicSpawner_Task_04482A:
        tst.b   0x106ed3.l                             | +000
        beq.w   Jsr5B6ThenJmpScheduler_044842          | +006
        move.w  0x106f5c.l, d0                         | +00a
        cmp.w   0x82(a6), d0                           | +010
        bcs.w   PeriodicSpawner_Tick_044850            | +014
        .size   PeriodicSpawner_Task_04482A, .-PeriodicSpawner_Task_04482A

        .globl  PeriodicSpawner_Tick_044850
        .type   PeriodicSpawner_Tick_044850, @function
        .section .text.PeriodicSpawner_Tick_044850, "ax", @progbits
PeriodicSpawner_Tick_044850:
        move.w  0x80(a6), d0                           | +000
        beq.w   PeriodicSpawner_Fire_044860            | +004
        subq.w  #0x1, d0                               | +008
        .size   PeriodicSpawner_Tick_044850, .-PeriodicSpawner_Tick_044850

        .globl  PeriodicSpawner_Fire_044860
        .type   PeriodicSpawner_Fire_044860, @function
        .section .text.PeriodicSpawner_Fire_044860, "ax", @progbits
PeriodicSpawner_Fire_044860:
        clr.w   d0                                     | +000
        move.b  0x84(a6), d0                           | +002
        cmpi.b  #0xff, d0                              | +006
        beq.b   .L44876                                | +00a
        cmp.w   0x106e88.l, d0                         | +00c
        bcs.w   Stub_000448A4                          | +012
.L44876:
        movea.l 0x70(a6), a1                           | +016
        bsr.w   MissionVM_ExecOp_0443C0                | +01a
        move.w  0x7e(a6), 0x80(a6)                     | +01e
        move.b  0x7c(a6), d0                           | +024
        beq.w   Stub_000448A4                          | +028
        subq.b  #0x1, d0                               | +02c
        move.b  d0, 0x7c(a6)                           | +02e
        bne.w   Stub_000448A4                          | +032
        .size   PeriodicSpawner_Fire_044860, .-PeriodicSpawner_Fire_044860

