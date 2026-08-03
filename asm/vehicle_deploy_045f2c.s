| =====================================================================
| vehicle_deploy_045f2c.s — Wave YY (parte 2/2)
| Region: $045F2C..$046258 (15 entradas, 788 B, byte-exacto)
| =====================================================================
|
| DESPLIEGUE / LANZAMIENTO DE VEHICULO + ENEMY46
|
| * Task_KillFlag10060C_045F2C: fija el bit 4 de $10060C y se auto-mata
|   via $518 (aviso al sistema de que el vehiculo termino).
| * VehicleAnim_Table_045F3A: tabla de datos con dos guiones de animacion
|   de 80 bytes (cabeceras 0x03 / 0x04) terminados en FFFF FFFF —
|   secuencias de frames del despliegue del vehiculo.
| * Vehicle_JmpDeploy_045FDE: trampolin jmp al lanzador.
| * Vehicle_Launch_045FE4: lanzamiento — calcula el angulo hacia el
|   objetivo via $13C0E, convierte a velocidades X/Y con las tablas de
|   senos y arranca el vuelo.
| * Vehicle_Flight_046078: vuelo balistico con gravedad; al impactar
|   bifurca a CrashA/CrashB segun el lado.
| * Vehicle_CrashA_0460EE / Vehicle_CrashB_0460FA: choques cortos que
|   encadenan la explosion. Stub_00046106: rts suelto de relleno.
| * Entity_CmpDepthToParent_046108 / _046124: comparadores de profundidad
|   contra el padre; retornan con flags via los islotes SetXN_04611e /
|   SetXN_04613a (ori.b #$11,ccr ; rts) — mismo idiom que $04578A.
| * Enemy46_*: maquina de estados del enemigo tipo $46 — Boot inicializa
|   y registra la tarea, PhaseA espera la condicion de activacion, Move
|   desplaza con velocidad fija, PhaseB alterna el patron y Tail cierra
|   instalando el handler del hueco siguiente (Fn_00046260) y llamando a
|   la rutina futura Fn_000463C2 (ambas pendientes de la proxima wave).
| =====================================================================

        .globl  Task_KillFlag10060C_045F2C
        .type   Task_KillFlag10060C_045F2C, @function
        .section .text.Task_KillFlag10060C_045F2C, "ax", @progbits
Task_KillFlag10060C_045F2C:
        bset    #0x4, 0x10060c.l                       | +000
        jmp     0x518.l                                | +008
        .size   Task_KillFlag10060C_045F2C, .-Task_KillFlag10060C_045F2C

        .globl  VehicleAnim_Table_045F3A
        .section .text.VehicleAnim_Table_045F3A, "ax", @progbits
VehicleAnim_Table_045F3A:
        | ---- script A (cabecera 0x03) ----
        .byte   0x03, 0x02, 0x00, 0x0a, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00
        .byte   0xff, 0xfd, 0x00, 0x03, 0xff, 0xfd, 0x00, 0x03, 0x02, 0x00
        .byte   0x00, 0x24, 0x36, 0x3a, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00
        .byte   0x02, 0x00, 0x00, 0x24, 0x36, 0x44, 0xff, 0xff, 0xff, 0xfd
        .byte   0x00, 0x03, 0x02, 0x00, 0x00, 0x24, 0x36, 0x4e, 0xff, 0xff
        .byte   0xff, 0xfd, 0xff, 0xfd, 0x02, 0x00, 0x00, 0x24, 0x36, 0x58
        .byte   0xff, 0xff, 0x00, 0x03, 0x00, 0x03, 0x02, 0x00, 0x00, 0x24
        .byte   0x36, 0x62, 0xff, 0xff, 0x00, 0x03, 0xff, 0xfd, 0x1d, 0x00
        | ---- script B (cabecera 0x04) ----
        .byte   0x04, 0x02, 0x00, 0x0a, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00
        .byte   0xff, 0xfd, 0x00, 0x03, 0xff, 0xfd, 0x00, 0x03, 0x02, 0x00
        .byte   0x00, 0x24, 0x36, 0x3a, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00
        .byte   0x02, 0x00, 0x00, 0x24, 0x36, 0x44, 0xff, 0xff, 0xff, 0xfd
        .byte   0x00, 0x03, 0x02, 0x00, 0x00, 0x24, 0x36, 0x4e, 0xff, 0xff
        .byte   0xff, 0xfd, 0xff, 0xfd, 0x02, 0x00, 0x00, 0x24, 0x36, 0x58
        .byte   0xff, 0xff, 0x00, 0x03, 0x00, 0x03, 0x02, 0x00, 0x00, 0x24
        .byte   0x36, 0x62, 0xff, 0xff, 0x00, 0x03, 0xff, 0xfd, 0x1d, 0x00
        .word   0xFFFF, 0xFFFF                         | terminador
        .size   VehicleAnim_Table_045F3A, .-VehicleAnim_Table_045F3A

        .globl  Vehicle_JmpDeploy_045FDE
        .type   Vehicle_JmpDeploy_045FDE, @function
        .section .text.Vehicle_JmpDeploy_045FDE, "ax", @progbits
Vehicle_JmpDeploy_045FDE:
        jmp     0x308b0.l                              | +000
        .size   Vehicle_JmpDeploy_045FDE, .-Vehicle_JmpDeploy_045FDE

        .globl  Vehicle_Launch_045FE4
        .type   Vehicle_Launch_045FE4, @function
        .section .text.Vehicle_Launch_045FE4, "ax", @progbits
Vehicle_Launch_045FE4:
        bset    #0x4, 0x6b(a6)                         | +000
        lea     0x30804.l, a0                          | +006
        move.l  a0, 0x4c(a6)                           | +00c
        jsr     0x283ca.l                              | +010
        move.w  #0xd000, 0x38(a6)                      | +016
        lea     0x77a96.l, a1                          | +01c
        jsr     0x4ae.l                                | +022
        jsr     0x5dd02.l                              | +028
        move.b  0x98(a6), d0                           | +02e
        move.w  d0, d1                                 | +032
        moveq   #0x0, d0                               | +034
        andi.w  #0x3, d0                               | +036
        subq.w  #0x2, d0                               | +03a
        add.w   d1, d0                                 | +03c
        andi.w  #0xff, d0                              | +03e
        move.w  #0x800, d1                             | +042
        jsr     0x13c0e.l                              | +046
        move.w  d1, 0x28(a6)                           | +04c
        move.w  d2, 0x2a(a6)                           | +050
        move.w  #0x2, d1                               | +054
        jsr     0x236e.l                               | +058
        move.b  0x98(a6), d0                           | +05e
        addi.w  #0x8, d0                               | +062
        andi.w  #0xf0, d0                              | +066
        lsr.w   #0x4, d0                               | +06a
        movea.l #0x29d452, a0                          | +06c
        lsl.w   #0x2, d0                               | +072
        movea.l (a0, d0.w), a0                         | +074
        cmpa.l  #0xffffffff, a0                        | +078
        beq.w   .L4606c                                | +07e
        jsr     0x28cd4.l                              | +082
.L4606c:
        jsr     0x283ca.l                              | +088
        lea     Vehicle_Flight_046078(pc), a1          | +08e
        move.l  a1, (a6)                               | +092
        .size   Vehicle_Launch_045FE4, .-Vehicle_Launch_045FE4

        .globl  Vehicle_Flight_046078
        .type   Vehicle_Flight_046078, @function
        .section .text.Vehicle_Flight_046078, "ax", @progbits
Vehicle_Flight_046078:
        bset    #0x6, 0x13(a6)                         | +000
        jsr     0x27cee.l                              | +006
        bcs.w   .L46092                                | +00c
        jsr     0x27cee.l                              | +010
        bcc.w   .L460ba                                | +016
.L46092:
        lea     Vehicle_CrashA_0460EE(pc), a1          | +01a
        move.l  a1, (a6)                               | +01e
        cmpi.b  #0x38, d0                              | +020
        beq.w   .L460b4                                | +024
        andi.b  #0xc0, d7                              | +028
        cmpi.b  #0x0, d7                               | +02c
        beq.w   .L460ba                                | +030
        cmpi.b  #0xc0, d7                              | +034
        beq.w   .L460ba                                | +038
.L460b4:
        jmp     0x518.l                                | +03c
.L460ba:
        jsr     0x28d70.l                              | +042
        jsr     0x283d8.l                              | +048
        btst    #0x1, 0x13(a6)                         | +04e
        beq.w   .L460d6                                | +054
        lea     Vehicle_CrashB_0460FA(pc), a1          | +058
        move.l  a1, (a6)                               | +05c
.L460d6:
        movea.l #0xffffffff, a0                        | +05e
        jsr     0x5dd56.l                              | +064
        bcc.w   .L460ec                                | +06a
        jmp     0x518.l                                | +06e
.L460ec:
        rts                                            | +074
        .size   Vehicle_Flight_046078, .-Vehicle_Flight_046078

        .globl  Vehicle_CrashA_0460EE
        .type   Vehicle_CrashA_0460EE, @function
        .section .text.Vehicle_CrashA_0460EE, "ax", @progbits
Vehicle_CrashA_0460EE:
        jsr     0x13600.l                              | +000
        jmp     0x31c72.l                              | +006
        .size   Vehicle_CrashA_0460EE, .-Vehicle_CrashA_0460EE

        .globl  Vehicle_CrashB_0460FA
        .type   Vehicle_CrashB_0460FA, @function
        .section .text.Vehicle_CrashB_0460FA, "ax", @progbits
Vehicle_CrashB_0460FA:
        jsr     0x13600.l                              | +000
        jmp     0x31cca.l                              | +006
        .size   Vehicle_CrashB_0460FA, .-Vehicle_CrashB_0460FA

        .globl  Stub_00046106
        .type   Stub_00046106, @function
        .section .text.Stub_00046106, "ax", @progbits
Stub_00046106:
        rts                                            | +000
        .size   Stub_00046106, .-Stub_00046106

        .globl  Entity_CmpDepthToParent_046108
        .type   Entity_CmpDepthToParent_046108, @function
        .section .text.Entity_CmpDepthToParent_046108, "ax", @progbits
Entity_CmpDepthToParent_046108:
        movea.l 0x8(a6), a1                            | +000
        move.b  0x10(a6), d0                           | +004
        cmp.b   0x10(a1), d0                           | +008
        bcs.w   SetXN_04611e                           | +00c
        .size   Entity_CmpDepthToParent_046108, .-Entity_CmpDepthToParent_046108

        .globl  Entity_CmpDepthToParent_046124
        .type   Entity_CmpDepthToParent_046124, @function
        .section .text.Entity_CmpDepthToParent_046124, "ax", @progbits
Entity_CmpDepthToParent_046124:
        movea.l 0x8(a6), a1                            | +000
        move.b  0x10(a6), d0                           | +004
        cmp.b   0x10(a1), d0                           | +008
        bcs.w   SetXN_04613a                           | +00c
        .size   Entity_CmpDepthToParent_046124, .-Entity_CmpDepthToParent_046124

        .globl  Enemy46_Boot_046140
        .type   Enemy46_Boot_046140, @function
        .section .text.Enemy46_Boot_046140, "ax", @progbits
Enemy46_Boot_046140:
        jsr     0x5e7c0.l                              | +000
        move.w  #0x1a, d1                              | +006
        tst.b   0x9a(a6)                               | +00a
        beq.w   .L46156                                | +00e
        move.w  #0x19f, d1                             | +012
.L46156:
        jsr     0x236e.l                               | +016
        move.b  0x99(a6), 0x3a(a6)                     | +01c
        move.w  #0x8000, d0                            | +022
        jsr     0x28134.l                              | +026
        andi.w  #0xffe3, 0x38(a6)                      | +02c
        ori.w   #0x18, 0x38(a6)                        | +032
        tst.b   0x9d(a6)                               | +038
        beq.w   Enemy46_Move_0461BC                    | +03c
        lea     0x2e2286.l, a0                         | +040
        jsr     0x28cd4.l                              | +046
        lea     Enemy46_PhaseA_046192(pc), a1          | +04c
        move.l  a1, (a6)                               | +050
        .size   Enemy46_Boot_046140, .-Enemy46_Boot_046140

        .globl  Enemy46_PhaseA_046192
        .type   Enemy46_PhaseA_046192, @function
        .section .text.Enemy46_PhaseA_046192, "ax", @progbits
Enemy46_PhaseA_046192:
        jsr     0x2783a.l                              | +000
        jsr     0x27eba.l                              | +006
        bcc.w   .L461a8                                | +00c
        jsr     0x27c8c.l                              | +010
.L461a8:
        jsr     0x28d70.l                              | +016
        bcc.w   .L461b8                                | +01c
        lea     Enemy46_Move_0461BC(pc), a1            | +020
        move.l  a1, (a6)                               | +024
.L461b8:
        bra.w   Enemy46_Tail_046220                    | +026
        .size   Enemy46_PhaseA_046192, .-Enemy46_PhaseA_046192

        .globl  Enemy46_Move_0461BC
        .type   Enemy46_Move_0461BC, @function
        .section .text.Enemy46_Move_0461BC, "ax", @progbits
Enemy46_Move_0461BC:
        lea     0x2b7716.l, a0                         | +000
        jsr     0x799de.l                              | +006
        btst    #0x0, 0x3a(a6)                         | +00c
        bne.w   .L461d4                                | +012
        neg.w   d0                                     | +016
.L461d4:
        move.w  d0, 0x28(a6)                           | +018
        lea     0x29b744.l, a0                         | +01c
        jsr     0x28cd4.l                              | +022
        lea     Enemy46_PhaseB_0461EA(pc), a1          | +028
        move.l  a1, (a6)                               | +02c
        .size   Enemy46_Move_0461BC, .-Enemy46_Move_0461BC

        .globl  Enemy46_PhaseB_0461EA
        .type   Enemy46_PhaseB_0461EA, @function
        .section .text.Enemy46_PhaseB_0461EA, "ax", @progbits
Enemy46_PhaseB_0461EA:
        jsr     0x2783a.l                              | +000
        jsr     0x27eba.l                              | +006
        bcc.w   .L46204                                | +00c
        jsr     0x27c8c.l                              | +010
        bra.w   .L4621a                                | +016
.L46204:
        jsr     0x27a92.l                              | +01a
        btst    #0x5, 0x5a(a6)                         | +020
        beq.w   .L4621a                                | +026
        lea     Fn_00046260(pc), a1                    | +02a
        move.l  a1, (a6)                               | +02e
.L4621a:
        jsr     0x28d70.l                              | +030
        .size   Enemy46_PhaseB_0461EA, .-Enemy46_PhaseB_0461EA

        .globl  Enemy46_Tail_046220
        .type   Enemy46_Tail_046220, @function
        .section .text.Enemy46_Tail_046220, "ax", @progbits
Enemy46_Tail_046220:
        jsr     0x2870a.l                              | +000
        bcc.w   .L46248                                | +006
        jsr     Fn_000463C2(pc)                    | +00a
        tst.b   0x9a(a6)                               | +00e
        bne.w   .L46240                                | +012
        jsr     0x49fd0.l                              | +016
        bra.w   .L46248                                | +01c
.L46240:
        lea     0x4abc0.l, a1                          | +020
        move.l  a1, (a6)                               | +026
.L46248:
        movea.l #0xffffffff, a0                        | +028
        jsr     0x5dd5c.l                              | +02e
        bcc.w   SetHandlerRts_04625e                   | +034
        .size   Enemy46_Tail_046220, .-Enemy46_Tail_046220

