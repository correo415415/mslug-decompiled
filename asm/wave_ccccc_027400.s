| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $027400..$02A000  (6,380 B, 117 entradas, 59 huecos)
| ============================================================================
|
|  BORRADOR generado por tools/gen_asm_region.py — pendiente de análisis
|  semántico (nombres, comentarios de campo, evidencias).
|
|  Verificación: cada sección .text.<Sym> se coloca en su dirección CPU
|  absoluta y reensambla byte-exacta contra build/mslug_prom.bin
|  (MD5 816b3f74c76b3373993407615f1850fe).
| ============================================================================

        .text

| ----------------------------------------------------------------------------
|  TaskHandler_027400  @ $027400  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027400, "ax", @progbits
        .global TaskHandler_027400
TaskHandler_027400:
        cmpi.w  #0xffff,0x34(a6)                | +000
        beq.w   TaskHandler_02742a              | +006
        jsr     0x44182.l                       | +00a
        move.w  -0x1148(a5),d1                  | +010
        move.w  -0x1146(a5),d2                  | +014
        jsr     SpritePubEffect_027EBA__L027ec2(pc) | +018
        bcs.w   TaskHandler_02742a              | +01c
        movem.l (a7)+,d3-d6/a1                  | +020

| ----------------------------------------------------------------------------
|  TaskHandler_02742a  @ $02742A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02742a, "ax", @progbits
        .global TaskHandler_02742a
TaskHandler_02742a:
        movem.l (a7)+,d3-d6/a1                  | +000
        move.w  0x2a(a6),d0                     | +004
        move.w  #0x400,d1                       | +008
        jsr     ClampVelocity_Default_0267f4(pc) | +00c
        move.w  d0,0x2a(a6)                     | +010
        move.w  #0xffc0,0x2e(a6)                | +014

| ----------------------------------------------------------------------------
|  Sub_000027444  @ $027444  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000027444, "ax", @progbits
        .global Sub_000027444
Sub_000027444:
        jsr     TaskHandler_0274b8(pc)          | +000
        bcc.w   ClearXN_02745e                  | +004
        move.w  #0x0,0x2e(a6)                   | +008
        move.w  #0x0,0x2a(a6)                   | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_027464  @ $027464  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027464, "ax", @progbits
        .global TaskHandler_027464
TaskHandler_027464:
        move.w  #0xffff,0x34(a6)                | +000
        move.w  #0xffff,0x64(a6)                | +006
        move.w  0x2c(a6),d0                     | +00c
        add.w   0x28(a6),d0                     | +010
        jsr     ClampVelocity_0267f8(pc)        | +014
        move.w  d0,0x28(a6)                     | +018
        move.w  0x2e(a6),d0                     | +01c
        add.w   0x2a(a6),d0                     | +020
        bgt.w   .L027490                        | +024
        jsr     ClampVelocity_0267f8(pc)        | +028
.L027490:
        move.w  d0,0x2a(a6)                     | +02c
        jsr     Entity_MoveX_WallStop_026836(pc) | +030
        jsr     TaskHandler_0274b8__L0274cc(pc) | +034
        bcc.w   ClearXN_0274b2                  | +038
        move.w  #0x0,0x2e(a6)                   | +03c
        move.w  #0x0,0x2a(a6)                   | +042

| ----------------------------------------------------------------------------
|  TaskHandler_0274b8  @ $0274B8  (250 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0274b8, "ax", @progbits
        .global TaskHandler_0274b8
TaskHandler_0274b8:
        move.w  #0xffff,0x34(a6)                | +000
        move.w  #0xffff,0x64(a6)                | +006
        jsr     Entity_IntegrateVelocity_026814(pc) | +00c
        jsr     Entity_MoveX_WallStop_026836(pc) | +010
        .global TaskHandler_0274b8__L0274cc
TaskHandler_0274b8__L0274cc:
.L0274cc:
        jsr     0x44182.l                       | +014
        move.w  -0x1148(a5),d1                  | +01a
        move.w  -0x1146(a5),d2                  | +01e
        add.w   -0x1154(a5),d1                  | +022
        jsr     Sub_00027DB2(pc)                | +026
        cmpi.b  #0x1,d0                         | +02a
        beq.w   .L027526                        | +02e
        cmpi.b  #0x10,d0                        | +032
        beq.w   .L027526                        | +036
        cmpi.b  #0x26,d0                        | +03a
        beq.w   .L027526                        | +03e
        cmpi.b  #0x27,d0                        | +042
        beq.w   .L027526                        | +046
        cmpi.b  #0xc,d0                         | +04a
        beq.w   .L027526                        | +04e
        cmpi.b  #0xd,d0                         | +052
        beq.w   .L027526                        | +056
        cmpi.b  #0xe,d0                         | +05a
        beq.w   .L027526                        | +05e
        cmpi.b  #0xf,d0                         | +062
        beq.w   .L027526                        | +066
        bra.w   .L027534                        | +06a
.L027526:
        bset    #0x5,0x5a(a6)                   | +06e
        clr.w   -0x1154(a5)                     | +074
        clr.b   -0x1150(a5)                     | +078
.L027534:
        move.w  -0x1148(a5),d1                  | +07c
        move.w  -0x1146(a5),d2                  | +080
        add.w   -0x1154(a5),d1                  | +084
        add.w   -0x1152(a5),d2                  | +088
        subi.w  #0x1,d2                         | +08c
        jsr     Sub_00027DB2(pc)                | +090
        jsr     0x9993c.l                       | +094
        move.b  d6,0x106f44.l                   | +09a
        cmpi.b  #0xf,d6                         | +0a0
        beq.w   .L027572                        | +0a4
        moveq   #0,d0                           | +0a8
        moveq   #0,d3                           | +0aa
        move.w  d5,d4                           | +0ac
        moveq   #17,d0                          | +0ae
        lea     0x278ba8.l,a1                   | +0b0
        bra.w   .L027572                        | +0b6
.L027572:
        cmpi.b  #0x28,d0                        | +0ba
        bne.w   .L027586                        | +0be
        tst.w   0x2a(a6)                        | +0c2
        ble.w   .L027586                        | +0c6
        clr.w   0x2a(a6)                        | +0ca
.L027586:
        jsr     Sub_00027E28(pc)                | +0ce
        bcc.w   .L02759a                        | +0d2
        movea.l 0x4(a1),a3                      | +0d6
        bsr.w   TaskHandler_02822c              | +0da
        bcc.w   TaskHandler_0275b8              | +0de
        .global TaskHandler_0274b8__L02759a
TaskHandler_0274b8__L02759a:
.L02759a:
        move.w  d1,-0x1148(a5)                  | +0e2
        addi.w  #0x1,d2                         | +0e6
        move.w  d2,-0x1146(a5)                  | +0ea
        move.b  -0x1150(a5),-0x1144(a5)         | +0ee
        move.b  -0x114f(a5),-0x1143(a5)         | +0f4

| ----------------------------------------------------------------------------
|  TaskHandler_0275b8  @ $0275B8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0275b8, "ax", @progbits
        .global TaskHandler_0275b8
TaskHandler_0275b8:
        movea.l 0x4(a1),a3                      | +000
        jsr     TaskHandler_0275f6(pc)          | +004
        bcs.b   TaskHandler_0274b8__L02759a     | +008
        addi.w  #0x1,d2                         | +00a
        move.w  d1,-0x1148(a5)                  | +00e
        move.w  d2,-0x1146(a5)                  | +012
        move.b  #0x0,-0x1144(a5)                | +016
        asr.w   #0x1,d3                         | +01c
        move.b  0x12(a3,d3.w),d2                | +01e
        move.b  d2,-0x1143(a5)                  | +022
        bset    #0x6,0x5a(a6)                   | +026
        move.b  0x106f44.l,d0                   | +02c
        jsr     0x999ca.l                       | +032

| ----------------------------------------------------------------------------
|  TaskHandler_0275f6  @ $0275F6  (304 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0275f6, "ax", @progbits
        .global TaskHandler_0275f6
TaskHandler_0275f6:
        cmpi.b  #0xc,d0                         | +000
        beq.w   .L0276a8                        | +004
        cmpi.b  #0xd,d0                         | +008
        beq.w   .L0276a8                        | +00c
        cmpi.b  #0xe,d0                         | +010
        beq.w   .L0276a8                        | +014
        cmpi.b  #0xf,d0                         | +018
        beq.w   .L0276a8                        | +01c
        cmpi.b  #0x1c,d0                        | +020
        beq.w   .L0276a8                        | +024
        cmpi.b  #0x1d,d0                        | +028
        beq.w   .L0276a8                        | +02c
        cmpi.b  #0x1e,d0                        | +030
        beq.w   .L0276a8                        | +034
        cmpi.b  #0x1f,d0                        | +038
        beq.w   .L0276a8                        | +03c
        cmpi.b  #0x34,d0                        | +040
        beq.w   .L0276a8                        | +044
        cmpi.b  #0x2c,d0                        | +048
        beq.w   .L0276a6                        | +04c
        cmpi.b  #0x2d,d0                        | +050
        beq.w   .L0276a6                        | +054
        cmpi.b  #0x2e,d0                        | +058
        beq.w   .L0276a6                        | +05c
        cmpi.b  #0x2f,d0                        | +060
        beq.w   .L0276a6                        | +064
        cmpi.b  #0x3c,d0                        | +068
        beq.w   .L0276a6                        | +06c
        cmpi.b  #0x3d,d0                        | +070
        beq.w   .L0276a6                        | +074
        cmpi.b  #0x3e,d0                        | +078
        beq.w   .L0276a6                        | +07c
        cmpi.b  #0x3f,d0                        | +080
        beq.w   .L0276a6                        | +084
        cmpi.b  #0x20,d0                        | +088
        beq.w   .L0276a2                        | +08c
        cmpi.b  #0x21,d0                        | +090
        beq.w   .L0276a2                        | +094
        cmpi.b  #0x30,d0                        | +098
        beq.w   .L0276a2                        | +09c
        cmpi.b  #0x31,d0                        | +0a0
        beq.w   .L0276a2                        | +0a4
        bra.w   .L0276d4                        | +0a8
.L0276a2:
        bra.w   .L0276a8                        | +0ac
.L0276a6:
        addq.w  #0x8,d2                         | +0b0
.L0276a8:
        addq.w  #0x8,d2                         | +0b2
        jsr     Sub_00027DB2(pc)                | +0b4
        jsr     0x9993c.l                       | +0b8
        move.b  d6,0x106f44.l                   | +0be
        cmpi.b  #0xf,d6                         | +0c4
        beq.w   .L0276d4                        | +0c8
        moveq   #0,d0                           | +0cc
        moveq   #0,d3                           | +0ce
        move.w  d5,d4                           | +0d0
        moveq   #17,d0                          | +0d2
        lea     0x278ba8.l,a1                   | +0d4
        bra.w   .L0276d4                        | +0da
.L0276d4:
        movea.l 0x4(a1),a2                      | +0de
        move.w  (a2),0x34(a6)                   | +0e2
        move.w  0x1a(a2),0x64(a6)               | +0e6
        cmpi.w  #0x80,0x34(a6)                  | +0ec
        bne.w   .L0276fa                        | +0f2
        nop                                     | +0f6
        nop                                     | +0f8
        cmpi.w  #0x80,0x34(a6)                  | +0fa
        nop                                     | +100
        trap    #0xf                            | +102
.L0276fa:
        cmpi.w  #0xff80,0x34(a6)                | +104
        bne.w   .L027712                        | +10a
        nop                                     | +10e
        nop                                     | +110
        cmpi.w  #0xff80,0x34(a6)                | +112
        nop                                     | +118
        trap    #0xf                            | +11a
.L027712:
        btst    #0x7,0x13(a6)                   | +11c
        beq.w   TaskHandler_02772c              | +122
        cmpi.w  #0xffff,0x34(a6)                | +126
        bne.w   TaskHandler_02772c              | +12c

| ----------------------------------------------------------------------------
|  TaskHandler_02772c  @ $02772C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02772c, "ax", @progbits
        .global TaskHandler_02772c
TaskHandler_02772c:
        add.w   d4,d2                           | +000
        asl.w   #0x1,d3                         | +002
        move.w  0x2(a2,d3.w),d4                 | +004
        sub.w   d4,d2                           | +008

| ----------------------------------------------------------------------------
|  Sub_00002773C  @ $02773C  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00002773C, "ax", @progbits
        .global Sub_00002773C
Sub_00002773C:
        jsr     0x44182.l                       | +000
        jsr     Entity_IntegrateVelocity_026814(pc) | +006
        jsr     Entity_MoveX_WallStop_026836(pc) | +00a
        move.w  -0x1148(a5),d1                  | +00e
        move.w  -0x1146(a5),d2                  | +012
        add.w   -0x1154(a5),d1                  | +016
        add.w   -0x1152(a5),d2                  | +01a
        subq.w  #0x1,d2                         | +01e
        jsr     Sub_00027DB2(pc)                | +020
        jsr     0x9993c.l                       | +024
        move.b  d6,0x106f44.l                   | +02a
        cmpi.b  #0xf,d6                         | +030
        beq.w   .L027786                        | +034
        moveq   #0,d0                           | +038
        moveq   #0,d3                           | +03a
        move.w  d5,d4                           | +03c
        moveq   #17,d0                          | +03e
        lea     0x278ba8.l,a1                   | +040
        bra.w   .L027786                        | +046
.L027786:
        cmpi.b  #0x0,d0                         | +04a
        beq.w   .L0277c0                        | +04e
        jsr     TaskHandler_027e62(pc)          | +052
        bcc.w   .L0277c0                        | +056
        tst.w   0x106f2e.l                      | +05a
        beq.w   .L0277aa                        | +060
        tst.w   0x106f2e.l                      | +064
        bra.w   .L0277ae                        | +06a
.L0277aa:
        tst.w   0x2a(a6)                        | +06e
.L0277ae:
        bge.w   .L0277c0                        | +072
        cmpi.b  #0x0,0x9(a1)                    | +076
        bne.w   .L0277c0                        | +07c
        jmp     TaskHandler_0275b8(pc)          | +080
.L0277c0:
        jmp     TaskHandler_0274b8__L02759a(pc) | +084

| ----------------------------------------------------------------------------
|  Sub_000277C4  @ $0277C4  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000277C4, "ax", @progbits
        .global Sub_000277C4
Sub_000277C4:
        jsr     0x44182.l                       | +000
        jsr     Entity_IntegrateVelocity_026814(pc) | +006
        jsr     Entity_MoveX_WallStop_026836(pc) | +00a
        move.w  -0x1148(a5),d1                  | +00e
        move.w  -0x1146(a5),d2                  | +012
        add.w   -0x1154(a5),d1                  | +016
        add.w   -0x1152(a5),d2                  | +01a
        subq.w  #0x1,d2                         | +01e
        jsr     Sub_00027DB2(pc)                | +020
        jsr     0x9993c.l                       | +024
        move.b  d6,0x106f44.l                   | +02a
        cmpi.b  #0xf,d6                         | +030
        beq.w   .L02780e                        | +034
        moveq   #0,d0                           | +038
        moveq   #0,d3                           | +03a
        move.w  d5,d4                           | +03c
        moveq   #17,d0                          | +03e
        lea     0x278ba8.l,a1                   | +040
        bra.w   .L02780e                        | +046
.L02780e:
        cmpi.b  #0x0,d0                         | +04a
        beq.w   TaskHandler_027836              | +04e
        jsr     TaskHandler_027e62(pc)          | +052
        bcc.w   TaskHandler_027836              | +056
        cmpi.b  #0x0,0x9(a1)                    | +05a
        bne.w   TaskHandler_027836              | +060
        jsr     TaskHandler_0274b8__L02759a(pc) | +064
        movea.l 0x4(a1),a3                      | +068

| ----------------------------------------------------------------------------
|  TaskHandler_027836  @ $027836  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027836, "ax", @progbits
        .global TaskHandler_027836
TaskHandler_027836:
        jmp     TaskHandler_0274b8__L02759a(pc) | +000

| ----------------------------------------------------------------------------
|  TaskHandler_027902  @ $027902  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027902, "ax", @progbits
        .global TaskHandler_027902
TaskHandler_027902:
        movea.l #0x108080,a5                    | +000
        move.w  0x22(a6),-0x1148(a5)            | +006
        move.w  0x24(a6),-0x1146(a5)            | +00c
        move.b  0x26(a6),-0x1144(a5)            | +012
        move.b  0x27(a6),-0x1143(a5)            | +018
        jsr     Entity_MoveAndCollide_B_027036(pc) | +01e
        bcs.w   TaskHandler_02794a              | +022
        move.w  -0x1148(a5),0x22(a6)            | +026
        move.w  -0x1146(a5),0x24(a6)            | +02c
        move.b  -0x1144(a5),0x26(a6)            | +032
        move.b  -0x1143(a5),0x27(a6)            | +038
        jsr     Entity_ApplyFadeShade_028108(pc) | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_02794a  @ $02794A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02794a, "ax", @progbits
        .global TaskHandler_02794a
TaskHandler_02794a:
        move.w  -0x1148(a5),0x22(a6)            | +000
        move.w  -0x1146(a5),0x24(a6)            | +006
        move.b  -0x1144(a5),0x26(a6)            | +00c
        move.b  -0x1143(a5),0x27(a6)            | +012
        jsr     Entity_ApplyFadeShade_028108(pc) | +018

| ----------------------------------------------------------------------------
|  TaskHandler_02796c  @ $02796C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02796c, "ax", @progbits
        .global TaskHandler_02796c
TaskHandler_02796c:
        movea.l #0x108080,a5                    | +000
        move.w  0x22(a6),-0x1148(a5)            | +006
        move.w  0x82(a6),-0x1146(a5)            | +00c
        move.b  0x26(a6),-0x1144(a5)            | +012
        move.b  0x27(a6),-0x1143(a5)            | +018
        jsr     Entity_MoveAndCollide_C_0272a8(pc) | +01e
        bcc.w   TaskHandler_0279ba              | +022
        move.w  -0x1148(a5),0x22(a6)            | +026
        move.w  -0x1146(a5),0x82(a6)            | +02c
        move.b  -0x1144(a5),0x26(a6)            | +032
        move.b  -0x1143(a5),0x27(a6)            | +038
        andi.w  #0x3ff,0x82(a6)                 | +03e
        jsr     Entity_ApplyFadeShade_028108(pc) | +044

| ----------------------------------------------------------------------------
|  TaskHandler_0279ba  @ $0279BA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0279ba, "ax", @progbits
        .global TaskHandler_0279ba
TaskHandler_0279ba:
        move.w  -0x1148(a5),0x22(a6)            | +000
        move.b  -0x1144(a5),0x26(a6)            | +006
        move.w  -0x1146(a5),0x82(a6)            | +00c
        move.b  -0x1143(a5),0x27(a6)            | +012
        andi.w  #0x3ff,0x82(a6)                 | +018
        jsr     Entity_ApplyFadeShade_028108(pc) | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_0279e2  @ $0279E2  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0279e2, "ax", @progbits
        .global TaskHandler_0279e2
TaskHandler_0279e2:
        movea.l #0x108080,a5                    | +000
        move.w  #0xffff,0x34(a6)                | +006
        move.w  #0xffff,0x64(a6)                | +00c
        move.w  0x22(a6),-0x1148(a5)            | +012
        move.w  0x82(a6),-0x1146(a5)            | +018
        move.b  0x26(a6),-0x1144(a5)            | +01e
        move.b  0x27(a6),-0x1143(a5)            | +024
        jsr     TaskHandler_0274b8__L0274cc(pc) | +02a
        bcc.w   TaskHandler_027a6e              | +02e
        bra.w   .L027a3e                        | +032
        movea.l #0x108080,a5                    | +036
        move.w  0x22(a6),-0x1148(a5)            | +03c
        move.w  0x82(a6),-0x1146(a5)            | +042
        move.b  0x26(a6),-0x1144(a5)            | +048
        move.b  0x27(a6),-0x1143(a5)            | +04e
        jsr     Sub_000027444(pc)               | +054
        bcc.w   TaskHandler_027a6e              | +058
.L027a3e:
        move.w  #0x0,0x2e(a6)                   | +05c
        move.w  #0x0,0x2a(a6)                   | +062
        move.w  -0x1148(a5),0x22(a6)            | +068
        move.w  -0x1146(a5),0x82(a6)            | +06e
        move.b  -0x1144(a5),0x26(a6)            | +074
        move.b  -0x1143(a5),0x27(a6)            | +07a
        andi.w  #0x3ff,0x82(a6)                 | +080

| ----------------------------------------------------------------------------
|  TaskHandler_027a6e  @ $027A6E  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027a6e, "ax", @progbits
        .global TaskHandler_027a6e
TaskHandler_027a6e:
        move.w  -0x1148(a5),0x22(a6)            | +000
        move.w  -0x1146(a5),0x82(a6)            | +006
        move.b  -0x1144(a5),0x26(a6)            | +00c
        move.b  -0x1143(a5),0x27(a6)            | +012
        andi.w  #0x3ff,0x82(a6)                 | +018

| ----------------------------------------------------------------------------
|  TaskHandler_027b66  @ $027B66  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027b66, "ax", @progbits
        .global TaskHandler_027b66
TaskHandler_027b66:
        movea.l #0x108080,a5                    | +000
        move.w  0x22(a6),-0x1148(a5)            | +006
        move.w  0x24(a6),-0x1146(a5)            | +00c
        move.b  0x26(a6),-0x1144(a5)            | +012
        move.b  0x27(a6),-0x1143(a5)            | +018
        jsr     TaskHandler_027464(pc)          | +01e
        bcs.w   TaskHandler_027baa              | +022
        move.w  -0x1148(a5),0x22(a6)            | +026
        move.w  -0x1146(a5),0x24(a6)            | +02c
        move.b  -0x1144(a5),0x26(a6)            | +032
        move.b  -0x1143(a5),0x27(a6)            | +038

| ----------------------------------------------------------------------------
|  TaskHandler_027baa  @ $027BAA  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027baa, "ax", @progbits
        .global TaskHandler_027baa
TaskHandler_027baa:
        move.w  -0x1148(a5),0x22(a6)            | +000
        move.w  -0x1146(a5),0x24(a6)            | +006
        move.b  -0x1144(a5),0x26(a6)            | +00c
        move.b  -0x1143(a5),0x27(a6)            | +012

| ----------------------------------------------------------------------------
|  Sub_00027DB2  @ $027DB2  (118 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00027DB2, "ax", @progbits
        .global Sub_00027DB2
Sub_00027DB2:
        move.w  d1,d5                           | +000
        move.w  d2,d6                           | +002
        move.w  d1,d0                           | +004
        move.w  d2,d1                           | +006
        cmpi.w  #0x1ff,d1                       | +008
        bgt.w   .L027dcc                        | +00c
        jsr     0x43f02.l                       | +010
        bra.w   .L027dce                        | +016
.L027dcc:
        moveq   #0,d0                           | +01a
.L027dce:
        move.b  0x6b(a6),d2                     | +01c
.L027dd2:
        btst    #0x2,0x5b(a6)                   | +020
        beq.w   .L027dee                        | +026
        move.b  d0,d7                           | +02a
        andi.b  #0xc0,d7                        | +02c
        cmpi.b  #0xc0,d7                        | +030
        bne.w   .L027dee                        | +034
        move.b  #0x0,d0                         | +038
.L027dee:
        move.b  d0,d7                           | +03c
        andi.b  #0x3f,d0                        | +03e
        lea     0x278988.l,a1                   | +042
        move.w  d0,d1                           | +048
        andi.l  #0x3f,d1                        | +04a
        asl.l   #0x5,d1                         | +050
        adda.l  d1,a1                           | +052
        and.b   0xc(a1),d2                      | +054
        beq.w   .L027e22                        | +058
        moveq   #0,d2                           | +05c
        moveq   #0,d0                           | +05e
        move.b  0xd(a1),d1                      | +060
        cmpi.b  #0xff,d1                        | +064
        beq.w   .L027e22                        | +068
        move.b  d1,d0                           | +06c
        bra.b   .L027dd2                        | +06e
.L027e22:
        move.w  d5,d1                           | +070
        move.w  d6,d2                           | +072
        rts                                     | +074

| ----------------------------------------------------------------------------
|  Sub_00027E28  @ $027E28  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00027E28, "ax", @progbits
        .global Sub_00027E28
Sub_00027E28:
        movea.l (a1),a0                         | +000
        move.b  (a0,d4.w),d5                    | +002
        moveq   #7,d6                           | +006
        sub.w   d3,d6                           | +008
        btst    d6,d5                           | +00a
        beq.w   .L027e3c                        | +00c
        bra.w   SetXN_027e56                    | +010
.L027e3c:
        movea.l 0x10(a1),a0                     | +014
        move.b  (a0,d4.w),d5                    | +018
        move.w  #0x7,d6                         | +01c
        sub.w   d3,d6                           | +020
        btst    d6,d5                           | +022
        beq.w   ClearXN_027e5c                  | +024
        adda.l  #0x10,a1                        | +028

| ----------------------------------------------------------------------------
|  TaskHandler_027e62  @ $027E62  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027e62, "ax", @progbits
        .global TaskHandler_027e62
TaskHandler_027e62:
        movea.l (a1),a0                         | +000
        move.b  (a0,d4.w),d5                    | +002
        moveq   #7,d6                           | +006
        sub.w   d3,d6                           | +008
        btst    d6,d5                           | +00a
        beq.w   ClearXN_027e78                  | +00c

| ----------------------------------------------------------------------------
|  Sub_00027E7E  @ $027E7E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00027E7E, "ax", @progbits
        .global Sub_00027E7E
Sub_00027E7E:
        movea.l (a1),a0                         | +000
        move.b  (a0,d4.w),d0                    | +002
        move.w  #0x7,d6                         | +006
        sub.w   d3,d6                           | +00a
        btst    d6,d0                           | +00c
        beq.w   ClearXN_027e96                  | +00e

| ----------------------------------------------------------------------------
|  Sub_00027E9C  @ $027E9C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00027E9C, "ax", @progbits
        .global Sub_00027E9C
Sub_00027E9C:
        movea.l (a1),a0                         | +000
        move.b  0x8(a0,d4.w),d0                 | +002
        move.w  #0x7,d6                         | +006
        sub.w   d3,d6                           | +00a
        btst    d6,d0                           | +00c
        beq.w   ClearXN_027eb4                  | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_027f08  @ $027F08  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027f08, "ax", @progbits
        .global TaskHandler_027f08
TaskHandler_027f08:
        jsr     ActorCtxWrapper_02783a(pc)      | +000
        move.w  0x22(a6),d1                     | +004
        move.w  0x24(a6),d2                     | +008
        subq.w  #0x1,d2                         | +00c
        jsr     Sub_00027DB2(pc)                | +00e
        jsr     0x9993c.l                       | +012
        move.b  d6,0x106f44.l                   | +018
        cmpi.b  #0xf,d6                         | +01e
        beq.w   .L027f40                        | +022
        moveq   #0,d0                           | +026
        moveq   #0,d3                           | +028
        move.w  d5,d4                           | +02a
        moveq   #17,d0                          | +02c
        lea     0x278ba8.l,a1                   | +02e
        bra.w   .L027f40                        | +034
.L027f40:
        jsr     Sub_00027E28(pc)                | +038
        bcc.w   TaskHandler_027f58              | +03c
        bset    #0x6,0x5a(a6)                   | +040
        move.b  #0xff,d3                        | +046

| ----------------------------------------------------------------------------
|  TaskHandler_027f58  @ $027F58  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027f58, "ax", @progbits
        .global TaskHandler_027f58
TaskHandler_027f58:
        clr.b   d3                              | +000

| ----------------------------------------------------------------------------
|  TaskHandler_027f60  @ $027F60  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027f60, "ax", @progbits
        .global TaskHandler_027f60
TaskHandler_027f60:
        move.w  0x28(a6),-(a7)                  | +000
        move.w  0x2a(a6),-(a7)                  | +004
        move.w  0x22(a6),-(a7)                  | +008
        move.w  0x24(a6),-(a7)                  | +00c
        addi.w  #0x1,0x24(a6)                   | +010
        clr.w   0x28(a6)                        | +016
        move.w  #0xff00,0x2a(a6)                | +01a
        jsr     Entity_ProbeTransformFreeCcr_027bc8(pc) | +020
        bcs.w   .L027f92                        | +024
        clr.b   d3                              | +028
        ori.b   #0x11,ccr                       | +02a
        bra.w   .L027f9a                        | +02e
.L027f92:
        move.b  #0xff,d3                        | +032
        andi.b  #0xee,ccr                       | +036
.L027f9a:
        move.w  (a7)+,0x24(a6)                  | +03a
        move.w  (a7)+,0x22(a6)                  | +03e
        move.w  (a7)+,0x2a(a6)                  | +042
        move.w  (a7)+,0x28(a6)                  | +046
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_027fac  @ $027FAC  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027fac, "ax", @progbits
        .global TaskHandler_027fac
TaskHandler_027fac:
        move.w  0x22(a6),d1                     | +000
        move.w  0x24(a6),d2                     | +004
        subq.w  #0x1,d2                         | +008
        jsr     Sub_00027DB2(pc)                | +00a
        jsr     TaskHandler_027e62(pc)          | +00e
        bcc.w   SetXN_027fd2                    | +012
        cmpi.b  #0x0,0x9(a1)                    | +016
        bne.w   SetXN_027fd2                    | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_027fd8  @ $027FD8  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_027fd8, "ax", @progbits
        .global TaskHandler_027fd8
TaskHandler_027fd8:
        jsr     ActorCtxWrapper_02783a(pc)      | +000
        move.w  0x22(a6),d1                     | +004
        move.w  0x24(a6),d2                     | +008
        subq.w  #0x1,d2                         | +00c
        jsr     Sub_00027DB2(pc)                | +00e
        jsr     TaskHandler_027e62(pc)          | +012
        bcc.w   TaskHandler_028006              | +016
        cmpi.b  #0x0,0x9(a1)                    | +01a
        bne.w   TaskHandler_028006              | +020
        move.b  #0xff,d3                        | +024

| ----------------------------------------------------------------------------
|  TaskHandler_028006  @ $028006  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028006, "ax", @progbits
        .global TaskHandler_028006
TaskHandler_028006:
        clr.b   d3                              | +000

| ----------------------------------------------------------------------------
|  Sub_0002800E  @ $02800E  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0002800E, "ax", @progbits
        .global Sub_0002800E
Sub_0002800E:
        cmpi.w  #0x400,0x34(a6)                 | +000
        beq.w   .L028024                        | +006
        cmpi.w  #0xfc00,0x34(a6)                | +00a
        beq.w   .L028024                        | +010
.L028022:
        rts                                     | +014
.L028024:
        cmpi.b  #0x1,0x106ece.l                 | +016
        bne.w   .L028038                        | +01e
        tst.w   0x106f5e.l                      | +022
        bne.b   .L028022                        | +028
.L028038:
        move.w  d2,d0                           | +02a
        muls.w  #0xaa,d0                        | +02c
        asr.l   #0x8,d0                         | +030
        move.w  d0,d2                           | +032
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_028044  @ $028044  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028044, "ax", @progbits
        .global TaskHandler_028044
TaskHandler_028044:
        tst.w   d1                              | +000
        bra.w   ClearXN_02806e                  | +002

| ----------------------------------------------------------------------------
|  TaskHandler_02804a  @ $02804A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02804a, "ax", @progbits
        .global TaskHandler_02804a
TaskHandler_02804a:
        tst.w   d1                              | +000
        ble.w   ClearXN_02806e                  | +002
        bra.w   SetXN_028068                    | +006

| ----------------------------------------------------------------------------
|  TaskHandler_028054  @ $028054  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028054, "ax", @progbits
        .global TaskHandler_028054
TaskHandler_028054:
        tst.w   d1                              | +000
        bge.w   ClearXN_02806e                  | +002
        bra.w   SetXN_028068                    | +006

| ----------------------------------------------------------------------------
|  TaskHandler_02805e  @ $02805E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02805e, "ax", @progbits
        .global TaskHandler_02805e
TaskHandler_02805e:
        tst.w   d1                              | +000
        beq.w   ClearXN_02806e                  | +002
        bra.w   SetXN_028068                    | +006

| ----------------------------------------------------------------------------
|  Sub_00028074  @ $028074  (62 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00028074, "ax", @progbits
        .global Sub_00028074
Sub_00028074:
        cmpi.b  #0x9,0x106ece.l                 | +000
        beq.w   .L0280ac                        | +008
        move.w  #0x1e,d1                        | +00c
        sub.w   0x22(a6),d1                     | +010
        ble.w   .L028096                        | +014
        tst.w   d2                              | +018
        blt.w   TaskHandler_0280b8              | +01a
        bra.w   TaskHandler_0280b8__L0280ba     | +01e
.L028096:
        move.w  #0x117,d1                       | +022
        sub.w   0x22(a6),d1                     | +026
        bge.w   .L0280ac                        | +02a
        tst.w   d2                              | +02e
        bgt.w   TaskHandler_0280b8              | +030
        bra.w   TaskHandler_0280b8__L0280ba     | +034
.L0280ac:
        bset    #0x7,0x13(a6)                   | +038

| ----------------------------------------------------------------------------
|  TaskHandler_0280b8  @ $0280B8  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0280b8, "ax", @progbits
        .global TaskHandler_0280b8
TaskHandler_0280b8:
        moveq   #0,d2                           | +000
        .global TaskHandler_0280b8__L0280ba
TaskHandler_0280b8__L0280ba:
.L0280ba:
        bclr    #0x7,0x13(a6)                   | +002

| ----------------------------------------------------------------------------
|  Fn_000280C6  @ $0280C6  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Fn_000280C6, "ax", @progbits
        .global Fn_000280C6
Fn_000280C6:
        jsr     Sub_00027DB2(pc)                | +000
        jsr     0x9993c.l                       | +004
        move.b  d6,0x106f44.l                   | +00a
        cmpi.b  #0xf,d6                         | +010
        beq.w   .L0280f0                        | +014
        moveq   #0,d0                           | +018
        moveq   #0,d3                           | +01a
        move.w  d5,d4                           | +01c
        moveq   #17,d0                          | +01e
        lea     0x278ba8.l,a1                   | +020
        bra.w   .L0280f0                        | +026
.L0280f0:
        andi.b  #0x3f,d0                        | +02a
        cmpi.b  #0x0,d0                         | +02e
        beq.w   ClearXN_028102                  | +032

| ----------------------------------------------------------------------------
|  TaskHandler_02813c  @ $02813C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02813c, "ax", @progbits
        .global TaskHandler_02813c
TaskHandler_02813c:
        andi.w  #0xffe3,d0                      | +000
        ori.w   #0x0,d0                         | +004
        move.w  d0,d5                           | +008
        clr.w   d0                              | +00a
        jsr     0x440d0.l                       | +00c
        move.w  d1,d6                           | +012
        move.w  d5,0x38(a6)                     | +014
        jmp     Entity_ApplyFadeShade_028108__L028110(pc) | +018

| ----------------------------------------------------------------------------
|  TaskHandler_028158  @ $028158  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028158, "ax", @progbits
        .global TaskHandler_028158
TaskHandler_028158:
        clr.w   d0                              | +000
        move.w  0x34(a6),d7                     | +002
        cmpi.w  #0x400,d7                       | +006
        beq.w   .L028184                        | +00a
        cmpi.w  #0xfc00,d7                      | +00e
        beq.w   .L02818a                        | +012
        cmpi.w  #0x1000,d7                      | +016
        beq.w   .L028190                        | +01a
        cmpi.w  #0xf000,d7                      | +01e
        beq.w   .L028196                        | +022
        bra.w   .L028182                        | +026
.L028182:
        rts                                     | +02a
.L028184:
        move.w  #0x3,d0                         | +02c
        bra.b   .L028182                        | +030
.L02818a:
        move.w  #0x4,d0                         | +032
        bra.b   .L028182                        | +036
.L028190:
        move.w  #0x1,d0                         | +038
        bra.b   .L028182                        | +03c
.L028196:
        move.w  #0x2,d0                         | +03e
        bra.b   .L028182                        | +042

| ----------------------------------------------------------------------------
|  TaskHandler_02819c  @ $02819C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02819c, "ax", @progbits
        .global TaskHandler_02819c
TaskHandler_02819c:
        movea.l #0x108080,a5                    | +000
        move.w  #0xffff,-0x1142(a5)             | +006
        move.w  #0xffff,-0x1140(a5)             | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_0281b0  @ $0281B0  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0281b0, "ax", @progbits
        .global TaskHandler_0281b0
TaskHandler_0281b0:
        movea.l #0x108080,a5                    | +000
        move.w  d0,-0x1142(a5)                  | +006
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_0281bc  @ $0281BC  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0281bc, "ax", @progbits
        .global TaskHandler_0281bc
TaskHandler_0281bc:
        movea.l #0x108080,a5                    | +000
        move.w  d0,-0x1140(a5)                  | +006
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  PcThunkTarget_0281c8  @ $0281C8  (100 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_0281c8, "ax", @progbits
        .global PcThunkTarget_0281c8
PcThunkTarget_0281c8:
        movea.l #0x108080,a5                    | +000
        tst.w   -0x1142(a5)                     | +006
        beq.w   .L0281fc                        | +00a
        move.w  0x28(a6),d1                     | +00e
        clr.w   d2                              | +012
        tst.w   d1                              | +014
        beq.w   .L0281e6                        | +016
        move.w  #0x1,d2                         | +01a
.L0281e6:
        ext.l   d1                              | +01e
        swap    d1                              | +020
        or.w    d2,d1                           | +022
        cmp.w   -0x1142(a5),d1                  | +024
        bne.w   .L0281fc                        | +028
        clr.w   -0x1154(a5)                     | +02c
        clr.b   -0x1150(a5)                     | +030
.L0281fc:
        tst.w   -0x1140(a5)                     | +034
        beq.w   .L028222                        | +038
        move.w  0x2a(a6),d1                     | +03c
        clr.w   d2                              | +040
        tst.w   d1                              | +042
        beq.w   .L028214                        | +044
        move.w  #0x1,d2                         | +048
.L028214:
        ext.l   d1                              | +04c
        swap    d1                              | +04e
        or.w    d2,d1                           | +050
        cmp.w   -0x1140(a5),d1                  | +052
        bne.w   .L028222                        | +056
.L028222:
        clr.w   -0x1142(a5)                     | +05a
        clr.w   -0x1140(a5)                     | +05e
        rts                                     | +062

| ----------------------------------------------------------------------------
|  TaskHandler_02822c  @ $02822C  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02822c, "ax", @progbits
        .global TaskHandler_02822c
TaskHandler_02822c:
        movem.w d5-d7,-(a7)                     | +000
        move.w  0x28(a6),d5                     | +004
        move.w  0x2a(a6),d6                     | +008
        move.w  (a3),d7                         | +00c
        bne.w   .L028248                        | +00e
        tst.w   d6                              | +012
        bgt.w   TaskHandler_028288              | +014
        bra.w   .L02827e                        | +018
.L028248:
        cmpi.w  #0xffff,d7                      | +01c
        beq.w   TaskHandler_028288              | +020
        tst.w   d7                              | +024
        bmi.w   .L02826c                        | +026
        cmpi.w  #0x1000,d7                      | +02a
        bne.w   .L028262                        | +02e
        add.w   d6,d6                           | +032
        add.w   d6,d6                           | +034
.L028262:
        sub.w   d5,d6                           | +036
        bgt.w   TaskHandler_028288              | +038
        bra.w   .L02827e                        | +03c
.L02826c:
        cmpi.w  #0xf000,d7                      | +040
        bne.w   .L028278                        | +044
        add.w   d6,d6                           | +048
        add.w   d6,d6                           | +04a
.L028278:
        add.w   d5,d6                           | +04c
        bgt.w   TaskHandler_028288              | +04e
.L02827e:
        movem.w (a7)+,d5-d7                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_028288  @ $028288  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028288, "ax", @progbits
        .global TaskHandler_028288
TaskHandler_028288:
        movem.w (a7)+,d5-d7                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0282d8  @ $0282D8  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0282d8, "ax", @progbits
        .global TaskHandler_0282d8
TaskHandler_0282d8:
        cmpi.w  #0xffff,0x60(a6)                | +000
        beq.w   .L0282ea                        | +006
        move.l  #0xffffffff,0x60(a6)            | +00a
.L0282ea:
        move.w  0x34(a6),-(a7)                  | +012
        move.w  0x62(a6),0x34(a6)               | +016
        jsr     Entity_ProbeRevertCcr_027AFC(pc) | +01c
        bcc.w   .L02830e                        | +020
        move.w  0x34(a6),0x62(a6)               | +024
        move.w  (a7)+,0x34(a6)                  | +02a
        ori.b   #0x11,ccr                       | +02e
        bra.w   ClearXNMid_02831c               | +032
.L02830e:
        move.w  0x34(a6),0x62(a6)               | +036
        move.w  (a7)+,0x34(a6)                  | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_02831e  @ $02831E  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02831e, "ax", @progbits
        .global TaskHandler_02831e
TaskHandler_02831e:
        cmpi.w  #0xffff,0x60(a6)                | +000
        beq.w   .L028330                        | +006
        move.l  #0xffffffff,0x60(a6)            | +00a
.L028330:
        move.w  0x34(a6),-(a7)                  | +012
        move.w  0x62(a6),0x34(a6)               | +016
        jsr     Entity_ProbeTransformFreeCcr_027bc8(pc) | +01c
        bcc.w   .L028354                        | +020
        move.w  0x34(a6),0x62(a6)               | +024
        move.w  (a7)+,0x34(a6)                  | +02a
        ori.b   #0x11,ccr                       | +02e
        bra.w   ClearXNMid_028362               | +032
.L028354:
        move.w  0x34(a6),0x62(a6)               | +036
        move.w  (a7)+,0x34(a6)                  | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_028364  @ $028364  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028364, "ax", @progbits
        .global TaskHandler_028364
TaskHandler_028364:
        cmpi.w  #0xffff,0x60(a6)                | +000
        beq.w   .L028376                        | +006
        move.l  #0xffffffff,0x60(a6)            | +00a
.L028376:
        move.w  0x34(a6),-(a7)                  | +012
        move.w  0x62(a6),0x34(a6)               | +016
        jsr     Entity_ProbeTransformFreeCcr_027c8c(pc) | +01c
        bcc.w   .L02839a                        | +020
        move.w  0x34(a6),0x62(a6)               | +024
        move.w  (a7)+,0x34(a6)                  | +02a
        ori.b   #0x11,ccr                       | +02e
        bra.w   ClearXNMid_0283a8               | +032
.L02839a:
        move.w  0x34(a6),0x62(a6)               | +036
        move.w  (a7)+,0x34(a6)                  | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_0283aa  @ $0283AA  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0283aa, "ax", @progbits
        .global TaskHandler_0283aa
TaskHandler_0283aa:
        clr.b   d3                              | +000
        rts                                     | +002

| ----------------------------------------------------------------------------
|  TaskHandler_0283ae  @ $0283AE  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0283ae, "ax", @progbits
        .global TaskHandler_0283ae
TaskHandler_0283ae:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_0283c4                    | +00c

| ----------------------------------------------------------------------------
|  Sub_0002_83EC  @ $0283EC  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0002_83EC, "ax", @progbits
        .global Sub_0002_83EC
Sub_0002_83EC:
        move.b  (a0),d0                         | +000
        cmpi.b  #0xff,d0                        | +002
        bne.w   .L0283fe                        | +006
        suba.l  #0x50,a0                        | +00a
        rts                                     | +010
.L0283fe:
        cmpi.b  #0x5,d0                         | +012
        bcs.w   .L028412                        | +016
        nop                                     | +01a
        nop                                     | +01c
        cmpi.b  #0x5,d0                         | +01e
        nop                                     | +022
        trap    #0xf                            | +024
.L028412:
        ext.w   d0                              | +026
        lsl.w   #0x2,d0                         | +028
        lea     Data_028428(pc),a4              | +02a
        movea.l (a4,d0.w),a5                    | +02e
        jsr     (a5)                            | +032
        adda.l  #0x50,a0                        | +034
        bra.b   Sub_0002_83EC                   | +03a

| ----------------------------------------------------------------------------
|  Data_028428  @ $028428  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Data_028428, "ax", @progbits
        .global Data_028428
Data_028428:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x843c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x845e                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x8490                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x84c2                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x8518                        | +012  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_02843c  @ $02843C  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02843c, "ax", @progbits
        .global TaskHandler_02843c
TaskHandler_02843c:
        lea     0x100440.l,a1                   | +000
        jsr     Sub_00028576(pc)                | +006
        lea     0x1004e0.l,a1                   | +00a
        jsr     Sub_00028576(pc)                | +010
        lea     0x100580.l,a1                   | +014
        jsr     Sub_00028576(pc)                | +01a
        moveq   #-1,d7                          | +01e
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TaskHandler_02845e  @ $02845E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02845e, "ax", @progbits
        .global TaskHandler_02845e
TaskHandler_02845e:
        lea     0x100440.l,a1                   | +000
        jsr     Sub_00028576(pc)                | +006
        btst    #0x1,0x5a(a6)                   | +00a
        bne.w   .L028482                        | +010
        lea     0x100580.l,a1                   | +014
        jsr     Sub_00028576(pc)                | +01a
        moveq   #-1,d7                          | +01e
        bra.w   .L02848e                        | +020
.L028482:
        lea     0x100580.l,a1                   | +024
        jsr     Sub_00028576(pc)                | +02a
        moveq   #0,d7                           | +02e
.L02848e:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  TaskHandler_028490  @ $028490  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028490, "ax", @progbits
        .global TaskHandler_028490
TaskHandler_028490:
        lea     0x1004e0.l,a1                   | +000
        jsr     Sub_00028576(pc)                | +006
        btst    #0x1,0x5a(a6)                   | +00a
        bne.w   .L0284b4                        | +010
        lea     0x100580.l,a1                   | +014
        jsr     Sub_00028576(pc)                | +01a
        moveq   #-1,d7                          | +01e
        bra.w   .L0284c0                        | +020
.L0284b4:
        lea     0x100580.l,a1                   | +024
        jsr     Sub_00028576(pc)                | +02a
        moveq   #1,d7                           | +02e
.L0284c0:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  TaskHandler_0284c2  @ $0284C2  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0284c2, "ax", @progbits
        .global TaskHandler_0284c2
TaskHandler_0284c2:
        movea.l #0xffffffff,a4                  | +000
        jsr     TaskHandler_028926__L028930(pc) | +006
        bcs.w   .L028514                        | +00a
        btst    #0x1,0x13(a6)                   | +00e
        bne.w   .L028514                        | +014
        lea     0x100800.l,a1                   | +018
.L0284e0:
        movea.l 0x8(a1),a2                      | +01e
        move.b  0x100810.l,d0                   | +022
        cmp.b   0x10(a2),d0                     | +028
        bcc.w   .L028514                        | +02c
        movea.l a2,a1                           | +030
        movea.l 0x48(a1),a2                     | +032
        cmpa.l  #0xffffffff,a2                  | +036
        beq.b   .L0284e0                        | +03c
        cmpa.l  a6,a1                           | +03e
        beq.b   .L0284e0                        | +040
        jsr     Sub_00028594(pc)                | +042
        btst    #0x1,0x13(a6)                   | +046
        bne.w   .L028514                        | +04c
        bra.b   .L0284e0                        | +050
.L028514:
        moveq   #-1,d7                          | +052
        rts                                     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_028518  @ $028518  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028518, "ax", @progbits
        .global TaskHandler_028518
TaskHandler_028518:
        movea.l #0xffffffff,a4                  | +000
        jsr     TaskHandler_028926__L028930(pc) | +006
        bcs.w   .L02856a                        | +00a
        btst    #0x1,0x13(a6)                   | +00e
        bne.w   .L02856a                        | +014
        lea     0x1008a0.l,a1                   | +018
.L028536:
        movea.l 0x8(a1),a2                      | +01e
        move.b  0x1008b0.l,d0                   | +022
        cmp.b   0x10(a2),d0                     | +028
        bcc.w   .L02856a                        | +02c
        movea.l a2,a1                           | +030
        movea.l 0x48(a1),a2                     | +032
        cmpa.l  #0xffffffff,a2                  | +036
        beq.b   .L028536                        | +03c
        cmpa.l  a6,a1                           | +03e
        beq.b   .L028536                        | +040
        jsr     Sub_00028594(pc)                | +042
        btst    #0x1,0x13(a6)                   | +046
        bne.w   .L02856a                        | +04c
        bra.b   .L028536                        | +050
.L02856a:
        moveq   #-1,d7                          | +052
        rts                                     | +054

| ----------------------------------------------------------------------------
|  Data_02856e  @ $02856E  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Data_02856e, "ax", @progbits
        .global Data_02856e
Data_02856e:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x8886                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x876e                        | +006  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Sub_00028576  @ $028576  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00028576, "ax", @progbits
        .global Sub_00028576
Sub_00028576:
        cmpa.l  a6,a1                           | +000
        beq.w   Sub_00028594__L028686           | +002
        movea.l 0x48(a1),a2                     | +006
        cmpa.l  #0xffffffff,a2                  | +00a
        beq.w   Sub_00028594__L028686           | +010
        btst    #0x1,0x13(a6)                   | +014
        bne.w   Sub_00028594__L028686           | +01a

| ----------------------------------------------------------------------------
|  Sub_00028594  @ $028594  (246 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00028594, "ax", @progbits
        .global Sub_00028594
Sub_00028594:
        cmpi.l  #0xffffffff,(a2)                | +000
        beq.w   .L028686                        | +006
        btst    #0x3,0x13(a1)                   | +00a
        bne.w   .L028686                        | +010
        lea     Data_02856e(pc),a4              | +014
        lea     0x27928c.l,a5                   | +018
        cmpi.l  #0xffffffff,0x2(a2)             | +01e
        beq.w   .L0285c2                        | +026
        movea.l 0x2(a2),a5                      | +02a
.L0285c2:
        adda.l  #0xa,a2                         | +02e
        move.b  0x1(a0),d0                      | +034
        andi.w  #0xff,d0                        | +038
        move.b  (a5,d0.w),d1                    | +03c
        cmpi.b  #0x1,d1                         | +040
        beq.w   .L02860e                        | +044
        move.b  0x5(a0),d0                      | +048
        cmpi.b  #0x8,d0                         | +04c
        bcs.w   .L0285f4                        | +050
        nop                                     | +054
        nop                                     | +056
        cmpi.b  #0x8,d0                         | +058
        nop                                     | +05c
        trap    #0xf                            | +05e
.L0285f4:
        andi.w  #0xff,d0                        | +060
        movea.l (a4,d0.w),a3                    | +064
        adda.l  #0xa,a0                         | +068
        jsr     (a3)                            | +06e
        bcs.w   .L028618                        | +070
        suba.l  #0xa,a0                         | +074
.L02860e:
        adda.l  #0x46,a2                        | +07a
        bra.w   Sub_00028594                    | +080
.L028618:
        suba.l  #0xa,a0                         | +084
        suba.l  #0xa,a2                         | +08a
        tst.b   0x45(a1)                        | +090
        bne.w   TaskHandler_0286d4              | +094
        move.b  0x1(a0),d0                      | +098
        move.b  d0,0x58(a1)                     | +09c
        move.l  a6,0x50(a1)                     | +0a0
        move.b  0x4(a0),d0                      | +0a4
        cmpi.b  #0x0,d0                         | +0a8
        beq.w   TaskHandler_0286d4              | +0ac
        move.b  0x1(a0),d0                      | +0b0
        cmpi.b  #0x22,d0                        | +0b4
        bls.w   .L02865c                        | +0b8
        nop                                     | +0bc
        nop                                     | +0be
        cmpi.b  #0x22,d0                        | +0c0
        nop                                     | +0c4
        trap    #0xf                            | +0c6
.L02865c:
        cmpi.b  #0x0,d0                         | +0c8
        beq.w   TaskHandler_0286e4              | +0cc
        btst    #0x0,0x6b(a1)                   | +0d0
        beq.w   .L028676                        | +0d6
        cmpi.b  #0xb,d0                         | +0da
        beq.w   TaskHandler_0286e4              | +0de
.L028676:
        bset    #0x3,0x13(a1)                   | +0e2
        bset    #0x0,0x5a(a1)                   | +0e8
        bra.w   TaskHandler_028690              | +0ee
        .global Sub_00028594__L028686
Sub_00028594__L028686:
.L028686:
        move.b  #0xff,d1                        | +0f2

| ----------------------------------------------------------------------------
|  TaskHandler_028690  @ $028690  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028690, "ax", @progbits
        .global TaskHandler_028690
TaskHandler_028690:
        move.w  0x2(a0),d0                      | +000
        jsr     TaskHandler_0286ec(pc)          | +004
        move.b  0x4(a0),d0                      | +008
        move.b  0x1(a2),d1                      | +00c
        cmp.b   d1,d0                           | +010
        bgt.w   TaskHandler_0286c4              | +012
        .global TaskHandler_028690__L0286a6
TaskHandler_028690__L0286a6:
.L0286a6:
        bset    #0x2,0x13(a6)                   | +016
        bset    #0x1,0x13(a6)                   | +01c
        bset    #0x2,0x5a(a6)                   | +022
        bset    #0x1,0x5a(a6)                   | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0286c4  @ $0286C4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0286c4, "ax", @progbits
        .global TaskHandler_0286c4
TaskHandler_0286c4:
        bset    #0x2,0x13(a6)                   | +000
        bset    #0x2,0x5a(a6)                   | +006
        jmp     ClearXN_02868a(pc)              | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0286d4  @ $0286D4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0286d4, "ax", @progbits
        .global TaskHandler_0286d4
TaskHandler_0286d4:
        bset    #0x1,0x13(a6)                   | +000
        bset    #0x1,0x5a(a6)                   | +006
        jmp     Sub_00028594__L028686(pc)       | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0286e4  @ $0286E4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0286e4, "ax", @progbits
        .global TaskHandler_0286e4
TaskHandler_0286e4:
        move.b  #0xff,d1                        | +000
        jmp     TaskHandler_028690__L0286a6(pc) | +004

| ----------------------------------------------------------------------------
|  TaskHandler_0286ec  @ $0286EC  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0286ec, "ax", @progbits
        .global TaskHandler_0286ec
TaskHandler_0286ec:
        move.w  0x66(a1),d1                     | +000
        sub.w   d0,d1                           | +004
        bgt.w   .L0286fe                        | +006
        bset    #0x0,0x13(a1)                   | +00a
        eor.w   d1,d1                           | +010
.L0286fe:
        move.w  d1,0x66(a1)                     | +012

| ----------------------------------------------------------------------------
|  JmpTarget_02870a  @ $02870A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.JmpTarget_02870a, "ax", @progbits
        .global JmpTarget_02870a
JmpTarget_02870a:
        btst    #0x3,0x13(a6)                   | +000
        bne.w   TaskHandler_02871c              | +006
        clr.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_02871c  @ $02871C  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02871c, "ax", @progbits
        .global TaskHandler_02871c
TaskHandler_02871c:
        movea.l 0x48(a6),a2                     | +000
        move.w  #0x1,d0                         | +004
        cmpa.l  #0xffffffff,a2                  | +008
        beq.w   SetXN_028752                    | +00e
        adda.l  #0xa,a2                         | +012
        move.w  0x54(a6),d0                     | +018
        move.w  (a2),d2                         | +01c
        move.w  0x2(a2),d3                      | +01e
        add.w   0x22(a6),d2                     | +022
        add.w   0x22(a6),d3                     | +026
        add.w   d2,d3                           | +02a
        asr.w   #0x1,d3                         | +02c
        sub.w   d3,d0                           | +02e
        subx.w  d0,d0                           | +030
        addi.w  #0x2,d0                         | +032

| ----------------------------------------------------------------------------
|  JmpTarget_028758  @ $028758  (10 B)
| ----------------------------------------------------------------------------
        .section .text.JmpTarget_028758, "ax", @progbits
        .global JmpTarget_028758
JmpTarget_028758:
        btst    #0x0,0x13(a6)                   | +000
        bne.w   SetXN_028768                    | +006

| ----------------------------------------------------------------------------
|  TaskHandler_02876e  @ $02876E  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02876e, "ax", @progbits
        .global TaskHandler_02876e
TaskHandler_02876e:
        move.w  (a0),d0                         | +000
        move.w  0x2(a0),d1                      | +002
        btst    #0x0,0x3a(a6)                   | +006
        beq.w   .L028784                        | +00c
        neg.w   d0                              | +010
        neg.w   d1                              | +012
        exg     d0,d1                           | +014
.L028784:
        cmp.w   d1,d0                           | +016
        blt.w   .L028794                        | +018
        nop                                     | +01c
        nop                                     | +01e
        cmp.w   d1,d0                           | +020
        nop                                     | +022
        trap    #0xf                            | +024
.L028794:
        move.w  (a2),d2                         | +026
        move.w  0x2(a2),d3                      | +028
        btst    #0x0,0x3a(a1)                   | +02c
        beq.w   .L0287aa                        | +032
        neg.w   d2                              | +036
        neg.w   d3                              | +038
        exg     d2,d3                           | +03a
.L0287aa:
        cmp.w   d3,d2                           | +03c
        blt.w   .L0287ba                        | +03e
        nop                                     | +042
        nop                                     | +044
        cmp.w   d3,d2                           | +046
        nop                                     | +048
        trap    #0xf                            | +04a
.L0287ba:
        add.w   0x22(a6),d0                     | +04c
        add.w   0x22(a6),d1                     | +050
        add.w   0x22(a1),d2                     | +054
        add.w   0x22(a1),d3                     | +058
        cmp.w   d1,d2                           | +05c
        bgt.w   ClearXN_0287d6                  | +05e
        cmp.w   d3,d0                           | +062
        ble.w   TaskHandler_0287dc              | +064

| ----------------------------------------------------------------------------
|  TaskHandler_0287dc  @ $0287DC  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0287dc, "ax", @progbits
        .global TaskHandler_0287dc
TaskHandler_0287dc:
        cmp.w   d2,d0                           | +000
        bcc.w   .L0287e4                        | +002
        move.w  d2,d0                           | +006
.L0287e4:
        cmp.w   d3,d1                           | +008
        bcs.w   .L0287ec                        | +00a
        move.w  d3,d1                           | +00e
.L0287ec:
        add.w   d0,d1                           | +010
        asr.w   #0x1,d1                         | +012
        move.w  d1,d7                           | +014
        move.w  0x4(a0),d0                      | +016
        move.w  0x6(a0),d1                      | +01a
        btst    #0x1,0x3a(a6)                   | +01e
        beq.w   .L02880a                        | +024
        neg.w   d0                              | +028
        neg.w   d1                              | +02a
        exg     d0,d1                           | +02c
.L02880a:
        cmp.w   d1,d0                           | +02e
        blt.w   .L02881a                        | +030
        nop                                     | +034
        nop                                     | +036
        cmp.w   d1,d0                           | +038
        nop                                     | +03a
        trap    #0xf                            | +03c
.L02881a:
        move.w  0x4(a2),d2                      | +03e
        move.w  0x6(a2),d3                      | +042
        btst    #0x1,0x3a(a1)                   | +046
        beq.w   .L028832                        | +04c
        neg.w   d2                              | +050
        neg.w   d3                              | +052
        exg     d2,d3                           | +054
.L028832:
        cmp.w   d3,d2                           | +056
        blt.w   .L028842                        | +058
        nop                                     | +05c
        nop                                     | +05e
        cmp.w   d3,d2                           | +060
        nop                                     | +062
        trap    #0xf                            | +064
.L028842:
        add.w   0x24(a6),d0                     | +066
        add.w   0x24(a6),d1                     | +06a
        add.w   0x24(a1),d2                     | +06e
        add.w   0x24(a1),d3                     | +072
        cmp.w   d1,d2                           | +076
        bgt.w   ClearXN_02885e                  | +078
        cmp.w   d3,d0                           | +07c
        ble.w   TaskHandler_028864              | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_028864  @ $028864  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028864, "ax", @progbits
        .global TaskHandler_028864
TaskHandler_028864:
        cmp.w   d2,d0                           | +000
        bcc.w   .L02886c                        | +002
        move.w  d2,d0                           | +006
.L02886c:
        cmp.w   d3,d1                           | +008
        bcs.w   .L028874                        | +00a
        move.w  d3,d1                           | +00e
.L028874:
        add.w   d0,d1                           | +010
        asr.w   #0x1,d1                         | +012
        move.w  d1,0x56(a1)                     | +014
        move.w  d7,0x54(a1)                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_028886  @ $028886  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028886, "ax", @progbits
        .global TaskHandler_028886
TaskHandler_028886:
        move.w  (a0),d0                         | +000
        move.w  (a2),d2                         | +002
        move.w  0x2(a2),d3                      | +004
        btst    #0x0,0x3a(a1)                   | +008
        beq.w   .L02889e                        | +00e
        neg.w   d2                              | +012
        neg.w   d3                              | +014
        exg     d2,d3                           | +016
.L02889e:
        cmp.w   d3,d2                           | +018
        blt.w   .L0288ae                        | +01a
        nop                                     | +01e
        nop                                     | +020
        cmp.w   d3,d2                           | +022
        nop                                     | +024
        trap    #0xf                            | +026
.L0288ae:
        add.w   0x22(a6),d0                     | +028
        add.w   0x22(a1),d2                     | +02c
        add.w   0x22(a1),d3                     | +030
        cmp.w   d0,d2                           | +034
        bgt.w   ClearXN_0288c6                  | +036
        cmp.w   d3,d0                           | +03a
        ble.w   TaskHandler_0288cc              | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_0288cc  @ $0288CC  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0288cc, "ax", @progbits
        .global TaskHandler_0288cc
TaskHandler_0288cc:
        move.w  d0,d7                           | +000
        move.w  0x4(a0),d0                      | +002
        move.w  0x4(a2),d2                      | +006
        move.w  0x6(a2),d3                      | +00a
        btst    #0x1,0x3a(a1)                   | +00e
        beq.w   .L0288ea                        | +014
        neg.w   d2                              | +018
        neg.w   d3                              | +01a
        exg     d2,d3                           | +01c
.L0288ea:
        cmp.w   d3,d2                           | +01e
        blt.w   .L0288fa                        | +020
        nop                                     | +024
        nop                                     | +026
        cmp.w   d3,d2                           | +028
        nop                                     | +02a
        trap    #0xf                            | +02c
.L0288fa:
        add.w   0x24(a6),d0                     | +02e
        add.w   0x24(a1),d2                     | +032
        add.w   0x24(a1),d3                     | +036
        cmp.w   d0,d2                           | +03a
        bgt.w   ClearXN_028912                  | +03c
        cmp.w   d3,d0                           | +040
        ble.w   TaskHandler_028918              | +042

| ----------------------------------------------------------------------------
|  TaskHandler_028918  @ $028918  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028918, "ax", @progbits
        .global TaskHandler_028918
TaskHandler_028918:
        move.w  d7,0x54(a1)                     | +000
        move.w  d0,0x56(a1)                     | +004

| ----------------------------------------------------------------------------
|  TaskHandler_028926  @ $028926  (96 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028926, "ax", @progbits
        .global TaskHandler_028926
TaskHandler_028926:
        ori.b   #0x1,d0                         | +000
        ori.b   #0x1,d0                         | +004
        dc.w    Sub_0000FFFF                    | +008
        .global TaskHandler_028926__L028930
TaskHandler_028926__L028930:
.L028930:
        cmpi.b  #0x0,0x6(a0)                    | +00a
        bne.w   ClearC_028986                   | +010
        cmpa.l  #0xffffffff,a4                  | +014
        bne.w   .L028948                        | +01a
        lea     TaskHandler_028926(pc),a4       | +01e
.L028948:
        move.w  #0x0,d0                         | +022
        move.w  #0x140,d1                       | +026
        move.w  #0x100,d2                       | +02a
        move.w  #0x1f0,d3                       | +02e
        add.w   (a4),d0                         | +032
        add.w   0x2(a4),d1                      | +034
        sub.w   0x6(a4),d2                      | +038
        sub.w   0x4(a4),d3                      | +03c
        cmp.w   0x22(a6),d0                     | +040
        bgt.w   SetC_02898c                     | +044
        cmp.w   0x22(a6),d1                     | +048
        blt.w   SetC_02898c                     | +04c
        cmp.w   0x24(a6),d2                     | +050
        bgt.w   SetC_02898c                     | +054
        cmp.w   0x24(a6),d3                     | +058
        blt.w   SetC_02898c                     | +05c

| ----------------------------------------------------------------------------
|  Data_0289f6  @ $0289F6  (160 B)
| ----------------------------------------------------------------------------
        .section .text.Data_0289f6, "ax", @progbits
        .global Data_0289f6
Data_0289f6:
        .dc.w   0x8001                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +010  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +012  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +014  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +018  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +01e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +020  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xffe0                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0018                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0020                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x8001                        | +050  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +054  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +058  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +060  (dato / opcode no decodificado)
        .dc.w   0x3608                        | +062  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +064  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +068  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x3612                        | +06e  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +070  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +074  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +078  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +07a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +07c  (dato / opcode no decodificado)
        .dc.w   0xfffc                        | +07e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +080  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +082  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +084  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +086  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +088  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +08a  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +08c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +08e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +090  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +092  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +094  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +096  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +098  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +09a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09c  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +09e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  Fn_00028C20  @ $028C20  (64 B)
| ----------------------------------------------------------------------------
        .section .text.Fn_00028C20, "ax", @progbits
        .global Fn_00028C20
Fn_00028C20:
        move.w  (a4),d0                         | +000
        move.w  (a3),d2                         | +002
        move.w  0x2(a3),d3                      | +004
        btst    #0x0,0x3a(a1)                   | +008
        beq.w   .L028c38                        | +00e
        neg.w   d2                              | +012
        neg.w   d3                              | +014
        exg     d2,d3                           | +016
.L028c38:
        cmp.w   d3,d2                           | +018
        blt.w   .L028c48                        | +01a
        nop                                     | +01e
        nop                                     | +020
        cmp.w   d3,d2                           | +022
        nop                                     | +024
        trap    #0xf                            | +026
.L028c48:
        add.w   0x22(a6),d0                     | +028
        add.w   0x22(a1),d2                     | +02c
        add.w   0x22(a1),d3                     | +030
        cmp.w   d0,d2                           | +034
        bgt.w   ClearXN_028c60                  | +036
        cmp.w   d3,d0                           | +03a
        ble.w   TaskHandler_028c66              | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_028c66  @ $028C66  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028c66, "ax", @progbits
        .global TaskHandler_028c66
TaskHandler_028c66:
        move.w  d0,d7                           | +000
        move.w  0x4(a4),d0                      | +002
        move.w  0x4(a3),d2                      | +006
        move.w  0x6(a3),d3                      | +00a
        btst    #0x1,0x3a(a1)                   | +00e
        beq.w   .L028c84                        | +014
        neg.w   d2                              | +018
        neg.w   d3                              | +01a
        exg     d2,d3                           | +01c
.L028c84:
        cmp.w   d3,d2                           | +01e
        blt.w   .L028c94                        | +020
        nop                                     | +024
        nop                                     | +026
        cmp.w   d3,d2                           | +028
        nop                                     | +02a
        trap    #0xf                            | +02c
.L028c94:
        add.w   0x24(a6),d0                     | +02e
        add.w   0x24(a1),d2                     | +032
        add.w   0x24(a1),d3                     | +036
        cmp.w   d0,d2                           | +03a
        bgt.w   ClearXN_028cac                  | +03c
        cmp.w   d3,d0                           | +040
        ble.w   SetXN_028cb2                    | +042

| ----------------------------------------------------------------------------
|  TaskHandler_028cb8  @ $028CB8  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028cb8, "ax", @progbits
        .global TaskHandler_028cb8
TaskHandler_028cb8:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_028cce                    | +00c

| ----------------------------------------------------------------------------
|  Data_028cf0  @ $028CF0  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Data_028cf0, "ax", @progbits
        .global Data_028cf0
Data_028cf0:
        .dc.w   0x0002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x8dd4                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +004  (dato / opcode no decodificado)
        .dc.w   0x8df2                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +008  (dato / opcode no decodificado)
        .dc.w   0x8dfe                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +00c  (dato / opcode no decodificado)
        .dc.w   0x902c                        | +00e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +010  (dato / opcode no decodificado)
        .dc.w   0x906e                        | +012  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +014  (dato / opcode no decodificado)
        .dc.w   0x909e                        | +016  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +018  (dato / opcode no decodificado)
        .dc.w   0x90ca                        | +01a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +01c  (dato / opcode no decodificado)
        .dc.w   0x90f6                        | +01e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +020  (dato / opcode no decodificado)
        .dc.w   0x9122                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +024  (dato / opcode no decodificado)
        .dc.w   0x913e                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +028  (dato / opcode no decodificado)
        .dc.w   0x9152                        | +02a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +02c  (dato / opcode no decodificado)
        .dc.w   0x9166                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +030  (dato / opcode no decodificado)
        .dc.w   0x917a                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +034  (dato / opcode no decodificado)
        .dc.w   0x9196                        | +036  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +038  (dato / opcode no decodificado)
        .dc.w   0x91c4                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x91f2                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +040  (dato / opcode no decodificado)
        .dc.w   0x9220                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +044  (dato / opcode no decodificado)
        .dc.w   0x924e                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +048  (dato / opcode no decodificado)
        .dc.w   0x927c                        | +04a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +04c  (dato / opcode no decodificado)
        .dc.w   0x92aa                        | +04e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +050  (dato / opcode no decodificado)
        .dc.w   0x92d8                        | +052  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +054  (dato / opcode no decodificado)
        .dc.w   0x9306                        | +056  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +058  (dato / opcode no decodificado)
        .dc.w   0x9334                        | +05a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +05c  (dato / opcode no decodificado)
        .dc.w   0x933c                        | +05e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +060  (dato / opcode no decodificado)
        .dc.w   0x934c                        | +062  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +064  (dato / opcode no decodificado)
        .dc.w   0x935c                        | +066  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +068  (dato / opcode no decodificado)
        .dc.w   0x9376                        | +06a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +06c  (dato / opcode no decodificado)
        .dc.w   0x93a8                        | +06e  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +070  (dato / opcode no decodificado)
        .dc.w   0x93c6                        | +072  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +074  (dato / opcode no decodificado)
        .dc.w   0x93ec                        | +076  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +078  (dato / opcode no decodificado)
        .dc.w   0x94b0                        | +07a  (dato / opcode no decodificado)
        .dc.w   0x0002                        | +07c  (dato / opcode no decodificado)
        .dc.w   0x9086                        | +07e  (dato / opcode no decodificado)

| ----------------------------------------------------------------------------
|  TaskHandler_028dd4  @ $028DD4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028dd4, "ax", @progbits
        .global TaskHandler_028dd4
TaskHandler_028dd4:
        move.b  0x46(a6),d2                     | +000
        bne.w   .L028de4                        | +004
        move.b  0x1(a1),d2                      | +008
        move.b  d2,0x46(a6)                     | +00c
.L028de4:
        adda.l  #0x2,a1                         | +010
        move.l  a1,0x3c(a6)                     | +016
        jmp     Script_DispatchOpcode__L028da8(pc) | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_028df2  @ $028DF2  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028df2, "ax", @progbits
        .global TaskHandler_028df2
TaskHandler_028df2:
        movea.l 0x2(a1),a1                      | +000
        move.l  a1,0x3c(a6)                     | +004
        jmp     Script_DispatchOpcode__L028da8(pc) | +008

| ----------------------------------------------------------------------------
|  TaskHandler_028dfe  @ $028DFE  (546 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_028dfe, "ax", @progbits
        .global TaskHandler_028dfe
TaskHandler_028dfe:
        move.l  a1,0x40(a6)                     | +000
        .global TaskHandler_028dfe__L028e02
TaskHandler_028dfe__L028e02:
.L028e02:
        move.b  0x1(a1),d1                      | +004
        move.b  0x3a(a6),d2                     | +008
        eor.b   d1,d2                           | +00c
        btst    #0x3,0x1(a1)                    | +00e
        beq.w   .L028e22                        | +014
        move.w  0x22(a6),d0                     | +018
        move.w  0x24(a6),d1                     | +01c
        bra.w   .L028e46                        | +020
.L028e22:
        move.w  0x8(a1),d0                      | +024
        btst    #0x0,d2                         | +028
        beq.w   .L028e30                        | +02c
        neg.w   d0                              | +030
.L028e30:
        add.w   0x22(a6),d0                     | +032
        move.w  0xa(a1),d1                      | +036
        btst    #0x1,d2                         | +03a
        beq.w   .L028e42                        | +03e
        neg.w   d1                              | +042
.L028e42:
        add.w   0x24(a6),d1                     | +044
.L028e46:
        swap    d0                              | +048
        subi.w  #0x1,d1                         | +04a
        move.w  d1,d0                           | +04e
        move.b  0x59(a6),d1                     | +050
        andi.b  #0x1,d1                         | +054
        beq.w   .L028e9c                        | +058
        btst    #0x6,0x6b(a6)                   | +05c
        beq.w   .L028e6e                        | +062
        bset    #0x7,0x5a(a6)                   | +066
        bra.w   .L028e9c                        | +06c
.L028e6e:
        btst    #0x0,0x6b(a6)                   | +070
        beq.w   .L028e82                        | +076
        bset    #0x7,0x5a(a6)                   | +07a
        bra.w   .L028e9c                        | +080
.L028e82:
        btst    #0x1,0x100001.l                 | +084
        bne.w   .L028e96                        | +08c
        jmp     .L028fca(pc)                    | +090
        bra.w   .L028e9c                        | +094
.L028e96:
        bset    #0x7,0x5a(a6)                   | +098
.L028e9c:
        btst    #0x5,0x12(a6)                   | +09e
        beq.w   .L028ec6                        | +0a4
        bclr    #0x5,0x12(a6)                   | +0a8
        move.b  0x12(a6),d1                     | +0ae
        andi.b  #0x10,d1                        | +0b2
        ror.b   #0x4,d1                         | +0b6
        move.b  0x1(a1),d2                      | +0b8
        andi.b  #0x10,d2                        | +0bc
        ror.b   #0x4,d2                         | +0c0
        eor.b   d2,d1                           | +0c2
        bne.w   .L028fca                        | +0c4
.L028ec6:
        btst    #0x2,0x1(a1)                    | +0c8
        beq.w   .L028eda                        | +0ce
        btst    #0x3,0x12(a6)                   | +0d2
        bne.w   .L028fca                        | +0d8
.L028eda:
        move.w  0x38(a6),d1                     | +0dc
        cmpi.w  #0xffff,0x6(a1)                 | +0e0
        beq.w   .L028eec                        | +0e6
        move.w  0x6(a1),d1                      | +0ea
.L028eec:
        move.w  0x14(a6),d2                     | +0ee
        btst    #0x7,0x5a(a6)                   | +0f2
        bne.w   .L028f04                        | +0f8
        btst    #0x0,0x5a(a6)                   | +0fc
        beq.w   .L028f22                        | +102
.L028f04:
        btst    #0x0,0x13(a6)                   | +106
        bne.w   .L028f22                        | +10c
        cmpi.w  #0xffff,0x1c(a6)                | +110
        beq.w   .L028f22                        | +116
        move.w  0x1c(a6),d2                     | +11a
        bset    #0x7,0x5a(a6)                   | +11e
.L028f22:
        btst    #0x4,0x100001.l                 | +124
        beq.w   .L028f3e                        | +12c
        btst    #0x0,0x6b(a6)                   | +130
        beq.w   .L028f3e                        | +136
        jsr     0x2f84a.l                       | +13a
.L028f3e:
        move.b  0x3a(a6),d5                     | +140
        move.b  0x12(a6),d4                     | +144
        andi.b  #0x4,d4                         | +148
        or.b    d4,d5                           | +14c
        move.b  0x1(a1),d4                      | +14e
        andi.b  #0x3,d4                         | +152
        eor.b   d4,d5                           | +156
        move.b  0x33(a6),d3                     | +158
        move.b  0x32(a6),d4                     | +15c
        movea.l 0x2(a1),a0                      | +160
        movem.l d0,-(a7)                        | +164
        movem.l d1,-(a7)                        | +168
        move.w  d0,d1                           | +16c
        swap    d0                              | +16e
        cmpi.w  #0xff00,d0                      | +170
        bgt.w   .L028f7a                        | +174
        bra.w   .L028fa2                        | +178
.L028f7a:
        cmpi.w  #0x200,d0                       | +17c
        blt.w   .L028f86                        | +180
        bra.w   .L028fa2                        | +184
.L028f86:
        cmpi.w  #0x0,d1                         | +188
        bgt.w   .L028f92                        | +18c
        bra.w   .L028fa2                        | +190
.L028f92:
        cmpi.w  #0x300,d1                       | +194
        blt.w   .L028f9e                        | +198
        bra.w   .L028fa2                        | +19c
.L028f9e:
        bra.w   .L028fa8                        | +1a0
.L028fa2:
        movea.l #0xffffffff,a0                  | +1a4
.L028fa8:
        movem.l (a7)+,d1                        | +1aa
        movem.l (a7)+,d0                        | +1ae
        btst    #0x6,0x12(a6)                   | +1b2
        bne.w   .L028fc4                        | +1b8
        jsr     0x5a9e2.l                       | +1bc
        bra.w   .L028fca                        | +1c2
.L028fc4:
        jsr     0x5a9d6.l                       | +1c6
.L028fca:
        move.b  0x1(a1),d0                      | +1cc
        adda.l  #0x8,a1                         | +1d0
        btst    #0x3,d0                         | +1d6
        bne.w   .L028fe2                        | +1da
        adda.l  #0x4,a1                         | +1de
.L028fe2:
        cmpi.b  #0x18,(a1)                      | +1e4
        bne.w   .L028fee                        | +1e8
        jmp     TaskHandler_02934c(pc)          | +1ec
.L028fee:
        cmpi.b  #0x2,(a1)                       | +1f0
        bne.w   .L028ffa                        | +1f4
        jmp     .L028e02(pc)                    | +1f8
.L028ffa:
        tst.b   0x46(a6)                        | +1fc
        beq.w   SetXN_029026                    | +200
        tst.b   0x44(a6)                        | +204
        bne.w   ClearXN_029020                  | +208
        subi.b  #0x1,0x46(a6)                   | +20c
        bne.w   ClearXN_029020                  | +212
        cmpi.b  #0x16,(a1)                      | +216
        beq.w   SetXN_029026                    | +21a
        move.l  a1,0x3c(a6)                     | +21e

| ----------------------------------------------------------------------------
|  TaskHandler_02902c  @ $02902C  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02902c, "ax", @progbits
        .global TaskHandler_02902c
TaskHandler_02902c:
        tst.b   0x47(a6)                        | +000
        bne.w   .L02903e                        | +004
        move.b  0x1(a1),d1                      | +008
        addq.b  #0x1,d1                         | +00c
        move.b  d1,0x47(a6)                     | +00e
.L02903e:
        move.b  0x47(a6),d1                     | +012
        clr.b   d0                              | +016
        subi.b  #0x1,d1                         | +018
        addx.b  d0,d1                           | +01c
        move.b  d1,0x47(a6)                     | +01e
        tst.b   d1                              | +022
        beq.w   .L029060                        | +024
        movea.l 0x2(a1),a1                      | +028
        move.l  a1,0x3c(a6)                     | +02c
        jmp     Script_DispatchOpcode__L028da8(pc) | +030
.L029060:
        adda.l  #0x6,a1                         | +034
        move.l  a1,0x3c(a6)                     | +03a
        jmp     Script_DispatchOpcode__L028da8(pc) | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_02906e  @ $02906E  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02906e, "ax", @progbits
        .global TaskHandler_02906e
TaskHandler_02906e:
        move.w  0x2(a1),d0                      | +000
        jsr     0x2352.l                        | +004
        adda.l  #0x4,a1                         | +00a
        move.l  a1,0x3c(a6)                     | +010
        jmp     Script_DispatchOpcode__L028da8(pc) | +014

| ----------------------------------------------------------------------------
|  TaskHandler_029086  @ $029086  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029086, "ax", @progbits
        .global TaskHandler_029086
TaskHandler_029086:
        move.w  0x2(a1),d0                      | +000
        jsr     0x2222.l                        | +004
        adda.l  #0x4,a1                         | +00a
        move.l  a1,0x3c(a6)                     | +010
        jmp     Script_DispatchOpcode__L028da8(pc) | +014

| ----------------------------------------------------------------------------
|  TaskHandler_02909e  @ $02909E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02909e, "ax", @progbits
        .global TaskHandler_02909e
TaskHandler_02909e:
        move.w  0x2(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0290b6                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0290b6:
        move.b  0x1(a1),(a6,d1.w)               | +018
        adda.l  #0x4,a1                         | +01e
        move.l  a1,0x3c(a6)                     | +024
        jmp     Script_DispatchOpcode__L028da8(pc) | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0290ca  @ $0290CA  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0290ca, "ax", @progbits
        .global TaskHandler_0290ca
TaskHandler_0290ca:
        move.w  0x4(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0290e2                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0290e2:
        move.w  0x2(a1),(a6,d1.w)               | +018
        adda.l  #0x6,a1                         | +01e
        move.l  a1,0x3c(a6)                     | +024
        jmp     Script_DispatchOpcode__L028da8(pc) | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0290f6  @ $0290F6  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0290f6, "ax", @progbits
        .global TaskHandler_0290f6
TaskHandler_0290f6:
        move.w  0x6(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L02910e                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L02910e:
        move.l  0x2(a1),(a6,d1.w)               | +018
        adda.l  #0x8,a1                         | +01e
        move.l  a1,0x3c(a6)                     | +024
        jmp     Script_DispatchOpcode__L028da8(pc) | +028

| ----------------------------------------------------------------------------
|  TaskHandler_029122  @ $029122  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029122, "ax", @progbits
        .global TaskHandler_029122
TaskHandler_029122:
        movem.l a1,-(a7)                        | +000
        movea.l 0x2(a1),a1                      | +004
        jsr     (a1)                            | +008
        movem.l (a7)+,a1                        | +00a
        adda.l  #0x6,a1                         | +00e
        move.l  a1,0x3c(a6)                     | +014
        jmp     Script_DispatchOpcode__L028da8(pc) | +018

| ----------------------------------------------------------------------------
|  TaskHandler_02913e  @ $02913E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02913e, "ax", @progbits
        .global TaskHandler_02913e
TaskHandler_02913e:
        move.l  0x2(a1),0x48(a6)                | +000
        adda.l  #0x6,a1                         | +006
        move.l  a1,0x3c(a6)                     | +00c
        jmp     Script_DispatchOpcode__L028da8(pc) | +010

| ----------------------------------------------------------------------------
|  TaskHandler_029152  @ $029152  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029152, "ax", @progbits
        .global TaskHandler_029152
TaskHandler_029152:
        move.l  0x2(a1),0x4c(a6)                | +000
        adda.l  #0x6,a1                         | +006
        move.l  a1,0x3c(a6)                     | +00c
        jmp     Script_DispatchOpcode__L028da8(pc) | +010

| ----------------------------------------------------------------------------
|  TaskHandler_029166  @ $029166  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029166, "ax", @progbits
        .global TaskHandler_029166
TaskHandler_029166:
        move.l  0x2(a1),0x60(a6)                | +000
        adda.l  #0x6,a1                         | +006
        move.l  a1,0x3c(a6)                     | +00c
        jmp     Script_DispatchOpcode__L028da8(pc) | +010

| ----------------------------------------------------------------------------
|  TaskHandler_02917a  @ $02917A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02917a, "ax", @progbits
        .global TaskHandler_02917a
TaskHandler_02917a:
        move.b  0x1(a1),d0                      | +000
        andi.w  #0xff,d0                        | +004
        move.w  (a6,d0.w),0x14(a6)              | +008
        adda.l  #0x2,a1                         | +00e
        move.l  a1,0x3c(a6)                     | +014
        jmp     Script_DispatchOpcode__L028da8(pc) | +018

| ----------------------------------------------------------------------------
|  TaskHandler_029196  @ $029196  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029196, "ax", @progbits
        .global TaskHandler_029196
TaskHandler_029196:
        move.w  0x2(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0291ae                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0291ae:
        move.b  0x1(a1),d0                      | +018
        and.b   d0,(a6,d1.w)                    | +01c
        adda.l  #0x4,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_0291c4  @ $0291C4  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0291c4, "ax", @progbits
        .global TaskHandler_0291c4
TaskHandler_0291c4:
        move.w  0x4(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0291dc                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0291dc:
        move.w  0x2(a1),d0                      | +018
        and.w   d0,(a6,d1.w)                    | +01c
        adda.l  #0x6,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_0291f2  @ $0291F2  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0291f2, "ax", @progbits
        .global TaskHandler_0291f2
TaskHandler_0291f2:
        move.w  0x6(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L02920a                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L02920a:
        move.l  0x2(a1),d0                      | +018
        and.l   d0,(a6,d1.w)                    | +01c
        adda.l  #0x8,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_029220  @ $029220  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029220, "ax", @progbits
        .global TaskHandler_029220
TaskHandler_029220:
        move.w  0x2(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L029238                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L029238:
        move.b  0x1(a1),d0                      | +018
        or.b    d0,(a6,d1.w)                    | +01c
        adda.l  #0x4,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_02924e  @ $02924E  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02924e, "ax", @progbits
        .global TaskHandler_02924e
TaskHandler_02924e:
        move.w  0x4(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L029266                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L029266:
        move.w  0x2(a1),d0                      | +018
        or.w    d0,(a6,d1.w)                    | +01c
        adda.l  #0x6,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_02927c  @ $02927C  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02927c, "ax", @progbits
        .global TaskHandler_02927c
TaskHandler_02927c:
        move.w  0x6(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L029294                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L029294:
        move.l  0x2(a1),d0                      | +018
        or.l    d0,(a6,d1.w)                    | +01c
        adda.l  #0x8,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_0292aa  @ $0292AA  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0292aa, "ax", @progbits
        .global TaskHandler_0292aa
TaskHandler_0292aa:
        move.w  0x2(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0292c2                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0292c2:
        move.b  0x1(a1),d0                      | +018
        eor.b   d0,(a6,d1.w)                    | +01c
        adda.l  #0x4,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_0292d8  @ $0292D8  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0292d8, "ax", @progbits
        .global TaskHandler_0292d8
TaskHandler_0292d8:
        move.w  0x4(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L0292f0                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0292f0:
        move.w  0x2(a1),d0                      | +018
        eor.w   d0,(a6,d1.w)                    | +01c
        adda.l  #0x6,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_029306  @ $029306  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029306, "ax", @progbits
        .global TaskHandler_029306
TaskHandler_029306:
        move.w  0x6(a1),d1                      | +000
        cmpi.w  #0xa0,d1                        | +004
        blt.w   .L02931e                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0xa0,d1                        | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L02931e:
        move.l  0x2(a1),d0                      | +018
        eor.l   d0,(a6,d1.w)                    | +01c
        adda.l  #0x8,a1                         | +020
        move.l  a1,0x3c(a6)                     | +026
        jmp     Script_DispatchOpcode__L028da8(pc) | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_029334  @ $029334  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029334, "ax", @progbits
        .global TaskHandler_029334
TaskHandler_029334:
        movea.l 0x40(a6),a1                     | +000
        jmp     TaskHandler_028dfe(pc)          | +004

| ----------------------------------------------------------------------------
|  TaskHandler_02933c  @ $02933C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02933c, "ax", @progbits
        .global TaskHandler_02933c
TaskHandler_02933c:
        bset    #0x5,0x12(a6)                   | +000
        adda.l  #0x2,a1                         | +006
        jmp     TaskHandler_028dfe(pc)          | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_02934c  @ $02934C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02934c, "ax", @progbits
        .global TaskHandler_02934c
TaskHandler_02934c:
        bset    #0x5,0x12(a6)                   | +000
        adda.l  #0x2,a1                         | +006
        jmp     TaskHandler_028dfe__L028e02(pc) | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_02935c  @ $02935C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02935c, "ax", @progbits
        .global TaskHandler_02935c
TaskHandler_02935c:
        move.l  a1,0x70(a6)                     | +000
        addi.l  #0x2,0x70(a6)                   | +004
        adda.l  #0xa,a1                         | +00c
        move.l  a1,0x3c(a6)                     | +012
        jmp     Script_DispatchOpcode__L028da8(pc) | +016

| ----------------------------------------------------------------------------
|  TaskHandler_029376  @ $029376  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029376, "ax", @progbits
        .global TaskHandler_029376
TaskHandler_029376:
        move.b  0x46(a6),d2                     | +000
        bne.w   .L02939a                        | +004
        move.b  0x1(a1),d0                      | +008
        andi.w  #0xff,d0                        | +00c
        movea.l (a6,d0.w),a0                    | +010
        move.b  0x3b(a6),d0                     | +014
        andi.w  #0xff,d0                        | +018
        move.b  (a0,d0.w),d2                    | +01c
        move.b  d2,0x46(a6)                     | +020
.L02939a:
        adda.l  #0x2,a1                         | +024
        move.l  a1,0x3c(a6)                     | +02a
        jmp     Script_DispatchOpcode__L028da8(pc) | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_0293a8  @ $0293A8  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0293a8, "ax", @progbits
        .global TaskHandler_0293a8
TaskHandler_0293a8:
        clr.w   d0                              | +000
        move.b  0x1(a1),d0                      | +002
        move.w  (a6,d0.w),d1                    | +006
        mulu.w  #0x12,d1                        | +00a
        adda.l  #0x2,a1                         | +00e
        adda.l  d1,a1                           | +014
        move.l  a1,0x3c(a6)                     | +016
        jmp     Script_DispatchOpcode__L028da8(pc) | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0293c6  @ $0293C6  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0293c6, "ax", @progbits
        .global TaskHandler_0293c6
TaskHandler_0293c6:
        clr.w   d0                              | +000
        move.b  0x1(a1),d0                      | +002
        move.w  (a6,d0.w),d1                    | +006
        mulu.w  #0x1c,d1                        | +00a
        adda.l  #0x2,a1                         | +00e
        adda.l  d1,a1                           | +014
        move.l  a1,0x3c(a6)                     | +016
        jmp     Script_DispatchOpcode__L028da8(pc) | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0293e4  @ $0293E4  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0293e4, "ax", @progbits
        .global TaskHandler_0293e4
TaskHandler_0293e4:
        ori.b   #0x4e,d0                        | +000
        ori.b   #0x4e,d0                        | +004

| ----------------------------------------------------------------------------
|  TaskHandler_0293ec  @ $0293EC  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0293ec, "ax", @progbits
        .global TaskHandler_0293ec
TaskHandler_0293ec:
        move.b  0x1(a1),d0                      | +000
        andi.w  #0xf,d0                         | +004
        asl.w   #0x2,d0                         | +008
        move.l  TaskHandler_0293e4(pc,d0.w),d1  | +00a
        suba.l  d1,a1                           | +00e
        move.l  a1,0x3c(a6)                     | +010
        jmp     TaskHandler_028dfe__L028e02(pc) | +014

| ----------------------------------------------------------------------------
|  TaskHandler_029404  @ $029404  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029404, "ax", @progbits
        .global TaskHandler_029404
TaskHandler_029404:
        move.b  0x45(a6),d0                     | +000
        move.b  #0x0,d1                         | +004
        subi.b  #0x1,d0                         | +008
        addx.b  d1,d0                           | +00c
        move.b  d0,0x45(a6)                     | +00e
        move.b  0x44(a6),d0                     | +012
        move.b  #0x0,d1                         | +016
        subi.b  #0x1,d0                         | +01a
        addx.b  d1,d0                           | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_02942a  @ $02942A  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_02942a, "ax", @progbits
        .global TaskHandler_02942a
TaskHandler_02942a:
        move.w  0x2(a1),d0                      | +000
        cmpi.b  #0xb,0x106ece.l                 | +004
        beq.w   .L029446                        | +00c
        cmpi.b  #0xc,0x106ece.l                 | +010
        bne.w   .L02944a                        | +018
.L029446:
        andi.w  #0xfff,d0                       | +01c
.L02944a:
        move.w  0x4(a1),d1                      | +020
        jsr     0x440d0.l                       | +024
        move.w  d0,0x22(a6)                     | +02a
        move.w  d1,0x24(a6)                     | +02e
        move.w  0xc(a1),0x38(a6)                | +032
        move.w  0xa(a1),0x70(a6)                | +038
        move.b  0x1(a1),d0                      | +03e
        or.b    d0,0x12(a6)                     | +042
        tst.b   (a1)                            | +046
        bne.w   JsrPcRts_0294ae                 | +048
        move.w  0xe(a1),d1                      | +04c
        cmpi.w  #0xffff,d1                      | +050
        bne.w   .L02948e                        | +054
        nop                                     | +058
        nop                                     | +05a
        cmpi.w  #0xffff,d1                      | +05c
        nop                                     | +060
        trap    #0xf                            | +062
.L02948e:
        movem.l a1,-(a7)                        | +064
        jsr     0x236e.l                        | +068
        movem.l (a7)+,a1                        | +06e
        movea.l 0x10(a1),a0                     | +072
        cmpa.l  #0xffffffff,a0                  | +076
        beq.w   JsrPcRts_0294ae                 | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_0294b0  @ $0294B0  (170 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0294b0, "ax", @progbits
        .global TaskHandler_0294b0
TaskHandler_0294b0:
        move.l  a1,0x40(a6)                     | +000
.L0294b4:
        move.w  0x22(a6),d0                     | +004
        move.w  0x24(a6),d1                     | +008
        bra.w   .L0294c8                        | +00c
        move.w  0x22(a6),d0                     | +010
        move.w  0x24(a6),d1                     | +014
.L0294c8:
        swap    d0                              | +018
        subi.w  #0x1,d1                         | +01a
        move.w  d1,d0                           | +01e
        move.w  0x38(a6),d1                     | +020
        move.w  0x14(a6),d2                     | +024
        move.b  0x3a(a6),d5                     | +028
        move.b  0x12(a6),d4                     | +02c
        andi.b  #0x4,d4                         | +030
        or.b    d4,d5                           | +034
        move.b  0x33(a6),d3                     | +036
        move.b  0x32(a6),d4                     | +03a
        movea.l 0x2(a1),a0                      | +03e
        btst    #0x6,0x12(a6)                   | +042
        bne.w   .L029506                        | +048
        jsr     0x5a9e2.l                       | +04c
        bra.w   .L02950c                        | +052
.L029506:
        jsr     0x5a9d6.l                       | +056
.L02950c:
        move.b  0x1(a1),d0                      | +05c
        adda.l  #0x8,a1                         | +060
        btst    #0x3,d0                         | +066
        bne.w   .L029524                        | +06a
        adda.l  #0x4,a1                         | +06e
.L029524:
        cmpi.b  #0x18,(a1)                      | +074
        bne.w   .L029530                        | +078
        jmp     TaskHandler_02934c(pc)          | +07c
.L029530:
        cmpi.b  #0x2,(a1)                       | +080
        bne.w   .L02953c                        | +084
        jmp     .L0294b4(pc)                    | +088
.L02953c:
        tst.b   0x46(a6)                        | +08c
        beq.w   SetXN_029560                    | +090
        subi.b  #0x1,0x46(a6)                   | +094
        bne.w   ClearXN_02955a                  | +09a
        cmpi.b  #0x16,(a1)                      | +09e
        beq.w   SetXN_029560                    | +0a2
        move.l  a1,0x3c(a6)                     | +0a6

| ----------------------------------------------------------------------------
|  TaskHandler_029566  @ $029566  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_029566, "ax", @progbits
        .global TaskHandler_029566
TaskHandler_029566:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_02957c                    | +00c
