| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $062000..$066000  (15,242 B, 210 entradas, 110 huecos)
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
|  Sub_00062008  @ $062008  (4 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062008, "ax", @progbits
        .global Sub_00062008
Sub_00062008:
        jsr     TaskHandler_06282e(pc)          | +000

| ----------------------------------------------------------------------------
|  Sub_00062014  @ $062014  (50 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062014, "ax", @progbits
        .global Sub_00062014
Sub_00062014:
        jsr     0x2870a.l                       | +000
        bcc.w   Sub_00062046__L062066           | +006
        bclr    #0x3,0x13(a6)                   | +00a
        lea     0x5e766.l,a0                    | +010
        jsr     0x5e770.l                       | +016
        lea     LateProp_Spark_061e72(pc),a1    | +01c
        move.l  a1,(a6)                         | +020
        jsr     0x5e844.l                       | +022
        bcc.w   Sub_00062046                    | +028
        lea     LateProp_SparkHit_061ebe(pc),a1 | +02c
        move.l  a1,(a6)                         | +030

| ----------------------------------------------------------------------------
|  Sub_00062046  @ $062046  (54 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062046, "ax", @progbits
        .global Sub_00062046
Sub_00062046:
        jsr     0x28758.l                       | +000
        bcc.w   .L062066                        | +006
        lea     LateProp_Shot_061f26(pc),a1     | +00a
        move.l  a1,(a6)                         | +00e
        jsr     0x5e844.l                       | +010
        bcc.w   .L062066                        | +016
        lea     LateProp_ShotB_061f9e__L061fae(pc),a1 | +01a
        move.l  a1,(a6)                         | +01e
        .global Sub_00062046__L062066
Sub_00062046__L062066:
.L062066:
        movea.l #0xffffffff,a0                  | +020
        lea     0x2c36f4.l,a0                   | +026
        jsr     0x5dd5c.l                       | +02c
        bcc.w   SetHandlerRts_062082            | +032

| ----------------------------------------------------------------------------
|  Sub_00062084  @ $062084  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062084, "ax", @progbits
        .global Sub_00062084
Sub_00062084:
        jsr     0x2870a.l                       | +000
        bcc.b   Sub_00062046                    | +006
        lea     0x5e766.l,a0                    | +008
        jsr     0x5e770.l                       | +00e
        bclr    #0x3,0x13(a6)                   | +014
        bra.b   Sub_00062046                    | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0620a0  @ $0620A0  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0620a0, "ax", @progbits
        .global TaskHandler_0620a0
TaskHandler_0620a0:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0620a6  @ $0620A6  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0620a6, "ax", @progbits
        .global TaskHandler_0620a6
TaskHandler_0620a6:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_0620b6  @ $0620B6  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0620b6, "ax", @progbits
        .global TaskHandler_0620b6
TaskHandler_0620b6:
        lea     0x2b7aa4.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x36(a6)                     | +00c
        tst.b   0x99(a6)                        | +010
        bne.w   .L0620d4                        | +014
        jmp     0x4c862.l                       | +018
.L0620d4:
        jmp     0x4c846.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_0620da  @ $0620DA  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0620da, "ax", @progbits
        .global TaskHandler_0620da
TaskHandler_0620da:
        move.w  #0x8,d1                         | +000
        jsr     0x236e.l                        | +004
        eori.b  #0x1,0x3a(a6)                   | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x1c,0x38(a6)                  | +020
        lea     0x2c4022.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L062112(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L062112:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   SetHandlerRts_062128            | +044

| ----------------------------------------------------------------------------
|  TaskHandler_06212a  @ $06212A  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06212a, "ax", @progbits
        .global TaskHandler_06212a
TaskHandler_06212a:
        move.w  #0x1a,d1                        | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e0d4.l                       | +00a
        bcs.w   .L062156                        | +010
        andi.b  #0xfe,0x3a(a6)                  | +014
        move.w  0x22(a0),d0                     | +01a
        cmp.w   0x22(a6),d0                     | +01e
        bcs.w   .L062156                        | +022
        ori.b   #0x1,0x3a(a6)                   | +026
.L062156:
        move.b  0x9a(a6),d0                     | +02c
        cmpi.b  #0x2,d0                         | +030
        bne.w   .L06216c                        | +034
        jsr     0x5e9b6.l                       | +038
        andi.b  #0x1,d0                         | +03e
.L06216c:
        tst.b   d0                              | +042
        beq.w   TaskHandler_0621d8__L0621fe     | +044
        move.b  #0x0,0x9a(a6)                   | +048
        move.b  #0x1,0x82(a6)                   | +04e
        lea     0x2c413a.l,a0                   | +054
        jsr     0x28cd4.l                       | +05a
        lea     .L062190(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L062190:
        jsr     TaskHandler_062684(pc)          | +066
        jsr     0x28d70.l                       | +06a
        bcc.w   .L0621a4                        | +070
        lea     TaskHandler_0621a8(pc),a1       | +074
        move.l  a1,(a6)                         | +078
.L0621a4:
        bra.w   TaskHandler_062326              | +07a

| ----------------------------------------------------------------------------
|  TaskHandler_0621a8  @ $0621A8  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0621a8, "ax", @progbits
        .global TaskHandler_0621a8
TaskHandler_0621a8:
        move.b  #0x2,0x82(a6)                   | +000
        lea     0x2c4188.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L0621c0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L0621c0:
        jsr     TaskHandler_062684(pc)          | +018
        jsr     0x28d70.l                       | +01c
        bcc.w   .L0621d4                        | +022
        lea     TaskHandler_0621d8(pc),a1       | +026
        move.l  a1,(a6)                         | +02a
.L0621d4:
        bra.w   TaskHandler_062326              | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_0621d8  @ $0621D8  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0621d8, "ax", @progbits
        .global TaskHandler_0621d8
TaskHandler_0621d8:
        lea     TaskHandler_062446__L062456(pc),a1 | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  #0x0,0x82(a6)                   | +010
        lea     0x2c4106.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        bra.w   .L062216                        | +022
        .global TaskHandler_0621d8__L0621fe
TaskHandler_0621d8__L0621fe:
.L0621fe:
        move.b  #0x1,0x9a(a6)                   | +026
        move.b  #0x0,0x82(a6)                   | +02c
        lea     0x2c40c4.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
.L062216:
        lea     .L06221c(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L06221c:
        jsr     TaskHandler_062684(pc)          | +044
        jsr     0x28d70.l                       | +048
        bcc.w   .L062230                        | +04e
        lea     TaskHandler_062234(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L062230:
        bra.w   TaskHandler_062326              | +058

| ----------------------------------------------------------------------------
|  TaskHandler_062234  @ $062234  (90 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062234, "ax", @progbits
        .global TaskHandler_062234
TaskHandler_062234:
        eori.b  #0x1,0x3a(a6)                   | +000
        move.w  #0xff78,d0                      | +006
        jsr     0x5dca4.l                       | +00a
        move.w  d0,0x28(a6)                     | +010
        move.w  #0x43f,0x2a(a6)                 | +014
        move.w  #0xff6f,0x2e(a6)                | +01a
        move.w  #0x0,0x2c(a6)                   | +020
        move.w  #0x8000,d0                      | +026
        jsr     0x28134.l                       | +02a
        andi.w  #0xffe3,0x38(a6)                | +030
        ori.w   #0x18,0x38(a6)                  | +036
        movea.l 0xc(a6),a0                      | +03c
        clr.b   0x20(a0)                        | +040
        cmpi.b  #0x1,0x9a(a6)                   | +044
        beq.w   .L062288                        | +04a
        jmp     0x58f82.l                       | +04e
.L062288:
        jmp     0x58fe2.l                       | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06228e  @ $06228E  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06228e, "ax", @progbits
        .global TaskHandler_06228e
TaskHandler_06228e:
        clr.b   0x44(a6)                        | +000
        jsr     Sub_000626B8(pc)                | +004
        bcs.w   .L0622aa                        | +008
        lea     0x2c41ea.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        bra.w   .L0622b6                        | +018
.L0622aa:
        lea     0x2c4274.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
.L0622b6:
        cmpi.b  #0x0,0x82(a6)                   | +028
        beq.w   .L0622ee                        | +02e
        cmpi.b  #0x2,0x82(a6)                   | +032
        beq.w   .L0622de                        | +038
        lea     TaskHandler_062446(pc),a1       | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046
        bra.w   .L0622ee                        | +04c
.L0622de:
        lea     TaskHandler_062446__L062456(pc),a1 | +050
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
.L0622ee:
        lea     .L0622f4(pc),a1                 | +060
        move.l  a1,(a6)                         | +064
.L0622f4:
        jsr     TaskHandler_062684(pc)          | +066
        addq.w  #0x1,0x38(a6)                   | +06a
        jsr     0x28d70.l                       | +06e
        bcc.w   .L062314                        | +074
        movea.l 0xc(a6),a0                      | +078
        clr.b   0x20(a0)                        | +07c
        lea     TaskHandler_0620a0(pc),a1       | +080
        move.l  a1,(a6)                         | +084
.L062314:
        jsr     0x5e45a.l                       | +086
        bcc.w   SetHandlerRts_062324            | +08c

| ----------------------------------------------------------------------------
|  TaskHandler_062326  @ $062326  (44 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062326, "ax", @progbits
        .global TaskHandler_062326
TaskHandler_062326:
        jsr     0x2870a.l                       | +000
        bcc.w   .L062348                        | +006
        lea     0x5e766.l,a0                    | +00a
        jsr     0x5e770.l                       | +010
        lea     TaskHandler_06228e(pc),a1       | +016
        move.l  a1,(a6)                         | +01a
        jsr     0x519be.l                       | +01c
.L062348:
        jsr     0x5e45a.l                       | +022
        bcc.w   SetHandlerRts_062358            | +028

| ----------------------------------------------------------------------------
|  TaskHandler_06235a  @ $06235A  (186 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06235a, "ax", @progbits
        .global TaskHandler_06235a
TaskHandler_06235a:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0xd000,0x38(a6)                | +006
        move.w  #0x16,d1                        | +00c
        jsr     0x236e.l                        | +010
        move.w  #0x1d2,d1                       | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x167,d1                       | +020
        jsr     0x236e.l                        | +024
        move.w  #0x200,d0                       | +02a
        btst    #0x0,0x3a(a6)                   | +02e
        bne.w   .L062394                        | +034
        neg.w   d0                              | +038
.L062394:
        move.w  d0,0x28(a6)                     | +03a
        asr.w   #0x5,d0                         | +03e
        move.w  d0,0x2c(a6)                     | +040
        move.w  #0x1,0x66(a6)                   | +044
        lea     0x2c4504.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        lea     .L0623b6(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L0623b6:
        lea     0x2c3750.l,a0                   | +05c
        move.b  0x106f28.l,d0                   | +062
        andi.w  #0x3,d0                         | +068
        add.w   d0,d0                           | +06c
        move.w  (a0,d0.w),d0                    | +06e
        move.w  (a6,d0.w),0x14(a6)              | +072
        jsr     0x27cee.l                       | +078
        jsr     0x28d70.l                       | +07e
        jsr     0x283d8.l                       | +084
        btst    #0x1,0x13(a6)                   | +08a
        beq.w   .L0623f4                        | +090
        lea     TaskHandler_06241c(pc),a1       | +094
        move.l  a1,(a6)                         | +098
.L0623f4:
        jsr     0x2870a.l                       | +09a
        bcc.w   .L062404                        | +0a0
        lea     TaskHandler_06241c(pc),a1       | +0a4
        move.l  a1,(a6)                         | +0a8
.L062404:
        movea.l #0xffffffff,a0                  | +0aa
        jsr     0x5dd56.l                       | +0b0
        bcc.w   SetHandlerRts_06241a            | +0b6

| ----------------------------------------------------------------------------
|  TaskHandler_06241c  @ $06241C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06241c, "ax", @progbits
        .global TaskHandler_06241c
TaskHandler_06241c:
        jsr     0x13600.l                       | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x0,0x38(a6)                   | +016
        move.l  #0xffffffff,0x48(a6)            | +01c
        jmp     0x77f6a.l                       | +024

| ----------------------------------------------------------------------------
|  TaskHandler_062446  @ $062446  (136 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062446, "ax", @progbits
        .global TaskHandler_062446
TaskHandler_062446:
        lea     0x2c4398.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L062462                        | +00c
        .global TaskHandler_062446__L062456
TaskHandler_062446__L062456:
.L062456:
        lea     0x2c444e.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L062462:
        move.w  #0x1a,d1                        | +01c
        jsr     0x236e.l                        | +020
        move.w  #0x2d,d0                        | +026
        jsr     0x5dca4.l                       | +02a
        move.w  d0,0x28(a6)                     | +030
        move.w  #0xb56,0x2a(a6)                 | +034
        move.w  #0xff7f,0x2e(a6)                | +03a
        move.w  #0x0,0x2c(a6)                   | +040
        move.w  #0x8000,d0                      | +046
        jsr     0x28134.l                       | +04a
        andi.w  #0xffe3,0x38(a6)                | +050
        ori.w   #0x0,0x38(a6)                   | +056
        lea     .L0624a8(pc),a1                 | +05c
        move.l  a1,(a6)                         | +060
.L0624a8:
        jsr     0x27c8c.l                       | +062
        bcc.w   .L0624b8                        | +068
        lea     Jsr5B6ThenJmpScheduler_0620a8(pc),a1 | +06c
        move.l  a1,(a6)                         | +070
.L0624b8:
        jsr     0x28d70.l                       | +072
        movea.l #0xffffffff,a0                  | +078
        jsr     0x5dd56.l                       | +07e
        bcc.w   SetHandlerRts_0624d4            | +084

| ----------------------------------------------------------------------------
|  TaskHandler_0624d6  @ $0624D6  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0624d6, "ax", @progbits
        .global TaskHandler_0624d6
TaskHandler_0624d6:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x18,d1                        | +016
        jsr     0x236e.l                        | +01a
        lea     0x2c4306.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L062508(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L062508:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L06251e                        | +03e
        lea     Jsr5B6ThenJmpScheduler_0620a8(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L06251e:
        movea.l #0xffffffff,a0                  | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   SetHandlerRts_062534            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_062536  @ $062536  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062536, "ax", @progbits
        .global TaskHandler_062536
TaskHandler_062536:
        move.w  #0xd000,0x38(a6)                | +000
        jsr     0x5e9b6.l                       | +006
        move.w  d0,d1                           | +00c
        andi.w  #0x700,d1                       | +00e
        addi.w  #0x200,d1                       | +012
        move.w  d1,0x36(a6)                     | +016
        btst    #0x0,d0                         | +01a
        beq.w   .L06256c                        | +01e
        lea     0x2de43a.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        move.w  #0x77,d1                        | +02e
        bra.w   .L06257c                        | +032
.L06256c:
        lea     0x2de5f4.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        move.w  #0x78,d1                        | +042
.L06257c:
        jsr     0x236e.l                        | +046
        jmp     0x6dce0.l                       | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_062588  @ $062588  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062588, "ax", @progbits
        .global TaskHandler_062588
TaskHandler_062588:
        move.w  #0xd000,0x38(a6)                | +000
        jsr     0x5e9b6.l                       | +006
        move.w  d0,d1                           | +00c
        andi.w  #0x700,d1                       | +00e
        addi.w  #0x100,d1                       | +012
        move.w  d1,0x36(a6)                     | +016
        btst    #0x0,d0                         | +01a
        beq.w   .L0625be                        | +01e
        lea     0x2de43a.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        move.w  #0x77,d1                        | +02e
        bra.w   .L0625ce                        | +032
.L0625be:
        lea     0x2de5f4.l,a0                   | +036
        jsr     0x28cd4.l                       | +03c
        move.w  #0x78,d1                        | +042
.L0625ce:
        jsr     0x236e.l                        | +046
        jmp     0x6dd16.l                       | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_0625da  @ $0625DA  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0625da, "ax", @progbits
        .global TaskHandler_0625da
TaskHandler_0625da:
        move.w  #0xd000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        move.w  #0x148,d1                       | +016
        jsr     0x236e.l                        | +01a
        lea     0x2c3f0a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L06260c(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L06260c:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L062622                        | +03e
        lea     Jsr5B6ThenJmpScheduler_0620a8(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L062622:
        movea.l #0xffffffff,a0                  | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   SetHandlerRts_062638            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_06263a  @ $06263A  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06263a, "ax", @progbits
        .global TaskHandler_06263a
TaskHandler_06263a:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd9ee.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L062656(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L062656:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L06266c                        | +028
        lea     Jsr5B6ThenJmpScheduler_0620a8(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L06266c:
        movea.l #0xffffffff,a0                  | +032
        jsr     0x5dd56.l                       | +038
        bcc.w   SetHandlerRts_062682            | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_062684  @ $062684  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062684, "ax", @progbits
        .global TaskHandler_062684
TaskHandler_062684:
        movea.l 0xc(a6),a0                      | +000
        move.w  #0xfffc,d0                      | +004
        btst    #0x0,0x3a(a0)                   | +008
        beq.w   .L062698                        | +00e
        neg.w   d0                              | +012
.L062698:
        add.w   0x22(a0),d0                     | +014
        move.w  d0,0x22(a6)                     | +018
        move.w  0x24(a0),d0                     | +01c
        addi.w  #0x28,d0                        | +020
        move.w  d0,0x24(a6)                     | +024
        move.w  0x38(a0),d0                     | +028
        subq.w  #0x1,d0                         | +02c

| ----------------------------------------------------------------------------
|  Sub_000626B8  @ $0626B8  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000626B8, "ax", @progbits
        .global Sub_000626B8
Sub_000626B8:
        btst    #0x0,0x3a(a6)                   | +000
        .global Sub_000626B8__L0626be
Sub_000626B8__L0626be:
.L0626be:
        bne.w   ClearXN_0626ca                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_0626d0  @ $0626D0  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0626d0, "ax", @progbits
        .global TaskHandler_0626d0
TaskHandler_0626d0:
        btst    #0x0,0x77(a6)                   | +000
        bra.b   Sub_000626B8__L0626be           | +006

| ----------------------------------------------------------------------------
|  Sub_000626D8  @ $0626D8  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000626D8, "ax", @progbits
        .global Sub_000626D8
Sub_000626D8:
        move.b  0x3a(a6),d0                     | +000
        cmp.b   0x77(a6),d0                     | +004
        bne.w   SetXN_0626ea                    | +008

| ----------------------------------------------------------------------------
|  Sub_000626F0  @ $0626F0  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000626F0, "ax", @progbits
        .global Sub_000626F0
Sub_000626F0:
        move.w  0x22(a6),d1                     | +000
        move.w  #0x88,d0                        | +004
        jsr     TaskHandler_0626d0(pc)          | +008
        bcs.w   .L062708                        | +00c
        add.w   0x72(a6),d0                     | +010
        cmp.w   d1,d0                           | +014
        rts                                     | +016
.L062708:
        add.w   0x72(a6),d0                     | +018
        cmp.w   d0,d1                           | +01c
        rts                                     | +01e

| ----------------------------------------------------------------------------
|  Sub_00062710  @ $062710  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062710, "ax", @progbits
        .global Sub_00062710
Sub_00062710:
        subq.w  #0x1,0x74(a6)                   | +000
        bgt.w   ClearXN_06272c                  | +004
        cmpi.w  #0x120,0x22(a6)                 | +008
        bgt.w   ClearXN_06272c                  | +00e
        clr.w   0x74(a6)                        | +012

| ----------------------------------------------------------------------------
|  Sub_00062732  @ $062732  (38 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062732, "ax", @progbits
        .global Sub_00062732
Sub_00062732:
        clr.l   d0                              | +000
        move.w  0x78(a6),d0                     | +002
        addq.w  #0x1,0x78(a6)                   | +006
        lsr.w   #0x4,d0                         | +00a
        cmpi.w  #0x3,d0                         | +00c
        ble.w   .L06274a                        | +010
        move.w  #0x3,d0                         | +014
.L06274a:
        lsl.w   #0x2,d0                         | +018
        movea.l 0x7c(a6),a0                     | +01a
        move.l  (a0,d0.w),0x5c(a6)              | +01e
        rts                                     | +024

| ----------------------------------------------------------------------------
|  Sub_00062758  @ $062758  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00062758, "ax", @progbits
        .global Sub_00062758
Sub_00062758:
        cmpi.w  #0x0,0x80(a6)                   | +000
        ble.w   JsrAbsThunk_06276c              | +006
        subq.w  #0x1,0x80(a6)                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_062774  @ $062774  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062774, "ax", @progbits
        .global TaskHandler_062774
TaskHandler_062774:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        tst.b   0x83(a6)                        | +016
        bne.w   JsrAbsRts_0627c8                | +01a
        move.l  #0xffffffff,0x4c(a6)            | +01e
        lea     0x2c379a.l,a1                   | +026
        tst.b   0x84(a6)                        | +02c
        beq.w   .L0627ae                        | +030
        lea     0x2c37c2.l,a1                   | +034
.L0627ae:
        jsr     0x43fac.l                       | +03a
        move.l  #0x2c3650,0x4c(a6)              | +040
        jsr     0x283ca.l                       | +048

| ----------------------------------------------------------------------------
|  TaskHandler_0627ca  @ $0627CA  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0627ca, "ax", @progbits
        .global TaskHandler_0627ca
TaskHandler_0627ca:
        move.w  #0x1067,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_0620b6(pc),a1       | +00a
        jsr     0x6fe.l                         | +00e
        jsr     0x5dd02.l                       | +014
        addi.w  #0x2c,0x24(a0)                  | +01a
        move.w  #0xffe2,d0                      | +020
        btst    #0x0,0x3a(a6)                   | +024
        beq.w   .L0627fa                        | +02a
        neg.w   d0                              | +02e
.L0627fa:
        add.w   d0,0x22(a0)                     | +030
        lea     TaskHandler_0620da(pc),a1       | +034
        jsr     0x6fe.l                         | +038
        jsr     0x5dd02.l                       | +03e
        addi.w  #0xa,0x24(a0)                   | +044
        rts                                     | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_062816  @ $062816  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062816, "ax", @progbits
        .global TaskHandler_062816
TaskHandler_062816:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x2c3788.l,a1                   | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06282e  @ $06282E  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06282e, "ax", @progbits
        .global TaskHandler_06282e
TaskHandler_06282e:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  #0x10,d0                        | +01c
.L06284e:
        movem.w d0,-(a7)                        | +020
        lea     TaskHandler_062536(pc),a1       | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        addi.w  #0x18,0x24(a0)                  | +034
        movem.w (a7)+,d0                        | +03a
        subq.w  #0x1,d0                         | +03e
        bcc.b   .L06284e                        | +040
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_062872  @ $062872  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062872, "ax", @progbits
        .global TaskHandler_062872
TaskHandler_062872:
        lea     TaskHandler_06212a(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  0x9a(a6),0x9a(a0)               | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_06288a  @ $06288A  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06288a, "ax", @progbits
        .global TaskHandler_06288a
TaskHandler_06288a:
        lea     TaskHandler_0625da(pc),a1       | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  TaskHandler_06289c  @ $06289C  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06289c, "ax", @progbits
        .global TaskHandler_06289c
TaskHandler_06289c:
        move.b  0x9b(a6),d0                     | +000
        move.w  #0x9c,d1                        | +004

| ----------------------------------------------------------------------------
|  TaskHandler_0628ac  @ $0628AC  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0628ac, "ax", @progbits
        .global TaskHandler_0628ac
TaskHandler_0628ac:
        move.w  #0x10aa,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_0624d6(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     TaskHandler_06235a(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        addi.w  #0x10,0x24(a0)                  | +02a
        move.w  #0x10,d0                        | +030
        jsr     Sub_000626B8(pc)                | +034
        add.w   d0,0x22(a0)                     | +038
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_0628ea  @ $0628EA  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0628ea, "ax", @progbits
        .global TaskHandler_0628ea
TaskHandler_0628ea:
        move.w  #0x10,d0                        | +000
.L0628ee:
        move.w  d0,-(a7)                        | +004
        lea     TaskHandler_062588(pc),a1       | +006
        jsr     0x4ae.l                         | +00a
        jsr     0x5dd02.l                       | +010
        addq.w  #0x1,0x38(a0)                   | +016
        move.w  (a7)+,d0                        | +01a
        subq.w  #0x1,d0                         | +01c
        cmpi.w  #0x0,d0                         | +01e
        bgt.b   .L0628ee                        | +022
        rts                                     | +024

| ----------------------------------------------------------------------------
|  TaskHandler_062910  @ $062910  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062910, "ax", @progbits
        .global TaskHandler_062910
TaskHandler_062910:
        lea     TaskHandler_06263a(pc),a1       | +000
        jsr     0x4ae.l                         | +004

| ----------------------------------------------------------------------------
|  TaskHandler_062922  @ $062922  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062922, "ax", @progbits
        .global TaskHandler_062922
TaskHandler_062922:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_062938                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06293e  @ $06293E  (194 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06293e, "ax", @progbits
        .global TaskHandler_06293e
TaskHandler_06293e:
        lea     TaskHandler_062a10(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.b  #0x1,0x83(a0)                   | +010
        move.b  #0x1,0x21(a6)                   | +016
        move.b  0x98(a6),d0                     | +01c
        andi.w  #0x3,d0                         | +020
        lsl.w   #0x2,d0                         | +024
        lea     0x2c4a34.l,a1                   | +026
        movea.l (a1,d0.w),a0                    | +02c
        jsr     0x799de.l                       | +030
        move.w  d0,0x86(a6)                     | +036
        move.w  #0x2d,0x70(a6)                  | +03a
        lea     .L062984(pc),a1                 | +040
        move.l  a1,(a6)                         | +044
.L062984:
        jsr     0x2783a.l                       | +046
        movea.l 0xc(a6),a0                      | +04c
        move.b  0x20(a0),0x20(a6)               | +050
        tst.b   0x21(a6)                        | +056
        bne.w   .L0629e6                        | +05a
        subq.w  #0x1,0x70(a6)                   | +05e
        cmpi.w  #0x0,0x70(a6)                   | +062
        bgt.w   .L0629e6                        | +068
        subq.w  #0x1,0x86(a6)                   | +06c
        cmpi.w  #0x0,0x86(a6)                   | +070
        blt.w   JmpToScheduler_062a08           | +076
        move.w  #0x2d,0x70(a6)                  | +07a
        lea     TaskHandler_062a10(pc),a1       | +080
        jsr     0x4ae.l                         | +084
        jsr     0x5dd02.l                       | +08a
        clr.b   0x83(a0)                        | +090
        movea.l 0xc(a6),a1                      | +094
        cmpi.b  #0xff,0x20(a1)                  | +098
        beq.w   .L0629e6                        | +09e
        move.b  #0x1,0x83(a0)                   | +0a2
.L0629e6:
        clr.b   0x21(a6)                        | +0a8
        movea.l #0xffffffff,a0                  | +0ac
        lea     0x2c49c4.l,a0                   | +0b2
        jsr     0x5dd5c.l                       | +0b8
        bcc.w   SetHandlerRts_062a06            | +0be

| ----------------------------------------------------------------------------
|  TaskHandler_062a10  @ $062A10  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062a10, "ax", @progbits
        .global TaskHandler_062a10
TaskHandler_062a10:
        move.b  #0x0,0x20(a6)                   | +000
        lea     .L062a1c(pc),a1                 | +006
        move.l  a1,(a6)                         | +00a
.L062a1c:
        movea.l 0xc(a6),a0                      | +00c
        cmpi.b  #0xff,0x20(a0)                  | +010
        bne.w   .L062a2e                        | +016
        clr.b   0x83(a6)                        | +01a
.L062a2e:
        jsr     PcThunkTarget_063336(pc)        | +01e
        cmpi.w  #0x20,0x22(a6)                  | +022
        ble.w   Jsr5B6ThenJmpScheduler_062f8c   | +028
        cmpi.w  #0x110,0x22(a6)                 | +02c
        ble.w   TaskHandler_062a4e              | +032

| ----------------------------------------------------------------------------
|  TaskHandler_062a4e  @ $062A4E  (304 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062a4e, "ax", @progbits
        .global TaskHandler_062a4e
TaskHandler_062a4e:
        bset    #0x1,0x12(a6)                   | +000
        move.w  #0x6d,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x5,0x1c(a6)                   | +010
        jsr     0x138fe.l                       | +016
        lea     0x2b80fc.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        lea     0x2b8200.l,a0                   | +02c
        jsr     0x799de.l                       | +032
        move.w  d0,0x72(a6)                     | +038
        jsr     0x5e9b6.l                       | +03c
        andi.w  #0x1f,d0                        | +042
        add.w   d0,0x72(a6)                     | +046
        jsr     0x5e9b6.l                       | +04a
        andi.b  #0x1,d0                         | +050
        addq.b  #0x2,d0                         | +054
        move.b  d0,0x82(a6)                     | +056
        move.w  #0x100,0x38(a6)                 | +05a
        move.w  #0x2,0x7c(a6)                   | +060
        move.w  #0x18,0x28(a6)                  | +066
        move.w  #0x120,0x2a(a6)                 | +06c
        move.w  #0x58,0x70(a6)                  | +072
        movea.l 0xc(a6),a0                      | +078
        cmpi.b  #0xff,0x20(a0)                  | +07c
        bne.w   .L062ae6                        | +082
        move.w  #0x38,0x28(a6)                  | +086
        move.w  #0x180,0x2a(a6)                 | +08c
        move.w  #0x30,0x70(a6)                  | +092
.L062ae6:
        clr.b   0x76(a6)                        | +098
        clr.b   0x77(a6)                        | +09c
        lea     0x2c4a74.l,a0                   | +0a0
        jsr     0x28cd4.l                       | +0a6
        lea     .L062b00(pc),a1                 | +0ac
        move.l  a1,(a6)                         | +0b0
.L062b00:
        cmpi.w  #0x30,0x70(a6)                  | +0b2
        bcc.w   .L062b1a                        | +0b8
        move.w  #0x1071,d0                      | +0bc
        jsr     0x2352.l                        | +0c0
        lea     .L062b1a(pc),a1                 | +0c6
        move.l  a1,(a6)                         | +0ca
.L062b1a:
        move.b  0x106f28.l,d0                   | +0cc
        andi.b  #0x1,d0                         | +0d2
        bne.w   .L062b3e                        | +0d6
        lea     TaskHandler_06331a(pc),a1       | +0da
        jsr     0x4ae.l                         | +0de
        jsr     0x5dd02.l                       | +0e4
        addi.w  #0x18,0x24(a0)                  | +0ea
.L062b3e:
        move.b  0x106f28.l,d0                   | +0f0
        andi.b  #0x3,d0                         | +0f6
        bne.w   .L062b5e                        | +0fa
        neg.w   0x2a(a6)                        | +0fe
        jsr     0x27cee.l                       | +102
        neg.w   0x2a(a6)                        | +108
        bra.w   .L062b64                        | +10c
.L062b5e:
        jsr     0x27cee.l                       | +110
.L062b64:
        jsr     0x28d70.l                       | +116
        subq.w  #0x1,0x70(a6)                   | +11c
        cmpi.w  #0x0,0x70(a6)                   | +120
        bgt.w   JsrPcThunk_062b7e               | +126
        lea     TaskHandler_062b84(pc),a1       | +12a
        move.l  a1,(a6)                         | +12e

| ----------------------------------------------------------------------------
|  TaskHandler_062b84  @ $062B84  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062b84, "ax", @progbits
        .global TaskHandler_062b84
TaskHandler_062b84:
        move.w  #0x1071,d0                      | +000
        jsr     0x2222.l                        | +004
        move.l  #0x2c459c,0x48(a6)              | +00a
        jsr     0x267e2.l                       | +012
        lea     0x2c4aa2.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L062bae(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L062bae:
        jsr     0x2783a.l                       | +02a
        jsr     0x28d70.l                       | +030
        bcc.w   .L062bc4                        | +036
        lea     TaskHandler_062bc8(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
.L062bc4:
        bra.w   TaskHandler_062ee0              | +040

| ----------------------------------------------------------------------------
|  TaskHandler_062bc8  @ $062BC8  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062bc8, "ax", @progbits
        .global TaskHandler_062bc8
TaskHandler_062bc8:
        andi.w  #0x3,0x7c(a6)                   | +000
        lea     0x2c4b38.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L062be0(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L062be0:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        subq.w  #0x1,0x72(a6)                   | +024
        cmpi.w  #0x0,0x72(a6)                   | +028
        bgt.w   .L062c32                        | +02e
        jsr     TaskHandler_063396(pc)          | +032
        bcs.w   .L062c32                        | +036
        move.w  #0x1,d1                         | +03a
        cmp.w   0x7c(a6),d0                     | +03e
        beq.w   .L062c24                        | +042
        bgt.w   .L062c16                        | +046
        move.w  #0xffff,d1                      | +04a
.L062c16:
        move.w  d1,0x80(a6)                     | +04e
        lea     TaskHandler_062c36(pc),a1       | +052
        move.l  a1,(a6)                         | +056
        bra.w   .L062c32                        | +058
.L062c24:
        jsr     TaskHandler_06340e(pc)          | +05c
        bcs.w   .L062c32                        | +060
        lea     TaskHandler_062cb4(pc),a1       | +064
        move.l  a1,(a6)                         | +068
.L062c32:
        bra.w   TaskHandler_062ee0              | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_062c36  @ $062C36  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062c36, "ax", @progbits
        .global TaskHandler_062c36
TaskHandler_062c36:
        move.w  0x7c(a6),d0                     | +000
        lsl.w   #0x1,d0                         | +004
        move.w  d0,0x34(a6)                     | +006
        clr.w   0x70(a6)                        | +00a
        clr.b   0x78(a6)                        | +00e
        andi.w  #0x7,0x34(a6)                   | +012
        lea     0x2c4bd6.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        lea     .L062c60(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L062c60:
        jsr     0x2783a.l                       | +02a
        subq.w  #0x1,0x70(a6)                   | +030
        cmpi.w  #0x0,0x70(a6)                   | +034
        bgt.w   .L062caa                        | +03a
        addq.b  #0x1,0x78(a6)                   | +03e
        cmpi.b  #0x3,0x78(a6)                   | +042
        beq.w   TaskHandler_062bc8              | +048
        lea     0x2b8386.l,a0                   | +04c
        jsr     0x799de.l                       | +052
        move.w  d0,0x70(a6)                     | +058
        move.w  0x80(a6),d0                     | +05c
        move.w  0x34(a6),d1                     | +060
        add.w   d1,d0                           | +064
        andi.w  #0x7,d0                         | +066
        move.w  d0,0x34(a6)                     | +06a
        lsr.w   #0x1,d0                         | +06e
        move.w  d0,0x7c(a6)                     | +070
.L062caa:
        jsr     0x28d70.l                       | +074
        bra.w   TaskHandler_062ee0              | +07a

| ----------------------------------------------------------------------------
|  TaskHandler_062cb4  @ $062CB4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062cb4, "ax", @progbits
        .global TaskHandler_062cb4
TaskHandler_062cb4:
        lea     0x2b817e.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x74(a6)                     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_062cc4  @ $062CC4  (174 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062cc4, "ax", @progbits
        .global TaskHandler_062cc4
TaskHandler_062cc4:
        jsr     TaskHandler_06340e(pc)          | +000
        bcs.w   TaskHandler_062bc8              | +004
        lea     0x2b8282.l,a0                   | +008
        jsr     0x799de.l                       | +00e
        move.w  d0,0x72(a6)                     | +014
        andi.w  #0x3,0x7c(a6)                   | +018
        move.w  0x7c(a6),d0                     | +01e
        lea     0x2c49ec.l,a0                   | +022
        cmpi.w  #0x1,0x74(a6)                   | +028
        blt.w   .L062d06                        | +02e
        cmpi.w  #0x2d,0x72(a6)                  | +032
        bge.w   .L062d06                        | +038
        lea     0x2c49fc.l,a0                   | +03c
.L062d06:
        lsl.w   #0x2,d0                         | +042
        movea.l (a0,d0.w),a0                    | +044
        cmpa.l  #0xffffffff,a0                  | +048
        beq.w   .L062d1c                        | +04e
        jsr     0x28cd4.l                       | +052
.L062d1c:
        lea     .L062d22(pc),a1                 | +058
        move.l  a1,(a6)                         | +05c
.L062d22:
        jsr     0x2783a.l                       | +05e
        jsr     TaskHandler_0633e0(pc)          | +064
        jsr     0x28d70.l                       | +068
        bcc.w   .L062d6e                        | +06e
        lea     TaskHandler_062cc4(pc),a1       | +072
        move.l  a1,(a6)                         | +076
        subq.w  #0x1,0x74(a6)                   | +078
        cmpi.w  #0x0,0x74(a6)                   | +07c
        bgt.w   .L062d6e                        | +082
        lea     0x2b8200.l,a0                   | +086
        jsr     0x799de.l                       | +08c
        move.w  d0,0x72(a6)                     | +092
        jsr     0x5e9b6.l                       | +096
        andi.w  #0x1f,d0                        | +09c
        add.w   d0,0x72(a6)                     | +0a0
        lea     TaskHandler_062bc8(pc),a1       | +0a4
        move.l  a1,(a6)                         | +0a8
.L062d6e:
        bra.w   TaskHandler_062ee0              | +0aa

| ----------------------------------------------------------------------------
|  TaskHandler_062d72  @ $062D72  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062d72, "ax", @progbits
        .global TaskHandler_062d72
TaskHandler_062d72:
        move.w  #0x1020,d0                      | +000
        jsr     0x2352.l                        | +004
        jsr     0x267e2.l                       | +00a
        lea     0x2c50f0.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L062d94(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L062d94:
        jsr     PcThunkTarget_063336(pc)        | +022
        jsr     0x2783a.l                       | +026
        jsr     0x28d70.l                       | +02c
        bcc.w   .L062dae                        | +032
        lea     TaskHandler_062dcc(pc),a1       | +036
        move.l  a1,(a6)                         | +03a
.L062dae:
        movea.l #0xffffffff,a0                  | +03c
        lea     0x2c49c4.l,a0                   | +042
        jsr     0x5dd56.l                       | +048
        bcc.w   SetHandlerRts_062dca            | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_062dcc  @ $062DCC  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062dcc, "ax", @progbits
        .global TaskHandler_062dcc
TaskHandler_062dcc:
        move.w  #0x8000,0x38(a6)                | +000
        jsr     0x267e2.l                       | +006
        subi.w  #0x20,0x24(a6)                  | +00c
        lea     0x2c5162.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        lea     .L062df0(pc),a1                 | +01e
        move.l  a1,(a6)                         | +022
.L062df0:
        jsr     PcThunkTarget_063336(pc)        | +024
        jsr     0x27bc8.l                       | +028
        bcc.w   .L062e04                        | +02e
        lea     TaskHandler_062e44(pc),a1       | +032
        move.l  a1,(a6)                         | +036
.L062e04:
        jsr     0x28d70.l                       | +038
        jsr     0x283ca.l                       | +03e
        jsr     0x283d8.l                       | +044
        btst    #0x1,0x13(a6)                   | +04a
        beq.w   .L062e26                        | +050
        lea     TaskHandler_062e8a(pc),a1       | +054
        move.l  a1,(a6)                         | +058
.L062e26:
        movea.l #0xffffffff,a0                  | +05a
        lea     0x2c49c4.l,a0                   | +060
        jsr     0x5dd56.l                       | +066
        bcc.w   SetHandlerRts_062e42            | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_062e44  @ $062E44  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062e44, "ax", @progbits
        .global TaskHandler_062e44
TaskHandler_062e44:
        lea     0x2c51ca.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L062e56(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L062e56:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L062e6c                        | +01e
        bclr    #0x1,0x12(a6)                   | +022
.L062e6c:
        movea.l #0xffffffff,a0                  | +028
        lea     0x2c49c4.l,a0                   | +02e
        jsr     0x5dd56.l                       | +034
        bcc.w   SetHandlerRts_062e88            | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_062e8a  @ $062E8A  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062e8a, "ax", @progbits
        .global TaskHandler_062e8a
TaskHandler_062e8a:
        bclr    #0x1,0x12(a6)                   | +000
        jsr     0x267e2.l                       | +006
        lea     0x2c5236.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L062ea8(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L062ea8:
        jsr     PcThunkTarget_063336(pc)        | +01e
        jsr     0x2783a.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L062ec2                        | +02e
        lea     Jsr5B6ThenJmpScheduler_062f8c(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L062ec2:
        movea.l #0xffffffff,a0                  | +038
        lea     0x2c49c4.l,a0                   | +03e
        jsr     0x5dd56.l                       | +044
        bcc.w   SetHandlerRts_062ede            | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_062ee0  @ $062EE0  (164 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062ee0, "ax", @progbits
        .global TaskHandler_062ee0
TaskHandler_062ee0:
        jsr     PcThunkTarget_063336(pc)        | +000
        lea     0x5e766.l,a0                    | +004
        jsr     0x5e770.l                       | +00a
        jsr     0x2870a.l                       | +010
        bcc.w   .L062f06                        | +016
        bclr    #0x3,0x13(a6)                   | +01a
        move.b  #0xa,0x76(a6)                   | +020
.L062f06:
        subq.b  #0x1,0x76(a6)                   | +026
        cmpi.b  #0x0,0x76(a6)                   | +02a
        bgt.w   .L062f1a                        | +030
        move.b  #0x0,0x76(a6)                   | +034
.L062f1a:
        cmpi.b  #0x0,0x76(a6)                   | +03a
        beq.w   .L062f40                        | +040
        addq.b  #0x1,0x77(a6)                   | +044
        move.b  0x77(a6),d0                     | +048
        andi.w  #0x3,d0                         | +04c
        add.w   d0,d0                           | +050
        lea     0x2c49d4.l,a0                   | +052
        move.w  (a0,d0.w),d0                    | +058
        add.w   d0,0x24(a6)                     | +05c
.L062f40:
        cmpi.b  #0x0,0x83(a6)                   | +060
        beq.w   .L062f5e                        | +066
        movea.l 0xc(a6),a0                      | +06a
        cmpi.b  #0xff,0x20(a0)                  | +06e
        bne.w   .L062f5e                        | +074
        lea     TaskHandler_062d72(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L062f5e:
        jsr     0x28758.l                       | +07e
        bcc.w   .L062f6e                        | +084
        lea     TaskHandler_062e8a(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L062f6e:
        movea.l #0xffffffff,a0                  | +08e
        lea     0x2c49c4.l,a0                   | +094
        jsr     0x5dd56.l                       | +09a
        bcc.w   SetHandlerRts_062f8a            | +0a0

| ----------------------------------------------------------------------------
|  TaskHandler_062fa8  @ $062FA8  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062fa8, "ax", @progbits
        .global TaskHandler_062fa8
TaskHandler_062fa8:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_062fcc  @ $062FCC  (116 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_062fcc, "ax", @progbits
        .global TaskHandler_062fcc
TaskHandler_062fcc:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x58,d1                        | +006
        jsr     0x236e.l                        | +00a
        move.w  #0xd000,0x38(a6)                | +010
        jsr     TaskHandler_063342(pc)          | +016
        lea     0x2c5248.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L062ff8(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L062ff8:
        jsr     0x27d50.l                       | +02c
        bcc.w   .L063008                        | +032
        lea     TaskHandler_063048(pc),a1       | +036
        move.l  a1,(a6)                         | +03a
.L063008:
        jsr     0x28d70.l                       | +03c
        jsr     0x283ca.l                       | +042
        jsr     0x283d8.l                       | +048
        btst    #0x1,0x13(a6)                   | +04e
        beq.w   .L06302a                        | +054
        lea     TaskHandler_062fa8(pc),a1       | +058
        move.l  a1,(a6)                         | +05c
.L06302a:
        movea.l #0xffffffff,a0                  | +05e
        lea     0x2c49cc.l,a0                   | +064
        jsr     0x5dd56.l                       | +06a
        bcc.w   SetHandlerRts_063046            | +070

| ----------------------------------------------------------------------------
|  TaskHandler_063048  @ $063048  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063048, "ax", @progbits
        .global TaskHandler_063048
TaskHandler_063048:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   TaskHandler_062fa8              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_063056  @ $063056  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063056, "ax", @progbits
        .global TaskHandler_063056
TaskHandler_063056:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        eori.b  #0x1,0x3a(a6)                   | +016
        move.w  #0x8,d1                         | +01c
        jsr     0x236e.l                        | +020
        lea     0x2c9c60.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L06308e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L06308e:
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bcc.w   .L0630a4                        | +044
        lea     Jsr5B6ThenJmpScheduler_062f9a(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L0630a4:
        movea.l #0xffffffff,a0                  | +04e
        jsr     0x5dd56.l                       | +054
        bcc.w   SetHandlerRts_0630ba            | +05a

| ----------------------------------------------------------------------------
|  TaskHandler_0630bc  @ $0630BC  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0630bc, "ax", @progbits
        .global TaskHandler_0630bc
TaskHandler_0630bc:
        move.w  #0x1d,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2e2286.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0630d8(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0630d8:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L0630ee                        | +028
        lea     TaskHandler_063106(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L0630ee:
        movea.l #0xffffffff,a0                  | +032
        jsr     0x5dd56.l                       | +038
        bcc.w   SetHandlerRts_063104            | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_063106  @ $063106  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063106, "ax", @progbits
        .global TaskHandler_063106
TaskHandler_063106:
        jsr     0x5e9b6.l                       | +000
        lea     0x2c4a44.l,a1                   | +006
        andi.w  #0x3,d1                         | +00c
        lsl.w   #0x2,d1                         | +010
        movem.l d1/a1,-(a7)                     | +012
        lea     Jsr5B6ThenJmpScheduler_062f9a(pc),a1 | +016
        jsr     0x4498e.l                       | +01a
        jsr     0x5dd02.l                       | +020
        movem.l (a7)+,d1/a1                     | +026
        move.l  (a1,d1.w),(a0)                  | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_063142  @ $063142  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063142, "ax", @progbits
        .global TaskHandler_063142
TaskHandler_063142:
        jsr     0x4a0d4.l                       | +000
        jmp     0x5724e.l                       | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06314e  @ $06314E  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06314e, "ax", @progbits
        .global TaskHandler_06314e
TaskHandler_06314e:
        jsr     0x4a0d4.l                       | +000
        jmp     0x58fc2.l                       | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06315a  @ $06315A  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06315a, "ax", @progbits
        .global TaskHandler_06315a
TaskHandler_06315a:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        move.w  #0x8000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x1c,0x38(a6)                  | +026
        lea     0x2dd37e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.w  #0x6c,d1                        | +038
        jsr     0x236e.l                        | +03c
        lea     .L0631a2(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L0631a2:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        bcc.w   .L0631b8                        | +054
        lea     Jsr5B6ThenJmpScheduler_062f9a(pc),a1 | +058
        move.l  a1,(a6)                         | +05c
.L0631b8:
        movea.l #0xffffffff,a0                  | +05e
        jsr     0x5dd56.l                       | +064
        bcc.w   SetHandlerRts_0631ce            | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_0631d0  @ $0631D0  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0631d0, "ax", @progbits
        .global TaskHandler_0631d0
TaskHandler_0631d0:
        move.w  #0x2000,0x38(a6)                | +000
        move.w  #0x6d,d1                        | +006
        jsr     0x236e.l                        | +00a
        jsr     0x267e2.l                       | +010
        lea     0x2c52ba.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L0631f8(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L0631f8:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L06320e                        | +034
        lea     Jsr5B6ThenJmpScheduler_062f9a(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L06320e:
        movea.l #0xffffffff,a0                  | +03e
        jsr     0x5dd56.l                       | +044
        bcc.w   SetHandlerRts_063224            | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_063226  @ $063226  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063226, "ax", @progbits
        .global TaskHandler_063226
TaskHandler_063226:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.w  #0x6e,d1                        | +016
        jsr     0x236e.l                        | +01a
        jsr     0x5e9b6.l                       | +020
        btst    #0x0,d0                         | +026
        beq.w   .L063264                        | +02a
        lea     0x2ddf9e.l,a0                   | +02e
        jsr     0x28cd4.l                       | +034
        bra.w   .L063270                        | +03a
.L063264:
        lea     0x2de014.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
.L063270:
        move.w  #0x200,0x36(a6)                 | +04a
        jmp     0x6dce0.l                       | +050

| ----------------------------------------------------------------------------
|  TaskHandler_06327c  @ $06327C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06327c, "ax", @progbits
        .global TaskHandler_06327c
TaskHandler_06327c:
        move.w  #0x8000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        jsr     0x5e9b6.l                       | +016
        btst    #0x0,d0                         | +01c
        beq.w   .L0632b4                        | +020
        lea     0x2de43a.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        move.w  #0x77,d1                        | +030
        bra.w   .L0632c4                        | +034
.L0632b4:
        lea     0x2de5f4.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        move.w  #0x78,d1                        | +044
.L0632c4:
        jsr     0x236e.l                        | +048
        jmp     0x6dce0.l                       | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_0632d0  @ $0632D0  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0632d0, "ax", @progbits
        .global TaskHandler_0632d0
TaskHandler_0632d0:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd72a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L0632ec(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L0632ec:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L063302                        | +028
        lea     Jsr5B6ThenJmpScheduler_062f9a(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L063302:
        movea.l #0xffffffff,a0                  | +032
        jsr     0x5dd56.l                       | +038
        bcc.w   SetHandlerRts_063318            | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_06331a  @ $06331A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06331a, "ax", @progbits
        .global TaskHandler_06331a
TaskHandler_06331a:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2dd80c.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        jmp     0x725ca.l                       | +016

| ----------------------------------------------------------------------------
|  PcThunkTarget_063336  @ $063336  (12 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_063336, "ax", @progbits
        .global PcThunkTarget_063336
PcThunkTarget_063336:
        movea.l 0xc(a6),a0                      | +000
        move.b  #0x1,0x21(a0)                   | +004
        rts                                     | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_063342  @ $063342  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063342, "ax", @progbits
        .global TaskHandler_063342
TaskHandler_063342:
        lea     0x2b8304.l,a0                   | +000
        jsr     0x799de.l                       | +006
        andi.w  #0x3,d0                         | +00c
        lsl.w   #0x2,d0                         | +010
        lea     0x2c4a0c.l,a0                   | +012
        move.l  (a0,d0.w),0x5c(a6)              | +018
        move.w  0x7c(a6),d0                     | +01e
        andi.w  #0x3,d0                         | +022
        move.w  d0,d1                           | +026
        add.w   d0,d0                           | +028
        add.w   d1,d0                           | +02a
        add.w   d0,d0                           | +02c
        movea.l 0x5c(a6),a0                     | +02e
        move.w  (a0,d0.w),0x28(a6)              | +032
        move.w  0x2(a0,d0.w),0x2e(a6)           | +038
        move.w  0x4(a0,d0.w),0x2a(a6)           | +03e
        btst    #0x0,0x3a(a6)                   | +044
        beq.w   .L063394                        | +04a
        neg.w   0x28(a6)                        | +04e
.L063394:
        rts                                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_063396  @ $063396  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063396, "ax", @progbits
        .global TaskHandler_063396
TaskHandler_063396:
        jsr     0x5e0d4.l                       | +000
        bcs.w   SetXN_0633da                    | +006
        move.w  0x22(a6),d1                     | +00a
        sub.w   0x22(a0),d1                     | +00e
        btst    #0x0,0x3a(a6)                   | +012
        beq.w   .L0633b4                        | +018
        neg.w   d1                              | +01c
.L0633b4:
        clr.w   d0                              | +01e
        cmpi.w  #0x60,d1                        | +020
        bgt.w   ClearXN_0633d4                  | +024
        addq.w  #0x1,d0                         | +028
        cmpi.w  #0x0,d1                         | +02a
        bgt.w   ClearXN_0633d4                  | +02e
        addq.w  #0x1,d0                         | +032
        cmpi.w  #0xffa0,d1                      | +034
        bgt.w   ClearXN_0633d4                  | +038
        addq.w  #0x1,d0                         | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_0633e0  @ $0633E0  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0633e0, "ax", @progbits
        .global TaskHandler_0633e0
TaskHandler_0633e0:
        subq.w  #0x1,0x72(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +004
        ble.w   .L0633f4                        | +00a
        move.b  #0x1,0x44(a6)                   | +00e
.L0633f4:
        rts                                     | +014

| ----------------------------------------------------------------------------
|  TaskHandler_0633f6  @ $0633F6  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0633f6, "ax", @progbits
        .global TaskHandler_0633f6
TaskHandler_0633f6:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   ClearXN_063408                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06340e  @ $06340E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06340e, "ax", @progbits
        .global TaskHandler_06340e
TaskHandler_06340e:
        cmpi.w  #0x10,0x22(a6)                  | +000
        blt.w   SetXN_063428                    | +006
        cmpi.w  #0x110,0x22(a6)                 | +00a
        bgt.w   SetXN_063428                    | +010

| ----------------------------------------------------------------------------
|  TaskHandler_06342e  @ $06342E  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06342e, "ax", @progbits
        .global TaskHandler_06342e
TaskHandler_06342e:
        lea     TaskHandler_0632d0(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0xffd8,d0                      | +010
        jsr     TaskHandler_0633f6(pc)          | +014
        add.w   d0,0x22(a0)                     | +018
        addq.w  #0x3,0x24(a0)                   | +01c
        addq.w  #0x1,0x38(a0)                   | +020
        lea     TaskHandler_0632d0(pc),a1       | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        move.w  #0x24,d0                        | +034
        jsr     TaskHandler_0633f6(pc)          | +038
        add.w   d0,0x22(a0)                     | +03c
        addq.w  #0x3,0x24(a0)                   | +040
        eori.b  #0x1,0x3a(a0)                   | +044
        subq.w  #0x1,0x38(a0)                   | +04a
        rts                                     | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_06347e  @ $06347E  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06347e, "ax", @progbits
        .global TaskHandler_06347e
TaskHandler_06347e:
        move.w  #0x1067,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x7c(a6),d3                     | +00a
        andi.w  #0x3,d3                         | +00e
        lsl.w   #0x2,d3                         | +012
        lea     0x2c49dc.l,a1                   | +014
        move.w  (a1,d3.w),d0                    | +01a
        jsr     TaskHandler_0633f6(pc)          | +01e
        move.w  0x2(a1,d3.w),d1                 | +022
        movem.l d0-d1,-(a7)                     | +026
        lea     TaskHandler_063056(pc),a1       | +02a
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        movem.l (a7)+,d0-d1                     | +03a
        add.w   d0,0x22(a0)                     | +03e
        add.w   d1,0x24(a0)                     | +042
        move.w  0x7c(a6),0x7c(a0)               | +046
        movem.l d0-d1,-(a7)                     | +04c
        lea     TaskHandler_062fcc(pc),a1       | +050
        jsr     0x4ae.l                         | +054
        jsr     0x5dd02.l                       | +05a
        movem.l (a7)+,d0-d1                     | +060
        add.w   d0,0x22(a0)                     | +064
        add.w   d1,0x24(a0)                     | +068
        move.w  0x7c(a6),0x7c(a0)               | +06c

| ----------------------------------------------------------------------------
|  PcThunkTarget_0634f6  @ $0634F6  (46 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_0634f6, "ax", @progbits
        .global PcThunkTarget_0634f6
PcThunkTarget_0634f6:
        lea     TaskHandler_0631d0(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        addi.w  #0x18,0x22(a0)                  | +010
        lea     TaskHandler_0631d0(pc),a1       | +016
        jsr     0x4ae.l                         | +01a
        jsr     0x5dd02.l                       | +020
        addi.w  #0xfff0,0x22(a0)                | +026
        rts                                     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_063524  @ $063524  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063524, "ax", @progbits
        .global TaskHandler_063524
TaskHandler_063524:
        lea     TaskHandler_06315a(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  #0x10,d0                        | +010
.L063538:
        movem.w d0,-(a7)                        | +014
        lea     TaskHandler_063226(pc),a1       | +018
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        addi.w  #0x18,0x24(a0)                  | +028
        movem.w (a7)+,d0                        | +02e
        subq.w  #0x1,d0                         | +032
        bcc.b   .L063538                        | +034
        rts                                     | +036

| ----------------------------------------------------------------------------
|  TaskHandler_06355c  @ $06355C  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06355c, "ax", @progbits
        .global TaskHandler_06355c
TaskHandler_06355c:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        move.w  #0x10,d0                        | +01c
.L06357c:
        movem.w d0,-(a7)                        | +020
        lea     TaskHandler_06327c(pc),a1       | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        addi.w  #0x18,0x24(a0)                  | +034
        movem.w (a7)+,d0                        | +03a
        subq.w  #0x1,d0                         | +03e
        bcc.b   .L06357c                        | +040
        rts                                     | +042

| ----------------------------------------------------------------------------
|  TaskHandler_0635a0  @ $0635A0  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0635a0, "ax", @progbits
        .global TaskHandler_0635a0
TaskHandler_0635a0:
        subq.b  #0x1,0x82(a6)                   | +000
        cmpi.b  #0x0,0x82(a6)                   | +004
        blt.w   .L0635d6                        | +00a
        lea     TaskHandler_0630bc(pc),a1       | +00e
        jsr     0x4ae.l                         | +012
        jsr     0x5dd02.l                       | +018
        move.w  #0x1f,d0                        | +01e
        jsr     TaskHandler_0633f6(pc)          | +022
        add.w   d0,0x22(a0)                     | +026
        addi.w  #0x18,0x24(a0)                  | +02a
        eori.b  #0x1,0x3a(a0)                   | +030
.L0635d6:
        rts                                     | +036

| ----------------------------------------------------------------------------
|  TaskHandler_0635d8  @ $0635D8  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0635d8, "ax", @progbits
        .global TaskHandler_0635d8
TaskHandler_0635d8:
        move.l  #0x2c4920,0x4c(a6)              | +000
        jsr     0x283ca.l                       | +008
        jsr     0x283d8.l                       | +00e
        move.l  #0xffffffff,0x4c(a6)            | +014
        lea     0x2c4a54.l,a1                   | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_063602  @ $063602  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063602, "ax", @progbits
        .global TaskHandler_063602
TaskHandler_063602:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_063618                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_06361e  @ $06361E  (264 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06361e, "ax", @progbits
        .global TaskHandler_06361e
TaskHandler_06361e:
        clr.b   0x78(a6)                        | +000
        bra.w   .L06362c                        | +004
        move.b  #0x1,0x78(a6)                   | +008
.L06362c:
        jsr     0x5e7c0.l                       | +00e
        jsr     0x267e2.l                       | +014
        lea     TaskHandler_063bc2(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        move.w  #0x2c,d1                        | +02a
        jsr     0x236e.l                        | +02e
        move.w  #0x6,0x1c(a6)                   | +034
        jsr     0x138fe.l                       | +03a
        lea     0x2b8610.l,a0                   | +040
        jsr     0x799de.l                       | +046
        move.w  d0,0x66(a6)                     | +04c
        move.w  0x66(a6),d0                     | +050
        asr.w   #0x1,d0                         | +054
        move.w  d0,0x7a(a6)                     | +056
        move.b  #0x0,0x79(a6)                   | +05a
        move.w  #0x8000,d0                      | +060
        jsr     0x28134.l                       | +064
        andi.w  #0xffe3,0x38(a6)                | +06a
        ori.w   #0x8,0x38(a6)                   | +070
        move.b  #0x0,d0                         | +076
        btst    #0x0,0x98(a6)                   | +07a
        beq.w   .L0636a6                        | +080
        move.b  #0x1,d0                         | +084
.L0636a6:
        move.b  d0,0x3a(a6)                     | +088
        move.b  #0x0,d0                         | +08c
        btst    #0x1,0x98(a6)                   | +090
        beq.w   .L0636bc                        | +096
        move.b  #0x1,d0                         | +09a
.L0636bc:
        move.b  d0,0x72(a6)                     | +09e
        move.b  0x99(a6),d1                     | +0a2
        andi.w  #0xff,d1                        | +0a6
        lsl.w   #0x4,d1                         | +0aa
        move.w  d1,0x74(a6)                     | +0ac
        lea     0x2c56a8.l,a0                   | +0b0
        move.b  0x9a(a6),d0                     | +0b6
        andi.w  #0x3,d0                         | +0ba
        lsl.w   #0x2,d0                         | +0be
        movea.l (a0,d0.w),a0                    | +0c0
        jsr     TaskHandler_063c76(pc)          | +0c4
        clr.b   0x20(a6)                        | +0c8
        move.l  #0x2c5356,0x48(a6)              | +0cc
        move.l  #0x2c55ee,0x60(a6)              | +0d4
        lea     0x723d2.l,a1                    | +0dc
        jsr     0x4ae.l                         | +0e2
        jsr     0x5dd02.l                       | +0e8
        clr.w   0x98(a0)                        | +0ee
        clr.b   0x90(a6)                        | +0f2
        clr.b   0x91(a6)                        | +0f6
        move.w  #0x10a5,d0                      | +0fa
        jsr     0x2352.l                        | +0fe
        bra.w   TaskHandler_063726__L06376e     | +104

| ----------------------------------------------------------------------------
|  TaskHandler_063726  @ $063726  (348 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063726, "ax", @progbits
        .global TaskHandler_063726
TaskHandler_063726:
        jsr     0x267e2.l                       | +000
        lea     0x2c58e0.l,a0                   | +006
        move.l  #0x2c58fe,d0                    | +00c
        move.l  d0,0x7c(a6)                     | +012
        cmpi.b  #0x0,0x79(a6)                   | +016
        beq.w   .L063748                        | +01c
        movea.l d0,a0                           | +020
.L063748:
        jsr     0x28cd4.l                       | +022
        lea     .L063754(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L063754:
        jsr     TaskHandler_063d6e(pc)          | +02e
        jsr     0x28998.l                       | +032
        jsr     0x2783a.l                       | +038
        jsr     0x28d70.l                       | +03e
        bra.w   .L0637e8                        | +044
        .global TaskHandler_063726__L06376e
TaskHandler_063726__L06376e:
.L06376e:
        move.l  #0x2c544e,0x4c(a6)              | +048
        move.w  #0xff40,d0                      | +050
        btst    #0x0,0x3a(a6)                   | +054
        beq.w   .L063788                        | +05a
        move.w  #0x180,d0                       | +05e
.L063788:
        move.w  d0,0x28(a6)                     | +062
        lea     0x2c5828.l,a0                   | +066
        move.l  #0x2c5856,d0                    | +06c
        move.l  d0,0x7c(a6)                     | +072
        cmpi.b  #0x0,0x79(a6)                   | +076
        beq.w   .L0637a8                        | +07c
        movea.l d0,a0                           | +080
.L0637a8:
        jsr     0x28cd4.l                       | +082
        lea     .L0637b4(pc),a1                 | +088
        move.l  a1,(a6)                         | +08c
.L0637b4:
        jsr     TaskHandler_063d6e(pc)          | +08e
        jsr     0x28998.l                       | +092
        jsr     0x27cee.l                       | +098
        jsr     0x28d70.l                       | +09e
        jsr     TaskHandler_063c44(pc)          | +0a4
        bcc.w   .L0637d8                        | +0a8
        lea     TaskHandler_063726(pc),a1       | +0ac
        move.l  a1,(a6)                         | +0b0
.L0637d8:
        jsr     0x283ca.l                       | +0b2
        jsr     0x283d8.l                       | +0b8
        bra.w   .L0637e8                        | +0be
.L0637e8:
        lea     0x5e766.l,a0                    | +0c2
        jsr     0x5e770.l                       | +0c8
        jsr     0x2870a.l                       | +0ce
        bcc.w   .L06383c                        | +0d4
        bclr    #0x3,0x13(a6)                   | +0d8
        cmpi.b  #0x4,0x58(a6)                   | +0de
        bne.w   .L063812                        | +0e4
        addq.b  #0x1,0x90(a6)                   | +0e8
.L063812:
        cmpi.b  #0x0,0x79(a6)                   | +0ec
        bne.w   .L06383c                        | +0f2
        move.w  0x7a(a6),d0                     | +0f6
        cmp.w   0x66(a6),d0                     | +0fa
        blt.w   .L06383c                        | +0fe
        move.b  #0x1,0x79(a6)                   | +102
        movea.l 0x7c(a6),a0                     | +108
        jsr     0x28cd4.l                       | +10c
        jsr     TaskHandler_063df2(pc)          | +112
.L06383c:
        cmpi.b  #0x1,0x90(a6)                   | +116
        blt.w   .L06384c                        | +11c
        lea     TaskHandler_06388a(pc),a1       | +120
        move.l  a1,(a6)                         | +124
.L06384c:
        jsr     0x28758.l                       | +126
        bcc.w   .L06385c                        | +12c
        lea     TaskHandler_06388a(pc),a1       | +130
        move.l  a1,(a6)                         | +134
.L06385c:
        cmpi.w  #0xff80,0x22(a6)                | +136
        bgt.w   .L06386c                        | +13c
        lea     TaskHandler_063942(pc),a1       | +140
        move.l  a1,(a6)                         | +144
.L06386c:
        movea.l #0xffffffff,a0                  | +146
        lea     0x2c55e6.l,a0                   | +14c
        jsr     0x5dd5c.l                       | +152
        bcc.w   SetHandlerRts_063888            | +158

| ----------------------------------------------------------------------------
|  TaskHandler_06388a  @ $06388A  (114 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06388a, "ax", @progbits
        .global TaskHandler_06388a
TaskHandler_06388a:
        bset    #0x0,0x13(a6)                   | +000
        move.w  #0x10a5,d0                      | +006
        jsr     0x2222.l                        | +00a
        bclr    #0x1,0x12(a6)                   | +010
        jsr     0x267e2.l                       | +016
        lea     0x2c5918.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L0638b8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0638b8:
        jsr     0x2783a.l                       | +02e
        jsr     0x28d70.l                       | +034
        bcc.w   .L0638d6                        | +03a
        tst.b   0x9b(a6)                        | +03e
        beq.w   .L0638d6                        | +042
        lea     TaskHandler_063904(pc),a1       | +046
        move.l  a1,(a6)                         | +04a
.L0638d6:
        cmpi.w  #0xff80,0x22(a6)                | +04c
        bgt.w   .L0638e6                        | +052
        lea     TaskHandler_063942(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L0638e6:
        movea.l #0xffffffff,a0                  | +05c
        lea     0x2c55e6.l,a0                   | +062
        jsr     0x5dd5c.l                       | +068
        bcc.w   SetHandlerRts_063902            | +06e

| ----------------------------------------------------------------------------
|  TaskHandler_063904  @ $063904  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063904, "ax", @progbits
        .global TaskHandler_063904
TaskHandler_063904:
        move.w  #0x102f,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     0x77fd6.l,a1                    | +00a
        jsr     0x4ae.l                         | +010
        jsr     0x5dd02.l                       | +016
        addi.w  #0xffe0,0x22(a0)                | +01c
        lea     0x77fd6.l,a1                    | +022
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        addi.w  #0x20,0x22(a0)                  | +034
        jsr     TaskHandler_063d82(pc)          | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_063942  @ $063942  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063942, "ax", @progbits
        .global TaskHandler_063942
TaskHandler_063942:
        jmp     0x518.l                         | +000

| ----------------------------------------------------------------------------
|  TaskHandler_063948  @ $063948  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063948, "ax", @progbits
        .global TaskHandler_063948
TaskHandler_063948:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_063960  @ $063960  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063960, "ax", @progbits
        .global TaskHandler_063960
TaskHandler_063960:
        movea.l 0xc(a6),a0                      | +000
        move.w  #0x120,0x98(a6)                 | +004
        move.w  0x80(a6),d0                     | +00a
        move.w  0x82(a6),d1                     | +00e
        move.b  d0,0x9a(a6)                     | +012
        move.b  d1,0x9b(a6)                     | +016
        move.b  #0xff,0x9c(a6)                  | +01a
        clr.b   0x9e(a6)                        | +020
        move.b  0x72(a0),0x9d(a6)               | +024
        jmp     0x6a7d6.l                       | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_063990  @ $063990  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063990, "ax", @progbits
        .global TaskHandler_063990
TaskHandler_063990:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77f6a.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_0639b4  @ $0639B4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0639b4, "ax", @progbits
        .global TaskHandler_0639b4
TaskHandler_0639b4:
        move.w  #0xe,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0xffff,0x88(a6)                | +00a
        jsr     0x5e9b6.l                       | +010
        andi.w  #0xf,d0                         | +016
        move.w  d0,0x70(a6)                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0639d2  @ $0639D2  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0639d2, "ax", @progbits
        .global TaskHandler_0639d2
TaskHandler_0639d2:
        move.b  0x86(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        movea.l #0x2c5698,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L0639f6                        | +01a
        jsr     0x28cd4.l                       | +01e
.L0639f6:
        lea     .L0639fc(pc),a1                 | +024
        move.l  a1,(a6)                         | +028
.L0639fc:
        jsr     TaskHandler_063cd4(pc)          | +02a
        jsr     TaskHandler_063cf0(pc)          | +02e
        jsr     0x28d70.l                       | +032
        jsr     TaskHandler_063cb4(pc)          | +038
        bcc.w   .L063a18                        | +03c
        lea     TaskHandler_063a4a(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L063a18:
        bra.w   TaskHandler_063b38              | +046

| ----------------------------------------------------------------------------
|  TaskHandler_063a1c  @ $063A1C  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063a1c, "ax", @progbits
        .global TaskHandler_063a1c
TaskHandler_063a1c:
        lea     0x29bf34.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L063a2e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L063a2e:
        jsr     TaskHandler_063cd4(pc)          | +012
        jsr     TaskHandler_063cf0(pc)          | +016
        jsr     0x28d70.l                       | +01a
        bcc.w   .L063a46                        | +020
        lea     TaskHandler_0639d2(pc),a1       | +024
        move.l  a1,(a6)                         | +028
.L063a46:
        bra.w   TaskHandler_063b38              | +02a

| ----------------------------------------------------------------------------
|  TaskHandler_063a4a  @ $063A4A  (120 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063a4a, "ax", @progbits
        .global TaskHandler_063a4a
TaskHandler_063a4a:
        jsr     0x5e0d4.l                       | +000
        bcs.w   .L063a72                        | +006
        move.w  0x22(a0),d0                     | +00a
        clr.w   d1                              | +00e
        cmp.w   0x22(a6),d0                     | +010
        blt.w   .L063a6c                        | +014
        btst    d1,0x3a(a6)                     | +018
        beq.b   TaskHandler_063a1c              | +01c
        bra.w   .L063a72                        | +01e
.L063a6c:
        btst    d1,0x3a(a6)                     | +022
        bne.b   TaskHandler_063a1c              | +026
.L063a72:
        lea     0x2b8818.l,a0                   | +028
        jsr     0x799de.l                       | +02e
        move.w  d0,0x70(a6)                     | +034
        lea     0x29ba70.l,a0                   | +038
        jsr     0x28cd4.l                       | +03e
        jsr     0x5e9b6.l                       | +044
        andi.b  #0x3,d0                         | +04a
        addi.b  #0x14,d0                        | +04e
        move.b  d0,0x5c(a6)                     | +052
        lea     .L063aa6(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L063aa6:
        jsr     TaskHandler_063cd4(pc)          | +05c
        jsr     TaskHandler_063cf0(pc)          | +060
        jsr     0x28d70.l                       | +064
        bcc.w   .L063abe                        | +06a
        lea     TaskHandler_0639d2(pc),a1       | +06e
        move.l  a1,(a6)                         | +072
.L063abe:
        bra.w   TaskHandler_063b38              | +074

| ----------------------------------------------------------------------------
|  TaskHandler_063ac2  @ $063AC2  (110 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063ac2, "ax", @progbits
        .global TaskHandler_063ac2
TaskHandler_063ac2:
        jsr     0x4a0d4.l                       | +000
        move.w  #0x0,0x70(a6)                   | +006
        lea     0x4b136.l,a0                    | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L063ae0(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L063ae0:
        jsr     TaskHandler_063cd4(pc)          | +01e
        jsr     TaskHandler_063cf0(pc)          | +022
        addq.w  #0x1,0x70(a6)                   | +026
        move.w  0x70(a6),d0                     | +02a
        cmpi.w  #0xa,d0                         | +02e
        blt.w   .L063b10                        | +032
        btst    #0x0,d0                         | +036
        beq.w   .L063b20                        | +03a
        cmpi.w  #0x1e,0x70(a6)                  | +03e
        blt.w   .L063b10                        | +044
        lea     Jsr5B6ThenJmpScheduler_063952(pc),a1 | +048
        move.l  a1,(a6)                         | +04c
.L063b10:
        jsr     0x28d70.l                       | +04e
        bcs.w   .L063b20                        | +054
        move.w  #0x0,0x70(a6)                   | +058
.L063b20:
        movea.l #0xffffffff,a0                  | +05e
        jsr     0x5dd5c.l                       | +064
        bcc.w   SetHandlerRts_063b36            | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_063b38  @ $063B38  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063b38, "ax", @progbits
        .global TaskHandler_063b38
TaskHandler_063b38:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0xff,0x20(a0)                  | +004
        bne.w   .L063b88                        | +00a
        lea     0x2c5718.l,a0                   | +00e
        move.b  0x86(a6),d0                     | +014
        andi.w  #0x3,d0                         | +018
        move.w  d0,d1                           | +01c
        add.w   d1,d1                           | +01e
        add.w   d1,d0                           | +020
        add.w   d0,d0                           | +022
        move.w  (a0,d0.w),0x28(a6)              | +024
        btst    #0x0,0x3a(a6)                   | +02a
        beq.w   .L063b70                        | +030
        neg.w   0x28(a6)                        | +034
.L063b70:
        move.w  #0x0,0x2c(a6)                   | +038
        move.w  0x2(a0,d0.w),0x2e(a6)           | +03e
        move.w  0x4(a0,d0.w),0x2a(a6)           | +044
        lea     TaskHandler_063bb6(pc),a1       | +04a
        move.l  a1,(a6)                         | +04e
.L063b88:
        jsr     0x2870a.l                       | +050
        bcc.w   .L063b9e                        | +056
        lea     TaskHandler_063ac2(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
        jsr     0x519be.l                       | +060
.L063b9e:
        movea.l #0xffffffff,a0                  | +066
        jsr     0x5dd5c.l                       | +06c
        bcc.w   SetHandlerRts_063bb4            | +072

| ----------------------------------------------------------------------------
|  TaskHandler_063bb6  @ $063BB6  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063bb6, "ax", @progbits
        .global TaskHandler_063bb6
TaskHandler_063bb6:
        jsr     0x4a0d4.l                       | +000
        jmp     0x57226.l                       | +006

| ----------------------------------------------------------------------------
|  TaskHandler_063bc2  @ $063BC2  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063bc2, "ax", @progbits
        .global TaskHandler_063bc2
TaskHandler_063bc2:
        move.l  #0x2c563a,0x60(a6)              | +000
        lea     .L063bd0(pc),a1                 | +008
        move.l  a1,(a6)                         | +00c
.L063bd0:
        jsr     0x5e506.l                       | +00e
        jsr     0x28998.l                       | +014
        jsr     TaskHandler_063d2a(pc)          | +01a
        jsr     0x28d70.l                       | +01e
        movea.l 0xc(a6),a0                      | +024
        btst    #0x0,0x13(a0)                   | +028
        beq.w   .L063bfa                        | +02e
        lea     Jsr5B6ThenJmpScheduler_063952(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L063bfa:
        jsr     0x5e45a.l                       | +038
        bcc.w   SetHandlerRts_063c0a            | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_063c0c  @ $063C0C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063c0c, "ax", @progbits
        .global TaskHandler_063c0c
TaskHandler_063c0c:
        cmpi.b  #0x1,0x3a(a6)                   | +000
        .global TaskHandler_063c0c__L063c12
TaskHandler_063c0c__L063c12:
.L063c12:
        beq.w   ClearXN_063c1e                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_063c24  @ $063C24  (8 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063c24, "ax", @progbits
        .global TaskHandler_063c24
TaskHandler_063c24:
        cmpi.b  #0x1,0x72(a6)                   | +000
        bra.b   TaskHandler_063c0c__L063c12     | +006

| ----------------------------------------------------------------------------
|  TaskHandler_063c2c  @ $063C2C  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063c2c, "ax", @progbits
        .global TaskHandler_063c2c
TaskHandler_063c2c:
        move.b  0x3a(a6),d0                     | +000
        cmp.b   0x72(a6),d0                     | +004
        bne.w   SetXN_063c3e                    | +008

| ----------------------------------------------------------------------------
|  TaskHandler_063c44  @ $063C44  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063c44, "ax", @progbits
        .global TaskHandler_063c44
TaskHandler_063c44:
        move.w  0x22(a6),d1                     | +000
        cmpi.b  #0x0,0x72(a6)                   | +004
        bne.w   .L063c60                        | +00a
        move.w  0x74(a6),d0                     | +00e
        cmp.w   d0,d1                           | +012
        bgt.w   ClearXN_063c6a                  | +014
        bra.w   SetXN_063c70                    | +018
.L063c60:
        move.w  0x74(a6),d0                     | +01c
        cmp.w   d0,d1                           | +020
        bge.w   SetXN_063c70                    | +022

| ----------------------------------------------------------------------------
|  TaskHandler_063c76  @ $063C76  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063c76, "ax", @progbits
        .global TaskHandler_063c76
TaskHandler_063c76:
        clr.b   d6                              | +000
        movea.l a0,a2                           | +002
.L063c7a:
        move.l  (a2),d0                         | +004
        cmpi.l  #0xffffffff,d0                  | +006
        beq.w   .L063cb2                        | +00c
        movem.l d6/a2,-(a7)                     | +010
        movea.l d0,a1                           | +014
        jsr     0x4ae.l                         | +016
        jsr     0x5dd02.l                       | +01c
        movem.l (a7)+,d6/a2                     | +022
        move.b  d6,0x86(a0)                     | +026
        addq.b  #0x1,d6                         | +02a
        addq.l  #0x4,a2                         | +02c
        move.w  (a2),0x80(a0)                   | +02e
        addq.l  #0x2,a2                         | +032
        move.w  (a2),0x82(a0)                   | +034
        addq.l  #0x2,a2                         | +038
        bra.b   .L063c7a                        | +03a
.L063cb2:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_063cb4  @ $063CB4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063cb4, "ax", @progbits
        .global TaskHandler_063cb4
TaskHandler_063cb4:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   ClearXN_063cce                  | +00a
        move.w  #0x0,0x70(a6)                   | +00e

| ----------------------------------------------------------------------------
|  TaskHandler_063cd4  @ $063CD4  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063cd4, "ax", @progbits
        .global TaskHandler_063cd4
TaskHandler_063cd4:
        move.b  0x3a(a6),0x87(a6)               | +000
        jsr     0x5e506.l                       | +006
        move.b  0x87(a6),0x3a(a6)               | +00c
        move.w  0x88(a6),d0                     | +012
        add.w   d0,0x38(a6)                     | +016
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_063cf0  @ $063CF0  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063cf0, "ax", @progbits
        .global TaskHandler_063cf0
TaskHandler_063cf0:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x80(a6),d0                     | +004
        move.w  0x82(a6),d1                     | +008
        cmpi.b  #0x1,0x3a(a0)                   | +00c
        beq.w   .L063d08                        | +012
        neg.w   d0                              | +016
.L063d08:
        add.w   d0,0x22(a6)                     | +018
        add.w   d1,0x24(a6)                     | +01c
        move.b  0x3b(a0),d0                     | +020
        andi.w  #0x3,d0                         | +024
        add.w   d0,d0                           | +028
        lea     0x2c5820.l,a1                   | +02a
        move.w  (a1,d0.w),d0                    | +030
        add.w   d0,0x24(a6)                     | +034
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_063d2a  @ $063D2A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063d2a, "ax", @progbits
        .global TaskHandler_063d2a
TaskHandler_063d2a:
        move.w  #0xffe4,d0                      | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L063d3c                        | +00a
        move.w  #0x0,d0                         | +00e
.L063d3c:
        add.w   0x22(a6),d0                     | +012
        move.w  0x24(a6),d1                     | +016
        addi.w  #0x2c,d1                        | +01a
        move.w  #0x1c,d2                        | +01e
        jsr     0x99812.l                       | +022
        move.w  #0xffd0,d0                      | +028
        add.w   0x22(a6),d0                     | +02c
        move.w  0x24(a6),d1                     | +030
        addi.w  #0x1c,d1                        | +034
        move.w  #0x60,d2                        | +038

| ----------------------------------------------------------------------------
|  TaskHandler_063d6e  @ $063D6E  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063d6e, "ax", @progbits
        .global TaskHandler_063d6e
TaskHandler_063d6e:
        tst.b   0x91(a6)                        | +000
        beq.w   .L063d7c                        | +004
        bset    #0x0,0x5a(a6)                   | +008
.L063d7c:
        clr.b   0x91(a6)                        | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_063d82  @ $063D82  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063d82, "ax", @progbits
        .global TaskHandler_063d82
TaskHandler_063d82:
        jsr     0x6e412.l                       | +000
        move.w  #0xc,d0                         | +006
.L063d8c:
        movem.w d0,-(a7)                        | +00a
        lea     0x62536.l,a1                    | +00e
        jsr     0x4ae.l                         | +014
        jsr     0x5dd02.l                       | +01a
        addi.w  #0x18,0x24(a0)                  | +020
        movem.w (a7)+,d0                        | +026
        subq.w  #0x1,d0                         | +02a
        bcc.b   .L063d8c                        | +02c
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_063db2  @ $063DB2  (64 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063db2, "ax", @progbits
        .global TaskHandler_063db2
TaskHandler_063db2:
        move.w  #0x1033,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x38(a6),0x5c(a6)               | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x8,0x38(a6)                   | +020
        lea     0x77fd6.l,a1                    | +026
        jsr     0x4ae.l                         | +02c
        jsr     0x5dd02.l                       | +032
        move.w  0x5c(a6),0x38(a6)               | +038
        rts                                     | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_063df2  @ $063DF2  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063df2, "ax", @progbits
        .global TaskHandler_063df2
TaskHandler_063df2:
        move.w  #0x1023,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  0x38(a6),0x5c(a6)               | +00a
        move.w  #0x4000,d0                      | +010
        jsr     0x28134.l                       | +014
        andi.w  #0xffe3,0x38(a6)                | +01a
        ori.w   #0x8,0x38(a6)                   | +020
        lea     0x2c5686.l,a1                   | +026
        jsr     0x77c7e.l                       | +02c
        move.w  0x5c(a6),0x38(a6)               | +032
        rts                                     | +038

| ----------------------------------------------------------------------------
|  TaskHandler_063e2c  @ $063E2C  (94 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063e2c, "ax", @progbits
        .global TaskHandler_063e2c
TaskHandler_063e2c:
        move.b  #0xff,0x20(a6)                  | +000
        tst.b   0x9b(a6)                        | +006
        beq.w   .L063e3c                        | +00a
        rts                                     | +00e
.L063e3c:
        move.l  #0x2c5542,0x4c(a6)              | +010
        jsr     0x283ca.l                       | +018
        jsr     0x283d8.l                       | +01e
        move.l  #0xffffffff,0x4c(a6)            | +024
        lea     0x2c5730.l,a1                   | +02c
        tst.b   0x78(a6)                        | +032
        beq.w   .L063e6c                        | +036
        lea     0x2c57a8.l,a1                   | +03a
.L063e6c:
        btst    #0x0,0x3a(a6)                   | +040
        beq.w   JsrAbsThunk_063e8a              | +046
        lea     0x2c576c.l,a1                   | +04a
        tst.b   0x78(a6)                        | +050
        beq.w   JsrAbsThunk_063e8a              | +054
        lea     0x2c57e4.l,a1                   | +058

| ----------------------------------------------------------------------------
|  TaskHandler_063e92  @ $063E92  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063e92, "ax", @progbits
        .global TaskHandler_063e92
TaskHandler_063e92:
        lea     0x77fd6.l,a1                    | +000
        jsr     0x4ae.l                         | +006
        jsr     0x5dd02.l                       | +00c
        subq.w  #0x1,0x38(a0)                   | +012
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_063eaa  @ $063EAA  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063eaa, "ax", @progbits
        .global TaskHandler_063eaa
TaskHandler_063eaa:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_063ec0                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_063ec6  @ $063EC6  (216 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063ec6, "ax", @progbits
        .global TaskHandler_063ec6
TaskHandler_063ec6:
        bset    #0x3,0x13(a6)                   | +000
        move.l  #0x2c5958,0x48(a6)              | +006
        bra.w   .L063ee6                        | +00e
        bset    #0x3,0x13(a6)                   | +012
        move.l  #0x2c59ac,0x48(a6)              | +018
.L063ee6:
        move.w  #0x83,d1                        | +020
        jsr     0x236e.l                        | +024
        move.w  #0x19,0x1c(a6)                  | +02a
        jsr     0x138fe.l                       | +030
        lea     0x2b8c68.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x72(a6)                     | +042
        lea     0x2b8b64.l,a0                   | +046
        jsr     0x799de.l                       | +04c
        move.w  d0,0x66(a6)                     | +052
        asr.w   #0x1,d0                         | +056
        move.w  d0,0x80(a6)                     | +058
        clr.b   0x83(a6)                        | +05c
        clr.b   0x82(a6)                        | +060
        move.w  #0x100,d0                       | +064
        jsr     0x28134.l                       | +068
        andi.w  #0xffe3,0x38(a6)                | +06e
        ori.w   #0x14,0x38(a6)                  | +074
        jsr     0x267e2.l                       | +07a
        lea     0x723d2.l,a1                    | +080
        jsr     0x4ae.l                         | +086
        jsr     0x5dd02.l                       | +08c
        clr.w   0x98(a0)                        | +092
        move.b  0x98(a6),d0                     | +096
        andi.w  #0x1f,d0                        | +09a
        lsl.w   #0x4,d0                         | +09e
        move.w  d0,0x86(a6)                     | +0a0
        lea     0x2c5ab2.l,a0                   | +0a4
        jsr     0x28cd4.l                       | +0aa
        lea     .L063f7c(pc),a1                 | +0b0
        move.l  a1,(a6)                         | +0b4
.L063f7c:
        jsr     0x2783a.l                       | +0b6
        jsr     0x28d70.l                       | +0bc
        move.w  0x22(a6),d0                     | +0c2
        cmp.w   0x86(a6),d0                     | +0c6
        bgt.w   .L063f9a                        | +0ca
        lea     TaskHandler_063f9e(pc),a1       | +0ce
        move.l  a1,(a6)                         | +0d2
.L063f9a:
        bra.w   TaskHandler_0641d2              | +0d4

| ----------------------------------------------------------------------------
|  TaskHandler_063f9e  @ $063F9E  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063f9e, "ax", @progbits
        .global TaskHandler_063f9e
TaskHandler_063f9e:
        bclr    #0x3,0x13(a6)                   | +000
        lea     0x2c5ac6.l,a0                   | +006
        jsr     0x28cd4.l                       | +00c
        lea     .L063fb6(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L063fb6:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L063fcc                        | +024
        lea     TaskHandler_063fd0(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L063fcc:
        bra.w   TaskHandler_0641d2              | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_063fd0  @ $063FD0  (156 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_063fd0, "ax", @progbits
        .global TaskHandler_063fd0
TaskHandler_063fd0:
        btst    #0x0,0x83(a6)                   | +000
        bne.w   .L064002                        | +006
        jsr     TaskHandler_064422(pc)          | +00a
        bcs.w   .L063ff2                        | +00e
        lea     0x2c5b90.l,a0                   | +012
        jsr     0x28cd4.l                       | +018
        bra.w   .L063ffe                        | +01e
.L063ff2:
        lea     0x2c5bb8.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
.L063ffe:
        bra.w   .L064026                        | +02e
.L064002:
        jsr     TaskHandler_064422(pc)          | +032
        bcs.w   .L06401a                        | +036
        lea     0x2c5b7c.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        bra.w   .L064026                        | +046
.L06401a:
        lea     0x2c5ba4.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
.L064026:
        lea     .L06402c(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L06402c:
        jsr     0x2783a.l                       | +05c
        jsr     0x28d70.l                       | +062
        jsr     TaskHandler_06443c(pc)          | +068
        bcc.w   .L064046                        | +06c
        lea     TaskHandler_063fd0(pc),a1       | +070
        move.l  a1,(a6)                         | +074
.L064046:
        cmpi.w  #0x20,0x22(a6)                  | +076
        blt.w   .L064068                        | +07c
        cmpi.w  #0x120,0x22(a6)                 | +080
        bgt.w   .L064068                        | +086
        jsr     TaskHandler_0643da(pc)          | +08a
        bcc.w   .L064068                        | +08e
        lea     TaskHandler_06406c(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L064068:
        bra.w   TaskHandler_0641d2              | +098

| ----------------------------------------------------------------------------
|  TaskHandler_06406c  @ $06406C  (132 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06406c, "ax", @progbits
        .global TaskHandler_06406c
TaskHandler_06406c:
        jsr     TaskHandler_0643f4(pc)          | +000
        bcs.w   TaskHandler_0640f0              | +004
        btst    #0x0,0x83(a6)                   | +008
        bne.w   .L0640a6                        | +00e
        jsr     TaskHandler_064422(pc)          | +012
        bcs.w   .L064096                        | +016
        lea     0x2c5bcc.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   .L0640a2                        | +026
.L064096:
        lea     0x2c5c84.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
.L0640a2:
        bra.w   .L0640ca                        | +036
.L0640a6:
        jsr     TaskHandler_064422(pc)          | +03a
        bcs.w   .L0640be                        | +03e
        lea     0x2c5c28.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        bra.w   .L0640ca                        | +04e
.L0640be:
        lea     0x2c5ce0.l,a0                   | +052
        jsr     0x28cd4.l                       | +058
.L0640ca:
        eori.b  #0x1,0x83(a6)                   | +05e
        lea     .L0640d6(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L0640d6:
        jsr     0x2783a.l                       | +06a
        jsr     0x28d70.l                       | +070
        bcc.w   .L0640ec                        | +076
        lea     TaskHandler_0640f0(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L0640ec:
        bra.w   TaskHandler_0641d2              | +080

| ----------------------------------------------------------------------------
|  TaskHandler_0640f0  @ $0640F0  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0640f0, "ax", @progbits
        .global TaskHandler_0640f0
TaskHandler_0640f0:
        clr.w   0x76(a6)                        | +000
        lea     0x2b8be6.l,a0                   | +004
        jsr     0x799de.l                       | +00a
        move.w  d0,0x74(a6)                     | +010

| ----------------------------------------------------------------------------
|  TaskHandler_064104  @ $064104  (174 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064104, "ax", @progbits
        .global TaskHandler_064104
TaskHandler_064104:
        lea     0x2b8cea.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x72(a6)                     | +00c
        btst    #0x0,0x83(a6)                   | +010
        bne.w   .L064146                        | +016
        jsr     TaskHandler_064422(pc)          | +01a
        bcs.w   .L064136                        | +01e
        lea     0x2c5d8c.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        bra.w   .L064142                        | +02e
.L064136:
        lea     0x2c5e2c.l,a0                   | +032
        jsr     0x28cd4.l                       | +038
.L064142:
        bra.w   .L06416a                        | +03e
.L064146:
        jsr     TaskHandler_064422(pc)          | +042
        bcs.w   .L06415e                        | +046
        lea     0x2c5d3c.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
        bra.w   .L06416a                        | +056
.L06415e:
        lea     0x2c5ddc.l,a0                   | +05a
        jsr     0x28cd4.l                       | +060
.L06416a:
        lea     .L064170(pc),a1                 | +066
        move.l  a1,(a6)                         | +06a
.L064170:
        jsr     0x2783a.l                       | +06c
        jsr     0x28d70.l                       | +072
        bcc.w   .L0641ae                        | +078
        lea     TaskHandler_064104(pc),a1       | +07c
        move.l  a1,(a6)                         | +080
        addq.w  #0x1,0x76(a6)                   | +082
        subq.w  #0x1,0x74(a6)                   | +086
        cmpi.w  #0x0,0x74(a6)                   | +08a
        bgt.w   .L0641ae                        | +090
        lea     TaskHandler_063fd0(pc),a1       | +094
        move.l  a1,(a6)                         | +098
        lea     0x2b8c68.l,a0                   | +09a
        jsr     0x799de.l                       | +0a0
        move.w  d0,0x72(a6)                     | +0a6
.L0641ae:
        bra.w   TaskHandler_0641d2              | +0aa

| ----------------------------------------------------------------------------
|  TaskHandler_0641b2  @ $0641B2  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0641b2, "ax", @progbits
        .global TaskHandler_0641b2
TaskHandler_0641b2:
        lea     0x2c5e7c.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        jsr     0x2783a.l                       | +00c
        jsr     0x28d70.l                       | +012

| ----------------------------------------------------------------------------
|  TaskHandler_0641d2  @ $0641D2  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0641d2, "ax", @progbits
        .global TaskHandler_0641d2
TaskHandler_0641d2:
        jsr     0x2870a.l                       | +000
        bcc.w   .L0641f4                        | +006
        jsr     0x519be.l                       | +00a
        lea     0x5e766.l,a0                    | +010
        jsr     0x5e770.l                       | +016
        bclr    #0x3,0x13(a6)                   | +01c
.L0641f4:
        jsr     0x28758.l                       | +022
        bcc.w   .L064204                        | +028
        lea     TaskHandler_0641b2(pc),a1       | +02c
        move.l  a1,(a6)                         | +030
.L064204:
        movea.l #0xffffffff,a0                  | +032
        lea     0x2c5a54.l,a0                   | +038
        jsr     0x5dd5c.l                       | +03e
        bcc.w   SetHandlerRts_064220            | +044

| ----------------------------------------------------------------------------
|  TaskHandler_064238  @ $064238  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064238, "ax", @progbits
        .global TaskHandler_064238
TaskHandler_064238:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        move.l  #0xffffffff,0x48(a6)            | +016
        jmp     0x77efe.l                       | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_06425c  @ $06425C  (278 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06425c, "ax", @progbits
        .global TaskHandler_06425c
TaskHandler_06425c:
        move.w  0x76(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        move.w  d0,d1                           | +008
        add.w   d0,d0                           | +00a
        add.w   d1,d0                           | +00c
        add.w   d0,d0                           | +00e
        lea     0x2c5a9a.l,a0                   | +010
        move.w  0x2(a0,d0.w),0x2e(a6)           | +016
        move.w  0x4(a0,d0.w),0x2a(a6)           | +01c
        move.w  (a0,d0.w),d0                    | +022
        jsr     TaskHandler_064486(pc)          | +026
        move.w  d0,0x28(a6)                     | +02a
        bset    #0x4,0x6b(a6)                   | +02e
        move.l  #0x2c5a00,0x4c(a6)              | +034
        move.w  #0x167,d1                       | +03c
        jsr     0x236e.l                        | +040
        move.w  #0x1d2,d1                       | +046
        jsr     0x236e.l                        | +04a
        jsr     0x283ca.l                       | +050
        move.w  #0xd000,d0                      | +056
        jsr     0x28134.l                       | +05a
        andi.w  #0xffe3,0x38(a6)                | +060
        ori.w   #0x14,0x38(a6)                  | +066
        lea     0x2c5e92.l,a0                   | +06c
        jsr     0x28cd4.l                       | +072
        lea     .L0642da(pc),a1                 | +078
        move.l  a1,(a6)                         | +07c
.L0642da:
        jsr     0x27cee.l                       | +07e
        bcc.w   .L0642ea                        | +084
        lea     TaskHandler_064238(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
.L0642ea:
        move.w  0x28(a6),d0                     | +08e
        move.w  0x2a(a6),d1                     | +092
        cmpi.w  #0x0,d0                         | +096
        blt.w   .L0642fc                        | +09a
        neg.w   d0                              | +09e
.L0642fc:
        asr.w   #0x4,d0                         | +0a0
        asr.w   #0x4,d1                         | +0a2
        jsr     0x5e018.l                       | +0a4
        cmpi.w  #0x60,d0                        | +0aa
        bge.w   .L064312                        | +0ae
        move.w  #0x60,d0                        | +0b2
.L064312:
        cmpi.w  #0xc0,d0                        | +0b6
        ble.w   .L06431e                        | +0ba
        move.w  #0xc0,d0                        | +0be
.L06431e:
        subi.w  #0x60,d0                        | +0c2
        lsr.w   #0x2,d0                         | +0c6
        move.w  d0,0x34(a6)                     | +0c8
        move.w  0x16(a6),d0                     | +0cc
        btst    #0x0,0x106f28.l                 | +0d0
        beq.w   .L06433c                        | +0d8
        move.w  0x18(a6),d0                     | +0dc
.L06433c:
        move.w  d0,0x14(a6)                     | +0e0
        jsr     0x28d70.l                       | +0e4
        jsr     0x283d8.l                       | +0ea
        btst    #0x1,0x13(a6)                   | +0f0
        beq.w   .L06435c                        | +0f6
        lea     TaskHandler_064238(pc),a1       | +0fa
        move.l  a1,(a6)                         | +0fe
.L06435c:
        movea.l #0xffffffff,a0                  | +100
        lea     0x2c5a5c.l,a0                   | +106
        jsr     0x5dd56.l                       | +10c
        bcc.w   SetHandlerRts_064378            | +112

| ----------------------------------------------------------------------------
|  TaskHandler_06437a  @ $06437A  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06437a, "ax", @progbits
        .global TaskHandler_06437a
TaskHandler_06437a:
        move.w  #0x83,d1                        | +000
        jsr     0x236e.l                        | +004
        move.w  #0xd000,d0                      | +00a
        jsr     0x28134.l                       | +00e
        andi.w  #0xffe3,0x38(a6)                | +014
        ori.w   #0x14,0x38(a6)                  | +01a
        lea     0x2c605e.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L0643ac(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L0643ac:
        jsr     0x2783a.l                       | +032
        jsr     0x28d70.l                       | +038
        bcc.w   .L0643c2                        | +03e
        lea     Jsr5B6ThenJmpScheduler_06422a(pc),a1 | +042
        move.l  a1,(a6)                         | +046
.L0643c2:
        movea.l #0xffffffff,a0                  | +048
        jsr     0x5dd56.l                       | +04e
        bcc.w   SetHandlerRts_0643d8            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_0643da  @ $0643DA  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0643da, "ax", @progbits
        .global TaskHandler_0643da
TaskHandler_0643da:
        subq.w  #0x1,0x72(a6)                   | +000
        cmpi.w  #0x0,0x72(a6)                   | +004
        bgt.w   ClearXN_0643ee                  | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_0643f4  @ $0643F4  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0643f4, "ax", @progbits
        .global TaskHandler_0643f4
TaskHandler_0643f4:
        move.b  0x3a(a6),0x84(a6)               | +000
        move.b  0x83(a6),0x3a(a6)               | +006
        jsr     0x5e5a8.l                       | +00c
        bcc.w   TaskHandler_064416              | +012
        move.b  0x84(a6),0x3a(a6)               | +016

| ----------------------------------------------------------------------------
|  TaskHandler_064416  @ $064416  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064416, "ax", @progbits
        .global TaskHandler_064416
TaskHandler_064416:
        move.b  0x84(a6),0x3a(a6)               | +000

| ----------------------------------------------------------------------------
|  TaskHandler_064422  @ $064422  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064422, "ax", @progbits
        .global TaskHandler_064422
TaskHandler_064422:
        jsr     TaskHandler_06443c(pc)          | +000
        btst    #0x0,0x82(a6)                   | +004
        bne.w   SetXN_064436                    | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06443c  @ $06443C  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06443c, "ax", @progbits
        .global TaskHandler_06443c
TaskHandler_06443c:
        btst    #0x0,0x82(a6)                   | +000
        bne.w   ClearXN_064480                  | +006
        move.w  0x66(a6),d0                     | +00a
        cmp.w   0x80(a6),d0                     | +00e
        bgt.w   ClearXN_064480                  | +012
        move.w  #0x1027,d0                      | +016
        jsr     0x2352.l                        | +01a
        ori.b   #0x1,0x82(a6)                   | +020
        lea     0x2c5a76.l,a1                   | +026
        jsr     0x77c7e.l                       | +02c
        lea     0x2c5a88.l,a1                   | +032
        jsr     0x77c7e.l                       | +038

| ----------------------------------------------------------------------------
|  TaskHandler_064486  @ $064486  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064486, "ax", @progbits
        .global TaskHandler_064486
TaskHandler_064486:
        btst    #0x0,0x3a(a6)                   | +000
        beq.w   ClearXN_064498                  | +006
        neg.w   d0                              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_06449e  @ $06449E  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06449e, "ax", @progbits
        .global TaskHandler_06449e
TaskHandler_06449e:
        move.w  #0x1066,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_06437a(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        eori.b  #0x1,0x3a(a0)                   | +01a
        lea     TaskHandler_06425c(pc),a1       | +020
        jsr     0x4ae.l                         | +024
        jsr     0x5dd02.l                       | +02a
        addi.w  #0x3c,0x24(a0)                  | +030
        addi.w  #0x10,0x22(a0)                  | +036
        eori.b  #0x1,0x3a(a0)                   | +03c
        move.w  0x76(a6),0x76(a0)               | +042
        rts                                     | +048

| ----------------------------------------------------------------------------
|  TaskHandler_0644e8  @ $0644E8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0644e8, "ax", @progbits
        .global TaskHandler_0644e8
TaskHandler_0644e8:
        move.w  #0x1066,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_06437a(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        lea     TaskHandler_06425c(pc),a1       | +01a
        jsr     0x4ae.l                         | +01e
        jsr     0x5dd02.l                       | +024
        addi.w  #0x3c,0x24(a0)                  | +02a
        addi.w  #0xfff0,0x22(a0)                | +030
        move.w  0x76(a6),0x76(a0)               | +036
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_064526  @ $064526  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064526, "ax", @progbits
        .global TaskHandler_064526
TaskHandler_064526:
        lea     0x2c5a64.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_064534  @ $064534  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064534, "ax", @progbits
        .global TaskHandler_064534
TaskHandler_064534:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_06454a                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_064550  @ $064550  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064550, "ax", @progbits
        .global TaskHandler_064550
TaskHandler_064550:
        bclr    #0x1,0x12(a6)                   | +000
        clr.b   0x10e27a.l                      | +006
        clr.b   0x10e27b.l                      | +00c
        lea     .L064568(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L064568:
        move.b  0x10e277.l,0x10e276.l           | +018
        move.b  0x10e279.l,0x10e278.l           | +022
        clr.b   0x10e277.l                      | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_06458a  @ $06458A  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06458a, "ax", @progbits
        .global TaskHandler_06458a
TaskHandler_06458a:
        move.b  #0x34,0x10e27a.l                | +000
        move.b  #0x35,0x10e27b.l                | +008

| ----------------------------------------------------------------------------
|  TaskHandler_0645a8  @ $0645A8  (152 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0645a8, "ax", @progbits
        .global TaskHandler_0645a8
TaskHandler_0645a8:
        ori.b   #0x1,0x3a(a6)                   | +000
        move.b  #0x1,0x79(a6)                   | +006
        move.w  #0x44,d1                        | +00c
        jsr     0x236e.l                        | +010
        jsr     0x5e7c0.l                       | +016
        lea     0x2b958c.l,a0                   | +01c
        jsr     0x799de.l                       | +022
        move.w  d0,0x66(a6)                     | +028
        move.b  #0x0,0x78(a6)                   | +02c
        move.w  #0x4000,d0                      | +032
        jsr     0x28134.l                       | +036
        andi.w  #0xffe3,0x38(a6)                | +03c
        ori.w   #0x14,0x38(a6)                  | +042
        clr.b   0x21(a6)                        | +048
        bra.w   Sub_00064640__L064682           | +04c
        ori.b   #0x1,0x3a(a6)                   | +050
        move.b  #0x0,0x79(a6)                   | +056
        jsr     Sub_00064640(pc)                | +05c
        lea     TaskHandler_06504e(pc),a1       | +060
        jsr     0x4ae.l                         | +064
        jsr     0x5dd02.l                       | +06a
        bra.w   TaskHandler_064ad8              | +070
        ori.b   #0x1,0x3a(a6)                   | +074
        move.b  #0x1,0x79(a6)                   | +07a
        bra.w   .L064638                        | +080
        ori.b   #0x1,0x3a(a6)                   | +084
        move.b  #0x0,0x79(a6)                   | +08a
.L064638:
        jsr     Sub_00064640(pc)                | +090
        bra.w   Sub_00064640__L064682           | +094

| ----------------------------------------------------------------------------
|  Sub_00064640  @ $064640  (184 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00064640, "ax", @progbits
        .global Sub_00064640
Sub_00064640:
        move.w  #0x146,d1                       | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e7c0.l                       | +00a
        lea     0x2b999c.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.w  d0,0x66(a6)                     | +01c
        move.b  #0x1,0x78(a6)                   | +020
        move.w  #0x8000,d0                      | +026
        jsr     0x28134.l                       | +02a
        andi.w  #0xffe3,0x38(a6)                | +030
        ori.w   #0x14,0x38(a6)                  | +036
        clr.b   0x21(a6)                        | +03c
        rts                                     | +040
        .global Sub_00064640__L064682
Sub_00064640__L064682:
.L064682:
        move.b  0x9a(a6),d0                     | +042
        andi.w  #0x1f,d0                        | +046
        lsl.w   #0x4,d0                         | +04a
        move.w  d0,0x80(a6)                     | +04c
        move.b  0x99(a6),d0                     | +050
        andi.w  #0xff,d0                        | +054
        lsl.w   #0x8,d0                         | +058
        move.w  d0,0x84(a6)                     | +05a
        move.w  #0x0,0x8e(a6)                   | +05e
        lea     .L0646aa(pc),a1                 | +064
        move.l  a1,(a6)                         | +068
.L0646aa:
        cmpi.b  #0x0,0x98(a6)                   | +06a
        bne.w   .L0646ba                        | +070
        lea     TaskHandler_064700(pc),a1       | +074
        move.l  a1,(a6)                         | +078
.L0646ba:
        jsr     TaskHandler_065d4a(pc)          | +07a
        cmp.b   0x98(a6),d0                     | +07e
        bge.w   .L0646da                        | +082
        lea     TaskHandler_064700(pc),a1       | +086
        move.l  a1,(a6)                         | +08a
        jsr     TaskHandler_065d7a(pc)          | +08c
        move.w  #0x105d,d0                      | +090
        jsr     0x2352.l                        | +094
.L0646da:
        cmpi.b  #0x34,0x10e27a.l                | +09a
        bne.w   .L0646ec                        | +0a2
        lea     TaskHandler_064d7a(pc),a1       | +0a6
        move.l  a1,(a6)                         | +0aa
.L0646ec:
        cmpi.b  #0x35,0x10e27b.l                | +0ac
        bne.w   SetHandlerRts_0646fe            | +0b4

| ----------------------------------------------------------------------------
|  TaskHandler_064700  @ $064700  (192 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064700, "ax", @progbits
        .global TaskHandler_064700
TaskHandler_064700:
        cmpi.b  #0x1,0x79(a6)                   | +000
        bne.w   .L06471a                        | +006
        lea     TaskHandler_064d98(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
.L06471a:
        move.l  #0x2c60c4,d0                    | +01a
        tst.b   0x78(a6)                        | +020
        beq.w   .L06472e                        | +024
        move.l  #0x2c6118,d0                    | +028
.L06472e:
        move.l  d0,0x48(a6)                     | +02e
        cmpi.b  #0x1,0x78(a6)                   | +032
        beq.w   TaskHandler_06498e              | +038
        move.w  #0x1,0x8e(a6)                   | +03c
        lea     0x2b960e.l,a0                   | +042
        jsr     0x799de.l                       | +048
        jsr     Entity_MirrorDeltaByFacing_065D32(pc) | +04e
        neg.w   d0                              | +052
        move.w  d0,0x28(a6)                     | +054
        move.w  d0,0x82(a6)                     | +058
        lea     0x2c661c.l,a0                   | +05c
        jsr     0x28cd4.l                       | +062
        cmpi.b  #0x1,0x78(a6)                   | +068
        bne.w   .L06477e                        | +06e
        lea     0x2c679a.l,a0                   | +072
        jsr     0x28cd4.l                       | +078
.L06477e:
        lea     .L064784(pc),a1                 | +07e
        move.l  a1,(a6)                         | +082
.L064784:
        jsr     0x27afc.l                       | +084
        bcc.w   .L064794                        | +08a
        lea     TaskHandler_0647c0(pc),a1       | +08e
        move.l  a1,(a6)                         | +092
.L064794:
        jsr     0x28d70.l                       | +094
        jsr     TaskHandler_065c02(pc)          | +09a
        bcc.w   .L0647a8                        | +09e
        lea     TaskHandler_0647c0(pc),a1       | +0a2
        move.l  a1,(a6)                         | +0a6
.L0647a8:
        subq.w  #0x1,0x84(a6)                   | +0a8
        cmpi.w  #0x0,0x84(a6)                   | +0ac
        bgt.w   .L0647bc                        | +0b2
        lea     TaskHandler_0647c0(pc),a1       | +0b6
        move.l  a1,(a6)                         | +0ba
.L0647bc:
        bra.w   TaskHandler_064a80              | +0bc

| ----------------------------------------------------------------------------
|  TaskHandler_0647c0  @ $0647C0  (186 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0647c0, "ax", @progbits
        .global TaskHandler_0647c0
TaskHandler_0647c0:
        move.w  #0x1060,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x0,0x8e(a6)                   | +00a
        cmpi.b  #0x1,0x78(a6)                   | +010
        beq.w   .L0647f6                        | +016
        move.w  0x28(a6),d0                     | +01a
        asr.w   #0x5,d0                         | +01e
        neg.w   d0                              | +020
        move.w  d0,0x2c(a6)                     | +022
        lea     0x2c6714.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        bra.w   .L06480e                        | +032
.L0647f6:
        move.w  0x28(a6),d0                     | +036
        asr.w   #0x5,d0                         | +03a
        neg.w   d0                              | +03c
        move.w  d0,0x2c(a6)                     | +03e
        lea     0x2c6860.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
.L06480e:
        lea     .L064814(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L064814:
        jsr     0x27afc.l                       | +054
        cmpi.w  #0x0,0x28(a6)                   | +05a
        bne.w   .L064828                        | +060
        clr.w   0x2c(a6)                        | +064
.L064828:
        move.b  0x106f28.l,d0                   | +068
        andi.b  #0x7,d0                         | +06e
        bne.w   .L06485c                        | +072
        lea     TaskHandler_065aa4(pc),a1       | +076
        jsr     0x4ae.l                         | +07a
        jsr     0x5dd02.l                       | +080
        move.w  #0x18,d0                        | +086
        jsr     Entity_MirrorDeltaByFacing_065D32(pc) | +08a
        add.w   d0,0x22(a0)                     | +08e
        move.w  0x2c(a6),0x28(a0)               | +092
        addq.w  #0x1,0x38(a0)                   | +098
.L06485c:
        jsr     0x28d70.l                       | +09c
        bcc.w   .L064876                        | +0a2
        cmpi.w  #0x0,0x28(a6)                   | +0a6
        bne.w   .L064876                        | +0ac
        lea     TaskHandler_06487a(pc),a1       | +0b0
        move.l  a1,(a6)                         | +0b4
.L064876:
        bra.w   TaskHandler_064a80              | +0b6

| ----------------------------------------------------------------------------
|  TaskHandler_06487a  @ $06487A  (190 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06487a, "ax", @progbits
        .global TaskHandler_06487a
TaskHandler_06487a:
        move.w  #0x0,0x8e(a6)                   | +000
        jsr     0x267e2.l                       | +006
        cmpi.b  #0x1,0x78(a6)                   | +00c
        beq.w   .L0648f2                        | +012
        move.w  #0xa,0x70(a6)                   | +016
        lea     0x2c674c.l,a0                   | +01c
        jsr     0x28cd4.l                       | +022
        lea     .L0648a8(pc),a1                 | +028
        move.l  a1,(a6)                         | +02c
.L0648a8:
        subq.w  #0x1,0x70(a6)                   | +02e
        cmpi.w  #0x0,0x70(a6)                   | +032
        bgt.w   .L0648c2                        | +038
        move.b  #0x8,0x20(a6)                   | +03c
        lea     .L0648c2(pc),a1                 | +042
        move.l  a1,(a6)                         | +046
.L0648c2:
        jsr     0x2783a.l                       | +048
        jsr     0x28d70.l                       | +04e
        cmpi.b  #0x4,0x21(a6)                   | +054
        bne.w   .L0648de                        | +05a
        lea     TaskHandler_064938(pc),a1       | +05e
        move.l  a1,(a6)                         | +062
.L0648de:
        cmpi.w  #0x0,0x22(a6)                   | +064
        bgt.w   .L0648ee                        | +06a
        lea     TaskHandler_064d7a(pc),a1       | +06e
        move.l  a1,(a6)                         | +072
.L0648ee:
        bra.w   TaskHandler_064a80              | +074
.L0648f2:
        lea     0x2b9aa0.l,a0                   | +078
        jsr     0x799de.l                       | +07e
        move.w  d0,0x70(a6)                     | +084
        lea     0x2c68e8.l,a0                   | +088
        jsr     0x28cd4.l                       | +08e
        lea     .L064914(pc),a1                 | +094
        move.l  a1,(a6)                         | +098
.L064914:
        jsr     0x2783a.l                       | +09a
        jsr     0x28d70.l                       | +0a0
        subq.w  #0x1,0x70(a6)                   | +0a6
        cmpi.w  #0x0,0x70(a6)                   | +0aa
        bgt.w   .L064934                        | +0b0
        lea     TaskHandler_064938(pc),a1       | +0b4
        move.l  a1,(a6)                         | +0b8
.L064934:
        bra.w   TaskHandler_064a80              | +0ba

| ----------------------------------------------------------------------------
|  TaskHandler_064938  @ $064938  (86 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064938, "ax", @progbits
        .global TaskHandler_064938
TaskHandler_064938:
        move.w  #0x105e,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x0,0x8e(a6)                   | +00a
        cmpi.b  #0x1,0x78(a6)                   | +010
        beq.w   .L064962                        | +016
        lea     0x2c675c.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        bra.w   .L06496e                        | +026
.L064962:
        lea     0x2c6906.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
.L06496e:
        lea     .L064974(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L064974:
        jsr     0x27afc.l                       | +03c
        jsr     0x28d70.l                       | +042
        bcc.w   .L06498a                        | +048
        lea     TaskHandler_06498e(pc),a1       | +04c
        move.l  a1,(a6)                         | +050
.L06498a:
        bra.w   TaskHandler_064a80              | +052

| ----------------------------------------------------------------------------
|  TaskHandler_06498e  @ $06498E  (150 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06498e, "ax", @progbits
        .global TaskHandler_06498e
TaskHandler_06498e:
        lea     TaskHandler_0650fe(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        lea     0x2b9a1e.l,a0                   | +010
        jsr     0x799de.l                       | +016
        jsr     Entity_MirrorDeltaByFacing_065D32(pc) | +01c
        neg.w   d0                              | +020
        move.w  d0,0x82(a6)                     | +022
        move.l  #0x2c6310,0x4c(a6)              | +026
        move.w  #0x1,0x8e(a6)                   | +02e
        lea     0x2c664e.l,a0                   | +034
        jsr     0x28cd4.l                       | +03a
        cmpi.b  #0x1,0x78(a6)                   | +040
        bne.w   .L0649e4                        | +046
        lea     0x2c67cc.l,a0                   | +04a
        jsr     0x28cd4.l                       | +050
.L0649e4:
        lea     .L0649ea(pc),a1                 | +056
        move.l  a1,(a6)                         | +05a
.L0649ea:
        jsr     0x27afc.l                       | +05c
        bcc.w   .L0649fa                        | +062
        lea     TaskHandler_064a24(pc),a1       | +066
        move.l  a1,(a6)                         | +06a
.L0649fa:
        jsr     TaskHandler_065c30(pc)          | +06c
        jsr     0x28d70.l                       | +070
        jsr     0x283ca.l                       | +076
        jsr     0x283d8.l                       | +07c
        btst    #0x1,0x13(a6)                   | +082
        beq.w   .L064a20                        | +088
        lea     TaskHandler_064a24(pc),a1       | +08c
        move.l  a1,(a6)                         | +090
.L064a20:
        bra.w   TaskHandler_064a80              | +092

| ----------------------------------------------------------------------------
|  TaskHandler_064a24  @ $064A24  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064a24, "ax", @progbits
        .global TaskHandler_064a24
TaskHandler_064a24:
        move.w  #0x1021,d0                      | +000
        jsr     0x2352.l                        | +004
        .global TaskHandler_064a24__L064a2e
TaskHandler_064a24__L064a2e:
.L064a2e:
        move.w  #0x0,0x8e(a6)                   | +00a
        bclr    #0x1,0x12(a6)                   | +010
        lea     0x2c66e2.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L064a4c(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L064a4c:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L064a62                        | +034
        lea     TaskHandler_064d7a(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L064a62:
        movea.l #0xffffffff,a0                  | +03e
        lea     0x2c63b8.l,a0                   | +044
        jsr     0x5dd5c.l                       | +04a
        bcc.w   SetHandlerRts_064a7e            | +050

| ----------------------------------------------------------------------------
|  TaskHandler_064a80  @ $064A80  (80 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064a80, "ax", @progbits
        .global TaskHandler_064a80
TaskHandler_064a80:
        jsr     0x27eba.l                       | +000
        bcc.w   .L064a90                        | +006
        lea     TaskHandler_064a24(pc),a1       | +00a
        move.l  a1,(a6)                         | +00e
        .global TaskHandler_064a80__L064a90
TaskHandler_064a80__L064a90:
.L064a90:
        clr.b   0x21(a6)                        | +010
        jsr     TaskHandler_065d7a(pc)          | +014
        lea     0x5e766.l,a0                    | +018
        jsr     0x5e770.l                       | +01e
        jsr     0x28758.l                       | +024
        bcc.w   .L064ab4                        | +02a
        lea     TaskHandler_064a24(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L064ab4:
        bclr    #0x3,0x13(a6)                   | +034
        movea.l #0xffffffff,a0                  | +03a
        lea     0x2c63b8.l,a0                   | +040
        jsr     0x5dd5c.l                       | +046
        bcc.w   SetHandlerRts_064ad6            | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_064ad8  @ $064AD8  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064ad8, "ax", @progbits
        .global TaskHandler_064ad8
TaskHandler_064ad8:
        lea     .L064ade(pc),a1                 | +000
        move.l  a1,(a6)                         | +004
.L064ade:
        cmpi.b  #0x0,0x98(a6)                   | +006
        bne.w   .L064aee                        | +00c
        lea     TaskHandler_064b18(pc),a1       | +010
        move.l  a1,(a6)                         | +014
.L064aee:
        jsr     TaskHandler_065d4a(pc)          | +016
        cmp.b   0x98(a6),d0                     | +01a
        bge.w   .L064b04                        | +01e
        lea     TaskHandler_064b18(pc),a1       | +022
        move.l  a1,(a6)                         | +026
        jsr     TaskHandler_065d7a(pc)          | +028
.L064b04:
        cmpi.b  #0x35,0x10e27b.l                | +02c
        bne.w   SetHandlerRts_064b16            | +034

| ----------------------------------------------------------------------------
|  TaskHandler_064b18  @ $064B18  (174 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064b18, "ax", @progbits
        .global TaskHandler_064b18
TaskHandler_064b18:
        move.l  #0x2c6118,0x48(a6)              | +000
        move.w  #0x105e,d0                      | +008
        jsr     0x2352.l                        | +00c
        move.b  0x9a(a6),d0                     | +012
        andi.w  #0x1f,d0                        | +016
        lsl.w   #0x4,d0                         | +01a
        move.w  d0,0x80(a6)                     | +01c
        lea     0x2b960e.l,a0                   | +020
        jsr     0x799de.l                       | +026
        jsr     Entity_MirrorDeltaByFacing_065D32(pc) | +02c
        neg.w   d0                              | +030
        move.w  d0,0x28(a6)                     | +032
        move.w  d0,0x82(a6)                     | +036
        lea     0x2c6962.l,a0                   | +03a
        jsr     0x28cd4.l                       | +040
        lea     .L064b64(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L064b64:
        jsr     0x27afc.l                       | +04c
        bcc.w   .L064b74                        | +052
        lea     TaskHandler_064bc6(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L064b74:
        jsr     TaskHandler_065c30(pc)          | +05c
        jsr     0x28d70.l                       | +060
        move.w  0x22(a6),d0                     | +066
        btst    #0x0,0x3a(a6)                   | +06a
        beq.w   .L064b98                        | +070
        cmp.w   0x80(a6),d0                     | +074
        blt.w   .L064ba6                        | +078
        bra.w   .L064ba0                        | +07c
.L064b98:
        cmp.w   0x80(a6),d0                     | +080
        bgt.w   .L064ba6                        | +084
.L064ba0:
        lea     TaskHandler_064bc6(pc),a1       | +088
        move.l  a1,(a6)                         | +08c
        .global TaskHandler_064b18__L064ba6
TaskHandler_064b18__L064ba6:
.L064ba6:
        jsr     0x283ca.l                       | +08e
        jsr     0x283d8.l                       | +094
        btst    #0x1,0x13(a6)                   | +09a
        beq.w   .L064bc2                        | +0a0
        lea     TaskHandler_064d10(pc),a1       | +0a4
        move.l  a1,(a6)                         | +0a8
.L064bc2:
        bra.w   TaskHandler_064a80__L064a90     | +0aa

| ----------------------------------------------------------------------------
|  TaskHandler_064bc6  @ $064BC6  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064bc6, "ax", @progbits
        .global TaskHandler_064bc6
TaskHandler_064bc6:
        move.l  #0x2c6364,0x4c(a6)              | +000
        lea     0x2c6990.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L064be0(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L064be0:
        jsr     0x27cee.l                       | +01a
        jsr     TaskHandler_065c30(pc)          | +020
        jsr     0x28d70.l                       | +024
        bcc.w   .L064bfa                        | +02a
        lea     TaskHandler_064bfc(pc),a1       | +02e
        move.l  a1,(a6)                         | +032
.L064bfa:
        bra.b   TaskHandler_064b18__L064ba6     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_064bfc  @ $064BFC  (234 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064bfc, "ax", @progbits
        .global TaskHandler_064bfc
TaskHandler_064bfc:
        move.w  #0x105c,d0                      | +000
        jsr     0x2352.l                        | +004
        lea     TaskHandler_0651be(pc),a1       | +00a
        jsr     0x4ae.l                         | +00e
        jsr     0x5dd02.l                       | +014
        move.w  #0x18,d0                        | +01a
        btst    #0x0,0x3a(a6)                   | +01e
        beq.w   .L064c26                        | +024
        neg.w   d0                              | +028
.L064c26:
        add.w   d0,0x22(a0)                     | +02a
        addq.w  #0x8,0x24(a0)                   | +02e
        move.w  0x28(a6),d0                     | +032
        asr.w   #0x1,d0                         | +036
        move.w  d0,0x28(a0)                     | +038
        move.w  #0xaa7,0x2a(a6)                 | +03c
        move.w  #0xfed1,0x2e(a6)                | +042
        move.w  0x28(a6),d0                     | +048
        asr.w   #0x2,d0                         | +04c
        move.w  d0,0x28(a6)                     | +04e
        addi.w  #0x20,0x24(a6)                  | +052
        move.w  #0xd000,0x38(a6)                | +058
        lea     0x2c69ee.l,a0                   | +05e
        jsr     0x28cd4.l                       | +064
        lea     .L064c6c(pc),a1                 | +06a
        move.l  a1,(a6)                         | +06e
.L064c6c:
        jsr     0x27d50.l                       | +070
        bcc.w   .L064c7c                        | +076
        lea     TaskHandler_064cee(pc),a1       | +07a
        move.l  a1,(a6)                         | +07e
.L064c7c:
        jsr     0x28d70.l                       | +080
        cmpi.w  #0x120,0x24(a6)                 | +086
        bgt.w   .L064c92                        | +08c
        lea     TaskHandler_064cee(pc),a1       | +090
        move.l  a1,(a6)                         | +094
.L064c92:
        jsr     0x283ca.l                       | +096
        jsr     0x283d8.l                       | +09c
        btst    #0x1,0x13(a6)                   | +0a2
        beq.w   .L064cae                        | +0a8
        lea     TaskHandler_064d1e(pc),a1       | +0ac
        move.l  a1,(a6)                         | +0b0
.L064cae:
        bclr    #0x3,0x13(a6)                   | +0b2
        lea     0x5e766.l,a0                    | +0b8
        jsr     0x5e770.l                       | +0be
        jsr     0x28758.l                       | +0c4
        bcc.w   .L064cd0                        | +0ca
        lea     TaskHandler_064d1e(pc),a1       | +0ce
        move.l  a1,(a6)                         | +0d2
.L064cd0:
        movea.l #0xffffffff,a0                  | +0d4
        lea     0x2c63b8.l,a0                   | +0da
        jsr     0x5dd5c.l                       | +0e0
        bcc.w   SetHandlerRts_064cec            | +0e6

| ----------------------------------------------------------------------------
|  TaskHandler_064cee  @ $064CEE  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064cee, "ax", @progbits
        .global TaskHandler_064cee
TaskHandler_064cee:
        move.w  #0x120,0x24(a6)                 | +000
        move.w  #0xf000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x1c,0x38(a6)                  | +016
        jmp     0x78890.l                       | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_064d10  @ $064D10  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064d10, "ax", @progbits
        .global TaskHandler_064d10
TaskHandler_064d10:
        move.w  #0x1023,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   TaskHandler_064a24__L064a2e     | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_064d1e  @ $064D1E  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064d1e, "ax", @progbits
        .global TaskHandler_064d1e
TaskHandler_064d1e:
        move.w  #0x1023,d0                      | +000
        jsr     0x2352.l                        | +004
        move.w  #0x0,0x8e(a6)                   | +00a
        bclr    #0x1,0x12(a6)                   | +010
        lea     0x2c66fe.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L064d46(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L064d46:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L064d5c                        | +034
        lea     TaskHandler_064d7a(pc),a1       | +038
        move.l  a1,(a6)                         | +03c
.L064d5c:
        movea.l #0xffffffff,a0                  | +03e
        lea     0x2c63b8.l,a0                   | +044
        jsr     0x5dd5c.l                       | +04a
        bcc.w   SetHandlerRts_064d78            | +050

| ----------------------------------------------------------------------------
|  TaskHandler_064d7a  @ $064D7A  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064d7a, "ax", @progbits
        .global TaskHandler_064d7a
TaskHandler_064d7a:
        move.l  #0xffffffff,0x48(a6)            | +000
        jmp     0x518.l                         | +008

| ----------------------------------------------------------------------------
|  TaskHandler_064d88  @ $064D88  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064d88, "ax", @progbits
        .global TaskHandler_064d88
TaskHandler_064d88:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_064d98  @ $064D98  (38 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064d98, "ax", @progbits
        .global TaskHandler_064d98
TaskHandler_064d98:
        lea     0x2b9816.l,a0                   | +000
        jsr     0x799de.l                       | +006
        move.w  d0,0x74(a6)                     | +00c
        move.w  #0x8,0x72(a6)                   | +010
        move.b  #0x0,0x20(a6)                   | +016
        move.w  #0x45,d1                        | +01c
        jsr     0x236e.l                        | +020

| ----------------------------------------------------------------------------
|  TaskHandler_064dbe  @ $064DBE  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064dbe, "ax", @progbits
        .global TaskHandler_064dbe
TaskHandler_064dbe:
        move.b  0x20(a6),d0                     | +000
        andi.w  #0xf,d0                         | +004
        movea.l #0x2c6510,a0                    | +008
        lsl.w   #0x2,d0                         | +00e
        movea.l (a0,d0.w),a0                    | +010
        cmpa.l  #0xffffffff,a0                  | +014
        beq.w   .L064de2                        | +01a
        jsr     0x28cd4.l                       | +01e
.L064de2:
        move.b  #0x1,0x21(a6)                   | +024
        lea     .L064dee(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L064dee:
        jsr     TaskHandler_065bb0(pc)          | +030
        cmpi.w  #0x8,d0                         | +034
        beq.w   TaskHandler_064e1a__L064e8e     | +038
        cmpi.w  #0x9,d0                         | +03c
        beq.w   TaskHandler_064f8e__L065020     | +040
        jsr     0x28d70.l                       | +044
        jsr     TaskHandler_065af4(pc)          | +04a
        bcc.w   .L064e16                        | +04e
        lea     TaskHandler_064e1a(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L064e16:
        bra.w   TaskHandler_06503c              | +058

| ----------------------------------------------------------------------------
|  TaskHandler_064e1a  @ $064E1A  (178 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064e1a, "ax", @progbits
        .global TaskHandler_064e1a
TaskHandler_064e1a:
        subq.w  #0x1,0x74(a6)                   | +000
        cmpi.w  #0x0,0x74(a6)                   | +004
        bgt.w   .L064e4c                        | +00a
        lea     0x2b9816.l,a0                   | +00e
        jsr     0x799de.l                       | +014
        move.w  d0,0x74(a6)                     | +01a
        lea     0x2b9898.l,a0                   | +01e
        jsr     0x799de.l                       | +024
        move.w  d0,0x72(a6)                     | +02a
        bra.w   .L064e5c                        | +02e
.L064e4c:
        lea     0x2b991a.l,a0                   | +032
        jsr     0x799de.l                       | +038
        move.w  d0,0x72(a6)                     | +03e
.L064e5c:
        lea     0x2c6c58.l,a0                   | +042
        jsr     0x28cd4.l                       | +048
        lea     .L064e6e(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L064e6e:
        jsr     TaskHandler_065bb0(pc)          | +054
        cmpi.w  #0x9,d0                         | +058
        beq.w   TaskHandler_064f8e__L065020     | +05c
        jsr     0x28d70.l                       | +060
        bcc.w   .L064e8a                        | +066
        lea     TaskHandler_064dbe(pc),a1       | +06a
        move.l  a1,(a6)                         | +06e
.L064e8a:
        bra.w   TaskHandler_06503c              | +070
        .global TaskHandler_064e1a__L064e8e
TaskHandler_064e1a__L064e8e:
.L064e8e:
        move.w  #0x0,0x88(a6)                   | +074
        move.b  #0x2,0x21(a6)                   | +07a
        lea     0x2c6c88.l,a0                   | +080
        jsr     0x28cd4.l                       | +086
        lea     .L064eac(pc),a1                 | +08c
        move.l  a1,(a6)                         | +090
.L064eac:
        jsr     TaskHandler_065bb0(pc)          | +092
        cmpi.w  #0x9,d0                         | +096
        beq.w   TaskHandler_064f8e__L065020     | +09a
        jsr     0x28d70.l                       | +09e
        bcc.w   .L064ec8                        | +0a4
        lea     TaskHandler_064ecc(pc),a1       | +0a8
        move.l  a1,(a6)                         | +0ac
.L064ec8:
        bra.w   TaskHandler_06503c              | +0ae

| ----------------------------------------------------------------------------
|  TaskHandler_064ecc  @ $064ECC  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064ecc, "ax", @progbits
        .global TaskHandler_064ecc
TaskHandler_064ecc:
        move.w  #0x6,0x72(a6)                   | +000
        bra.w   .L064ee6                        | +006
        .global TaskHandler_064ecc__L064ed6
TaskHandler_064ecc__L064ed6:
.L064ed6:
        lea     0x2b9712.l,a0                   | +00a
        jsr     0x799de.l                       | +010
        move.w  d0,0x72(a6)                     | +016
.L064ee6:
        lea     0x2b9690.l,a0                   | +01a
        jsr     0x799de.l                       | +020
        move.w  d0,0x74(a6)                     | +026
        clr.w   0x84(a6)                        | +02a
        bra.w   TaskHandler_064efe__L064f1e     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_064efe  @ $064EFE  (144 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064efe, "ax", @progbits
        .global TaskHandler_064efe
TaskHandler_064efe:
        addq.w  #0x1,0x84(a6)                   | +000
        subq.w  #0x1,0x74(a6)                   | +004
        cmpi.w  #0x0,0x74(a6)                   | +008
        ble.b   TaskHandler_064ecc__L064ed6     | +00e
        lea     0x2b9794.l,a0                   | +010
        jsr     0x799de.l                       | +016
        move.w  d0,0x72(a6)                     | +01c
        .global TaskHandler_064efe__L064f1e
TaskHandler_064efe__L064f1e:
.L064f1e:
        move.b  #0x3,0x21(a6)                   | +020
        lea     0x2c6c9e.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L064f36(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L064f36:
        jsr     TaskHandler_065bb0(pc)          | +038
        cmpi.w  #0x9,d0                         | +03c
        beq.w   TaskHandler_064f8e__L065020     | +040
        jsr     TaskHandler_065b36(pc)          | +044
        bcc.w   .L064f6c                        | +048
        subq.w  #0x1,0x72(a6)                   | +04c
        cmpi.w  #0x0,0x72(a6)                   | +050
        bgt.w   .L064f6c                        | +056
        clr.w   0x72(a6)                        | +05a
        cmpi.w  #0x10,0x22(a6)                  | +05e
        blt.w   .L064f6c                        | +064
        lea     TaskHandler_064f8e(pc),a1       | +068
        move.l  a1,(a6)                         | +06c
.L064f6c:
        cmpi.w  #0x8,0x88(a6)                   | +06e
        bcs.w   .L064f84                        | +074
        nop                                     | +078
        nop                                     | +07a
        cmpi.w  #0x8,0x88(a6)                   | +07c
        nop                                     | +082
        trap    #0xf                            | +084
.L064f84:
        jsr     0x28d70.l                       | +086
        bra.w   TaskHandler_06503c              | +08c

| ----------------------------------------------------------------------------
|  TaskHandler_064f8e  @ $064F8E  (166 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_064f8e, "ax", @progbits
        .global TaskHandler_064f8e
TaskHandler_064f8e:
        move.w  0x88(a6),d0                     | +000
        lea     0x2c65ac.l,a0                   | +004
        andi.w  #0x7,d0                         | +00a
        add.w   d0,d0                           | +00e
        move.w  (a0,d0.w),d0                    | +010
        cmpi.w  #0xffff,d0                      | +014
        bne.w   .L064fb6                        | +018
        nop                                     | +01c
        nop                                     | +01e
        cmpi.w  #0xffff,d0                      | +020
        nop                                     | +024
        trap    #0xf                            | +026
.L064fb6:
        movea.l #0x2c6550,a0                    | +028
        lsl.w   #0x2,d0                         | +02e
        movea.l (a0,d0.w),a0                    | +030
        cmpa.l  #0xffffffff,a0                  | +034
        beq.w   .L064fd2                        | +03a
        jsr     0x28cd4.l                       | +03e
.L064fd2:
        lea     .L064fd8(pc),a1                 | +044
        move.l  a1,(a6)                         | +048
.L064fd8:
        jsr     TaskHandler_065bb0(pc)          | +04a
        cmpi.w  #0x9,d0                         | +04e
        beq.w   .L065020                        | +052
        jsr     0x28d70.l                       | +056
        bcc.w   .L064ff4                        | +05c
        lea     TaskHandler_064efe(pc),a1       | +060
        move.l  a1,(a6)                         | +064
.L064ff4:
        bra.w   TaskHandler_06503c              | +066
        move.b  #0x4,0x21(a6)                   | +06a
        jsr     TaskHandler_065bb0(pc)          | +070
        cmpi.w  #0x9,d0                         | +074
        beq.w   .L065020                        | +078
        jsr     0x28d70.l                       | +07c
        lea     TaskHandler_064dbe(pc),a1       | +082
        move.l  a1,(a6)                         | +086
        move.b  0x4.w,0x20(a6)                  | +088
        bra.w   TaskHandler_06503c              | +08e
        .global TaskHandler_064f8e__L065020
TaskHandler_064f8e__L065020:
.L065020:
        jsr     TaskHandler_065bb0(pc)          | +092
        lea     TaskHandler_06546a(pc),a1       | +096
        jsr     0x4ae.l                         | +09a
        jsr     0x5dd02.l                       | +0a0

| ----------------------------------------------------------------------------
|  TaskHandler_06503c  @ $06503C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06503c, "ax", @progbits
        .global TaskHandler_06503c
TaskHandler_06503c:
        jsr     0x5e45a.l                       | +000
        bcc.w   SetHandlerRts_06504c            | +006

| ----------------------------------------------------------------------------
|  TaskHandler_06504e  @ $06504E  (98 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06504e, "ax", @progbits
        .global TaskHandler_06504e
TaskHandler_06504e:
        move.w  #0x10,d1                        | +000
        jsr     0x236e.l                        | +004
        lea     0x2c7514.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L06506a(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L06506a:
        jsr     0x5e45a.l                       | +01c
        bcc.w   .L06507a                        | +022
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +026
        move.l  a1,(a6)                         | +02a
.L06507a:
        jsr     0x5e506.l                       | +02c
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0x9,0x20(a0)                   | +036
        bne.w   .L065094                        | +03c
        lea     TaskHandler_0650b8(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L065094:
        jsr     0x28d70.l                       | +046
        movea.l #0xffffffff,a0                  | +04c
        lea     0x2c63b8.l,a0                   | +052
        jsr     0x5dd5c.l                       | +058
        bcc.w   SetHandlerRts_0650b6            | +05e

| ----------------------------------------------------------------------------
|  TaskHandler_0650b8  @ $0650B8  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0650b8, "ax", @progbits
        .global TaskHandler_0650b8
TaskHandler_0650b8:
        lea     0x2c7542.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L0650ca(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L0650ca:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L0650e0                        | +01e
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +022
        move.l  a1,(a6)                         | +026
.L0650e0:
        movea.l #0xffffffff,a0                  | +028
        lea     0x2c63b8.l,a0                   | +02e
        jsr     0x5dd5c.l                       | +034
        bcc.w   SetHandlerRts_0650fc            | +03a

| ----------------------------------------------------------------------------
|  TaskHandler_0650fe  @ $0650FE  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0650fe, "ax", @progbits
        .global TaskHandler_0650fe
TaskHandler_0650fe:
        move.w  #0x14,d0                        | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L06510e                        | +00a
        neg.w   d0                              | +00e
.L06510e:
        move.w  d0,0x90(a6)                     | +010
        lea     .L065118(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L065118:
        move.b  0x106f28.l,d0                   | +01a
        andi.b  #0x3,d0                         | +020
        bne.w   .L06514c                        | +024
        jsr     0x5e506.l                       | +028
        move.w  0x90(a6),d0                     | +02e
        add.w   d0,0x22(a6)                     | +032
        lea     0x77f46.l,a1                    | +036
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        move.w  #0x4000,0x38(a0)                | +048
.L06514c:
        jsr     0x5e45a.l                       | +04e
        bcc.w   SetHandlerRts_06515c            | +054

| ----------------------------------------------------------------------------
|  TaskHandler_065166  @ $065166  (88 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065166, "ax", @progbits
        .global TaskHandler_065166
TaskHandler_065166:
        move.w  #0x1022,d0                      | +000
        jsr     0x2352.l                        | +004
        bra.w   .L065194                        | +00a
        .global TaskHandler_065166__L065174
TaskHandler_065166__L065174:
.L065174:
        move.w  #0x1022,d0                      | +00e
        jsr     0x2352.l                        | +012
        .global TaskHandler_065166__L06517e
TaskHandler_065166__L06517e:
.L06517e:
        move.w  #0x4000,d0                      | +018
        jsr     0x28134.l                       | +01c
        andi.w  #0xffe3,0x38(a6)                | +022
        ori.w   #0x0,0x38(a6)                   | +028
        .global TaskHandler_065166__L065194
TaskHandler_065166__L065194:
.L065194:
        jsr     0x13600.l                       | +02e
        move.w  #0x4000,d0                      | +034
        jsr     0x28134.l                       | +038
        andi.w  #0xffe3,0x38(a6)                | +03e
        ori.w   #0x1c,0x38(a6)                  | +044
        move.l  #0xffffffff,0x48(a6)            | +04a
        jmp     0x77f6a.l                       | +052

| ----------------------------------------------------------------------------
|  TaskHandler_0651be  @ $0651BE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0651be, "ax", @progbits
        .global TaskHandler_0651be
TaskHandler_0651be:
        move.w  #0x32a,0x2a(a6)                 | +000
        move.w  #0xffaf,0x2e(a6)                | +006
        neg.w   0x28(a6)                        | +00c
        move.w  #0xe,d1                         | +010
        jsr     0x236e.l                        | +014
        lea     0x58fe2.l,a1                    | +01a
        move.l  a1,(a6)                         | +020
        jmp     0x58fe2.l                       | +022

| ----------------------------------------------------------------------------
|  TaskHandler_0651e6  @ $0651E6  (142 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0651e6, "ax", @progbits
        .global TaskHandler_0651e6
TaskHandler_0651e6:
        move.w  #0xa1,d1                        | +000
        jsr     0x236e.l                        | +004
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0x1,d0                         | +010
        beq.w   .L06520e                        | +014
        lea     0x29c8d6.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
        bra.w   .L06521a                        | +024
.L06520e:
        lea     0x29c920.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
.L06521a:
        move.w  #0xd000,0x38(a6)                | +034
        lea     .L065226(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L065226:
        jsr     0x27d50.l                       | +040
        bcc.w   .L065236                        | +046
        lea     TaskHandler_06527c(pc),a1       | +04a
        move.l  a1,(a6)                         | +04e
.L065236:
        jsr     0x28d70.l                       | +050
        cmpi.w  #0x120,0x24(a6)                 | +056
        bgt.w   .L06524c                        | +05c
        lea     TaskHandler_06527c(pc),a1       | +060
        move.l  a1,(a6)                         | +064
.L06524c:
        jsr     0x2870a.l                       | +066
        jsr     0x49fd0.l                       | +06c
        lea     0x5e766.l,a0                    | +072
        jsr     0x5e770.l                       | +078
        movea.l #0xffffffff,a0                  | +07e
        jsr     0x5dd56.l                       | +084
        bcc.w   SetHandlerRts_06527a            | +08a

| ----------------------------------------------------------------------------
|  TaskHandler_06527c  @ $06527C  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06527c, "ax", @progbits
        .global TaskHandler_06527c
TaskHandler_06527c:
        move.w  #0x1e,0x70(a6)                  | +000
        jsr     0x267e2.l                       | +006
        bset    #0x6,0x12(a6)                   | +00c
        move.w  #0xf000,d0                      | +012
        jsr     0x28134.l                       | +016
        andi.w  #0xffe3,0x38(a6)                | +01c
        ori.w   #0x1c,0x38(a6)                  | +022
        lea     0x78908.l,a1                    | +028
        jsr     0x4ae.l                         | +02e
        jsr     0x5dd02.l                       | +034
        lea     .L0652bc(pc),a1                 | +03a
        move.l  a1,(a6)                         | +03e
.L0652bc:
        move.b  #0x1,0x45(a6)                   | +040
        jsr     0x2783a.l                       | +046
        subq.w  #0x1,0x70(a6)                   | +04c
        cmpi.w  #0x0,0x70(a6)                   | +050
        bgt.w   .L0652dc                        | +056
        lea     TaskHandler_0652f4(pc),a1       | +05a
        move.l  a1,(a6)                         | +05e
.L0652dc:
        movea.l #0xffffffff,a0                  | +060
        jsr     0x5dd56.l                       | +066
        bcc.w   SetHandlerRts_0652f2            | +06c

| ----------------------------------------------------------------------------
|  TaskHandler_0652f4  @ $0652F4  (126 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0652f4, "ax", @progbits
        .global TaskHandler_0652f4
TaskHandler_0652f4:
        subi.w  #0x18,0x24(a6)                  | +000
        move.w  #0x100,d0                       | +006
        btst    #0x0,0x3a(a6)                   | +00a
        bne.w   .L06530a                        | +010
        neg.w   d0                              | +014
.L06530a:
        move.w  d0,0x28(a6)                     | +016
        move.w  #0x160,d1                       | +01a
        jsr     0x236e.l                        | +01e
        lea     0x29c962.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        lea     .L06532a(pc),a1                 | +030
        move.l  a1,(a6)                         | +034
.L06532a:
        jsr     0x27cee.l                       | +036
        jsr     0x28d70.l                       | +03c
        bcc.w   .L065346                        | +042
        lea     0x29c9c2.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
.L065346:
        jsr     0x2870a.l                       | +052
        bcc.w   .L065356                        | +058
        lea     TaskHandler_06537a(pc),a1       | +05c
        move.l  a1,(a6)                         | +060
.L065356:
        lea     0x5e766.l,a0                    | +062
        jsr     0x5e770.l                       | +068
        movea.l #0xffffffff,a0                  | +06e
        jsr     0x5dd56.l                       | +074
        bcc.w   SetHandlerRts_065378            | +07a

| ----------------------------------------------------------------------------
|  TaskHandler_06537a  @ $06537A  (60 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06537a, "ax", @progbits
        .global TaskHandler_06537a
TaskHandler_06537a:
        asr.w   0x28(a6)                        | +000
        lea     0x29c992.l,a0                   | +004
        jsr     0x28cd4.l                       | +00a
        lea     .L065390(pc),a1                 | +010
        move.l  a1,(a6)                         | +014
.L065390:
        jsr     0x27cee.l                       | +016
        jsr     0x28d70.l                       | +01c
        bcc.w   .L0653a6                        | +022
        lea     TaskHandler_0653be(pc),a1       | +026
        move.l  a1,(a6)                         | +02a
.L0653a6:
        movea.l #0xffffffff,a0                  | +02c
        jsr     0x5dd56.l                       | +032
        bcc.w   SetHandlerRts_0653bc            | +038

| ----------------------------------------------------------------------------
|  TaskHandler_0653be  @ $0653BE  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0653be, "ax", @progbits
        .global TaskHandler_0653be
TaskHandler_0653be:
        addi.w  #0x18,0x24(a6)                  | +000
        jsr     0x13600.l                       | +006
        move.w  #0xc,d1                         | +00c
        jsr     0x236e.l                        | +010
        lea     0x2de6e0.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L0653e6(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L0653e6:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L0653fc                        | +034
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L0653fc:
        movea.l #0xffffffff,a0                  | +03e
        jsr     0x5dd56.l                       | +044
        bcc.w   SetHandlerRts_065412            | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_065414  @ $065414  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065414, "ax", @progbits
        .global TaskHandler_065414
TaskHandler_065414:
        move.w  #0xc000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x1c,0x38(a6)                  | +010
        lea     0x4acfe.l,a0                    | +016
        jsr     0x28cd4.l                       | +01c
        lea     .L06543c(pc),a1                 | +022
        move.l  a1,(a6)                         | +026
.L06543c:
        jsr     0x2783a.l                       | +028
        jsr     0x28d70.l                       | +02e
        bcc.w   .L065452                        | +034
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +038
        move.l  a1,(a6)                         | +03c
.L065452:
        movea.l #0xffffffff,a0                  | +03e
        jsr     0x5dd56.l                       | +044
        bcc.w   SetHandlerRts_065468            | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_06546a  @ $06546A  (130 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06546a, "ax", @progbits
        .global TaskHandler_06546a
TaskHandler_06546a:
        move.w  #0x4,0x80(a6)                   | +000
        move.w  #0x4000,d0                      | +006
        jsr     0x28134.l                       | +00a
        andi.w  #0xffe3,0x38(a6)                | +010
        ori.w   #0x1c,0x38(a6)                  | +016
        move.w  #0x45,d1                        | +01c
        jsr     0x236e.l                        | +020
        jsr     0x5e9b6.l                       | +026
        andi.w  #0x7,d0                         | +02c
        beq.w   TaskHandler_065522__L065554     | +030
        jsr     0x5e9b6.l                       | +034
        andi.w  #0x3,d0                         | +03a
        move.w  d0,d1                           | +03e
        add.w   d1,d1                           | +040
        add.w   d0,d1                           | +042
        add.w   d1,d1                           | +044
        lea     0x2c65c4.l,a0                   | +046
        move.w  0x4(a0,d1.w),0x2a(a6)           | +04c
        move.w  0x2(a0,d1.w),0x2e(a6)           | +052
        move.w  (a0,d1.w),d0                    | +058
        btst    #0x0,0x3a(a6)                   | +05c
        bne.w   .L0654d2                        | +062
        neg.w   d0                              | +066
.L0654d2:
        move.w  d0,0x28(a6)                     | +068
        jsr     0x5e9b6.l                       | +06c
        andi.w  #0x1f,d0                        | +072
        addi.w  #0x18,d0                        | +076
        move.w  d0,0x70(a6)                     | +07a
        bra.w   TaskHandler_0654ec__L0654f8     | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_0654ec  @ $0654EC  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0654ec, "ax", @progbits
        .global TaskHandler_0654ec
TaskHandler_0654ec:
        move.w  0x2a(a6),d0                     | +000
        asr.w   #0x1,d0                         | +004
        neg.w   d0                              | +006
        move.w  d0,0x2a(a6)                     | +008
        .global TaskHandler_0654ec__L0654f8
TaskHandler_0654ec__L0654f8:
.L0654f8:
        lea     0x2c6dd0.l,a0                   | +00c
        jsr     0x28cd4.l                       | +012
        lea     .L06550a(pc),a1                 | +018
        move.l  a1,(a6)                         | +01c
.L06550a:
        jsr     0x27d50.l                       | +01e
        bcc.w   .L06551a                        | +024
        lea     TaskHandler_065522(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L06551a:
        jsr     TaskHandler_065db4(pc)          | +02e
        bra.w   TaskHandler_065522__L0655bc     | +032

| ----------------------------------------------------------------------------
|  TaskHandler_065522  @ $065522  (202 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065522, "ax", @progbits
        .global TaskHandler_065522
TaskHandler_065522:
        subq.w  #0x1,0x80(a6)                   | +000
        blt.w   .L065554                        | +004
        lea     0x2c6dea.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L06553c(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L06553c:
        jsr     0x2783a.l                       | +01a
        jsr     TaskHandler_065db4(pc)          | +020
        bcc.w   .L065550                        | +024
        lea     TaskHandler_0654ec(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L065550:
        bra.w   .L0655bc                        | +02e
        .global TaskHandler_065522__L065554
TaskHandler_065522__L065554:
.L065554:
        move.w  0x28(a6),d0                     | +032
        movem.w d0,-(a7)                        | +036
        jsr     0x267e2.l                       | +03a
        movem.w (a7)+,d0                        | +040
        cmpi.w  #0x200,d0                       | +044
        bge.w   .L065588                        | +048
        cmpi.w  #0xfe00,d0                      | +04c
        ble.w   .L065588                        | +050
        move.w  #0x200,d1                       | +054
        cmpi.w  #0x0,d0                         | +058
        bge.w   .L065586                        | +05c
        move.w  #0xfe00,d1                      | +060
.L065586:
        move.w  d1,d0                           | +064
.L065588:
        move.w  d0,0x28(a6)                     | +066
        lea     0x2c6dd0.l,a0                   | +06a
        jsr     0x28cd4.l                       | +070
        lea     .L06559e(pc),a1                 | +076
        move.l  a1,(a6)                         | +07a
.L06559e:
        jsr     0x27a92.l                       | +07c
        jsr     0x5e9b6.l                       | +082
        andi.w  #0xf,d0                         | +088
        bne.w   .L0655b8                        | +08c
        lea     TaskHandler_0655f4(pc),a1       | +090
        move.l  a1,(a6)                         | +094
.L0655b8:
        jsr     TaskHandler_065db4(pc)          | +096
        .global TaskHandler_065522__L0655bc
TaskHandler_065522__L0655bc:
.L0655bc:
        cmpi.w  #0x0,0x70(a6)                   | +09a
        bgt.w   .L0655cc                        | +0a0
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +0a4
        move.l  a1,(a6)                         | +0a8
.L0655cc:
        cmpi.w  #0xffd0,0x22(a6)                | +0aa
        bgt.w   .L0655dc                        | +0b0
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +0b4
        move.l  a1,(a6)                         | +0b8
.L0655dc:
        movea.l #0xffffffff,a0                  | +0ba
        jsr     0x5dd56.l                       | +0c0
        bcc.w   SetHandlerRts_0655f2            | +0c6

| ----------------------------------------------------------------------------
|  TaskHandler_0655f4  @ $0655F4  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0655f4, "ax", @progbits
        .global TaskHandler_0655f4
TaskHandler_0655f4:
        move.w  #0x2a9,0x2a(a6)                 | +000
        move.w  #0xff1d,0x2e(a6)                | +006
        bra.w   TaskHandler_0654ec__L0654f8     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_065604  @ $065604  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065604, "ax", @progbits
        .global TaskHandler_065604
TaskHandler_065604:
        lea     0x2c7366.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L065640                        | +00c
        lea     0x2c73b8.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        bra.w   .L065640                        | +01c
        lea     0x2c740a.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        bra.w   .L065640                        | +02c
        lea     0x2c745c.l,a0                   | +030
        jsr     0x28cd4.l                       | +036
.L065640:
        move.w  #0x4,d1                         | +03c
        jsr     0x236e.l                        | +040
        .global TaskHandler_065604__L06564a
TaskHandler_065604__L06564a:
.L06564a:
        lea     .L065650(pc),a1                 | +046
        move.l  a1,(a6)                         | +04a
.L065650:
        jsr     0x2783a.l                       | +04c
        jsr     0x28d70.l                       | +052
        bcc.w   SetHandlerRts_065666            | +058

| ----------------------------------------------------------------------------
|  TaskHandler_065668  @ $065668  (252 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065668, "ax", @progbits
        .global TaskHandler_065668
TaskHandler_065668:
        clr.w   0x84(a6)                        | +000
        move.w  #0x7fff,0x8a(a0)                | +004
        bra.w   .L06568c                        | +00a
        .global TaskHandler_065668__L065676
TaskHandler_065668__L065676:
.L065676:
        move.w  #0x46,d1                        | +00e
        jsr     0x236e.l                        | +012
        lea     0x2c6e14.l,a0                   | +018
        jsr     0x28cd4.l                       | +01e
.L06568c:
        bset    #0x4,0x6b(a6)                   | +024
        move.w  #0x10a4,d0                      | +02a
        jsr     0x2352.l                        | +02e
        clr.w   0x70(a6)                        | +034
        clr.w   0x86(a6)                        | +038
        clr.b   0x3a(a6)                        | +03c
        move.w  #0x5,0x66(a6)                   | +040
        move.w  #0x200,0x36(a6)                 | +046
        andi.w  #0x1f,0x88(a6)                  | +04c
        move.w  #0xd000,d0                      | +052
        jsr     0x28134.l                       | +056
        andi.w  #0xffe3,0x38(a6)                | +05c
        ori.w   #0x1c,0x38(a6)                  | +062
        lea     .L0656d6(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L0656d6:
        subq.w  #0x1,0x8a(a6)                   | +06e
        cmpi.w  #0x0,0x8a(a6)                   | +072
        ble.w   .L0656f4                        | +078
        jsr     TaskHandler_06576c(pc)          | +07c
        jsr     TaskHandler_06580e(pc)          | +080
        jsr     TaskHandler_065888(pc)          | +084
        jsr     TaskHandler_065c64(pc)          | +088
.L0656f4:
        jsr     0x27cee.l                       | +08c
        bcc.w   .L065704                        | +092
        lea     TaskHandler_065166(pc),a1       | +096
        move.l  a1,(a6)                         | +09a
.L065704:
        cmpi.w  #0x20,0x88(a6)                  | +09c
        bcs.w   .L06571c                        | +0a2
        nop                                     | +0a6
        nop                                     | +0a8
        cmpi.w  #0x20,0x88(a6)                  | +0aa
        nop                                     | +0b0
        trap    #0xf                            | +0b2
.L06571c:
        move.w  0x88(a6),0x34(a6)               | +0b4
        jsr     0x28d70.l                       | +0ba
        jsr     0x283d8.l                       | +0c0
        btst    #0x1,0x13(a6)                   | +0c6
        beq.w   .L06573e                        | +0cc
        lea     TaskHandler_065166__L065194(pc),a1 | +0d0
        move.l  a1,(a6)                         | +0d4
.L06573e:
        jsr     0x28758.l                       | +0d6
        bcc.w   .L06574e                        | +0dc
        lea     TaskHandler_065166(pc),a1       | +0e0
        move.l  a1,(a6)                         | +0e4
.L06574e:
        bclr    #0x3,0x13(a6)                   | +0e6
        movea.l #0xffffffff,a0                  | +0ec
        jsr     0x5dd56.l                       | +0f2
        bcc.w   SetHandlerRts_06576a            | +0f8

| ----------------------------------------------------------------------------
|  TaskHandler_06576c  @ $06576C  (162 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06576c, "ax", @progbits
        .global TaskHandler_06576c
TaskHandler_06576c:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L06580c                        | +00a
        jsr     0x5e9b6.l                       | +00e
        andi.w  #0x1f,d0                        | +014
        addi.w  #0x30,d0                        | +018
        move.w  d0,0x70(a6)                     | +01c
        addq.w  #0x1,0x86(a6)                   | +020
        cmpi.w  #0x1,0x86(a6)                   | +024
        bgt.w   .L0657ce                        | +02a
        jsr     0x5e9b6.l                       | +02e
        andi.w  #0xf,d0                         | +034
        lsl.w   #0x2,d0                         | +038
        lea     0x2c65dc.l,a0                   | +03a
        move.w  (a0,d0.w),0x80(a6)              | +040
        move.w  0x2(a0,d0.w),0x82(a6)           | +046
        jsr     0x5e9b6.l                       | +04c
        andi.w  #0x1f0,d0                       | +052
        addi.w  #0x100,d0                       | +056
        move.w  d0,0x36(a6)                     | +05a
        bra.w   .L0657f0                        | +05e
.L0657ce:
        jsr     0x5e0d4.l                       | +062
        move.w  0x22(a0),0x80(a6)               | +068
        move.w  0x24(a0),0x82(a6)               | +06e
        move.w  #0x300,0x36(a6)                 | +074
        cmpi.w  #0x3,0x86(a6)                   | +07a
        bgt.w   .L06580c                        | +080
.L0657f0:
        jsr     0x5e9b6.l                       | +084
        andi.w  #0x1f,d0                        | +08a
        add.w   d0,0x80(a6)                     | +08e
        jsr     0x5e9b6.l                       | +092
        andi.w  #0x1f,d0                        | +098
        add.w   d0,0x82(a6)                     | +09c
.L06580c:
        rts                                     | +0a0

| ----------------------------------------------------------------------------
|  TaskHandler_06580e  @ $06580E  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_06580e, "ax", @progbits
        .global TaskHandler_06580e
TaskHandler_06580e:
        move.b  0x106f28.l,d0                   | +000
        andi.b  #0x3,d0                         | +006
        bne.w   TaskHandler_065866__L065870     | +00a
        move.w  0x80(a6),d0                     | +00e
        sub.w   0x22(a6),d0                     | +012
        move.w  0x82(a6),d1                     | +016
        sub.w   0x24(a6),d1                     | +01a
        jsr     0x5e018.l                       | +01e
        lsr.w   #0x3,d0                         | +024
        move.w  #0x1,d1                         | +026
        cmp.w   0x88(a6),d0                     | +02a
        beq.w   TaskHandler_065866              | +02e
        bgt.w   .L065848                        | +032
        move.w  #0xffff,d1                      | +036
.L065848:
        cmpi.w  #0x0,0x84(a6)                   | +03a
        beq.w   .L065856                        | +040
        move.w  0x84(a6),d1                     | +044
.L065856:
        add.w   d1,0x88(a6)                     | +048
        andi.w  #0x1f,0x88(a6)                  | +04c

| ----------------------------------------------------------------------------
|  TaskHandler_065866  @ $065866  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065866, "ax", @progbits
        .global TaskHandler_065866
TaskHandler_065866:
        move.w  0x70(a6),d0                     | +000
        asr.w   #0x1,d0                         | +004
        move.w  d0,0x70(a6)                     | +006
        .global TaskHandler_065866__L065870
TaskHandler_065866__L065870:
.L065870:
        jsr     0x5e9b6.l                       | +00a
        andi.w  #0x1f,d0                        | +010
        bne.w   ClearXN_065882                  | +014
        neg.w   0x84(a6)                        | +018

| ----------------------------------------------------------------------------
|  TaskHandler_065888  @ $065888  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065888, "ax", @progbits
        .global TaskHandler_065888
TaskHandler_065888:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x80(a6),d0                     | +004
        move.w  0x24(a6),d1                     | +008
        sub.w   0x82(a6),d1                     | +00c
        cmpi.w  #0xfff0,d0                      | +010
        blt.w   .L0658bc                        | +014
        cmpi.w  #0x10,d0                        | +018
        bgt.w   .L0658bc                        | +01c
        cmpi.w  #0xfff0,d1                      | +020
        blt.w   .L0658bc                        | +024
        cmpi.w  #0x10,d1                        | +028
        bgt.w   .L0658bc                        | +02c
        clr.w   0x70(a6)                        | +030
.L0658bc:
        rts                                     | +034

| ----------------------------------------------------------------------------
|  TaskHandler_0658be  @ $0658BE  (118 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0658be, "ax", @progbits
        .global TaskHandler_0658be
TaskHandler_0658be:
        move.w  #0x12,d0                        | +000
        move.w  #0x1e,d1                        | +004
        bra.w   .L0658ea                        | +008
        move.w  #0x15,d0                        | +00c
        move.w  #0x1b,d1                        | +010
        bra.w   .L0658ea                        | +014
        move.w  #0x1b,d0                        | +018
        move.w  #0x15,d1                        | +01c
        bra.w   .L0658ea                        | +020
        move.w  #0x1e,d0                        | +024
        move.w  #0x12,d1                        | +028
.L0658ea:
        btst    #0x0,0x3a(a6)                   | +02c
        beq.w   .L0658f6                        | +032
        move.w  d1,d0                           | +036
.L0658f6:
        move.w  d0,0x88(a6)                     | +038
        movea.l 0xc(a6),a0                      | +03c
        move.w  0x84(a0),d0                     | +040
        andi.w  #0x3,d0                         | +044
        add.w   d0,d0                           | +048
        lea     0x2c65a4.l,a0                   | +04a
        move.w  (a0,d0.w),0x84(a6)              | +050
        movea.l 0xc(a6),a0                      | +056
        movea.l 0xc(a0),a0                      | +05a
        move.b  0x9b(a0),d0                     | +05e
        andi.w  #0xff,d0                        | +062
        bne.w   .L06592c                        | +066
        move.w  #0x7fff,d0                      | +06a
.L06592c:
        move.w  d0,0x8a(a6)                     | +06e
        bra.w   TaskHandler_065668__L065676     | +072

| ----------------------------------------------------------------------------
|  TaskHandler_065934  @ $065934  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065934, "ax", @progbits
        .global TaskHandler_065934
TaskHandler_065934:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x2c74ae.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        move.w  #0x4000,d0                      | +016
        jsr     0x28134.l                       | +01a
        andi.w  #0xffe3,0x38(a6)                | +020
        ori.w   #0x1c,0x38(a6)                  | +026
        bra.w   TaskHandler_065604__L06564a     | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_065964  @ $065964  (312 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065964, "ax", @progbits
        .global TaskHandler_065964
TaskHandler_065964:
        bset    #0x4,0x6b(a6)                   | +000
        move.w  #0x15d,d1                       | +006
        jsr     0x236e.l                        | +00a
        move.w  #0x164,d1                       | +010
        jsr     0x236e.l                        | +014
        move.w  #0x165,d1                       | +01a
        jsr     0x236e.l                        | +01e
        clr.b   0x7a(a6)                        | +024
        move.w  #0x8,0x66(a6)                   | +028
        cmpi.w  #0x178,0x24(a6)                 | +02e
        blt.w   .L0659c0                        | +034
        move.w  #0x111,d0                       | +038
        jsr     0x5dca4.l                       | +03c
        move.w  d0,0x28(a6)                     | +042
        move.w  #0x21c,0x2a(a6)                 | +046
        move.w  #0xffdc,0x2e(a6)                | +04c
        move.w  #0x0,0x2c(a6)                   | +052
        bra.w   .L0659e0                        | +058
.L0659c0:
        move.w  #0x5b,d0                        | +05c
        jsr     0x5dca4.l                       | +060
        move.w  d0,0x28(a6)                     | +066
        move.w  #0x5a0,0x2a(a6)                 | +06a
        move.w  #0xffe0,0x2e(a6)                | +070
        move.w  #0x0,0x2c(a6)                   | +076
.L0659e0:
        move.w  #0xd000,d0                      | +07c
        jsr     0x28134.l                       | +080
        andi.w  #0xffe3,0x38(a6)                | +086
        ori.w   #0x0,0x38(a6)                   | +08c
        lea     0x2c72b4.l,a0                   | +092
        jsr     0x28cd4.l                       | +098
        lea     .L065a08(pc),a1                 | +09e
        move.l  a1,(a6)                         | +0a2
.L065a08:
        btst    #0x0,0x106f28.l                 | +0a4
        bne.w   .L065a38                        | +0ac
        move.b  0x7a(a6),d0                     | +0b0
        addq.b  #0x1,0x7a(a6)                   | +0b4
        andi.w  #0x3,d0                         | +0b8
        add.w   d0,d0                           | +0bc
        lea     0x2c65bc.l,a0                   | +0be
        move.w  (a0,d0.w),d0                    | +0c4
        andi.w  #0x3,d0                         | +0c8
        add.w   d0,d0                           | +0cc
        move.w  0x16(a6,d0.w),0x14(a6)          | +0ce
.L065a38:
        jsr     0x27d50.l                       | +0d4
        bcc.w   .L065a48                        | +0da
        lea     TaskHandler_065166__L065174(pc),a1 | +0de
        move.l  a1,(a6)                         | +0e2
.L065a48:
        jsr     0x28d70.l                       | +0e4
        jsr     0x283d8.l                       | +0ea
        btst    #0x1,0x13(a6)                   | +0f0
        beq.w   .L065a64                        | +0f6
        lea     TaskHandler_065166__L065194(pc),a1 | +0fa
        move.l  a1,(a6)                         | +0fe
.L065a64:
        jsr     0x2870a.l                       | +100
        bclr    #0x3,0x13(a6)                   | +106
        jsr     0x28758.l                       | +10c
        bcc.w   .L065a80                        | +112
        lea     TaskHandler_065166__L06517e(pc),a1 | +116
        move.l  a1,(a6)                         | +11a
.L065a80:
        lea     0x5e766.l,a0                    | +11c
        jsr     0x5e770.l                       | +122
        movea.l #0xffffffff,a0                  | +128
        jsr     0x5dd56.l                       | +12e
        bcc.w   SetHandlerRts_065aa2            | +134

| ----------------------------------------------------------------------------
|  TaskHandler_065aa4  @ $065AA4  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065aa4, "ax", @progbits
        .global TaskHandler_065aa4
TaskHandler_065aa4:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        move.w  #0x20,0x2a(a6)                  | +00a
        lea     0x2dd80c.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
        lea     .L065ac6(pc),a1                 | +01c
        move.l  a1,(a6)                         | +020
.L065ac6:
        jsr     0x27cee.l                       | +022
        jsr     0x28d70.l                       | +028
        bcc.w   .L065adc                        | +02e
        lea     Jsr5B6ThenJmpScheduler_064d8a(pc),a1 | +032
        move.l  a1,(a6)                         | +036
.L065adc:
        movea.l #0xffffffff,a0                  | +038
        jsr     0x5dd56.l                       | +03e
        bcc.w   SetHandlerRts_065af2            | +044

| ----------------------------------------------------------------------------
|  TaskHandler_065af4  @ $065AF4  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065af4, "ax", @progbits
        .global TaskHandler_065af4
TaskHandler_065af4:
        cmpi.w  #0x20,0x22(a6)                  | +000
        blt.w   ClearXN_065b30                  | +006
        cmpi.w  #0x120,0x22(a6)                 | +00a
        bgt.w   ClearXN_065b30                  | +010
        subq.w  #0x1,0x72(a6)                   | +014
        bcc.w   .L065b16                        | +018
        move.w  #0x0,0x72(a6)                   | +01c
.L065b16:
        cmpi.w  #0x0,0x8e(a6)                   | +022
        beq.w   ClearXN_065b30                  | +028
        cmpi.w  #0x0,0x72(a6)                   | +02c
        bgt.w   ClearXN_065b30                  | +032

| ----------------------------------------------------------------------------
|  TaskHandler_065b36  @ $065B36  (104 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065b36, "ax", @progbits
        .global TaskHandler_065b36
TaskHandler_065b36:
        jsr     0x5e136.l                       | +000
        bcs.w   .L065b98                        | +006
        cmpi.w  #0x80,d0                        | +00a
        bge.w   .L065b50                        | +00e
        move.w  #0x100,d1                       | +012
        sub.w   d0,d1                           | +016
        move.w  d1,d0                           | +018
.L065b50:
        subi.w  #0x80,d0                        | +01a
        lsr.w   #0x3,d0                         | +01e
        lea     0x2c6560.l,a0                   | +020
        btst    #0x0,0x3a(a6)                   | +026
        beq.w   .L065b6c                        | +02c
        lea     0x2c6582.l,a0                   | +030
.L065b6c:
        add.w   d0,d0                           | +036
        move.w  (a0,d0.w),d1                    | +038
        move.w  #0xffff,d0                      | +03c
        cmp.w   0x88(a6),d1                     | +040
        beq.w   TaskHandler_065ba4              | +044
        blt.w   .L065b86                        | +048
        move.w  #0x1,d0                         | +04c
.L065b86:
        move.b  0x106f28.l,d1                   | +050
        andi.w  #0x1,d1                         | +056
        bne.w   .L065b98                        | +05a
        add.w   d0,0x88(a6)                     | +05e
.L065b98:
        andi.w  #0x7,0x88(a6)                   | +062

| ----------------------------------------------------------------------------
|  TaskHandler_065ba4  @ $065BA4  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065ba4, "ax", @progbits
        .global TaskHandler_065ba4
TaskHandler_065ba4:
        andi.w  #0x7,0x88(a6)                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065bb0  @ $065BB0  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065bb0, "ax", @progbits
        .global TaskHandler_065bb0
TaskHandler_065bb0:
        jsr     0x5e506.l                       | +000
        move.b  0x21(a6),0x21(a0)               | +006
        move.w  0x8e(a0),0x8e(a6)               | +00c
        move.b  0x20(a0),d0                     | +012
        andi.w  #0xf,d0                         | +016
        cmpi.w  #0xf,d0                         | +01a
        beq.w   .L065c00                        | +01e
        move.b  #0xf,0x20(a0)                   | +022
        movem.l d0,-(a7)                        | +028
        move.b  d0,0x20(a6)                     | +02c
        movea.l #0x2c6510,a0                    | +030
        lsl.w   #0x2,d0                         | +036
        movea.l (a0,d0.w),a0                    | +038
        cmpa.l  #0xffffffff,a0                  | +03c
        beq.w   .L065bfc                        | +042
        jsr     0x28cd4.l                       | +046
.L065bfc:
        movem.l (a7)+,d0                        | +04c
.L065c00:
        rts                                     | +050

| ----------------------------------------------------------------------------
|  TaskHandler_065c02  @ $065C02  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065c02, "ax", @progbits
        .global TaskHandler_065c02
TaskHandler_065c02:
        move.w  0x80(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        beq.w   .L065c1c                        | +00a
        cmp.w   0x22(a6),d0                     | +00e
        bgt.w   ClearXN_065c2a                  | +012
        bra.w   SetXN_065c24                    | +016
.L065c1c:
        cmp.w   0x22(a6),d0                     | +01a
        blt.w   ClearXN_065c2a                  | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_065c30  @ $065C30  (52 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065c30, "ax", @progbits
        .global TaskHandler_065c30
TaskHandler_065c30:
        move.w  0x82(a6),d0                     | +000
        btst    #0x0,0x3a(a6)                   | +004
        bne.w   .L065c52                        | +00a
        cmp.w   0x28(a6),d0                     | +00e
        blt.w   .L065c62                        | +012
        move.w  d0,0x28(a6)                     | +016
        clr.w   0x2c(a6)                        | +01a
        bra.w   .L065c62                        | +01e
.L065c52:
        cmp.w   0x28(a6),d0                     | +022
        bgt.w   .L065c62                        | +026
        move.w  d0,0x28(a6)                     | +02a
        clr.w   0x2c(a6)                        | +02e
.L065c62:
        rts                                     | +032

| ----------------------------------------------------------------------------
|  TaskHandler_065c64  @ $065C64  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065c64, "ax", @progbits
        .global TaskHandler_065c64
TaskHandler_065c64:
        move.w  0x88(a6),d0                     | +000
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
|  TaskHandler_065ce8  @ $065CE8  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065ce8, "ax", @progbits
        .global TaskHandler_065ce8
TaskHandler_065ce8:
        lea     TaskHandler_065934(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        move.w  0x88(a6),d0                     | +010
        lsl.w   #0x3,d0                         | +014
        andi.w  #0xff,d0                        | +016
        add.w   d0,d0                           | +01a
        lea     0x2c072c.l,a1                   | +01c
        lea     0x2c07ac.l,a2                   | +022
        move.w  (a1,d0.w),d1                    | +028
        move.w  (a2,d0.w),d2                    | +02c
        asr.w   #0x4,d1                         | +030
        asr.w   #0x4,d2                         | +032
        move.w  d1,d3                           | +034
        move.w  d2,d4                           | +036
        asr.w   #0x1,d1                         | +038
        asr.w   #0x1,d2                         | +03a
        add.w   d3,d1                           | +03c
        add.w   d4,d2                           | +03e
        sub.w   d1,0x24(a0)                     | +040
        sub.w   d2,0x22(a0)                     | +044
        rts                                     | +048

| ----------------------------------------------------------------------------
|  TaskHandler_065d4a  @ $065D4A  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065d4a, "ax", @progbits
        .global TaskHandler_065d4a
TaskHandler_065d4a:
        cmpi.b  #0x0,0x78(a6)                   | +000
        bne.w   .L065d64                        | +006
        move.b  0x10e276.l,d0                   | +00a
        move.b  0x10e277.l,d1                   | +010
        bra.w   .L065d70                        | +016
.L065d64:
        move.b  0x10e278.l,d0                   | +01a
        move.b  0x10e279.l,d1                   | +020
.L065d70:
        cmp.b   d0,d1                           | +026
        ble.w   .L065d78                        | +028
        move.b  d1,d0                           | +02c
.L065d78:
        rts                                     | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_065d7a  @ $065D7A  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065d7a, "ax", @progbits
        .global TaskHandler_065d7a
TaskHandler_065d7a:
        cmpi.b  #0x0,0x78(a6)                   | +000
        bne.w   .L065d8c                        | +006
        addq.b  #0x1,0x10e277.l                 | +00a
        rts                                     | +010
.L065d8c:
        addq.b  #0x1,0x10e279.l                 | +012
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_065d94  @ $065D94  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065d94, "ax", @progbits
        .global TaskHandler_065d94
TaskHandler_065d94:
        move.w  0x82(a6),d0                     | +000
        asr.w   #0x5,d0                         | +004
        cmpi.w  #0x0,d0                         | +006
        bne.w   SetTaskW_065dae                 | +00a
        nop                                     | +00e
        nop                                     | +010
        cmpi.w  #0x0,d0                         | +012
        nop                                     | +016
        trap    #0xf                            | +018

| ----------------------------------------------------------------------------
|  TaskHandler_065db4  @ $065DB4  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065db4, "ax", @progbits
        .global TaskHandler_065db4
TaskHandler_065db4:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0xf,0x70(a6)                   | +004
        bgt.w   JsrAbsThunk_065dce              | +00a
        move.w  0x70(a6),d0                     | +00e
        btst    #0x0,d0                         | +012
        beq.w   ClearXN_065dd6                  | +016

| ----------------------------------------------------------------------------
|  TaskHandler_065ddc  @ $065DDC  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065ddc, "ax", @progbits
        .global TaskHandler_065ddc
TaskHandler_065ddc:
        move.w  #0x4000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x0,0x38(a6)                   | +010
        lea     0x77fd6.l,a1                    | +016
        jsr     0x4ae.l                         | +01c
        jsr     0x5dd02.l                       | +022
        subq.w  #0x1,0x38(a0)                   | +028
        lea     TaskHandler_06546a(pc),a1       | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        lea     TaskHandler_06546a(pc),a1       | +03c
        jsr     0x4ae.l                         | +040
        jsr     0x5dd02.l                       | +046
        eori.b  #0x1,0x3a(a0)                   | +04c
        rts                                     | +052

| ----------------------------------------------------------------------------
|  TaskHandler_065e30  @ $065E30  (62 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065e30, "ax", @progbits
        .global TaskHandler_065e30
TaskHandler_065e30:
        lea     TaskHandler_065414(pc),a1       | +000
        jsr     0x4ae.l                         | +004
        jsr     0x5dd02.l                       | +00a
        subq.w  #0x1,0x38(a0)                   | +010
        addi.w  #0x10,0x24(a0)                  | +014
        cmpi.b  #0x1,0x79(a6)                   | +01a
        bne.w   .L065e6c                        | +020
        lea     TaskHandler_065414(pc),a1       | +024
        jsr     0x4ae.l                         | +028
        jsr     0x5dd02.l                       | +02e
        subq.w  #0x1,0x38(a0)                   | +034
        addq.w  #0x8,0x24(a0)                   | +038
.L065e6c:
        rts                                     | +03c

| ----------------------------------------------------------------------------
|  TaskHandler_065e6e  @ $065E6E  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065e6e, "ax", @progbits
        .global TaskHandler_065e6e
TaskHandler_065e6e:
        cmpi.b  #0x1,0x78(a6)                   | +000
        beq.w   .L065ea4                        | +006
        jsr     0x4a16e.l                       | +00a
        move.w  0x38(a6),d0                     | +010
        subq.w  #0x1,d0                         | +014
        move.w  d0,0x38(a0)                     | +016
        addi.w  #0x10,0x24(a0)                  | +01a
        cmpi.b  #0x1,0x79(a6)                   | +020
        bne.w   .L065ea2                        | +026
        jsr     0x4a16e.l                       | +02a
        addq.w  #0x8,0x24(a0)                   | +030
.L065ea2:
        rts                                     | +034
.L065ea4:
        lea     0x4a7a6.l,a1                    | +036
        jsr     0x4ae.l                         | +03c
        jsr     0x5dd02.l                       | +042
        move.w  0x38(a6),d0                     | +048
        subq.w  #0x1,d0                         | +04c
        move.w  d0,0x38(a0)                     | +04e
        addi.w  #0x10,0x24(a0)                  | +052
        cmpi.b  #0x1,0x79(a6)                   | +058
        bne.w   .L065ee6                        | +05e
        lea     0x4a7a6.l,a1                    | +062
        jsr     0x4ae.l                         | +068
        jsr     0x5dd02.l                       | +06e
        addq.w  #0x8,0x24(a0)                   | +074
.L065ee6:
        rts                                     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_065ee8  @ $065EE8  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065ee8, "ax", @progbits
        .global TaskHandler_065ee8
TaskHandler_065ee8:
        lea     0x2c6500.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065ef4  @ $065EF4  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065ef4, "ax", @progbits
        .global TaskHandler_065ef4
TaskHandler_065ef4:
        lea     0x2c6460.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065f00  @ $065F00  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065f00, "ax", @progbits
        .global TaskHandler_065f00
TaskHandler_065f00:
        lea     0x2c6488.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065f0c  @ $065F0C  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065f0c, "ax", @progbits
        .global TaskHandler_065f0c
TaskHandler_065f0c:
        lea     0x2c64b0.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065f18  @ $065F18  (6 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065f18, "ax", @progbits
        .global TaskHandler_065f18
TaskHandler_065f18:
        lea     0x2c64d8.l,a1                   | +000

| ----------------------------------------------------------------------------
|  TaskHandler_065f24  @ $065F24  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065f24, "ax", @progbits
        .global TaskHandler_065f24
TaskHandler_065f24:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_065f3a                    | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_065f40  @ $065F40  (122 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065f40, "ax", @progbits
        .global TaskHandler_065f40
TaskHandler_065f40:
        move.w  #0x179,d1                       | +000
        jsr     0x236e.l                        | +004
        bset    #0x6,0x12(a6)                   | +00a
        move.w  #0x8000,0x38(a6)                | +010
        jsr     0x267e2.l                       | +016
        jsr     0x27cee.l                       | +01c
        move.w  #0x1,0x66(a6)                   | +022
        lea     0x2c756c.l,a0                   | +028
        jsr     0x28cd4.l                       | +02e
        lea     .L065f7a(pc),a1                 | +034
        move.l  a1,(a6)                         | +038
.L065f7a:
        jsr     0x2783a.l                       | +03a
        jsr     0x28d70.l                       | +040
        jsr     Sub_00066622(pc)                | +046  -> $066622 (hueco futuro, defsym forward)
        jsr     Sub_00066644(pc)                | +04a  -> $066644 (hueco futuro, defsym forward)
        bcc.w   .L065f9c                        | +04e
        move.l  a0,0x70(a6)                     | +052
        lea     TaskHandler_065fba(pc),a1       | +056
        move.l  a1,(a6)                         | +05a
.L065f9c:
        movea.l #0xffffffff,a0                  | +05c
        lea     0x2c7998.l,a0                   | +062
        jsr     0x5dd5c.l                       | +068
        bcc.w   .L065fb8                        | +06e
        jmp     0x518.l                         | +072
.L065fb8:
        rts                                     | +078

| ----------------------------------------------------------------------------
|  TaskHandler_065fba  @ $065FBA  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_065fba, "ax", @progbits
        .global TaskHandler_065fba
TaskHandler_065fba:
        lea     0x2c75e6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  0x98(a6),0x74(a6)               | +00c
        lea     .L065fd2(pc),a1                 | +012
        move.l  a1,(a6)                         | +016
.L065fd2:
        jsr     0x2783a.l                       | +018
        jsr     0x28d70.l                       | +01e
        bcc.w   .L065ff0                        | +024
        subq.b  #0x1,0x74(a6)                   | +028
        bne.w   .L065ff0                        | +02c
        lea     Sub_0006600E(pc),a1             | +030  -> $06600E (hueco futuro, defsym forward)
        move.l  a1,(a6)                         | +034
.L065ff0:
        movea.l #0xffffffff,a0                  | +036
        lea     0x2c7998.l,a0                   | +03c
        .dc.w   0x4eb9                        | +042  (dato / opcode no decodificado)
        .dc.w   0x0005                        | +044  (dato / opcode no decodificado)
