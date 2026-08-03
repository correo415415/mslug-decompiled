| ============================================================================
|  Metal Slug 1 - asm/ent_aim_input_044f8a.s
|  ----------------------------------------------------------------------------
|  Wave XX (3/3) - EL NUCLEO DE LA PUNTERIA DEL JUGADOR. Cluster
|  $044F8A..$045806, 13 entradas, 2,136 B. Completa el megabloque
|  contiguo $040EF2..$045806 (~18.7 KB sin huecos).
|
|  Ent_AimUpdate_045022 (1,008 B, la funcion mas grande de la wave):
|  resuelve el ANGULO DE PUNTERIA por frame segun el arma activa
|  $106F2A (0=pistola, 1=HMG, 2/3=flame, 4=rocket, 5=shotgun):
|    - arma 1 (HMG):    tabla $5D3A6 o $5D4C6 segun bit5 de $7B(a6)
|    - arma 5 (shotgun): 8 tablas $5D3C6..$5D526 segun Y<=$180, el
|                        estado del player ($5CEF8 carry) y bits4/5
|    - arma 4 (rocket): tabla fija $5D346
|    - armas 3/2 (flame): matriz de transicion de PUNTERIA DIAGONAL:
|        compara pares de bits de $83(a6) (dir vieja|nueva) $30/$90/
|        $60/$C0 y conmuta la tabla activa $84(a6) entre las 8 tablas
|        (suaviza el giro 8-direccional del lanzallamas); si la dir
|        del pad $71 coincide con la actual $7B, salta por bit4..7
|    - resto: tabla base $5D326
|  Con la tabla elegida: entrada = tabla[(estado & $F)*2] (word de
|  angulo objetivo, $FFFF = sin cambio, $8000 = flip). El angulo actual
|  $7C(a6) se integra con EASING: paso = delta>>3 - fric>>5 acumulado
|  en $7E(a6) (velocidad angular con friccion), clampeado al delta
|  restante, con modo snap ($73(a6): paso fijo $800) y direccion por
|  smi/not del signo ($77(a6) = ultimo sentido de giro).
|
|  Ent_AimInit_045412: resetea el estado de punteria; arma 2 (flame)
|  arranca en dir 6/tabla $5D406, resto en dir 4/tabla $5D386.
|
|  Ent_InputSample_04546E (474 B): muestrea el pad hacia $70/$71/$72
|  (prev/cur/edge). Fuente: el contexto del player via $68(a1) (indice
|  0=$100440, 1=$1004E0, $FF=neutral $5CC08 - demo/attract) y la tabla
|  de layouts $5D674. Aplica MASCARAS POR ARMA a las direcciones
|  aceptadas: $30 (HMG/shotgun: solo diag arriba), $F0 (flame/rocket:
|  8-dir), $10 (pistola). Gestiona ademas el latch anti-repeat $74(a6)
|  (6 frames) para congelar la punteria en cambios bruscos.
|
|  Ent_FireGate_045648 (184 B): cadencia de disparo. Edge de gatillo
|  ($72 & mascara por arma) arma $76(a6)=$FF; el cooldown $75(a6) se
|  recarga a 24 frames (HMG auto: 6 el resto, 2 si hay $76 pendiente)
|  y decrementa cada frame; Ent_AmmoTick_045706 resta municion $79(a6).
|
|  Ent_GroundProbe_04572C: sonda el mapa de colision via $280C6 con el
|  offset de la pose actual (tabla $29D752[$35(a6)*4]): tiles $00 aire,
|  $10/$20/$21 y familia $Fx = suelo (C invertido via islas CCR).
|  Ent_ClampY_045716 / Ent_SfxByMode_0457A6 / Ent_AnimFrame_0457CC:
|  clamp inferior Y=368, sfx $1D6/$2 segun modo $6D, y fetch del frame
|  de anim desde la matriz $289204 [pose $34 redondeada x3 + $3B].
|
|  Todo byte-exacto contra la ROM (verificado por match_batch).
| ============================================================================

        .globl  Task_KillFlagParent_044F8A
        .type   Task_KillFlagParent_044F8A, @function
        .section .text.Task_KillFlagParent_044F8A, "ax", @progbits
Task_KillFlagParent_044F8A:
        movea.l 0x70(a6), a0                           | +000
        bset    #0x4, 0x8c(a0)                         | +004
        jmp     0x518.l                                | +00a
        .size   Task_KillFlagParent_044F8A, .-Task_KillFlagParent_044F8A

        .globl  Task_Kill_044F9A
        .type   Task_Kill_044F9A, @function
        .section .text.Task_Kill_044F9A, "ax", @progbits
Task_Kill_044F9A:
        jmp     0x518.l                                | +000
        .size   Task_Kill_044F9A, .-Task_Kill_044F9A

        .globl  Ent_FaceTarget_044FA0
        .type   Ent_FaceTarget_044FA0, @function
        .section .text.Ent_FaceTarget_044FA0, "ax", @progbits
Ent_FaceTarget_044FA0:
        jsr     0x5e4ca.l                              | +000
        move.b  0x59(a0), 0x59(a6)                     | +006
        btst    #0x6, 0x6b(a0)                         | +00c
        beq.w   .L44ffc                                | +012
        cmpi.b  #0x0, 0x106f2a.l                       | +016
        bne.w   .L44fd0                                | +01e
        move.b  #0x1, d1                               | +022
        not.b   d1                                     | +026
        and.b   d1, 0x3a(a6)                           | +028
        bra.w   .L44fdc                                | +02c
.L44fd0:
        move.b  0x3a(a0), d1                           | +030
        eori.b  #0x1, d1                               | +034
        move.b  d1, 0x3a(a6)                           | +038
.L44fdc:
        move.w  0x22(a0), d1                           | +03c
        move.w  0x24(a0), d2                           | +040
        addi.w  #0x10, d2                              | +044
        ori.w   #0x1, d0                               | +048
        move.w  d0, 0x38(a6)                           | +04c
        move.w  d1, 0x22(a6)                           | +050
        move.w  d2, 0x24(a6)                           | +054
        bra.w   .L45020                                | +058
.L44ffc:
        movea.l 0x70(a0), a3                           | +05c
        add.w   0x4(a3), d1                            | +060
        add.w   0x6(a3), d2                            | +064
        movea.l (a3), a3                               | +068
        add.w   (a3), d1                               | +06a
        add.w   0x2(a3), d2                            | +06c
        ori.w   #0x1, d0                               | +070
        move.w  d0, 0x38(a6)                           | +074
        move.w  d1, 0x22(a6)                           | +078
        move.w  d2, 0x24(a6)                           | +07c
.L45020:
        rts                                            | +080
        .size   Ent_FaceTarget_044FA0, .-Ent_FaceTarget_044FA0

        .globl  Ent_AimUpdate_045022
        .type   Ent_AimUpdate_045022, @function
        .section .text.Ent_AimUpdate_045022, "ax", @progbits
Ent_AimUpdate_045022:
        cmpi.b  #0x3, 0x106f2a.l                       | +000
        beq.w   .L45036                                | +008
        tst.b   0x74(a6)                               | +00c
        beq.w   .L45408                                | +010
.L45036:
        cmpi.b  #0x2, 0x106f2a.l                       | +014
        beq.w   .L4504a                                | +01c
        tst.b   0x74(a6)                               | +020
        beq.w   .L45408                                | +024
.L4504a:
        move.b  0x71(a6), d0                           | +028
        tst.b   0x106f2a.l                             | +02c
        beq.w   .L4505a                                | +032
        clr.b   d0                                     | +036
.L4505a:
        tst.b   0x106f2a.l                             | +038
        beq.w   .L45370                                | +03e
        move.b  0x106f2a.l, d1                         | +042
        cmpi.b  #0x1, d1                               | +048
        bne.w   .L45090                                | +04c
        btst    #0x5, 0x7b(a6)                         | +050
        beq.w   .L45086                                | +056
        lea     0x5d3a6.l, a0                          | +05a
        bra.w   .L4508c                                | +060
.L45086:
        lea     0x5d4c6.l, a0                          | +064
.L4508c:
        bra.w   .L4537a                                | +06a
.L45090:
        move.b  0x106f2a.l, d1                         | +06e
        cmpi.b  #0x5, d1                               | +074
        bne.w   .L45184                                | +078
        cmpi.w  #0x180, 0x24(a6)                       | +07c
        bgt.w   .L45116                                | +082
        movem.l a6, -(a7)                              | +086
        movea.l 0xc(a6), a6                            | +08a
        jsr     0x5cef8.l                              | +08e
        movem.l (a7)+, a6                              | +094
        bcs.w   .L450ea                                | +098
        btst    #0x4, 0x7b(a6)                         | +09c
        beq.w   .L450d2                                | +0a2
        lea     0x5d3c6.l, a0                          | +0a6
        bra.w   .L45180                                | +0ac
.L450d2:
        btst    #0x5, 0x7b(a6)                         | +0b0
        beq.w   .L450e6                                | +0b6
        lea     0x5d426.l, a0                          | +0ba
        bra.w   .L45180                                | +0c0
.L450e6:
        bra.w   .L45112                                | +0c4
.L450ea:
        btst    #0x4, 0x7b(a6)                         | +0c8
        beq.w   .L450fe                                | +0ce
        lea     0x5d4c6.l, a0                          | +0d2
        bra.w   .L45180                                | +0d8
.L450fe:
        btst    #0x5, 0x7b(a6)                         | +0dc
        beq.w   .L45112                                | +0e2
        lea     0x5d4e6.l, a0                          | +0e6
        bra.w   .L45180                                | +0ec
.L45112:
        bra.w   .L45180                                | +0f0
.L45116:
        movem.l a6, -(a7)                              | +0f4
        movea.l 0xc(a6), a6                            | +0f8
        jsr     0x5cef8.l                              | +0fc
        movem.l (a7)+, a6                              | +102
        bcs.w   .L45158                                | +106
        btst    #0x4, 0x7b(a6)                         | +10a
        beq.w   .L45140                                | +110
        lea     0x5d446.l, a0                          | +114
        bra.w   .L45180                                | +11a
.L45140:
        btst    #0x5, 0x7b(a6)                         | +11e
        beq.w   .L45154                                | +124
        lea     0x5d486.l, a0                          | +128
        bra.w   .L45180                                | +12e
.L45154:
        bra.w   .L45180                                | +132
.L45158:
        btst    #0x4, 0x7b(a6)                         | +136
        beq.w   .L4516c                                | +13c
        lea     0x5d506.l, a0                          | +140
        bra.w   .L45180                                | +146
.L4516c:
        btst    #0x5, 0x7b(a6)                         | +14a
        beq.w   .L45180                                | +150
        lea     0x5d526.l, a0                          | +154
        bra.w   .L45180                                | +15a
.L45180:
        bra.w   .L4537a                                | +15e
.L45184:
        cmpi.b  #0x4, d1                               | +162
        bne.w   .L45196                                | +166
        lea     0x5d346.l, a0                          | +16a
        bra.w   .L4537a                                | +170
.L45196:
        cmpi.b  #0x3, d1                               | +174
        bne.w   .L45292                                | +178
        movea.l 0x84(a6), a0                           | +17c
        move.b  0x83(a6), d1                           | +180
        andi.b  #0x30, d1                              | +184
        cmpi.b  #0x30, d1                              | +188
        bne.w   .L451c0                                | +18c
        lea     0x5d3c6.l, a0                          | +190
        move.l  a0, 0x84(a6)                           | +196
        bra.w   .L4528e                                | +19a
.L451c0:
        move.b  0x83(a6), d1                           | +19e
        andi.b  #0x90, d1                              | +1a2
        cmpi.b  #0x90, d1                              | +1a6
        bne.w   .L451de                                | +1aa
        lea     0x5d446.l, a0                          | +1ae
        move.l  a0, 0x84(a6)                           | +1b4
        bra.w   .L4528e                                | +1b8
.L451de:
        move.b  0x83(a6), d1                           | +1bc
        andi.b  #0x60, d1                              | +1c0
        cmpi.b  #0x60, d1                              | +1c4
        bne.w   .L451fc                                | +1c8
        lea     0x5d406.l, a0                          | +1cc
        move.l  a0, 0x84(a6)                           | +1d2
        bra.w   .L4528e                                | +1d6
.L451fc:
        move.b  0x83(a6), d1                           | +1da
        andi.b  #0xc0, d1                              | +1de
        cmpi.b  #0xc0, d1                              | +1e2
        bne.w   .L4521a                                | +1e6
        lea     0x5d486.l, a0                          | +1ea
        move.l  a0, 0x84(a6)                           | +1f0
        bra.w   .L4528e                                | +1f4
.L4521a:
        move.b  0x71(a6), d1                           | +1f8
        andi.b  #0xf0, d1                              | +1fc
        cmp.b   0x7b(a6), d1                           | +200
        beq.w   .L4522e                                | +204
        bra.w   .L4528e                                | +208
.L4522e:
        btst    #0x4, 0x7b(a6)                         | +20c
        beq.w   .L45246                                | +212
        lea     0x5d386.l, a0                          | +216
        move.l  a0, 0x84(a6)                           | +21c
        bra.w   .L4528e                                | +220
.L45246:
        btst    #0x5, 0x7b(a6)                         | +224
        beq.w   .L4525e                                | +22a
        lea     0x5d3e6.l, a0                          | +22e
        move.l  a0, 0x84(a6)                           | +234
        bra.w   .L4528e                                | +238
.L4525e:
        btst    #0x6, 0x7b(a6)                         | +23c
        beq.w   .L45276                                | +242
        lea     0x5d426.l, a0                          | +246
        move.l  a0, 0x84(a6)                           | +24c
        bra.w   .L4528e                                | +250
.L45276:
        btst    #0x7, 0x7b(a6)                         | +254
        beq.w   .L4528e                                | +25a
        lea     0x5d466.l, a0                          | +25e
        move.l  a0, 0x84(a6)                           | +264
        bra.w   .L4528e                                | +268
.L4528e:
        bra.w   .L4537a                                | +26c
.L45292:
        cmpi.b  #0x2, d1                               | +270
        bne.w   .L45370                                | +274
        movea.l 0x84(a6), a0                           | +278
        move.b  0x83(a6), d1                           | +27c
        andi.b  #0x30, d1                              | +280
        cmpi.b  #0x30, d1                              | +284
        bne.w   .L452bc                                | +288
        lea     0x5d546.l, a0                          | +28c
        move.l  a0, 0x84(a6)                           | +292
        bra.w   .L4536c                                | +296
.L452bc:
        move.b  0x83(a6), d1                           | +29a
        andi.b  #0x60, d1                              | +29e
        cmpi.b  #0x60, d1                              | +2a2
        bne.w   .L452da                                | +2a6
        lea     0x5d406.l, a0                          | +2aa
        move.l  a0, 0x84(a6)                           | +2b0
        bra.w   .L4536c                                | +2b4
.L452da:
        move.b  0x83(a6), d1                           | +2b8
        andi.b  #0xc0, d1                              | +2bc
        cmpi.b  #0xc0, d1                              | +2c0
        bne.w   .L452f8                                | +2c4
        lea     0x5d526.l, a0                          | +2c8
        move.l  a0, 0x84(a6)                           | +2ce
        bra.w   .L4536c                                | +2d2
.L452f8:
        move.b  0x71(a6), d1                           | +2d6
        andi.b  #0xf0, d1                              | +2da
        cmp.b   0x7b(a6), d1                           | +2de
        beq.w   .L4530c                                | +2e2
        bra.w   .L4536c                                | +2e6
.L4530c:
        btst    #0x4, 0x7b(a6)                         | +2ea
        beq.w   .L45324                                | +2f0
        lea     0x5d386.l, a0                          | +2f4
        move.l  a0, 0x84(a6)                           | +2fa
        bra.w   .L4536c                                | +2fe
.L45324:
        btst    #0x5, 0x7b(a6)                         | +302
        beq.w   .L4533c                                | +308
        lea     0x5d3c6.l, a0                          | +30c
        move.l  a0, 0x84(a6)                           | +312
        bra.w   .L4536c                                | +316
.L4533c:
        btst    #0x6, 0x7b(a6)                         | +31a
        beq.w   .L45354                                | +320
        lea     0x5d3e6.l, a0                          | +324
        move.l  a0, 0x84(a6)                           | +32a
        bra.w   .L4536c                                | +32e
.L45354:
        btst    #0x7, 0x7b(a6)                         | +332
        beq.w   .L4536c                                | +338
        lea     0x5d426.l, a0                          | +33c
        move.l  a0, 0x84(a6)                           | +342
        bra.w   .L4536c                                | +346
.L4536c:
        bra.w   .L4537a                                | +34a
.L45370:
        lea     0x5d326.l, a0                          | +34e
        bra.w   .L4537a                                | +354
.L4537a:
        andi.w  #0xf, d0                               | +358
        asl.w   #0x1, d0                               | +35c
        cmpi.w  #0x0, d0                               | +35e
        move.w  (a0, d0.w), d1                         | +362
        cmpi.w  #0xffff, d1                            | +366
        beq.w   .L45408                                | +36a
        move.w  0x7c(a6), d0                           | +36e
        sub.w   d0, d1                                 | +372
        beq.w   .L453a0                                | +374
        smi     d2                                     | +378
        bra.w   .L453a4                                | +37a
.L453a0:
        move.b  0x77(a6), d2                           | +37e
.L453a4:
        cmpi.w  #0x8000, d1                            | +382
        bne.w   .L453b2                                | +386
        move.b  0x77(a6), d2                           | +38a
        not.b   d2                                     | +38e
.L453b2:
        move.b  d2, 0x77(a6)                           | +390
        ext.w   d2                                     | +394
        move.w  d1, d3                                 | +396
        bpl.w   .L453c0                                | +398
        neg.w   d3                                     | +39c
.L453c0:
        tst.b   0x73(a6)                               | +39e
        beq.w   .L453d4                                | +3a2
        move.w  #0x800, d4                             | +3a6
        clr.w   0x7e(a6)                               | +3aa
        bra.w   .L453ee                                | +3ae
.L453d4:
        move.w  d1, d4                                 | +3b2
        asr.w   #0x3, d4                               | +3b4
        move.w  0x7e(a6), d5                           | +3b6
        asr.w   #0x5, d5                               | +3ba
        sub.w   d5, d4                                 | +3bc
        add.w   0x7e(a6), d4                           | +3be
        move.w  d4, 0x7e(a6)                           | +3c2
        bpl.w   .L453ee                                | +3c6
        neg.w   d4                                     | +3ca
.L453ee:
        cmp.w   d4, d3                                 | +3cc
        bcc.w   .L453fa                                | +3ce
        move.w  d3, d4                                 | +3d2
        clr.w   0x7e(a6)                               | +3d4
.L453fa:
        eor.w   d2, d4                                 | +3d8
        sub.w   d2, d4                                 | +3da
        add.w   d4, 0x7c(a6)                           | +3dc
        move.b  0x7c(a6), d0                           | +3e0
        rts                                            | +3e4
.L45408:
        move.b  0x7c(a6), d0                           | +3e6
        clr.w   0x7e(a6)                               | +3ea
        rts                                            | +3ee
        .size   Ent_AimUpdate_045022, .-Ent_AimUpdate_045022

        .globl  Ent_AimInit_045412
        .type   Ent_AimInit_045412, @function
        .section .text.Ent_AimInit_045412, "ax", @progbits
Ent_AimInit_045412:
        clr.b   0x70(a6)                               | +000
        clr.b   0x71(a6)                               | +004
        clr.b   0x72(a6)                               | +008
        clr.b   0x73(a6)                               | +00c
        clr.b   0x74(a6)                               | +010
        move.b  0x106f2a.l, d1                         | +014
        cmpi.b  #0x2, d1                               | +01a
        bne.w   .L4544c                                | +01e
        move.b  #0x6, 0x7b(a6)                         | +022
        move.b  #0x6, 0x83(a6)                         | +028
        move.l  #0x5d406, 0x84(a6)                     | +02e
        bra.w   .L45460                                | +036
.L4544c:
        move.b  #0x4, 0x7b(a6)                         | +03a
        move.b  #0x4, 0x83(a6)                         | +040
        move.l  #0x5d386, 0x84(a6)                     | +046
.L45460:
        clr.b   0x76(a6)                               | +04e
        clr.b   0x75(a6)                               | +052
        clr.b   0x82(a6)                               | +056
        rts                                            | +05a
        .size   Ent_AimInit_045412, .-Ent_AimInit_045412

        .globl  Ent_InputSample_04546E
        .type   Ent_InputSample_04546E, @function
        .section .text.Ent_InputSample_04546E, "ax", @progbits
Ent_InputSample_04546E:
        move.b  0x71(a6), 0x70(a6)                     | +000
        clr.b   0x71(a6)                               | +006
        clr.b   0x72(a6)                               | +00a
        btst    #0x7, 0x100000.l                       | +00e
        bne.w   .L4549a                                | +016
        lea     0x1001c0.l, a4                         | +01a
        tst.b   0x44(a4)                               | +020
        beq.w   .L4549a                                | +024
        bra.w   .L455a4                                | +028
.L4549a:
        movea.l 0xc(a6), a1                            | +02c
        move.b  0x68(a1), 0x68(a6)                     | +030
        cmpi.b  #0xff, 0x68(a6)                        | +036
        beq.w   .L454da                                | +03c
        cmpi.b  #0x0, 0x68(a6)                         | +040
        bne.w   .L454c2                                | +046
        lea     0x100440.l, a1                         | +04a
        bra.w   .L454c8                                | +050
.L454c2:
        lea     0x1004e0.l, a1                         | +054
.L454c8:
        movea.l 0xc(a1), a1                            | +05a
        movea.l 0x72(a1), a2                           | +05e
        jsr     0x5d674.l                              | +062
        bra.w   .L454e0                                | +068
.L454da:
        lea     0x5cc08.l, a2                          | +06c
.L454e0:
        move.b  0x2(a2), d0                            | +072
        move.b  d0, 0x71(a6)                           | +076
        move.b  0x106f2a.l, d1                         | +07a
        cmpi.b  #0x1, d1                               | +080
        bne.w   .L454fe                                | +084
        move.b  #0x30, d1                              | +088
        bra.w   .L45532                                | +08c
.L454fe:
        cmpi.b  #0x2, d1                               | +090
        bne.w   .L4550e                                | +094
        move.b  #0xf0, d1                              | +098
        bra.w   .L45532                                | +09c
.L4550e:
        cmpi.b  #0x3, d1                               | +0a0
        bne.w   .L4551e                                | +0a4
        move.b  #0xf0, d1                              | +0a8
        bra.w   .L45532                                | +0ac
.L4551e:
        cmpi.b  #0x5, d1                               | +0b0
        bne.w   .L4552e                                | +0b4
        move.b  #0x30, d1                              | +0b8
        bra.w   .L45532                                | +0bc
.L4552e:
        move.b  #0x10, d1                              | +0c0
.L45532:
        and.b   d1, d0                                 | +0c4
        beq.w   .L4553c                                | +0c6
        move.b  d0, 0x83(a6)                           | +0ca
.L4553c:
        move.b  0x3(a2), d0                            | +0ce
        move.b  d0, 0x72(a6)                           | +0d2
        move.b  0x106f2a.l, d1                         | +0d6
        cmpi.b  #0x1, d1                               | +0dc
        bne.w   .L4555a                                | +0e0
        move.b  #0x30, d1                              | +0e4
        bra.w   .L4559a                                | +0e8
.L4555a:
        cmpi.b  #0x2, d1                               | +0ec
        bne.w   .L4556a                                | +0f0
        move.b  #0xf0, d1                              | +0f4
        bra.w   .L4559a                                | +0f8
.L4556a:
        cmpi.b  #0x3, d1                               | +0fc
        bne.w   .L45586                                | +100
        move.b  #0xf0, d1                              | +104
        andi.b  #0xf0, d0                              | +108
        move.b  d0, 0x7b(a6)                           | +10c
        bra.w   .L455a4                                | +110
        bra.w   .L4559a                                | +114
.L45586:
        cmpi.b  #0x5, d1                               | +118
        bne.w   .L45596                                | +11c
        move.b  #0x30, d1                              | +120
        bra.w   .L4559a                                | +124
.L45596:
        move.b  #0x10, d1                              | +128
.L4559a:
        and.b   d1, d0                                 | +12c
        beq.w   .L455a4                                | +12e
        move.b  d0, 0x7b(a6)                           | +132
.L455a4:
        move.b  0x70(a6), d1                           | +136
        andi.b  #0xf, d1                               | +13a
        beq.w   .L455b8                                | +13e
        clr.b   0x73(a6)                               | +142
        bra.w   .L455c4                                | +146
.L455b8:
        move.b  0x71(a6), d0                           | +14a
        andi.b  #0xf, d0                               | +14e
        sne     0x73(a6)                               | +152
.L455c4:
        move.b  0x106f2a.l, d1                         | +156
        cmpi.b  #0x1, d1                               | +15c
        bne.w   .L455da                                | +160
        move.b  #0x30, d2                              | +164
        bra.w   .L4560e                                | +168
.L455da:
        cmpi.b  #0x2, d1                               | +16c
        bne.w   .L455ea                                | +170
        move.b  #0xf0, d2                              | +174
        bra.w   .L4560e                                | +178
.L455ea:
        cmpi.b  #0x3, d1                               | +17c
        bne.w   .L455fa                                | +180
        move.b  #0xf0, d2                              | +184
        bra.w   .L4560e                                | +188
.L455fa:
        cmpi.b  #0x5, d1                               | +18c
        bne.w   .L4560a                                | +190
        move.b  #0x30, d2                              | +194
        bra.w   .L4560e                                | +198
.L4560a:
        move.b  #0x10, d2                              | +19c
.L4560e:
        move.b  d2, d1                                 | +1a0
        and.b   0x71(a6), d2                           | +1a2
        beq.w   .L45632                                | +1a6
        move.b  d1, d2                                 | +1aa
        and.b   0x70(a6), d2                           | +1ac
        beq.w   .L4562e                                | +1b0
        tst.b   0x74(a6)                               | +1b4
        beq.w   .L4562e                                | +1b8
        subq.b  #0x1, 0x74(a6)                         | +1bc
.L4562e:
        bra.w   .L45646                                | +1c0
.L45632:
        tst.b   0x74(a6)                               | +1c4
        bne.w   .L45640                                | +1c8
        move.b  #0xff, 0x73(a6)                        | +1cc
.L45640:
        move.b  #0x6, 0x74(a6)                         | +1d2
.L45646:
        rts                                            | +1d8
        .size   Ent_InputSample_04546E, .-Ent_InputSample_04546E

        .globl  Ent_FireGate_045648
        .type   Ent_FireGate_045648, @function
        .section .text.Ent_FireGate_045648, "ax", @progbits
Ent_FireGate_045648:
        move.b  0x106f2a.l, d0                         | +000
        cmpi.b  #0x1, d0                               | +006
        bne.w   .L4565e                                | +00a
        move.b  #0x30, d0                              | +00e
        bra.w   .L4568e                                | +012
.L4565e:
        cmpi.b  #0x2, d0                               | +016
        bne.w   .L4566e                                | +01a
        move.b  #0xf0, d0                              | +01e
        bra.w   .L4568e                                | +022
.L4566e:
        cmpi.b  #0x3, d0                               | +026
        bne.w   .L4567e                                | +02a
        move.b  #0xf0, d0                              | +02e
        bra.w   .L4568e                                | +032
.L4567e:
        cmpi.b  #0x5, d0                               | +036
        bne.w   .L4568a                                | +03a
        move.b  #0x30, d0                              | +03e
.L4568a:
        move.b  #0x10, d0                              | +042
.L4568e:
        and.b   0x72(a6), d0                           | +046
        beq.w   .L4569c                                | +04a
        move.b  #0xff, 0x76(a6)                        | +04e
.L4569c:
        move.b  0x75(a6), d0                           | +054
        beq.w   .L456ae                                | +058
        subq.b  #0x1, d0                               | +05c
        move.b  d0, 0x75(a6)                           | +05e
        bra.w   ClearXN_045700                         | +062
.L456ae:
        tst.b   0x74(a6)                               | +066
        bne.w   .L456ee                                | +06a
        cmpi.b  #0x3, 0x106f2a.l                       | +06e
        beq.w   .L456ee                                | +076
        cmpi.b  #0x2, 0x106f2a.l                       | +07a
        beq.w   .L456ee                                | +082
        cmpi.b  #0x1, 0x106f2a.l                       | +086
        bne.w   .L456e4                                | +08e
        move.b  #0x18, 0x75(a6)                        | +092
        bra.w   Ent_AmmoTick_045706                    | +098
.L456e4:
        move.b  #0x6, 0x75(a6)                         | +09c
        bra.w   Ent_AmmoTick_045706                    | +0a2
.L456ee:
        tst.b   0x76(a6)                               | +0a6
        beq.w   ClearXN_045700                         | +0aa
        move.b  #0x2, 0x75(a6)                         | +0ae
        bra.w   Ent_AmmoTick_045706                    | +0b4
        .size   Ent_FireGate_045648, .-Ent_FireGate_045648

        .globl  Ent_AmmoTick_045706
        .type   Ent_AmmoTick_045706, @function
        .section .text.Ent_AmmoTick_045706, "ax", @progbits
Ent_AmmoTick_045706:
        subi.b  #0x1, 0x79(a6)                         | +000
        clr.b   0x76(a6)                               | +006
        .size   Ent_AmmoTick_045706, .-Ent_AmmoTick_045706

        .globl  Ent_ClampY_045716
        .type   Ent_ClampY_045716, @function
        .section .text.Ent_ClampY_045716, "ax", @progbits
Ent_ClampY_045716:
        cmpi.w  #0x170, 0x24(a6)                       | +000
        bge.w   .L45720                                | +006
.L45720:
        cmpi.w  #0x170, 0x24(a6)                       | +00a
        ble.w   .L4572a                                | +010
.L4572a:
        rts                                            | +014
        .size   Ent_ClampY_045716, .-Ent_ClampY_045716

        .globl  Ent_GroundProbe_04572C
        .type   Ent_GroundProbe_04572C, @function
        .section .text.Ent_GroundProbe_04572C, "ax", @progbits
Ent_GroundProbe_04572C:
        move.b  0x35(a6), d0                           | +000
        lsl.w   #0x2, d0                               | +004
        andi.w  #0xff, d0                              | +006
        lea     0x29d752.l, a0                         | +00a
        move.w  0x22(a6), d1                           | +010
        move.w  0x24(a6), d2                           | +014
        add.w   (a0, d0.w), d1                         | +018
        add.w   0x2(a0, d0.w), d2                      | +01c
        jsr     0x280c6.l                              | +020
        cmpi.b  #0x0, d0                               | +026
        beq.w   SetXN_045784                           | +02a
        cmpi.b  #0x10, d0                              | +02e
        beq.w   ClearXN_04577e                         | +032
        cmpi.b  #0x20, d0                              | +036
        beq.w   ClearXN_04577e                         | +03a
        cmpi.b  #0x21, d0                              | +03e
        beq.w   ClearXN_04577e                         | +042
        andi.b  #0xf0, d0                              | +046
        cmpi.b  #0x0, d0                               | +04a
        bne.w   SetXN_045784                           | +04e
        .size   Ent_GroundProbe_04572C, .-Ent_GroundProbe_04572C

        .globl  Entity_CmpDepthToParent_04578A
        .type   Entity_CmpDepthToParent_04578A, @function
        .section .text.Entity_CmpDepthToParent_04578A, "ax", @progbits
Entity_CmpDepthToParent_04578A:
        movea.l 0x8(a6), a1                            | +000
        move.b  0x10(a6), d0                           | +004
        cmp.b   0x10(a1), d0                           | +008
        bcs.w   SetXN_0457a0                           | +00c
        .size   Entity_CmpDepthToParent_04578A, .-Entity_CmpDepthToParent_04578A

        .globl  Ent_SfxByMode_0457A6
        .type   Ent_SfxByMode_0457A6, @function
        .section .text.Ent_SfxByMode_0457A6, "ax", @progbits
Ent_SfxByMode_0457A6:
        move.w  #0x1d6, d1                             | +000
        move.b  0x6d(a6), d0                           | +004
        cmpi.b  #0x1, d0                               | +008
        beq.w   .L457c6                                | +00c
        move.w  #0x1d6, d1                             | +010
        cmpi.b  #0x2, d0                               | +014
        beq.w   .L457c6                                | +018
        move.w  #0x2, d1                               | +01c
.L457c6:
        jmp     0x236e.l                               | +020
        .size   Ent_SfxByMode_0457A6, .-Ent_SfxByMode_0457A6

        .globl  Ent_AnimFrame_0457CC
        .type   Ent_AnimFrame_0457CC, @function
        .section .text.Ent_AnimFrame_0457CC, "ax", @progbits
Ent_AnimFrame_0457CC:
        lea     0x289204.l, a0                         | +000
        move.b  0x34(a6), d0                           | +006
        addq.w  #0x4, d0                               | +00a
        andi.w  #0xf8, d0                              | +00c
        move.w  d0, d1                                 | +010
        add.w   d0, d0                                 | +012
        add.w   d0, d1                                 | +014
        move.b  0x3b(a6), d0                           | +016
        andi.w  #0xff, d0                              | +01a
        add.w   d0, d0                                 | +01e
        add.w   d0, d0                                 | +020
        add.w   d1, d0                                 | +022
        move.l  (a0, d0.w), 0x3c(a6)                   | +024
        jsr     0x5ca2a.l                              | +02a
        move.b  0x3b(a6), d0                           | +030
        beq.w   SetTaskBRts_04580a                     | +034
        subq.b  #0x1, d0                               | +038
        .size   Ent_AnimFrame_0457CC, .-Ent_AnimFrame_0457CC

