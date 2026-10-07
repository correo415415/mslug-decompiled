| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $06DFE8..$071FFC  (15,422 B, 221 entradas, 97 huecos)
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
|  TaskHandler_06dfe8  @ $06DFE8  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dfe8, "ax", @progbits
        .global TaskHandler_06dfe8
TaskHandler_06dfe8:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x1086,d0                      | +006
        jsr     0x2352.l                        | +00a
        eori.b  #0x1,0x3a(a6)                   | +010
        move.l  #0x2d3d9e,0x4c(a6)              | +016
        move.w  #0x3,d1                         | +01e
        jsr     0x236e.l                        | +022
        addi.w  #0x10,0x24(a6)                  | +028
        clr.w   0x2e(a6)                        | +02e
        move.w  #0xff00,0x2a(a6)                | +032
.L06e020:
        jsr     0x27bc8.l                       | +038
        bcc.b   .L06e020                        | +03e
        jsr     0x267e2.l                       | +040
        move.w  #0x180,d0                       | +046
        jsr     Sub_0006E20C(pc)                | +04a
        move.w  d0,0x28(a6)                     | +04e
        clr.w   0x72(a6)                        | +052
        jsr     0x283ca.l                       | +056
        move.w  #0xd000,d0                      | +05c
        jsr     0x28134.l                       | +060
        andi.w  #0xffe3,0x38(a6)                | +066
        ori.w   #0x14,0x38(a6)                  | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_06e062  @ $06E062  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e062, "ax", @progbits
        .global TaskHandler_06e062
TaskHandler_06e062:
        jsr     0x27a92.l                       | +000
        jsr     0x28d70.l                       | +006
        jsr     Sub_0006E2FE(pc)                | +00c
        cmpi.w  #0x32,0x72(a6)                  | +010
        blt.w   .L06e084                        | +016
        lea     0x31c20.l,a1                    | +01a
        move.l  a1,(a6)                         | +020
.L06e084:
        btst    #0x1,0x13(a6)                   | +022
        beq.w   .L06e096                        | +028
        lea     0x31c20.l,a1                    | +02c
        move.l  a1,(a6)                         | +032
.L06e096:
        bra.w   Sub_0006E15E                    | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06e09a  @ $06E09A  (188 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e09a, "ax", @progbits
        .global TaskHandler_06e09a
TaskHandler_06e09a:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x1086,d0                      | +006
        jsr     0x2352.l                        | +00a
        move.l  #0x2d3cfa,0x4c(a6)              | +010
        move.w  #0x58,d1                        | +018
        jsr     0x236e.l                        | +01c
        jsr     0x5e9b6.l                       | +022
        andi.w  #0x3,d0                         | +028
        move.w  d0,d1                           | +02c
        add.w   d0,d0                           | +02e
        add.w   d1,d0                           | +030
        add.w   d0,d0                           | +032
        ext.l   d0                              | +034
        lea     0x2d4212.l,a0                   | +036
        adda.l  d0,a0                           | +03c
        move.w  0x2(a0),0x2e(a6)                | +03e
        move.w  0x4(a0),0x2a(a6)                | +044
        move.w  (a0),d0                         | +04a
        jsr     Sub_0006E20C(pc)                | +04c
        move.w  d0,0x28(a6)                     | +050
        clr.w   0x72(a6)                        | +054
        jsr     0x283ca.l                       | +058
        move.w  #0xd000,d0                      | +05e
        jsr     0x28134.l                       | +062
        andi.w  #0xffe3,0x38(a6)                | +068
        ori.w   #0x14,0x38(a6)                  | +06e
        lea     .L06e114(pc),a1                 | +074
        move.l  a1,(a6)                         | +078
.L06e114:
        jsr     0x27d50.l                       | +07a
        bcc.w   .L06e126                        | +080
        lea     0x31c20.l,a1                    | +084
        move.l  a1,(a6)                         | +08a
.L06e126:
        jsr     0x28d70.l                       | +08c
        jsr     Sub_0006E2FE(pc)                | +092
        btst    #0x1,0x13(a6)                   | +096
        beq.w   .L06e140                        | +09c
        lea     Frag_ExplodeB_06da9e(pc),a1     | +0a0
        move.l  a1,(a6)                         | +0a4
.L06e140:
        movea.l #0xffffffff,a0                  | +0a6
        lea     0x2d4046.l,a0                   | +0ac
        jsr     0x5dd56.l                       | +0b2
        bcc.w   SetHandlerRts_06e15c            | +0b8

| ----------------------------------------------------------------------------
|  Sub_0006E15E  @ $06E15E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E15E, "ax", @progbits
        .global Sub_0006E15E
Sub_0006E15E:
        movea.l #0xffffffff,a0                  | +000
        jsr     0x5dd56.l                       | +006
        bcc.w   SetHandlerRts_06e174            | +00c

| ----------------------------------------------------------------------------
|  Sub_0006E176  @ $06E176  (150 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E176, "ax", @progbits
        .global Sub_0006E176
Sub_0006E176:
        lea     0x2b8f74.l,a0                   | +000
        jsr     0x799de.l                       | +006
        andi.w  #0x3,d0                         | +00c
        cmpi.w  #0x2,0x70(a6)                   | +010
        bne.w   .L06e194                        | +016
        move.w  #0x4,d0                         | +01a
.L06e194:
        lsl.w   #0x2,d0                         | +01e
        lea     0x2d4186.l,a0                   | +020
        move.l  (a0,d0.w),0x5c(a6)              | +026
        lea     0x2d4036.l,a0                   | +02c
        jsr     0x5e086.l                       | +032
        bcc.w   .L06e1b8                        | +038
        jsr     0x5e0d4.l                       | +03c
.L06e1b8:
        move.w  0x22(a6),d0                     | +042
        sub.w   0x22(a0),d0                     | +046
        cmpi.w  #0x60,d0                        | +04a
        bgt.w   .L06e1cc                        | +04e
        move.w  #0x60,d0                        | +052
.L06e1cc:
        subi.w  #0x60,d0                        | +056
        lsr.w   #0x4,d0                         | +05a
        cmpi.w  #0x3,d0                         | +05c
        bcs.w   .L06e1de                        | +060
        move.w  #0x3,d0                         | +064
.L06e1de:
        move.w  d0,d1                           | +068
        add.w   d0,d0                           | +06a
        add.w   d1,d0                           | +06c
        add.w   d0,d0                           | +06e
        movea.l 0x5c(a6),a0                     | +070
        adda.l  d0,a0                           | +074
        move.w  (a0),0x28(a6)                   | +076
        move.w  0x2(a0),0x2e(a6)                | +07a
        move.w  0x4(a0),0x2a(a6)                | +080
        btst    #0x0,0x3a(a6)                   | +086
        beq.w   .L06e20a                        | +08c
        neg.w   0x28(a6)                        | +090
.L06e20a:
        rts                                     | +094

| ----------------------------------------------------------------------------
|  Sub_0006E20C  @ $06E20C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E20C, "ax", @progbits
        .global Sub_0006E20C
Sub_0006E20C:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   ClearXN_06e21e                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06e24a  @ $06E24A  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e24a, "ax", @progbits
        .global TaskHandler_06e24a
TaskHandler_06e24a:
        movem.w d0,-(a7)                        | +000
        lea     FireBurst_Wide_06df06(pc),a1    | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        movem.w (a7)+,d0                        | +014
        andi.b  #0x1,d0                         | +018
        move.b  d0,0x3a(a0)                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_06e270  @ $06E270  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e270, "ax", @progbits
        .global TaskHandler_06e270
TaskHandler_06e270:
        movem.w d0,-(a7)                        | +000
        lea     TaskHandler_06e09a(pc),a1       | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        movem.w (a7)+,d0                        | +014
        andi.b  #0x1,d0                         | +018
        move.b  d0,0x3a(a0)                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_06e296  @ $06E296  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e296, "ax", @progbits
        .global TaskHandler_06e296
TaskHandler_06e296:
        movem.w d0,-(a7)                        | +000
        lea     TaskHandler_06dfe8(pc),a1       | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        movem.w (a7)+,d0                        | +014
        andi.b  #0x1,d0                         | +018
        move.b  d0,0x3a(a0)                     | +01c

| ----------------------------------------------------------------------------
|  Sub_0006E2FE  @ $06E2FE  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E2FE, "ax", @progbits
        .global Sub_0006E2FE
Sub_0006E2FE:
        addq.w  #0x1,0x72(a6)                   | +000
        cmpi.w  #0x4,0x72(a6)                   | +004
        ble.w   SetXN_06e318                    | +00a
        jsr     0x283d8.l                       | +00e

| ----------------------------------------------------------------------------
|  Sub_0006E31E  @ $06E31E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E31E, "ax", @progbits
        .global Sub_0006E31E
Sub_0006E31E:
        lea     0x2d4284.l,a0                   | +000
        move.b  0x9b(a6),d0                     | +006
        andi.w  #0x3,d0                         | +00a
        add.w   d0,d0                           | +00e
        move.w  (a0,d0.w),d1                    | +010
        jsr     0x236e.l                        | +014
        move.w  #0x1,d1                         | +01a
        jsr     0x236e.l                        | +01e
        move.w  0x16(a6),0x14(a6)               | +024
        rts                                     | +02a

| ----------------------------------------------------------------------------
|  Sub_0006E34A  @ $06E34A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E34A, "ax", @progbits
        .global Sub_0006E34A
Sub_0006E34A:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x14(a0),0x14(a6)               | +004
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  Sub_0006E356  @ $06E356  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E356, "ax", @progbits
        .global Sub_0006E356
Sub_0006E356:
        move.w  #0x157,d1                       | +000
        jsr     0x236e.l                        | +004
        move.w  #0x158,d1                       | +00a
        jsr     0x236e.l                        | +00e
        move.w  #0x159,d1                       | +014
        jsr     0x236e.l                        | +018
        move.w  0x16(a6),0x14(a6)               | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  TaskHandler_06e37c  @ $06E37C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e37c, "ax", @progbits
        .global TaskHandler_06e37c
TaskHandler_06e37c:
        move.w  #0x47,d1                        | +000
        tst.b   0x9e(a6)                        | +004
        beq.w   JsrAbsThunk_06e38c              | +008
        move.w  #0x144,d1                       | +00c

| ----------------------------------------------------------------------------
|  Sub_0006E394  @ $06E394  (8 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E394, "ax", @progbits
        .global Sub_0006E394
Sub_0006E394:
        tst.b   0x9c(a6)                        | +000
        beq.w   ClrRamWordRts_06e3a2            | +004

| ----------------------------------------------------------------------------
|  TaskHandler_06e3a4  @ $06E3A4  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e3a4, "ax", @progbits
        .global TaskHandler_06e3a4
TaskHandler_06e3a4:
        move.w  #0x1065,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     Frag_Spark_06dc90(pc),a1        | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.w  #0xd000,0x38(a0)                | +01a
        move.w  #0xffec,d0                      | +020
        jsr     Sub_0006E20C(pc)                | +024
        add.w   d0,0x22(a0)                     | +028
        addi.w  #0xc,0x24(a0)                   | +02c
        lea     Frag_Debris_06dbd4__L06dbde(pc),a1 | +032
        jsr     0x4ae.l                         | +036
        jsr     0x5dd02.l                       | +03c
        move.b  0x9f(a6),0x89(a0)               | +042
        move.w  0x70(a6),0x70(a0)               | +048
        move.b  0x9e(a6),0x9e(a0)               | +04e
        move.w  #0xd000,0x38(a0)                | +054
        move.w  #0xffd0,d0                      | +05a
        jsr     Sub_0006E20C(pc)                | +05e
        add.w   d0,0x22(a0)                     | +062
        addi.w  #0x2c,0x24(a0)                  | +066
        rts                                     | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_06e436  @ $06E436  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e436, "ax", @progbits
        .global TaskHandler_06e436
TaskHandler_06e436:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0x8000,0x38(a0)                | +012
        addi.w  #0x10,0x24(a0)                  | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06e456  @ $06E456  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e456, "ax", @progbits
        .global TaskHandler_06e456
TaskHandler_06e456:
        lea     0x7801e.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.w  #0x2000,0x38(a0)                | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_06e470  @ $06E470  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e470, "ax", @progbits
        .global TaskHandler_06e470
TaskHandler_06e470:
        lea     0x2d40a2.l,a1                   | +000
        jsr     0x77c7e.l                       | +006
        move.w  #0x8000,0x38(a0)                | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Sub_0006E484  @ $06E484  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006E484, "ax", @progbits
        .global Sub_0006E484
Sub_0006E484:
        tst.b   0x7e(a6)                        | +000
        beq.w   JsrAbsRts_06e4a6                | +004
        lea     0x2d422a.l,a1                   | +008
        tst.b   0x7f(a6)                        | +00e
        beq.w   JsrAbsThunk_06e4a0              | +012
        lea     0x2d424a.l,a1                   | +016

| ----------------------------------------------------------------------------
|  TaskHandler_06e4a8  @ $06E4A8  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e4a8, "ax", @progbits
        .global TaskHandler_06e4a8
TaskHandler_06e4a8:
        move.w  #0xc,d0                         | +000
.L06e4ac:
        move.w  d0,-(a7)                        | +004
        lea     Frag_Scatter_06dd5c__L06dd86(pc),a1 | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        move.b  0x9b(a6),0x9b(a0)               | +016
        move.w  (a7)+,d0                        | +01c
        subq.w  #0x1,d0                         | +01e
        bne.b   .L06e4ac                        | +020
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06e4cc  @ $06E4CC  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e4cc, "ax", @progbits
        .global TaskHandler_06e4cc
TaskHandler_06e4cc:
        lea     Frag_Smoke_06dc5e(pc),a1        | +000
        jsr     0x6fe.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x9e(a6),0x9e(a0)               | +010
        subq.w  #0x1,0x38(a0)                   | +016
        move.b  0x88(a6),d0                     | +01a
        addq.b  #0x1,d0                         | +01e
        andi.w  #0x3,d0                         | +020
        cmpi.w  #0x3,d0                         | +024
        blt.w   .L06e4fa                        | +028
        clr.w   d0                              | +02c
.L06e4fa:
        move.w  d0,d1                           | +02e
        move.b  d0,0x88(a6)                     | +030
        add.w   d1,d1                           | +034
        addi.w  #0x16,d1                        | +036
        move.w  (a6,d1.w),0x14(a6)              | +03a
        rts                                     | +040

| ----------------------------------------------------------------------------
|  TaskHandler_06e50e  @ $06E50E  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e50e, "ax", @progbits
        .global TaskHandler_06e50e
TaskHandler_06e50e:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06e524                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06e52a  @ $06E52A  (314 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e52a, "ax", @progbits
        .global TaskHandler_06e52a
TaskHandler_06e52a:
        move.w  #0x3b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        lea     0x2d4a4e.l,a0                   | +016
        move.b  0x9a(a6),d0                     | +01c
        andi.w  #0x3,d0                         | +020
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        jsr     0x799de.l                       | +02a
        move.w  d0,0x72(a6)                     | +030
        lea     0x2b91fe.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        move.w  d0,0x66(a6)                     | +040
        move.w  #0xc000,d0                      | +044
        jsr     0x28134.l                       | +048
        andi.w  #0xffe3,0x38(a6)                | +04e
        ori.w   #0x14,0x38(a6)                  | +054
        move.l  #0x2d46c6,0x48(a6)              | +05a
        clr.b   0x20(a6)                        | +062
        jsr     TaskHandler_06eeae(pc)          | +066
        lea     0x723da.l,a1                    | +06a
        jsr     0x4ae.l                         | +070
        jsr     0x5dd02.l                       | +076
        move.w  #0xffe0,0x98(a0)                | +07c
        lea     TaskHandler_06e966(pc),a1       | +082
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        lea     TaskHandler_06e9c0(pc),a1       | +092
        jsr     0x4ae.l                         | +096
        jsr     0x5dd02.l                       | +09c
        move.b  0x99(a6),0x99(a0)               | +0a2
        move.b  0x9b(a6),0x9b(a0)               | +0a8
        move.b  0x9d(a6),0x9d(a0)               | +0ae
        move.b  0x9e(a6),0x9e(a0)               | +0b4
        lea     .L06e5ea(pc),a1                 | +0ba
        move.l  a1,(a6)                         | +0be
.L06e5ea:
        tst.b   0x9d(a6)                        | +0c0
        beq.w   .L06e5f8                        | +0c4
        bset    #0x6,0x13(a6)                   | +0c8
.L06e5f8:
        jsr     TaskHandler_06f094(pc)          | +0ce
        jsr     0x78f8a.l                       | +0d2
        bcc.w   .L06e60c                        | +0d8
        lea     TaskHandler_06e752__L06e770(pc),a1 | +0dc
        move.l  a1,(a6)                         | +0e0
.L06e60c:
        jsr     Stub_0006EF0E(pc)               | +0e2
        jsr     0x28d70.l                       | +0e6
        jsr     0x2870a.l                       | +0ec
        bcc.w   .L06e63e                        | +0f2
        bclr    #0x3,0x13(a6)                   | +0f6
        lea     0x2d4d20.l,a0                   | +0fc
        jsr     0x28cd4.l                       | +102
        lea     0x5e766.l,a0                    | +108
        jsr     0x5e770.l                       | +10e
.L06e63e:
        jsr     0x28758.l                       | +114
        bcc.w   .L06e64e                        | +11a
        lea     TaskHandler_06e66c(pc),a1       | +11e
        move.l  a1,(a6)                         | +122
.L06e64e:
        movea.l #0xffffffff,a0                  | +124
        lea     0x2d49b6.l,a0                   | +12a
        jsr     0x5dd5c.l                       | +130
        bcc.w   SetHandlerRts_06e66a            | +136

| ----------------------------------------------------------------------------
|  TaskHandler_06e66c  @ $06E66C  (222 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e66c, "ax", @progbits
        .global TaskHandler_06e66c
TaskHandler_06e66c:
        move.b  0x9c(a6),d0                     | +000
        cmpi.b  #0x2,d0                         | +004
        bne.w   .L06e682                        | +008
        jsr     0x5e9b6.l                       | +00c
        andi.b  #0x1,d0                         | +012
.L06e682:
        move.b  d0,0x9c(a6)                     | +016
        tst.b   d0                              | +01a
        bne.w   TaskHandler_06e752__L06e770     | +01c
        move.w  #0x10b8,d0                      | +020
        jsr     0x2352.l                        | +024
        move.w  #0xc000,d0                      | +02a
        jsr     0x28134.l                       | +02e
        andi.w  #0xffe3,0x38(a6)                | +034
        ori.w   #0x1c,0x38(a6)                  | +03a
        move.l  #0x2d4816,0x4c(a6)              | +040
        jsr     0x283ca.l                       | +048
        clr.w   0x2a(a6)                        | +04e
        move.w  #0xffe0,0x2e(a6)                | +052
        move.w  0x36(a6),d0                     | +058
        cmpi.w  #0x600,d0                       | +05c
        ble.w   .L06e6d4                        | +060
        move.w  #0x600,d0                       | +064
.L06e6d4:
        btst    #0x0,0x3a(a6)                   | +068
        bne.w   .L06e6e0                        | +06e
        neg.w   d0                              | +072
.L06e6e0:
        move.w  d0,0x28(a6)                     | +074
        lea     0x2d4c9c.l,a0                   | +078
        jsr     0x28cd4.l                       | +07e
        lea     .L06e6f6(pc),a1                 | +084
        move.l  a1,(a6)                         | +088
.L06e6f6:
        jsr     0x27d50.l                       | +08a
        bcc.w   .L06e706                        | +090
        lea     TaskHandler_06e752__L06e770(pc),a1 | +094
        move.l  a1,(a6)                         | +098
.L06e706:
        jsr     Stub_0006EF0E(pc)               | +09a
        jsr     0x28d70.l                       | +09e
        jsr     0x283ca.l                       | +0a4
        jsr     0x283d8.l                       | +0aa
        tst.b   0x9b(a6)                        | +0b0
        beq.w   .L06e734                        | +0b4
        cmpi.w  #0x120,0x24(a6)                 | +0b8
        bgt.w   .L06e734                        | +0be
        lea     TaskHandler_06e752(pc),a1       | +0c2
        move.l  a1,(a6)                         | +0c6
.L06e734:
        movea.l #0xffffffff,a0                  | +0c8
        lea     0x2d49b6.l,a0                   | +0ce
        jsr     0x5dd5c.l                       | +0d4
        bcc.w   SetHandlerRts_06e750            | +0da

| ----------------------------------------------------------------------------
|  TaskHandler_06e752  @ $06E752  (184 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e752, "ax", @progbits
        .global TaskHandler_06e752
TaskHandler_06e752:
        move.w  #0x120,0x24(a6)                 | +000
        move.w  #0xffff,0x38(a6)                | +006
        lea     0x78890.l,a1                    | +00c
        jsr     0x4ae.l                         | +012
        jsr     0x5dd02.l                       | +018
        .global TaskHandler_06e752__L06e770
TaskHandler_06e752__L06e770:
.L06e770:
        tst.b   0x9c(a6)                        | +01e
        bne.w   .L06e786                        | +022
        tst.b   0x9b(a6)                        | +026
        bne.w   .L06e786                        | +02a
        jsr     0x434dc.l                       | +02e
.L06e786:
        move.w  #0x4000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x0,0x38(a6)                   | +044
        move.w  #0x1023,d0                      | +04a
        jsr     0x2352.l                        | +04e
        lea     0x77fd6.l,a1                    | +054
        jsr     0x4ae.l                         | +05a
        jsr     0x5dd02.l                       | +060
        lea     0x2d49d0.l,a1                   | +066
        jsr     0x77c7e.l                       | +06c
        lea     TaskHandler_06edfc(pc),a1       | +072
        jsr     0x4ae.l                         | +076
        jsr     0x5dd02.l                       | +07c
        lea     TaskHandler_06edfc(pc),a1       | +082
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        eori.b  #0x1,0x3a(a0)                   | +092
        lea     TaskHandler_06edfc__L06ee0c(pc),a1 | +098
        jsr     0x4ae.l                         | +09c
        jsr     0x5dd02.l                       | +0a2
        lea     TaskHandler_06edfc__L06ee1c(pc),a1 | +0a8
        jsr     0x4ae.l                         | +0ac
        jsr     0x5dd02.l                       | +0b2

| ----------------------------------------------------------------------------
|  TaskHandler_06e80a  @ $06E80A  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e80a, "ax", @progbits
        .global TaskHandler_06e80a
TaskHandler_06e80a:
        move.b  #0x1,0x20(a6)                   | +000
        jmp     0x518.l                         | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06e816  @ $06E816  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e816, "ax", @progbits
        .global TaskHandler_06e816
TaskHandler_06e816:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06e820  @ $06E820  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e820, "ax", @progbits
        .global TaskHandler_06e820
TaskHandler_06e820:
        move.w  #0x3b,d1                        | +000
        jsr     0x236e.l                        | +004
        clr.b   0x20(a6)                        | +00a
        move.w  #0x10b8,d0                      | +00e
        jsr     0x2352.l                        | +012
        move.w  #0xd000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x18,0x38(a6)                  | +028
        bset    #0x6,0x12(a6)                   | +02e
        move.w  #0xd000,0x34(a6)                | +034
        lea     0x2d4c42.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        move.w  #0x5,0x70(a6)                   | +046
        clr.b   0x7e(a6)                        | +04c
        lea     .L06e876(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L06e876:
        subq.w  #0x1,0x70(a6)                   | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   SetHandlerRts_06e88a            | +060

| ----------------------------------------------------------------------------
|  TaskHandler_06e88c  @ $06E88C  (158 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e88c, "ax", @progbits
        .global TaskHandler_06e88c
TaskHandler_06e88c:
        lea     TaskHandler_06e966(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        bset    #0x6,0x12(a0)                   | +010
        lea     .L06e8a8(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06e8a8:
        movea.l 0xc(a6),a0                      | +01c
        lea     0x2c07ac.l,a1                   | +020
        lea     0x2c072c.l,a2                   | +026
        move.b  0x34(a6),d0                     | +02c
        andi.w  #0xff,d0                        | +030
        lsl.w   #0x1,d0                         | +034
        move.w  (a1,d0.w),d1                    | +036
        move.w  (a2,d0.w),d2                    | +03a
        move.w  0x22(a0),d0                     | +03e
        add.w   d0,d1                           | +042
        move.w  d1,0x22(a6)                     | +044
        addi.w  #0x100,d2                       | +048
        add.w   0x24(a0),d2                     | +04c
        move.w  d2,0x24(a6)                     | +050
        cmpi.w  #0x140,0x22(a6)                 | +054
        bgt.w   .L06e8f0                        | +05a
        jsr     0x28d70.l                       | +05e
.L06e8f0:
        subi.w  #0x80,0x34(a6)                  | +064
        cmpi.b  #0xc0,0x34(a6)                  | +06a
        bne.w   .L06e90a                        | +070
        movea.l 0xc(a6),a0                      | +074
        move.b  #0x1,0x21(a0)                   | +078
.L06e90a:
        move.b  0x34(a6),d0                     | +07e
        andi.w  #0xff,d0                        | +082
        cmpi.w  #0x80,d0                        | +086
        bgt.w   .L06e920                        | +08a
        lea     TaskHandler_06e932(pc),a1       | +08e
        move.l  a1,(a6)                         | +092
.L06e920:
        cmpi.w  #0xffe0,0x22(a6)                | +094
        bgt.w   SetHandlerRts_06e930            | +09a

| ----------------------------------------------------------------------------
|  TaskHandler_06e932  @ $06E932  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e932, "ax", @progbits
        .global TaskHandler_06e932
TaskHandler_06e932:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x2,0x21(a0)                   | +004
        move.w  #0xfe00,0x28(a6)                | +00a
        lea     .L06e948(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06e948:
        jsr     0x27cee.l                       | +016
        jsr     0x28d70.l                       | +01c
        cmpi.w  #0xffe0,0x22(a6)                | +022
        bgt.w   SetHandlerRts_06e964            | +028

| ----------------------------------------------------------------------------
|  TaskHandler_06e966  @ $06E966  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e966, "ax", @progbits
        .global TaskHandler_06e966
TaskHandler_06e966:
        move.w  #0x3b,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x9,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        lea     0x2d4d6e.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L06e98e(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L06e98e:
        jsr     0x5e506.l                       | +028
        btst    #0x0,0x5a(a0)                   | +02e
        beq.w   .L06e9a4                        | +034
        bset    #0x0,0x5a(a6)                   | +038
.L06e9a4:
        movea.l 0xc(a6),a0                      | +03e
        cmpi.b  #0x1,0x20(a0)                   | +042
        bne.w   JsrAbsThunk_06e9b8              | +048
        lea     JmpToScheduler_06e818(pc),a1    | +04c
        move.l  a1,(a6)                         | +050

| ----------------------------------------------------------------------------
|  TaskHandler_06e9c0  @ $06E9C0  (206 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06e9c0, "ax", @progbits
        .global TaskHandler_06e9c0
TaskHandler_06e9c0:
        bset    #0x4,0x6b(a6)                   | +000
        move.b  0x99(a6),d0                     | +006
        cmpi.b  #0x2,d0                         | +00a
        bne.w   .L06e9d8                        | +00e
        jsr     0x5e9b6.l                       | +012
.L06e9d8:
        andi.b  #0x1,d0                         | +018
        move.b  d0,0x99(a6)                     | +01c
        move.w  #0x3c,d1                        | +020
        jsr     0x236e.l                        | +024
        move.w  #0x6,0x1c(a6)                   | +02a
        jsr     0x138fe.l                       | +030
        lea     0x2b950a.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x36(a6)                     | +042
        move.w  #0xd000,d0                      | +046
        jsr     0x28134.l                       | +04a
        andi.w  #0xffe3,0x38(a6)                | +050
        ori.w   #0x1c,0x38(a6)                  | +056
        move.l  #0x2d471a,0x48(a6)              | +05c
        lea     0x2b9488.l,a0                   | +064
        jsr     0x799de.l                       | +06a
        move.w  d0,0x66(a6)                     | +070
        lea     0x2d4e64.l,a0                   | +074
        jsr     0x28cd4.l                       | +07a
        lea     .L06ea46(pc),a1                 | +080
        move.l  a1,(a6)                         | +084
.L06ea46:
        jsr     0x5e506.l                       | +086
        jsr     TaskHandler_06ef2c(pc)          | +08c
        jsr     0x28d70.l                       | +090
        jsr     TaskHandler_06ef66(pc)          | +096
        bcc.w   .L06ea64                        | +09a
        lea     TaskHandler_06ea96(pc),a1       | +09e
        move.l  a1,(a6)                         | +0a2
        .global TaskHandler_06e9c0__L06ea64
TaskHandler_06e9c0__L06ea64:
.L06ea64:
        movea.l 0xc(a6),a0                      | +0a4
        cmpi.b  #0x1,0x20(a0)                   | +0a8
        bne.w   .L06ea78                        | +0ae
        lea     JmpToScheduler_06e818(pc),a1    | +0b2
        move.l  a1,(a6)                         | +0b6
.L06ea78:
        movea.l #0xffffffff,a0                  | +0b8
        lea     0x2d49b6.l,a0                   | +0be
        jsr     0x5dd5c.l                       | +0c4
        bcc.w   SetHandlerRts_06ea94            | +0ca

| ----------------------------------------------------------------------------
|  TaskHandler_06ea96  @ $06EA96  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ea96, "ax", @progbits
        .global TaskHandler_06ea96
TaskHandler_06ea96:
        lea     0x2d4e70.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06eaa8(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06eaa8:
        jsr     0x5e506.l                       | +012
        jsr     TaskHandler_06ef2c(pc)          | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L06ead0                        | +022
        lea     TaskHandler_06ead2(pc),a1       | +026
        move.l  a1,(a6)                         | +02a
        tst.b   0x9e(a6)                        | +02c
        beq.w   .L06ead0                        | +030
        lea     TaskHandler_06ec54(pc),a1       | +034
        move.l  a1,(a6)                         | +038
.L06ead0:
        bra.b   TaskHandler_06e9c0__L06ea64     | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_06ead2  @ $06EAD2  (170 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ead2, "ax", @progbits
        .global TaskHandler_06ead2
TaskHandler_06ead2:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0xff80,0x2e(a6)                | +016
        lea     0x2d4e9a.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L06eb00(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L06eb00:
        jsr     0x27d50.l                       | +02e
        jsr     TaskHandler_06eff0(pc)          | +034
        bcc.w   .L06eb14                        | +038
        lea     Data_06eb84__L06ebd0(pc),a1     | +03c
        move.l  a1,(a6)                         | +040
.L06eb14:
        jsr     0x28d70.l                       | +042
        .global TaskHandler_06ead2__L06eb1a
TaskHandler_06ead2__L06eb1a:
.L06eb1a:
        jsr     0x283d8.l                       | +048
        btst    #0x1,0x13(a6)                   | +04e
        beq.w   .L06eb30                        | +054
        lea     TaskHandler_06ee82__L06ee8a(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L06eb30:
        jsr     0x2870a.l                       | +05e
        bcc.w   .L06eb4c                        | +064
        bclr    #0x3,0x13(a6)                   | +068
        lea     0x5e766.l,a0                    | +06e
        jsr     0x5e770.l                       | +074
.L06eb4c:
        jsr     0x28758.l                       | +07a
        bcc.w   .L06eb66                        | +080
        move.w  #0x1022,d0                      | +084
        jsr     0x2352.l                        | +088
        lea     TaskHandler_06ee82__L06ee8a(pc),a1 | +08e
        move.l  a1,(a6)                         | +092
.L06eb66:
        movea.l #0xffffffff,a0                  | +094
        lea     0x2d49b6.l,a0                   | +09a
        jsr     0x5dd56.l                       | +0a0
        bcc.w   SetHandlerRts_06eb82            | +0a6

| ----------------------------------------------------------------------------
|  Data_06eb84  @ $06EB84  (168 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06eb84, "ax", @progbits
        .global Data_06eb84
Data_06eb84:
        .dc.w   0x8002                        | +000  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +002  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +004  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +008  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +00c  (dato / opcode no decodificado)
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
        .dc.w   0xfff0                        | +022  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +024  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +026  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +028  (dato / opcode no decodificado)
        .dc.w   0x361c                        | +02a  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +02c  (dato / opcode no decodificado)
        .dc.w   0xfff0                        | +02e  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +030  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +032  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +034  (dato / opcode no decodificado)
        .dc.w   0x3626                        | +036  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +038  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +03a  (dato / opcode no decodificado)
        .dc.w   0x0004                        | +03c  (dato / opcode no decodificado)
        .dc.w   0x0200                        | +03e  (dato / opcode no decodificado)
        .dc.w   0x0024                        | +040  (dato / opcode no decodificado)
        .dc.w   0x3630                        | +042  (dato / opcode no decodificado)
        .dc.w   0xffff                        | +044  (dato / opcode no decodificado)
        .dc.w   0x0010                        | +046  (dato / opcode no decodificado)
        .dc.w   0x0000                        | +048  (dato / opcode no decodificado)
        .dc.w   0x1d01                        | +04a  (dato / opcode no decodificado)
        .global Data_06eb84__L06ebd0
Data_06eb84__L06ebd0:
.L06ebd0:
        move.l  #0x6eb84,0x60(a6)               | +04c
        move.w  0x2a(a6),d0                     | +054
        asr.w   #0x4,d0                         | +058
        neg.w   d0                              | +05a
        move.w  d0,0x2e(a6)                     | +05c
        lea     0x2d4ee4.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        lea     .L06ebf6(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L06ebf6:
        tst.b   0x9d(a6)                        | +072
        beq.w   .L06ec04                        | +076
        bset    #0x6,0x13(a6)                   | +07a
.L06ec04:
        jsr     0x27cee.l                       | +080
        jsr     0x28998.l                       | +086
        jsr     TaskHandler_06f020(pc)          | +08c
        bcc.w   .L06ec1e                        | +090
        lea     TaskHandler_06ec2c(pc),a1       | +094
        move.l  a1,(a6)                         | +098
.L06ec1e:
        jsr     TaskHandler_06f050(pc)          | +09a
        jsr     0x28d70.l                       | +09e
        bra.w   TaskHandler_06ead2__L06eb1a     | +0a4

| ----------------------------------------------------------------------------
|  TaskHandler_06ec2c  @ $06EC2C  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ec2c, "ax", @progbits
        .global TaskHandler_06ec2c
TaskHandler_06ec2c:
        clr.w   0x2a(a6)                        | +000
        clr.w   0x2e(a6)                        | +004
        lea     .L06ec3a(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L06ec3a:
        jsr     0x27cee.l                       | +00e
        jsr     0x28998.l                       | +014
        jsr     TaskHandler_06f050(pc)          | +01a
        jsr     0x28d70.l                       | +01e
        bra.w   TaskHandler_06ead2__L06eb1a     | +024

| ----------------------------------------------------------------------------
|  TaskHandler_06ec54  @ $06EC54  (230 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ec54, "ax", @progbits
        .global TaskHandler_06ec54
TaskHandler_06ec54:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0xff80,0x2e(a6)                | +016
        move.l  #0x2d4962,0x4c(a6)              | +01c
        move.l  #0x2d47c2,0x48(a6)              | +024
        move.w  #0x28,d0                        | +02c
        move.w  d0,0x8c(a6)                     | +030
        sub.w   d0,0x24(a6)                     | +034
        lea     0x2d4ea0.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        lea     .L06ec9e(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L06ec9e:
        move.w  0x2a(a6),0x8a(a6)               | +04a
        move.w  0x24(a6),0x86(a6)               | +050
        move.b  0x27(a6),0x89(a6)               | +056
        jsr     0x27bc8.l                       | +05c
        bcc.w   .L06ecde                        | +062
        move.w  #0xff80,0x2e(a6)                | +066
        move.w  0x8a(a6),0x2a(a6)               | +06c
        move.w  0x86(a6),0x24(a6)               | +072
        move.b  0x89(a6),0x27(a6)               | +078
        jsr     0x27cee.l                       | +07e
        lea     TaskHandler_06ed42(pc),a1       | +084
        move.l  a1,(a6)                         | +088
.L06ecde:
        move.w  0x8c(a6),d0                     | +08a
        add.w   d0,0x24(a6)                     | +08e
        jsr     0x28d70.l                       | +092
        move.w  0x8c(a6),d0                     | +098
        sub.w   d0,0x24(a6)                     | +09c
        .global TaskHandler_06ec54__L06ecf4
TaskHandler_06ec54__L06ecf4:
.L06ecf4:
        jsr     0x283d8.l                       | +0a0
        btst    #0x1,0x13(a6)                   | +0a6
        beq.w   .L06ed0a                        | +0ac
        lea     TaskHandler_06ee82(pc),a1       | +0b0
        move.l  a1,(a6)                         | +0b4
.L06ed0a:
        jsr     0x2870a.l                       | +0b6
        bcc.w   .L06ed24                        | +0bc
        move.w  #0x1022,d0                      | +0c0
        jsr     0x2352.l                        | +0c4
        lea     TaskHandler_06ee82(pc),a1       | +0ca
        move.l  a1,(a6)                         | +0ce
.L06ed24:
        movea.l #0xffffffff,a0                  | +0d0
        lea     0x2d49b6.l,a0                   | +0d6
        jsr     0x5dd56.l                       | +0dc
        bcc.w   SetHandlerRts_06ed40            | +0e2

| ----------------------------------------------------------------------------
|  TaskHandler_06ed42  @ $06ED42  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ed42, "ax", @progbits
        .global TaskHandler_06ed42
TaskHandler_06ed42:
        move.w  0x8c(a6),d0                     | +000
        add.w   d0,0x24(a6)                     | +004
        move.l  #0x2d490e,0x4c(a6)              | +008
        move.l  #0x2d476e,0x48(a6)              | +010
        move.w  #0x8,d0                         | +018
        move.w  d0,0x8c(a6)                     | +01c
        sub.w   d0,0x24(a6)                     | +020
        move.w  #0xff80,d1                      | +024
        move.w  d1,0x2e(a6)                     | +028
        lea     0x2d4ee4.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        lea     .L06ed80(pc),a1                 | +038
        move.l  a1,(a6)                         | +03c
.L06ed80:
        tst.b   0x9d(a6)                        | +03e
        beq.w   .L06ed8e                        | +042
        bset    #0x6,0x13(a6)                   | +046
.L06ed8e:
        jsr     0x27bc8.l                       | +04c
        bcc.w   .L06ed9e                        | +052
        lea     TaskHandler_06edbc(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L06ed9e:
        jsr     TaskHandler_06f050(pc)          | +05c
        move.w  0x8c(a6),d0                     | +060
        add.w   d0,0x24(a6)                     | +064
        jsr     0x28d70.l                       | +068
        move.w  0x8c(a6),d0                     | +06e
        sub.w   d0,0x24(a6)                     | +072
        bra.w   TaskHandler_06ec54__L06ecf4     | +076

| ----------------------------------------------------------------------------
|  TaskHandler_06edbc  @ $06EDBC  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06edbc, "ax", @progbits
        .global TaskHandler_06edbc
TaskHandler_06edbc:
        clr.w   0x2e(a6)                        | +000
        clr.w   0x2a(a6)                        | +004
        lea     .L06edca(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L06edca:
        tst.b   0x9d(a6)                        | +00e
        beq.w   .L06edd8                        | +012
        bset    #0x6,0x13(a6)                   | +016
.L06edd8:
        jsr     0x27a92.l                       | +01c
        jsr     TaskHandler_06f050(pc)          | +022
        move.w  0x8c(a6),d0                     | +026
        add.w   d0,0x24(a6)                     | +02a
        jsr     0x28d70.l                       | +02e
        move.w  0x8c(a6),d0                     | +034
        sub.w   d0,0x24(a6)                     | +038
        bra.w   TaskHandler_06ec54__L06ecf4     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_06edfc  @ $06EDFC  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06edfc, "ax", @progbits
        .global TaskHandler_06edfc
TaskHandler_06edfc:
        lea     0x2d4fc4.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L06ee28                        | +00c
        .global TaskHandler_06edfc__L06ee0c
TaskHandler_06edfc__L06ee0c:
.L06ee0c:
        lea     0x2d501a.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L06ee28                        | +01c
        .global TaskHandler_06edfc__L06ee1c
TaskHandler_06edfc__L06ee1c:
.L06ee1c:
        lea     0x2d5070.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
.L06ee28:
        move.w  #0x8000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x0,0x38(a6)                   | +03c
        move.w  #0x3b,d1                        | +042
        jsr     0x236e.l                        | +046
        jmp     0x6dce0.l                       | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_06ee4e  @ $06EE4E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ee4e, "ax", @progbits
        .global TaskHandler_06ee4e
TaskHandler_06ee4e:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd8b6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L06ee6a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06ee6a:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   SetHandlerRts_06ee80            | +028

| ----------------------------------------------------------------------------
|  TaskHandler_06ee82  @ $06EE82  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ee82, "ax", @progbits
        .global TaskHandler_06ee82
TaskHandler_06ee82:
        move.w  0x8c(a6),d0                     | +000
        add.w   d0,0x24(a6)                     | +004
        .global TaskHandler_06ee82__L06ee8a
TaskHandler_06ee82__L06ee8a:
.L06ee8a:
        move.w  #0x4000,d0                      | +008
        jsr     0x28134.l                       | +00c
        andi.w  #0xffe3,0x38(a6)                | +012
        ori.w   #0x1c,0x38(a6)                  | +018
        move.l  #0xffffffff,0x48(a6)            | +01e
        jmp     0x77f6a.l                       | +026

| ----------------------------------------------------------------------------
|  TaskHandler_06eeae  @ $06EEAE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06eeae, "ax", @progbits
        .global TaskHandler_06eeae
TaskHandler_06eeae:
        move.b  0x98(a6),d0                     | +000
        andi.w  #0x7,d0                         | +004
        lsl.w   #0x2,d0                         | +008
        lea     0x2d4a2e.l,a0                   | +00a
        move.l  (a0,d0.w),0x90(a6)              | +010
        clr.w   0x94(a6)                        | +016
        clr.b   0x96(a6)                        | +01a
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06eece  @ $06EECE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06eece, "ax", @progbits
        .global TaskHandler_06eece
TaskHandler_06eece:
        move.w  #0x2,d0                         | +000
        move.w  0x24(a6),d1                     | +004
        cmpi.w  #0x1a0,d1                       | +008
        bgt.w   .L06eeec                        | +00c
        move.w  #0x1,d0                         | +010
        cmpi.w  #0x160,d1                       | +014
        bgt.w   .L06eeec                        | +018
        clr.w   d0                              | +01c
.L06eeec:
        btst    #0x0,0x3a(a6)                   | +01e
        beq.w   .L06eefa                        | +024
        addi.w  #0x3,d0                         | +028
.L06eefa:
        add.w   d0,d0                           | +02c
        lea     0x2d4a22.l,a0                   | +02e
        move.w  (a0,d0.w),d0                    | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06ef10  @ $06EF10  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ef10, "ax", @progbits
        .global TaskHandler_06ef10
TaskHandler_06ef10:
        move.b  0x106f28.l,d0                   | +000
        andi.w  #0x3,d0                         | +006
        add.w   d0,d0                           | +00a
        lea     0x2d4a9e.l,a0                   | +00c
        move.w  (a0,d0.w),d1                    | +012
        add.w   d1,0x24(a6)                     | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_06ef2c  @ $06EF2C  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ef2c, "ax", @progbits
        .global TaskHandler_06ef2c
TaskHandler_06ef2c:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x7d(a0),d1                     | +004
        andi.w  #0x7,d1                         | +008
        lsl.w   #0x3,d1                         | +00c
        lea     0x2d4a5e.l,a1                   | +00e
        move.w  0x4(a1,d1.w),d0                 | +014
        add.w   d0,0x38(a6)                     | +018
        move.w  0x2(a1,d1.w),d0                 | +01c
        add.w   d0,0x24(a6)                     | +020
        move.w  (a1,d1.w),d0                    | +024
        btst    #0x0,0x3a(a6)                   | +028
        beq.w   .L06ef60                        | +02e
        neg.w   d0                              | +032
.L06ef60:
        add.w   d0,0x22(a6)                     | +034
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_06ef66  @ $06EF66  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ef66, "ax", @progbits
        .global TaskHandler_06ef66
TaskHandler_06ef66:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x0,0x7f(a0)                   | +004
        beq.w   SetXN_06ef7a                    | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06ef80  @ $06EF80  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ef80, "ax", @progbits
        .global TaskHandler_06ef80
TaskHandler_06ef80:
        move.w  0x24(a6),d2                     | +000
        addq.w  #0x8,d2                         | +004
.L06ef86:
        subq.w  #0x8,d2                         | +006
        cmpi.w  #0x100,d2                       | +008
        blt.w   .L06efb6                        | +00c
        move.w  0x22(a6),d1                     | +010
        movem.w d2,-(a7)                        | +014
        jsr     0x280c6.l                       | +018
        movem.w (a7)+,d2                        | +01e
        tst.b   d0                              | +022
        beq.b   .L06ef86                        | +024
        andi.b  #0x30,d0                        | +026
        tst.b   d0                              | +02a
        beq.w   .L06efb6                        | +02c
        cmpi.b  #0x20,d0                        | +030
        bne.b   .L06ef86                        | +034
.L06efb6:
        move.w  0x24(a6),d1                     | +036
        sub.w   d2,d1                           | +03a
        cmp.w   0x82(a6),d1                     | +03c
        bgt.w   ClearXN_06efca                  | +040

| ----------------------------------------------------------------------------
|  TaskHandler_06efd0  @ $06EFD0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06efd0, "ax", @progbits
        .global TaskHandler_06efd0
TaskHandler_06efd0:
        cmpi.b  #0x1,0x9b(a6)                   | +000
        beq.w   .L06efdc                        | +006
        move.w  d3,d2                           | +00a
.L06efdc:
        cmp.w   0x24(a6),d2                     | +00c
        bgt.w   SetXN_06efea                    | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06eff0  @ $06EFF0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06eff0, "ax", @progbits
        .global TaskHandler_06eff0
TaskHandler_06eff0:
        move.w  #0x44,d1                        | +000
        move.w  #0x190,d2                       | +004
        move.w  #0x180,d3                       | +008
        cmpi.b  #0x0,0x99(a6)                   | +00c
        beq.w   .L06f012                        | +012
        move.w  #0x28,d1                        | +016
        move.w  #0x168,d2                       | +01a
        move.w  #0x158,d3                       | +01e
.L06f012:
        tst.b   0x9b(a6)                        | +022
        bne.b   TaskHandler_06efd0              | +026
        move.w  d1,0x82(a6)                     | +028
        bra.w   TaskHandler_06ef80              | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_06f020  @ $06F020  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f020, "ax", @progbits
        .global TaskHandler_06f020
TaskHandler_06f020:
        move.w  #0x2c,d1                        | +000
        move.w  #0x178,d2                       | +004
        move.w  #0x168,d3                       | +008
        cmpi.b  #0x0,0x99(a6)                   | +00c
        beq.w   .L06f042                        | +012
        move.w  #0x10,d1                        | +016
        move.w  #0x150,d2                       | +01a
        move.w  #0x140,d3                       | +01e
.L06f042:
        tst.b   0x9b(a6)                        | +022
        bne.b   TaskHandler_06efd0              | +026
        move.w  d1,0x82(a6)                     | +028
        bra.w   TaskHandler_06ef80              | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_06f050  @ $06F050  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f050, "ax", @progbits
        .global TaskHandler_06f050
TaskHandler_06f050:
        move.w  0x28(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L06f060                        | +00a
        neg.w   d0                              | +00e
.L06f060:
        cmp.w   0x36(a6),d0                     | +010
        bcs.w   .L06f080                        | +014
        move.w  0x36(a6),d0                     | +018
        btst    #0x0,0x3a(a6)                   | +01c
        bne.w   .L06f078                        | +022
        neg.w   d0                              | +026
.L06f078:
        move.w  d0,0x28(a6)                     | +028
        clr.w   0x2c(a6)                        | +02c
.L06f080:
        cmpi.w  #0x0,0x2a(a6)                   | +030
        blt.w   .L06f092                        | +036
        clr.w   0x2e(a6)                        | +03a
        clr.w   0x2a(a6)                        | +03e
.L06f092:
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_06f094  @ $06F094  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f094, "ax", @progbits
        .global TaskHandler_06f094
TaskHandler_06f094:
        lea     0x2d4aa6.l,a0                   | +000
        move.b  0x34(a6),d0                     | +006
        andi.w  #0xff,d0                        | +00a
        lsr.w   #0x3,d0                         | +00e
        lsl.w   #0x1,d0                         | +010
        move.w  (a0,d0.w),d0                    | +012
        move.w  0x72(a6),d1                     | +016
        muls.w  d1,d0                           | +01a
        asr.l   #0x8,d0                         | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_06f0b8  @ $06F0B8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0b8, "ax", @progbits
        .global TaskHandler_06f0b8
TaskHandler_06f0b8:
        lea     0x2d4c42.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f0c6  @ $06F0C6  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0c6, "ax", @progbits
        .global TaskHandler_06f0c6
TaskHandler_06f0c6:
        lea     0x2d4c52.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f0d4  @ $06F0D4  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0d4, "ax", @progbits
        .global TaskHandler_06f0d4
TaskHandler_06f0d4:
        lea     0x2d4c7e.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f0e2  @ $06F0E2  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0e2, "ax", @progbits
        .global TaskHandler_06f0e2
TaskHandler_06f0e2:
        lea     0x2d4c9c.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f0f0  @ $06F0F0  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0f0, "ax", @progbits
        .global TaskHandler_06f0f0
TaskHandler_06f0f0:
        lea     0x2d4cc8.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f0fe  @ $06F0FE  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f0fe, "ax", @progbits
        .global TaskHandler_06f0fe
TaskHandler_06f0fe:
        lea     0x2d4ce6.l,a0                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06f10c  @ $06F10C  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f10c, "ax", @progbits
        .global TaskHandler_06f10c
TaskHandler_06f10c:
        move.w  0x36(a6),d0                     | +000
        asr.w   #0x7,d0                         | +004
        btst    #0x0,0x3a(a6)                   | +006
        bne.w   SetTaskW_06f11e                 | +00c
        neg.w   d0                              | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06f124  @ $06F124  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f124, "ax", @progbits
        .global TaskHandler_06f124
TaskHandler_06f124:
        move.w  0x36(a6),d0                     | +000
        asr.w   #0x4,d0                         | +004
        btst    #0x0,0x3a(a6)                   | +006
        bne.w   SetTaskW_06f136                 | +00c
        neg.w   d0                              | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06f13c  @ $06F13C  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f13c, "ax", @progbits
        .global TaskHandler_06f13c
TaskHandler_06f13c:
        lea     TaskHandler_06ee4e(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x10,d0                        | +010
        btst    #0x0,0x3a(a6)                   | +014
        beq.w   .L06f15c                        | +01a
        neg.w   d0                              | +01e
.L06f15c:
        add.w   d0,0x22(a0)                     | +020
        rts                                     | +024

| ----------------------------------------------------------------------------
|  TaskHandler_06f162  @ $06F162  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f162, "ax", @progbits
        .global TaskHandler_06f162
TaskHandler_06f162:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06f178                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06f17e  @ $06F17E  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f17e, "ax", @progbits
        .global TaskHandler_06f17e
TaskHandler_06f17e:
        move.w  #0x10b5,d0                      | +000
        jsr     0x2352.l                        | +004
        ori.b   #0x1,0x3a(a6)                   | +00a
        move.w  #0xfc00,0x28(a6)                | +010
        bra.w   .L06f1ac                        | +016
        move.w  #0x10c3,d0                      | +01a
        jsr     0x2352.l                        | +01e
        clr.b   0x3a(a6)                        | +024
        move.w  0x400.w,0x28(a6)                | +028
.L06f1ac:
        clr.w   0x38(a6)                        | +02e
        move.w  #0x12b,d1                       | +032
        jsr     0x236e.l                        | +036
        lea     0x2d889e.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L06f1cc(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L06f1cc:
        jsr     0x27cee.l                       | +04e
        jsr     0x28d70.l                       | +054
        movea.l #0xffffffff,a0                  | +05a
        lea     0x2d531e.l,a0                   | +060
        jsr     0x5dd5c.l                       | +066
        bcc.w   SetHandlerRts_06f1f4            | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_06f1f6  @ $06F1F6  (322 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f1f6, "ax", @progbits
        .global TaskHandler_06f1f6
TaskHandler_06f1f6:
        move.w  #0x59,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xb,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        move.l  #0x2d50c6,0x48(a6)              | +016
        move.w  #0x4000,d0                      | +01e
        jsr     0x28134.l                       | +022
        andi.w  #0xffe3,0x38(a6)                | +028
        ori.w   #0x14,0x38(a6)                  | +02e
        lea     0x2b9b22.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        lea     0x2b9ba4.l,a0                   | +040
        lsl.w   #0x3,d0                         | +046
        move.w  (a0,d0.w),0x66(a6)              | +048
        move.w  0x2(a0,d0.w),0x7a(a6)           | +04e
        move.w  0x4(a0,d0.w),0x7c(a6)           | +054
        clr.b   0x73(a6)                        | +05a
        move.b  #0xff,0x8e(a6)                  | +05e
        move.b  #0x0,0x8b(a6)                   | +064
        lea     TaskHandler_06fba0(pc),a1       | +06a
        jsr     0x4ae.l                         | +06e
        jsr     0x5dd02.l                       | +074
        lea     TaskHandler_06fb10(pc),a1       | +07a
        jsr     0x4ae.l                         | +07e
        jsr     0x5dd02.l                       | +084
        clr.b   0x98(a0)                        | +08a
        lea     TaskHandler_06fb10(pc),a1       | +08e
        jsr     0x4ae.l                         | +092
        jsr     0x5dd02.l                       | +098
        move.b  #0x1,0x98(a0)                   | +09e
        lea     TaskHandler_06fb10(pc),a1       | +0a4
        jsr     0x4ae.l                         | +0a8
        jsr     0x5dd02.l                       | +0ae
        move.b  #0x2,0x98(a0)                   | +0b4
        clr.b   0x9b(a6)                        | +0ba
        lea     0x682d0.l,a1                    | +0be
        jsr     0x4ae.l                         | +0c4
        jsr     0x5dd02.l                       | +0ca
        lea     0x683f0.l,a1                    | +0d0
        jsr     0x4ae.l                         | +0d6
        jsr     0x5dd02.l                       | +0dc
        move.w  #0x200,0x36(a6)                 | +0e2
        clr.b   0x21(a6)                        | +0e8
        clr.w   0x22(a6)                        | +0ec
        move.w  #0x160,0x24(a6)                 | +0f0
        move.b  #0x1,0x8b(a6)                   | +0f6
        move.w  #0x600,0x28(a6)                 | +0fc
        move.b  #0xff,0x9a(a6)                  | +102
        move.w  #0x10b9,d0                      | +108
        jsr     0x2352.l                        | +10c
        jsr     TaskHandler_070cbe(pc)          | +112
        lea     0x2d5e02.l,a0                   | +116
        jsr     0x28cd4.l                       | +11c
        lea     .L06f31e(pc),a1                 | +122
        move.l  a1,(a6)                         | +126
.L06f31e:
        jsr     0x27cee.l                       | +128
        jsr     0x27cee.l                       | +12e
        jsr     PcThunkTarget_070ab0(pc)        | +134
        cmpi.w  #0x198,0x22(a6)                 | +138
        blt.w   SetHandlerRts_06f33e            | +13e

| ----------------------------------------------------------------------------
|  TaskHandler_06f340  @ $06F340  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f340, "ax", @progbits
        .global TaskHandler_06f340
TaskHandler_06f340:
        clr.b   0x9a(a6)                        | +000
        move.w  #0x78,0x70(a6)                  | +004
        move.w  #0x198,0x22(a6)                 | +00a
        move.w  #0x140,0x24(a6)                 | +010
        move.b  #0x1,0x8b(a6)                   | +016
        jsr     0x267e2.l                       | +01c
        move.b  #0x2,0x8a(a6)                   | +022
        lea     .L06f36e(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L06f36e:
        tst.b   0x21(a6)                        | +02e
        beq.w   SetHandlerRts_06f38a            | +032
        subq.w  #0x1,0x70(a6)                   | +036
        cmpi.w  #0x0,0x70(a6)                   | +03a
        bgt.w   SetHandlerRts_06f38a            | +040

| ----------------------------------------------------------------------------
|  TaskHandler_06f38c  @ $06F38C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f38c, "ax", @progbits
        .global TaskHandler_06f38c
TaskHandler_06f38c:
        move.w  #0x10ba,d0                      | +000
        jsr     0x2352.l                        | +004

| ----------------------------------------------------------------------------
|  TaskHandler_06f396  @ $06F396  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f396, "ax", @progbits
        .global TaskHandler_06f396
TaskHandler_06f396:
        move.w  #0xa0,0x80(a6)                  | +000
        move.w  #0x190,0x82(a6)                 | +006
        lea     0x2d53f2.l,a0                   | +00c
        move.b  0x73(a6),d0                     | +012
        andi.w  #0x3,d0                         | +016
        lsl.w   #0x2,d0                         | +01a
        movea.l (a0,d0.w),a0                    | +01c
        jsr     0x799de.l                       | +020
        move.w  d0,0x70(a6)                     | +026
        move.w  #0x200,0x36(a6)                 | +02a
        move.b  #0x0,0x8c(a6)                   | +030
        move.w  #0x258,0x94(a6)                 | +036
        lea     0x2d55ce.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L06f3e4(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L06f3e4:
        jsr     0x28998.l                       | +04e
        jsr     TaskHandler_070ad8(pc)          | +054
        bcc.w   .L06f3f8                        | +058
        move.b  #0x4,0x8c(a6)                   | +05c
.L06f3f8:
        jsr     PcThunkTarget_070ab0(pc)        | +062
        jsr     TaskHandler_070c02(pc)          | +066
        lea     0x2d53d2.l,a0                   | +06a
        movea.l #0xffffffff,a1                  | +070
        jsr     0x772.l                         | +076
        bra.w   TaskHandler_06fac2              | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_06f416  @ $06F416  (264 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f416, "ax", @progbits
        .global TaskHandler_06f416
TaskHandler_06f416:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0x3,d0                         | +006
        move.b  d0,0x88(a6)                     | +00a
        .global TaskHandler_06f416__L06f424
TaskHandler_06f416__L06f424:
.L06f424:
        jsr     0x5e1ea.l                       | +00e
        move.l  a0,0x9c(a6)                     | +014
        clr.b   d0                              | +018
        lea     0x2d547e.l,a1                   | +01a
        move.w  0x22(a0),d3                     | +020
        cmp.w   0x22(a6),d3                     | +024
        blt.w   .L06f44c                        | +028
        move.b  #0x1,d0                         | +02c
        lea     0x2d549e.l,a1                   | +030
.L06f44c:
        move.b  d0,0x87(a6)                     | +036
        move.b  0x88(a6),d0                     | +03a
        cmpi.b  #0x3,d0                         | +03e
        beq.w   .L06f472                        | +042
        cmpi.b  #0x4,d0                         | +046
        beq.w   .L06f472                        | +04a
        cmpi.w  #0x164,0x24(a0)                 | +04e
        blt.w   .L06f472                        | +054
        addi.b  #0x5,d0                         | +058
.L06f472:
        andi.w  #0x7,d0                         | +05c
        lsl.w   #0x2,d0                         | +060
        move.w  (a1,d0.w),0x80(a6)              | +062
        move.w  0x2(a1,d0.w),0x82(a6)           | +068
        move.b  0x88(a6),d0                     | +06e
        cmpi.b  #0x3,d0                         | +072
        beq.w   .L06f4ac                        | +076
        cmpi.b  #0x4,d0                         | +07a
        beq.w   .L06f4ac                        | +07e
        jsr     0x5e9b6.l                       | +082
        btst    #0x0,d0                         | +088
        beq.w   .L06f4ac                        | +08c
        addi.w  #0x28,0x82(a6)                  | +090
.L06f4ac:
        move.w  #0x1074,d0                      | +096
        jsr     0x2352.l                        | +09a
        move.b  #0x1,0x8c(a6)                   | +0a0
        move.w  #0x258,0x94(a6)                 | +0a6
        move.w  #0x200,0x36(a6)                 | +0ac
        lea     0x2d55da.l,a0                   | +0b2
        jsr     0x28cd4.l                       | +0b8
        tst.b   0x9b(a6)                        | +0be
        beq.w   .L06f4f2                        | +0c2
        addq.b  #0x2,0x87(a6)                   | +0c6
        move.w  #0x400,0x36(a6)                 | +0ca
        lea     0x2d5666.l,a0                   | +0d0
        jsr     0x28cd4.l                       | +0d6
.L06f4f2:
        lea     .L06f4f8(pc),a1                 | +0dc
        move.l  a1,(a6)                         | +0e0
.L06f4f8:
        jsr     0x28998.l                       | +0e2
        jsr     TaskHandler_070ad8(pc)          | +0e8
        bcc.w   .L06f50c                        | +0ec
        move.b  #0x3,0x8c(a6)                   | +0f0
.L06f50c:
        jsr     PcThunkTarget_070ab0(pc)        | +0f6
        bcc.w   .L06f51a                        | +0fa
        lea     TaskHandler_06f51e(pc),a1       | +0fe
        move.l  a1,(a6)                         | +102
.L06f51a:
        bra.w   TaskHandler_06fac2              | +104

| ----------------------------------------------------------------------------
|  TaskHandler_06f51e  @ $06F51E  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f51e, "ax", @progbits
        .global TaskHandler_06f51e
TaskHandler_06f51e:
        move.w  #0x100,0x36(a6)                 | +000
        move.b  #0x2,0x8c(a6)                   | +006
        move.w  #0x258,0x94(a6)                 | +00c
        move.b  0x87(a6),d0                     | +012
        andi.w  #0x1,d0                         | +016
        tst.b   0x9b(a6)                        | +01a
        beq.w   .L06f542                        | +01e
        addq.w  #0x2,d0                         | +022
.L06f542:
        movea.l #0x2d53b2,a0                    | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06f55e                        | +036
        jsr     0x28cd4.l                       | +03a
.L06f55e:
        lea     .L06f564(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L06f564:
        jsr     0x28998.l                       | +046
        jsr     TaskHandler_070ad8(pc)          | +04c
        bcc.w   .L06f578                        | +050
        move.b  #0x3,0x8c(a6)                   | +054
.L06f578:
        jsr     PcThunkTarget_070ab0(pc)        | +05a
        bcc.w   .L06f58c                        | +05e
        move.b  #0x1,0x9b(a6)                   | +062
        lea     TaskHandler_06f396(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L06f58c:
        bra.w   TaskHandler_06fac2              | +06e
        move.b  #0x4,0x88(a6)                   | +072
        bra.w   TaskHandler_06f416__L06f424     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_06f59a  @ $06F59A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f59a, "ax", @progbits
        .global TaskHandler_06f59a
TaskHandler_06f59a:
        move.b  #0xff,0x20(a6)                  | +000
        jsr     TaskHandler_070cfe(pc)          | +006
        move.b  #0x0,0x8c(a6)                   | +00a
        move.w  #0x258,0x94(a6)                 | +010
        move.w  #0x1075,d0                      | +016
        jsr     0x2352.l                        | +01a
        lea     0x2d5e7a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L06f5cc(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L06f5cc:
        jsr     0x28998.l                       | +032
        jsr     TaskHandler_070ad8(pc)          | +038
        bcc.w   .L06f5e0                        | +03c
        move.b  #0x4,0x8c(a6)                   | +040
.L06f5e0:
        jsr     PcThunkTarget_070ab0(pc)        | +046
        bcc.w   .L06f5ee                        | +04a
        lea     TaskHandler_06f5f2(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L06f5ee:
        bra.w   TaskHandler_06fac2              | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06f5f2  @ $06F5F2  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f5f2, "ax", @progbits
        .global TaskHandler_06f5f2
TaskHandler_06f5f2:
        move.w  #0xb4,0x70(a6)                  | +000
        lea     .L06f5fe(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L06f5fe:
        jsr     0x28998.l                       | +00c
        jsr     TaskHandler_070ad8(pc)          | +012
        bcc.w   .L06f612                        | +016
        move.b  #0x4,0x8c(a6)                   | +01a
.L06f612:
        jsr     PcThunkTarget_070ab0(pc)        | +020
        subq.w  #0x1,0x70(a6)                   | +024
        cmpi.w  #0x0,0x70(a6)                   | +028
        bgt.w   .L06f62a                        | +02e
        lea     TaskHandler_06f63c(pc),a1       | +032
        move.l  a1,(a6)                         | +036
.L06f62a:
        tst.b   0x20(a6)                        | +038
        bne.w   .L06f638                        | +03c
        lea     TaskHandler_06f63c(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L06f638:
        bra.w   TaskHandler_06fac2              | +046

| ----------------------------------------------------------------------------
|  TaskHandler_06f63c  @ $06F63C  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f63c, "ax", @progbits
        .global TaskHandler_06f63c
TaskHandler_06f63c:
        move.w  #0x1075,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2d6002.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L06f658(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06f658:
        jsr     0x28998.l                       | +01c
        jsr     TaskHandler_070ad8(pc)          | +022
        bcc.w   .L06f66c                        | +026
        move.b  #0x4,0x8c(a6)                   | +02a
.L06f66c:
        jsr     PcThunkTarget_070ab0(pc)        | +030
        bcc.w   .L06f67a                        | +034
        lea     TaskHandler_06f396(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L06f67a:
        bra.w   TaskHandler_06fac2              | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_06f67e  @ $06F67E  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f67e, "ax", @progbits
        .global TaskHandler_06f67e
TaskHandler_06f67e:
        jsr     TaskHandler_070cde(pc)          | +000
        move.w  #0x12c,0x70(a6)                 | +004
        move.w  #0x200,0x36(a6)                 | +00a
        move.b  #0x0,0x8c(a6)                   | +010
        move.w  #0x258,0x94(a6)                 | +016
        lea     0x2d55ce.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L06f6ac(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L06f6ac:
        jsr     0x28998.l                       | +02e
        jsr     TaskHandler_070ad8(pc)          | +034
        bcc.w   .L06f6c0                        | +038
        lea     TaskHandler_06f6dc(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L06f6c0:
        jsr     PcThunkTarget_070ab0(pc)        | +042
        subq.w  #0x1,0x70(a6)                   | +046
        cmpi.w  #0x0,0x70(a6)                   | +04a
        bgt.w   .L06f6d8                        | +050
        lea     TaskHandler_06f6dc(pc),a1       | +054
        move.l  a1,(a6)                         | +058
.L06f6d8:
        bra.w   TaskHandler_06fac2              | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_06f6dc  @ $06F6DC  (220 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f6dc, "ax", @progbits
        .global TaskHandler_06f6dc
TaskHandler_06f6dc:
        jsr     0x5e896.l                       | +000
        bcs.w   .L06f70a                        | +006
        lea     0x2ba37e.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x72(a6)                     | +016
        lea     0x2ba410.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x70(a6)                     | +026
        bra.w   .L06f72a                        | +02a
.L06f70a:
        lea     0x2ba492.l,a0                   | +02e
        jsr     0x799de.l                       | +034
        move.b  d0,0x72(a6)                     | +03a
        lea     0x2ba514.l,a0                   | +03e
        jsr     0x799de.l                       | +044
        move.w  d0,0x70(a6)                     | +04a
.L06f72a:
        andi.b  #0x7f,0x72(a6)                  | +04e
        bne.w   .L06f73a                        | +054
        move.b  #0x1,0x72(a6)                   | +058
.L06f73a:
        move.b  #0x3,0x8c(a6)                   | +05e
        lea     .L06f746(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L06f746:
        jsr     0x28998.l                       | +06a
        jsr     TaskHandler_070ad8(pc)          | +070
        jsr     PcThunkTarget_070ab0(pc)        | +074
        subq.w  #0x1,0x70(a6)                   | +078
        cmpi.w  #0x0,0x70(a6)                   | +07c
        bgt.w   .L06f7a4                        | +082
        lea     0x2ba410.l,a0                   | +086
        jsr     0x799de.l                       | +08c
        move.w  d0,0x70(a6)                     | +092
        jsr     0x5e896.l                       | +096
        bcc.w   .L06f78c                        | +09c
        lea     0x2ba514.l,a0                   | +0a0
        jsr     0x799de.l                       | +0a6
        move.w  d0,0x70(a6)                     | +0ac
.L06f78c:
        jsr     TaskHandler_070ea2(pc)          | +0b0
        subq.b  #0x1,0x72(a6)                   | +0b4
        cmpi.b  #0x0,0x72(a6)                   | +0b8
        bgt.w   .L06f7a4                        | +0be
        lea     TaskHandler_06f7b8(pc),a1       | +0c2
        move.l  a1,(a6)                         | +0c6
.L06f7a4:
        btst    #0x0,0x13(a6)                   | +0c8
        beq.w   .L06f7b4                        | +0ce
        lea     TaskHandler_06f7b8(pc),a1       | +0d2
        move.l  a1,(a6)                         | +0d6
.L06f7b4:
        bra.w   TaskHandler_06fac2              | +0d8

| ----------------------------------------------------------------------------
|  TaskHandler_06f7b8  @ $06F7B8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f7b8, "ax", @progbits
        .global TaskHandler_06f7b8
TaskHandler_06f7b8:
        move.b  #0x4,0x8c(a6)                   | +000
        move.w  #0x1e,0x70(a6)                  | +006
        lea     .L06f7ca(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06f7ca:
        jsr     0x28998.l                       | +012
        jsr     TaskHandler_070ad8(pc)          | +018
        jsr     PcThunkTarget_070ab0(pc)        | +01c
        subq.w  #0x1,0x70(a6)                   | +020
        cmpi.w  #0x0,0x70(a6)                   | +024
        bgt.w   .L06f7ec                        | +02a
        lea     TaskHandler_06f396(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L06f7ec:
        bra.w   TaskHandler_06fac2              | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06f7f0  @ $06F7F0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f7f0, "ax", @progbits
        .global TaskHandler_06f7f0
TaskHandler_06f7f0:
        jsr     TaskHandler_070cfe(pc)          | +000
        move.b  #0x0,0x8c(a6)                   | +004
        move.w  #0x258,0x94(a6)                 | +00a
        move.w  #0x1075,d0                      | +010
        jsr     0x2352.l                        | +014
        lea     0x2d5a78.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06f81c(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06f81c:
        jsr     0x28998.l                       | +02c
        jsr     TaskHandler_070ad8(pc)          | +032
        bcc.w   .L06f830                        | +036
        move.b  #0x4,0x8c(a6)                   | +03a
.L06f830:
        jsr     PcThunkTarget_070ab0(pc)        | +040
        bcc.w   .L06f83e                        | +044
        lea     TaskHandler_06f842(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06f83e:
        bra.w   TaskHandler_06fac2              | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_06f842  @ $06F842  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f842, "ax", @progbits
        .global TaskHandler_06f842
TaskHandler_06f842:
        lea     0x2b9d28.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x72(a6)                     | +00c
        move.w  #0x2d,0x70(a6)                  | +010
        lea     .L06f85e(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06f85e:
        jsr     0x28998.l                       | +01c
        jsr     TaskHandler_070ad8(pc)          | +022
        bcc.w   .L06f872                        | +026
        move.b  #0x4,0x8c(a6)                   | +02a
.L06f872:
        jsr     PcThunkTarget_070ab0(pc)        | +030
        cmpi.b  #0x1,0x8a(a6)                   | +034
        bne.w   .L06f88e                        | +03a
        cmpi.b  #0x0,0x72(a6)                   | +03e
        ble.w   .L06f88e                        | +044
        jsr     TaskHandler_070e5a(pc)          | +048
.L06f88e:
        cmpi.b  #0x0,0x72(a6)                   | +04c
        bgt.w   .L06f8ac                        | +052
        subq.w  #0x1,0x70(a6)                   | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   .L06f8ac                        | +060
        lea     TaskHandler_06f8c0(pc),a1       | +064
        move.l  a1,(a6)                         | +068
.L06f8ac:
        btst    #0x0,0x13(a6)                   | +06a
        beq.w   .L06f8bc                        | +070
        lea     TaskHandler_06f8c0(pc),a1       | +074
        move.l  a1,(a6)                         | +078
.L06f8bc:
        bra.w   TaskHandler_06fac2              | +07a

| ----------------------------------------------------------------------------
|  TaskHandler_06f8c0  @ $06F8C0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f8c0, "ax", @progbits
        .global TaskHandler_06f8c0
TaskHandler_06f8c0:
        move.w  #0x1075,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2d5c00.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L06f8dc(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06f8dc:
        jsr     0x28998.l                       | +01c
        jsr     TaskHandler_070ad8(pc)          | +022
        bcc.w   .L06f8f0                        | +026
        move.b  #0x4,0x8c(a6)                   | +02a
.L06f8f0:
        jsr     PcThunkTarget_070ab0(pc)        | +030
        bcc.w   .L06f8fe                        | +034
        lea     TaskHandler_06f396(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L06f8fe:
        bra.w   TaskHandler_06fac2              | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_06f902  @ $06F902  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f902, "ax", @progbits
        .global TaskHandler_06f902
TaskHandler_06f902:
        jsr     TaskHandler_070cbe(pc)          | +000
        move.b  #0x0,0x8c(a6)                   | +004
        move.w  #0x258,0x94(a6)                 | +00a
        move.w  #0x1075,d0                      | +010
        jsr     0x2352.l                        | +014
        lea     0x2d5cf2.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06f92e(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06f92e:
        jsr     0x28998.l                       | +02c
        jsr     TaskHandler_070ad8(pc)          | +032
        bcc.w   .L06f942                        | +036
        move.b  #0x4,0x8c(a6)                   | +03a
.L06f942:
        jsr     PcThunkTarget_070ab0(pc)        | +040
        bcc.w   .L06f950                        | +044
        lea     TaskHandler_06f954(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06f950:
        bra.w   TaskHandler_06fac2              | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_06f954  @ $06F954  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f954, "ax", @progbits
        .global TaskHandler_06f954
TaskHandler_06f954:
        move.b  #0x7f,0x72(a6)                  | +000
        move.w  #0x2d,0x70(a6)                  | +006
        move.b  #0x0,0x8c(a6)                   | +00c
        move.w  #0x258,0x94(a6)                 | +012
        lea     .L06f972(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L06f972:
        jsr     0x28998.l                       | +01e
        jsr     TaskHandler_070ad8(pc)          | +024
        bcc.w   .L06f990                        | +028
        move.b  #0x0,0x8c(a6)                   | +02c
        move.w  #0x258,0x94(a6)                 | +032
        jsr     TaskHandler_070cbe(pc)          | +038
.L06f990:
        jsr     PcThunkTarget_070ab0(pc)        | +03c
        cmpi.b  #0x2,0x73(a6)                   | +040
        blt.w   .L06f9aa                        | +046
        move.l  #0x2d50c6,0x48(a6)              | +04a
        bra.w   .L06f9c6                        | +052
.L06f9aa:
        tst.b   0x72(a6)                        | +056
        bne.w   .L06f9c6                        | +05a
        subq.w  #0x1,0x70(a6)                   | +05e
        cmpi.w  #0x0,0x70(a6)                   | +062
        bgt.w   .L06f9c6                        | +068
        lea     TaskHandler_06f8c0(pc),a1       | +06c
        move.l  a1,(a6)                         | +070
.L06f9c6:
        bra.w   TaskHandler_06fae4              | +072

| ----------------------------------------------------------------------------
|  TaskHandler_06f9ca  @ $06F9CA  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06f9ca, "ax", @progbits
        .global TaskHandler_06f9ca
TaskHandler_06f9ca:
        clr.b   0x106ed3.l                      | +000
        move.w  #0x200,0x28(a6)                 | +006
        move.w  #0xff80,0x2a(a6)                | +00c
        move.b  #0x3,0x73(a6)                   | +012
        lea     0x2d544c.l,a1                   | +018
        jsr     0x77c7e.l                       | +01e
        move.w  #0xc000,0x38(a0)                | +024
        lea     0x2d60f6.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L06fa06(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L06fa06:
        bset    #0x6,0x13(a6)                   | +03c
        jsr     0x27cee.l                       | +042
        bcc.w   .L06fa1a                        | +048
        clr.w   0x2a(a6)                        | +04c
.L06fa1a:
        cmpi.w  #0x160,0x22(a6)                 | +050
        blt.w   .L06fa30                        | +056
        jsr     0x267e2.l                       | +05a
        lea     TaskHandler_06fa60(pc),a1       | +060
        move.l  a1,(a6)                         | +064
.L06fa30:
        cmpi.w  #0x140,0x24(a6)                 | +066
        bgt.w   .L06fa40                        | +06c
        move.w  #0x40,0x2a(a6)                  | +070
.L06fa40:
        cmpi.w  #0x170,0x24(a6)                 | +076
        blt.w   .L06fa50                        | +07c
        move.w  #0xffc0,0x2a(a6)                | +080
.L06fa50:
        jsr     PcThunkTarget_070ab0(pc)        | +086
        bcc.w   SetHandlerRts_06fa5e            | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_06fa60  @ $06FA60  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fa60, "ax", @progbits
        .global TaskHandler_06fa60
TaskHandler_06fa60:
        move.w  0x22(a6),0x5c(a6)               | +000
        move.w  #0x120,0x22(a6)                 | +006
        move.w  #0x1033,d0                      | +00c
        jsr     0x2352.l                        | +010
        move.b  #0x4,0x10a2d0.l                 | +016
        move.b  #0x0,0x10a2d1.l                 | +01e
        lea     0x77fd6.l,a1                    | +026
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        addq.w  #0x1,0x38(a0)                   | +038
        jsr     0x628ea.l                       | +03c
        jsr     0x628ea.l                       | +042
        move.w  0x5c(a6),0x22(a6)               | +048
        lea     JmpToScheduler_06faba(pc),a1    | +04e
        move.l  a1,(a6)                         | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06fac2  @ $06FAC2  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fac2, "ax", @progbits
        .global TaskHandler_06fac2
TaskHandler_06fac2:
        jsr     TaskHandler_070c4e(pc)          | +000
        jsr     0x28758.l                       | +004
        bcc.w   .L06fae2                        | +00a
        bclr    #0x0,0x13(a6)                   | +00e
        move.w  #0x1,0x66(a6)                   | +014
        move.b  #0x2,0x73(a6)                   | +01a
.L06fae2:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TaskHandler_06fae4  @ $06FAE4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fae4, "ax", @progbits
        .global TaskHandler_06fae4
TaskHandler_06fae4:
        jsr     TaskHandler_070c4e(pc)          | +000
        jsr     0x28758.l                       | +004
        bcc.w   SetHandlerRts_06faf8            | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06fb10  @ $06FB10  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fb10, "ax", @progbits
        .global TaskHandler_06fb10
TaskHandler_06fb10:
        move.w  #0x59,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xb,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        clr.b   0x73(a6)                        | +016
        clr.b   0x9b(a6)                        | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_06fb2e  @ $06FB2E  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fb2e, "ax", @progbits
        .global TaskHandler_06fb2e
TaskHandler_06fb2e:
        move.b  0x98(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        lsl.w   #0x2,d0                         | +008
        move.b  0x73(a6),d1                     | +00a
        andi.w  #0x3,d1                         | +00e
        add.w   d1,d0                           | +012
        movea.l #0x2d5372,a0                    | +014
        lsl.w   #0x2,d0                         | +01a
        movea.l (a0,d0.w),a0                    | +01c
        cmpa.l  #0xffffffff,a0                  | +020
        beq.w   .L06fb5e                        | +026
        jsr     0x28cd4.l                       | +02a
.L06fb5e:
        lea     .L06fb64(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L06fb64:
        jsr     TaskHandler_070d5a(pc)          | +036
        jsr     0x28d70.l                       | +03a
        movea.l 0xc(a6),a0                      | +040
        move.b  0x9b(a0),0x9b(a6)               | +044
        move.b  0x73(a0),d0                     | +04a
        cmp.b   0x73(a6),d0                     | +04e
        beq.w   .L06fb8e                        | +052
        move.b  d0,0x73(a6)                     | +056
        lea     TaskHandler_06fb2e(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L06fb8e:
        jsr     0x5e45a.l                       | +060
        bcc.w   SetHandlerRts_06fb9e            | +066

| ----------------------------------------------------------------------------
|  TaskHandler_06fba0  @ $06FBA0  (168 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fba0, "ax", @progbits
        .global TaskHandler_06fba0
TaskHandler_06fba0:
        move.w  #0x59,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xb,0x1c(a6)                   | +00a
        jsr     0x138fe.l                       | +010
        .global TaskHandler_06fba0__L06fbb6
TaskHandler_06fba0__L06fbb6:
.L06fbb6:
        move.w  #0x3c,0x70(a6)                  | +016
        lea     0x2d61a8.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L06fbce(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L06fbce:
        jsr     TaskHandler_070d5a(pc)          | +02e
        jsr     0x28d70.l                       | +032
        bcc.w   .L06fbe8                        | +038
        lea     0x2d61a8.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
.L06fbe8:
        movea.l 0xc(a6),a0                      | +048
        cmpi.b  #0x2,0x8b(a0)                   | +04c
        bne.w   .L06fbfc                        | +052
        lea     TaskHandler_06fc50(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L06fbfc:
        cmpi.b  #0x3,0x8b(a0)                   | +05c
        bne.w   .L06fc0c                        | +062
        lea     TaskHandler_06fca2(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
        .global TaskHandler_06fba0__L06fc0c
TaskHandler_06fba0__L06fc0c:
.L06fc0c:
        movea.l 0xc(a6),a0                      | +06c
        cmpi.b  #0x0,0x8b(a0)                   | +070
        beq.w   .L06fc3e                        | +076
        subq.w  #0x1,0x70(a6)                   | +07a
        cmpi.w  #0x0,0x70(a6)                   | +07e
        bgt.w   .L06fc3e                        | +084
        lea     TaskHandler_0708c0(pc),a1       | +088
        jsr     0x4ae.l                         | +08c
        jsr     0x5dd02.l                       | +092
        move.w  #0x3c,0x70(a6)                  | +098
.L06fc3e:
        jsr     0x5e45a.l                       | +09e
        bcc.w   SetHandlerRts_06fc4e            | +0a4

| ----------------------------------------------------------------------------
|  TaskHandler_06fc50  @ $06FC50  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fc50, "ax", @progbits
        .global TaskHandler_06fc50
TaskHandler_06fc50:
        lea     0x2d638a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06fc62(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06fc62:
        jsr     TaskHandler_070d5a(pc)          | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L06fc76                        | +01c
        lea     TaskHandler_06fc50(pc),a1       | +020
        move.l  a1,(a6)                         | +024
.L06fc76:
        movea.l 0xc(a6),a0                      | +026
        cmpi.b  #0x1,0x8b(a0)                   | +02a
        bne.w   .L06fc8a                        | +030
        lea     TaskHandler_06fba0__L06fbb6(pc),a1 | +034
        move.l  a1,(a6)                         | +038
.L06fc8a:
        cmpi.b  #0x3,0x8b(a0)                   | +03a
        bne.w   .L06fc9a                        | +040
        lea     TaskHandler_06fca2(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L06fc9a:
        subq.w  #0x3,0x70(a6)                   | +04a
        bra.w   TaskHandler_06fba0__L06fc0c     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_06fca2  @ $06FCA2  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fca2, "ax", @progbits
        .global TaskHandler_06fca2
TaskHandler_06fca2:
        lea     0x2d638a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06fcb4(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06fcb4:
        jsr     TaskHandler_070d5a(pc)          | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L06fcce                        | +01c
        lea     0x2d638a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
.L06fcce:
        movea.l 0xc(a6),a0                      | +02c
        cmpi.b  #0x1,0x8b(a0)                   | +030
        bne.w   .L06fce2                        | +036
        lea     TaskHandler_06fba0__L06fbb6(pc),a1 | +03a
        move.l  a1,(a6)                         | +03e
.L06fce2:
        cmpi.b  #0x2,0x8b(a0)                   | +040
        bne.w   .L06fcf2                        | +046
        lea     TaskHandler_06fc50(pc),a1       | +04a
        move.l  a1,(a6)                         | +04e
.L06fcf2:
        subq.w  #0x7,0x70(a6)                   | +050
        bra.w   TaskHandler_06fba0__L06fc0c     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06fcfa  @ $06FCFA  (238 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fcfa, "ax", @progbits
        .global TaskHandler_06fcfa
TaskHandler_06fcfa:
        move.w  #0x10a3,d0                      | +000
        jsr     0x2352.l                        | +004
        bset    #0x4,0x6b(a6)                   | +00a
        lea     0x2b9c24.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.w  d0,0x66(a6)                     | +01c
        move.w  d0,d1                           | +020
        lsr.w   #0x2,d1                         | +022
        move.w  d1,0x7c(a6)                     | +024
        sub.w   d1,d0                           | +028
        move.w  d0,0x7a(a6)                     | +02a
        move.w  #0x5a,d1                        | +02e
        jsr     0x236e.l                        | +032
        move.w  #0xb,0x1c(a6)                   | +038
        jsr     0x138fe.l                       | +03e
        clr.b   0x84(a6)                        | +044
        clr.b   0x85(a6)                        | +048
        clr.b   0x89(a6)                        | +04c
        move.b  0x87(a6),d0                     | +050
        andi.w  #0x1,d0                         | +054
        tst.b   0x9b(a6)                        | +058
        beq.w   .L06fd5c                        | +05c
        addq.w  #0x2,d0                         | +060
.L06fd5c:
        movea.l #0x2d53c2,a0                    | +062
        lsl.w   #0x2,d0                         | +068
        movea.l (a0,d0.w),a0                    | +06a
        cmpa.l  #0xffffffff,a0                  | +06e
        beq.w   .L06fd78                        | +074
        jsr     0x28cd4.l                       | +078
.L06fd78:
        lea     .L06fd7e(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L06fd7e:
        movea.l 0xc(a6),a0                      | +084
        move.w  0x22(a0),d1                     | +088
        move.w  0x24(a0),d2                     | +08c
        move.b  0x84(a6),d3                     | +090
        move.b  0x85(a6),d4                     | +094
        ext.w   d3                              | +098
        ext.w   d4                              | +09a
        add.w   d3,d1                           | +09c
        add.w   d4,d2                           | +09e
        addi.w  #0x2c,d2                        | +0a0
        move.w  d1,0x22(a6)                     | +0a4
        move.w  d2,0x24(a6)                     | +0a8
        move.w  0x38(a0),d0                     | +0ac
        subq.w  #0x2,d0                         | +0b0
        move.w  d0,0x38(a6)                     | +0b2
        jsr     0x28d70.l                       | +0b6
        bcc.w   .L06fde6                        | +0bc
        lea     TaskHandler_06fde8(pc),a1       | +0c0
        move.l  a1,(a6)                         | +0c4
        cmpi.b  #0x4,0x88(a6)                   | +0c6
        bcs.w   .L06fdd0                        | +0cc
        lea     TaskHandler_06ffca(pc),a1       | +0d0
        move.l  a1,(a6)                         | +0d4
.L06fdd0:
        move.w  #0x8000,d0                      | +0d6
        jsr     0x28134.l                       | +0da
        andi.w  #0xffe3,0x38(a6)                | +0e0
        ori.w   #0x1c,0x38(a6)                  | +0e6
.L06fde6:
        rts                                     | +0ec

| ----------------------------------------------------------------------------
|  TaskHandler_06fde8  @ $06FDE8  (256 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fde8, "ax", @progbits
        .global TaskHandler_06fde8
TaskHandler_06fde8:
        jsr     0x267e2.l                       | +000
        lea     0x2b9ca6.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.w  d0,0x36(a6)                     | +012
        tst.b   0x34(a6)                        | +016
        beq.w   .L06fe08                        | +01a
        neg.w   d0                              | +01e
.L06fe08:
        move.w  d0,0x28(a6)                     | +020
        move.b  0x88(a6),d0                     | +024
        andi.w  #0x3,d0                         | +028
        lea     0x2d545e.l,a0                   | +02c
        tst.b   0x34(a6)                        | +032
        beq.w   .L06fe28                        | +036
        lea     0x2d546e.l,a0                   | +03a
.L06fe28:
        lsl.w   #0x2,d0                         | +040
        move.l  (a0,d0.w),0x90(a6)              | +042
        clr.w   0x94(a6)                        | +048
        clr.b   0x96(a6)                        | +04c
        lea     0x2d6d70.l,a0                   | +050
        jsr     0x28cd4.l                       | +056
        lea     .L06fe4a(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L06fe4a:
        jsr     0x78f90.l                       | +062
        move.b  0x34(a6),d0                     | +068
        andi.w  #0xff,d0                        | +06c
        lsr.w   #0x3,d0                         | +070
        move.w  d0,0x76(a6)                     | +072
        jsr     0x28d70.l                       | +076
        .global TaskHandler_06fde8__L06fe64
TaskHandler_06fde8__L06fe64:
.L06fe64:
        jsr     0x283d8.l                       | +07c
        btst    #0x1,0x13(a6)                   | +082
        beq.w   .L06fe7a                        | +088
        lea     TaskHandler_070074(pc),a1       | +08c
        move.l  a1,(a6)                         | +090
.L06fe7a:
        jsr     0x2870a.l                       | +092
        bcc.w   .L06fec2                        | +098
        bclr    #0x3,0x13(a6)                   | +09c
        move.w  0x7a(a6),d0                     | +0a2
        cmp.w   0x66(a6),d0                     | +0a6
        bcs.w   .L06fec2                        | +0aa
        sub.w   0x7c(a6),d0                     | +0ae
        move.w  d0,0x7a(a6)                     | +0b2
        jsr     0x13600.l                       | +0b6
        addq.b  #0x1,0x73(a6)                   | +0bc
        move.b  0x73(a6),d0                     | +0c0
        andi.w  #0x7,d0                         | +0c4
        add.w   d0,d0                           | +0c8
        lea     0x2d552e.l,a0                   | +0ca
        move.w  (a0,d0.w),d1                    | +0d0
        jsr     0x236e.l                        | +0d4
.L06fec2:
        jsr     0x28758.l                       | +0da
        bcc.w   .L06fed2                        | +0e0
        lea     TaskHandler_070074(pc),a1       | +0e4
        move.l  a1,(a6)                         | +0e8
.L06fed2:
        movea.l #0xffffffff,a0                  | +0ea
        lea     0x2d5316.l,a0                   | +0f0
        jsr     0x5dd56.l                       | +0f6
        bcc.w   SetHandlerRts_06feee            | +0fc

| ----------------------------------------------------------------------------
|  TaskHandler_06fef0  @ $06FEF0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06fef0, "ax", @progbits
        .global TaskHandler_06fef0
TaskHandler_06fef0:
        jsr     0x267e2.l                       | +000
        move.w  #0xffa0,0x2e(a6)                | +006
        lea     .L06ff02(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06ff02:
        bset    #0x6,0x13(a6)                   | +012
        jsr     0x27d50.l                       | +018
        bcc.w   .L06ff18                        | +01e
        lea     TaskHandler_070074(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L06ff18:
        jsr     0x28d70.l                       | +028
        cmpi.w  #0x150,0x24(a6)                 | +02e
        bgt.w   .L06ff2e                        | +034
        lea     TaskHandler_070074(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L06ff2e:
        bra.w   TaskHandler_06fde8__L06fe64     | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_06ff32  @ $06FF32  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ff32, "ax", @progbits
        .global TaskHandler_06ff32
TaskHandler_06ff32:
        lea     .L06ff38(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L06ff38:
        bset    #0x6,0x13(a6)                   | +006
        jsr     0x27cee.l                       | +00c
        jsr     0x28d70.l                       | +012
        jsr     0x5e1ea.l                       | +018
        move.w  0x22(a6),d0                     | +01e
        sub.w   0x22(a0),d0                     | +022
        cmpi.w  #0x40,d0                        | +026
        bgt.w   .L06ff6e                        | +02a
        cmpi.w  #0xffc0,d0                      | +02e
        blt.w   .L06ff6e                        | +032
        lea     TaskHandler_06ff72(pc),a1       | +036
        move.l  a1,(a6)                         | +03a
.L06ff6e:
        bra.w   TaskHandler_06fde8__L06fe64     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_06ff72  @ $06FF72  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ff72, "ax", @progbits
        .global TaskHandler_06ff72
TaskHandler_06ff72:
        move.w  #0xff80,0x2e(a6)                | +000
        lea     .L06ff7e(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L06ff7e:
        bset    #0x6,0x13(a6)                   | +00c
        jsr     0x27d50.l                       | +012
        bcc.w   .L06ff98                        | +018
        lea     TaskHandler_070074(pc),a1       | +01c
        move.l  a1,(a6)                         | +020
        bra.w   .L06ffb0                        | +022
.L06ff98:
        move.w  0x28(a6),d0                     | +026
        move.w  0x2a(a6),d1                     | +02a
        asr.w   #0x4,d0                         | +02e
        asr.w   #0x4,d1                         | +030
        jsr     0x5e018.l                       | +032
        lsr.w   #0x3,d0                         | +038
        move.w  d0,0x76(a6)                     | +03a
.L06ffb0:
        jsr     0x28d70.l                       | +03e
        cmpi.w  #0x150,0x24(a6)                 | +044
        bgt.w   .L06ffc6                        | +04a
        lea     TaskHandler_070074(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L06ffc6:
        bra.w   TaskHandler_06fde8__L06fe64     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06ffca  @ $06FFCA  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ffca, "ax", @progbits
        .global TaskHandler_06ffca
TaskHandler_06ffca:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x20,d0                        | +006
        addi.w  #0x10,d0                        | +00a
        move.w  d0,0x82(a6)                     | +00e
        move.w  #0x80,d0                        | +012
        tst.b   0x34(a6)                        | +016
        beq.w   .L06ffec                        | +01a
        move.w  #0xff80,d0                      | +01e
.L06ffec:
        move.w  d0,0x28(a6)                     | +022
        move.b  0x34(a6),d0                     | +026
        andi.w  #0xff,d0                        | +02a
        lsr.w   #0x3,d0                         | +02e
        move.w  d0,0x76(a6)                     | +030
        move.w  #0xfa00,0x2a(a6)                | +034
        lea     0x2d6d70.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L070016(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L070016:
        bset    #0x6,0x13(a6)                   | +04c
        jsr     0x27d50.l                       | +052
        bcc.w   .L07002c                        | +058
        lea     TaskHandler_070046(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
.L07002c:
        jsr     0x28d70.l                       | +062
        cmpi.w  #0x150,0x24(a6)                 | +068
        bgt.w   .L070042                        | +06e
        lea     TaskHandler_070046(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L070042:
        bra.w   TaskHandler_06fde8__L06fe64     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_070046  @ $070046  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070046, "ax", @progbits
        .global TaskHandler_070046
TaskHandler_070046:
        clr.w   0x2a(a6)                        | +000
        clr.w   0x2e(a6)                        | +004
        move.w  0x28(a6),d0                     | +008
        lsl.w   #0x4,d0                         | +00c
        move.w  d0,0x28(a6)                     | +00e
        lea     .L07005e(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L07005e:
        bset    #0x6,0x13(a6)                   | +018
        jsr     0x27cee.l                       | +01e
        jsr     0x28d70.l                       | +024
        bra.w   TaskHandler_06fde8__L06fe64     | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_070074  @ $070074  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070074, "ax", @progbits
        .global TaskHandler_070074
TaskHandler_070074:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.w  #0x1021,d0                      | +016
        jsr     0x2352.l                        | +01a
        lea     0x77fd6.l,a1                    | +020
        jsr     0x4ae.l                         | +026
        jsr     0x5dd02.l                       | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0700a6  @ $0700A6  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0700a6, "ax", @progbits
        .global TaskHandler_0700a6
TaskHandler_0700a6:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0700ac  @ $0700AC  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0700ac, "ax", @progbits
        .global TaskHandler_0700ac
TaskHandler_0700ac:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0700ae  @ $0700AE  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0700ae, "ax", @progbits
        .global TaskHandler_0700ae
TaskHandler_0700ae:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2d81f8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0700ca(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0700ca:
        jsr     0x5e506.l                       | +01c
        move.w  0x76(a0),0x76(a6)               | +022
        move.b  0x89(a0),0x89(a6)               | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0700fc                        | +034
        btst    #0x0,0x89(a6)                   | +038
        beq.w   .L0700fc                        | +03e
        lea     0x2d78e8.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
.L0700fc:
        jsr     0x5e45a.l                       | +04e
        bcc.w   SetHandlerRts_07010c            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_07010e  @ $07010E  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07010e, "ax", @progbits
        .global TaskHandler_07010e
TaskHandler_07010e:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2d81f8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07012a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07012a:
        jsr     0x5e506.l                       | +01c
        move.w  0x76(a0),0x76(a6)               | +022
        move.b  0x89(a0),0x89(a6)               | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L07015c                        | +034
        btst    #0x1,0x89(a6)                   | +038
        beq.w   .L07015c                        | +03e
        lea     0x2d6fcc.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
.L07015c:
        jsr     0x5e45a.l                       | +04e
        bcc.w   SetHandlerRts_07016c            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_07016e  @ $07016E  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07016e, "ax", @progbits
        .global TaskHandler_07016e
TaskHandler_07016e:
        jsr     0x5e9b6.l                       | +000
        andi.b  #0x7,d0                         | +006
        move.b  d0,0x99(a6)                     | +00a
        clr.w   0x70(a6)                        | +00e
        lea     .L070186(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L070186:
        movea.l 0xc(a6),a0                      | +018
        tst.b   0x20(a0)                        | +01c
        bne.w   .L0701e8                        | +020
        addq.w  #0x1,0x70(a6)                   | +024
        andi.w  #0x3,0x70(a6)                   | +028
        bne.w   .L0701e8                        | +02e
        jsr     0x5e506.l                       | +032
        move.w  0x76(a0),d0                     | +038
        move.w  d0,0x76(a6)                     | +03c
        andi.w  #0x1f,d0                        | +040
        lsl.w   #0x2,d0                         | +044
        lea     0x2d554e.l,a0                   | +046
        move.w  (a0,d0.w),d1                    | +04c
        move.w  0x2(a0,d0.w),d2                 | +050
        add.w   d1,0x22(a6)                     | +054
        addq.w  #0x4,d2                         | +058
        add.w   d2,0x24(a6)                     | +05a
        lea     TaskHandler_07020e(pc),a1       | +05e
        jsr     0x6fe.l                         | +062
        jsr     0x5dd02.l                       | +068
        move.b  0x99(a6),0x99(a0)               | +06e
        move.w  0x76(a6),0x76(a0)               | +074
.L0701e8:
        jsr     0x5e45a.l                       | +07a
        bcc.w   .L0701f8                        | +080
        lea     TaskHandler_0700a6(pc),a1       | +084
        move.l  a1,(a6)                         | +088
.L0701f8:
        movea.l 0xc(a6),a0                      | +08a
        cmpi.l  #0x6fef0,(a0)                   | +08e
        bne.w   SetHandlerRts_07020c            | +094

| ----------------------------------------------------------------------------
|  TaskHandler_07020e  @ $07020E  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07020e, "ax", @progbits
        .global TaskHandler_07020e
TaskHandler_07020e:
        move.w  0x76(a6),d0                     | +000
        lsl.w   #0x4,d0                         | +004
        lea     0x2c07ac.l,a1                   | +006
        lea     0x2c072c.l,a2                   | +00c
        move.w  (a1,d0.w),d1                    | +012
        move.w  (a2,d0.w),d2                    | +016
        lsl.w   #0x2,d1                         | +01a
        lsl.w   #0x2,d2                         | +01c
        neg.w   d1                              | +01e
        neg.w   d2                              | +020
        move.w  d1,0x28(a6)                     | +022
        move.w  d2,0x2a(a6)                     | +026
        subq.w  #0x8,0x24(a6)                   | +02a
        move.b  0x99(a6),d0                     | +02e
        andi.w  #0x7,d0                         | +032
        add.w   d0,d0                           | +036
        lea     0x2d553e.l,a0                   | +038
        move.w  (a0,d0.w),d1                    | +03e
        jsr     0x236e.l                        | +042
        move.w  #0x8,0x70(a6)                   | +048
        lea     0x2d8204.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        lea     .L07026e(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L07026e:
        jsr     0x27cee.l                       | +060
        jsr     0x28d70.l                       | +066
        subq.w  #0x1,0x70(a6)                   | +06c
        cmpi.w  #0x0,0x70(a6)                   | +070
        bgt.w   SetHandlerRts_07028e            | +076

| ----------------------------------------------------------------------------
|  TaskHandler_070290  @ $070290  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070290, "ax", @progbits
        .global TaskHandler_070290
TaskHandler_070290:
        lea     .L070296(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L070296:
        jsr     0x28d70.l                       | +006
        bcc.w   SetHandlerRts_0702a6            | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0702a8  @ $0702A8  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0702a8, "ax", @progbits
        .global TaskHandler_0702a8
TaskHandler_0702a8:
        move.w  #0x10a4,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x4,d1                         | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2d82ba.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        .global TaskHandler_0702a8__L0702c8
TaskHandler_0702a8__L0702c8:
.L0702c8:
        lea     .L0702ce(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L0702ce:
        jsr     TaskHandler_070d9c(pc)          | +026
        jsr     0x28d70.l                       | +02a
        bcc.w   SetHandlerRts_0702e2            | +030

| ----------------------------------------------------------------------------
|  TaskHandler_0702e4  @ $0702E4  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0702e4, "ax", @progbits
        .global TaskHandler_0702e4
TaskHandler_0702e4:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2d8320.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.b   TaskHandler_0702a8__L0702c8     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_0702fc  @ $0702FC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0702fc, "ax", @progbits
        .global TaskHandler_0702fc
TaskHandler_0702fc:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x9c(a6)                     | +006
        jsr     0x4a110.l                       | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_07030c  @ $07030C  (116 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07030c, "ax", @progbits
        .global TaskHandler_07030c
TaskHandler_07030c:
        lea     0x2b9dfa.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        lea     0x29b816.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L07032e(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L07032e:
        jsr     TaskHandler_070d1e(pc)          | +022
        jsr     0x28d70.l                       | +026
        subq.w  #0x1,0x70(a6)                   | +02c
        cmpi.w  #0x0,0x70(a6)                   | +030
        bgt.w   .L07034c                        | +036
        lea     TaskHandler_070380(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
.L07034c:
        bra.w   TaskHandler_070468__L070492     | +040
        .global TaskHandler_07030c__L070350
TaskHandler_07030c__L070350:
.L070350:
        eori.b  #0x1,0x3a(a6)                   | +044
        lea     0x29bfb8.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     .L070368(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L070368:
        jsr     TaskHandler_070d1e(pc)          | +05c
        jsr     0x28d70.l                       | +060
        bcc.w   .L07037c                        | +066
        lea     TaskHandler_070392(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
.L07037c:
        bra.w   TaskHandler_070468__L070492     | +070

| ----------------------------------------------------------------------------
|  TaskHandler_070380  @ $070380  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070380, "ax", @progbits
        .global TaskHandler_070380
TaskHandler_070380:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x9c(a6)                     | +006
        jsr     0x5e618.l                       | +00a
        bcs.b   TaskHandler_07030c__L070350     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_070392  @ $070392  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070392, "ax", @progbits
        .global TaskHandler_070392
TaskHandler_070392:
        movea.l 0x9c(a6),a0                     | +000
        move.w  0x22(a0),d0                     | +004
        sub.w   0x22(a6),d0                     | +008
        cmpi.w  #0x0,d0                         | +00c
        bgt.w   .L0703a8                        | +010
        neg.w   d0                              | +014
.L0703a8:
        clr.w   d1                              | +016
        cmpi.w  #0x20,d0                        | +018
        blt.w   .L0703ce                        | +01c
        move.w  #0x1,d1                         | +020
        cmpi.w  #0x50,d0                        | +024
        blt.w   .L0703ce                        | +028
        move.w  #0x2,d1                         | +02c
        cmpi.w  #0x80,d0                        | +030
        blt.w   .L0703ce                        | +034
        move.w  #0x3,d1                         | +038
.L0703ce:
        addi.b  #0xc,d1                         | +03c
        move.b  d1,0x5c(a6)                     | +040
        lea     0x29ba70.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        lea     .L0703e8(pc),a1                 | +050
        move.l  a1,(a6)                         | +054
.L0703e8:
        jsr     TaskHandler_070d1e(pc)          | +056
        jsr     0x28d70.l                       | +05a
        bcc.w   .L070414                        | +060
        lea     TaskHandler_07030c(pc),a1       | +064
        move.l  a1,(a6)                         | +068
        movea.l 0xc(a6),a0                      | +06a
        subq.b  #0x1,0x72(a0)                   | +06e
        cmpi.b  #0x0,0x72(a0)                   | +072
        bgt.w   .L070414                        | +078
        lea     TaskHandler_070418(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
.L070414:
        bra.w   TaskHandler_070468__L070492     | +082

| ----------------------------------------------------------------------------
|  TaskHandler_070418  @ $070418  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070418, "ax", @progbits
        .global TaskHandler_070418
TaskHandler_070418:
        lea     0x29b816.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07042a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07042a:
        jsr     TaskHandler_070d1e(pc)          | +012
        jsr     0x28d70.l                       | +016
        bra.w   TaskHandler_070468__L070492     | +01c
        .global TaskHandler_070418__L070438
TaskHandler_070418__L070438:
.L070438:
        jsr     0x4a110.l                       | +020
        lea     0x2e2286.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L070450(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L070450:
        jsr     TaskHandler_070d1e(pc)          | +038
        jsr     0x28d70.l                       | +03c
        bcc.w   .L070464                        | +042
        lea     TaskHandler_070468(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L070464:
        bra.w   TaskHandler_070468__L070492     | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_070468  @ $070468  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070468, "ax", @progbits
        .global TaskHandler_070468
TaskHandler_070468:
        lea     0x29bd40.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L07047a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L07047a:
        jsr     TaskHandler_070d1e(pc)          | +012
        jsr     0x28d70.l                       | +016
        bcc.w   .L07048e                        | +01c
        lea     TaskHandler_070380(pc),a1       | +020
        move.l  a1,(a6)                         | +024
.L07048e:
        bra.w   .L070492                        | +026
        .global TaskHandler_070468__L070492
TaskHandler_070468__L070492:
.L070492:
        jsr     0x2870a.l                       | +02a
        bcc.w   TaskHandler_0704cc              | +030
        movea.l 0xc(a6),a0                      | +034
        move.b  #0x1,0x8a(a0)                   | +038
        move.w  #0x8000,d0                      | +03e
        jsr     0x28134.l                       | +042
        andi.w  #0xffe3,0x38(a6)                | +048
        ori.w   #0x1c,0x38(a6)                  | +04e
        movea.l 0xc(a6),a0                      | +054
        subq.b  #0x1,0x72(a0)                   | +058

| ----------------------------------------------------------------------------
|  TaskHandler_0704cc  @ $0704CC  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0704cc, "ax", @progbits
        .global TaskHandler_0704cc
TaskHandler_0704cc:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x8a(a0)                   | +004
        bne.w   .L0704e0                        | +00a
        lea     JmpToScheduler_0704f2(pc),a1    | +00e
        move.l  a1,(a6)                         | +012
.L0704e0:
        jsr     0x5e45a.l                       | +014
        bcc.w   SetHandlerRts_0704f0            | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0704fa  @ $0704FA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0704fa, "ax", @progbits
        .global TaskHandler_0704fa
TaskHandler_0704fa:
        move.w  #0x98,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0x2,0x76(a6)                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_07050a  @ $07050A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07050a, "ax", @progbits
        .global TaskHandler_07050a
TaskHandler_07050a:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x9c(a6)                     | +006
        lea     0x2b9ecc.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x70(a6)                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_070524  @ $070524  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070524, "ax", @progbits
        .global TaskHandler_070524
TaskHandler_070524:
        lea     0x2d83ae.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L070536(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L070536:
        jsr     TaskHandler_070d7c(pc)          | +012
        jsr     0x28d70.l                       | +016
        tst.b   0x9a(a6)                        | +01c
        bne.w   .L07056c                        | +020
        subq.w  #0x1,0x70(a6)                   | +024
        cmpi.w  #0x0,0x70(a6)                   | +028
        bgt.w   .L07056c                        | +02e
        lea     TaskHandler_0705b2(pc),a1       | +032
        move.l  a1,(a6)                         | +036
        cmpi.b  #0x0,0x72(a6)                   | +038
        bgt.w   .L07056c                        | +03e
        lea     TaskHandler_0705c2(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L07056c:
        movea.l 0x9c(a6),a0                     | +048
        move.w  0x22(a0),d0                     | +04c
        move.w  0x24(a0),d1                     | +050
        sub.w   0x22(a6),d0                     | +054
        sub.w   0x24(a6),d1                     | +058
        addi.w  #0x20,d0                        | +05c
        jsr     0x5e018.l                       | +060
        jsr     0x4206a.l                       | +066
        move.w  d0,0x78(a6)                     | +06c
        move.w  #0xffff,d1                      | +070
        move.w  0x78(a6),d0                     | +074
        cmp.w   0x76(a6),d0                     | +078
        beq.w   .L0705ae                        | +07c
        bcs.w   .L0705aa                        | +080
        neg.w   d1                              | +084
.L0705aa:
        add.w   d1,0x76(a6)                     | +086
.L0705ae:
        bra.w   TaskHandler_0707d0              | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_0705b2  @ $0705B2  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0705b2, "ax", @progbits
        .global TaskHandler_0705b2
TaskHandler_0705b2:
        lea     0x2b9f4e.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x72(a6)                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0705c2  @ $0705C2  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0705c2, "ax", @progbits
        .global TaskHandler_0705c2
TaskHandler_0705c2:
        jsr     0x5e1ea.l                       | +000
        move.l  a0,0x9c(a6)                     | +006
        lea     0x2b9fd0.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x70(a6)                     | +016
        move.w  0x76(a6),d0                     | +01a
        lsr.w   #0x1,d0                         | +01e
        movea.l #0x2d5402,a0                    | +020
        lsl.w   #0x2,d0                         | +026
        movea.l (a0,d0.w),a0                    | +028
        cmpa.l  #0xffffffff,a0                  | +02c
        beq.w   .L0705fe                        | +032
        jsr     0x28cd4.l                       | +036
.L0705fe:
        lea     .L070604(pc),a1                 | +03c
        move.l  a1,(a6)                         | +040
.L070604:
        jsr     TaskHandler_070d7c(pc)          | +042
        jsr     0x28d70.l                       | +046
        bcc.w   .L070634                        | +04c
        lea     TaskHandler_070524(pc),a1       | +050
        move.l  a1,(a6)                         | +054
        subq.b  #0x1,0x72(a6)                   | +056
        cmpi.b  #0x0,0x72(a6)                   | +05a
        bgt.w   .L070634                        | +060
        lea     TaskHandler_07050a(pc),a1       | +064
        move.l  a1,(a6)                         | +068
        movea.l 0xc(a6),a0                      | +06a
        clr.b   0x72(a0)                        | +06e
.L070634:
        bra.w   TaskHandler_070816              | +072

| ----------------------------------------------------------------------------
|  TaskHandler_070638  @ $070638  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070638, "ax", @progbits
        .global TaskHandler_070638
TaskHandler_070638:
        subi.w  #0x1e,0x70(a6)                  | +000
        lea     0x2d8714.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L070650(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L070650:
        jsr     TaskHandler_070d7c(pc)          | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L07066a                        | +022
        bclr    #0x3,0x13(a6)                   | +026
        lea     TaskHandler_070524(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L07066a:
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0x2,0x8a(a0)                   | +036
        bne.w   .L07067e                        | +03c
        lea     JmpToScheduler_0704f2(pc),a1    | +040
        move.l  a1,(a6)                         | +044
.L07067e:
        movea.l 0xc(a6),a0                      | +046
        btst    #0x0,0x13(a0)                   | +04a
        beq.w   SetHandlerRts_070692            | +050

| ----------------------------------------------------------------------------
|  TaskHandler_070694  @ $070694  (160 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070694, "ax", @progbits
        .global TaskHandler_070694
TaskHandler_070694:
        clr.b   0x21(a6)                        | +000
        move.w  #0xd000,d0                      | +004
        jsr     0x28134.l                       | +008
        andi.w  #0xffe3,0x38(a6)                | +00e
        ori.w   #0x18,0x38(a6)                  | +014
        move.l  #0xffffffff,0x48(a6)            | +01a
        move.w  #0xf800,0x28(a6)                | +022
        move.w  #0x40,0x2c(a6)                  | +028
        move.w  #0x200,0x2a(a6)                 | +02e
        move.w  #0xffc0,0x2e(a6)                | +034
        move.w  #0x98,d1                        | +03a
        jsr     0x236e.l                        | +03e
        bset    #0x6,0x12(a6)                   | +044
        lea     0x6e820.l,a1                    | +04a
        jsr     0x4ae.l                         | +050
        jsr     0x5dd02.l                       | +056
        lea     0x2d875c.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        lea     .L070702(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L070702:
        bset    #0x6,0x13(a6)                   | +06e
        jsr     0x27d50.l                       | +074
        bcc.w   .L070718                        | +07a
        lea     TaskHandler_07073c(pc),a1       | +07e
        move.l  a1,(a6)                         | +082
.L070718:
        jsr     0x28d70.l                       | +084
        cmpi.w  #0xff80,0x28(a6)                | +08a
        blt.w   .L07072c                        | +090
        clr.w   0x2c(a6)                        | +094
.L07072c:
        tst.b   0x21(a6)                        | +098
        beq.w   SetHandlerRts_07073a            | +09c

| ----------------------------------------------------------------------------
|  TaskHandler_07073c  @ $07073C  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07073c, "ax", @progbits
        .global TaskHandler_07073c
TaskHandler_07073c:
        jsr     0x267e2.l                       | +000
        move.w  #0x21c,0x2a(a6)                 | +006
        move.w  #0xffdc,0x2e(a6)                | +00c
        lea     0x28732c.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L070760(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L070760:
        jsr     0x27d50.l                       | +024
        bcc.w   .L070788                        | +02a
        clr.b   0x106ed2.l                      | +02e
        lea     0x28735e.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        lea     .L070782(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L070782:
        jsr     0x2783a.l                       | +046
.L070788:
        jsr     0x28d70.l                       | +04c
        tst.b   0x21(a6)                        | +052
        beq.w   SetHandlerRts_07079c            | +056

| ----------------------------------------------------------------------------
|  TaskHandler_07079e  @ $07079E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07079e, "ax", @progbits
        .global TaskHandler_07079e
TaskHandler_07079e:
        clr.b   0x106ed2.l                      | +000
        jsr     0x267e2.l                       | +006
        lea     .L0707b0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0707b0:
        jsr     0x2783a.l                       | +012
        cmpi.b  #0x1,0x21(a6)                   | +018
        beq.w   SetHandlerRts_0707c6            | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_0707d0  @ $0707D0  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0707d0, "ax", @progbits
        .global TaskHandler_0707d0
TaskHandler_0707d0:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x8a(a0)                   | +004
        bne.w   .L0707e4                        | +00a
        lea     JmpToScheduler_0704f2(pc),a1    | +00e
        move.l  a1,(a6)                         | +012
.L0707e4:
        jsr     0x2870a.l                       | +014
        bcc.w   .L070800                        | +01a
        bclr    #0x0,0x13(a6)                   | +01e
        move.w  #0x1,0x66(a6)                   | +024
        lea     TaskHandler_070638(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L070800:
        movea.l 0xc(a6),a0                      | +030
        btst    #0x0,0x13(a0)                   | +034
        beq.w   SetHandlerRts_070814            | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_070816  @ $070816  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070816, "ax", @progbits
        .global TaskHandler_070816
TaskHandler_070816:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x2,0x8a(a0)                   | +004
        bne.w   .L07082a                        | +00a
        lea     JmpToScheduler_0704f2(pc),a1    | +00e
        move.l  a1,(a6)                         | +012
.L07082a:
        jsr     0x2870a.l                       | +014
        bcc.w   .L070846                        | +01a
        bclr    #0x3,0x13(a6)                   | +01e
        bclr    #0x0,0x13(a6)                   | +024
        move.w  #0x1,0x66(a6)                   | +02a
.L070846:
        movea.l 0xc(a6),a0                      | +030
        btst    #0x0,0x13(a0)                   | +034
        beq.w   SetHandlerRts_07085a            | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_07085c  @ $07085C  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07085c, "ax", @progbits
        .global TaskHandler_07085c
TaskHandler_07085c:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x13600.l                       | +00a
        move.l  #0xffffffff,0x48(a6)            | +010
        bra.w   AnimSeq_00077F6A                | +018

| ----------------------------------------------------------------------------
|  TaskHandler_070878  @ $070878  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070878, "ax", @progbits
        .global TaskHandler_070878
TaskHandler_070878:
        lea     0x2d8770.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global TaskHandler_070878__L070884
TaskHandler_070878__L070884:
.L070884:
        move.w  #0x4,d1                         | +00c
        jsr     0x236e.l                        | +010
        lea     .L070894(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L070894:
        jsr     0x5e506.l                       | +01c
        addi.w  #0x2c,0x24(a6)                  | +022
        jsr     0x28d70.l                       | +028
        bcc.w   SetHandlerRts_0708b0            | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_0708b2  @ $0708B2  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0708b2, "ax", @progbits
        .global TaskHandler_0708b2
TaskHandler_0708b2:
        lea     0x2d87ae.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.b   TaskHandler_070878__L070884     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_0708c0  @ $0708C0  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0708c0, "ax", @progbits
        .global TaskHandler_0708c0
TaskHandler_0708c0:
        move.w  #0x2,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2d87d8.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0708dc(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0708dc:
        jsr     0x5e506.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   SetHandlerRts_0708f2            | +028

| ----------------------------------------------------------------------------
|  TaskHandler_0708f4  @ $0708F4  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0708f4, "ax", @progbits
        .global TaskHandler_0708f4
TaskHandler_0708f4:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd8b6.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L070910(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L070910:
        jsr     0x28d70.l                       | +01c
        bcc.w   SetHandlerRts_070920            | +022

| ----------------------------------------------------------------------------
|  TaskHandler_070922  @ $070922  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070922, "ax", @progbits
        .global TaskHandler_070922
TaskHandler_070922:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd944.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L07093e(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L07093e:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   SetHandlerRts_070954            | +028

| ----------------------------------------------------------------------------
|  TaskHandler_070956  @ $070956  (212 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070956, "ax", @progbits
        .global TaskHandler_070956
TaskHandler_070956:
        eori.b  #0x1,0x3a(a6)                   | +000
        move.w  #0xfe00,0x28(a6)                | +006
        move.w  #0x1e,0x66(a6)                  | +00c
        move.w  #0x1de,d1                       | +012
        jsr     0x236e.l                        | +016
        move.w  #0x173,d1                       | +01c
        jsr     0x236e.l                        | +020
        bset    #0x4,0x6b(a6)                   | +026
        move.w  #0xd000,d0                      | +02c
        jsr     0x28134.l                       | +030
        andi.w  #0xffe3,0x38(a6)                | +036
        ori.w   #0x14,0x38(a6)                  | +03c
        lea     0x2d883c.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        move.w  #0xffc0,0x2e(a6)                | +04e
        lea     .L0709b0(pc),a1                 | +054
        move.l  a1,(a6)                         | +058
.L0709b0:
        bset    #0x6,0x13(a6)                   | +05a
        jsr     0x27bc8.l                       | +060
        bcc.w   .L0709c6                        | +066
        lea     TaskHandler_070a64(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
        .global TaskHandler_070956__L0709c6
TaskHandler_070956__L0709c6:
.L0709c6:
        btst    #0x1,0x106f28.l                 | +070
        bne.w   .L0709da                        | +078
        move.w  0x16(a6),d0                     | +07c
        bra.w   .L0709de                        | +080
.L0709da:
        move.w  0x18(a6),d0                     | +084
.L0709de:
        move.w  d0,0x14(a6)                     | +088
        jsr     0x28d70.l                       | +08c
        jsr     0x283d8.l                       | +092
        btst    #0x1,0x13(a6)                   | +098
        beq.w   .L0709fe                        | +09e
        lea     TaskHandler_07085c(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6
.L0709fe:
        jsr     0x2870a.l                       | +0a8
        bclr    #0x3,0x13(a6)                   | +0ae
        jsr     0x28758.l                       | +0b4
        bcc.w   .L070a1a                        | +0ba
        lea     TaskHandler_07085c(pc),a1       | +0be
        move.l  a1,(a6)                         | +0c2
.L070a1a:
        movea.l #0xffffffff,a0                  | +0c4
        jsr     0x5dd5c.l                       | +0ca
        bcc.w   SetHandlerRts_070a30            | +0d0

| ----------------------------------------------------------------------------
|  TaskHandler_070a32  @ $070A32  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070a32, "ax", @progbits
        .global TaskHandler_070a32
TaskHandler_070a32:
        clr.w   0x2e(a6)                        | +000
        clr.w   0x2a(a6)                        | +004
        lea     .L070a40(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L070a40:
        bset    #0x6,0x13(a6)                   | +00e
        jsr     0x27eba.l                       | +014
        bcc.w   .L070a5a                        | +01a
        jsr     0x27a92.l                       | +01e
        bra.w   .L070a60                        | +024
.L070a5a:
        jsr     0x27c8c.l                       | +028
.L070a60:
        bra.w   TaskHandler_070956__L0709c6     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_070a64  @ $070A64  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070a64, "ax", @progbits
        .global TaskHandler_070a64
TaskHandler_070a64:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x700,d0                       | +006
        subi.w  #0x280,d0                       | +00a
        neg.w   d0                              | +00e
        move.w  d0,0x28(a6)                     | +010
        jsr     0x5e9b6.l                       | +014
        andi.w  #0x700,d0                       | +01a
        addi.w  #0x400,d0                       | +01e
        move.w  d0,0x2a(a6)                     | +022
        move.w  #0xffc0,0x2e(a6)                | +026
        lea     .L070a96(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L070a96:
        bset    #0x6,0x13(a6)                   | +032
        jsr     0x27bc8.l                       | +038
        bcc.w   .L070aac                        | +03e
        lea     TaskHandler_07085c(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L070aac:
        bra.w   TaskHandler_070956__L0709c6     | +048

| ----------------------------------------------------------------------------
|  PcThunkTarget_070ab0  @ $070AB0  (22 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_070ab0, "ax", @progbits
        .global PcThunkTarget_070ab0
PcThunkTarget_070ab0:
        addi.w  #0x2c,0x24(a6)                  | +000
        jsr     0x28d70.l                       | +006
        bcs.w   TaskHandler_070acc              | +00c
        subi.w  #0x2c,0x24(a6)                  | +010

| ----------------------------------------------------------------------------
|  TaskHandler_070acc  @ $070ACC  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070acc, "ax", @progbits
        .global TaskHandler_070acc
TaskHandler_070acc:
        subi.w  #0x2c,0x24(a6)                  | +000

| ----------------------------------------------------------------------------
|  TaskHandler_070ad8  @ $070AD8  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ad8, "ax", @progbits
        .global TaskHandler_070ad8
TaskHandler_070ad8:
        move.b  0x8c(a6),d0                     | +000
        andi.w  #0x7,d0                         | +004
        lsl.w   #0x2,d0                         | +008
        lea     0x2d541a.l,a0                   | +00a
        movea.l (a0,d0.w),a1                    | +010
        jmp     (a1)                            | +014

| ----------------------------------------------------------------------------
|  TaskHandler_070aee  @ $070AEE  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070aee, "ax", @progbits
        .global TaskHandler_070aee
TaskHandler_070aee:
        move.w  #0x10,d3                        | +000
        .global TaskHandler_070aee__L070af2
TaskHandler_070aee__L070af2:
.L070af2:
        move.w  0x80(a6),d0                     | +004
        move.w  0x82(a6),d1                     | +008
        sub.w   0x22(a6),d0                     | +00c
        sub.w   0x24(a6),d1                     | +010
        move.w  d0,d2                           | +014
        add.w   d1,d2                           | +016
        cmpi.w  #0x0,d2                         | +018
        bge.w   .L070b10                        | +01c
        neg.w   d2                              | +020
.L070b10:
        cmp.w   d3,d2                           | +022
        ble.w   TaskHandler_070b66              | +024
        jsr     0x5e018.l                       | +028
        add.w   d0,d0                           | +02e
        lea     0x2c072c.l,a1                   | +030
        lea     0x2c07ac.l,a2                   | +036
        move.w  (a1,d0.w),d1                    | +03c
        move.w  (a2,d0.w),d2                    | +040
        move.w  0x36(a6),d0                     | +044
        muls.w  d0,d1                           | +048
        muls.w  d0,d2                           | +04a
        asr.l   #0x8,d1                         | +04c
        asr.l   #0x8,d2                         | +04e
        move.w  d1,0x2a(a6)                     | +050
        move.w  d2,0x28(a6)                     | +054
        bset    #0x6,0x13(a6)                   | +058
        jsr     0x27d50.l                       | +05e
        subq.w  #0x1,0x94(a6)                   | +064
        cmpi.w  #0x0,0x94(a6)                   | +068
        ble.w   TaskHandler_070b66              | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_070b66  @ $070B66  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070b66, "ax", @progbits
        .global TaskHandler_070b66
TaskHandler_070b66:
        clr.b   0x74(a6)                        | +000

| ----------------------------------------------------------------------------
|  TaskHandler_070b70  @ $070B70  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070b70, "ax", @progbits
        .global TaskHandler_070b70
TaskHandler_070b70:
        move.w  #0x8,d3                         | +000
        bra.w   TaskHandler_070aee__L070af2     | +004

| ----------------------------------------------------------------------------
|  TaskHandler_070b78  @ $070B78  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070b78, "ax", @progbits
        .global TaskHandler_070b78
TaskHandler_070b78:
        move.w  #0x1,d3                         | +000
        bra.w   TaskHandler_070aee__L070af2     | +004

| ----------------------------------------------------------------------------
|  TaskHandler_070b86  @ $070B86  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070b86, "ax", @progbits
        .global TaskHandler_070b86
TaskHandler_070b86:
        move.b  0x74(a6),d0                     | +000
        addq.b  #0x8,0x74(a6)                   | +004
        lea     0x2c072c.l,a0                   | +008
        andi.w  #0xff,d0                        | +00e
        add.w   d0,d0                           | +012
        move.w  (a0,d0.w),0x2a(a6)              | +014
        clr.w   0x28(a6)                        | +01a
        bset    #0x6,0x13(a6)                   | +01e
        jsr     0x27d50.l                       | +024
        bcc.w   ClearXN_070bbc                  | +02a
        clr.w   0x2e(a6)                        | +02e
        clr.w   0x2a(a6)                        | +032

| ----------------------------------------------------------------------------
|  TaskHandler_070bc2  @ $070BC2  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070bc2, "ax", @progbits
        .global TaskHandler_070bc2
TaskHandler_070bc2:
        move.b  0x74(a6),d0                     | +000
        addq.b  #0x8,0x74(a6)                   | +004
        lea     0x2c072c.l,a0                   | +008
        andi.w  #0xff,d0                        | +00e
        add.w   d0,d0                           | +012
        move.w  (a0,d0.w),d0                    | +014
        asr.w   #0x1,d0                         | +018
        move.w  d0,0x2a(a6)                     | +01a
        clr.w   0x28(a6)                        | +01e
        bset    #0x6,0x13(a6)                   | +022
        jsr     0x27d50.l                       | +028
        bcc.w   ClearXN_070bfc                  | +02e
        clr.w   0x2e(a6)                        | +032
        clr.w   0x2a(a6)                        | +036

| ----------------------------------------------------------------------------
|  TaskHandler_070c02  @ $070C02  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070c02, "ax", @progbits
        .global TaskHandler_070c02
TaskHandler_070c02:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   TaskHandler_070c46              | +00a
        move.w  #0x2,d0                         | +00e
        cmpi.b  #0x2,0x73(a6)                   | +012
        bge.w   SetXN_070c40                    | +018
        lea     0x2ba166.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        cmpi.b  #0x1,0x73(a6)                   | +028
        beq.w   SetXN_070c40                    | +02e
        lea     0x2ba0e4.l,a0                   | +032
        jsr     0x799de.l                       | +038

| ----------------------------------------------------------------------------
|  TaskHandler_070c46  @ $070C46  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070c46, "ax", @progbits
        .global TaskHandler_070c46
TaskHandler_070c46:
        clr.w   d0                              | +000

| ----------------------------------------------------------------------------
|  TaskHandler_070c4e  @ $070C4E  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070c4e, "ax", @progbits
        .global TaskHandler_070c4e
TaskHandler_070c4e:
        jsr     0x2870a.l                       | +000
        bcc.w   .L070cbc                        | +006
        bclr    #0x3,0x13(a6)                   | +00a
        lea     0x5e766.l,a0                    | +010
        jsr     0x5e770.l                       | +016
        move.w  0x66(a6),d0                     | +01c
        cmp.w   0x7a(a6),d0                     | +020
        bgt.w   .L070cbc                        | +024
        move.b  0x73(a6),d1                     | +028
        cmp.b   0x8e(a6),d1                     | +02c
        beq.w   .L070c9c                        | +030
        move.b  d1,0x8e(a6)                     | +034
        move.w  #0x1023,d0                      | +038
        jsr     0x2352.l                        | +03c
        lea     0x2d543a.l,a1                   | +042
        jsr     0x77c7e.l                       | +048
.L070c9c:
        clr.w   d2                              | +04e
        move.b  #0x2,d1                         | +050
        move.w  0x7c(a6),d0                     | +054
        cmp.w   0x7a(a6),d0                     | +058
        beq.w   .L070cb4                        | +05c
        move.b  #0x1,d1                         | +060
        move.w  d0,d2                           | +064
.L070cb4:
        move.w  d2,0x7a(a6)                     | +066
        move.b  d1,0x73(a6)                     | +06a
.L070cbc:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_070cbe  @ $070CBE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070cbe, "ax", @progbits
        .global TaskHandler_070cbe
TaskHandler_070cbe:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x7,d0                         | +006
        lsl.w   #0x2,d0                         | +00a
        lea     0x2d54be.l,a0                   | +00c
        move.w  (a0,d0.w),0x80(a6)              | +012
        move.w  0x2(a0,d0.w),0x82(a6)           | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_070cde  @ $070CDE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070cde, "ax", @progbits
        .global TaskHandler_070cde
TaskHandler_070cde:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x3,d0                         | +006
        lsl.w   #0x2,d0                         | +00a
        lea     0x2d551e.l,a0                   | +00c
        move.w  (a0,d0.w),0x80(a6)              | +012
        move.w  0x2(a0,d0.w),0x82(a6)           | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_070cfe  @ $070CFE  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070cfe, "ax", @progbits
        .global TaskHandler_070cfe
TaskHandler_070cfe:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x7,d0                         | +006
        lsl.w   #0x2,d0                         | +00a
        lea     0x2d54fe.l,a0                   | +00c
        move.w  (a0,d0.w),0x80(a6)              | +012
        move.w  0x2(a0,d0.w),0x82(a6)           | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_070d1e  @ $070D1E  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070d1e, "ax", @progbits
        .global TaskHandler_070d1e
TaskHandler_070d1e:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),0x22(a6)               | +004
        move.w  0x24(a0),0x24(a6)               | +00a
        move.b  0x26(a0),0x26(a6)               | +010
        move.b  0x27(a0),0x27(a6)               | +016
        move.w  0x38(a0),0x38(a6)               | +01c
        move.b  0x85(a0),d0                     | +022
        ext.w   d0                              | +026
        addi.w  #0x2e,d0                        | +028
        add.w   d0,0x24(a6)                     | +02c
        addi.w  #0xffdd,0x22(a6)                | +030
        addq.w  #0x2,0x38(a6)                   | +036
        rts                                     | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_070d5a  @ $070D5A  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070d5a, "ax", @progbits
        .global TaskHandler_070d5a
TaskHandler_070d5a:
        jsr     0x5e506.l                       | +000
        addi.w  #0x2c,0x24(a6)                  | +006
        movea.l 0xc(a6),a0                      | +00c
        btst    #0x7,0x5a(a0)                   | +010
        beq.w   .L070d7a                        | +016
        bset    #0x0,0x5a(a6)                   | +01a
.L070d7a:
        rts                                     | +020

| ----------------------------------------------------------------------------
|  TaskHandler_070d7c  @ $070D7C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070d7c, "ax", @progbits
        .global TaskHandler_070d7c
TaskHandler_070d7c:
        jsr     0x5e506.l                       | +000
        move.b  0x85(a0),d0                     | +006
        ext.w   d0                              | +00a
        addi.w  #0x35,d0                        | +00c
        add.w   d0,0x24(a6)                     | +010
        addi.w  #0xfff0,0x22(a6)                | +014
        addq.w  #0x2,0x38(a6)                   | +01a
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_070d9c  @ $070D9C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070d9c, "ax", @progbits
        .global TaskHandler_070d9c
TaskHandler_070d9c:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x86(a6),d0                     | +004
        ext.w   d0                              | +008
        add.w   0x38(a0),d0                     | +00a
        move.w  d0,0x38(a6)                     | +00e
        move.w  0x22(a0),0x22(a6)               | +012
        move.w  0x24(a0),0x24(a6)               | +018
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_070dbc  @ $070DBC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070dbc, "ax", @progbits
        .global TaskHandler_070dbc
TaskHandler_070dbc:
        move.w  0x24(a6),d2                     | +000
        addq.w  #0x8,d2                         | +004
.L070dc2:
        subq.w  #0x8,d2                         | +006
        cmpi.w  #0x100,d2                       | +008
        blt.w   .L070df2                        | +00c
        move.w  0x22(a6),d1                     | +010
        move.w  d2,0x5c(a6)                     | +014
        jsr     0x280c6.l                       | +018
        move.w  0x5c(a6),d2                     | +01e
        tst.b   d0                              | +022
        beq.b   .L070dc2                        | +024
        andi.w  #0x30,d0                        | +026
        tst.w   d0                              | +02a
        beq.w   .L070df2                        | +02c
        cmpi.w  #0x20,d0                        | +030
        bne.b   .L070dc2                        | +034
.L070df2:
        move.w  0x24(a6),d1                     | +036
        sub.w   d2,d1                           | +03a
        cmp.w   0x82(a6),d1                     | +03c
        bgt.w   ClearXN_070e06                  | +040

| ----------------------------------------------------------------------------
|  TaskHandler_070e0c  @ $070E0C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070e0c, "ax", @progbits
        .global TaskHandler_070e0c
TaskHandler_070e0c:
        move.w  0x76(a6),d0                     | +000
        lsl.w   #0x4,d0                         | +004
        lea     0x2c072c.l,a1                   | +006
        lea     0x2c07ac.l,a2                   | +00c
        move.w  (a1,d0.w),d1                    | +012
        move.w  (a2,d0.w),d2                    | +016
        move.w  0x36(a6),d0                     | +01a
        muls.w  d0,d1                           | +01e
        muls.w  d0,d2                           | +020
        asr.l   #0x8,d1                         | +022
        asr.l   #0x8,d2                         | +024
        move.w  d1,0x2a(a6)                     | +026
        move.w  d2,0x28(a6)                     | +02a
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_070e3c  @ $070E3C  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070e3c, "ax", @progbits
        .global TaskHandler_070e3c
TaskHandler_070e3c:
        move.b  #0x0,0x8a(a6)                   | +000
        lea     TaskHandler_0702fc(pc),a1       | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        bset    #0x1,0x12(a0)                   | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_070e5a  @ $070E5A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070e5a, "ax", @progbits
        .global TaskHandler_070e5a
TaskHandler_070e5a:
        move.b  #0x0,0x8a(a6)                   | +000
        lea     TaskHandler_070418__L070438(pc),a1 | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        bset    #0x1,0x12(a0)                   | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_070e78  @ $070E78  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070e78, "ax", @progbits
        .global TaskHandler_070e78
TaskHandler_070e78:
        move.b  #0x0,0x8a(a6)                   | +000
        lea     TaskHandler_0704fa(pc),a1       | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        bset    #0x1,0x12(a0)                   | +016
        move.b  0x9a(a6),0x9a(a0)               | +01c
        move.w  0x66(a6),0x66(a0)               | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_070ea2  @ $070EA2  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ea2, "ax", @progbits
        .global TaskHandler_070ea2
TaskHandler_070ea2:
        lea     TaskHandler_070956(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addq.w  #0x8,0x24(a0)                   | +010
        addi.w  #0xffc5,0x22(a0)                | +014
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_070ebe  @ $070EBE  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ebe, "ax", @progbits
        .global TaskHandler_070ebe
TaskHandler_070ebe:
        lea     0x4cbd4.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        move.l  a0,0x90(a6)                     | +012
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_070ed6  @ $070ED6  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ed6, "ax", @progbits
        .global TaskHandler_070ed6
TaskHandler_070ed6:
        movea.l 0x90(a6),a0                     | +000
        clr.b   0x80(a0)                        | +004
        rts                                     | +008

| ----------------------------------------------------------------------------
|  TaskHandler_070ee0  @ $070EE0  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ee0, "ax", @progbits
        .global TaskHandler_070ee0
TaskHandler_070ee0:
        lea     TaskHandler_070878(pc),a1       | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  TaskHandler_070ef2  @ $070EF2  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070ef2, "ax", @progbits
        .global TaskHandler_070ef2
TaskHandler_070ef2:
        lea     TaskHandler_0708b2(pc),a1       | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  TaskHandler_070f04  @ $070F04  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070f04, "ax", @progbits
        .global TaskHandler_070f04
TaskHandler_070f04:
        lea     TaskHandler_0708f4(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0xffd4,0x22(a0)                | +010
        addi.w  #0xffc8,0x24(a0)                | +016
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_070f22  @ $070F22  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070f22, "ax", @progbits
        .global TaskHandler_070f22
TaskHandler_070f22:
        lea     TaskHandler_06fcfa(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x9b(a6),0x9b(a0)               | +010
        move.b  0x73(a6),0x73(a0)               | +016
        move.b  0x87(a6),0x87(a0)               | +01c
        move.b  0x88(a6),0x88(a0)               | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_070f4c  @ $070F4C  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070f4c, "ax", @progbits
        .global TaskHandler_070f4c
TaskHandler_070f4c:
        lea     TaskHandler_0702a8(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  #0xd7,0x84(a0)                  | +010
        move.b  #0xf1,0x85(a0)                  | +016
        move.b  #0x1,0x86(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_070f70  @ $070F70  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070f70, "ax", @progbits
        .global TaskHandler_070f70
TaskHandler_070f70:
        lea     TaskHandler_0702a8(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        ori.b   #0x1,0x3a(a0)                   | +010
        move.b  #0xdf,0x84(a0)                  | +016
        move.b  #0xf1,0x85(a0)                  | +01c
        move.b  #0x1,0x86(a0)                   | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_070f9a  @ $070F9A  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070f9a, "ax", @progbits
        .global TaskHandler_070f9a
TaskHandler_070f9a:
        lea     TaskHandler_0702e4(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  #0xd7,0x84(a0)                  | +010
        move.b  #0xf1,0x85(a0)                  | +016
        move.b  #0x1,0x86(a0)                   | +01c
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_070fbe  @ $070FBE  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070fbe, "ax", @progbits
        .global TaskHandler_070fbe
TaskHandler_070fbe:
        lea     TaskHandler_0702e4(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        ori.b   #0x1,0x3a(a0)                   | +010
        move.b  #0xdf,0x84(a0)                  | +016
        move.b  #0xf1,0x85(a0)                  | +01c
        move.b  #0x1,0x86(a0)                   | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  TaskHandler_070fe8  @ $070FE8  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_070fe8, "ax", @progbits
        .global TaskHandler_070fe8
TaskHandler_070fe8:
        lea     TaskHandler_07016e(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     TaskHandler_0700ae(pc),a1       | +010
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        lea     TaskHandler_07010e(pc),a1       | +020
        jsr     0x4ae.l                         | +024

| ----------------------------------------------------------------------------
|  TaskHandler_07101a  @ $07101A  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07101a, "ax", @progbits
        .global TaskHandler_07101a
TaskHandler_07101a:
        lea     0x77efe.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        addi.w  #0x38,0x24(a0)                  | +012
        addq.w  #0x4,0x38(a0)                   | +018
        rts                                     | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_071038  @ $071038  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071038, "ax", @progbits
        .global TaskHandler_071038
TaskHandler_071038:
        cmpi.w  #0xb,0x76(a6)                   | +000
        bcs.w   .L07104a                        | +006
        move.w  d0,d0                           | +00a
        move.w  d1,d1                           | +00c
        move.w  d2,d2                           | +00e
        trap    #0xf                            | +010
.L07104a:
        rts                                     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_07104c  @ $07104C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07104c, "ax", @progbits
        .global TaskHandler_07104c
TaskHandler_07104c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_071062                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_071068  @ $071068  (116 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071068, "ax", @progbits
        .global TaskHandler_071068
TaskHandler_071068:
        move.w  #0x28,d0                        | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_071160(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.w  #0xffc0,0x22(a0)                | +01a
        move.w  #0x140,0x24(a0)                 | +020
        move.b  #0x1,0x3a(a0)                   | +026
        move.b  #0x1,0x77(a0)                   | +02c
        move.b  #0x1,0x9f(a0)                   | +032
        lea     TaskHandler_07122e(pc),a1       | +038
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        move.w  #0x180,0x22(a0)                 | +048
        move.w  #0x1b0,0x24(a0)                 | +04e
        move.b  #0x1,0x77(a0)                   | +054
        clr.b   0x9f(a0)                        | +05a
        move.b  #0x0,0x20(a6)                   | +05e
        lea     TaskHandler_0710dc(pc),a1       | +064
        move.l  a1,(a6)                         | +068
        clr.b   0x21(a6)                        | +06a
        clr.b   0x76(a6)                        | +06e
        rts                                     | +072

| ----------------------------------------------------------------------------
|  TaskHandler_0710dc  @ $0710DC  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0710dc, "ax", @progbits
        .global TaskHandler_0710dc
TaskHandler_0710dc:
        cmpi.b  #0x1,0x76(a6)                   | +000
        bge.w   .L0710f2                        | +006
        clr.b   0x106ed3.l                      | +00a
        lea     .L0710f2(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L0710f2:
        cmpi.b  #0x2,0x21(a6)                   | +016
        bge.w   .L071112                        | +01c
        move.b  #0x1,0x20(a6)                   | +020
        cmpi.b  #0x0,0x21(a6)                   | +026
        bne.w   .L071112                        | +02c
        lea     TaskHandler_07111c(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L071112:
        clr.b   0x21(a6)                        | +036
        clr.b   0x76(a6)                        | +03a
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_07111c  @ $07111C  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07111c, "ax", @progbits
        .global TaskHandler_07111c
TaskHandler_07111c:
        move.w  #0x1071,d0                      | +000
        jsr     0x2222.l                        | +004
        move.w  #0x1e,0x70(a6)                  | +00a
        lea     .L071132(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L071132:
        subq.w  #0x1,0x70(a6)                   | +016
        cmpi.w  #0x0,0x70(a6)                   | +01a
        bgt.w   .L07114c                        | +020
        clr.b   0x106ed2.l                      | +024
        jmp     0x518.l                         | +02a
.L07114c:
        rts                                     | +030

| ----------------------------------------------------------------------------
|  TaskHandler_07114e  @ $07114E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07114e, "ax", @progbits
        .global TaskHandler_07114e
TaskHandler_07114e:
        clr.b   0x77(a6)                        | +000
        move.w  #0x1e5,d1                       | +004
        jsr     0x236e.l                        | +008
        bra.w   TaskHandler_071160__L071182     | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_071160  @ $071160  (206 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071160, "ax", @progbits
        .global TaskHandler_071160
TaskHandler_071160:
        bset    #0x1,0x12(a6)                   | +000
        move.w  #0xeac,d0                       | +006
        move.w  #0xea,d1                        | +00a
        jsr     0x440d0.l                       | +00e
        move.w  d1,0x24(a6)                     | +014
        move.w  #0x5b,d1                        | +018
        jsr     0x236e.l                        | +01c
        .global TaskHandler_071160__L071182
TaskHandler_071160__L071182:
.L071182:
        move.b  #0x0,0x80(a6)                   | +022
        move.w  #0xc,0x1c(a6)                   | +028
        jsr     0x138fe.l                       | +02e
        lea     TaskHandler_071940(pc),a1       | +034
        jsr     0x4ae.l                         | +038
        jsr     0x5dd02.l                       | +03e
        lea     TaskHandler_07186e(pc),a1       | +044
        jsr     0x4ae.l                         | +048
        jsr     0x5dd02.l                       | +04e
        lea     TaskHandler_071898(pc),a1       | +054
        jsr     0x4ae.l                         | +058
        jsr     0x5dd02.l                       | +05e
        lea     TaskHandler_0718c2(pc),a1       | +064
        jsr     0x4ae.l                         | +068
        jsr     0x5dd02.l                       | +06e
        lea     TaskHandler_0718ec(pc),a1       | +074
        jsr     0x4ae.l                         | +078
        jsr     0x5dd02.l                       | +07e
        move.b  #0x1,0x84(a6)                   | +084
        move.b  #0x1,0x85(a6)                   | +08a
        move.b  #0x1,0x86(a6)                   | +090
        move.b  #0x1,0x87(a6)                   | +096
        move.b  #0x0,0x83(a6)                   | +09c
        lea     0x2ba79e.l,a0                   | +0a2
        jsr     0x799de.l                       | +0a8
        move.w  d0,0x66(a6)                     | +0ae
        bra.w   TaskHandler_07122e__L0712e0     | +0b2
        move.b  #0x1,0x3a(a6)                   | +0b6
        clr.b   0x77(a6)                        | +0bc
        move.w  #0x1e5,d1                       | +0c0
        jsr     0x236e.l                        | +0c4
        bra.w   TaskHandler_07122e__L071250     | +0ca

| ----------------------------------------------------------------------------
|  TaskHandler_07122e  @ $07122E  (364 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07122e, "ax", @progbits
        .global TaskHandler_07122e
TaskHandler_07122e:
        bset    #0x1,0x12(a6)                   | +000
        move.w  #0xeac,d0                       | +006
        move.w  #0x97,d1                        | +00a
        jsr     0x440d0.l                       | +00e
        move.w  d1,0x24(a6)                     | +014
        move.w  #0x5c,d1                        | +018
        jsr     0x236e.l                        | +01c
        .global TaskHandler_07122e__L071250
TaskHandler_07122e__L071250:
.L071250:
        move.b  #0x1,0x80(a6)                   | +022
        move.w  #0xc,0x1c(a6)                   | +028
        jsr     0x138fe.l                       | +02e
        lea     TaskHandler_071940(pc),a1       | +034
        jsr     0x4ae.l                         | +038
        jsr     0x5dd02.l                       | +03e
        lea     TaskHandler_07186e(pc),a1       | +044
        jsr     0x4ae.l                         | +048
        jsr     0x5dd02.l                       | +04e
        lea     TaskHandler_071898(pc),a1       | +054
        jsr     0x4ae.l                         | +058
        jsr     0x5dd02.l                       | +05e
        lea     TaskHandler_0718c2(pc),a1       | +064
        jsr     0x4ae.l                         | +068
        jsr     0x5dd02.l                       | +06e
        lea     TaskHandler_071916(pc),a1       | +074
        jsr     0x4ae.l                         | +078
        jsr     0x5dd02.l                       | +07e
        move.b  #0x1,0x84(a6)                   | +084
        move.b  #0x1,0x85(a6)                   | +08a
        move.b  #0x1,0x86(a6)                   | +090
        move.b  #0x1,0x87(a6)                   | +096
        move.b  #0x0,0x83(a6)                   | +09c
        lea     0x2ba820.l,a0                   | +0a2
        jsr     0x799de.l                       | +0a8
        move.w  d0,0x66(a6)                     | +0ae
        .global TaskHandler_07122e__L0712e0
TaskHandler_07122e__L0712e0:
.L0712e0:
        lea     Sub_000723D2(pc),a1             | +0b2  -> $0723D2 (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +0b6
        jsr     0x5dd02.l                       | +0bc
        clr.w   0x98(a0)                        | +0c2
        move.w  #0x4001,d0                      | +0c6
        jsr     0x28134.l                       | +0ca
        andi.w  #0xffe3,0x38(a6)                | +0d0
        ori.w   #0x14,0x38(a6)                  | +0d6
        tst.b   0x9f(a6)                        | +0dc
        beq.w   .L071328                        | +0e0
        move.w  #0x8001,d0                      | +0e4
        jsr     0x28134.l                       | +0e8
        andi.w  #0xffe3,0x38(a6)                | +0ee
        ori.w   #0x14,0x38(a6)                  | +0f4
.L071328:
        jsr     0x5e9b6.l                       | +0fa
        andi.w  #0x3f,d0                        | +100
        move.w  d0,0x72(a6)                     | +104
        jsr     0x5e9b6.l                       | +108
        andi.w  #0x3f,d0                        | +10e
        move.w  d0,0x74(a6)                     | +112
        move.b  #0x0,0x88(a6)                   | +116
        move.b  #0x0,0x8a(a6)                   | +11c
        move.l  #0x2d90e4,0x60(a6)              | +122
        move.l  #0x2d9130,0x94(a6)              | +12a
        move.l  #0x2d898c,0x48(a6)              | +132
        move.l  #0x2d9068,0x4c(a6)              | +13a
        move.w  #0x1,0x70(a6)                   | +142
        move.w  #0xff40,d0                      | +148
        jsr     Sub_00072782(pc)                | +14c  -> $072782 (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +150
        lea     .L071388(pc),a1                 | +154
        move.l  a1,(a6)                         | +158
.L071388:
        jsr     Sub_0007279A(pc)                | +15a  -> $07279A (hueco futuro, defsym forward)
        subq.w  #0x1,0x70(a6)                   | +15e
        cmpi.w  #0x0,0x70(a6)                   | +162
        bgt.w   SetHandlerRts_0713a0            | +168

| ----------------------------------------------------------------------------
|  TaskHandler_0713a2  @ $0713A2  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0713a2, "ax", @progbits
        .global TaskHandler_0713a2
TaskHandler_0713a2:
        jsr     0x5e7c0.l                       | +000
        bra.w   TaskHandler_0713ac              | +006

| ----------------------------------------------------------------------------
|  TaskHandler_0713ac  @ $0713AC  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0713ac, "ax", @progbits
        .global TaskHandler_0713ac
TaskHandler_0713ac:
        move.w  0x28(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L0713bc                        | +00a
        neg.w   d0                              | +00e
.L0713bc:
        cmpi.w  #0x0,d0                         | +010
        bgt.w   .L0713d4                        | +014
        lea     0x2d9332.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        bra.w   .L0713e0                        | +024
.L0713d4:
        lea     0x2d9362.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
.L0713e0:
        move.w  #0x1071,d0                      | +034
        jsr     0x2352.l                        | +038
        lea     .L0713f0(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L0713f0:
        jsr     0x28998.l                       | +044
        jsr     Sub_00072C44(pc)                | +04a  -> $072C44 (hueco futuro, defsym forward)
        jsr     0x27afc.l                       | +04e
        bcc.w   .L07140a                        | +054
        lea     TaskHandler_071448(pc),a1       | +058
        move.l  a1,(a6)                         | +05c
.L07140a:
        jsr     0x28d70.l                       | +05e
        jsr     Sub_000726D4(pc)                | +064  -> $0726D4 (hueco futuro, defsym forward)
        bcc.w   .L07141e                        | +068
        lea     TaskHandler_071448(pc),a1       | +06c
        move.l  a1,(a6)                         | +070
.L07141e:
        jsr     Sub_00072750(pc)                | +072  -> $072750 (hueco futuro, defsym forward)
        bcc.w   .L07142c                        | +076
        lea     TaskHandler_071448(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L07142c:
        jsr     0x283ca.l                       | +080
        jsr     0x283d8.l                       | +086
        bra.w   TaskHandler_071664              | +08c
        bra.b   .L0713e0                        | +090

| ----------------------------------------------------------------------------
|  TaskHandler_07143e  @ $07143E  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07143e, "ax", @progbits
        .global TaskHandler_07143e
TaskHandler_07143e:
        move.w  #0x1,0x70(a6)                   | +000
        bra.w   TaskHandler_071448__L07144e     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_071448  @ $071448  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071448, "ax", @progbits
        .global TaskHandler_071448
TaskHandler_071448:
        move.w  #0x3c,0x70(a6)                  | +000
        .global TaskHandler_071448__L07144e
TaskHandler_071448__L07144e:
.L07144e:
        neg.w   0x28(a6)                        | +006
        move.w  #0x1071,d0                      | +00a
        jsr     0x2222.l                        | +00e
        lea     0x2d9392.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L07146e(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L07146e:
        jsr     0x28998.l                       | +026
        jsr     Sub_00072C44(pc)                | +02c  -> $072C44 (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +030
        jsr     0x28d70.l                       | +036
        bcc.w   .L07148e                        | +03c
        lea     TaskHandler_0714a4(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L07148e:
        jsr     0x283ca.l                       | +046
        jsr     0x283d8.l                       | +04c
        bra.w   TaskHandler_071664              | +052

| ----------------------------------------------------------------------------
|  TaskHandler_07149e  @ $07149E  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07149e, "ax", @progbits
        .global TaskHandler_07149e
TaskHandler_07149e:
        move.w  #0x3c,0x70(a6)                  | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0714a4  @ $0714A4  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0714a4, "ax", @progbits
        .global TaskHandler_0714a4
TaskHandler_0714a4:
        lea     0x2d93cc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0714b6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0714b6:
        jsr     0x28998.l                       | +012
        jsr     Sub_00072C44(pc)                | +018  -> $072C44 (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        subq.w  #0x1,0x70(a6)                   | +028
        cmpi.w  #0x0,0x70(a6)                   | +02c
        bne.w   .L0714e0                        | +032
        lea     TaskHandler_0714e4(pc),a1       | +036
        move.l  a1,(a6)                         | +03a
.L0714e0:
        bra.w   TaskHandler_071664              | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_0714e4  @ $0714E4  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0714e4, "ax", @progbits
        .global TaskHandler_0714e4
TaskHandler_0714e4:
        lea     0x2d93ee.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0714f6(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0714f6:
        jsr     0x28998.l                       | +012
        jsr     Sub_00072C44(pc)                | +018  -> $072C44 (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L071516                        | +028
        lea     TaskHandler_0713ac(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L071516:
        jsr     0x283ca.l                       | +032
        jsr     0x283d8.l                       | +038
        bra.w   TaskHandler_071664              | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_071526  @ $071526  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071526, "ax", @progbits
        .global TaskHandler_071526
TaskHandler_071526:
        bclr    #0x3,0x13(a6)                   | +000
        lea     0x2d9428.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   .L07154e                        | +012
        bclr    #0x3,0x13(a6)                   | +016
        lea     0x2d9462.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
.L07154e:
        move.w  #0x1071,d0                      | +028
        jsr     0x2222.l                        | +02c
        lea     .L07155e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L07155e:
        jsr     0x28998.l                       | +038
        jsr     Sub_00072C44(pc)                | +03e  -> $072C44 (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +042
        jsr     0x28d70.l                       | +048
        bcc.w   .L07157e                        | +04e
        lea     TaskHandler_0714e4(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L07157e:
        bra.w   TaskHandler_071664              | +058

| ----------------------------------------------------------------------------
|  TaskHandler_071582  @ $071582  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071582, "ax", @progbits
        .global TaskHandler_071582
TaskHandler_071582:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        bclr    #0x1,0x12(a6)                   | +00a
        lea     0x2d94f0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        move.b  #0x5,0x84(a6)                   | +01c
        move.b  #0x5,0x85(a6)                   | +022
        move.b  #0x5,0x86(a6)                   | +028
        move.b  #0x5,0x87(a6)                   | +02e
        move.b  #0x5,0x83(a6)                   | +034
        lea     .L0715c2(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L0715c2:
        jsr     0x2783a.l                       | +040
        jsr     0x28d70.l                       | +046
        bcc.w   .L0715d8                        | +04c
        lea     TaskHandler_0715fa(pc),a1       | +050
        move.l  a1,(a6)                         | +054
.L0715d8:
        jsr     Sub_000727C4(pc)                | +056  -> $0727C4 (hueco futuro, defsym forward)
        movea.l #0xffffffff,a0                  | +05a
        lea     0x2d90bc.l,a0                   | +060
        jsr     0x5dd5c.l                       | +066
        bcc.w   SetHandlerRts_0715f8            | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_0715fa  @ $0715FA  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0715fa, "ax", @progbits
        .global TaskHandler_0715fa
TaskHandler_0715fa:
        bclr    #0x1,0x12(a6)                   | +000
        move.w  #0x102f,d0                      | +006
        jsr     0x2352.l                        | +00a
        lea     0x77fd6.l,a1                    | +010
        jsr     0x4ae.l                         | +016
        jsr     0x5dd02.l                       | +01c
        addi.w  #0xffc8,0x22(a0)                | +022
        lea     0x77fd6.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        addi.w  #0x0,0x22(a0)                   | +03a
        lea     0x77fd6.l,a1                    | +040
        jsr     0x4ae.l                         | +046
        jsr     0x5dd02.l                       | +04c
        addi.w  #0x38,0x22(a0)                  | +052
        jsr     Sub_00072DB8(pc)                | +058  -> $072DB8 (hueco futuro, defsym forward)
        jsr     0x28d70.l                       | +05c
        jmp     0x518.l                         | +062

| ----------------------------------------------------------------------------
|  TaskHandler_071662  @ $071662  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071662, "ax", @progbits
        .global TaskHandler_071662
TaskHandler_071662:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_071664  @ $071664  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071664, "ax", @progbits
        .global TaskHandler_071664
TaskHandler_071664:
        jsr     Sub_0007279A(pc)                | +000  -> $07279A (hueco futuro, defsym forward)
        jsr     Sub_000727EA(pc)                | +004  -> $0727EA (hueco futuro, defsym forward)
        jsr     0x2870a.l                       | +008
        bcc.w   .L071688                        | +00e
        lea     0x5e766.l,a0                    | +012
        jsr     0x5e770.l                       | +018
        bclr    #0x3,0x13(a6)                   | +01e
.L071688:
        jsr     0x28758.l                       | +024
        bcc.w   .L071698                        | +02a
        lea     TaskHandler_071582(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L071698:
        movea.l #0xffffffff,a0                  | +034
        lea     0x2d90bc.l,a0                   | +03a
        jsr     0x5dd5c.l                       | +040
        bcc.w   SetHandlerRts_0716b4            | +046

| ----------------------------------------------------------------------------
|  TaskHandler_0716b6  @ $0716B6  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0716b6, "ax", @progbits
        .global TaskHandler_0716b6
TaskHandler_0716b6:
        bclr    #0x1,0x12(a6)                   | +000
        move.b  #0x5,0x84(a6)                   | +006
        move.b  #0x5,0x85(a6)                   | +00c
        move.b  #0x5,0x86(a6)                   | +012
        move.b  #0x5,0x87(a6)                   | +018
        move.b  #0x5,0x83(a6)                   | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_071700  @ $071700  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071700, "ax", @progbits
        .global TaskHandler_071700
TaskHandler_071700:
        movea.l 0xc(a6),a0                      | +000
        move.b  0x77(a0),0x77(a6)               | +004
        jsr     Sub_00072B96(pc)                | +00a  -> $072B96 (hueco futuro, defsym forward)
        move.w  0x88(a6),d0                     | +00e
        cmpi.w  #0x5,d0                         | +012
        bcs.w   .L071726                        | +016
        nop                                     | +01a
        nop                                     | +01c
        cmpi.w  #0x5,d0                         | +01e
        nop                                     | +022
        trap    #0xf                            | +024
.L071726:
        movea.l #0x2d92a6,a0                    | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L071742                        | +038
        jsr     0x28cd4.l                       | +03c
        .global TaskHandler_071700__L071742
TaskHandler_071700__L071742:
.L071742:
        lea     .L071748(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L071748:
        movea.l 0xc(a6),a0                      | +048
        move.b  0x20(a0),0x20(a6)               | +04c
        jsr     Sub_00072BCE(pc)                | +052  -> $072BCE (hueco futuro, defsym forward)
        jsr     Sub_00072BDE(pc)                | +056  -> $072BDE (hueco futuro, defsym forward)
        lea     0x2d9272.l,a0                   | +05a
        jsr     Sub_00072C08(pc)                | +060  -> $072C08 (hueco futuro, defsym forward)
        bcc.w   .L07176a                        | +064
        jmp     (a0)                            | +068
.L07176a:
        movea.l 0xc(a6),a0                      | +06a
        btst    #0x7,0x5a(a0)                   | +06e
        beq.w   JsrAbsThunk_07177e              | +074
        bset    #0x0,0x5a(a6)                   | +078

| ----------------------------------------------------------------------------
|  TaskHandler_071786  @ $071786  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071786, "ax", @progbits
        .global TaskHandler_071786
TaskHandler_071786:
        move.w  0x88(a6),d0                     | +000
        cmpi.w  #0x5,d0                         | +004
        bcs.w   .L07179e                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0x5,d0                         | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L07179e:
        movea.l #0x2d92ba,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L0717ba                        | +02a
        jsr     0x28cd4.l                       | +02e
.L0717ba:
        bra.w   TaskHandler_071700__L071742     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_0717be  @ $0717BE  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0717be, "ax", @progbits
        .global TaskHandler_0717be
TaskHandler_0717be:
        move.w  0x88(a6),d0                     | +000
        cmpi.w  #0x5,d0                         | +004
        bcs.w   .L0717d6                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0x5,d0                         | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L0717d6:
        movea.l #0x2d92ce,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L0717f2                        | +02a
        jsr     0x28cd4.l                       | +02e
.L0717f2:
        bra.w   TaskHandler_071700__L071742     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_0717f6  @ $0717F6  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0717f6, "ax", @progbits
        .global TaskHandler_0717f6
TaskHandler_0717f6:
        move.w  0x88(a6),d0                     | +000
        cmpi.w  #0x5,d0                         | +004
        bcs.w   .L07180e                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0x5,d0                         | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L07180e:
        movea.l #0x2d92e2,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L07182a                        | +02a
        jsr     0x28cd4.l                       | +02e
.L07182a:
        bra.w   TaskHandler_071700__L071742     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_07182e  @ $07182E  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07182e, "ax", @progbits
        .global TaskHandler_07182e
TaskHandler_07182e:
        move.w  0x88(a6),d0                     | +000
        cmpi.w  #0x5,d0                         | +004
        bcs.w   .L071846                        | +008
        nop                                     | +00c
        nop                                     | +00e
        cmpi.w  #0x5,d0                         | +010
        nop                                     | +014
        trap    #0xf                            | +016
.L071846:
        movea.l #0x2d92f6,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L071862                        | +02a
        jsr     0x28cd4.l                       | +02e
.L071862:
        bra.w   TaskHandler_071700__L071742     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_07186e  @ $07186E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_07186e, "ax", @progbits
        .global TaskHandler_07186e
TaskHandler_07186e:
        move.w  #0x0,0x84(a6)                   | +000
        move.w  #0x84,0x86(a6)                  | +006
        move.l  #0x2d9138,0x80(a6)              | +00c
        move.w  #0x0,0x88(a6)                   | +014
        lea     0x2d9508.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   TaskHandler_071700              | +026

| ----------------------------------------------------------------------------
|  TaskHandler_071898  @ $071898  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071898, "ax", @progbits
        .global TaskHandler_071898
TaskHandler_071898:
        move.w  #0xffff,0x84(a6)                | +000
        move.w  #0x85,0x86(a6)                  | +006
        move.l  #0x2d9138,0x80(a6)              | +00c
        move.w  #0x1,0x88(a6)                   | +014
        lea     0x2d961c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   TaskHandler_071700              | +026

| ----------------------------------------------------------------------------
|  TaskHandler_0718c2  @ $0718C2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0718c2, "ax", @progbits
        .global TaskHandler_0718c2
TaskHandler_0718c2:
        move.w  #0xffff,0x84(a6)                | +000
        move.w  #0x86,0x86(a6)                  | +006
        move.l  #0x2d9168,0x80(a6)              | +00c
        move.w  #0x2,0x88(a6)                   | +014
        lea     0x2d9714.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   TaskHandler_071700              | +026

| ----------------------------------------------------------------------------
|  TaskHandler_0718ec  @ $0718EC  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0718ec, "ax", @progbits
        .global TaskHandler_0718ec
TaskHandler_0718ec:
        move.w  #0xffff,0x84(a6)                | +000
        move.w  #0x87,0x86(a6)                  | +006
        move.l  #0x2d9198,0x80(a6)              | +00c
        move.w  #0x3,0x88(a6)                   | +014
        lea     0x2d9828.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   TaskHandler_071700              | +026

| ----------------------------------------------------------------------------
|  TaskHandler_071916  @ $071916  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071916, "ax", @progbits
        .global TaskHandler_071916
TaskHandler_071916:
        move.w  #0xffff,0x84(a6)                | +000
        move.w  #0x87,0x86(a6)                  | +006
        move.l  #0x2d9198,0x80(a6)              | +00c
        move.w  #0x4,0x88(a6)                   | +014
        lea     0x2d990a.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   TaskHandler_071700              | +026

| ----------------------------------------------------------------------------
|  TaskHandler_071940  @ $071940  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071940, "ax", @progbits
        .global TaskHandler_071940
TaskHandler_071940:
        move.w  #0x0,0x84(a6)                   | +000
        move.w  #0x83,0x86(a6)                  | +006
        move.l  #0x2d91c8,0x80(a6)              | +00c
        move.w  #0x5,0x88(a6)                   | +014
        jsr     0x5e9b6.l                       | +01a
        andi.w  #0x3f,d0                        | +020
        addi.w  #0x1e,d0                        | +024
        move.w  d0,0x72(a6)                     | +028
        move.w  #0x38,d1                        | +02c
        jsr     0x236e.l                        | +030

| ----------------------------------------------------------------------------
|  TaskHandler_071976  @ $071976  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071976, "ax", @progbits
        .global TaskHandler_071976
TaskHandler_071976:
        move.b  #0x1,0x8a(a6)                   | +000
        move.w  #0x1,0x66(a6)                   | +006
        bclr    #0x3,0x13(a6)                   | +00c
        bclr    #0x0,0x13(a6)                   | +012
        lea     0x2da2d0.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        jsr     0x28d70.l                       | +024
        lea     .L0719a6(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L0719a6:
        movea.l 0xc(a6),a0                      | +030
        move.b  0x20(a0),0x20(a6)               | +034
        jsr     Sub_00072B5A(pc)                | +03a  -> $072B5A (hueco futuro, defsym forward)
        bcc.w   .L0719be                        | +03e
        lea     TaskHandler_071a7a(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L0719be:
        movea.l 0xc(a6),a1                      | +048
        move.w  0x86(a6),d1                     | +04c
        cmpi.b  #0x5,(a1,d1.w)                  | +050
        bne.w   .L0719d6                        | +056
        lea     TaskHandler_071b9a(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L0719d6:
        lea     0x2d928e.l,a0                   | +060
        jsr     Sub_00072C08(pc)                | +066  -> $072C08 (hueco futuro, defsym forward)
        bcc.w   .L0719e6                        | +06a
        jmp     (a0)                            | +06e
.L0719e6:
        rts                                     | +070

| ----------------------------------------------------------------------------
|  TaskHandler_0719e8  @ $0719E8  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0719e8, "ax", @progbits
        .global TaskHandler_0719e8
TaskHandler_0719e8:
        move.w  #0x1,0x66(a6)                   | +000
        bclr    #0x3,0x13(a6)                   | +006
        bclr    #0x0,0x13(a6)                   | +00c
        lea     .L071a00(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L071a00:
        jsr     Sub_00072BCE(pc)                | +018  -> $072BCE (hueco futuro, defsym forward)
        jsr     Sub_00072BDE(pc)                | +01c  -> $072BDE (hueco futuro, defsym forward)
        jsr     0x28d70.l                       | +020
        bcc.w   .L071a18                        | +026
        lea     TaskHandler_071976(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L071a18:
        jsr     0x2870a.l                       | +030
        bcc.w   .L071a2e                        | +036
        jsr     0x519be.l                       | +03a
        lea     TaskHandler_071a90(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L071a2e:
        movea.l 0xc(a6),a1                      | +046
        move.w  0x86(a6),d1                     | +04a
        move.b  (a1,d1.w),d0                    | +04e
        clr.b   (a1,d1.w)                       | +052
        cmpi.b  #0x5,d0                         | +056
        bne.w   SetHandlerRts_071a4c            | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_071a4e  @ $071A4E  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071a4e, "ax", @progbits
        .global TaskHandler_071a4e
TaskHandler_071a4e:
        move.b  #0x2,0x8a(a6)                   | +000
        lea     0x2da2e2.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   TaskHandler_0719e8              | +012

| ----------------------------------------------------------------------------
|  TaskHandler_071a64  @ $071A64  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071a64, "ax", @progbits
        .global TaskHandler_071a64
TaskHandler_071a64:
        move.b  #0x3,0x8a(a6)                   | +000
        lea     0x2da380.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   TaskHandler_0719e8              | +012

| ----------------------------------------------------------------------------
|  TaskHandler_071a7a  @ $071A7A  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071a7a, "ax", @progbits
        .global TaskHandler_071a7a
TaskHandler_071a7a:
        move.b  #0x4,0x8a(a6)                   | +000
        lea     0x2da41e.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   TaskHandler_0719e8              | +012

| ----------------------------------------------------------------------------
|  TaskHandler_071a90  @ $071A90  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071a90, "ax", @progbits
        .global TaskHandler_071a90
TaskHandler_071a90:
        cmpi.b  #0x4,0x8a(a6)                   | +000
        bne.w   .L071aaa                        | +006
        lea     Sub_0007229C(pc),a1             | +00a  -> $07229C (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
.L071aaa:
        move.b  #0x5,0x8a(a6)                   | +01a
        move.w  #0x1041,d0                      | +020
        jsr     0x2352.l                        | +024
        lea     0x2c41ea.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L071acc(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L071acc:
        jsr     Sub_00072BCE(pc)                | +03c  -> $072BCE (hueco futuro, defsym forward)
        jsr     Sub_00072BDE(pc)                | +040  -> $072BDE (hueco futuro, defsym forward)
        jsr     0x28d70.l                       | +044
        bcc.w   .L071ae4                        | +04a
        lea     TaskHandler_071b04(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L071ae4:
        movea.l 0xc(a6),a1                      | +054
        move.w  0x86(a6),d1                     | +058
        move.b  (a1,d1.w),d0                    | +05c
        clr.b   (a1,d1.w)                       | +060
        cmpi.b  #0x5,d0                         | +064
        bne.w   SetHandlerRts_071b02            | +068

| ----------------------------------------------------------------------------
|  TaskHandler_071b04  @ $071B04  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071b04, "ax", @progbits
        .global TaskHandler_071b04
TaskHandler_071b04:
        move.b  #0x3c,0x70(a6)                  | +000
        lea     .L071b10(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L071b10:
        jsr     Sub_00072BCE(pc)                | +00c  -> $072BCE (hueco futuro, defsym forward)
        jsr     Sub_00072BDE(pc)                | +010  -> $072BDE (hueco futuro, defsym forward)
        subq.b  #0x1,0x70(a6)                   | +014
        bne.w   .L071b26                        | +018
        lea     TaskHandler_071b56(pc),a1       | +01c
        move.l  a1,(a6)                         | +020
.L071b26:
        btst    #0x0,0x70(a6)                   | +022
        bne.w   .L071b36                        | +028
        jsr     0x28d70.l                       | +02c
.L071b36:
        movea.l 0xc(a6),a1                      | +032
        move.w  0x86(a6),d1                     | +036
        move.b  (a1,d1.w),d0                    | +03a
        clr.b   (a1,d1.w)                       | +03e
        cmpi.b  #0x5,d0                         | +042
        bne.w   SetHandlerRts_071b54            | +046

| ----------------------------------------------------------------------------
|  TaskHandler_071b56  @ $071B56  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071b56, "ax", @progbits
        .global TaskHandler_071b56
TaskHandler_071b56:
        lea     0x2badb6.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        lea     .L071b6c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L071b6c:
        subq.w  #0x1,0x70(a6)                   | +016
        bgt.w   .L071b7a                        | +01a
        lea     TaskHandler_071976(pc),a1       | +01e
        move.l  a1,(a6)                         | +022
.L071b7a:
        movea.l 0xc(a6),a1                      | +024
        move.w  0x86(a6),d1                     | +028
        move.b  (a1,d1.w),d0                    | +02c
        clr.b   (a1,d1.w)                       | +030
        cmpi.b  #0x5,d0                         | +034
        bne.w   SetHandlerRts_071b98            | +038

| ----------------------------------------------------------------------------
|  TaskHandler_071b9a  @ $071B9A  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071b9a, "ax", @progbits
        .global TaskHandler_071b9a
TaskHandler_071b9a:
        cmpi.b  #0x4,0x8a(a6)                   | +000
        bne.w   .L071bb4                        | +006
        lea     Sub_0007229C(pc),a1             | +00a  -> $07229C (hueco futuro, defsym forward)
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
.L071bb4:
        jsr     Sub_00072BCE(pc)                | +01a  -> $072BCE (hueco futuro, defsym forward)
        jsr     Sub_00072BDE(pc)                | +01e  -> $072BDE (hueco futuro, defsym forward)
        jsr     0x28d70.l                       | +022
        move.w  #0xd000,d0                      | +028
        jsr     0x28134.l                       | +02c
        andi.w  #0xffe3,0x38(a6)                | +032
        ori.w   #0x14,0x38(a6)                  | +038
        jsr     0x4a0d4.l                       | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_071be6  @ $071BE6  (150 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071be6, "ax", @progbits
        .global TaskHandler_071be6
TaskHandler_071be6:
        lea     0x2d9cbe.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L071c02                        | +00c
        lea     0x2d9d66.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L071c02:
        movea.l 0xc(a6),a0                      | +01c
        tst.b   0x77(a0)                        | +020
        beq.w   .L071c16                        | +024
        move.w  #0x5d,d1                        | +028
        bra.w   .L071c1a                        | +02c
.L071c16:
        move.w  #0x1e6,d1                       | +030
.L071c1a:
        jsr     0x236e.l                        | +034
        bset    #0x4,0x6b(a6)                   | +03a
        move.w  #0x0,0x84(a6)                   | +040
        lea     .L071c32(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L071c32:
        jsr     Sub_00072BCE(pc)                | +04c  -> $072BCE (hueco futuro, defsym forward)
        move.w  0x9a(a6),d0                     | +050
        add.w   d0,0x22(a6)                     | +054
        move.w  0x9c(a6),d0                     | +058
        add.w   d0,0x24(a6)                     | +05c
        jsr     0x28d70.l                       | +060
        bcc.w   .L071c56                        | +066
        lea     TaskHandler_071c84(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
.L071c56:
        jsr     0x5e45a.l                       | +070
        bcc.w   .L071c66                        | +076
        lea     TaskHandler_071dba(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L071c66:
        movea.l #0xffffffff,a0                  | +080
        lea     0x2d90c4.l,a0                   | +086
        jsr     0x5dd56.l                       | +08c
        bcc.w   SetHandlerRts_071c82            | +092

| ----------------------------------------------------------------------------
|  TaskHandler_071c84  @ $071C84  (302 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071c84, "ax", @progbits
        .global TaskHandler_071c84
TaskHandler_071c84:
        lea     0x2bae38.l,a0                   | +000
        cmpi.b  #0x1,0x20(a6)                   | +006
        bne.w   .L071c9a                        | +00c
        lea     0x2bae98.l,a0                   | +010
.L071c9a:
        move.b  #0x0,0x3a(a6)                   | +016
        move.b  0x98(a6),d0                     | +01c
        andi.w  #0xf,d0                         | +020
        move.w  d0,d1                           | +024
        add.w   d0,d0                           | +026
        add.w   d1,d0                           | +028
        add.w   d0,d0                           | +02a
        move.w  0x2(a0,d0.w),0x2e(a6)           | +02c
        move.w  0x4(a0,d0.w),0x2a(a6)           | +032
        move.w  (a0,d0.w),d0                    | +038
        jsr     Sub_00072782(pc)                | +03c  -> $072782 (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +040
        jsr     0x5e5e0.l                       | +044
        bcc.w   .L071cd6                        | +04a
        neg.w   0x28(a6)                        | +04e
.L071cd6:
        move.w  #0x32,0x66(a6)                  | +052
        move.l  #0x2d8a84,0x48(a6)              | +058
        lea     0x2d9e36.l,a0                   | +060
        jsr     0x28cd4.l                       | +066
        lea     .L071cf6(pc),a1                 | +06c
        move.l  a1,(a6)                         | +070
.L071cf6:
        cmpi.w  #0x0,0x2a(a6)                   | +072
        bgt.w   .L071d0c                        | +078
        move.w  #0xd000,0x38(a6)                | +07c
        lea     .L071d0c(pc),a1                 | +082
        move.l  a1,(a6)                         | +086
.L071d0c:
        jsr     0x27d50.l                       | +088
        bcc.w   .L071d1c                        | +08e
        lea     TaskHandler_071dba(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L071d1c:
        cmpi.w  #0x0,0x2a(a6)                   | +098
        blt.w   .L071d30                        | +09e
        movea.l 0xc(a6),a0                      | +0a2
        move.w  0x38(a0),0x38(a6)               | +0a6
.L071d30:
        move.w  0x28(a6),d0                     | +0ac
        move.w  0x2a(a6),d1                     | +0b0
        asr.w   #0x4,d0                         | +0b4
        asr.w   #0x4,d1                         | +0b6
        jsr     0x5e018.l                       | +0b8
        lsr.w   #0x3,d0                         | +0be
        move.w  d0,0x34(a6)                     | +0c0
        jsr     0x28d70.l                       | +0c4
        jsr     0x283d8.l                       | +0ca
        btst    #0x1,0x13(a6)                   | +0d0
        beq.w   .L071d64                        | +0d6
        lea     Sub_00072364(pc),a1             | +0da  -> $072364 (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +0de
.L071d64:
        jsr     0x2870a.l                       | +0e0
        lea     0x5e766.l,a0                    | +0e6
        jsr     0x5e770.l                       | +0ec
        bclr    #0x3,0x13(a6)                   | +0f2
        jsr     0x28758.l                       | +0f8
        bcc.w   .L071d8c                        | +0fe
        lea     TaskHandler_071dba(pc),a1       | +102
        move.l  a1,(a6)                         | +106
.L071d8c:
        jsr     0x5e45a.l                       | +108
        bcc.w   .L071d9c                        | +10e
        lea     TaskHandler_071dba(pc),a1       | +112
        move.l  a1,(a6)                         | +116
.L071d9c:
        movea.l #0xffffffff,a0                  | +118
        lea     0x2d90c4.l,a0                   | +11e
        jsr     0x5dd56.l                       | +124
        bcc.w   SetHandlerRts_071db8            | +12a

| ----------------------------------------------------------------------------
|  TaskHandler_071dba  @ $071DBA  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071dba, "ax", @progbits
        .global TaskHandler_071dba
TaskHandler_071dba:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   Sub_00072364                    | +00a  -> $072364 (hueco futuro, defsym forward)

| ----------------------------------------------------------------------------
|  TaskHandler_071dc8  @ $071DC8  (128 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071dc8, "ax", @progbits
        .global TaskHandler_071dc8
TaskHandler_071dc8:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x77(a0)                        | +004
        beq.w   .L071ddc                        | +008
        move.w  #0x5d,d1                        | +00c
        bra.w   .L071de0                        | +010
.L071ddc:
        move.w  #0x1e6,d1                       | +014
.L071de0:
        jsr     0x236e.l                        | +018
        move.w  #0x0,0x84(a6)                   | +01e
        lea     0x2d9bb8.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        .global TaskHandler_071dc8__L071df8
TaskHandler_071dc8__L071df8:
.L071df8:
        lea     .L071dfe(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L071dfe:
        jsr     Sub_00072BCE(pc)                | +036  -> $072BCE (hueco futuro, defsym forward)
        move.w  0x9a(a6),d0                     | +03a
        add.w   d0,0x22(a6)                     | +03e
        move.w  0x9c(a6),d0                     | +042
        add.w   d0,0x24(a6)                     | +046
        jsr     0x28d70.l                       | +04a
        bcc.w   .L071e22                        | +050
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +054
        move.l  a1,(a6)                         | +058
.L071e22:
        jsr     0x5e45a.l                       | +05a
        bcc.w   .L071e32                        | +060
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +064
        move.l  a1,(a6)                         | +068
.L071e32:
        movea.l #0xffffffff,a0                  | +06a
        lea     0x2d90c4.l,a0                   | +070
        jsr     0x5dd56.l                       | +076
        bcc.w   SetHandlerRts_071e4e            | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_071e50  @ $071E50  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071e50, "ax", @progbits
        .global TaskHandler_071e50
TaskHandler_071e50:
        move.w  #0x8,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2c4022.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        eori.b  #0x1,0x3a(a6)                   | +016
        move.w  #0x4000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x1c,0x38(a6)                  | +02c
        lea     .L071e88(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L071e88:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L071e9e                        | +044
        lea     Jsr5B6ThenJmpScheduler_0716f2(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L071e9e:
        movea.l #0xffffffff,a0                  | +04e
        jsr     0x5dd56.l                       | +054
        bcc.w   SetHandlerRts_071eb4            | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_071eb6  @ $071EB6  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071eb6, "ax", @progbits
        .global TaskHandler_071eb6
TaskHandler_071eb6:
        move.w  #0x5e,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2d997a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0xffea,d0                      | +016
        jsr     Sub_00072782(pc)                | +01a  -> $072782 (hueco futuro, defsym forward)
        move.w  d0,0x9a(a6)                     | +01e
        move.w  #0x28,0x9c(a6)                  | +022
        clr.w   0x84(a6)                        | +028
        bra.w   TaskHandler_071dc8__L071df8     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_071ee6  @ $071EE6  (132 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071ee6, "ax", @progbits
        .global TaskHandler_071ee6
TaskHandler_071ee6:
        move.w  #0x1,0x84(a6)                   | +000
        move.w  #0x1cf,d1                       | +006
        jsr     0x236e.l                        | +00a
        lea     0x2d9a1c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L071f08(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L071f08:
        jsr     Sub_00072BCE(pc)                | +022  -> $072BCE (hueco futuro, defsym forward)
        move.w  #0xffb0,d0                      | +026
        jsr     Sub_00072782(pc)                | +02a  -> $072782 (hueco futuro, defsym forward)
        add.w   d0,0x22(a6)                     | +02e
        addi.w  #0x38,0x24(a6)                  | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L071f2e                        | +03e
        lea     TaskHandler_071f72(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L071f2e:
        jsr     0x283d8.l                       | +048
        btst    #0x1,0x13(a6)                   | +04e
        beq.w   .L071f44                        | +054
        lea     Sub_0007233A(pc),a1             | +058  -> $07233A (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +05c
.L071f44:
        jsr     0x5e45a.l                       | +05e
        bcc.w   .L071f54                        | +064
        lea     Sub_0007233A(pc),a1             | +068  -> $07233A (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +06c
.L071f54:
        movea.l #0xffffffff,a0                  | +06e
        lea     0x2d90bc.l,a0                   | +074
        jsr     0x5dd56.l                       | +07a
        bcc.w   SetHandlerRts_071f70            | +080

| ----------------------------------------------------------------------------
|  TaskHandler_071f72  @ $071F72  (138 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_071f72, "ax", @progbits
        .global TaskHandler_071f72
TaskHandler_071f72:
        bset    #0x4,0x6b(a6)                   | +000
        lea     0x2baa28.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        neg.w   d0                              | +012
        jsr     Sub_00072782(pc)                | +014  -> $072782 (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +018
        move.w  #0xd000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x1c,0x38(a6)                  | +02c
        move.w  #0x1cf,d1                       | +032
        jsr     0x236e.l                        | +036
        lea     0x2d9a9c.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
        lea     .L071fc0(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L071fc0:
        jsr     0x27cee.l                       | +04e
        bcc.w   .L071fd0                        | +054
        lea     Sub_0007231E(pc),a1             | +058  -> $07231E (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +05c
.L071fd0:
        jsr     0x28d70.l                       | +05e
        jsr     0x283d8.l                       | +064
        btst    #0x1,0x13(a6)                   | +06a
        beq.w   .L071fec                        | +070
        lea     Sub_0007231E(pc),a1             | +074  -> $07231E (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +078
.L071fec:
        jsr     0x5e45a.l                       | +07a
        bcc.w   Sub_00071FFC                    | +080  -> $071FFC (hueco futuro, defsym forward)
        lea     Sub_0007231E(pc),a1             | +084  -> $07231E (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +088
