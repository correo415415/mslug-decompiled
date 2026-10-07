| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $06A000..$06DFE8  (15,616 B, 179 entradas, 71 huecos)
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
|  TaskHandler_06a000  @ $06A000  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a000, "ax", @progbits
        .global TaskHandler_06a000
TaskHandler_06a000:
        andi.w  #0xffe3,0x38(a6)                | +000
        ori.w   #0x14,0x38(a6)                  | +006
        tst.b   0x7d(a6)                        | +00c
        bne.w   JsrAbsRts_06a042                | +010
        move.l  #0x2c8a74,0x4c(a6)              | +014
        jsr     0x283ca.l                       | +01c
        jsr     0x283d8.l                       | +022
        lea     0x2c8c7e.l,a1                   | +028
        tst.b   0x7e(a6)                        | +02e
        beq.w   JsrAbsThunk_06a03c              | +032
        lea     0x2c8ca6.l,a1                   | +036

| ----------------------------------------------------------------------------
|  TaskHandler_06a044  @ $06A044  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a044, "ax", @progbits
        .global TaskHandler_06a044
TaskHandler_06a044:
        move.w  #0xe,d1                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06a050  @ $06A050  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a050, "ax", @progbits
        .global TaskHandler_06a050
TaskHandler_06a050:
        addi.w  #0x0,0x38(a6)                   | +000
        move.w  #0x1a,d1                        | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06a062  @ $06A062  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a062, "ax", @progbits
        .global TaskHandler_06a062
TaskHandler_06a062:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06a078                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06a07e  @ $06A07E  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a07e, "ax", @progbits
        .global TaskHandler_06a07e
TaskHandler_06a07e:
        move.l  #0x2ca238,0x4c(a6)              | +000
        bra.w   .L06a092                        | +008
        move.l  #0x2ca2d8,0x4c(a6)              | +00c
.L06a092:
        jsr     0x283ca.l                       | +014
        clr.b   0x9d(a6)                        | +01a
        clr.b   0x9e(a6)                        | +01e
        jsr     TaskHandler_06b78a(pc)          | +022
        lea     0x2bd468.l,a0                   | +026
        jsr     0x799de.l                       | +02c
        move.w  d0,0x36(a6)                     | +032
        lea     0x2bd4ea.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x70(a6)                     | +042
        lea     0x2bd3e6.l,a0                   | +046
        jsr     0x799de.l                       | +04c
        move.w  d0,0x66(a6)                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06a0d4  @ $06A0D4  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a0d4, "ax", @progbits
        .global TaskHandler_06a0d4
TaskHandler_06a0d4:
        move.b  #0x0,0x72(a6)                   | +000
        lea     0x2bd670.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.b  d0,0x74(a6)                     | +012
        clr.w   0x28(a6)                        | +016
        lea     0x2ca84a.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06a100(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06a100:
        jsr     TaskHandler_06b232(pc)          | +02c
        jsr     0x28d70.l                       | +030
        subq.b  #0x1,0x74(a6)                   | +036
        bgt.w   .L06a118                        | +03a
        lea     TaskHandler_06a20c(pc),a1       | +03e
        move.l  a1,(a6)                         | +042
.L06a118:
        jsr     TaskHandler_06b47e(pc)          | +044
        bcc.w   .L06a126                        | +048
        lea     TaskHandler_06a270(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06a126:
        bra.w   TaskHandler_06a2d0__L06a390     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06a12a  @ $06A12A  (138 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a12a, "ax", @progbits
        .global TaskHandler_06a12a
TaskHandler_06a12a:
        move.b  #0x0,0x72(a6)                   | +000
        move.w  0x36(a6),d0                     | +006
        btst    #0x0,0x75(a6)                   | +00a
        bne.w   .L06a140                        | +010
        neg.w   d0                              | +014
.L06a140:
        move.w  d0,0x28(a6)                     | +016
        jsr     TaskHandler_06b368(pc)          | +01a
        bcs.w   .L06a15c                        | +01e
        lea     0x2ca690.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L06a168                        | +02e
.L06a15c:
        lea     0x2ca74a.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
.L06a168:
        lea     .L06a16e(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L06a16e:
        jsr     TaskHandler_06b232(pc)          | +044
        bcc.w   .L06a182                        | +048
        eori.b  #0x1,0x75(a6)                   | +04c
        lea     TaskHandler_06a1b4(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L06a182:
        jsr     0x28d70.l                       | +058
        jsr     TaskHandler_06b27a(pc)          | +05e
        bcc.w   .L06a196                        | +062
        lea     TaskHandler_06a1b4(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L06a196:
        jsr     TaskHandler_06b47e(pc)          | +06c
        bcc.w   .L06a1a4                        | +070
        lea     TaskHandler_06a1b4(pc),a1       | +074
        move.l  a1,(a6)                         | +078
.L06a1a4:
        jsr     0x283ca.l                       | +07a
        jsr     0x283d8.l                       | +080
        bra.w   TaskHandler_06a2d0__L06a390     | +086

| ----------------------------------------------------------------------------
|  TaskHandler_06a1b4  @ $06A1B4  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a1b4, "ax", @progbits
        .global TaskHandler_06a1b4
TaskHandler_06a1b4:
        clr.w   0x28(a6)                        | +000
        move.b  #0x0,0x72(a6)                   | +004
        jsr     TaskHandler_06b368(pc)          | +00a
        bcs.w   .L06a1d6                        | +00e
        lea     0x2ca996.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        bra.w   .L06a1e2                        | +01e
.L06a1d6:
        lea     0x2ca898.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
.L06a1e2:
        lea     .L06a1e8(pc),a1                 | +02e
        move.l  a1,(a6)                         | +032
.L06a1e8:
        jsr     TaskHandler_06b232(pc)          | +034
        jsr     0x28d70.l                       | +038
        bcc.w   .L06a1fc                        | +03e
        lea     TaskHandler_06a0d4(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L06a1fc:
        jsr     0x283ca.l                       | +048
        jsr     0x283d8.l                       | +04e
        bra.w   TaskHandler_06a2d0__L06a390     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06a20c  @ $06A20C  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a20c, "ax", @progbits
        .global TaskHandler_06a20c
TaskHandler_06a20c:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     TaskHandler_06b368(pc)          | +006
        bcs.w   .L06a22a                        | +00a
        lea     0x2caa94.l,a0                   | +00e
        jsr     0x28cd4.l                       | +014
        bra.w   .L06a236                        | +01a
.L06a22a:
        lea     0x2cabaa.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
.L06a236:
        lea     .L06a23c(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a23c:
        jsr     TaskHandler_06b232(pc)          | +030
        bcc.w   .L06a250                        | +034
        eori.b  #0x1,0x75(a6)                   | +038
        lea     TaskHandler_06a1b4(pc),a1       | +03e
        move.l  a1,(a6)                         | +042
.L06a250:
        jsr     0x28d70.l                       | +044
        bcc.w   .L06a260                        | +04a
        lea     TaskHandler_06a12a(pc),a1       | +04e
        move.l  a1,(a6)                         | +052
.L06a260:
        jsr     0x283ca.l                       | +054
        jsr     0x283d8.l                       | +05a
        bra.w   TaskHandler_06a2d0__L06a390     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_06a270  @ $06A270  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a270, "ax", @progbits
        .global TaskHandler_06a270
TaskHandler_06a270:
        jsr     0x267e2.l                       | +000
        jsr     0x5e0d4.l                       | +006
        move.l  a0,0x90(a6)                     | +00c
        move.b  #0x1,0x72(a6)                   | +010
        clr.b   0x74(a6)                        | +016
        clr.b   0x73(a6)                        | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a2a0(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a2a0:
        jsr     TaskHandler_06b232(pc)          | +030
        jsr     0x28d70.l                       | +034
        tst.b   0x73(a6)                        | +03a
        beq.w   .L06a2b8                        | +03e
        lea     TaskHandler_06a2bc(pc),a1       | +042
        move.l  a1,(a6)                         | +046
.L06a2b8:
        bra.w   TaskHandler_06a2d0__L06a390     | +048

| ----------------------------------------------------------------------------
|  TaskHandler_06a2bc  @ $06A2BC  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a2bc, "ax", @progbits
        .global TaskHandler_06a2bc
TaskHandler_06a2bc:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c
        bra.w   TaskHandler_06a2d0__L06a2de     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06a2d0  @ $06A2D0  (254 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a2d0, "ax", @progbits
        .global TaskHandler_06a2d0
TaskHandler_06a2d0:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   .L06a33e                        | +00a
        .global TaskHandler_06a2d0__L06a2de
TaskHandler_06a2d0__L06a2de:
.L06a2de:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd56c.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cacc0.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a30a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a30a:
        jsr     TaskHandler_06b232(pc)          | +03a
        jsr     0x28d70.l                       | +03e
        tst.b   0x73(a6)                        | +044
        beq.w   .L06a326                        | +048
        clr.b   0x73(a6)                        | +04c
        move.b  #0x4,0x72(a6)                   | +050
.L06a326:
        subq.w  #0x1,0x70(a6)                   | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   .L06a33a                        | +060
        lea     TaskHandler_06a2d0(pc),a1       | +064
        move.l  a1,(a6)                         | +068
.L06a33a:
        bra.w   .L06a390                        | +06a
.L06a33e:
        lea     0x2bd4ea.l,a0                   | +06e
        jsr     0x799de.l                       | +074
        move.w  d0,0x70(a6)                     | +07a
        addi.w  #0x1e,0x70(a6)                  | +07e
        clr.b   0x74(a6)                        | +084
        clr.b   0x73(a6)                        | +088
        move.b  #0x3,0x72(a6)                   | +08c
        lea     0x2ca84a.l,a0                   | +092
        jsr     0x28cd4.l                       | +098
        lea     .L06a374(pc),a1                 | +09e
        move.l  a1,(a6)                         | +0a2
.L06a374:
        jsr     TaskHandler_06b232(pc)          | +0a4
        jsr     0x28d70.l                       | +0a8
        tst.b   0x73(a6)                        | +0ae
        beq.w   .L06a38c                        | +0b2
        lea     TaskHandler_06a20c(pc),a1       | +0b6
        move.l  a1,(a6)                         | +0ba
.L06a38c:
        bra.w   .L06a390                        | +0bc
        .global TaskHandler_06a2d0__L06a390
TaskHandler_06a2d0__L06a390:
.L06a390:
        clr.b   0x78(a6)                        | +0c0
        jsr     0x2870a.l                       | +0c4
        bcc.w   .L06a3b6                        | +0ca
        lea     0x5e766.l,a0                    | +0ce
        jsr     0x5e770.l                       | +0d4
        bclr    #0x3,0x13(a6)                   | +0da
        move.b  #0x1,0x78(a6)                   | +0e0
.L06a3b6:
        jsr     0x28758.l                       | +0e6
        bcc.w   .L06a3c6                        | +0ec
        lea     TaskHandler_06a3d6(pc),a1       | +0f0
        move.l  a1,(a6)                         | +0f4
.L06a3c6:
        jsr     TaskHandler_06b1e2(pc)          | +0f6
        bcc.w   SetHandlerRts_06a3d4            | +0fa

| ----------------------------------------------------------------------------
|  TaskHandler_06a3d6  @ $06A3D6  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a3d6, "ax", @progbits
        .global TaskHandler_06a3d6
TaskHandler_06a3d6:
        clr.w   0x28(a6)                        | +000
        bclr    #0x1,0x12(a6)                   | +004
        move.b  #0x6,0x72(a6)                   | +00a
        lea     0x2caf58.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L06a3f8(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06a3f8:
        jsr     TaskHandler_06b20e(pc)          | +022
        jsr     0x28d70.l                       | +026
        bcc.w   .L06a40c                        | +02c
        lea     TaskHandler_06a41c(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L06a40c:
        jsr     TaskHandler_06b1e2(pc)          | +036
        bcc.w   SetHandlerRts_06a41a            | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_06a41c  @ $06A41C  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a41c, "ax", @progbits
        .global TaskHandler_06a41c
TaskHandler_06a41c:
        move.w  #0x3c,0x70(a6)                  | +000
        move.l  #0xffffffff,0x48(a6)            | +006
        bclr    #0x1,0x12(a6)                   | +00e
        move.b  #0x6,0x72(a6)                   | +014
        lea     .L06a43c(pc),a1                 | +01a
        move.l  a1,(a6)                         | +01e
.L06a43c:
        subq.w  #0x1,0x70(a6)                   | +020
        cmpi.w  #0x0,0x70(a6)                   | +024
        bgt.w   SetHandlerRts_06a450            | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_06a476  @ $06A476  (96 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a476, "ax", @progbits
        .global TaskHandler_06a476
TaskHandler_06a476:
        move.b  #0x1,0x9c(a6)                   | +000
        move.b  #0x1,0x9e(a6)                   | +006
        bra.w   .L06a490                        | +00c
        clr.b   0x9c(a6)                        | +010
        move.b  #0x1,0x9e(a6)                   | +014
        .global TaskHandler_06a476__L06a490
TaskHandler_06a476__L06a490:
.L06a490:
        move.b  #0x1,0x9d(a6)                   | +01a
        jsr     TaskHandler_06b78a(pc)          | +020
        lea     0x2bd774.l,a0                   | +024
        jsr     0x799de.l                       | +02a
        move.w  d0,0x36(a6)                     | +030
        lea     0x2bd7f6.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        move.w  d0,0x70(a6)                     | +040
        lea     0x2bd6f2.l,a0                   | +044
        jsr     0x799de.l                       | +04a
        move.w  d0,0x66(a6)                     | +050
        jsr     TaskHandler_06b3a4(pc)          | +054
        move.l  #0x2ca2d8,0x4c(a6)              | +058

| ----------------------------------------------------------------------------
|  TaskHandler_06a4d6  @ $06A4D6  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a4d6, "ax", @progbits
        .global TaskHandler_06a4d6
TaskHandler_06a4d6:
        jsr     TaskHandler_06b2dc(pc)          | +000
        move.b  #0x0,0x72(a6)                   | +004
        move.w  0x36(a6),d0                     | +00a
        btst    #0x0,0x75(a6)                   | +00e
        bne.w   .L06a4f0                        | +014
        neg.w   d0                              | +018
.L06a4f0:
        move.w  d0,0x28(a6)                     | +01a
        jsr     TaskHandler_06b368(pc)          | +01e
        bcs.w   .L06a50c                        | +022
        lea     0x2ca690.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        bra.w   .L06a518                        | +032
.L06a50c:
        lea     0x2ca74a.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
.L06a518:
        lea     .L06a51e(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L06a51e:
        jsr     TaskHandler_06b232(pc)          | +048
        bcc.w   .L06a532                        | +04c
        eori.b  #0x1,0x75(a6)                   | +050
        lea     TaskHandler_06a568(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L06a532:
        jsr     0x28d70.l                       | +05c
        jsr     TaskHandler_06b27a(pc)          | +062
        bcc.w   .L06a546                        | +066
        lea     TaskHandler_06a568(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
.L06a546:
        subq.w  #0x1,0x70(a6)                   | +070
        jsr     TaskHandler_06b4f2(pc)          | +074
        bcc.w   .L06a558                        | +078
        lea     TaskHandler_06a5cc(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
.L06a558:
        jsr     0x283ca.l                       | +082
        jsr     0x283d8.l                       | +088
        bra.w   TaskHandler_06a6e0__L06a772     | +08e

| ----------------------------------------------------------------------------
|  TaskHandler_06a568  @ $06A568  (100 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a568, "ax", @progbits
        .global TaskHandler_06a568
TaskHandler_06a568:
        jsr     TaskHandler_06b38a(pc)          | +000
        move.b  #0x0,0x72(a6)                   | +004
        lea     0x2bd9fe.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x74(a6)                     | +016
        clr.w   0x28(a6)                        | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a598(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a598:
        jsr     TaskHandler_06b232(pc)          | +030
        jsr     0x28d70.l                       | +034
        subq.b  #0x1,0x74(a6)                   | +03a
        cmpi.b  #0x0,0x74(a6)                   | +03e
        bgt.w   .L06a5b6                        | +044
        lea     TaskHandler_06a4d6(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06a5b6:
        subq.w  #0x1,0x70(a6)                   | +04e
        jsr     TaskHandler_06b4f2(pc)          | +052
        bcc.w   .L06a5c8                        | +056
        lea     TaskHandler_06a5cc(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L06a5c8:
        bra.w   TaskHandler_06a6e0__L06a772     | +060

| ----------------------------------------------------------------------------
|  TaskHandler_06a5cc  @ $06A5CC  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a5cc, "ax", @progbits
        .global TaskHandler_06a5cc
TaskHandler_06a5cc:
        tst.b   0x9e(a6)                        | +000
        beq.w   TaskHandler_06a6c2              | +004
        jsr     0x267e2.l                       | +008
        lea     0x2bd8fa.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x70(a6)                     | +01a
        lea     0x2ca84a.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06a5fc(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06a5fc:
        jsr     TaskHandler_06b232(pc)          | +030
        jsr     0x28d70.l                       | +034
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L06a61a                        | +044
        lea     TaskHandler_06a6c2(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06a61a:
        bra.w   TaskHandler_06a6e0__L06a772     | +04e
        clr.b   0x73(a6)                        | +052
        lea     0x2bd97c.l,a0                   | +056
        jsr     0x799de.l                       | +05c
        move.b  d0,0x74(a6)                     | +062
        bra.w   TaskHandler_06a636__L06a644     | +066

| ----------------------------------------------------------------------------
|  TaskHandler_06a636  @ $06A636  (140 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a636, "ax", @progbits
        .global TaskHandler_06a636
TaskHandler_06a636:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   TaskHandler_06a4d6              | +00a
        .global TaskHandler_06a636__L06a644
TaskHandler_06a636__L06a644:
.L06a644:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd878.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cad10.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a670(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a670:
        subq.w  #0x1,0x70(a6)                   | +03a
        cmpi.w  #0x0,0x70(a6)                   | +03e
        bgt.w   .L06a684                        | +044
        lea     TaskHandler_06a636(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06a684:
        jsr     TaskHandler_06b232(pc)          | +04e
        bcc.w   .L06a698                        | +052
        eori.b  #0x1,0x75(a6)                   | +056
        lea     TaskHandler_06a568(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
.L06a698:
        jsr     0x28d70.l                       | +062
        tst.b   0x73(a6)                        | +068
        beq.w   .L06a6b0                        | +06c
        clr.b   0x73(a6)                        | +070
        move.b  #0x0,0x72(a6)                   | +074
.L06a6b0:
        jsr     TaskHandler_06b27a(pc)          | +07a
        bcc.w   .L06a6be                        | +07e
        lea     TaskHandler_06a568(pc),a1       | +082
        move.l  a1,(a6)                         | +086
.L06a6be:
        bra.w   TaskHandler_06a6e0__L06a772     | +088

| ----------------------------------------------------------------------------
|  TaskHandler_06a6c2  @ $06A6C2  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a6c2, "ax", @progbits
        .global TaskHandler_06a6c2
TaskHandler_06a6c2:
        jsr     0x267e2.l                       | +000
        clr.b   0x73(a6)                        | +006
        lea     0x2bd97c.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.b  d0,0x74(a6)                     | +016
        bra.w   TaskHandler_06a6e0__L06a6ee     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_06a6e0  @ $06A6E0  (212 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a6e0, "ax", @progbits
        .global TaskHandler_06a6e0
TaskHandler_06a6e0:
        subq.b  #0x1,0x74(a6)                   | +000
        cmpi.b  #0x0,0x74(a6)                   | +004
        ble.w   .L06a752                        | +00a
        .global TaskHandler_06a6e0__L06a6ee
TaskHandler_06a6e0__L06a6ee:
.L06a6ee:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x2,0x72(a6)                   | +012
        lea     0x2bd878.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        lea     0x2cacc0.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L06a71a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06a71a:
        jsr     TaskHandler_06b232(pc)          | +03a
        jsr     0x28d70.l                       | +03e
        bcc.w   .L06a73a                        | +044
        tst.b   0x73(a6)                        | +048
        beq.w   .L06a73a                        | +04c
        clr.b   0x73(a6)                        | +050
        move.b  #0x0,0x72(a6)                   | +054
.L06a73a:
        subq.w  #0x1,0x70(a6)                   | +05a
        cmpi.w  #0x0,0x70(a6)                   | +05e
        bgt.w   .L06a74e                        | +064
        lea     TaskHandler_06a6e0(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L06a74e:
        bra.w   .L06a772                        | +06e
.L06a752:
        lea     0x2bd7f6.l,a0                   | +072
        jsr     0x799de.l                       | +078
        move.w  d0,0x70(a6)                     | +07e
        move.l  #0xffffffff,0x90(a6)            | +082
        jsr     TaskHandler_06b3a4(pc)          | +08a
        bra.w   TaskHandler_06a568              | +08e
        .global TaskHandler_06a6e0__L06a772
TaskHandler_06a6e0__L06a772:
.L06a772:
        jsr     TaskHandler_06b3a4(pc)          | +092
        clr.b   0x78(a6)                        | +096
        jsr     0x2870a.l                       | +09a
        bcc.w   .L06a79c                        | +0a0
        lea     0x5e766.l,a0                    | +0a4
        jsr     0x5e770.l                       | +0aa
        bclr    #0x3,0x13(a6)                   | +0b0
        move.b  #0x1,0x78(a6)                   | +0b6
.L06a79c:
        jsr     0x28758.l                       | +0bc
        bcc.w   .L06a7ac                        | +0c2
        lea     TaskHandler_06a3d6(pc),a1       | +0c6
        move.l  a1,(a6)                         | +0ca
.L06a7ac:
        jsr     TaskHandler_06b1e2(pc)          | +0cc
        bcc.w   SetHandlerRts_06a7ba            | +0d0

| ----------------------------------------------------------------------------
|  TaskHandler_06a7bc  @ $06A7BC  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a7bc, "ax", @progbits
        .global TaskHandler_06a7bc
TaskHandler_06a7bc:
        move.b  #0x1,0x9c(a6)                   | +000
        clr.b   0x9e(a6)                        | +006
        bra.w   TaskHandler_06a476__L06a490     | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06a7ca  @ $06A7CA  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a7ca, "ax", @progbits
        .global TaskHandler_06a7ca
TaskHandler_06a7ca:
        clr.b   0x9c(a6)                        | +000
        clr.b   0x9e(a6)                        | +004
        bra.w   TaskHandler_06a476__L06a490     | +008

| ----------------------------------------------------------------------------
|  TaskHandler_06a7d6  @ $06A7D6  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a7d6, "ax", @progbits
        .global TaskHandler_06a7d6
TaskHandler_06a7d6:
        clr.b   0x7a(a6)                        | +000
        bra.w   .L06a7e4                        | +004
        move.b  #0x1,0x7a(a6)                   | +008
.L06a7e4:
        jsr     TaskHandler_06b810(pc)          | +00e
        clr.b   0x74(a6)                        | +012
        lea     0x2bd4ea.l,a0                   | +016
        jsr     0x799de.l                       | +01c
        move.w  d0,0x70(a6)                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06a7fc  @ $06A7FC  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a7fc, "ax", @progbits
        .global TaskHandler_06a7fc
TaskHandler_06a7fc:
        jsr     0x5e0d4.l                       | +000
        move.l  a0,0x90(a6)                     | +006
        move.b  #0x4,0x72(a6)                   | +00a
        clr.b   0x73(a6)                        | +010
        lea     0x2ca804.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L06a822(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06a822:
        jsr     TaskHandler_06b87a(pc)          | +026
        jsr     0x28d70.l                       | +02a
        jsr     TaskHandler_06b8bc(pc)          | +030
        bcc.w   .L06a864                        | +034
        tst.b   0x74(a6)                        | +038
        beq.w   .L06a842                        | +03c
        lea     TaskHandler_06a878(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L06a842:
        move.b  #0x1,0x72(a6)                   | +046
        subq.w  #0x1,0x70(a6)                   | +04c
        tst.b   0x73(a6)                        | +050
        beq.w   .L06a864                        | +054
        cmpi.w  #0x0,0x70(a6)                   | +058
        bgt.w   .L06a864                        | +05e
        lea     TaskHandler_06a868(pc),a1       | +062
        move.l  a1,(a6)                         | +066
.L06a864:
        bra.w   TaskHandler_06a96c              | +068

| ----------------------------------------------------------------------------
|  TaskHandler_06a868  @ $06A868  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a868, "ax", @progbits
        .global TaskHandler_06a868
TaskHandler_06a868:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06a878  @ $06A878  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a878, "ax", @progbits
        .global TaskHandler_06a878
TaskHandler_06a878:
        lea     0x2bd56c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        clr.b   0x73(a6)                        | +010
        move.b  #0x2,0x72(a6)                   | +014
        lea     0x2cacc0.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06a8a4(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06a8a4:
        jsr     TaskHandler_06b87a(pc)          | +02c
        jsr     0x28d70.l                       | +030
        bcc.w   .L06a8be                        | +036
        lea     0x2ca804.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
.L06a8be:
        subq.w  #0x1,0x70(a6)                   | +046
        cmpi.w  #0x0,0x70(a6)                   | +04a
        bgt.w   .L06a904                        | +050
        tst.b   0x73(a6)                        | +054
        beq.w   .L06a904                        | +058
        move.b  #0x4,0x72(a6)                   | +05c
        lea     TaskHandler_06a878(pc),a1       | +062
        move.l  a1,(a6)                         | +066
        subq.b  #0x1,0x74(a6)                   | +068
        cmpi.b  #0x0,0x74(a6)                   | +06c
        bgt.w   .L06a904                        | +072
        lea     0x2bd4ea.l,a0                   | +076
        jsr     0x799de.l                       | +07c
        move.w  d0,0x70(a6)                     | +082
        lea     TaskHandler_06a7fc(pc),a1       | +086
        move.l  a1,(a6)                         | +08a
.L06a904:
        bra.w   TaskHandler_06a96c              | +08c

| ----------------------------------------------------------------------------
|  TaskHandler_06a908  @ $06A908  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a908, "ax", @progbits
        .global TaskHandler_06a908
TaskHandler_06a908:
        move.b  #0x5,0x72(a6)                   | +000
        lea     0x2cae00.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06a920(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06a920:
        jsr     TaskHandler_06b87a(pc)          | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L06a93a                        | +022
        bclr    #0x3,0x13(a6)                   | +026
        lea     TaskHandler_06a7fc(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L06a93a:
        bra.w   TaskHandler_06a96c__L06a9e2     | +032

| ----------------------------------------------------------------------------
|  TaskHandler_06a93e  @ $06A93E  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a93e, "ax", @progbits
        .global TaskHandler_06a93e
TaskHandler_06a93e:
        move.b  #0x6,0x72(a6)                   | +000
        lea     0x2caf58.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06a956(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06a956:
        jsr     TaskHandler_06b87a(pc)          | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   SetHandlerRts_06a96a            | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06a96c  @ $06A96C  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06a96c, "ax", @progbits
        .global TaskHandler_06a96c
TaskHandler_06a96c:
        clr.b   0x78(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        cmpi.b  #0xff,0x20(a0)                  | +008
        bne.w   .L06a9a0                        | +00e
        btst    #0x0,0x13(a6)                   | +012
        bne.w   .L06a9a0                        | +018
        addi.w  #0x20,0x24(a6)                  | +01c
        lea     0x6ba34.l,a1                    | +022
        move.l  a1,(a6)                         | +028
        move.b  #0x7,0x72(a6)                   | +02a
        bra.w   .L06a9f2                        | +030
.L06a9a0:
        lea     0x5e766.l,a0                    | +034
        jsr     0x5e770.l                       | +03a
        jsr     0x2870a.l                       | +040
        bcc.w   .L06a9e2                        | +046
        bclr    #0x3,0x13(a6)                   | +04a
        cmpi.b  #0x2,0x58(a6)                   | +050
        beq.w   .L06a9d0                        | +056
        cmpi.b  #0x3,0x58(a6)                   | +05a
        bne.w   .L06a9e2                        | +060
.L06a9d0:
        bset    #0x3,0x13(a6)                   | +064
        lea     TaskHandler_06a908(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
        move.b  #0x1,0x78(a6)                   | +070
        .global TaskHandler_06a96c__L06a9e2
TaskHandler_06a96c__L06a9e2:
.L06a9e2:
        jsr     0x28758.l                       | +076
        bcc.w   .L06a9f2                        | +07c
        lea     TaskHandler_06a93e(pc),a1       | +080
        move.l  a1,(a6)                         | +084
.L06a9f2:
        jsr     0x5e45a.l                       | +086
        bcc.w   SetHandlerRts_06aa02            | +08c

| ----------------------------------------------------------------------------
|  TaskHandler_06aa04  @ $06AA04  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aa04, "ax", @progbits
        .global TaskHandler_06aa04
TaskHandler_06aa04:
        clr.b   0x9e(a6)                        | +000
        clr.b   0x7a(a6)                        | +004
        jsr     TaskHandler_06b810(pc)          | +008
        bra.w   .L06aa28                        | +00c
        clr.b   0x9e(a6)                        | +010
        clr.b   0x7a(a6)                        | +014
        jsr     TaskHandler_06b810(pc)          | +018
        move.l  #0xffffffff,0x48(a6)            | +01c
.L06aa28:
        move.b  #0x1,0x98(a0)                   | +024
        clr.b   0x9f(a0)                        | +02a
        move.b  0x98(a6),d0                     | +02e
        andi.w  #0xff,d0                        | +032
        lsl.w   #0x4,d0                         | +036
        move.w  #0x110,d0                       | +038
        move.w  d0,0x98(a6)                     | +03c
        clr.b   0x75(a6)                        | +040
        clr.b   0x74(a6)                        | +044
        lea     0x2bd4ea.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x70(a6)                     | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06aa5c  @ $06AA5C  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aa5c, "ax", @progbits
        .global TaskHandler_06aa5c
TaskHandler_06aa5c:
        jsr     0x5e0d4.l                       | +000
        move.l  a0,0x90(a6)                     | +006
        move.b  #0x4,0x72(a6)                   | +00a
        clr.b   0x73(a6)                        | +010
        lea     0x2ca832.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L06aa82(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L06aa82:
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        tst.b   0x74(a6)                        | +032
        bne.w   .L06aa9c                        | +036
        lea     TaskHandler_06aada(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
.L06aa9c:
        jsr     TaskHandler_06b8bc(pc)          | +040
        bcc.w   .L06aac6                        | +044
        move.b  #0x1,0x72(a6)                   | +048
        subq.w  #0x1,0x70(a6)                   | +04e
        tst.b   0x73(a6)                        | +052
        beq.w   .L06aac6                        | +056
        cmpi.w  #0x0,0x70(a6)                   | +05a
        bgt.w   .L06aac6                        | +060
        lea     TaskHandler_06aaca(pc),a1       | +064
        move.l  a1,(a6)                         | +068
.L06aac6:
        bra.w   TaskHandler_06abe2              | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_06aaca  @ $06AACA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aaca, "ax", @progbits
        .global TaskHandler_06aaca
TaskHandler_06aaca:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x74(a6)                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06aada  @ $06AADA  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aada, "ax", @progbits
        .global TaskHandler_06aada
TaskHandler_06aada:
        lea     0x2bd56c.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x70(a6)                     | +00c
        clr.b   0x73(a6)                        | +010
        move.b  #0x2,0x72(a6)                   | +014
        lea     0x2cacc0.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06ab06(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06ab06:
        jsr     0x2783a.l                       | +02c
        jsr     0x28d70.l                       | +032
        bcc.w   .L06ab22                        | +038
        lea     0x2ca832.l,a0                   | +03c
        jsr     0x28cd4.l                       | +042
.L06ab22:
        subq.w  #0x1,0x70(a6)                   | +048
        cmpi.w  #0x0,0x70(a6)                   | +04c
        bgt.w   .L06ab68                        | +052
        tst.b   0x73(a6)                        | +056
        beq.w   .L06ab68                        | +05a
        move.b  #0x4,0x72(a6)                   | +05e
        lea     TaskHandler_06aada(pc),a1       | +064
        move.l  a1,(a6)                         | +068
        subq.b  #0x1,0x74(a6)                   | +06a
        cmpi.b  #0x0,0x74(a6)                   | +06e
        bgt.w   .L06ab68                        | +074
        lea     0x2bd4ea.l,a0                   | +078
        jsr     0x799de.l                       | +07e
        move.w  d0,0x70(a6)                     | +084
        lea     TaskHandler_06aa5c(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L06ab68:
        bra.w   TaskHandler_06abe2              | +08e

| ----------------------------------------------------------------------------
|  TaskHandler_06ab6c  @ $06AB6C  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ab6c, "ax", @progbits
        .global TaskHandler_06ab6c
TaskHandler_06ab6c:
        move.b  #0x5,0x72(a6)                   | +000
        lea     0x2cae00.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06ab84(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06ab84:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06aba0                        | +024
        bclr    #0x3,0x13(a6)                   | +028
        lea     TaskHandler_06aa5c(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L06aba0:
        bra.w   TaskHandler_06abe2__L06ac28     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06aba4  @ $06ABA4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aba4, "ax", @progbits
        .global TaskHandler_06aba4
TaskHandler_06aba4:
        move.b  #0x6,0x72(a6)                   | +000
        lea     0x2caf58.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L06abbc(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L06abbc:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06abd2                        | +024
        lea     TaskHandler_06a41c(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L06abd2:
        jsr     TaskHandler_06b1e2(pc)          | +02e
        bcc.w   SetHandlerRts_06abe0            | +032

| ----------------------------------------------------------------------------
|  TaskHandler_06abe2  @ $06ABE2  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06abe2, "ax", @progbits
        .global TaskHandler_06abe2
TaskHandler_06abe2:
        clr.b   0x78(a6)                        | +000
        lea     0x5e766.l,a0                    | +004
        jsr     0x5e770.l                       | +00a
        jsr     0x2870a.l                       | +010
        bcc.w   .L06ac28                        | +016
        bclr    #0x3,0x13(a6)                   | +01a
        cmpi.b  #0x2,0x58(a6)                   | +020
        beq.w   .L06ac16                        | +026
        cmpi.b  #0x3,0x58(a6)                   | +02a
        bne.w   .L06ac28                        | +030
.L06ac16:
        bset    #0x3,0x13(a6)                   | +034
        lea     TaskHandler_06ab6c(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
        move.b  #0x1,0x78(a6)                   | +040
        .global TaskHandler_06abe2__L06ac28
TaskHandler_06abe2__L06ac28:
.L06ac28:
        jsr     0x28758.l                       | +046
        bcc.w   .L06ac38                        | +04c
        lea     TaskHandler_06aba4(pc),a1       | +050
        move.l  a1,(a6)                         | +054
.L06ac38:
        movea.l 0xc(a6),a0                      | +056
        cmpi.b  #0xff,0x20(a0)                  | +05a
        bne.w   .L06ac4c                        | +060
        lea     TaskHandler_06aba4(pc),a1       | +064
        move.l  a1,(a6)                         | +068
.L06ac4c:
        jsr     TaskHandler_06b1e2(pc)          | +06a
        bcc.w   SetHandlerRts_06ac5a            | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_06ac5c  @ $06AC5C  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ac5c, "ax", @progbits
        .global TaskHandler_06ac5c
TaskHandler_06ac5c:
        clr.b   0x20(a6)                        | +000
        lea     TaskHandler_06aa04(pc),a1       | +004
        jsr     0x4ae.l                         | +008
        jsr     0x5dd02.l                       | +00e
        move.b  #0x12,0x98(a0)                  | +014
        move.l  a0,0x5c(a6)                     | +01a
        lea     .L06ac80(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L06ac80:
        jsr     0x2783a.l                       | +024
        movea.l 0x5c(a6),a0                     | +02a
        btst    #0x0,0x13(a0)                   | +02e
        beq.w   .L06ac9a                        | +034
        lea     JmpToScheduler_06acaa(pc),a1    | +038
        move.l  a1,(a6)                         | +03c
.L06ac9a:
        jsr     TaskHandler_06b1e2(pc)          | +03e
        bcc.w   SetHandlerRts_06aca8            | +042

| ----------------------------------------------------------------------------
|  TaskHandler_06acb2  @ $06ACB2  (614 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06acb2, "ax", @progbits
        .global TaskHandler_06acb2
TaskHandler_06acb2:
        jsr     TaskHandler_06b94c(pc)          | +000
        andi.b  #0x3,0x98(a6)                   | +004
        move.b  #0x0,0x72(a6)                   | +00a
        tst.b   0x9d(a6)                        | +010
        bne.w   .L06aebc                        | +014
        move.w  #0x8,0x34(a6)                   | +018
        lea     0x2cb082.l,a0                   | +01e
        jsr     0x28cd4.l                       | +024
        lea     .L06ace2(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L06ace2:
        cmpi.b  #0x6,0x72(a6)                   | +030
        beq.w   TaskHandler_06af68              | +036
        jsr     TaskHandler_06b8ea(pc)          | +03a
        bcc.w   .L06acf6                        | +03e
        jmp     (a1)                            | +042
.L06acf6:
        jsr     TaskHandler_06b550(pc)          | +044
        jsr     0x28d70.l                       | +048
        bcc.w   .L06ad10                        | +04e
        lea     0x2cb082.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
.L06ad10:
        bra.w   TaskHandler_06af4e              | +05e
        clr.b   0x74(a6)                        | +062
        movea.l 0xc(a6),a0                      | +066
        move.l  0x90(a0),0x90(a6)               | +06a
        lea     0x2cb082.l,a0                   | +070
        jsr     0x28cd4.l                       | +076
        lea     .L06ad34(pc),a1                 | +07c
        move.l  a1,(a6)                         | +080
.L06ad34:
        addq.b  #0x1,0x74(a6)                   | +082
        andi.b  #0x3,0x74(a6)                   | +086
        bne.w   .L06ad68                        | +08c
        jsr     TaskHandler_06b5a4(pc)          | +090
        movea.l 0xc(a6),a0                      | +094
        clr.b   0x73(a0)                        | +098
        jsr     TaskHandler_06b5ea(pc)          | +09c
        bcc.w   .L06ad68                        | +0a0
        jsr     TaskHandler_06b6cc(pc)          | +0a4
        bcs.w   .L06ad68                        | +0a8
        movea.l 0xc(a6),a0                      | +0ac
        move.b  #0x1,0x73(a0)                   | +0b0
.L06ad68:
        cmpi.b  #0x6,0x72(a6)                   | +0b6
        beq.w   TaskHandler_06af68              | +0bc
        jsr     TaskHandler_06b8ea(pc)          | +0c0
        bcc.w   .L06ad7c                        | +0c4
        jmp     (a1)                            | +0c8
.L06ad7c:
        jsr     TaskHandler_06b550(pc)          | +0ca
        jsr     0x28d70.l                       | +0ce
        bcc.w   .L06ad96                        | +0d4
        lea     0x2cb082.l,a0                   | +0d8
        jsr     0x28cd4.l                       | +0de
.L06ad96:
        bra.w   TaskHandler_06af4e              | +0e4
        lea     0x2cb1ac.l,a0                   | +0e8
        move.w  0x34(a6),d0                     | +0ee
        andi.w  #0xf,d0                         | +0f2
        lsl.w   #0x2,d0                         | +0f6
        movea.l (a0,d0.w),a0                    | +0f8
        cmpa.l  #0xffffffff,a0                  | +0fc
        beq.w   .L06adbe                        | +102
        jsr     0x28cd4.l                       | +106
.L06adbe:
        jsr     TaskHandler_06b6f2(pc)          | +10c
        lea     .L06adc8(pc),a1                 | +110
        move.l  a1,(a6)                         | +114
.L06adc8:
        cmpi.b  #0x6,0x72(a6)                   | +116
        beq.w   TaskHandler_06af68              | +11c
        jsr     TaskHandler_06b8ea(pc)          | +120
        bcc.w   .L06addc                        | +124
        jmp     (a1)                            | +128
.L06addc:
        jsr     TaskHandler_06b550(pc)          | +12a
        jsr     0x28d70.l                       | +12e
        bcc.w   .L06adf4                        | +134
        movea.l 0xc(a6),a0                      | +138
        move.b  #0x1,0x73(a0)                   | +13c
.L06adf4:
        rts                                     | +142
        clr.b   0x74(a6)                        | +144
        lea     0x2cb082.l,a0                   | +148
        jsr     0x28cd4.l                       | +14e
        lea     .L06ae0c(pc),a1                 | +154
        move.l  a1,(a6)                         | +158
.L06ae0c:
        addq.b  #0x1,0x74(a6)                   | +15a
        andi.b  #0x3,0x74(a6)                   | +15e
        bne.w   .L06ae42                        | +164
        move.w  #0x1,d1                         | +168
        cmpi.w  #0x8,0x34(a6)                   | +16c
        beq.w   .L06ae38                        | +172
        blt.w   .L06ae30                        | +176
        move.w  #0xffff,d1                      | +17a
.L06ae30:
        add.w   d1,0x34(a6)                     | +17e
        bra.w   .L06ae42                        | +182
.L06ae38:
        movea.l 0xc(a6),a0                      | +186
        move.b  #0x1,0x73(a0)                   | +18a
.L06ae42:
        cmpi.b  #0x6,0x72(a6)                   | +190
        beq.w   TaskHandler_06af68              | +196
        jsr     TaskHandler_06b8ea(pc)          | +19a
        bcc.w   .L06ae56                        | +19e
        jmp     (a1)                            | +1a2
.L06ae56:
        jsr     TaskHandler_06b550(pc)          | +1a4
        jsr     0x28d70.l                       | +1a8
        bcc.w   .L06ae70                        | +1ae
        lea     0x2cb082.l,a0                   | +1b2
        jsr     0x28cd4.l                       | +1b8
.L06ae70:
        bra.w   TaskHandler_06af4e              | +1be
        clr.b   0x74(a6)                        | +1c2
        lea     0x2cb082.l,a0                   | +1c6
        jsr     0x28cd4.l                       | +1cc
        lea     .L06ae8a(pc),a1                 | +1d2
        move.l  a1,(a6)                         | +1d6
.L06ae8a:
        cmpi.b  #0x6,0x72(a6)                   | +1d8
        beq.w   TaskHandler_06af68              | +1de
        jsr     TaskHandler_06b8ea(pc)          | +1e2
        bcc.w   .L06ae9e                        | +1e6
        jmp     (a1)                            | +1ea
.L06ae9e:
        jsr     TaskHandler_06b550(pc)          | +1ec
        jsr     0x28d70.l                       | +1f0
        bcc.w   .L06aeb8                        | +1f6
        lea     0x2cb082.l,a0                   | +1fa
        jsr     0x28cd4.l                       | +200
.L06aeb8:
        bra.w   TaskHandler_06af4e              | +206
.L06aebc:
        clr.b   0x74(a6)                        | +20a
        move.w  #0xc,d0                         | +20e
        tst.b   0x9c(a6)                        | +212
        beq.w   .L06aed0                        | +216
        move.w  #0x4,d0                         | +21a
.L06aed0:
        move.w  d0,0x34(a6)                     | +21e
        lea     0x2cb082.l,a0                   | +222
        jsr     0x28cd4.l                       | +228
        lea     .L06aee6(pc),a1                 | +22e
        move.l  a1,(a6)                         | +232
.L06aee6:
        cmpi.b  #0x6,0x72(a6)                   | +234
        beq.w   TaskHandler_06af68              | +23a
        jsr     TaskHandler_06b8ea(pc)          | +23e
        bcc.w   .L06aefa                        | +242
        jmp     (a1)                            | +246
.L06aefa:
        jsr     TaskHandler_06b550(pc)          | +248
        jsr     0x28d70.l                       | +24c
        bcc.w   .L06af14                        | +252
        lea     0x2cb082.l,a0                   | +256
        jsr     0x28cd4.l                       | +25c
.L06af14:
        bra.w   TaskHandler_06af4e              | +262

| ----------------------------------------------------------------------------
|  TaskHandler_06af18  @ $06AF18  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06af18, "ax", @progbits
        .global TaskHandler_06af18
TaskHandler_06af18:
        lea     0x2cc1f0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06af2a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06af2a:
        cmpi.b  #0x6,0x72(a6)                   | +012
        beq.w   TaskHandler_06af68              | +018
        jsr     TaskHandler_06b8ea(pc)          | +01c
        bcc.w   .L06af3e                        | +020
        jmp     (a1)                            | +024
.L06af3e:
        jsr     TaskHandler_06b550(pc)          | +026
        subq.w  #0x1,0x38(a6)                   | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_06af4e  @ $06AF4E  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06af4e, "ax", @progbits
        .global TaskHandler_06af4e
TaskHandler_06af4e:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x78(a0)                        | +004
        beq.w   JsrAbsRts_06af66                | +008
        lea     0x2cb8ce.l,a0                   | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06af68  @ $06AF68  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06af68, "ax", @progbits
        .global TaskHandler_06af68
TaskHandler_06af68:
        move.w  #0xffde,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x663,0x2a(a6)                 | +00e
        move.w  #0xff93,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x4000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x0,0x38(a6)                   | +030
        lea     .L06afa4(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L06afa4:
        move.w  0x34(a6),0x5c(a6)               | +03c
        jsr     0x27bc8.l                       | +042
        bcc.w   .L06afba                        | +048
        lea     TaskHandler_06b014(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06afba:
        move.w  0x5c(a6),0x34(a6)               | +052
        jsr     0x28d70.l                       | +058
        cmpi.w  #0xc0,0x24(a6)                  | +05e
        bgt.w   .L06afd6                        | +064
        lea     JmpToScheduler_06a460(pc),a1    | +068
        move.l  a1,(a6)                         | +06c
.L06afd6:
        jsr     0x5e45a.l                       | +06e
        bcc.w   .L06afe6                        | +074
        lea     TaskHandler_06b014(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L06afe6:
        jsr     TaskHandler_06b1e2(pc)          | +07e
        bcc.w   SetHandlerRts_06aff4            | +082

| ----------------------------------------------------------------------------
|  TaskHandler_06aff6  @ $06AFF6  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06aff6, "ax", @progbits
        .global TaskHandler_06aff6
TaskHandler_06aff6:
        lea     .L06affc(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L06affc:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   SetHandlerRts_06b012            | +012

| ----------------------------------------------------------------------------
|  TaskHandler_06b014  @ $06B014  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b014, "ax", @progbits
        .global TaskHandler_06b014
TaskHandler_06b014:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06b038  @ $06B038  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b038, "ax", @progbits
        .global TaskHandler_06b038
TaskHandler_06b038:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x8,d1                         | +016
        jsr     0x236e.l                        | +01a
        move.w  0x34(a6),d0                     | +020
        andi.w  #0x7,d0                         | +024
        bra.w   .L06b082                        | +028
        move.w  0x34(a6),d0                     | +02c
        lsr.w   #0x3,d0                         | +030
        andi.w  #0x1,d0                         | +032
        eor.b   d0,0x3a(a6)                     | +036
        lea     0x2dd6d2.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        bra.w   .L06b0a6                        | +046
.L06b082:
        move.b  0x76(a6),d0                     | +04a
        move.b  0x77(a6),d1                     | +04e
        ext.w   d0                              | +052
        ext.w   d1                              | +054
        add.w   d0,0x22(a6)                     | +056
        add.w   d1,0x24(a6)                     | +05a
        subq.w  #0x8,0x24(a6)                   | +05e
        lea     0x2dd6d2.l,a0                   | +062
        jsr     0x28cd4.l                       | +068
.L06b0a6:
        bra.w   TaskHandler_06aff6              | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_06b0aa  @ $06B0AA  (290 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b0aa, "ax", @progbits
        .global TaskHandler_06b0aa
TaskHandler_06b0aa:
        lea     0x2ca4b0.l,a1                   | +000
        move.b  0x98(a6),d0                     | +006
        andi.w  #0x7,d0                         | +00a
        lsl.w   #0x2,d0                         | +00e
        movea.l (a1,d0.w),a1                    | +010
        move.w  0x34(a6),d0                     | +014
        andi.w  #0xf,d0                         | +018
        move.w  d0,d1                           | +01c
        add.w   d0,d0                           | +01e
        add.w   d1,d0                           | +020
        add.w   d0,d0                           | +022
        ext.l   d0                              | +024
        adda.l  d0,a1                           | +026
        move.w  (a1),0x28(a6)                   | +028
        move.w  0x2(a1),0x2e(a6)                | +02c
        move.w  0x4(a1),0x2a(a6)                | +032
        andi.b  #0xfe,0x3a(a6)                  | +038
        move.w  #0x173,d1                       | +03e
        jsr     0x236e.l                        | +042
        bset    #0x4,0x6b(a6)                   | +048
        lea     0x2cc686.l,a0                   | +04e
        jsr     0x28cd4.l                       | +054
        move.w  #0xd000,d0                      | +05a
        jsr     0x28134.l                       | +05e
        andi.w  #0xffe3,0x38(a6)                | +064
        ori.w   #0x1c,0x38(a6)                  | +06a
        clr.b   0x79(a6)                        | +070
        move.w  0x34(a6),d0                     | +074
        andi.w  #0xf,d0                         | +078
        cmpi.w  #0x0,d0                         | +07c
        beq.w   .L06b16c                        | +080
        cmpi.w  #0x8,d0                         | +084
        bge.w   .L06b16c                        | +088
        move.w  #0xd000,d0                      | +08c
        jsr     0x28134.l                       | +090
        andi.w  #0xffe3,0x38(a6)                | +096
        ori.w   #0x8,0x38(a6)                   | +09c
        move.b  #0x1,0x79(a6)                   | +0a2
        lea     .L06b158(pc),a1                 | +0a8
        move.l  a1,(a6)                         | +0ac
.L06b158:
        jsr     0x27bc8.l                       | +0ae
        bcc.w   .L06b168                        | +0b4
        lea     TaskHandler_06b1d4(pc),a1       | +0b8
        move.l  a1,(a6)                         | +0bc
.L06b168:
        bra.w   .L06b182                        | +0be
.L06b16c:
        lea     .L06b172(pc),a1                 | +0c2
        move.l  a1,(a6)                         | +0c6
.L06b172:
        jsr     0x27d50.l                       | +0c8
        bcc.w   .L06b182                        | +0ce
        lea     TaskHandler_06b1d4(pc),a1       | +0d2
        move.l  a1,(a6)                         | +0d6
.L06b182:
        move.w  0x28(a6),d0                     | +0d8
        move.w  0x2a(a6),d1                     | +0dc
        asr.w   #0x4,d0                         | +0e0
        asr.w   #0x4,d1                         | +0e2
        jsr     0x5e018.l                       | +0e4
        lsr.w   #0x3,d0                         | +0ea
        move.w  d0,0x34(a6)                     | +0ec
        jsr     0x28d70.l                       | +0f0
        jsr     0x283d8.l                       | +0f6
        btst    #0x1,0x13(a6)                   | +0fc
        beq.w   .L06b1b6                        | +102
        lea     TaskHandler_06b014(pc),a1       | +106
        move.l  a1,(a6)                         | +10a
.L06b1b6:
        movea.l #0xffffffff,a0                  | +10c
        lea     0x2ca3d8.l,a0                   | +112
        jsr     0x5dd56.l                       | +118
        bcc.w   SetHandlerRts_06b1d2            | +11e

| ----------------------------------------------------------------------------
|  TaskHandler_06b1d4  @ $06B1D4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b1d4, "ax", @progbits
        .global TaskHandler_06b1d4
TaskHandler_06b1d4:
        move.w  #0x1025,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   TaskHandler_06b014              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06b1e2  @ $06B1E2  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b1e2, "ax", @progbits
        .global TaskHandler_06b1e2
TaskHandler_06b1e2:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2ca3d0.l,a0                   | +006
        jsr     0x5dd5c.l                       | +00c
        bcs.w   SetXN_06b208                    | +012
        cmpi.w  #0xff80,0x22(a6)                | +016
        blt.w   SetXN_06b208                    | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_06b20e  @ $06B20E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b20e, "ax", @progbits
        .global TaskHandler_06b20e
TaskHandler_06b20e:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   .L06b230                        | +00c
        jsr     0x27c8c.l                       | +010
        bcc.w   .L06b230                        | +016
        clr.w   0x2a(a6)                        | +01a
        clr.w   0x2e(a6)                        | +01e
.L06b230:
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06b232  @ $06B232  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b232, "ax", @progbits
        .global TaskHandler_06b232
TaskHandler_06b232:
        btst    #0x6,0x13(a6)                   | +000
        beq.w   TaskHandler_06b242              | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06b242  @ $06B242  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b242, "ax", @progbits
        .global TaskHandler_06b242
TaskHandler_06b242:
        jsr     0x28998.l                       | +000
        jsr     0x2783a.l                       | +006
        jsr     0x27eba.l                       | +00c
        bcc.w   JsrAbsThunk_06b272              | +012
        jsr     0x27c8c.l                       | +016
        bcc.w   JsrAbsRts_06b278                | +01c
        clr.w   0x2a(a6)                        | +020
        clr.w   0x2e(a6)                        | +024
        andi.b  #0xee,ccr                       | +028
        bra.w   JsrAbsRts_06b278                | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_06b27a  @ $06B27A  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b27a, "ax", @progbits
        .global TaskHandler_06b27a
TaskHandler_06b27a:
        jsr     TaskHandler_06b336(pc)          | +000
        bcc.w   .L06b28c                        | +004
        eori.b  #0x1,0x75(a6)                   | +008
        bra.w   SetXN_06b2d6                    | +00e
.L06b28c:
        cmpi.w  #0x30,0x22(a6)                  | +012
        bgt.w   .L06b2aa                        | +018
        btst    #0x0,0x75(a6)                   | +01c
        bne.w   .L06b2aa                        | +022
        ori.b   #0x1,0x75(a6)                   | +026
        bra.w   SetXN_06b2d6                    | +02c
.L06b2aa:
        cmpi.w  #0x110,0x22(a6)                 | +030
        blt.w   .L06b2c8                        | +036
        btst    #0x0,0x75(a6)                   | +03a
        beq.w   .L06b2c8                        | +040
        andi.b  #0xfe,0x75(a6)                  | +044
        bra.w   SetXN_06b2d6                    | +04a
.L06b2c8:
        jsr     TaskHandler_06b2dc(pc)          | +04e
        bcs.w   SetXN_06b2d6                    | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06b2dc  @ $06B2DC  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b2dc, "ax", @progbits
        .global TaskHandler_06b2dc
TaskHandler_06b2dc:
        tst.b   0x9e(a6)                        | +000
        beq.w   ClearXN_06b32a                  | +004
        move.l  0x90(a6),d0                     | +008
        cmpi.l  #0xffffffff,d0                  | +00c
        beq.w   ClearXN_06b32a                  | +012
        movea.l d0,a0                           | +016
        move.w  0x22(a0),d0                     | +018
        btst    #0x0,0x75(a6)                   | +01c
        bne.w   .L06b316                        | +022
        addq.w  #0x8,d0                         | +026
        cmp.w   0x22(a6),d0                     | +028
        blt.w   ClearXN_06b32a                  | +02c
        eori.b  #0x1,0x75(a6)                   | +030
        bra.w   SetXN_06b330                    | +036
.L06b316:
        subq.w  #0x8,d0                         | +03a
        cmp.w   0x22(a6),d0                     | +03c
        bgt.w   ClearXN_06b32a                  | +040
        eori.b  #0x1,0x75(a6)                   | +044
        bra.w   SetXN_06b330                    | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_06b336  @ $06B336  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b336, "ax", @progbits
        .global TaskHandler_06b336
TaskHandler_06b336:
        cmpi.w  #0x0,0x28(a6)                   | +000
        beq.w   ClearXN_06b35c                  | +006
        bgt.w   .L06b352                        | +00a
        btst    #0x1,0x69(a6)                   | +00e
        bne.w   SetXN_06b362                    | +014
        bra.w   ClearXN_06b35c                  | +018
.L06b352:
        btst    #0x0,0x69(a6)                   | +01c
        bne.w   SetXN_06b362                    | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06b368  @ $06B368  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b368, "ax", @progbits
        .global TaskHandler_06b368
TaskHandler_06b368:
        move.b  0x75(a6),d0                     | +000
        move.b  0x3a(a6),d1                     | +004
        andi.b  #0x1,d0                         | +008
        andi.b  #0x1,d1                         | +00c
        cmp.b   d0,d1                           | +010
        bne.w   SetXN_06b384                    | +012

| ----------------------------------------------------------------------------
|  TaskHandler_06b38a  @ $06B38A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b38a, "ax", @progbits
        .global TaskHandler_06b38a
TaskHandler_06b38a:
        addq.b  #0x1,0x7b(a6)                   | +000
        andi.b  #0x3,0x7b(a6)                   | +004
        bne.w   .L06b3a2                        | +00a
        jsr     0x5e0d4.l                       | +00e
        move.l  a0,0x90(a6)                     | +014
.L06b3a2:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_06b3a4  @ $06B3A4  (170 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b3a4, "ax", @progbits
        .global TaskHandler_06b3a4
TaskHandler_06b3a4:
        move.l  0x90(a6),d0                     | +000
        cmpi.l  #0xffffffff,d0                  | +004
        beq.w   .L06b3e0                        | +00a
        clr.w   d0                              | +00e
        jsr     0x5e3a2.l                       | +010
        bcc.w   .L06b3c8                        | +016
        move.l  a0,d0                           | +01a
        cmp.l   0x90(a6),d0                     | +01c
        beq.w   ClearXN_06b454                  | +020
.L06b3c8:
        move.w  #0x1,d0                         | +024
        jsr     0x5e3a2.l                       | +028
        bcc.w   .L06b3e0                        | +02e
        move.l  a0,d0                           | +032
        cmp.l   0x90(a6),d0                     | +034
        beq.w   ClearXN_06b454                  | +038
.L06b3e0:
        jsr     0x5e0d4.l                       | +03c
        move.l  a0,0x90(a6)                     | +042
        clr.w   d0                              | +046
        jsr     0x5e3a2.l                       | +048
        bcc.w   .L06b426                        | +04e
        jsr     TaskHandler_06b45a(pc)          | +052
        bcc.w   .L06b426                        | +056
        move.l  a0,0x90(a6)                     | +05a
        move.w  #0x1,d0                         | +05e
        jsr     0x5e3a2.l                       | +062
        bcc.w   SetXN_06b44e                    | +068
        jsr     TaskHandler_06b45a(pc)          | +06c
        bcc.w   SetXN_06b44e                    | +070
        jsr     0x5e0d4.l                       | +074
        move.l  a0,0x90(a6)                     | +07a
        bra.w   SetXN_06b44e                    | +07e
.L06b426:
        move.w  #0x1,d0                         | +082
        jsr     0x5e3a2.l                       | +086
        bcc.w   .L06b444                        | +08c
        jsr     TaskHandler_06b45a(pc)          | +090
        bcc.w   .L06b444                        | +094
        move.l  a0,0x90(a6)                     | +098
        bra.w   SetXN_06b44e                    | +09c
.L06b444:
        jsr     0x5e0d4.l                       | +0a0
        move.l  a0,0x90(a6)                     | +0a6

| ----------------------------------------------------------------------------
|  TaskHandler_06b45a  @ $06B45A  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b45a, "ax", @progbits
        .global TaskHandler_06b45a
TaskHandler_06b45a:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x22(a0),d0                     | +004
        cmpi.w  #0xffe0,d0                      | +008
        blt.w   ClearXN_06b478                  | +00c
        cmpi.w  #0x20,d0                        | +010
        bgt.w   ClearXN_06b478                  | +014

| ----------------------------------------------------------------------------
|  TaskHandler_06b47e  @ $06B47E  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b47e, "ax", @progbits
        .global TaskHandler_06b47e
TaskHandler_06b47e:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   ClearXN_06b4ec                  | +00a
        jsr     0x5e0d4.l                       | +00e
        move.w  0x24(a0),d0                     | +014
        sub.w   0x24(a6),d0                     | +018
        cmpi.w  #0x0,d0                         | +01c
        bgt.w   .L06b4a4                        | +020
        neg.w   d0                              | +024
.L06b4a4:
        cmpi.w  #0x80,d0                        | +026
        ble.w   .L06b4b0                        | +02a
        move.w  #0x80,d0                        | +02e
.L06b4b0:
        lsr.w   #0x5,d0                         | +032
        lea     0x2ca44c.l,a1                   | +034
        move.b  0x98(a6),d1                     | +03a
        andi.w  #0x3,d1                         | +03e
        lsl.w   #0x2,d1                         | +042
        movea.l (a1,d1.w),a2                    | +044
        move.b  (a2,d0.w),d1                    | +048
        andi.w  #0xff,d1                        | +04c
        move.w  d1,d2                           | +050
        neg.w   d1                              | +052
        move.w  0x22(a0),d0                     | +054
        sub.w   0x22(a6),d0                     | +058
        cmp.w   d1,d0                           | +05c
        blt.w   ClearXN_06b4ec                  | +05e
        cmp.w   d2,d0                           | +062
        bgt.w   ClearXN_06b4ec                  | +064

| ----------------------------------------------------------------------------
|  TaskHandler_06b4f2  @ $06B4F2  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b4f2, "ax", @progbits
        .global TaskHandler_06b4f2
TaskHandler_06b4f2:
        cmpi.w  #0x0,0x70(a6)                   | +000
        bgt.w   ClearXN_06b534                  | +006
        tst.b   0x9e(a6)                        | +00a
        beq.w   SetXN_06b52e                    | +00e
        move.l  0x90(a6),d0                     | +012
        cmpi.l  #0xffffffff,d0                  | +016
        beq.w   ClearXN_06b534                  | +01c
        movea.l d0,a0                           | +020
        move.w  0x22(a0),d0                     | +022
        move.w  d0,d1                           | +026
        subq.w  #0x8,d0                         | +028
        addq.w  #0x8,d1                         | +02a
        cmp.w   0x22(a6),d0                     | +02c
        bgt.w   ClearXN_06b534                  | +030
        cmp.w   0x22(a6),d1                     | +034
        blt.w   ClearXN_06b534                  | +038

| ----------------------------------------------------------------------------
|  TaskHandler_06b53a  @ $06B53A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b53a, "ax", @progbits
        .global TaskHandler_06b53a
TaskHandler_06b53a:
        cmpi.w  #0x0,0x70(a6)                   | +000
        bgt.w   ClearXN_06b54a                  | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06b550  @ $06B550  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b550, "ax", @progbits
        .global TaskHandler_06b550
TaskHandler_06b550:
        jsr     0x5e506.l                       | +000
        move.b  0x76(a0),d0                     | +006
        ext.w   d0                              | +00a
        btst    #0x0,0x3a(a6)                   | +00c
        beq.w   .L06b568                        | +012
        neg.w   d0                              | +016
.L06b568:
        add.w   d0,0x22(a6)                     | +018
        move.b  0x77(a0),d0                     | +01c
        ext.w   d0                              | +020
        add.w   d0,0x24(a6)                     | +022
        move.w  0x34(a6),d0                     | +026
        andi.w  #0xf,d0                         | +02a
        beq.w   .L06b58a                        | +02e
        cmpi.w  #0x8,d0                         | +032
        bcs.w   .L06b58e                        | +036
.L06b58a:
        subq.w  #0x1,0x38(a6)                   | +03a
.L06b58e:
        movea.l 0xc(a6),a0                      | +03e
        btst    #0x7,0x5a(a0)                   | +042
        beq.w   .L06b5a2                        | +048
        bset    #0x0,0x5a(a6)                   | +04c
.L06b5a2:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06b5a4  @ $06B5A4  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b5a4, "ax", @progbits
        .global TaskHandler_06b5a4
TaskHandler_06b5a4:
        tst.b   0x9f(a6)                        | +000
        beq.w   .L06b5e8                        | +004
        movea.l 0x90(a6),a0                     | +008
        move.w  0x22(a6),d0                     | +00c
        move.w  0x24(a6),d1                     | +010
        move.w  0x22(a0),d2                     | +014
        move.w  0x24(a0),d3                     | +018
        jsr     0x5e23a.l                       | +01c
        clr.b   d1                              | +022
        cmpi.w  #0x70,d0                        | +024
        blt.w   .L06b5e0                        | +028
        move.b  #0x1,d1                         | +02c
        cmpi.w  #0xb0,d0                        | +030
        blt.w   .L06b5e0                        | +034
        move.b  #0x2,d1                         | +038
.L06b5e0:
        andi.b  #0x3,d1                         | +03c
        move.b  d1,0x98(a6)                     | +040
.L06b5e8:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  TaskHandler_06b5ea  @ $06B5EA  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b5ea, "ax", @progbits
        .global TaskHandler_06b5ea
TaskHandler_06b5ea:
        movea.l 0x90(a6),a0                     | +000
        lea     0x2ca5f0.l,a1                   | +004
        move.w  0x24(a0),d0                     | +00a
        cmp.w   0x24(a6),d0                     | +00e
        bgt.w   .L06b606                        | +012
        lea     0x2ca612.l,a1                   | +016
.L06b606:
        move.w  0x22(a0),d0                     | +01c
        sub.w   0x22(a6),d0                     | +020
        cmpi.w  #0xff00,d0                      | +024
        bgt.w   .L06b61a                        | +028
        move.w  #0xff00,d0                      | +02c
.L06b61a:
        cmpi.w  #0x100,d0                       | +030
        blt.w   .L06b626                        | +034
        move.w  #0x100,d0                       | +038
.L06b626:
        addi.w  #0x100,d0                       | +03c
        lsr.w   #0x4,d0                         | +040
        move.b  (a1,d0.w),d0                    | +042
        btst    #0x0,0x3a(a6)                   | +046
        beq.w   .L06b644                        | +04c
        lea     0x2ca634.l,a1                   | +050
        move.b  (a1,d0.w),d0                    | +056
.L06b644:
        jsr     TaskHandler_06b678(pc)          | +05a
        andi.w  #0xf,d0                         | +05e
        cmp.w   0x34(a6),d0                     | +062
        beq.w   SetXN_06b672                    | +066
        sub.w   0x34(a6),d0                     | +06a
        move.w  #0x1,d1                         | +06e
        cmpi.w  #0x0,d0                         | +072
        bgt.w   .L06b668                        | +076
        move.w  #0xffff,d1                      | +07a
.L06b668:
        add.w   d1,0x34(a6)                     | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_06b678  @ $06B678  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b678, "ax", @progbits
        .global TaskHandler_06b678
TaskHandler_06b678:
        move.b  d0,0x5c(a6)                     | +000
        movea.l 0x90(a6),a0                     | +004
        move.w  0x24(a0),d0                     | +008
        sub.w   0x24(a6),d0                     | +00c
        cmpi.w  #0xfff8,d0                      | +010
        blt.w   TaskHandler_06b6c2              | +014
        cmpi.w  #0x28,d0                        | +018
        bgt.w   TaskHandler_06b6c2              | +01c
        move.w  0x22(a0),d0                     | +020
        sub.w   0x22(a6),d0                     | +024
        btst    #0x0,0x3a(a6)                   | +028
        beq.w   .L06b6ac                        | +02e
        neg.w   d0                              | +032
.L06b6ac:
        clr.b   d1                              | +034
        cmpi.w  #0x0,d0                         | +036
        bgt.w   .L06b6ba                        | +03a
        move.b  #0x8,d1                         | +03e
.L06b6ba:
        move.b  d1,d0                           | +042

| ----------------------------------------------------------------------------
|  TaskHandler_06b6c2  @ $06B6C2  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b6c2, "ax", @progbits
        .global TaskHandler_06b6c2
TaskHandler_06b6c2:
        move.b  0x5c(a6),d0                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06b6cc  @ $06B6CC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b6cc, "ax", @progbits
        .global TaskHandler_06b6cc
TaskHandler_06b6cc:
        tst.b   0x9e(a6)                        | +000
        beq.w   ClearXN_06b6ec                  | +004
        tst.w   0x34(a6)                        | +008
        beq.w   ClearXN_06b6ec                  | +00c
        cmpi.w  #0xd,0x34(a6)                   | +010
        bcc.w   ClearXN_06b6ec                  | +016

| ----------------------------------------------------------------------------
|  TaskHandler_06b6f2  @ $06B6F2  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b6f2, "ax", @progbits
        .global TaskHandler_06b6f2
TaskHandler_06b6f2:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x34(a6),d0                     | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        beq.w   .L06b714                        | +014
        lea     0x2ca634.l,a1                   | +018
        move.b  (a1,d0.w),d0                    | +01e
.L06b714:
        andi.w  #0xf,d0                         | +022
        move.w  d0,0x5c(a6)                     | +026
        lea     TaskHandler_06b038(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     0x2ca470.l,a1                   | +03a
        move.w  0x34(a6),d0                     | +040
        move.w  d0,0x34(a0)                     | +044
        lsl.w   #0x2,d0                         | +048
        move.w  (a1,d0.w),d1                    | +04a
        move.w  0x2(a1,d0.w),d2                 | +04e
        btst    #0x0,0x3a(a6)                   | +052
        beq.w   .L06b750                        | +058
        neg.w   d1                              | +05c
.L06b750:
        move.w  d1,d3                           | +05e
        move.w  d2,d4                           | +060
        move.b  d3,0x76(a0)                     | +062
        move.b  d4,0x77(a0)                     | +066
        movem.w d1-d2,-(a7)                     | +06a
        lea     TaskHandler_06b0aa(pc),a1       | +06e
        jsr     0x4ae.l                         | +072
        jsr     0x5dd02.l                       | +078
        movem.w (a7)+,d1-d2                     | +07e
        move.w  0x5c(a6),0x34(a0)               | +082
        add.w   d1,0x22(a0)                     | +088
        add.w   d2,0x24(a0)                     | +08c
        move.b  0x98(a6),0x98(a0)               | +090
        rts                                     | +096

| ----------------------------------------------------------------------------
|  TaskHandler_06b78a  @ $06B78A  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b78a, "ax", @progbits
        .global TaskHandler_06b78a
TaskHandler_06b78a:
        jsr     0x5e7c0.l                       | +000
        move.b  0x99(a6),d0                     | +006
        andi.b  #0x1,d0                         | +00a
        move.b  d0,0x3a(a6)                     | +00e
        move.b  d0,0x75(a6)                     | +012
        move.b  0x9a(a6),0x7a(a6)               | +016
        jsr     TaskHandler_06b94c(pc)          | +01c
        move.w  #0x8000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x14,0x38(a6)                  | +030
        move.l  #0x2ca198,0x60(a6)              | +036
        move.l  #0x2ca1e4,0x48(a6)              | +03e
        lea     TaskHandler_06acb2(pc),a1       | +046
        jsr     0x4ae.l                         | +04a
        jsr     0x5dd02.l                       | +050
        clr.b   0x9e(a0)                        | +056
        move.b  0x9f(a6),0x9f(a0)               | +05a
        move.b  0x98(a6),0x98(a0)               | +060
        move.b  0x9a(a6),0x7a(a0)               | +066
        move.b  0x9c(a6),0x9c(a0)               | +06c
        move.b  0x9d(a6),0x9d(a0)               | +072
        move.b  0x9e(a6),0x9e(a0)               | +078
        bset    #0x2,0x6b(a6)                   | +07e
        rts                                     | +084

| ----------------------------------------------------------------------------
|  TaskHandler_06b810  @ $06B810  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b810, "ax", @progbits
        .global TaskHandler_06b810
TaskHandler_06b810:
        jsr     TaskHandler_06b94c(pc)          | +000
        move.w  #0x8000,d0                      | +004
        jsr     0x28134.l                       | +008
        andi.w  #0xffe3,0x38(a6)                | +00e
        ori.w   #0x8,0x38(a6)                   | +014
        lea     0x2bd3e6.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x66(a6)                     | +026
        move.l  #0x2ca1e4,0x48(a6)              | +02a
        move.b  0x9d(a6),0x75(a6)               | +032
        lea     TaskHandler_06acb2(pc),a1       | +038
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        clr.b   0x98(a0)                        | +048
        clr.b   0x9f(a0)                        | +04c
        clr.b   0x9c(a0)                        | +050
        clr.b   0x9d(a0)                        | +054
        clr.b   0x9e(a0)                        | +058
        move.b  0x9e(a6),0x9e(a0)               | +05c
        move.b  0x7a(a6),0x7a(a0)               | +062
        rts                                     | +068

| ----------------------------------------------------------------------------
|  TaskHandler_06b87a  @ $06B87A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b87a, "ax", @progbits
        .global TaskHandler_06b87a
TaskHandler_06b87a:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x22(a0),d0                     | +004
        move.w  0x24(a0),d1                     | +008
        move.w  0x38(a0),d2                     | +00c
        move.b  0x9a(a6),d3                     | +010
        move.b  0x9b(a6),d4                     | +014
        move.b  0x9c(a6),d5                     | +018
        ext.w   d3                              | +01c
        ext.w   d4                              | +01e
        ext.w   d5                              | +020
        btst    #0x0,0x3a(a0)                   | +022
        bne.w   .L06b8a8                        | +028
        neg.w   d3                              | +02c
.L06b8a8:
        add.w   d3,d0                           | +02e
        add.w   d4,d1                           | +030
        add.w   d5,d2                           | +032
        move.w  d0,0x22(a6)                     | +034
        move.w  d1,0x24(a6)                     | +038
        move.w  d2,0x38(a6)                     | +03c
        rts                                     | +040

| ----------------------------------------------------------------------------
|  TaskHandler_06b8bc  @ $06B8BC  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b8bc, "ax", @progbits
        .global TaskHandler_06b8bc
TaskHandler_06b8bc:
        move.w  0x22(a6),d0                     | +000
        move.w  0x98(a6),d1                     | +004
        btst    #0x0,0x75(a6)                   | +008
        beq.w   .L06b8d8                        | +00e
        cmp.w   d0,d1                           | +012
        bgt.w   ClearXN_06b8e4                  | +014
        bra.w   SetXN_06b8de                    | +018
.L06b8d8:
        cmp.w   d0,d1                           | +01c
        blt.w   ClearXN_06b8e4                  | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06b8ea  @ $06B8EA  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b8ea, "ax", @progbits
        .global TaskHandler_06b8ea
TaskHandler_06b8ea:
        lea     0x2ca644.l,a1                   | +000
        tst.b   0x9d(a6)                        | +006
        beq.w   .L06b8fe                        | +00a
        lea     0x2ca664.l,a1                   | +00e
.L06b8fe:
        movea.l 0xc(a6),a0                      | +014
        move.b  0x72(a0),d0                     | +018
        cmp.b   0x72(a6),d0                     | +01c
        beq.w   TaskHandler_06b942              | +020
        move.b  d0,0x72(a6)                     | +024
        cmpi.b  #0x8,d0                         | +028
        bcs.w   .L06b926                        | +02c
        nop                                     | +030
        nop                                     | +032
        cmpi.b  #0x8,d0                         | +034
        nop                                     | +038
        trap    #0xf                            | +03a
.L06b926:
        andi.w  #0x7,d0                         | +03c
        lsl.w   #0x2,d0                         | +040
        move.l  (a1,d0.w),d0                    | +042
        cmpi.l  #0xffffffff,d0                  | +046
        beq.w   ClearXN_06b946                  | +04c
        movea.l d0,a1                           | +050

| ----------------------------------------------------------------------------
|  TaskHandler_06b942  @ $06B942  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b942, "ax", @progbits
        .global TaskHandler_06b942
TaskHandler_06b942:
        move.b  d0,0x72(a6)                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06b94c  @ $06B94C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b94c, "ax", @progbits
        .global TaskHandler_06b94c
TaskHandler_06b94c:
        lea     0x2ca404.l,a1                   | +000
        move.b  0x7a(a6),d0                     | +006
        andi.w  #0x3,d0                         | +00a
        lsl.w   #0x1,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        jsr     0x236e.l                        | +014
        move.w  #0x7,0x1c(a6)                   | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_06b974  @ $06B974  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b974, "ax", @progbits
        .global TaskHandler_06b974
TaskHandler_06b974:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3e0.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x2ca3f2.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        move.w  #0x4000,0x38(a0)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_06b9a4  @ $06B9A4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b9a4, "ax", @progbits
        .global TaskHandler_06b9a4
TaskHandler_06b9a4:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3f2.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06b9da  @ $06B9DA  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06b9da, "ax", @progbits
        .global TaskHandler_06b9da
TaskHandler_06b9da:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ca3f2.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06ba10  @ $06BA10  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ba10, "ax", @progbits
        .global TaskHandler_06ba10
TaskHandler_06ba10:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06ba26                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06ba2c  @ $06BA2C  (214 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ba2c, "ax", @progbits
        .global TaskHandler_06ba2c
TaskHandler_06ba2c:
        clr.b   0x9e(a6)                        | +000
        bra.w   .L06ba3a                        | +004
        move.b  #0x1,0x9e(a6)                   | +008
.L06ba3a:
        jsr     0x283ca.l                       | +00e
        jsr     0x5e7c0.l                       | +014
        move.b  0x99(a6),d0                     | +01a
        andi.b  #0x1,d0                         | +01e
        move.b  d0,0x3a(a6)                     | +022
        move.b  d0,0x8d(a6)                     | +026
        jsr     TaskHandler_06d446(pc)          | +02a
        move.b  0x9a(a6),0x96(a6)               | +02e
        jsr     TaskHandler_06d574(pc)          | +034
        lea     0x2bd468.l,a0                   | +038
        jsr     0x799de.l                       | +03e
        move.w  d0,0x36(a6)                     | +044
        lea     0x2bd4ea.l,a0                   | +048
        jsr     0x799de.l                       | +04e
        move.w  d0,0x70(a6)                     | +054
        lea     0x2bd3e6.l,a0                   | +058
        jsr     0x799de.l                       | +05e
        move.w  d0,0x66(a6)                     | +064
        move.w  #0x8000,d0                      | +068
        jsr     0x28134.l                       | +06c
        andi.w  #0xffe3,0x38(a6)                | +072
        ori.w   #0x14,0x38(a6)                  | +078
        move.l  #0x2cc7a6,0x60(a6)              | +07e
        move.l  #0x2cc8d6,0x48(a6)              | +086
        clr.b   0x89(a6)                        | +08e
        move.w  #0x8,0x8a(a6)                   | +092
        move.b  #0xff,0x97(a6)                  | +098
        move.w  0x24(a6),0x82(a6)               | +09e
        jsr     0x30696.l                       | +0a4
        lea     TaskHandler_06c562(pc),a1       | +0aa
        jsr     0x4ae.l                         | +0ae
        jsr     0x5dd02.l                       | +0b4
        move.b  0x9f(a6),0x9f(a0)               | +0ba
        move.b  0x98(a6),0x98(a0)               | +0c0
        move.b  0x9a(a6),0x96(a0)               | +0c6
        bset    #0x2,0x6b(a6)                   | +0cc
        bra.w   TaskHandler_06bee0              | +0d2

| ----------------------------------------------------------------------------
|  TaskHandler_06bb02  @ $06BB02  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06bb02, "ax", @progbits
        .global TaskHandler_06bb02
TaskHandler_06bb02:
        move.b  #0x0,0x72(a6)                   | +000
        move.w  0x36(a6),d0                     | +006
        btst    #0x0,0x8d(a6)                   | +00a
        bne.w   .L06bb18                        | +010
        neg.w   d0                              | +014
.L06bb18:
        move.w  d0,0x28(a6)                     | +016
        jsr     TaskHandler_06d206(pc)          | +01a
        bcs.w   Data_06bb58__L06bb6c            | +01e
        jsr     TaskHandler_06d1dc(pc)          | +022
        bcs.w   TaskHandler_06c272              | +026
        move.b  d0,0x97(a6)                     | +02a
        andi.w  #0x3,d0                         | +02e
        movea.l #0x6bb58,a0                     | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L06bb50                        | +044
        jsr     0x28cd4.l                       | +048
.L06bb50:
        jsr     TaskHandler_06d484(pc)          | +04e
        bra.w   Data_06bb58__L06bb68            | +052

| ----------------------------------------------------------------------------
|  Data_06bb58  @ $06BB58  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bb58, "ax", @progbits
        .global Data_06bb58
Data_06bb58:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xda24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd8b0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd96a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bb58__L06bb68
Data_06bb58__L06bb68:
.L06bb68:
        bra.w   Data_06bba0__L06bbb0            | +010
        .global Data_06bb58__L06bb6c
Data_06bb58__L06bb6c:
.L06bb6c:
        jsr     TaskHandler_06d1dc(pc)          | +014
        bcs.w   TaskHandler_06c272              | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bba0,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bb98                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bb98:
        jsr     TaskHandler_06d484(pc)          | +040
        bra.w   Data_06bba0__L06bbb0            | +044

| ----------------------------------------------------------------------------
|  Data_06bba0  @ $06BBA0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bba0, "ax", @progbits
        .global Data_06bba0
Data_06bba0:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdc52                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdade                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdb98                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bba0__L06bbb0
Data_06bba0__L06bbb0:
.L06bbb0:
        lea     .L06bbb6(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06bbb6:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        bcc.w   .L06bbd0                        | +020
        eori.b  #0x1,0x8d(a6)                   | +024
        lea     TaskHandler_06bcfc(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L06bbd0:
        jsr     TaskHandler_06d1dc(pc)          | +030
        bcs.w   TaskHandler_06c272              | +034
        cmp.b   0x97(a6),d0                     | +038
        beq.w   Data_06bc64__L06bc74            | +03c
        jsr     TaskHandler_06d206(pc)          | +040
        bcs.w   Data_06bc1c__L06bc30            | +044
        jsr     TaskHandler_06d1dc(pc)          | +048
        bcs.w   TaskHandler_06c272              | +04c
        move.b  d0,0x97(a6)                     | +050
        andi.w  #0x3,d0                         | +054
        movea.l #0x6bc1c,a0                     | +058
        lsl.w   #0x2,d0                         | +05e
        movea.l (a0,d0.w),a0                    | +060
        cmpa.l  #0xffffffff,a0                  | +064
        beq.w   .L06bc14                        | +06a
        jsr     0x28cd4.l                       | +06e
.L06bc14:
        jsr     TaskHandler_06d484(pc)          | +074
        bra.w   Data_06bc1c__L06bc2c            | +078

| ----------------------------------------------------------------------------
|  Data_06bc1c  @ $06BC1C  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bc1c, "ax", @progbits
        .global Data_06bc1c
Data_06bc1c:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xda24                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xd8b0                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xd96a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bc1c__L06bc2c
Data_06bc1c__L06bc2c:
.L06bc2c:
        bra.w   Data_06bc64__L06bc74            | +010
        .global Data_06bc1c__L06bc30
Data_06bc1c__L06bc30:
.L06bc30:
        jsr     TaskHandler_06d1dc(pc)          | +014
        bcs.w   TaskHandler_06c272              | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bc64,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bc5c                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bc5c:
        jsr     TaskHandler_06d484(pc)          | +040
        bra.w   Data_06bc64__L06bc74            | +044

| ----------------------------------------------------------------------------
|  Data_06bc64  @ $06BC64  (152 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bc64, "ax", @progbits
        .global Data_06bc64
Data_06bc64:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xdc52                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdade                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdb98                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bc64__L06bc74
Data_06bc64__L06bc74:
.L06bc74:
        jsr     0x28d70.l                       | +010
        jsr     TaskHandler_06d172(pc)          | +016
        bcc.w   .L06bc92                        | +01a
        eori.b  #0x1,0x8d(a6)                   | +01e
        lea     TaskHandler_06bcfc(pc),a1       | +024
        move.l  a1,(a6)                         | +028
        bra.w   .L06bcda                        | +02a
.L06bc92:
        cmpi.w  #0x30,0x22(a6)                  | +02e
        bgt.w   .L06bcb6                        | +034
        btst    #0x0,0x8d(a6)                   | +038
        bne.w   .L06bcb6                        | +03e
        ori.b   #0x1,0x8d(a6)                   | +042
        clr.w   0x28(a6)                        | +048
        lea     TaskHandler_06bcfc(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06bcb6:
        cmpi.w  #0x110,0x22(a6)                 | +052
        blt.w   .L06bcda                        | +058
        btst    #0x0,0x8d(a6)                   | +05c
        beq.w   .L06bcda                        | +062
        andi.b  #0xfe,0x8d(a6)                  | +066
        clr.w   0x28(a6)                        | +06c
        lea     TaskHandler_06bcfc(pc),a1       | +070
        move.l  a1,(a6)                         | +074
.L06bcda:
        jsr     TaskHandler_06d228(pc)          | +076
        bcc.w   .L06bcec                        | +07a
        clr.w   0x28(a6)                        | +07e
        lea     TaskHandler_06bcfc(pc),a1       | +082
        move.l  a1,(a6)                         | +086
.L06bcec:
        jsr     0x283ca.l                       | +088
        jsr     0x283d8.l                       | +08e
        bra.w   TaskHandler_06c37c__L06c418     | +094

| ----------------------------------------------------------------------------
|  TaskHandler_06bcfc  @ $06BCFC  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06bcfc, "ax", @progbits
        .global TaskHandler_06bcfc
TaskHandler_06bcfc:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     TaskHandler_06d1dc(pc)          | +006
        bcs.w   TaskHandler_06c272              | +00a
        jsr     TaskHandler_06d206(pc)          | +00e
        bcc.w   Data_06bd46__L06bd5a            | +012
        jsr     TaskHandler_06d1dc(pc)          | +016
        bcs.w   TaskHandler_06c272              | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6bd46,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06bd3e                        | +038
        jsr     0x28cd4.l                       | +03c
.L06bd3e:
        jsr     TaskHandler_06d484(pc)          | +042
        bra.w   Data_06bd46__L06bd56            | +046

| ----------------------------------------------------------------------------
|  Data_06bd46  @ $06BD46  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bd46, "ax", @progbits
        .global Data_06bd46
Data_06bd46:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe038                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xde3c                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdf3a                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bd46__L06bd56
Data_06bd46__L06bd56:
.L06bd56:
        bra.w   Data_06bd8e__L06bd9e            | +010
        .global Data_06bd46__L06bd5a
Data_06bd46__L06bd5a:
.L06bd5a:
        jsr     TaskHandler_06d1dc(pc)          | +014
        bcs.w   TaskHandler_06c272              | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6bd8e,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06bd86                        | +036
        jsr     0x28cd4.l                       | +03a
.L06bd86:
        jsr     TaskHandler_06d484(pc)          | +040
        bra.w   Data_06bd8e__L06bd9e            | +044

| ----------------------------------------------------------------------------
|  Data_06bd8e  @ $06BD8E  (90 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bd8e, "ax", @progbits
        .global Data_06bd8e
Data_06bd8e:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe332                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe136                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe234                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bd8e__L06bd9e
Data_06bd8e__L06bd9e:
.L06bd9e:
        clr.w   0x28(a6)                        | +010
        lea     .L06bda8(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L06bda8:
        jsr     0x28998.l                       | +01a
        jsr     TaskHandler_06d0f6(pc)          | +020
        jsr     TaskHandler_06d1dc(pc)          | +024
        bcs.w   .L06bdc2                        | +028
        cmp.b   0x97(a6),d0                     | +02c
        beq.w   .L06bdc8                        | +030
.L06bdc2:
        lea     TaskHandler_06bee0(pc),a1       | +034
        move.l  a1,(a6)                         | +038
.L06bdc8:
        jsr     0x28d70.l                       | +03a
        bcc.w   .L06bdd8                        | +040
        lea     TaskHandler_06bee0(pc),a1       | +044
        move.l  a1,(a6)                         | +048
.L06bdd8:
        jsr     0x283ca.l                       | +04a
        jsr     0x283d8.l                       | +050
        bra.w   TaskHandler_06c37c__L06c418     | +056

| ----------------------------------------------------------------------------
|  TaskHandler_06bde8  @ $06BDE8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06bde8, "ax", @progbits
        .global TaskHandler_06bde8
TaskHandler_06bde8:
        move.b  #0x0,0x72(a6)                   | +000
        jsr     TaskHandler_06d1dc(pc)          | +006
        bcs.w   TaskHandler_06c272              | +00a
        jsr     TaskHandler_06d206(pc)          | +00e
        bcs.w   Data_06be32__L06be46            | +012
        jsr     TaskHandler_06d1dc(pc)          | +016
        bcs.w   TaskHandler_06c272              | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6be32,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06be2a                        | +038
        jsr     0x28cd4.l                       | +03c
.L06be2a:
        jsr     TaskHandler_06d484(pc)          | +042
        bra.w   Data_06be32__L06be42            | +046

| ----------------------------------------------------------------------------
|  Data_06be32  @ $06BE32  (72 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06be32, "ax", @progbits
        .global Data_06be32
Data_06be32:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe65c                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe430                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe546                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06be32__L06be42
Data_06be32__L06be42:
.L06be42:
        bra.w   Data_06be7a__L06be8a            | +010
        .global Data_06be32__L06be46
Data_06be32__L06be46:
.L06be46:
        jsr     TaskHandler_06d1dc(pc)          | +014
        bcs.w   TaskHandler_06c272              | +018
        move.b  d0,0x97(a6)                     | +01c
        andi.w  #0x3,d0                         | +020
        movea.l #0x6be7a,a0                     | +024
        lsl.w   #0x2,d0                         | +02a
        movea.l (a0,d0.w),a0                    | +02c
        cmpa.l  #0xffffffff,a0                  | +030
        beq.w   .L06be72                        | +036
        jsr     0x28cd4.l                       | +03a
.L06be72:
        jsr     TaskHandler_06d484(pc)          | +040
        bra.w   Data_06be7a__L06be8a            | +044

| ----------------------------------------------------------------------------
|  Data_06be7a  @ $06BE7A  (102 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06be7a, "ax", @progbits
        .global Data_06be7a
Data_06be7a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xe99e                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xe772                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xe888                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06be7a__L06be8a
Data_06be7a__L06be8a:
.L06be8a:
        lea     .L06be90(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06be90:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        bcc.w   .L06beaa                        | +020
        eori.b  #0x1,0x8d(a6)                   | +024
        lea     TaskHandler_06bcfc(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L06beaa:
        jsr     TaskHandler_06d1dc(pc)          | +030
        bcs.w   .L06beba                        | +034
        cmp.b   0x97(a6),d0                     | +038
        beq.w   .L06bec0                        | +03c
.L06beba:
        lea     TaskHandler_06bee0(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L06bec0:
        jsr     0x28d70.l                       | +046
        bcc.w   .L06bed0                        | +04c
        lea     TaskHandler_06bb02(pc),a1       | +050
        move.l  a1,(a6)                         | +054
.L06bed0:
        jsr     0x283ca.l                       | +056
        jsr     0x283d8.l                       | +05c
        bra.w   TaskHandler_06c37c__L06c418     | +062

| ----------------------------------------------------------------------------
|  TaskHandler_06bee0  @ $06BEE0  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06bee0, "ax", @progbits
        .global TaskHandler_06bee0
TaskHandler_06bee0:
        move.b  #0x0,0x72(a6)                   | +000
        lea     0x2bd670.l,a0                   | +006
        jsr     0x799de.l                       | +00c
        move.b  d0,0x88(a6)                     | +012
        jsr     TaskHandler_06d1dc(pc)          | +016
        bcs.w   TaskHandler_06c272              | +01a
        clr.w   0x28(a6)                        | +01e
        jsr     TaskHandler_06d1dc(pc)          | +022
        bcs.w   TaskHandler_06c272              | +026
        move.b  d0,0x97(a6)                     | +02a
        andi.w  #0x3,d0                         | +02e
        movea.l #0x6bf36,a0                     | +032
        lsl.w   #0x2,d0                         | +038
        movea.l (a0,d0.w),a0                    | +03a
        cmpa.l  #0xffffffff,a0                  | +03e
        beq.w   .L06bf2e                        | +044
        jsr     0x28cd4.l                       | +048
.L06bf2e:
        jsr     TaskHandler_06d484(pc)          | +04e
        bra.w   Data_06bf36__L06bf46            | +052

| ----------------------------------------------------------------------------
|  Data_06bf36  @ $06BF36  (100 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bf36, "ax", @progbits
        .global Data_06bf36
Data_06bf36:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bf36__L06bf46
Data_06bf36__L06bf46:
.L06bf46:
        lea     .L06bf4c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06bf4c:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        jsr     TaskHandler_06d1dc(pc)          | +020
        bcs.w   .L06bf66                        | +024
        cmp.b   0x97(a6),d0                     | +028
        beq.w   Data_06bf9a__L06bfaa            | +02c
.L06bf66:
        jsr     TaskHandler_06d1dc(pc)          | +030
        bcs.w   TaskHandler_06c272              | +034
        move.b  d0,0x97(a6)                     | +038
        andi.w  #0x3,d0                         | +03c
        movea.l #0x6bf9a,a0                     | +040
        lsl.w   #0x2,d0                         | +046
        movea.l (a0,d0.w),a0                    | +048
        cmpa.l  #0xffffffff,a0                  | +04c
        beq.w   .L06bf92                        | +052
        jsr     0x28cd4.l                       | +056
.L06bf92:
        jsr     TaskHandler_06d484(pc)          | +05c
        bra.w   Data_06bf9a__L06bfaa            | +060

| ----------------------------------------------------------------------------
|  Data_06bf9a  @ $06BF9A  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06bf9a, "ax", @progbits
        .global Data_06bf9a
Data_06bf9a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06bf9a__L06bfaa
Data_06bf9a__L06bfaa:
.L06bfaa:
        jsr     0x28d70.l                       | +010
        subq.b  #0x1,0x88(a6)                   | +016
        bgt.w   .L06bfbe                        | +01a
        lea     TaskHandler_06bde8(pc),a1       | +01e
        move.l  a1,(a6)                         | +022
.L06bfbe:
        jsr     TaskHandler_06d228(pc)          | +024
        bcc.w   .L06bfcc                        | +028
        lea     TaskHandler_06bfd0(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L06bfcc:
        bra.w   TaskHandler_06c37c__L06c418     | +032

| ----------------------------------------------------------------------------
|  TaskHandler_06bfd0  @ $06BFD0  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06bfd0, "ax", @progbits
        .global TaskHandler_06bfd0
TaskHandler_06bfd0:
        jsr     0x267e2.l                       | +000
        jsr     0x5e0d4.l                       | +006
        move.l  a0,0x90(a6)                     | +00c
        move.b  #0x2,0x72(a6)                   | +010
        clr.b   0x88(a6)                        | +016
        clr.b   0x73(a6)                        | +01a
        jsr     TaskHandler_06d1dc(pc)          | +01e
        bcs.w   TaskHandler_06c272              | +022
        jsr     TaskHandler_06d1dc(pc)          | +026
        bcs.w   TaskHandler_06c272              | +02a
        move.b  d0,0x97(a6)                     | +02e
        andi.w  #0x3,d0                         | +032
        movea.l #0x6c02a,a0                     | +036
        lsl.w   #0x2,d0                         | +03c
        movea.l (a0,d0.w),a0                    | +03e
        cmpa.l  #0xffffffff,a0                  | +042
        beq.w   .L06c022                        | +048
        jsr     0x28cd4.l                       | +04c
.L06c022:
        jsr     TaskHandler_06d484(pc)          | +052
        bra.w   Data_06c02a__L06c03a            | +056

| ----------------------------------------------------------------------------
|  Data_06c02a  @ $06C02A  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c02a, "ax", @progbits
        .global Data_06c02a
Data_06c02a:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c02a__L06c03a
Data_06c02a__L06c03a:
.L06c03a:
        lea     .L06c040(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c040:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c05e                        | +02a
        lea     TaskHandler_06c062(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L06c05e:
        bra.w   TaskHandler_06c37c__L06c418     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06c062  @ $06C062  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c062, "ax", @progbits
        .global TaskHandler_06c062
TaskHandler_06c062:
        lea     0x2bd5ee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.b  d0,0x88(a6)                     | +00c
        bra.w   TaskHandler_06c076__L06c084     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06c076  @ $06C076  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c076, "ax", @progbits
        .global TaskHandler_06c076
TaskHandler_06c076:
        subq.b  #0x1,0x88(a6)                   | +000
        cmpi.b  #0x0,0x88(a6)                   | +004
        ble.w   Data_06c0d2__L06c122            | +00a
        .global TaskHandler_06c076__L06c084
TaskHandler_06c076__L06c084:
.L06c084:
        clr.b   0x73(a6)                        | +00e
        move.b  #0x3,0x72(a6)                   | +012
        lea     0x2bd56c.l,a0                   | +018
        jsr     0x799de.l                       | +01e
        move.w  d0,0x70(a6)                     | +024
        jsr     TaskHandler_06d1dc(pc)          | +028
        bcs.w   TaskHandler_06c272              | +02c
        move.b  d0,0x97(a6)                     | +030
        andi.w  #0x3,d0                         | +034
        movea.l #0x6c0d2,a0                     | +038
        lsl.w   #0x2,d0                         | +03e
        movea.l (a0,d0.w),a0                    | +040
        cmpa.l  #0xffffffff,a0                  | +044
        beq.w   .L06c0ca                        | +04a
        jsr     0x28cd4.l                       | +04e
.L06c0ca:
        jsr     TaskHandler_06d484(pc)          | +054
        bra.w   Data_06c0d2__L06c0e2            | +058

| ----------------------------------------------------------------------------
|  Data_06c0d2  @ $06C0D2  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c0d2, "ax", @progbits
        .global Data_06c0d2
Data_06c0d2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xeb54                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xeab4                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xeb04                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c0d2__L06c0e2
Data_06c0d2__L06c0e2:
.L06c0e2:
        lea     .L06c0e8(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c0e8:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c10a                        | +02a
        clr.b   0x73(a6)                        | +02e
        move.b  #0x5,0x72(a6)                   | +032
.L06c10a:
        subq.w  #0x1,0x70(a6)                   | +038
        cmpi.w  #0x0,0x70(a6)                   | +03c
        bgt.w   .L06c11e                        | +042
        lea     TaskHandler_06c076(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L06c11e:
        bra.w   TaskHandler_06c37c__L06c418     | +04c
        .global Data_06c0d2__L06c122
Data_06c0d2__L06c122:
.L06c122:
        lea     0x2bd4ea.l,a0                   | +050
        jsr     0x799de.l                       | +056
        move.w  d0,0x70(a6)                     | +05c
        addi.w  #0x1e,0x70(a6)                  | +060
        clr.b   0x88(a6)                        | +066
        clr.b   0x73(a6)                        | +06a
        jsr     TaskHandler_06d1dc(pc)          | +06e
        bcs.w   TaskHandler_06c272              | +072
        move.b  #0x4,0x72(a6)                   | +076
        jsr     TaskHandler_06d1dc(pc)          | +07c
        bcs.w   TaskHandler_06c272              | +080
        move.b  d0,0x97(a6)                     | +084
        andi.w  #0x3,d0                         | +088
        movea.l #0x6c182,a0                     | +08c
        lsl.w   #0x2,d0                         | +092
        movea.l (a0,d0.w),a0                    | +094
        cmpa.l  #0xffffffff,a0                  | +098
        beq.w   .L06c17a                        | +09e
        jsr     0x28cd4.l                       | +0a2
.L06c17a:
        jsr     TaskHandler_06d484(pc)          | +0a8
        bra.w   Data_06c182__L06c192            | +0ac

| ----------------------------------------------------------------------------
|  Data_06c182  @ $06C182  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c182, "ax", @progbits
        .global Data_06c182
Data_06c182:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xddee                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xdd52                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xdda0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c182__L06c192
Data_06c182__L06c192:
.L06c192:
        lea     .L06c198(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c198:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        jsr     0x28d70.l                       | +020
        tst.b   0x73(a6)                        | +026
        beq.w   .L06c1b6                        | +02a
        lea     TaskHandler_06bde8(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L06c1b6:
        bra.w   TaskHandler_06c37c__L06c418     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06c1ba  @ $06C1BA  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c1ba, "ax", @progbits
        .global TaskHandler_06c1ba
TaskHandler_06c1ba:
        bclr    #0x3,0x13(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        jsr     TaskHandler_06d1dc(pc)          | +00a
        bcs.w   TaskHandler_06c272              | +00e
        move.b  #0x6,0x72(a6)                   | +012
        jsr     TaskHandler_06d1dc(pc)          | +018
        bcs.w   TaskHandler_06c272              | +01c
        move.b  d0,0x97(a6)                     | +020
        andi.w  #0x3,d0                         | +024
        movea.l #0x6c206,a0                     | +028
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c1fe                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c1fe:
        jsr     TaskHandler_06d484(pc)          | +044
        bra.w   Data_06c206__L06c216            | +048

| ----------------------------------------------------------------------------
|  Data_06c206  @ $06C206  (108 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c206, "ax", @progbits
        .global Data_06c206
Data_06c206:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xf124                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xee74                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xefcc                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c206__L06c216
Data_06c206__L06c216:
.L06c216:
        lea     .L06c21c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c21c:
        jsr     0x28998.l                       | +016
        jsr     TaskHandler_06d0f6(pc)          | +01c
        jsr     TaskHandler_06d1dc(pc)          | +020
        bcs.w   .L06c236                        | +024
        cmp.b   0x97(a6),d0                     | +028
        beq.w   .L06c23c                        | +02c
.L06c236:
        lea     TaskHandler_06bee0(pc),a1       | +030
        move.l  a1,(a6)                         | +034
.L06c23c:
        jsr     0x28d70.l                       | +036
        bcc.w   .L06c252                        | +03c
        bclr    #0x3,0x13(a6)                   | +040
        lea     TaskHandler_06bde8(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L06c252:
        jsr     0x2870a.l                       | +04c
        bcc.w   .L06c26e                        | +052
        lea     0x5e766.l,a0                    | +056
        jsr     0x5e770.l                       | +05c
        bclr    #0x3,0x13(a6)                   | +062
.L06c26e:
        bra.w   TaskHandler_06c37c__L06c448     | +068

| ----------------------------------------------------------------------------
|  TaskHandler_06c272  @ $06C272  (266 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c272, "ax", @progbits
        .global TaskHandler_06c272
TaskHandler_06c272:
        move.w  0x36(a6),d0                     | +000
        btst    #0x0,0x8d(a6)                   | +004
        bne.w   .L06c282                        | +00a
        neg.w   d0                              | +00e
.L06c282:
        move.w  d0,0x28(a6)                     | +010
        move.b  #0x1,0x72(a6)                   | +014
        move.l  #0x2cc9d2,0x48(a6)              | +01a
        move.l  #0x2cc88a,0x60(a6)              | +022
        move.l  #0x2ccda2,d0                    | +02a
        tst.b   0x9e(a6)                        | +030
        beq.w   .L06c2b0                        | +034
        move.l  #0x2cce42,d0                    | +038
.L06c2b0:
        move.l  d0,0x4c(a6)                     | +03e
        lea     0x2cd636.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     .L06c2c6(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L06c2c6:
        jsr     0x28998.l                       | +054
        jsr     TaskHandler_06d0f6(pc)          | +05a
        jsr     TaskHandler_06d1dc(pc)          | +05e
        bcs.w   .L06c2de                        | +062
        lea     TaskHandler_06bb02(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L06c2de:
        jsr     0x28d70.l                       | +06c
        jsr     0x2870a.l                       | +072
        bcc.w   .L06c2f4                        | +078
        bclr    #0x3,0x13(a6)                   | +07c
.L06c2f4:
        subq.w  #0x1,0x70(a6)                   | +082
        jsr     PcThunkTarget_06d13c(pc)        | +086
        bcc.w   .L06c310                        | +08a
        eori.b  #0x1,0x8d(a6)                   | +08e
        lea     TaskHandler_06c37c(pc),a1       | +094
        move.l  a1,(a6)                         | +098
        bra.w   .L06c36c                        | +09a
.L06c310:
        jsr     TaskHandler_06d172(pc)          | +09e
        bcc.w   .L06c328                        | +0a2
        eori.b  #0x1,0x8d(a6)                   | +0a6
        lea     TaskHandler_06c37c(pc),a1       | +0ac
        move.l  a1,(a6)                         | +0b0
        bra.w   .L06c36c                        | +0b2
.L06c328:
        cmpi.w  #0x10,0x22(a6)                  | +0b6
        bgt.w   .L06c34c                        | +0bc
        btst    #0x0,0x8d(a6)                   | +0c0
        bne.w   .L06c34c                        | +0c6
        ori.b   #0x1,0x8d(a6)                   | +0ca
        lea     TaskHandler_06c37c(pc),a1       | +0d0
        move.l  a1,(a6)                         | +0d4
        bra.w   .L06c36c                        | +0d6
.L06c34c:
        cmpi.w  #0x130,0x22(a6)                 | +0da
        blt.w   .L06c36c                        | +0e0
        btst    #0x0,0x8d(a6)                   | +0e4
        beq.w   .L06c36c                        | +0ea
        andi.b  #0xfe,0x8d(a6)                  | +0ee
        lea     TaskHandler_06c37c(pc),a1       | +0f4
        move.l  a1,(a6)                         | +0f8
.L06c36c:
        jsr     0x283ca.l                       | +0fa
        jsr     0x283d8.l                       | +100
        bra.w   TaskHandler_06c37c__L06c448     | +106

| ----------------------------------------------------------------------------
|  TaskHandler_06c37c  @ $06C37C  (228 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c37c, "ax", @progbits
        .global TaskHandler_06c37c
TaskHandler_06c37c:
        jsr     0x267e2.l                       | +000
        move.b  #0x1,0x72(a6)                   | +006
        move.l  #0x2cc9d2,0x48(a6)              | +00c
        move.l  #0x2cc88a,0x60(a6)              | +014
        move.l  #0x2ccda2,d0                    | +01c
        tst.b   0x9e(a6)                        | +022
        beq.w   .L06c3ac                        | +026
        move.l  #0x2cce42,d0                    | +02a
.L06c3ac:
        move.l  d0,0x4c(a6)                     | +030
        lea     0x2bd670.l,a0                   | +034
        jsr     0x799de.l                       | +03a
        lsr.b   #0x1,d0                         | +040
        move.b  d0,0x88(a6)                     | +042
        lea     0x2cd636.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     .L06c3d4(pc),a1                 | +052
        move.l  a1,(a6)                         | +056
.L06c3d4:
        jsr     0x28998.l                       | +058
        jsr     TaskHandler_06d0f6(pc)          | +05e
        jsr     TaskHandler_06d1dc(pc)          | +062
        bcs.w   .L06c3ec                        | +066
        lea     TaskHandler_06bee0(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
.L06c3ec:
        jsr     0x28d70.l                       | +070
        jsr     0x2870a.l                       | +076
        bcc.w   .L06c402                        | +07c
        bclr    #0x3,0x13(a6)                   | +080
.L06c402:
        subq.w  #0x1,0x70(a6)                   | +086
        subq.b  #0x1,0x88(a6)                   | +08a
        bgt.w   .L06c414                        | +08e
        lea     TaskHandler_06c272(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L06c414:
        bra.w   .L06c448                        | +098
        .global TaskHandler_06c37c__L06c418
TaskHandler_06c37c__L06c418:
.L06c418:
        clr.b   0x94(a6)                        | +09c
        jsr     0x2870a.l                       | +0a0
        bcc.w   .L06c448                        | +0a6
        lea     0x5e766.l,a0                    | +0aa
        jsr     0x5e770.l                       | +0b0
        bclr    #0x3,0x13(a6)                   | +0b6
        move.b  #0x1,0x94(a6)                   | +0bc
        bra.w   .L06c448                        | +0c2
        lea     TaskHandler_06c1ba(pc),a1       | +0c6
        move.l  a1,(a6)                         | +0ca
        .global TaskHandler_06c37c__L06c448
TaskHandler_06c37c__L06c448:
.L06c448:
        jsr     0x28758.l                       | +0cc
        bcc.w   .L06c458                        | +0d2
        lea     TaskHandler_06c468(pc),a1       | +0d6
        move.l  a1,(a6)                         | +0da
.L06c458:
        jsr     TaskHandler_06d05c(pc)          | +0dc
        bcc.w   SetHandlerRts_06c466            | +0e0

| ----------------------------------------------------------------------------
|  TaskHandler_06c468  @ $06C468  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c468, "ax", @progbits
        .global TaskHandler_06c468
TaskHandler_06c468:
        bclr    #0x1,0x12(a6)                   | +000
        clr.w   0x28(a6)                        | +006
        move.b  #0x7,0x72(a6)                   | +00a
        jsr     TaskHandler_06d1dc(pc)          | +010
        andi.w  #0x3,d0                         | +014
        movea.l #0x2ccfb6,a0                    | +018
        lsl.w   #0x2,d0                         | +01e
        movea.l (a0,d0.w),a0                    | +020
        cmpa.l  #0xffffffff,a0                  | +024
        beq.w   .L06c49c                        | +02a
        jsr     0x28cd4.l                       | +02e
.L06c49c:
        lea     .L06c4a2(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L06c4a2:
        jsr     TaskHandler_06d0f6(pc)          | +03a
        jsr     0x28d70.l                       | +03e
        bcc.w   .L06c4b6                        | +044
        lea     TaskHandler_06c4c6(pc),a1       | +048
        move.l  a1,(a6)                         | +04c
.L06c4b6:
        jsr     TaskHandler_06d05c(pc)          | +04e
        bcc.w   SetHandlerRts_06c4c4            | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06c4c6  @ $06C4C6  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c4c6, "ax", @progbits
        .global TaskHandler_06c4c6
TaskHandler_06c4c6:
        move.l  0x74(a6),d0                     | +000
        cmpi.l  #0xffffffff,d0                  | +004
        beq.w   .L06c4dc                        | +00a
        movea.l d0,a0                           | +00e
        move.l  #0x6c554,(a0)                   | +010
.L06c4dc:
        move.l  0x78(a6),d0                     | +016
        cmpi.l  #0xffffffff,d0                  | +01a
        beq.w   .L06c4f2                        | +020
        movea.l d0,a0                           | +024
        move.l  #0x6c554,(a0)                   | +026
.L06c4f2:
        move.l  0x7c(a6),d0                     | +02c
        cmpi.l  #0xffffffff,d0                  | +030
        beq.w   .L06c508                        | +036
        movea.l d0,a0                           | +03a
        move.l  #0x6c554,(a0)                   | +03c
.L06c508:
        move.w  #0x3c,0x70(a6)                  | +042
        move.l  #0xffffffff,0x48(a6)            | +048
        bclr    #0x1,0x12(a6)                   | +050
        move.b  #0x7,0x72(a6)                   | +056
        lea     .L06c528(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L06c528:
        subq.w  #0x1,0x70(a6)                   | +062
        cmpi.w  #0x0,0x70(a6)                   | +066
        bgt.w   SetHandlerRts_06c53c            | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_06c562  @ $06C562  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c562, "ax", @progbits
        .global TaskHandler_06c562
TaskHandler_06c562:
        jsr     TaskHandler_06d574(pc)          | +000
        andi.b  #0x3,0x98(a6)                   | +004
        move.b  #0x0,0x72(a6)                   | +00a
        .global TaskHandler_06c562__L06c572
TaskHandler_06c562__L06c572:
.L06c572:
        move.w  #0x8,0x34(a6)                   | +010
        movea.l 0xc(a6),a0                      | +016
        move.w  0x8a(a0),0x8a(a6)               | +01a
        jsr     TaskHandler_06d1dc(pc)          | +020
        bcs.w   Data_06c6b6__L06c6ca            | +024
        move.b  d0,0x97(a6)                     | +028
        andi.w  #0x3,d0                         | +02c
        movea.l #0x6c5b2,a0                     | +030
        lsl.w   #0x2,d0                         | +036
        movea.l (a0,d0.w),a0                    | +038
        cmpa.l  #0xffffffff,a0                  | +03c
        beq.w   .L06c5ae                        | +042
        jsr     0x28cd4.l                       | +046
.L06c5ae:
        bra.w   Data_06c5b2__L06c5c2            | +04c

| ----------------------------------------------------------------------------
|  Data_06c5b2  @ $06C5B2  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c5b2, "ax", @progbits
        .global Data_06c5b2
Data_06c5b2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c5b2__L06c5c2
Data_06c5b2__L06c5c2:
.L06c5c2:
        lea     .L06c5c8(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c5c8:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06c60c                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   TaskHandler_06cdba              | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06c5fa                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06c5fa:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06c60c:
        move.b  d0,0x72(a6)                     | +05a
        jsr     TaskHandler_06d29c(pc)          | +05e
        jsr     TaskHandler_06d1dc(pc)          | +062
        bcs.w   Data_06c6b6__L06c6ca            | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   Data_06c662__L06c672            | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     TaskHandler_06d1dc(pc)          | +080
        bcs.w   Data_06c6b6__L06c6ca            | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6c662,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06c65e                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06c65e:
        bra.w   Data_06c662__L06c672            | +0ac

| ----------------------------------------------------------------------------
|  Data_06c662  @ $06C662  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c662, "ax", @progbits
        .global Data_06c662
Data_06c662:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c662__L06c672
Data_06c662__L06c672:
.L06c672:
        jsr     0x28d70.l                       | +010
        bcc.w   Data_06c6b6__L06c6c6            | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     TaskHandler_06d1dc(pc)          | +024
        bcs.w   Data_06c6b6__L06c6ca            | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6c6b6,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06c6b2                        | +046
        jsr     0x28cd4.l                       | +04a
.L06c6b2:
        bra.w   Data_06c6b6__L06c6c6            | +050

| ----------------------------------------------------------------------------
|  Data_06c6b6  @ $06C6B6  (128 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c6b6, "ax", @progbits
        .global Data_06c6b6
Data_06c6b6:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c6b6__L06c6c6
Data_06c6b6__L06c6c6:
.L06c6c6:
        bra.w   TaskHandler_06cd62              | +010
        .global Data_06c6b6__L06c6ca
Data_06c6b6__L06c6ca:
.L06c6ca:
        move.w  #0x8,0x34(a6)                   | +014
        lea     0x2cf614.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L06c6e2(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L06c6e2:
        movea.l 0xc(a6),a0                      | +02c
        move.b  0x72(a0),d0                     | +030
        cmp.b   0x72(a6),d0                     | +034
        beq.w   .L06c726                        | +038
        cmpi.b  #0x7,0x72(a6)                   | +03c
        beq.w   TaskHandler_06cdba              | +042
        move.b  d0,0x72(a6)                     | +046
        cmpi.b  #0x9,d0                         | +04a
        bcs.w   .L06c714                        | +04e
        nop                                     | +052
        nop                                     | +054
        cmpi.b  #0x9,d0                         | +056
        nop                                     | +05a
        trap    #0xf                            | +05c
.L06c714:
        andi.w  #0xf,d0                         | +05e
        lea     0x2cd606.l,a0                   | +062
        lsl.w   #0x2,d0                         | +068
        movea.l (a0,d0.w),a1                    | +06a
        jmp     (a1)                            | +06e
.L06c726:
        move.b  d0,0x72(a6)                     | +070
        jsr     TaskHandler_06d29c(pc)          | +074
        jsr     TaskHandler_06d1dc(pc)          | +078
        bcc.w   TaskHandler_06c562__L06c572     | +07c

| ----------------------------------------------------------------------------
|  TaskHandler_06c73e  @ $06C73E  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c73e, "ax", @progbits
        .global TaskHandler_06c73e
TaskHandler_06c73e:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.l  0x90(a0),0x90(a6)               | +008
        movea.l 0xc(a6),a0                      | +00e
        move.w  0x8a(a0),0x8a(a6)               | +012
        jsr     TaskHandler_06d1dc(pc)          | +018
        bcs.w   Data_06c6b6__L06c6ca            | +01c
        move.b  d0,0x97(a6)                     | +020
        andi.w  #0x3,d0                         | +024
        movea.l #0x6c786,a0                     | +028
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c782                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c782:
        bra.w   Data_06c786__L06c796            | +044

| ----------------------------------------------------------------------------
|  Data_06c786  @ $06C786  (220 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c786, "ax", @progbits
        .global Data_06c786
Data_06c786:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c786__L06c796
Data_06c786__L06c796:
.L06c796:
        lea     .L06c79c(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c79c:
        addq.b  #0x1,0x88(a6)                   | +016
        andi.b  #0x3,0x88(a6)                   | +01a
        bne.w   .L06c7c8                        | +020
        jsr     TaskHandler_06d2fa(pc)          | +024
        movea.l 0xc(a6),a0                      | +028
        clr.b   0x73(a0)                        | +02c
        jsr     TaskHandler_06d340(pc)          | +030
        bcc.w   .L06c7c8                        | +034
        movea.l 0xc(a6),a0                      | +038
        move.b  #0x1,0x73(a0)                   | +03c
.L06c7c8:
        movea.l 0xc(a6),a0                      | +042
        move.b  0x72(a0),d0                     | +046
        cmp.b   0x72(a6),d0                     | +04a
        beq.w   .L06c80c                        | +04e
        cmpi.b  #0x7,0x72(a6)                   | +052
        beq.w   TaskHandler_06cdba              | +058
        move.b  d0,0x72(a6)                     | +05c
        cmpi.b  #0x9,d0                         | +060
        bcs.w   .L06c7fa                        | +064
        nop                                     | +068
        nop                                     | +06a
        cmpi.b  #0x9,d0                         | +06c
        nop                                     | +070
        trap    #0xf                            | +072
.L06c7fa:
        andi.w  #0xf,d0                         | +074
        lea     0x2cd606.l,a0                   | +078
        lsl.w   #0x2,d0                         | +07e
        movea.l (a0,d0.w),a1                    | +080
        jmp     (a1)                            | +084
.L06c80c:
        move.b  d0,0x72(a6)                     | +086
        jsr     TaskHandler_06d29c(pc)          | +08a
        jsr     TaskHandler_06d1dc(pc)          | +08e
        bcs.w   Data_06c6b6__L06c6ca            | +092
        cmp.b   0x97(a6),d0                     | +096
        beq.w   Data_06c862__L06c872            | +09a
        move.b  d0,0x97(a6)                     | +09e
        movea.l 0xc(a6),a0                      | +0a2
        move.w  0x8a(a0),0x8a(a6)               | +0a6
        jsr     TaskHandler_06d1dc(pc)          | +0ac
        bcs.w   Data_06c6b6__L06c6ca            | +0b0
        move.b  d0,0x97(a6)                     | +0b4
        andi.w  #0x3,d0                         | +0b8
        movea.l #0x6c862,a0                     | +0bc
        lsl.w   #0x2,d0                         | +0c2
        movea.l (a0,d0.w),a0                    | +0c4
        cmpa.l  #0xffffffff,a0                  | +0c8
        beq.w   .L06c85e                        | +0ce
        jsr     0x28cd4.l                       | +0d2
.L06c85e:
        bra.w   Data_06c862__L06c872            | +0d8

| ----------------------------------------------------------------------------
|  Data_06c862  @ $06C862  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c862, "ax", @progbits
        .global Data_06c862
Data_06c862:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c862__L06c872
Data_06c862__L06c872:
.L06c872:
        jsr     0x28d70.l                       | +010
        bcc.w   Data_06c8b6__L06c8c6            | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     TaskHandler_06d1dc(pc)          | +024
        bcs.w   Data_06c6b6__L06c6ca            | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6c8b6,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06c8b2                        | +046
        jsr     0x28cd4.l                       | +04a
.L06c8b2:
        bra.w   Data_06c8b6__L06c8c6            | +050

| ----------------------------------------------------------------------------
|  Data_06c8b6  @ $06C8B6  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c8b6, "ax", @progbits
        .global Data_06c8b6
Data_06c8b6:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c8b6__L06c8c6
Data_06c8b6__L06c8c6:
.L06c8c6:
        bra.w   TaskHandler_06cd62              | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06c8ca  @ $06C8CA  (176 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c8ca, "ax", @progbits
        .global TaskHandler_06c8ca
TaskHandler_06c8ca:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x8a(a0),0x8a(a6)               | +004
        jsr     TaskHandler_06d1dc(pc)          | +00a
        bcs.w   Data_06c6b6__L06c6ca            | +00e
        move.b  d0,0x97(a6)                     | +012
        andi.w  #0x3,d0                         | +016
        lea     0x2cd02a.l,a0                   | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        move.w  0x34(a6),d0                     | +026
        andi.w  #0xf,d0                         | +02a
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L06c90e                        | +03a
        jsr     0x28cd4.l                       | +03e
.L06c90e:
        jsr     TaskHandler_06d4c2(pc)          | +044
        lea     .L06c918(pc),a1                 | +048
        move.l  a1,(a6)                         | +04c
.L06c918:
        movea.l 0xc(a6),a0                      | +04e
        move.b  0x72(a0),d0                     | +052
        cmp.b   0x72(a6),d0                     | +056
        beq.w   .L06c95c                        | +05a
        cmpi.b  #0x7,0x72(a6)                   | +05e
        beq.w   TaskHandler_06cdba              | +064
        move.b  d0,0x72(a6)                     | +068
        cmpi.b  #0x9,d0                         | +06c
        bcs.w   .L06c94a                        | +070
        nop                                     | +074
        nop                                     | +076
        cmpi.b  #0x9,d0                         | +078
        nop                                     | +07c
        trap    #0xf                            | +07e
.L06c94a:
        andi.w  #0xf,d0                         | +080
        lea     0x2cd606.l,a0                   | +084
        lsl.w   #0x2,d0                         | +08a
        movea.l (a0,d0.w),a1                    | +08c
        jmp     (a1)                            | +090
.L06c95c:
        move.b  d0,0x72(a6)                     | +092
        jsr     TaskHandler_06d29c(pc)          | +096
        jsr     0x28d70.l                       | +09a
        bcc.w   .L06c978                        | +0a0
        movea.l 0xc(a6),a0                      | +0a4
        move.b  #0x1,0x73(a0)                   | +0a8
.L06c978:
        rts                                     | +0ae

| ----------------------------------------------------------------------------
|  TaskHandler_06c97a  @ $06C97A  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06c97a, "ax", @progbits
        .global TaskHandler_06c97a
TaskHandler_06c97a:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.w  0x8a(a0),0x8a(a6)               | +008
        jsr     TaskHandler_06d1dc(pc)          | +00e
        bcs.w   Data_06c6b6__L06c6ca            | +012
        move.b  d0,0x97(a6)                     | +016
        andi.w  #0x3,d0                         | +01a
        movea.l #0x6c9b8,a0                     | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L06c9b4                        | +030
        jsr     0x28cd4.l                       | +034
.L06c9b4:
        bra.w   Data_06c9b8__L06c9c8            | +03a

| ----------------------------------------------------------------------------
|  Data_06c9b8  @ $06C9B8  (230 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06c9b8, "ax", @progbits
        .global Data_06c9b8
Data_06c9b8:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06c9b8__L06c9c8
Data_06c9b8__L06c9c8:
.L06c9c8:
        lea     .L06c9ce(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06c9ce:
        addq.b  #0x1,0x88(a6)                   | +016
        andi.b  #0x3,0x88(a6)                   | +01a
        bne.w   .L06ca04                        | +020
        move.w  #0x1,d1                         | +024
        cmpi.w  #0x8,0x34(a6)                   | +028
        beq.w   .L06c9fa                        | +02e
        blt.w   .L06c9f2                        | +032
        move.w  #0xffff,d1                      | +036
.L06c9f2:
        add.w   d1,0x34(a6)                     | +03a
        bra.w   .L06ca04                        | +03e
.L06c9fa:
        movea.l 0xc(a6),a0                      | +042
        move.b  #0x1,0x73(a0)                   | +046
.L06ca04:
        movea.l 0xc(a6),a0                      | +04c
        move.b  0x72(a0),d0                     | +050
        cmp.b   0x72(a6),d0                     | +054
        beq.w   .L06ca48                        | +058
        cmpi.b  #0x7,0x72(a6)                   | +05c
        beq.w   TaskHandler_06cdba              | +062
        move.b  d0,0x72(a6)                     | +066
        cmpi.b  #0x9,d0                         | +06a
        bcs.w   .L06ca36                        | +06e
        nop                                     | +072
        nop                                     | +074
        cmpi.b  #0x9,d0                         | +076
        nop                                     | +07a
        trap    #0xf                            | +07c
.L06ca36:
        andi.w  #0xf,d0                         | +07e
        lea     0x2cd606.l,a0                   | +082
        lsl.w   #0x2,d0                         | +088
        movea.l (a0,d0.w),a1                    | +08a
        jmp     (a1)                            | +08e
.L06ca48:
        move.b  d0,0x72(a6)                     | +090
        jsr     TaskHandler_06d29c(pc)          | +094
        jsr     TaskHandler_06d1dc(pc)          | +098
        bcs.w   Data_06c6b6__L06c6ca            | +09c
        cmp.b   0x97(a6),d0                     | +0a0
        beq.w   Data_06ca9e__L06caae            | +0a4
        move.b  d0,0x97(a6)                     | +0a8
        movea.l 0xc(a6),a0                      | +0ac
        move.w  0x8a(a0),0x8a(a6)               | +0b0
        jsr     TaskHandler_06d1dc(pc)          | +0b6
        bcs.w   Data_06c6b6__L06c6ca            | +0ba
        move.b  d0,0x97(a6)                     | +0be
        andi.w  #0x3,d0                         | +0c2
        movea.l #0x6ca9e,a0                     | +0c6
        lsl.w   #0x2,d0                         | +0cc
        movea.l (a0,d0.w),a0                    | +0ce
        cmpa.l  #0xffffffff,a0                  | +0d2
        beq.w   .L06ca9a                        | +0d8
        jsr     0x28cd4.l                       | +0dc
.L06ca9a:
        bra.w   Data_06ca9e__L06caae            | +0e2

| ----------------------------------------------------------------------------
|  Data_06ca9e  @ $06CA9E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06ca9e, "ax", @progbits
        .global Data_06ca9e
Data_06ca9e:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06ca9e__L06caae
Data_06ca9e__L06caae:
.L06caae:
        jsr     0x28d70.l                       | +010
        bcc.w   Data_06caf2__L06cb02            | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     TaskHandler_06d1dc(pc)          | +024
        bcs.w   Data_06c6b6__L06c6ca            | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6caf2,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06caee                        | +046
        jsr     0x28cd4.l                       | +04a
.L06caee:
        bra.w   Data_06caf2__L06cb02            | +050

| ----------------------------------------------------------------------------
|  Data_06caf2  @ $06CAF2  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06caf2, "ax", @progbits
        .global Data_06caf2
Data_06caf2:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06caf2__L06cb02
Data_06caf2__L06cb02:
.L06cb02:
        bra.w   TaskHandler_06cd62              | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06cb06  @ $06CB06  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06cb06, "ax", @progbits
        .global TaskHandler_06cb06
TaskHandler_06cb06:
        clr.b   0x88(a6)                        | +000
        movea.l 0xc(a6),a0                      | +004
        move.w  0x8a(a0),0x8a(a6)               | +008
        jsr     TaskHandler_06d1dc(pc)          | +00e
        bcs.w   Data_06c6b6__L06c6ca            | +012
        move.b  d0,0x97(a6)                     | +016
        andi.w  #0x3,d0                         | +01a
        movea.l #0x6cb44,a0                     | +01e
        lsl.w   #0x2,d0                         | +024
        movea.l (a0,d0.w),a0                    | +026
        cmpa.l  #0xffffffff,a0                  | +02a
        beq.w   .L06cb40                        | +030
        jsr     0x28cd4.l                       | +034
.L06cb40:
        bra.w   Data_06cb44__L06cb54            | +03a

| ----------------------------------------------------------------------------
|  Data_06cb44  @ $06CB44  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cb44, "ax", @progbits
        .global Data_06cb44
Data_06cb44:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cb44__L06cb54
Data_06cb44__L06cb54:
.L06cb54:
        lea     .L06cb5a(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06cb5a:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06cb9e                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   TaskHandler_06cdba              | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06cb8c                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06cb8c:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06cb9e:
        move.b  d0,0x72(a6)                     | +05a
        jsr     TaskHandler_06d29c(pc)          | +05e
        jsr     TaskHandler_06d1dc(pc)          | +062
        bcs.w   Data_06c6b6__L06c6ca            | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   Data_06cbf4__L06cc04            | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     TaskHandler_06d1dc(pc)          | +080
        bcs.w   Data_06c6b6__L06c6ca            | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6cbf4,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06cbf0                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06cbf0:
        bra.w   Data_06cbf4__L06cc04            | +0ac

| ----------------------------------------------------------------------------
|  Data_06cbf4  @ $06CBF4  (84 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cbf4, "ax", @progbits
        .global Data_06cbf4
Data_06cbf4:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cbf4__L06cc04
Data_06cbf4__L06cc04:
.L06cc04:
        jsr     0x28d70.l                       | +010
        bcc.w   Data_06cc48__L06cc58            | +016
        movea.l 0xc(a6),a0                      | +01a
        move.w  0x8a(a0),0x8a(a6)               | +01e
        jsr     TaskHandler_06d1dc(pc)          | +024
        bcs.w   Data_06c6b6__L06c6ca            | +028
        move.b  d0,0x97(a6)                     | +02c
        andi.w  #0x3,d0                         | +030
        movea.l #0x6cc48,a0                     | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06cc44                        | +046
        jsr     0x28cd4.l                       | +04a
.L06cc44:
        bra.w   Data_06cc48__L06cc58            | +050

| ----------------------------------------------------------------------------
|  Data_06cc48  @ $06CC48  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cc48, "ax", @progbits
        .global Data_06cc48
Data_06cc48:
        .dc.w   0x002c                        | +000  (dato / opcode no decodificado)
        .dc.w   0xfada                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +004  (dato / opcode no decodificado)
        .dc.w   0xf886                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +008  (dato / opcode no decodificado)
        .dc.w   0xf9b0                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cc48__L06cc58
Data_06cc48__L06cc58:
.L06cc58:
        bra.w   TaskHandler_06cd62              | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06cc5c  @ $06CC5C  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06cc5c, "ax", @progbits
        .global TaskHandler_06cc5c
TaskHandler_06cc5c:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x8a(a0),0x8a(a6)               | +004
        jsr     TaskHandler_06d1dc(pc)          | +00a
        bcs.w   Data_06c6b6__L06c6ca            | +00e
        move.b  d0,0x97(a6)                     | +012
        andi.w  #0x3,d0                         | +016
        movea.l #0x6cc96,a0                     | +01a
        lsl.w   #0x2,d0                         | +020
        movea.l (a0,d0.w),a0                    | +022
        cmpa.l  #0xffffffff,a0                  | +026
        beq.w   .L06cc92                        | +02c
        jsr     0x28cd4.l                       | +030
.L06cc92:
        bra.w   Data_06cc96__L06cca6            | +036

| ----------------------------------------------------------------------------
|  Data_06cc96  @ $06CC96  (176 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cc96, "ax", @progbits
        .global Data_06cc96
Data_06cc96:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x35f8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2ccc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x3162                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cc96__L06cca6
Data_06cc96__L06cca6:
.L06cca6:
        lea     .L06ccac(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06ccac:
        movea.l 0xc(a6),a0                      | +016
        move.b  0x72(a0),d0                     | +01a
        cmp.b   0x72(a6),d0                     | +01e
        beq.w   .L06ccf0                        | +022
        cmpi.b  #0x7,0x72(a6)                   | +026
        beq.w   TaskHandler_06cdba              | +02c
        move.b  d0,0x72(a6)                     | +030
        cmpi.b  #0x9,d0                         | +034
        bcs.w   .L06ccde                        | +038
        nop                                     | +03c
        nop                                     | +03e
        cmpi.b  #0x9,d0                         | +040
        nop                                     | +044
        trap    #0xf                            | +046
.L06ccde:
        andi.w  #0xf,d0                         | +048
        lea     0x2cd606.l,a0                   | +04c
        lsl.w   #0x2,d0                         | +052
        movea.l (a0,d0.w),a1                    | +054
        jmp     (a1)                            | +058
.L06ccf0:
        move.b  d0,0x72(a6)                     | +05a
        jsr     TaskHandler_06d29c(pc)          | +05e
        jsr     TaskHandler_06d1dc(pc)          | +062
        bcs.w   Data_06c6b6__L06c6ca            | +066
        cmp.b   0x97(a6),d0                     | +06a
        beq.w   Data_06cd46__L06cd56            | +06e
        move.b  d0,0x97(a6)                     | +072
        movea.l 0xc(a6),a0                      | +076
        move.w  0x8a(a0),0x8a(a6)               | +07a
        jsr     TaskHandler_06d1dc(pc)          | +080
        bcs.w   Data_06c6b6__L06c6ca            | +084
        move.b  d0,0x97(a6)                     | +088
        andi.w  #0x3,d0                         | +08c
        movea.l #0x6cd46,a0                     | +090
        lsl.w   #0x2,d0                         | +096
        movea.l (a0,d0.w),a0                    | +098
        cmpa.l  #0xffffffff,a0                  | +09c
        beq.w   .L06cd42                        | +0a2
        jsr     0x28cd4.l                       | +0a6
.L06cd42:
        bra.w   Data_06cd46__L06cd56            | +0ac

| ----------------------------------------------------------------------------
|  Data_06cd46  @ $06CD46  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cd46, "ax", @progbits
        .global Data_06cd46
Data_06cd46:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x35f8                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x2ccc                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x3162                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cd46__L06cd56
Data_06cd46__L06cd56:
.L06cd56:
        subq.w  #0x1,0x38(a6)                   | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06cd62  @ $06CD62  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06cd62, "ax", @progbits
        .global TaskHandler_06cd62
TaskHandler_06cd62:
        movea.l 0xc(a6),a0                      | +000
        tst.b   0x94(a0)                        | +004
        beq.w   Data_06cda8__L06cdb8            | +008
        movea.l 0xc(a6),a0                      | +00c
        move.w  0x8a(a0),0x8a(a6)               | +010
        jsr     TaskHandler_06d1dc(pc)          | +016
        bcs.w   Data_06c6b6__L06c6ca            | +01a
        move.b  d0,0x97(a6)                     | +01e
        andi.w  #0x3,d0                         | +022
        movea.l #0x6cda8,a0                     | +026
        lsl.w   #0x2,d0                         | +02c
        movea.l (a0,d0.w),a0                    | +02e
        cmpa.l  #0xffffffff,a0                  | +032
        beq.w   .L06cda4                        | +038
        jsr     0x28cd4.l                       | +03c
.L06cda4:
        bra.w   Data_06cda8__L06cdb8            | +042

| ----------------------------------------------------------------------------
|  Data_06cda8  @ $06CDA8  (18 B)
| ----------------------------------------------------------------------------
        .section .text.Data_06cda8, "ax", @progbits
        .global Data_06cda8
Data_06cda8:
        .dc.w   0x002d                        | +000  (dato / opcode no decodificado)
        .dc.w   0x23aa                        | +002  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +004  (dato / opcode no decodificado)
        .dc.w   0x1166                        | +006  (dato / opcode no decodificado)
        .dc.w   0x002d                        | +008  (dato / opcode no decodificado)
        .dc.w   0x1a88                        | +00a  (dato / opcode no decodificado)
        .dc.w   0x002c                        | +00c  (dato / opcode no decodificado)
        .dc.w   0xd62a                        | +00e  (dato / opcode no decodificado)
        .global Data_06cda8__L06cdb8
Data_06cda8__L06cdb8:
.L06cdb8:
        rts                                     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06cdba  @ $06CDBA  (134 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06cdba, "ax", @progbits
        .global TaskHandler_06cdba
TaskHandler_06cdba:
        move.w  #0xffde,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x663,0x2a(a6)                 | +00e
        move.w  #0xff93,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x4000,d0                      | +020
        jsr     0x28134.l                       | +024
        andi.w  #0xffe3,0x38(a6)                | +02a
        ori.w   #0x0,0x38(a6)                   | +030
        lea     .L06cdf6(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L06cdf6:
        move.w  0x34(a6),0x5c(a6)               | +03c
        jsr     0x27bc8.l                       | +042
        bcc.w   .L06ce0c                        | +048
        lea     TaskHandler_06ce66(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06ce0c:
        move.w  0x5c(a6),0x34(a6)               | +052
        jsr     0x28d70.l                       | +058
        cmpi.w  #0xc0,0x24(a6)                  | +05e
        bgt.w   .L06ce28                        | +064
        lea     JmpToScheduler_06c54c(pc),a1    | +068
        move.l  a1,(a6)                         | +06c
.L06ce28:
        jsr     0x5e45a.l                       | +06e
        bcc.w   .L06ce38                        | +074
        lea     TaskHandler_06ce66(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L06ce38:
        jsr     TaskHandler_06d05c(pc)          | +07e
        bcc.w   SetHandlerRts_06ce46            | +082

| ----------------------------------------------------------------------------
|  TaskHandler_06ce48  @ $06CE48  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ce48, "ax", @progbits
        .global TaskHandler_06ce48
TaskHandler_06ce48:
        lea     .L06ce4e(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L06ce4e:
        jsr     0x2783a.l                       | +006
        jsr     0x28d70.l                       | +00c
        bcc.w   SetHandlerRts_06ce64            | +012

| ----------------------------------------------------------------------------
|  TaskHandler_06ce66  @ $06CE66  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ce66, "ax", @progbits
        .global TaskHandler_06ce66
TaskHandler_06ce66:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06ce8a  @ $06CE8A  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06ce8a, "ax", @progbits
        .global TaskHandler_06ce8a
TaskHandler_06ce8a:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x8,d1                         | +016
        jsr     0x236e.l                        | +01a
        cmpi.b  #0x1,0x97(a6)                   | +020
        bne.w   .L06cede                        | +026
        move.w  0x34(a6),d0                     | +02a
        andi.w  #0x7,d0                         | +02e
        bra.w   .L06cede                        | +032
        move.w  0x34(a6),d0                     | +036
        lsr.w   #0x3,d0                         | +03a
        andi.w  #0x1,d0                         | +03c
        eor.b   d0,0x3a(a6)                     | +040
        lea     0x2dd6d2.l,a0                   | +044
        jsr     0x28cd4.l                       | +04a
        bra.w   .L06cf02                        | +050
.L06cede:
        move.b  0x8e(a6),d0                     | +054
        move.b  0x8f(a6),d1                     | +058
        ext.w   d0                              | +05c
        ext.w   d1                              | +05e
        add.w   d0,0x22(a6)                     | +060
        add.w   d1,0x24(a6)                     | +064
        subq.w  #0x8,0x24(a6)                   | +068
        lea     0x2dd6d2.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
.L06cf02:
        bra.w   TaskHandler_06ce48              | +078

| ----------------------------------------------------------------------------
|  TaskHandler_06cf06  @ $06CF06  (320 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06cf06, "ax", @progbits
        .global TaskHandler_06cf06
TaskHandler_06cf06:
        lea     0x2cd10a.l,a1                   | +000
        btst    #0x0,0x3a(a6)                   | +006
        beq.w   .L06cf1c                        | +00c
        lea     0x2cd11a.l,a1                   | +010
.L06cf1c:
        move.b  0x97(a6),d0                     | +016
        andi.w  #0x3,d0                         | +01a
        lsl.w   #0x2,d0                         | +01e
        movea.l (a1,d0.w),a1                    | +020
        move.b  0x98(a6),d0                     | +024
        andi.w  #0x7,d0                         | +028
        lsl.w   #0x2,d0                         | +02c
        movea.l (a1,d0.w),a1                    | +02e
        move.w  0x34(a6),d0                     | +032
        andi.w  #0xf,d0                         | +036
        move.w  d0,d1                           | +03a
        add.w   d0,d0                           | +03c
        add.w   d1,d0                           | +03e
        add.w   d0,d0                           | +040
        ext.l   d0                              | +042
        adda.l  d0,a1                           | +044
        move.w  (a1),0x28(a6)                   | +046
        move.w  0x2(a1),0x2e(a6)                | +04a
        move.w  0x4(a1),0x2a(a6)                | +050
        andi.b  #0xfe,0x3a(a6)                  | +056
        move.w  #0x173,d1                       | +05c
        jsr     0x236e.l                        | +060
        bset    #0x4,0x6b(a6)                   | +066
        lea     0x2d3a8e.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        move.w  #0xd000,d0                      | +078
        jsr     0x28134.l                       | +07c
        andi.w  #0xffe3,0x38(a6)                | +082
        ori.w   #0x1c,0x38(a6)                  | +088
        clr.b   0x95(a6)                        | +08e
        move.w  0x34(a6),d0                     | +092
        andi.w  #0xf,d0                         | +096
        cmpi.w  #0x0,d0                         | +09a
        beq.w   .L06cfe6                        | +09e
        cmpi.w  #0x8,d0                         | +0a2
        bge.w   .L06cfe6                        | +0a6
        move.w  #0xd000,d0                      | +0aa
        jsr     0x28134.l                       | +0ae
        andi.w  #0xffe3,0x38(a6)                | +0b4
        ori.w   #0x8,0x38(a6)                   | +0ba
        move.b  #0x1,0x95(a6)                   | +0c0
        lea     .L06cfd2(pc),a1                 | +0c6
        move.l  a1,(a6)                         | +0ca
.L06cfd2:
        jsr     0x27bc8.l                       | +0cc
        bcc.w   .L06cfe2                        | +0d2
        lea     TaskHandler_06d04e(pc),a1       | +0d6
        move.l  a1,(a6)                         | +0da
.L06cfe2:
        bra.w   .L06cffc                        | +0dc
.L06cfe6:
        lea     .L06cfec(pc),a1                 | +0e0
        move.l  a1,(a6)                         | +0e4
.L06cfec:
        jsr     0x27d50.l                       | +0e6
        bcc.w   .L06cffc                        | +0ec
        lea     TaskHandler_06d04e(pc),a1       | +0f0
        move.l  a1,(a6)                         | +0f4
.L06cffc:
        move.w  0x28(a6),d0                     | +0f6
        move.w  0x2a(a6),d1                     | +0fa
        asr.w   #0x4,d0                         | +0fe
        asr.w   #0x4,d1                         | +100
        jsr     0x5e018.l                       | +102
        lsr.w   #0x3,d0                         | +108
        move.w  d0,0x34(a6)                     | +10a
        jsr     0x28d70.l                       | +10e
        jsr     0x283d8.l                       | +114
        btst    #0x1,0x13(a6)                   | +11a
        beq.w   .L06d030                        | +120
        lea     TaskHandler_06ce66(pc),a1       | +124
        move.l  a1,(a6)                         | +128
.L06d030:
        movea.l #0xffffffff,a0                  | +12a
        lea     0x2ccf42.l,a0                   | +130
        jsr     0x5dd56.l                       | +136
        bcc.w   SetHandlerRts_06d04c            | +13c

| ----------------------------------------------------------------------------
|  TaskHandler_06d04e  @ $06D04E  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d04e, "ax", @progbits
        .global TaskHandler_06d04e
TaskHandler_06d04e:
        move.w  #0x1025,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   TaskHandler_06ce66              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06d05c  @ $06D05C  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d05c, "ax", @progbits
        .global TaskHandler_06d05c
TaskHandler_06d05c:
        movea.l #0xffffffff,a0                  | +000
        lea     0x2ccf3a.l,a0                   | +006
        jsr     0x5dd5c.l                       | +00c
        bcs.w   SetXN_06d082                    | +012
        cmpi.w  #0xffdc,0x22(a6)                | +016
        blt.w   SetXN_06d082                    | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_06d088  @ $06D088  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d088, "ax", @progbits
        .global TaskHandler_06d088
TaskHandler_06d088:
        jsr     0x30704.l                       | +000
        beq.w   .L06d098                        | +006
        jsr     0x281b0.l                       | +00a
.L06d098:
        jsr     0x2788c.l                       | +010
        jsr     0x3076a.l                       | +016
        beq.w   ClearXN_06d0ae                  | +01c

| ----------------------------------------------------------------------------
|  Sub_0006D0B4  @ $06D0B4  (66 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006D0B4, "ax", @progbits
        .global Sub_0006D0B4
Sub_0006D0B4:
        movea.l 0x74(a6),a1                     | +000
        movea.l 0x78(a6),a2                     | +004
        move.w  0x22(a1),d0                     | +008
        sub.w   0x22(a2),d0                     | +00c
        move.w  0x24(a1),d1                     | +010
        sub.w   0x24(a2),d1                     | +014
        move.w  0x24(a1),d2                     | +018
        add.w   0x24(a2),d2                     | +01c
        lsr.w   #0x1,d2                         | +020
        move.w  d2,0x24(a6)                     | +022
        jsr     0x5e018.l                       | +026
        move.w  d0,d1                           | +02c
        addq.w  #0x8,d1                         | +02e
        lsr.w   #0x4,d1                         | +030
        btst    #0x0,d1                         | +032
        bne.w   .L06d0f4                        | +036
        addq.w  #0x4,d0                         | +03a
        andi.w  #0xf8,d0                        | +03c
.L06d0f4:
        rts                                     | +040

| ----------------------------------------------------------------------------
|  TaskHandler_06d0f6  @ $06D0F6  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d0f6, "ax", @progbits
        .global TaskHandler_06d0f6
TaskHandler_06d0f6:
        btst    #0x6,0x13(a6)                   | +000
        bne.w   ClearXN_06d136                  | +006
        jsr     0x2785c.l                       | +00a
        jsr     0x2a8c0.l                       | +010
        bcc.w   .L06d120                        | +016
        move.w  #0xffc0,0x2e(a6)                | +01a
        jsr     0x27a18.l                       | +020
        bra.w   ClearXN_06d136                  | +026
.L06d120:
        jsr     TaskHandler_06d088(pc)          | +02a
        jsr     Sub_0006D0B4(pc)                | +02e
        move.b  d0,0x89(a6)                     | +032
        jsr     TaskHandler_06d1a4(pc)          | +036

| ----------------------------------------------------------------------------
|  PcThunkTarget_06d13c  @ $06D13C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_06d13c, "ax", @progbits
        .global PcThunkTarget_06d13c
PcThunkTarget_06d13c:
        btst    #0x0,0x8d(a6)                   | +000
        beq.w   .L06d158                        | +006
        movea.l 0x74(a6),a1                     | +00a
        btst    #0x5,0x5a(a1)                   | +00e
        bne.w   SetXN_06d16c                    | +014
        bra.w   ClearXN_06d166                  | +018
.L06d158:
        movea.l 0x78(a6),a1                     | +01c
        btst    #0x5,0x5a(a1)                   | +020
        bne.w   SetXN_06d16c                    | +026

| ----------------------------------------------------------------------------
|  TaskHandler_06d172  @ $06D172  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d172, "ax", @progbits
        .global TaskHandler_06d172
TaskHandler_06d172:
        cmpi.w  #0x0,0x28(a6)                   | +000
        beq.w   ClearXN_06d198                  | +006
        bgt.w   .L06d18e                        | +00a
        btst    #0x1,0x69(a6)                   | +00e
        bne.w   SetXN_06d19e                    | +014
        bra.w   ClearXN_06d198                  | +018
.L06d18e:
        btst    #0x0,0x69(a6)                   | +01c
        bne.w   SetXN_06d19e                    | +022

| ----------------------------------------------------------------------------
|  TaskHandler_06d1a4  @ $06D1A4  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d1a4, "ax", @progbits
        .global TaskHandler_06d1a4
TaskHandler_06d1a4:
        addi.b  #0x20,d0                        | +000
        cmpi.b  #0x0,d0                         | +004
        bge.w   .L06d1b2                        | +008
        clr.b   d0                              | +00c
.L06d1b2:
        andi.w  #0xff,d0                        | +00e
        cmpi.w  #0x40,d0                        | +012
        blt.w   .L06d1c2                        | +016
        move.w  #0x40,d0                        | +01a
.L06d1c2:
        btst    #0x0,0x3a(a6)                   | +01e
        bne.w   .L06d1d4                        | +024
        move.w  #0x40,d2                        | +028
        sub.w   d0,d2                           | +02c
        move.w  d2,d0                           | +02e
.L06d1d4:
        lsr.w   #0x2,d0                         | +030

| ----------------------------------------------------------------------------
|  TaskHandler_06d1dc  @ $06D1DC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d1dc, "ax", @progbits
        .global TaskHandler_06d1dc
TaskHandler_06d1dc:
        move.w  0x8a(a6),d0                     | +000
        andi.w  #0x7,d0                         | +004
        bne.w   TaskHandler_06d1fc              | +008
        move.w  0x8a(a6),d0                     | +00c
        andi.w  #0x1f,d0                        | +010
        lsr.w   #0x3,d0                         | +014
        andi.w  #0x3,d0                         | +016

| ----------------------------------------------------------------------------
|  TaskHandler_06d1fc  @ $06D1FC  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d1fc, "ax", @progbits
        .global TaskHandler_06d1fc
TaskHandler_06d1fc:
        move.w  #0x3,d0                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06d206  @ $06D206  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d206, "ax", @progbits
        .global TaskHandler_06d206
TaskHandler_06d206:
        move.b  0x8d(a6),d0                     | +000
        move.b  0x3a(a6),d1                     | +004
        andi.b  #0x1,d0                         | +008
        andi.b  #0x1,d1                         | +00c
        cmp.b   d0,d1                           | +010
        bne.w   SetXN_06d222                    | +012

| ----------------------------------------------------------------------------
|  TaskHandler_06d228  @ $06D228  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d228, "ax", @progbits
        .global TaskHandler_06d228
TaskHandler_06d228:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   ClearXN_06d296                  | +00a
        jsr     0x5e0d4.l                       | +00e
        move.w  0x24(a0),d0                     | +014
        sub.w   0x24(a6),d0                     | +018
        cmpi.w  #0x0,d0                         | +01c
        bgt.w   .L06d24e                        | +020
        neg.w   d0                              | +024
.L06d24e:
        cmpi.w  #0x80,d0                        | +026
        ble.w   .L06d25a                        | +02a
        move.w  #0x80,d0                        | +02e
.L06d25a:
        lsr.w   #0x5,d0                         | +032
        lea     0x2ccfc6.l,a1                   | +034
        move.b  0x98(a6),d1                     | +03a
        andi.w  #0x3,d1                         | +03e
        lsl.w   #0x2,d1                         | +042
        movea.l (a1,d1.w),a2                    | +044
        move.b  (a2,d0.w),d1                    | +048
        andi.w  #0xff,d1                        | +04c
        move.w  d1,d2                           | +050
        neg.w   d1                              | +052
        move.w  0x22(a0),d0                     | +054
        sub.w   0x22(a6),d0                     | +058
        cmp.w   d1,d0                           | +05c
        blt.w   ClearXN_06d296                  | +05e
        cmp.w   d2,d0                           | +062
        bgt.w   ClearXN_06d296                  | +064

| ----------------------------------------------------------------------------
|  TaskHandler_06d29c  @ $06D29C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d29c, "ax", @progbits
        .global TaskHandler_06d29c
TaskHandler_06d29c:
        jsr     0x5e506.l                       | +000
        movea.l 0xc(a6),a0                      | +006
        move.w  0x8a(a0),0x8a(a6)               | +00a
        move.b  0x8e(a0),d0                     | +010
        ext.w   d0                              | +014
        btst    #0x0,0x3a(a6)                   | +016
        beq.w   .L06d2be                        | +01c
        neg.w   d0                              | +020
.L06d2be:
        add.w   d0,0x22(a6)                     | +022
        move.b  0x8f(a0),d0                     | +026
        ext.w   d0                              | +02a
        add.w   d0,0x24(a6)                     | +02c
        move.w  0x34(a6),d0                     | +030
        andi.w  #0xf,d0                         | +034
        beq.w   .L06d2e0                        | +038
        cmpi.w  #0x8,d0                         | +03c
        bcs.w   .L06d2e4                        | +040
.L06d2e0:
        subq.w  #0x1,0x38(a6)                   | +044
.L06d2e4:
        movea.l 0xc(a6),a0                      | +048
        btst    #0x7,0x5a(a0)                   | +04c
        beq.w   .L06d2f8                        | +052
        bset    #0x0,0x5a(a6)                   | +056
.L06d2f8:
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  TaskHandler_06d2fa  @ $06D2FA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d2fa, "ax", @progbits
        .global TaskHandler_06d2fa
TaskHandler_06d2fa:
        tst.b   0x9f(a6)                        | +000
        beq.w   .L06d33e                        | +004
        movea.l 0x90(a6),a0                     | +008
        move.w  0x22(a6),d0                     | +00c
        move.w  0x24(a6),d1                     | +010
        move.w  0x22(a0),d2                     | +014
        move.w  0x24(a0),d3                     | +018
        jsr     0x5e23a.l                       | +01c
        clr.b   d1                              | +022
        cmpi.w  #0x70,d0                        | +024
        blt.w   .L06d336                        | +028
        move.b  #0x1,d1                         | +02c
        cmpi.w  #0xb0,d0                        | +030
        blt.w   .L06d336                        | +034
        move.b  #0x2,d1                         | +038
.L06d336:
        andi.b  #0x3,d1                         | +03c
        move.b  d1,0x98(a6)                     | +040
.L06d33e:
        rts                                     | +044

| ----------------------------------------------------------------------------
|  TaskHandler_06d340  @ $06D340  (156 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d340, "ax", @progbits
        .global TaskHandler_06d340
TaskHandler_06d340:
        movea.l 0x90(a6),a0                     | +000
        lea     0x2cd4ea.l,a1                   | +004
        btst    #0x0,0x3a(a6)                   | +00a
        beq.w   .L06d35a                        | +010
        lea     0x2cd50a.l,a1                   | +014
.L06d35a:
        move.b  0x97(a6),d1                     | +01a
        andi.w  #0x3,d1                         | +01e
        move.w  0x24(a0),d0                     | +022
        cmp.w   0x24(a6),d0                     | +026
        bgt.w   .L06d370                        | +02a
        addq.w  #0x4,d1                         | +02e
.L06d370:
        lsl.w   #0x2,d1                         | +030
        movea.l (a1,d1.w),a1                    | +032
        move.w  0x22(a0),d0                     | +036
        sub.w   0x22(a6),d0                     | +03a
        cmpi.w  #0xff00,d0                      | +03e
        bgt.w   .L06d38a                        | +042
        move.w  #0xff00,d0                      | +046
.L06d38a:
        cmpi.w  #0x100,d0                       | +04a
        blt.w   .L06d396                        | +04e
        move.w  #0x100,d0                       | +052
.L06d396:
        addi.w  #0x100,d0                       | +056
        lsr.w   #0x4,d0                         | +05a
        move.b  (a1,d0.w),d0                    | +05c
        btst    #0x0,0x3a(a6)                   | +060
        beq.w   .L06d3b4                        | +066
        lea     0x2cd5f6.l,a1                   | +06a
        move.b  (a1,d0.w),d0                    | +070
.L06d3b4:
        jsr     TaskHandler_06d3e8(pc)          | +074
        andi.w  #0xf,d0                         | +078
        cmp.w   0x34(a6),d0                     | +07c
        beq.w   SetXN_06d3e2                    | +080
        sub.w   0x34(a6),d0                     | +084
        move.w  #0x1,d1                         | +088
        cmpi.w  #0x0,d0                         | +08c
        bgt.w   .L06d3d8                        | +090
        move.w  #0xffff,d1                      | +094
.L06d3d8:
        add.w   d1,0x34(a6)                     | +098

| ----------------------------------------------------------------------------
|  TaskHandler_06d3e8  @ $06D3E8  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d3e8, "ax", @progbits
        .global TaskHandler_06d3e8
TaskHandler_06d3e8:
        cmpi.b  #0x1,0x97(a6)                   | +000
        bne.w   ClearXN_06d440                  | +006
        move.b  d0,0x5c(a6)                     | +00a
        movea.l 0x90(a6),a0                     | +00e
        move.w  0x24(a0),d0                     | +012
        sub.w   0x24(a6),d0                     | +016
        cmpi.w  #0xfff8,d0                      | +01a
        blt.w   TaskHandler_06d43c              | +01e
        cmpi.w  #0x28,d0                        | +022
        bgt.w   TaskHandler_06d43c              | +026
        move.w  0x22(a0),d0                     | +02a
        sub.w   0x22(a6),d0                     | +02e
        btst    #0x0,0x3a(a6)                   | +032
        beq.w   .L06d426                        | +038
        neg.w   d0                              | +03c
.L06d426:
        clr.b   d1                              | +03e
        cmpi.w  #0x0,d0                         | +040
        bgt.w   .L06d434                        | +044
        move.b  #0x8,d1                         | +048
.L06d434:
        move.b  d1,d0                           | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_06d43c  @ $06D43C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d43c, "ax", @progbits
        .global TaskHandler_06d43c
TaskHandler_06d43c:
        move.b  0x5c(a6),d0                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06d446  @ $06D446  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d446, "ax", @progbits
        .global TaskHandler_06d446
TaskHandler_06d446:
        bset    #0x5,0x13(a6)                   | +000
        bset    #0x4,0x13(a6)                   | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06d454  @ $06D454  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d454, "ax", @progbits
        .global TaskHandler_06d454
TaskHandler_06d454:
        jsr     0x5e9b6.l                       | +000
        bset    #0x4,0x13(a6)                   | +006
        btst    #0x0,d0                         | +00c
        beq.w   .L06d46e                        | +010
        bclr    #0x4,0x13(a6)                   | +014
.L06d46e:
        bset    #0x5,0x13(a6)                   | +01a
        btst    #0x1,d0                         | +020
        beq.w   .L06d482                        | +024
        bclr    #0x5,0x13(a6)                   | +028
.L06d482:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_06d484  @ $06D484  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d484, "ax", @progbits
        .global TaskHandler_06d484
TaskHandler_06d484:
        lea     0x2ccfea.l,a1                   | +000
        lea     0x2ccffa.l,a2                   | +006
        lea     0x2cd00a.l,a3                   | +00c
        tst.b   0x9e(a6)                        | +012
        beq.w   .L06d4a4                        | +016
        lea     0x2cd01a.l,a3                   | +01a
.L06d4a4:
        move.b  0x97(a6),d0                     | +020
        andi.w  #0x3,d0                         | +024
        lsl.w   #0x2,d0                         | +028
        move.l  (a1,d0.w),0x48(a6)              | +02a
        move.l  (a2,d0.w),0x60(a6)              | +030
        move.l  (a3,d0.w),0x4c(a6)              | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_06d4c2  @ $06D4C2  (178 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d4c2, "ax", @progbits
        .global TaskHandler_06d4c2
TaskHandler_06d4c2:
        move.w  #0x1064,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x34(a6),d0                     | +00a
        btst    #0x0,0x3a(a6)                   | +00e
        beq.w   .L06d4e4                        | +014
        lea     0x2cd5f6.l,a1                   | +018
        move.b  (a1,d0.w),d0                    | +01e
.L06d4e4:
        andi.w  #0xf,d0                         | +022
        move.w  d0,0x5c(a6)                     | +026
        lea     TaskHandler_06ce8a(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     0x2cd03a.l,a1                   | +03a
        move.b  0x97(a6),d0                     | +040
        andi.w  #0x3,d0                         | +044
        lsl.w   #0x2,d0                         | +048
        movea.l (a1,d0.w),a1                    | +04a
        move.w  0x34(a6),d0                     | +04e
        move.w  d0,0x34(a0)                     | +052
        lsl.w   #0x2,d0                         | +056
        move.w  (a1,d0.w),d1                    | +058
        move.w  0x2(a1,d0.w),d2                 | +05c
        btst    #0x0,0x3a(a6)                   | +060
        beq.w   .L06d52e                        | +066
        neg.w   d1                              | +06a
.L06d52e:
        move.w  d1,d3                           | +06c
        move.w  d2,d4                           | +06e
        move.b  d3,0x8e(a0)                     | +070
        move.b  d4,0x8f(a0)                     | +074
        move.b  0x97(a6),0x97(a0)               | +078
        movem.w d1-d2,-(a7)                     | +07e
        lea     TaskHandler_06cf06(pc),a1       | +082
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        movem.w (a7)+,d1-d2                     | +092
        move.w  0x5c(a6),0x34(a0)               | +096
        add.w   d1,0x22(a0)                     | +09c
        add.w   d2,0x24(a0)                     | +0a0
        move.b  0x98(a6),0x98(a0)               | +0a4
        move.b  0x97(a6),0x97(a0)               | +0aa
        rts                                     | +0b0

| ----------------------------------------------------------------------------
|  TaskHandler_06d574  @ $06D574  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d574, "ax", @progbits
        .global TaskHandler_06d574
TaskHandler_06d574:
        lea     0x2ccf6e.l,a1                   | +000
        move.b  0x96(a6),d0                     | +006
        andi.w  #0x3,d0                         | +00a
        lsl.w   #0x1,d0                         | +00e
        move.w  (a1,d0.w),d1                    | +010
        jsr     0x236e.l                        | +014
        move.w  #0x7,0x1c(a6)                   | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_06d59c  @ $06D59C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d59c, "ax", @progbits
        .global TaskHandler_06d59c
TaskHandler_06d59c:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf4a.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x2ccf5c.l,a1                   | +01c
        jsr     0x77c7e.l                       | +022
        move.w  #0x4000,0x38(a0)                | +028
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_06d5cc  @ $06D5CC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d5cc, "ax", @progbits
        .global TaskHandler_06d5cc
TaskHandler_06d5cc:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf5c.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06d602  @ $06D602  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d602, "ax", @progbits
        .global TaskHandler_06d602
TaskHandler_06d602:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2ccf5c.l,a1                   | +00a
        jsr     0x77c7e.l                       | +010
        move.w  #0x4000,0x38(a0)                | +016
        lea     0x77fd6.l,a1                    | +01c
        jsr     0x4ae.l                         | +022
        jsr     0x5dd02.l                       | +028
        move.w  #0x4000,0x38(a0)                | +02e
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_06d638  @ $06D638  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d638, "ax", @progbits
        .global TaskHandler_06d638
TaskHandler_06d638:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06d64e                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06d654  @ $06D654  (154 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d654, "ax", @progbits
        .global TaskHandler_06d654
TaskHandler_06d654:
        clr.b   0x7e(a6)                        | +000
        clr.b   0x7f(a6)                        | +004
        bra.w   .L06d67a                        | +008
        move.b  #0x1,0x7e(a6)                   | +00c
        clr.b   0x7f(a6)                        | +012
        bra.w   .L06d67a                        | +016
        move.b  #0x1,0x7e(a6)                   | +01a
        move.b  #0x1,0x7f(a6)                   | +020
.L06d67a:
        jsr     Sub_0006D6EE(pc)                | +026
        clr.w   0x70(a6)                        | +02a
        bra.w   Sub_0006D6EE__L06d7a2           | +02e
        clr.b   0x7e(a6)                        | +032
        clr.b   0x7f(a6)                        | +036
        bra.w   .L06d6ac                        | +03a
        move.b  #0x1,0x7e(a6)                   | +03e
        clr.b   0x7f(a6)                        | +044
        bra.w   .L06d6ac                        | +048
        move.b  #0x1,0x7e(a6)                   | +04c
        move.b  #0x1,0x7f(a6)                   | +052
.L06d6ac:
        jsr     Sub_0006D6EE(pc)                | +058
        move.w  #0x1,0x70(a6)                   | +05c
        bra.w   TaskHandler_06d808__L06d83a     | +062
        clr.b   0x7e(a6)                        | +066
        clr.b   0x7f(a6)                        | +06a
        bra.w   .L06d6e0                        | +06e
        move.b  #0x1,0x7e(a6)                   | +072
        clr.b   0x7f(a6)                        | +078
        bra.w   .L06d6e0                        | +07c
        move.b  #0x1,0x7e(a6)                   | +080
        move.b  #0x1,0x7f(a6)                   | +086
.L06d6e0:
        jsr     Sub_0006D6EE(pc)                | +08c
        move.w  #0x2,0x70(a6)                   | +090
        bra.w   TaskHandler_06d808__L06d83a     | +096

| ----------------------------------------------------------------------------
|  Sub_0006D6EE  @ $06D6EE  (282 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0006D6EE, "ax", @progbits
        .global Sub_0006D6EE
Sub_0006D6EE:
        jsr     0x5e7c0.l                       | +000
        jsr     0x267e2.l                       | +006
        jsr     Sub_0006E31E(pc)                | +00c  -> $06E31E (hueco futuro, defsym forward)
        move.w  #0x8,0x1c(a6)                   | +010
        jsr     0x138fe.l                       | +016
        lea     0x2b8d6c.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        move.w  0x66(a6),0x7a(a6)               | +02c
        move.l  #0x2d3bae,0x48(a6)              | +032
        move.l  #0x2d4056,0x60(a6)              | +03a
        move.b  #0x0,0x20(a6)                   | +042
        clr.w   0x76(a6)                        | +048
        move.b  0x9a(a6),d0                     | +04c
        tst.b   d0                              | +050
        bne.w   .L06d748                        | +052
        move.b  #0x15,d0                        | +056
.L06d748:
        andi.w  #0xff,d0                        | +05a
        lsl.w   #0x4,d0                         | +05e
        move.w  d0,0x86(a6)                     | +060
        move.w  #0x4000,d0                      | +064
        jsr     0x28134.l                       | +068
        andi.w  #0xffe3,0x38(a6)                | +06e
        ori.w   #0x14,0x38(a6)                  | +074
        tst.b   0x9d(a6)                        | +07a
        beq.w   .L06d786                        | +07e
        move.w  #0x8000,d0                      | +082
        jsr     0x28134.l                       | +086
        andi.w  #0xffe3,0x38(a6)                | +08c
        ori.w   #0x14,0x38(a6)                  | +092
.L06d786:
        jsr     Sub_0006E484(pc)                | +098  -> $06E484 (hueco futuro, defsym forward)
        lea     0x723d2.l,a1                    | +09c
        jsr     0x4ae.l                         | +0a2
        jsr     0x5dd02.l                       | +0a8
        clr.w   0x98(a0)                        | +0ae
        rts                                     | +0b2
        .global Sub_0006D6EE__L06d7a2
Sub_0006D6EE__L06d7a2:
.L06d7a2:
        clr.w   0x76(a6)                        | +0b4
        lea     0x2d428c.l,a0                   | +0b8
        jsr     0x28cd4.l                       | +0be
        lea     .L06d7b8(pc),a1                 | +0c4
        move.l  a1,(a6)                         | +0c8
.L06d7b8:
        jsr     0x28998.l                       | +0ca
        jsr     0x2783a.l                       | +0d0
        jsr     0x28d70.l                       | +0d6
        move.w  0x22(a6),d0                     | +0dc
        cmp.w   0x86(a6),d0                     | +0e0
        bge.w   .L06d7f6                        | +0e4
        jsr     0x5e0d4.l                       | +0e8
        bcs.w   .L06d7f6                        | +0ee
        move.w  0x22(a6),d0                     | +0f2
        sub.w   0x22(a0),d0                     | +0f6
        cmpi.w  #0x60,d0                        | +0fa
        bgt.w   .L06d7f6                        | +0fe
        lea     TaskHandler_06d808(pc),a1       | +102
        move.l  a1,(a6)                         | +106
.L06d7f6:
        tst.w   0x76(a6)                        | +108
        beq.w   .L06d804                        | +10c
        lea     TaskHandler_06d808(pc),a1       | +110
        move.l  a1,(a6)                         | +114
.L06d804:
        bra.w   TaskHandler_06d926__L06d986     | +116

| ----------------------------------------------------------------------------
|  TaskHandler_06d808  @ $06D808  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d808, "ax", @progbits
        .global TaskHandler_06d808
TaskHandler_06d808:
        lea     0x2d42b0.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L06d81a(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L06d81a:
        jsr     0x28998.l                       | +012
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L06d836                        | +024
        lea     TaskHandler_06d900(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L06d836:
        bra.w   TaskHandler_06d926__L06d986     | +02e
        .global TaskHandler_06d808__L06d83a
TaskHandler_06d808__L06d83a:
.L06d83a:
        lea     0x2d4298.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
        lea     .L06d84c(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L06d84c:
        jsr     0x28998.l                       | +044
        jsr     0x2783a.l                       | +04a
        jsr     0x28d70.l                       | +050
        move.w  0x22(a6),d0                     | +056
        cmp.w   0x86(a6),d0                     | +05a
        bge.w   .L06d880                        | +05e
        lea     0x2d4036.l,a0                   | +062
        jsr     0x5e086.l                       | +068
        bcs.w   .L06d880                        | +06e
        lea     TaskHandler_06d900(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L06d880:
        bra.w   TaskHandler_06d926__L06d986     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_06d884  @ $06D884  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d884, "ax", @progbits
        .global TaskHandler_06d884
TaskHandler_06d884:
        lea     0x2b8dee.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        lea     0x2d42a4.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L06d8a6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06d8a6:
        jsr     0x28998.l                       | +022
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        subq.w  #0x1,0x72(a6)                   | +034
        cmpi.w  #0x0,0x72(a6)                   | +038
        bgt.w   .L06d8fc                        | +03e
        cmpi.w  #0x0,0x70(a6)                   | +042
        bne.w   .L06d8d6                        | +048
        lea     TaskHandler_06d916(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06d8d6:
        cmpi.w  #0x2,0x70(a6)                   | +052
        bne.w   .L06d8e6                        | +058
        lea     TaskHandler_06d916(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
.L06d8e6:
        lea     0x2d4036.l,a0                   | +062
        jsr     0x5e086.l                       | +068
        bcs.w   .L06d8fc                        | +06e
        lea     TaskHandler_06d916(pc),a1       | +072
        move.l  a1,(a6)                         | +076
.L06d8fc:
        bra.w   TaskHandler_06d926__L06d986     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_06d900  @ $06D900  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d900, "ax", @progbits
        .global TaskHandler_06d900
TaskHandler_06d900:
        move.w  #0x1,0x78(a6)                   | +000
        lea     0x2d42e8.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        bra.w   TaskHandler_06d926__L06d932     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_06d916  @ $06D916  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d916, "ax", @progbits
        .global TaskHandler_06d916
TaskHandler_06d916:
        lea     0x2b8ef2.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x78(a6)                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06d926  @ $06D926  (190 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d926, "ax", @progbits
        .global TaskHandler_06d926
TaskHandler_06d926:
        lea     0x2d431e.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global TaskHandler_06d926__L06d932
TaskHandler_06d926__L06d932:
.L06d932:
        lea     0x2b8e70.l,a0                   | +00c
        jsr     0x799de.l                       | +012
        move.w  d0,0x72(a6)                     | +018
        lea     .L06d948(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L06d948:
        jsr     0x28998.l                       | +022
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L06d986                        | +034
        subq.w  #0x1,0x72(a6)                   | +038
        cmpi.w  #0x0,0x72(a6)                   | +03c
        bgt.w   .L06d986                        | +042
        lea     TaskHandler_06d926(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
        subq.w  #0x1,0x78(a6)                   | +04c
        cmpi.w  #0x0,0x78(a6)                   | +050
        bgt.w   .L06d986                        | +056
        lea     TaskHandler_06d884(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
        .global TaskHandler_06d926__L06d986
TaskHandler_06d926__L06d986:
.L06d986:
        jsr     Sub_0006E394(pc)                | +060  -> $06E394 (hueco futuro, defsym forward)
        jsr     0x2870a.l                       | +064
        bcc.w   .L06d9a6                        | +06a
        bclr    #0x3,0x13(a6)                   | +06e
        lea     0x5e766.l,a0                    | +074
        jsr     0x5e770.l                       | +07a
.L06d9a6:
        jsr     0x28758.l                       | +080
        bcc.w   .L06d9b6                        | +086
        lea     TaskHandler_06d9ec(pc),a1       | +08a
        move.l  a1,(a6)                         | +08e
.L06d9b6:
        tst.b   0x99(a6)                        | +090
        beq.w   .L06d9ce                        | +094
        jsr     0x27eba.l                       | +098
        bcc.w   .L06d9ce                        | +09e
        lea     TaskHandler_06da4c(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6
.L06d9ce:
        movea.l #0xffffffff,a0                  | +0a8
        lea     0x2d402e.l,a0                   | +0ae
        jsr     0x5dd5c.l                       | +0b4
        bcc.w   SetHandlerRts_06d9ea            | +0ba

| ----------------------------------------------------------------------------
|  TaskHandler_06d9ec  @ $06D9EC  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06d9ec, "ax", @progbits
        .global TaskHandler_06d9ec
TaskHandler_06d9ec:
        bclr    #0x1,0x12(a6)                   | +000
        jsr     0x267e2.l                       | +006
        lea     0x2d4394.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L06da0a(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L06da0a:
        jsr     0x2783a.l                       | +01e
        jsr     0x28d70.l                       | +024
        tst.b   0x99(a6)                        | +02a
        beq.w   .L06da2e                        | +02e
        jsr     0x27eba.l                       | +032
        bcc.w   .L06da2e                        | +038
        lea     TaskHandler_06da4c(pc),a1       | +03c
        move.l  a1,(a6)                         | +040
.L06da2e:
        movea.l #0xffffffff,a0                  | +042
        lea     0x2d402e.l,a0                   | +048
        jsr     0x5dd5c.l                       | +04e
        bcc.w   SetHandlerRts_06da4a            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06da4c  @ $06DA4C  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06da4c, "ax", @progbits
        .global TaskHandler_06da4c
TaskHandler_06da4c:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  #0x8000,0x38(a0)                | +01c
        jsr     Entity_SpawnLoop16_06E412(pc)   | +022
        jmp     0x518.l                         | +026

| ----------------------------------------------------------------------------
|  TaskHandler_06da78  @ $06DA78  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06da78, "ax", @progbits
        .global TaskHandler_06da78
TaskHandler_06da78:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_06da9e  @ $06DA9E  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06da9e, "ax", @progbits
        .global TaskHandler_06da9e
TaskHandler_06da9e:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06dac2  @ $06DAC2  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dac2, "ax", @progbits
        .global TaskHandler_06dac2
TaskHandler_06dac2:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x1c,0x38(a6)                  | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77fd6.l                       | +024

| ----------------------------------------------------------------------------
|  TaskHandler_06daec  @ $06DAEC  (224 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06daec, "ax", @progbits
        .global TaskHandler_06daec
TaskHandler_06daec:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x0,0x38(a6)                   | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77fd6.l                       | +024
        .global TaskHandler_06daec__L06db16
TaskHandler_06daec__L06db16:
.L06db16:
        tst.b   0x89(a6)                        | +02a
        bne.b   TaskHandler_06daec              | +02e
        jsr     0x13600.l                       | +030
        move.w  #0x4000,d0                      | +036
        jsr     0x28134.l                       | +03a
        andi.w  #0xffe3,0x38(a6)                | +040
        ori.w   #0x1c,0x38(a6)                  | +046
        move.w  #0x1021,d0                      | +04c
        jsr     0x2352.l                        | +050
        lea     0x2d40b4.l,a1                   | +056
        jsr     0x77c7e.l                       | +05c
        move.w  #0xc000,0x38(a0)                | +062
        move.w  #0x4,d1                         | +068
        jsr     0x236e.l                        | +06c
        move.w  #0x2,0x72(a6)                   | +072
        move.l  #0x2d3f8a,0x4c(a6)              | +078
        jsr     0x283ca.l                       | +080
        lea     0x2dd37e.l,a0                   | +086
        jsr     0x28cd4.l                       | +08c
        lea     .L06db84(pc),a1                 | +092
        move.l  a1,(a6)                         | +096
.L06db84:
        jsr     0x2783a.l                       | +098
        jsr     0x28d70.l                       | +09e
        bcc.w   .L06db9a                        | +0a4
        lea     Jsr5B6ThenJmpScheduler_06da90(pc),a1 | +0a8
        move.l  a1,(a6)                         | +0ac
.L06db9a:
        jsr     0x283d8.l                       | +0ae
        btst    #0x1,0x13(a6)                   | +0b4
        subq.w  #0x1,0x72(a6)                   | +0ba
        cmpi.w  #0x0,0x72(a6)                   | +0be
        bgt.w   .L06dbbc                        | +0c4
        move.l  #0xffffffff,0x4c(a6)            | +0c8
.L06dbbc:
        movea.l #0xffffffff,a0                  | +0d0
        jsr     0x5dd56.l                       | +0d6
        bcc.w   SetHandlerRts_06dbd2            | +0dc

| ----------------------------------------------------------------------------
|  TaskHandler_06dbd4  @ $06DBD4  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dbd4, "ax", @progbits
        .global TaskHandler_06dbd4
TaskHandler_06dbd4:
        move.b  #0x1,0x89(a6)                   | +000
        bra.w   .L06dbe2                        | +006
        jsr     Sub_0006E176(pc)                | +00a  -> $06E176 (hueco futuro, defsym forward)
.L06dbe2:
        bset    #0x4,0x6b(a6)                   | +00e
        jsr     Sub_0006E356(pc)                | +014  -> $06E356 (hueco futuro, defsym forward)
        move.w  #0xd000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x14,0x38(a6)                  | +028
        lea     0x2d45fc.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        lea     .L06dc14(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L06dc14:
        jsr     0x27d50.l                       | +040
        bcc.w   .L06dc24                        | +046
        lea     TaskHandler_06daec__L06db16(pc),a1 | +04a
        move.l  a1,(a6)                         | +04e
.L06dc24:
        jsr     0x28d70.l                       | +050
        jsr     0x283d8.l                       | +056
        btst    #0x1,0x13(a6)                   | +05c
        beq.w   .L06dc40                        | +062
        lea     TaskHandler_06dac2(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L06dc40:
        movea.l #0xffffffff,a0                  | +06c
        lea     0x2d404e.l,a0                   | +072
        jsr     0x5dd56.l                       | +078
        bcc.w   SetHandlerRts_06dc5c            | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_06dc5e  @ $06DC5E  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dc5e, "ax", @progbits
        .global TaskHandler_06dc5e
TaskHandler_06dc5e:
        jsr     Sub_0006E356(pc)                | +000  -> $06E356 (hueco futuro, defsym forward)
        lea     0x2d4628.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L06dc74(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L06dc74:
        jsr     Sub_0006E34A(pc)                | +016  -> $06E34A (hueco futuro, defsym forward)
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   SetHandlerRts_06dc8e            | +026

| ----------------------------------------------------------------------------
|  TaskHandler_06dc90  @ $06DC90  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dc90, "ax", @progbits
        .global TaskHandler_06dc90
TaskHandler_06dc90:
        eori.b  #0x1,0x3a(a6)                   | +000
        move.w  #0x8,d1                         | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x0,0x38(a6)                   | +020
        lea     0x2d455a.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L06dcc8(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L06dcc8:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   SetHandlerRts_06dcde            | +044

| ----------------------------------------------------------------------------
|  TaskHandler_06dce0  @ $06DCE0  (124 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06dce0, "ax", @progbits
        .global TaskHandler_06dce0
TaskHandler_06dce0:
        cmpi.w  #0x0,0x36(a6)                   | +000
        bgt.w   .L06dcf0                        | +006
        move.w  #0x200,0x36(a6)                 | +00a
.L06dcf0:
        move.w  #0x1,0x80(a6)                   | +010
        move.w  #0x1,0x82(a6)                   | +016
        move.w  #0x8000,d0                      | +01c
        jsr     0x28134.l                       | +020
        andi.w  #0xffe3,0x38(a6)                | +026
        ori.w   #0x14,0x38(a6)                  | +02c
        bra.w   Template_06DD5C__L06dde4        | +032
        cmpi.w  #0x0,0x36(a6)                   | +036
        bgt.w   .L06dd26                        | +03c
        move.w  #0x200,0x36(a6)                 | +040
.L06dd26:
        move.w  #0x8000,d0                      | +046
        jsr     0x28134.l                       | +04a
        andi.w  #0xffe3,0x38(a6)                | +050
        ori.w   #0x14,0x38(a6)                  | +056
        move.w  #0x1,0x80(a6)                   | +05c
        move.w  #0x1,0x82(a6)                   | +062
        jsr     0x5e9b6.l                       | +068
        move.w  d0,d3                           | +06e
        andi.w  #0x3e,d0                        | +070
        addi.w  #0x50,d0                        | +074
        bra.w   Template_06DD5C__L06ddf4        | +078

| ----------------------------------------------------------------------------
|  Template_06DD5C  @ $06DD5C  (418 B)
| ----------------------------------------------------------------------------
        .section .text.Template_06DD5C, "ax", @progbits
        .global Template_06DD5C
Template_06DD5C:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0xf,d0                         | +006
        movea.l #0x2d4106,a0                    | +00a
        lsl.w   #0x2,d0                         | +010
        movea.l (a0,d0.w),a0                    | +012
        cmpa.l  #0xffffffff,a0                  | +016
        beq.w   .L06dd82                        | +01c
        jsr     0x28cd4.l                       | +020
.L06dd82:
        bra.w   .L06ddac                        | +026
        jsr     0x5e9b6.l                       | +02a
        andi.w  #0xf,d0                         | +030
        movea.l #0x2d4146,a0                    | +034
        lsl.w   #0x2,d0                         | +03a
        movea.l (a0,d0.w),a0                    | +03c
        cmpa.l  #0xffffffff,a0                  | +040
        beq.w   .L06ddac                        | +046
        jsr     0x28cd4.l                       | +04a
.L06ddac:
        move.w  #0x4000,d0                      | +050
        jsr     0x28134.l                       | +054
        andi.w  #0xffe3,0x38(a6)                | +05a
        ori.w   #0x0,0x38(a6)                   | +060
        jsr     0x5e9b6.l                       | +066
        andi.w  #0x700,d0                       | +06c
        addi.w  #0x200,d0                       | +070
        move.w  d0,0x36(a6)                     | +074
        move.w  #0x3,0x80(a6)                   | +078
        move.w  #0x3,0x82(a6)                   | +07e
        jsr     Sub_0006E31E(pc)                | +084  -> $06E31E (hueco futuro, defsym forward)
        .global Template_06DD5C__L06dde4
Template_06DD5C__L06dde4:
.L06dde4:
        jsr     0x5e9b6.l                       | +088
        move.w  d0,d3                           | +08e
        andi.w  #0x3e,d0                        | +090
        addi.w  #0x20,d0                        | +094
        .global Template_06DD5C__L06ddf4
Template_06DD5C__L06ddf4:
.L06ddf4:
        add.w   d0,d0                           | +098
        lea     0x2c072c.l,a1                   | +09a
        move.w  (a1,d0.w),d1                    | +0a0
        lea     0x2c07ac.l,a1                   | +0a4
        move.w  (a1,d0.w),d2                    | +0aa
        andi.w  #0x7,d3                         | +0ae
        addq.w  #0x1,d3                         | +0b2
        muls.w  d3,d1                           | +0b4
        asr.w   #0x2,d1                         | +0b6
        move.w  0x36(a6),d0                     | +0b8
        muls.w  d0,d1                           | +0bc
        muls.w  d0,d2                           | +0be
        asr.l   #0x8,d1                         | +0c0
        asr.l   #0x8,d2                         | +0c2
        move.w  d1,0x2a(a6)                     | +0c4
        move.w  d2,0x28(a6)                     | +0c8
        jsr     0x5e9b6.l                       | +0cc
        cmpi.b  #0x0,d0                         | +0d2
        bgt.w   .L06de66                        | +0d6
        jsr     0x5e9b6.l                       | +0da
        andi.w  #0x3,d0                         | +0e0
        addq.w  #0x2,d0                         | +0e4
        move.w  0x28(a6),d1                     | +0e6
        muls.w  d1,d0                           | +0ea
        asr.w   #0x1,d0                         | +0ec
        move.w  d0,0x28(a6)                     | +0ee
        jsr     0x5e9b6.l                       | +0f2
        andi.w  #0x3,d0                         | +0f8
        addq.w  #0x2,d0                         | +0fc
        move.w  0x2a(a6),d1                     | +0fe
        muls.w  d1,d0                           | +102
        asr.w   #0x1,d0                         | +104
        move.w  d0,0x2a(a6)                     | +106
.L06de66:
        move.w  0x2a(a6),d0                     | +10a
        move.w  d0,0x84(a6)                     | +10e
        move.w  #0xffd0,0x2e(a6)                | +112
        lea     .L06de7a(pc),a1                 | +118
        move.l  a1,(a6)                         | +11c
.L06de7a:
        jsr     0x27d50.l                       | +11e
        bcc.w   .L06deb6                        | +124
        move.w  #0xffd0,0x2e(a6)                | +128
        move.w  0x28(a6),d0                     | +12e
        asr.w   #0x2,d0                         | +132
        move.w  d0,0x28(a6)                     | +134
        move.w  0x84(a6),d0                     | +138
        asr.w   #0x1,d0                         | +13c
        move.w  d0,0x2a(a6)                     | +13e
        move.w  d0,0x84(a6)                     | +142
        subq.w  #0x1,0x80(a6)                   | +146
        cmpi.w  #0x0,0x80(a6)                   | +14a
        bgt.w   .L06deb6                        | +150
        lea     Jsr5B6ThenJmpScheduler_06da90(pc),a1 | +154
        move.l  a1,(a6)                         | +158
.L06deb6:
        move.w  0x82(a6),d0                     | +15a
        cmp.w   0x80(a6),d0                     | +15e
        beq.w   .L06ded0                        | +162
        addq.b  #0x1,0x72(a6)                   | +166
        btst    #0x0,0x72(a6)                   | +16a
        beq.w   .L06dee8                        | +170
.L06ded0:
        jsr     0x5e804.l                       | +174
        bcc.w   .L06dee2                        | +17a
        andi.b  #0xee,ccr                       | +17e
        bra.w   .L06dee8                        | +182
.L06dee2:
        jsr     0x28d70.l                       | +186
.L06dee8:
        movea.l #0xffffffff,a0                  | +18c
        lea     0x2d403e.l,a0                   | +192
        jsr     0x5dd56.l                       | +198
        bcc.w   SetHandlerRts_06df04            | +19e

| ----------------------------------------------------------------------------
|  TaskHandler_06df06  @ $06DF06  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06df06, "ax", @progbits
        .global TaskHandler_06df06
TaskHandler_06df06:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0xf00,d0                       | +006
        addi.w  #0x800,d0                       | +00a
        jsr     Sub_0006E20C(pc)                | +00e  -> $06E20C (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +012
        jsr     0x5e9b6.l                       | +016
        andi.w  #0xf00,d0                       | +01c
        addi.w  #0x800,d0                       | +020
        move.w  d0,0x2a(a6)                     | +024
        bra.w   Template_06DF32__L06df5a        | +028

| ----------------------------------------------------------------------------
|  Template_06DF32  @ $06DF32  (182 B)
| ----------------------------------------------------------------------------
        .section .text.Template_06DF32, "ax", @progbits
        .global Template_06DF32
Template_06DF32:
        jsr     0x5e9b6.l                       | +000
        andi.w  #0x700,d0                       | +006
        addi.w  #0x300,d0                       | +00a
        jsr     Sub_0006E20C(pc)                | +00e  -> $06E20C (hueco futuro, defsym forward)
        move.w  d0,0x28(a6)                     | +012
        jsr     0x5e9b6.l                       | +016
        andi.w  #0x700,d0                       | +01c
        addi.w  #0x500,d0                       | +020
        move.w  d0,0x2a(a6)                     | +024
        .global Template_06DF32__L06df5a
Template_06DF32__L06df5a:
.L06df5a:
        bset    #0x4,0x6b(a6)                   | +028
        move.w  #0x108e,d0                      | +02e
        jsr     0x2352.l                        | +032
        lea     0x2d4666.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        move.l  #0xffffffff,0x48(a6)            | +044
        move.l  #0x2d3e42,0x4c(a6)              | +04c
        move.w  #0x2,d1                         | +054
        jsr     0x236e.l                        | +058
        clr.w   0x72(a6)                        | +05e
        jsr     0x283ca.l                       | +062
        move.w  #0xd000,d0                      | +068
        jsr     0x28134.l                       | +06c
        andi.w  #0xffe3,0x38(a6)                | +072
        ori.w   #0x14,0x38(a6)                  | +078
        lea     .L06dfb6(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L06dfb6:
        jsr     0x27cee.l                       | +084
        bcc.w   .L06dfc8                        | +08a
        lea     0x31d26.l,a1                    | +08e
        move.l  a1,(a6)                         | +094
.L06dfc8:
        jsr     0x28d70.l                       | +096
        jsr     Sub_0006E2FE(pc)                | +09c  -> $06E2FE (hueco futuro, defsym forward)
        btst    #0x1,0x13(a6)                   | +0a0
        beq.w   .L06dfe4                        | +0a6
        lea     0x31d26.l,a1                    | +0aa
        move.l  a1,(a6)                         | +0b0
.L06dfe4:
        bra.w   Sub_0006E15E                    | +0b2  -> $06E15E (hueco futuro, defsym forward)
