| =====================================================================
| turret_boss2_04580c.s — Wave YY (parte 1/2)
| Region: $04580C..$045F24 (26 entradas, 1.788 B, byte-exacto)
| =====================================================================
|
| TORRETA CON PUNTERIA DE 5 DIRECCIONES + BOSS2 / MINIBOSS2
|
| * Turret_FireGroundSnap_04580C: entidad torreta que dispara al ras del
|   suelo; copia el flip del padre (+$68), instala el handler de despliegue
|   de vehiculo via $6FE y sincroniza su Y con la del padre.
| * Turret_FollowParent_0458DC / Turret_ReadPlayerInput_04593A: seguimiento
|   de la entidad padre y lectura del input del jugador para decidir la
|   direccion de encaramiento.
| * Turret_InitDir0..4: cinco inicializadores, uno por direccion de
|   punteria; cada uno carga su tabla de angulos ($2895x) y salta a
|   Turret_SetupCommon_045A0A. Los cuerpos reales viven en
|   Turret_Dir0Body_0459B6..Turret_Dir4Body_045ACC (etiquetas internas).
| * Turret_InitTable_045CD6: tabla de 5 punteros (dato) indexada por la
|   direccion (+$70) — despacho estilo jump-table usada por Boss2_Death.
| * Turret_Track_045A38 / Turret_AimSmooth_045B02: seguimiento del objetivo
|   con suavizado del angulo (mismo nucleo de punteria que Wave XX).
| * Turret_Tail_045B9C: cola comun — tick del contexto padre con d1=8
|   ($8F714); si el padre murio instala Boss2_Explode; sondeo del jugador
|   ($5E452) y, si procede, spawnea Boss2Shot via el scheduler ($4AE)
|   sobre el pool $100800 (entrada global Turret_SpawnShot_045BE6).
| * Boss2_*: maquina de estados del segundo jefe — explosion con secuencia
|   de anims, caida, intercambio de flags, espera del bit 4 del padre,
|   tick del padre y rutina de muerte que despacha por Turret_InitTable.
| * Boss2Shot_Init/Fly: proyectil del jefe — velocidad aleatoria escalada
|   ($5DCA4) y vuelo con la hitbox Boss2_HitboxTable_045DD4 (+ variante B
|   en $45DDC).
| * Miniboss2_*: montura del mini-jefe — Attach engancha a la torreta,
|   Ride sigue al padre, Dead salta al matador comun (Miniboss2_HopKill_
|   045EEA, etiqueta global interna a Hop) y Hop hace el salto con impulso
|   vertical aleatorio ($5DCA4) antes de morir.
|
| Patron identico al bloque Boss/Miniboss de Wave XX ($0448A6): handlers
| auto-reemplazantes `lea next(pc),a1 ; move.l a1,(a6)` sobre el
| planificador de tareas ($4AE alta / $518 baja / $5B6 tick).
| =====================================================================

        .globl  Turret_FireGroundSnap_04580C
        .type   Turret_FireGroundSnap_04580C, @function
        .section .text.Turret_FireGroundSnap_04580C, "ax", @progbits
Turret_FireGroundSnap_04580C:
        movea.l 0xc(a6), a0                            | +000
        move.b  0x68(a0), 0x68(a6)                     | +004
        lea     Vehicle_JmpDeploy_045FDE(pc), a1       | +00a
        jsr     0x6fe.l                                | +00e
        jsr     0x5dd02.l                              | +014
        jsr     0x517fe.l                              | +01a
        move.b  0x34(a6), d0                           | +020
        addq.b  #0x4, d0                               | +024
        move.b  d0, 0x98(a0)                           | +026
        move.b  0x80(a6), 0x9a(a0)                     | +02a
        move.b  0x81(a6), 0x9b(a0)                     | +030
        addi.b  #0x1, 0x81(a6)                         | +036
        movea.l 0xc(a6), a0                            | +03c
        bset    #0x2, 0x8c(a0)                         | +040
        subi.b  #0x1, 0x80(a6)                         | +046
        move.b  0x34(a6), d0                           | +04c
        addi.w  #0x4, d0                               | +050
        andi.w  #0xf8, d0                              | +054
        lsr.w   #0x1, d0                               | +058
        lea     0x29d752.l, a0                         | +05a
        move.w  0x22(a6), d1                           | +060
        move.w  0x24(a6), d2                           | +064
        add.w   (a0, d0.w), d1                         | +068
        add.w   0x2(a0, d0.w), d2                      | +06c
        jsr     0x280c6.l                              | +070
        cmpi.b  #0x0, d0                               | +076
        beq.w   .L458d4                                | +07a
        cmpi.b  #0x10, d0                              | +07e
        beq.w   .L458ae                                | +082
        cmpi.b  #0x20, d0                              | +086
        beq.w   .L458ae                                | +08a
        cmpi.b  #0x21, d0                              | +08e
        beq.w   .L458ae                                | +092
        andi.b  #0xf0, d0                              | +096
        cmpi.b  #0x0, d0                               | +09a
        bne.w   .L458d4                                | +09e
.L458ae:
        lea     0x30c14.l, a1                          | +0a2
        jsr     0x4ae.l                                | +0a8
        jsr     0x5dd02.l                              | +0ae
        move.b  0x34(a6), d0                           | +0b4
        addi.w  #0x4, d0                               | +0b8
        andi.w  #0xf8, d0                              | +0bc
        lsr.w   #0x3, d0                               | +0c0
        move.b  d0, 0x98(a0)                           | +0c2
        rts                                            | +0c6
.L458d4:
        move.b  #0x5, 0x3b(a6)                         | +0c8
        rts                                            | +0ce
        .size   Turret_FireGroundSnap_04580C, .-Turret_FireGroundSnap_04580C

        .globl  Turret_FollowParent_0458DC
        .type   Turret_FollowParent_0458DC, @function
        .section .text.Turret_FollowParent_0458DC, "ax", @progbits
Turret_FollowParent_0458DC:
        movea.l 0xc(a6), a0                            | +000
        move.b  0x59(a0), 0x59(a6)                     | +004
        move.w  0x38(a0), d0                           | +00a
        move.w  0x22(a0), d1                           | +00e
        move.w  0x24(a0), d2                           | +012
        movea.l 0x70(a0), a3                           | +016
        add.w   0x4(a3), d1                            | +01a
        add.w   0x6(a3), d2                            | +01e
        movea.l (a3), a3                               | +022
        add.w   (a3), d1                               | +024
        add.w   0x2(a3), d2                            | +026
        ori.w   #0x1, d0                               | +02a
        move.w  d0, 0x38(a6)                           | +02e
        move.w  d1, 0x22(a6)                           | +032
        move.w  d2, 0x24(a6)                           | +036
        move.b  0x6d(a0), d0                           | +03a
        cmp.b   0x6d(a6), d0                           | +03e
        beq.w   .L45938                                | +042
        move.b  d0, 0x6d(a6)                           | +046
        move.b  0x68(a0), d0                           | +04a
        move.b  d0, 0x68(a6)                           | +04e
        jsr     0x13600.l                              | +052
        bsr.w   Ent_SfxByMode_0457A6                   | +058
.L45938:
        rts                                            | +05c
        .size   Turret_FollowParent_0458DC, .-Turret_FollowParent_0458DC

        .globl  Turret_ReadPlayerInput_04593A
        .type   Turret_ReadPlayerInput_04593A, @function
        .section .text.Turret_ReadPlayerInput_04593A, "ax", @progbits
Turret_ReadPlayerInput_04593A:
        move.b  0x6d(a6), d0                           | +000
        bpl.w   .L4594e                                | +004
        clr.b   0x71(a6)                               | +008
        clr.b   0x7c(a6)                               | +00c
        bra.w   .L459a6                                | +010
.L4594e:
        cmpi.b  #0x1, d0                               | +014
        bne.w   .L45960                                | +018
        lea     0x100440.l, a1                         | +01c
        bra.w   .L45966                                | +022
.L45960:
        lea     0x1004e0.l, a1                         | +026
.L45966:
        movea.l 0xc(a1), a1                            | +02c
        movea.l 0x72(a1), a2                           | +030
        jsr     0x5d674.l                              | +034
        move.b  0x2(a2), d0                            | +03a
        andi.b  #0xf, d0                               | +03e
        move.b  d0, 0x71(a6)                           | +042
        move.b  0x3(a2), d0                            | +046
        andi.b  #0x10, d0                              | +04a
        beq.w   .L459a6                                | +04e
        move.w  #0x108c, d0                            | +052
        jsr     0x2352.l                               | +056
        tst.b   0x7c(a6)                               | +05c
        sne     d0                                     | +060
        move.b  #0x3, 0x7c(a6)                         | +062
        and.b   d0, 0x7d(a6)                           | +068
.L459a6:
        rts                                            | +06c
        .size   Turret_ReadPlayerInput_04593A, .-Turret_ReadPlayerInput_04593A

        .globl  Turret_InitDir0_0459A8
        .type   Turret_InitDir0_0459A8, @function
        .section .text.Turret_InitDir0_0459A8, "ax", @progbits
Turret_InitDir0_0459A8:
        move.b  #0x0, 0x70(a6)                         | +000
        move.l  #0x289504, 0x74(a6)                    | +006
        .globl  Turret_Dir0Body_0459B6
Turret_Dir0Body_0459B6:
        move.w  #0x4000, 0x34(a6)                      | +00e
        bra.w   Turret_SetupCommon_045A0A              | +014
        .size   Turret_InitDir0_0459A8, .-Turret_InitDir0_0459A8

        .globl  Turret_InitDir1_0459C0
        .type   Turret_InitDir1_0459C0, @function
        .section .text.Turret_InitDir1_0459C0, "ax", @progbits
Turret_InitDir1_0459C0:
        move.b  #0x1, 0x70(a6)                         | +000
        move.l  #0x289544, 0x74(a6)                    | +006
        .globl  Turret_Dir1Body_0459CE
Turret_Dir1Body_0459CE:
        move.w  #0x8000, 0x34(a6)                      | +00e
        bra.w   Turret_SetupCommon_045A0A              | +014
        .size   Turret_InitDir1_0459C0, .-Turret_InitDir1_0459C0

        .globl  Turret_InitDir2_0459D8
        .type   Turret_InitDir2_0459D8, @function
        .section .text.Turret_InitDir2_0459D8, "ax", @progbits
Turret_InitDir2_0459D8:
        move.b  #0x2, 0x70(a6)                         | +000
        move.l  #0x289584, 0x74(a6)                    | +006
        .globl  Turret_Dir2Body_0459E6
Turret_Dir2Body_0459E6:
        move.w  #0xc000, 0x34(a6)                      | +00e
        bra.w   Turret_SetupCommon_045A0A              | +014
        .size   Turret_InitDir2_0459D8, .-Turret_InitDir2_0459D8

        .globl  Turret_InitDir3_0459F0
        .type   Turret_InitDir3_0459F0, @function
        .section .text.Turret_InitDir3_0459F0, "ax", @progbits
Turret_InitDir3_0459F0:
        move.b  #0x3, 0x70(a6)                         | +000
        move.l  #0x2895c4, 0x74(a6)                    | +006
        .globl  Turret_Dir3Body_0459FE
Turret_Dir3Body_0459FE:
        move.w  #0x0, 0x34(a6)                         | +00e
        bra.w   Turret_SetupCommon_045A0A              | +014
        nop                                            | +018
        .size   Turret_InitDir3_0459F0, .-Turret_InitDir3_0459F0

        .globl  Turret_SetupCommon_045A0A
        .type   Turret_SetupCommon_045A0A, @function
        .section .text.Turret_SetupCommon_045A0A, "ax", @progbits
Turret_SetupCommon_045A0A:
        jsr     0x13600.l                              | +000
        move.w  #0x2, d1                               | +006
        jsr     0x236e.l                               | +00a
        move.b  #0xff, 0x68(a6)                        | +010
        move.b  #0xff, 0x6d(a6)                        | +016
        clr.b   0x81(a6)                               | +01c
        clr.b   0x3b(a6)                               | +020
        clr.w   0x78(a6)                               | +024
        lea     Turret_Track_045A38(pc), a1            | +028
        move.l  a1, (a6)                               | +02c
        .size   Turret_SetupCommon_045A0A, .-Turret_SetupCommon_045A0A

        .globl  Turret_Track_045A38
        .type   Turret_Track_045A38, @function
        .section .text.Turret_Track_045A38, "ax", @progbits
Turret_Track_045A38:
        bsr.w   Turret_FollowParent_0458DC             | +000
        bsr.w   Turret_ReadPlayerInput_04593A          | +004
        movea.l 0x74(a6), a0                           | +008
        move.b  0x71(a6), d0                           | +00c
        andi.w  #0xf, d0                               | +010
        add.w   d0, d0                                 | +014
        add.w   d0, d0                                 | +016
        move.l  (a0, d0.w), d0                         | +018
        bmi.w   .L45a88                                | +01c
        sub.w   0x34(a6), d0                           | +020
        ext.l   d0                                     | +024
        bpl.w   .L45a64                                | +026
        neg.w   d0                                     | +02a
.L45a64:
        asr.w   #0x2, d0                               | +02c
        move.w  0x78(a6), d1                           | +02e
        addi.w  #0x100, d1                             | +032
        cmp.w   d0, d1                                 | +036
        bcs.w   .L45a76                                | +038
        move.w  d0, d1                                 | +03c
.L45a76:
        move.w  d1, 0x78(a6)                           | +03e
        swap    d0                                     | +042
        eor.w   d0, d1                                 | +044
        sub.w   d0, d1                                 | +046
        add.w   d1, 0x34(a6)                           | +048
        bra.w   .L45a8c                                | +04c
.L45a88:
        clr.w   0x78(a6)                               | +050
.L45a8c:
        move.b  0x7d(a6), d0                           | +054
        beq.w   .L45a9e                                | +058
        subq.b  #0x1, d0                               | +05c
        move.b  d0, 0x7d(a6)                           | +05e
        bra.w   .L45ab6                                | +062
.L45a9e:
        move.b  #0x1, 0x7d(a6)                         | +066
        move.b  0x7c(a6), d0                           | +06c
        beq.w   .L45ab6                                | +070
        subq.b  #0x1, d0                               | +074
        move.b  d0, 0x7c(a6)                           | +076
        bsr.w   Turret_FireGroundSnap_04580C           | +07a
.L45ab6:
        bsr.w   Ent_AnimFrame_0457CC                   | +07e
        bra.w   Turret_Tail_045B9C                     | +082
        .size   Turret_Track_045A38, .-Turret_Track_045A38

        .globl  Turret_InitDir4_045ABE
        .type   Turret_InitDir4_045ABE, @function
        .section .text.Turret_InitDir4_045ABE, "ax", @progbits
Turret_InitDir4_045ABE:
        move.b  #0x4, 0x70(a6)                         | +000
        move.l  #0x289604, 0x74(a6)                    | +006
        .globl  Turret_Dir4Body_045ACC
Turret_Dir4Body_045ACC:
        move.w  #0x0, 0x34(a6)                         | +00e
        move.w  #0x0, 0x7e(a6)                         | +014
        jsr     0x13600.l                              | +01a
        move.w  #0x2, d1                               | +020
        jsr     0x236e.l                               | +024
        move.b  #0xff, 0x68(a6)                        | +02a
        move.b  #0xff, 0x6d(a6)                        | +030
        clr.b   0x3b(a6)                               | +036
        clr.w   0x78(a6)                               | +03a
        lea     Turret_AimSmooth_045B02(pc), a1        | +03e
        move.l  a1, (a6)                               | +042
        .size   Turret_InitDir4_045ABE, .-Turret_InitDir4_045ABE

        .globl  Turret_AimSmooth_045B02
        .type   Turret_AimSmooth_045B02, @function
        .section .text.Turret_AimSmooth_045B02, "ax", @progbits
Turret_AimSmooth_045B02:
        bsr.w   Turret_FollowParent_0458DC             | +000
        bsr.w   Turret_ReadPlayerInput_04593A          | +004
        movea.l 0x74(a6), a0                           | +008
        move.b  0x71(a6), d0                           | +00c
        andi.w  #0xf, d0                               | +010
        add.w   d0, d0                                 | +014
        add.w   d0, d0                                 | +016
        move.l  (a0, d0.w), d0                         | +018
        bmi.w   .L45b2a                                | +01c
        move.w  d0, 0x7e(a6)                           | +020
        bra.w   .L45b2e                                | +024
.L45b2a:
        move.w  0x7e(a6), d0                           | +028
.L45b2e:
        move.w  d0, d1                                 | +02c
        sub.w   0x34(a6), d0                           | +02e
        ext.l   d0                                     | +032
        bpl.w   .L45b3c                                | +034
        neg.w   d0                                     | +038
.L45b3c:
        move.w  #0x200, d2                             | +03a
        cmpi.w  #0x5000, d0                            | +03e
        bcs.w   .L45b4c                                | +042
        move.w  #0x800, d2                             | +046
.L45b4c:
        asr.w   #0x1, d0                               | +04a
        move.w  0x78(a6), d1                           | +04c
        add.w   d2, d1                                 | +050
        cmp.w   d0, d1                                 | +052
        bcs.w   .L45b5c                                | +054
        move.w  d0, d1                                 | +058
.L45b5c:
        move.w  d1, 0x78(a6)                           | +05a
        swap    d0                                     | +05e
        eor.w   d0, d1                                 | +060
        sub.w   d0, d1                                 | +062
        add.w   d1, 0x34(a6)                           | +064
        move.b  0x7d(a6), d0                           | +068
        beq.w   .L45b7c                                | +06c
        subq.b  #0x1, d0                               | +070
        move.b  d0, 0x7d(a6)                           | +072
        bra.w   .L45b94                                | +076
.L45b7c:
        move.b  #0x1, 0x7d(a6)                         | +07a
        move.b  0x7c(a6), d0                           | +080
        beq.w   .L45b94                                | +084
        subq.b  #0x1, d0                               | +088
        move.b  d0, 0x7c(a6)                           | +08a
        bsr.w   Turret_FireGroundSnap_04580C           | +08e
.L45b94:
        bsr.w   Ent_AnimFrame_0457CC                   | +092
        bra.w   Turret_Tail_045B9C                     | +096
        .size   Turret_AimSmooth_045B02, .-Turret_AimSmooth_045B02

        .globl  Turret_Tail_045B9C
        .type   Turret_Tail_045B9C, @function
        .section .text.Turret_Tail_045B9C, "ax", @progbits
Turret_Tail_045B9C:
        movea.l 0xc(a6), a0                            | +000
        move.b  0x68(a0), 0x68(a6)                     | +004
        bclr    #0x3, 0x8c(a6)                         | +00a
        movem.l a6, -(a7)                              | +010
        movea.l 0xc(a6), a6                            | +014
        move.b  #0x8, d1                               | +018
        jsr     0x8f714.l                              | +01c
        movem.l (a7)+, a6                              | +022
        bcc.w   .L45bcc                                | +026
        lea     Boss2_Explode_045C20(pc), a1           | +02a
        move.l  a1, (a6)                               | +02e
.L45bcc:
        jsr     0x5e452.l                              | +030
        bcc.w   .L45be6                                | +036
        movea.l 0xc(a6), a0                            | +03a
        btst    #0x0, 0x13(a0)                         | +03e
        bne.w   .L45be6                                | +044
        rts                                            | +048
.L45be6:
        .globl  Turret_SpawnShot_045BE6
Turret_SpawnShot_045BE6:
        move.l  a6, -(a7)                              | +04a
        lea     0x100800.l, a6                         | +04c
        lea     Boss2Shot_Init_045D3A(pc), a1          | +052
        jsr     0x4ae.l                                | +056
        movea.l (a7)+, a6                              | +05c
        move.w  0x22(a6), 0x22(a0)                     | +05e
        move.w  0x24(a6), 0x24(a0)                     | +064
        move.b  0x68(a6), 0x68(a0)                     | +06a
        move.b  0x6d(a6), 0x6d(a0)                     | +070
        .size   Turret_Tail_045B9C, .-Turret_Tail_045B9C

        .globl  Boss2_Explode_045C20
        .type   Boss2_Explode_045C20, @function
        .section .text.Boss2_Explode_045C20, "ax", @progbits
Boss2_Explode_045C20:
        move.w  #0x106a, d0                            | +000
        jsr     0x2352.l                               | +004
        move.l  a6, -(a7)                              | +00a
        lea     0x100800.l, a6                         | +00c
        lea     Boss2Shot_Init_045D3A(pc), a1          | +012
        jsr     0x4ae.l                                | +016
        movea.l (a7)+, a6                              | +01c
        move.w  0x22(a6), 0x22(a0)                     | +01e
        move.w  0x24(a6), 0x24(a0)                     | +024
        move.b  0x68(a6), 0x68(a0)                     | +02a
        move.b  0x6d(a6), 0x6d(a0)                     | +030
        lea     0x77f6a.l, a1                          | +036
        jsr     0x6fe.l                                | +03c
        jsr     0x5dd02.l                              | +042
        clr.w   0x7a(a6)                               | +048
        lea     Boss2_Fall_045C72(pc), a1              | +04c
        move.l  a1, (a6)                               | +050
        .size   Boss2_Explode_045C20, .-Boss2_Explode_045C20

        .globl  Boss2_Fall_045C72
        .type   Boss2_Fall_045C72, @function
        .section .text.Boss2_Fall_045C72, "ax", @progbits
Boss2_Fall_045C72:
        move.w  0x106f6c.l, d0                         | +000
        bpl.w   .L45c7e                                | +006
        neg.w   d0                                     | +00a
.L45c7e:
        add.w   0x7a(a6), d0                           | +00c
        move.w  d0, 0x7a(a6)                           | +010
        cmpi.w  #0x1e0, d0                             | +014
        ble.w   .L45c94                                | +018
        lea     Boss2_FlagSwap_045C98(pc), a1          | +01c
        move.l  a1, (a6)                               | +020
.L45c94:
        bra.w   Boss2_TickParent_045CC2                | +022
        .size   Boss2_Fall_045C72, .-Boss2_Fall_045C72

        .globl  Boss2_FlagSwap_045C98
        .type   Boss2_FlagSwap_045C98, @function
        .section .text.Boss2_FlagSwap_045C98, "ax", @progbits
Boss2_FlagSwap_045C98:
        movea.l 0xc(a6), a0                            | +000
        bset    #0x3, 0x8c(a0)                         | +004
        bclr    #0x4, 0x8c(a0)                         | +00a
        lea     Boss2_WaitBit4_045CAE(pc), a1          | +010
        move.l  a1, (a6)                               | +014
        .size   Boss2_FlagSwap_045C98, .-Boss2_FlagSwap_045C98

        .globl  Boss2_WaitBit4_045CAE
        .type   Boss2_WaitBit4_045CAE, @function
        .section .text.Boss2_WaitBit4_045CAE, "ax", @progbits
Boss2_WaitBit4_045CAE:
        movea.l 0xc(a6), a0                            | +000
        btst    #0x4, 0x8c(a0)                         | +004
        beq.w   Boss2_TickParent_045CC2                | +00a
        lea     Boss2_Death_045CEA(pc), a1             | +00e
        move.l  a1, (a6)                               | +012
        .size   Boss2_WaitBit4_045CAE, .-Boss2_WaitBit4_045CAE

        .globl  Boss2_TickParent_045CC2
        .type   Boss2_TickParent_045CC2, @function
        .section .text.Boss2_TickParent_045CC2, "ax", @progbits
Boss2_TickParent_045CC2:
        move.l  a6, -(a7)                              | +000
        movea.l 0xc(a6), a6                            | +002
        move.b  #0x1, d1                               | +006
        jsr     0x8f6f2.l                              | +00a
        movea.l (a7)+, a6                              | +010
        rts                                            | +012
        .size   Boss2_TickParent_045CC2, .-Boss2_TickParent_045CC2

        .globl  Turret_InitTable_045CD6
        .section .text.Turret_InitTable_045CD6, "ax", @progbits
Turret_InitTable_045CD6:
        .long   Turret_Dir0Body_0459B6                 | dir 0
        .long   Turret_Dir1Body_0459CE                 | dir 1
        .long   Turret_Dir2Body_0459E6                 | dir 2
        .long   Turret_Dir3Body_0459FE                 | dir 3
        .long   Turret_Dir4Body_045ACC                 | dir 4
        .size   Turret_InitTable_045CD6, .-Turret_InitTable_045CD6

        .globl  Boss2_Death_045CEA
        .type   Boss2_Death_045CEA, @function
        .section .text.Boss2_Death_045CEA, "ax", @progbits
Boss2_Death_045CEA:
        jsr     0x13600.l                              | +000
        bsr.w   Ent_SfxByMode_0457A6                   | +006
        move.w  #0x106a, d0                            | +00a
        jsr     0x2352.l                               | +00e
        lea     0x29d3a8.l, a0                         | +014
        jsr     0x28cd4.l                              | +01a
        lea     .L45d10(pc), a1                        | +020
        move.l  a1, (a6)                               | +024
.L45d10:
        bsr.w   Turret_FollowParent_0458DC             | +026
        jsr     0x28d70.l                              | +02a
        bcc.w   .L45d2e                                | +030
        move.b  0x70(a6), d0                           | +034
        andi.w  #0xff, d0                              | +038
        add.w   d0, d0                                 | +03c
        add.w   d0, d0                                 | +03e
        lea     Turret_InitTable_045CD6(pc), a0        | +040
.L45d2e:
        jsr     0x5e452.l                              | +044
        bcs.w   Turret_SpawnShot_045BE6                            | +04a
        bra.b   Boss2_TickParent_045CC2                | +04e
        .size   Boss2_Death_045CEA, .-Boss2_Death_045CEA

        .globl  Boss2Shot_Init_045D3A
        .type   Boss2Shot_Init_045D3A, @function
        .section .text.Boss2Shot_Init_045D3A, "ax", @progbits
Boss2Shot_Init_045D3A:
        bsr.w   Ent_SfxByMode_0457A6                   | +000
        move.w  #0xd000, d0                            | +004
        jsr     0x28134.l                              | +008
        andi.w  #0xffe3, 0x38(a6)                      | +00e
        ori.w   #0x10, 0x38(a6)                        | +014
        move.w  #0x38, d0                              | +01a
        jsr     0x5dca4.l                              | +01e
        move.w  d0, 0x28(a6)                           | +024
        move.w  #0x384, 0x2a(a6)                       | +028
        move.w  #0xffec, 0x2e(a6)                      | +02e
        move.w  #0x0, 0x2c(a6)                         | +034
        move.b  #0x3c, 0x45(a6)                        | +03a
        lea     0x29d090.l, a0                         | +040
        jsr     0x28cd4.l                              | +046
        lea     Boss2Shot_Fly_045D8C(pc), a1           | +04c
        move.l  a1, (a6)                               | +050
        .size   Boss2Shot_Init_045D3A, .-Boss2Shot_Init_045D3A

        .globl  Boss2Shot_Fly_045D8C
        .type   Boss2Shot_Fly_045D8C, @function
        .section .text.Boss2Shot_Fly_045D8C, "ax", @progbits
Boss2Shot_Fly_045D8C:
        jsr     0x27d50.l                              | +000
        jsr     0x28d70.l                              | +006
        tst.b   0x45(a6)                               | +00c
        bne.w   .L45db6                                | +010
        move.w  #0x1022, d0                            | +014
        jsr     0x2352.l                               | +018
        jsr     0x13600.l                              | +01e
        jmp     0x77f6a.l                              | +024
.L45db6:
        movea.l #0xffffffff, a0                        | +02a
        jsr     0x5dd56.l                              | +030
        bcc.w   Jsr5B6Rts_045dd2                       | +036
        .size   Boss2Shot_Fly_045D8C, .-Boss2Shot_Fly_045D8C

        .globl  Boss2_HitboxTable_045DD4
        .section .text.Boss2_HitboxTable_045DD4, "ax", @progbits
Boss2_HitboxTable_045DD4:
        .word   0xFFE8, 0x0018, 0xFFFC, 0x0008   | rect A: -24,+24,-4,+8
        .globl  Boss2_HitboxB_045DDC
Boss2_HitboxB_045DDC:
        .word   0x0000, 0x0001, 0x0000, 0x0001   | rect B: 0,+1,0,+1
        .size   Boss2_HitboxTable_045DD4, .-Boss2_HitboxTable_045DD4

        .globl  Miniboss2_Attach_045DE4
        .type   Miniboss2_Attach_045DE4, @function
        .section .text.Miniboss2_Attach_045DE4, "ax", @progbits
Miniboss2_Attach_045DE4:
        lea     0x100580.l, a0                         | +000
        cmpi.l  #0x2ae3e, (a0)                         | +006
        bne.w   .L45dfa                                | +00c
        jmp     0x518.l                                | +010
.L45dfa:
        btst    #0x3, 0x8c(a0)                         | +016
        bne.w   .L45e0a                                | +01c
        jmp     0x518.l                                | +020
.L45e0a:
        move.w  #0x2, d1                               | +026
        jsr     0x236e.l                               | +02a
        bset    #0x2, 0x5b(a6)                         | +030
        move.w  #0xc000, d0                            | +036
        jsr     0x28134.l                              | +03a
        andi.w  #0xffe3, 0x38(a6)                      | +040
        ori.w   #0x10, 0x38(a6)                        | +046
        lea     0x29d090.l, a0                         | +04c
        jsr     0x28cd4.l                              | +052
        lea     Miniboss2_Ride_045E42(pc), a1          | +058
        move.l  a1, (a6)                               | +05c
        .size   Miniboss2_Attach_045DE4, .-Miniboss2_Attach_045DE4

        .globl  Miniboss2_Ride_045E42
        .type   Miniboss2_Ride_045E42, @function
        .section .text.Miniboss2_Ride_045E42, "ax", @progbits
Miniboss2_Ride_045E42:
        jsr     0x27c8c.l                              | +000
        bcc.w   .L45e70                                | +006
        lea     Miniboss2_Hop_045EA4(pc), a1           | +00a
        move.l  a1, (a6)                               | +00e
        cmpi.b  #0x0, 0x106ece.l                       | +010
        bne.w   .L45e70                                | +018
        andi.b  #0xc0, d7                              | +01c
        cmpi.b  #0x40, d7                              | +020
        bne.w   .L45e70                                | +024
        lea     Miniboss2_Dead_045E8E(pc), a1          | +028
        move.l  a1, (a6)                               | +02c
.L45e70:
        jsr     0x28d70.l                              | +02e
        movea.l #0xffffffff, a0                        | +034
        jsr     0x5dd5c.l                              | +03a
        bcc.w   .L45e8c                                | +040
        jmp     0x518.l                                | +044
.L45e8c:
        rts                                            | +04a
        .size   Miniboss2_Ride_045E42, .-Miniboss2_Ride_045E42

        .globl  Miniboss2_Dead_045E8E
        .type   Miniboss2_Dead_045E8E, @function
        .section .text.Miniboss2_Dead_045E8E, "ax", @progbits
Miniboss2_Dead_045E8E:
        lea     0x29d234.l, a0                         | +000
        jsr     0x28cd4.l                              | +006
        lea     Miniboss2_HopKill_045EEA(pc), a1                    | +00c
        move.l  a1, (a6)                               | +010
        bra.w   Miniboss2_HopKill_045EEA                            | +012
        .size   Miniboss2_Dead_045E8E, .-Miniboss2_Dead_045E8E

        .globl  Miniboss2_Hop_045EA4
        .type   Miniboss2_Hop_045EA4, @function
        .section .text.Miniboss2_Hop_045EA4, "ax", @progbits
Miniboss2_Hop_045EA4:
        lea     0x29d178.l, a0                         | +000
        jsr     0x28cd4.l                              | +006
        move.w  #0x10, d0                              | +00c
        jsr     0x5dca4.l                              | +010
        move.w  d0, 0x28(a6)                           | +016
        move.w  #0x480, 0x2a(a6)                       | +01a
        move.w  #0xff70, 0x2e(a6)                      | +020
        move.w  #0x0, 0x2c(a6)                         | +026
        lea     .L45ed6(pc), a1                        | +02c
        move.l  a1, (a6)                               | +030
.L45ed6:
        jsr     0x27bc8.l                              | +032
        bcc.w   .L45ee6                                | +038
        lea     .L45eea(pc), a1                        | +03c
        move.l  a1, (a6)                               | +040
.L45ee6:
        bra.w   .L45ef0                                | +042
.L45eea:
        .globl  Miniboss2_HopKill_045EEA
Miniboss2_HopKill_045EEA:
        jsr     0x2783a.l                              | +046
.L45ef0:
        jsr     0x28d70.l                              | +04c
        movea.l #0xffffffff, a0                        | +052
        jsr     0x5dd5c.l                              | +058
        bcc.w   .L45f0c                                | +05e
        jmp     0x518.l                                | +062
.L45f0c:
        lea     0x100580.l, a1                         | +068
        lea     Boss2_HitboxTable_045DD4(pc), a2       | +06e
        lea     Boss2_HitboxB_045DDC(pc), a0           | +072
        jsr     0x28c20.l                              | +076
        bcc.w   SetHandlerRts_045f2a                   | +07c
        .size   Miniboss2_Hop_045EA4, .-Miniboss2_Hop_045EA4

