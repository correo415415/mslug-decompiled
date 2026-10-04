| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave OOO — borrador
|  Región: $08E4E4..$08F400  (3,318 B, 67 entradas, 46 huecos)
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
|  TaskHandler_08e4e6  @ $08E4E6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e4e6, "ax", @progbits
        .global TaskHandler_08e4e6
TaskHandler_08e4e6:
        move.w  #0xc000,0x38(a6)                | +000
        jsr     Sub_0008F002(pc)                | +006
        jsr     Sub_0008F010(pc)                | +00a
        move.w  #0x12a,d1                       | +00e
        jsr     0x236e.l                        | +012

| ----------------------------------------------------------------------------
|  TaskHandler_08e4fe  @ $08E4FE  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e4fe, "ax", @progbits
        .global TaskHandler_08e4fe
TaskHandler_08e4fe:
        move.w  #0x4,0x2e(a6)                   | +000
        move.w  #0x6,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f4036.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08e52a(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08e52a:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x1d0,0x24(a6)                 | +03c
        blt.w   TaskHandler_08e54c              | +042

| ----------------------------------------------------------------------------
|  TaskHandler_08e54c  @ $08E54C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e54c, "ax", @progbits
        .global TaskHandler_08e54c
TaskHandler_08e54c:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08e560               | +00a
        lea     TaskHandler_08e566(pc),a1       | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  TaskHandler_08e566  @ $08E566  (70 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e566, "ax", @progbits
        .global TaskHandler_08e566
TaskHandler_08e566:
        move.w  #0xfffc,0x2e(a6)                | +000
        move.w  #0x6,0x70(a6)                   | +006
        move.w  #0x7,d0                         | +00c
        jsr     0x5ea1c.l                       | +010
        add.w   d0,0x70(a6)                     | +016
        lea     0x2f4036.l,a0                   | +01a
        jsr     0x28cd4.l                       | +020
        lea     .L08e592(pc),a1                 | +026
        move.l  a1,(a6)                         | +02a
.L08e592:
        jsr     0x2783a.l                       | +02c
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +032
        jsr     0x28d70.l                       | +036
        cmpi.w  #0x160,0x24(a6)                 | +03c
        bge.w   TaskHandler_08e5b4              | +042

| ----------------------------------------------------------------------------
|  TaskHandler_08e5b4  @ $08E5B4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e5b4, "ax", @progbits
        .global TaskHandler_08e5b4
TaskHandler_08e5b4:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bhi.w   JsrPcThunk_08e5c8               | +00a
        lea     TaskHandler_08e4fe(pc),a1       | +00e
        move.l  a1,(a6)                         | +012

| ----------------------------------------------------------------------------
|  TaskHandler_08e5ce  @ $08E5CE  (76 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e5ce, "ax", @progbits
        .global TaskHandler_08e5ce
TaskHandler_08e5ce:
        jsr     Sub_0008F108(pc)                | +000
        jsr     Sub_0008F002(pc)                | +004
        jsr     Sub_0008F010(pc)                | +008
        move.w  #0xfd00,0x2a(a6)                | +00c
        move.w  #0x10,0x2e(a6)                  | +012
        move.w  #0x124,d1                       | +018
        jsr     0x236e.l                        | +01c
        lea     0x2f4072.l,a0                   | +022
        jsr     0x28cd4.l                       | +028
        jsr     0x27cee.l                       | +02e
        jsr     0x28d70.l                       | +034
        move.w  #0xf,d0                         | +03a
        jsr     0x5ea1c.l                       | +03e
        addi.w  #0xc,d0                         | +044
        move.w  d0,0x70(a6)                     | +048

| ----------------------------------------------------------------------------
|  TaskHandler_08e622  @ $08E622  (50 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e622, "ax", @progbits
        .global TaskHandler_08e622
TaskHandler_08e622:
        subq.w  #0x1,0x70(a6)                   | +000
        cmpi.w  #0x0,0x70(a6)                   | +004
        bgt.w   .L08e636                        | +00a
        lea     TaskHandler_08e65c(pc),a1       | +00e
        move.l  a1,(a6)                         | +012
.L08e636:
        cmpi.w  #0x40,0x2a(a6)                  | +014
        blt.w   .L08e646                        | +01a
        lea     TaskHandler_08e65c(pc),a1       | +01e
        move.l  a1,(a6)                         | +022
.L08e646:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +024
        jsr     0x2783a.l                       | +028
        jsr     PcThunkTarget_08efb0(pc)        | +02e

| ----------------------------------------------------------------------------
|  TaskHandler_08e65c  @ $08E65C  (84 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e65c, "ax", @progbits
        .global TaskHandler_08e65c
TaskHandler_08e65c:
        move.w  #0x3f,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        addi.w  #0x40,d0                        | +00a
        neg.w   d0                              | +00e
        move.w  d0,0x2a(a6)                     | +010
        move.w  #0x4,0x2e(a6)                   | +014
        move.w  #0xb,0x70(a6)                   | +01a
        lea     0x2f407e.l,a0                   | +020
        jsr     0x28cd4.l                       | +026
        lea     .L08e68e(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L08e68e:
        subq.w  #0x1,0x70(a6)                   | +032
        cmpi.w  #0x0,0x70(a6)                   | +036
        bgt.w   .L08e6a2                        | +03c
        lea     TaskHandler_08e6b8(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L08e6a2:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +046
        jsr     0x2783a.l                       | +04a
        jsr     PcThunkTarget_08efb0(pc)        | +050

| ----------------------------------------------------------------------------
|  TaskHandler_08e6b8  @ $08E6B8  (32 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e6b8, "ax", @progbits
        .global TaskHandler_08e6b8
TaskHandler_08e6b8:
        lea     0x2f408a.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e6ca(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e6ca:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +012
        jsr     0x2783a.l                       | +016
        jsr     PcThunkTarget_08efb0(pc)        | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_08e6e0  @ $08E6E0  (46 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e6e0, "ax", @progbits
        .global TaskHandler_08e6e0
TaskHandler_08e6e0:
        jsr     Sub_0008F108(pc)                | +000
        jsr     Sub_0008F002(pc)                | +004
        jsr     Sub_0008F010(pc)                | +008
        move.w  #0x128,d1                       | +00c
        jsr     0x236e.l                        | +010
        lea     0x2f4096.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        jsr     0x27cee.l                       | +022
        jsr     0x28d70.l                       | +028

| ----------------------------------------------------------------------------
|  TaskHandler_08e716  @ $08E716  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e716, "ax", @progbits
        .global TaskHandler_08e716
TaskHandler_08e716:
        jsr     Pos_IntegrateX88_08d2b0(pc)     | +000
        jsr     0x2783a.l                       | +004
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +00a
        bcc.w   JsrAbsThunk_08e730              | +00e
        jmp     0x518.l                         | +012

| ----------------------------------------------------------------------------
|  TaskHandler_08e72e  @ $08E72E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e72e, "ax", @progbits
        .global TaskHandler_08e72e
TaskHandler_08e72e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_08e738  @ $08E738  (146 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e738, "ax", @progbits
        .global TaskHandler_08e738
TaskHandler_08e738:
        move.b  #0x3,0x9a(a6)                   | +000
        clr.w   0x5c(a6)                        | +006
        bra.w   .L08e750                        | +00a
        jsr     Sub_0008F108(pc)                | +00e
        move.w  #0x1,0x5c(a6)                   | +012
.L08e750:
        jsr     Sub_0008F002(pc)                | +018
        move.w  #0x125,d1                       | +01c
        jsr     0x236e.l                        | +020
        lea     0x2f41f2.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     0x2f414a.l,a0                   | +032
        move.l  a0,0x48(a6)                     | +038
        bclr    #0x3,0x13(a6)                   | +03c
        move.w  #0x64,0x66(a6)                  | +042
.L08e780:
        subq.b  #0x1,0x9a(a6)                   | +048
        cmpi.b  #0x0,0x9a(a6)                   | +04c
        blt.w   .L08e7a0                        | +052
        lea     TaskHandler_08eb56(pc),a1       | +056
        jsr     0x4ae.l                         | +05a
        jsr     0x5dd02.l                       | +060
        bra.b   .L08e780                        | +066
.L08e7a0:
        lea     .L08e7a6(pc),a1                 | +068
        move.l  a1,(a6)                         | +06c
.L08e7a6:
        jsr     0x2783a.l                       | +06e
        jsr     0x28d70.l                       | +074
        jsr     0x2870a.l                       | +07a
        bcc.w   .L08e7c2                        | +080
        lea     TaskHandler_08e7de(pc),a1       | +084
        move.l  a1,(a6)                         | +088
.L08e7c2:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +08a
        bcc.w   JsrPcThunk_08e7d8               | +08e

| ----------------------------------------------------------------------------
|  TaskHandler_08e7de  @ $08E7DE  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e7de, "ax", @progbits
        .global TaskHandler_08e7de
TaskHandler_08e7de:
        lea     0x2f4202.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e7f0(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e7f0:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e806                        | +01e
        lea     TaskHandler_08e822(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e806:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e81c               | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_08e822  @ $08E822  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e822, "ax", @progbits
        .global TaskHandler_08e822
TaskHandler_08e822:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e834(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e834:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e850                        | +024
        lea     TaskHandler_08e86c(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L08e850:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e866               | +032

| ----------------------------------------------------------------------------
|  TaskHandler_08e86c  @ $08E86C  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e86c, "ax", @progbits
        .global TaskHandler_08e86c
TaskHandler_08e86c:
        lea     0x2f4284.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e87e(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e87e:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e894                        | +01e
        lea     TaskHandler_08e8b0(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e894:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e8aa               | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_08e8b0  @ $08E8B0  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e8b0, "ax", @progbits
        .global TaskHandler_08e8b0
TaskHandler_08e8b0:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e8c2(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e8c2:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e8de                        | +024
        lea     TaskHandler_08e8fa(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L08e8de:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e8f4               | +032

| ----------------------------------------------------------------------------
|  TaskHandler_08e8fa  @ $08E8FA  (48 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e8fa, "ax", @progbits
        .global TaskHandler_08e8fa
TaskHandler_08e8fa:
        lea     0x2f4306.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L08e90c(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e90c:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        bcc.w   .L08e922                        | +01e
        lea     TaskHandler_08e93e(pc),a1       | +022
        move.l  a1,(a6)                         | +026
.L08e922:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +028
        bcc.w   JsrPcThunk_08e938               | +02c

| ----------------------------------------------------------------------------
|  TaskHandler_08e93e  @ $08E93E  (54 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e93e, "ax", @progbits
        .global TaskHandler_08e93e
TaskHandler_08e93e:
        bclr    #0x3,0x13(a6)                   | +000
        move.w  #0x64,0x66(a6)                  | +006
        lea     .L08e950(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L08e950:
        jsr     0x2783a.l                       | +012
        jsr     0x28d70.l                       | +018
        jsr     0x2870a.l                       | +01e
        bcc.w   .L08e96c                        | +024
        lea     TaskHandler_08e988(pc),a1       | +028
        move.l  a1,(a6)                         | +02c
.L08e96c:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02e
        bcc.w   JsrPcThunk_08e982               | +032

| ----------------------------------------------------------------------------
|  TaskHandler_08e988  @ $08E988  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e988, "ax", @progbits
        .global TaskHandler_08e988
TaskHandler_08e988:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        lea     0x2f4388.l,a0                   | +008
        jsr     0x28cd4.l                       | +00e
        lea     .L08e9a2(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L08e9a2:
        jsr     0x2783a.l                       | +01a
        jsr     0x28d70.l                       | +020
        bcc.w   .L08e9b8                        | +026
        lea     TaskHandler_08e9d4(pc),a1       | +02a
        move.l  a1,(a6)                         | +02e
.L08e9b8:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +030
        bcc.w   JsrPcThunk_08e9ce               | +034

| ----------------------------------------------------------------------------
|  TaskHandler_08e9d4  @ $08E9D4  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e9d4, "ax", @progbits
        .global TaskHandler_08e9d4
TaskHandler_08e9d4:
        jsr     0x2783a.l                       | +000
        jsr     0x28d70.l                       | +006
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +00c
        bcc.w   JsrPcThunk_08e9f6               | +010

| ----------------------------------------------------------------------------
|  TaskHandler_08e9fc  @ $08E9FC  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08e9fc, "ax", @progbits
        .global TaskHandler_08e9fc
TaskHandler_08e9fc:
        lea     0xffff.w,a0                     | +000
        move.l  a0,0x48(a6)                     | +004
        move.w  #0xff80,0x2a(a6)                | +008
        move.w  #0xffc0,0x2e(a6)                | +00e
        lea     TaskHandler_08ea16(pc),a1       | +014
        move.l  a1,(a6)                         | +018

| ----------------------------------------------------------------------------
|  TaskHandler_08ea16  @ $08EA16  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ea16, "ax", @progbits
        .global TaskHandler_08ea16
TaskHandler_08ea16:
        jsr     0x27c8c.l                       | +000
        bcc.w   .L08ea2c                        | +006
        jsr     0x5b6.l                         | +00a
        jmp     0x518.l                         | +010
.L08ea2c:
        jsr     0x2783a.l                       | +016
        jsr     0x28d70.l                       | +01c
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +022
        bcc.w   Stub_0008EA4E                   | +026

| ----------------------------------------------------------------------------
|  PcThunkTarget_08ea50  @ $08EA50  (28 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_08ea50, "ax", @progbits
        .global PcThunkTarget_08ea50
PcThunkTarget_08ea50:
        cmpi.w  #0x0,0x5c(a6)                   | +000
        beq.w   .L08ea5c                        | +006
        rts                                     | +00a
.L08ea5c:
        movea.l 0xc(a6),a1                      | +00c
        cmpi.l  #0xffffffff,0x48(a1)            | +010
        bne.w   SetHandlerRts_08ea72            | +018

| ----------------------------------------------------------------------------
|  TaskHandler_08ea74  @ $08EA74  (106 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ea74, "ax", @progbits
        .global TaskHandler_08ea74
TaskHandler_08ea74:
        jsr     Sub_0008F108(pc)                | +000
        lea     0x2f419e.l,a0                   | +004
        move.l  a0,0x48(a6)                     | +00a
        bclr    #0x3,0x13(a6)                   | +00e
        move.w  #0x64,0x66(a6)                  | +014
        clr.b   0x20(a6)                        | +01a
.L08ea92:
        subq.b  #0x1,0x98(a6)                   | +01e
        cmpi.b  #0x0,0x98(a6)                   | +022
        blt.w   .L08eab2                        | +028
        lea     TaskHandler_08eb56(pc),a1       | +02c
        jsr     0x4ae.l                         | +030
        jsr     0x5dd02.l                       | +036
        bra.b   .L08ea92                        | +03c
.L08eab2:
        lea     TaskHandler_08ea74__L08eab8(pc),a1 | +03e
        move.l  a1,(a6)                         | +042
        .global TaskHandler_08ea74__L08eab8
TaskHandler_08ea74__L08eab8:
        jsr     0x2783a.l                       | +044
        jsr     0x2870a.l                       | +04a
        bcc.w   .L08eace                        | +050
        lea     TaskHandler_08eaee(pc),a1       | +054
        move.l  a1,(a6)                         | +058
.L08eace:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +05a
        bcc.w   Stub_0008EAEC                   | +05e
        lea     0xffff.w,a0                     | +062
        move.l  a0,0x48(a6)                     | +066

| ----------------------------------------------------------------------------
|  TaskHandler_08eaee  @ $08EAEE  (12 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08eaee, "ax", @progbits
        .global TaskHandler_08eaee
TaskHandler_08eaee:
        move.b  #0x1,0x20(a6)                   | +000
        move.w  #0x14,0x70(a6)                  | +006

| ----------------------------------------------------------------------------
|  TaskHandler_08eb02  @ $08EB02  (68 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08eb02, "ax", @progbits
        .global TaskHandler_08eb02
TaskHandler_08eb02:
        clr.b   0x20(a6)                        | +000
        lea     .L08eb0c(pc),a1                 | +004
        move.l  a1,(a6)                         | +008
.L08eb0c:
        jsr     0x2783a.l                       | +00a
        subq.w  #0x1,0x70(a6)                   | +010
        cmpi.w  #0x0,0x70(a6)                   | +014
        bgt.w   .L08eb36                        | +01a
        bclr    #0x3,0x13(a6)                   | +01e
        move.w  #0x64,0x66(a6)                  | +024
        clr.b   0x20(a6)                        | +02a
        lea     TaskHandler_08ea74__L08eab8(pc),a1 | +02e
        move.l  a1,(a6)                         | +032
.L08eb36:
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +034
        bcc.w   Stub_0008EB54                   | +038
        lea     0xffff.w,a0                     | +03c
        move.l  a0,0x48(a6)                     | +040

| ----------------------------------------------------------------------------
|  TaskHandler_08eb56  @ $08EB56  (22 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08eb56, "ax", @progbits
        .global TaskHandler_08eb56
TaskHandler_08eb56:
        move.w  #0x125,d1                       | +000
        jsr     0x236e.l                        | +004
        lea     0x2f440a.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010

| ----------------------------------------------------------------------------
|  TaskHandler_08eb6c  @ $08EB6C  (42 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08eb6c, "ax", @progbits
        .global TaskHandler_08eb6c
TaskHandler_08eb6c:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x22(a1),d0                     | +004
        subi.w  #0xc,d0                         | +008
        move.w  0x24(a1),d1                     | +00c
        subi.w  #0x18,d1                        | +010
        sub.w   0x22(a6),d0                     | +014
        sub.w   0x24(a6),d1                     | +018
        jsr     0x5e018.l                       | +01c
        move.w  d0,0x34(a6)                     | +022
        bra.w   TaskHandler_08eb96__L08eba4     | +026

| ----------------------------------------------------------------------------
|  TaskHandler_08eb96  @ $08EB96  (180 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08eb96, "ax", @progbits
        .global TaskHandler_08eb96
TaskHandler_08eb96:
        move.w  #0xff,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        move.w  d0,0x34(a6)                     | +00a
        .global TaskHandler_08eb96__L08eba4
TaskHandler_08eb96__L08eba4:
        move.w  #0xff,d0                        | +00e
        jsr     0x5ea1c.l                       | +012
        move.w  d0,d1                           | +018
        addi.w  #0x180,d1                       | +01a
        move.w  0x34(a6),d0                     | +01e
        jsr     0x13c0e.l                       | +022
        move.w  d1,0x28(a6)                     | +028
        move.w  d2,0x2a(a6)                     | +02c
        cmpi.w  #0x0,0x28(a6)                   | +030
        bge.w   .L08ebd8                        | +036
        clr.b   0x3a(a6)                        | +03a
        bra.w   .L08ebde                        | +03e
.L08ebd8:
        move.b  #0x1,0x3a(a6)                   | +042
.L08ebde:
        move.w  #0x3,d0                         | +048
        jsr     0x5ea1c.l                       | +04c
        addi.w  #0x1,d0                         | +052
        move.w  d0,0x70(a6)                     | +056
        lea     .L08ebf6(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ebf6:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ec30                        | +074
        move.w  #0x3,d0                         | +078
        jsr     0x5ea1c.l                       | +07c
        cmpi.w  #0x0,d0                         | +082
        bne.w   .L08ec2a                        | +086
        lea     TaskHandler_08eb6c(pc),a1       | +08a
        move.l  a1,(a6)                         | +08e
        bra.w   .L08ec30                        | +090
.L08ec2a:
        lea     TaskHandler_08eb96(pc),a1       | +094
        move.l  a1,(a6)                         | +098
.L08ec30:
        movea.l 0xc(a6),a1                      | +09a
        cmpi.b  #0x0,0x20(a1)                   | +09e
        beq.w   .L08ec44                        | +0a4
        lea     TaskHandler_08ec50(pc),a1       | +0a8
        move.l  a1,(a6)                         | +0ac
.L08ec44:
        jsr     0x28d70.l                       | +0ae

| ----------------------------------------------------------------------------
|  TaskHandler_08ec50  @ $08EC50  (132 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ec50, "ax", @progbits
        .global TaskHandler_08ec50
TaskHandler_08ec50:
        movea.l 0xc(a6),a1                      | +000
        move.w  0x22(a6),d0                     | +004
        addi.w  #0xc,d0                         | +008
        move.w  0x24(a6),d1                     | +00c
        addi.w  #0x18,d1                        | +010
        sub.w   0x22(a1),d0                     | +014
        sub.w   0x24(a1),d1                     | +018
        jsr     0x5e018.l                       | +01c
        move.w  d0,0x34(a6)                     | +022
        move.w  #0x300,d1                       | +026
        move.w  0x34(a6),d0                     | +02a
        jsr     0x13c0e.l                       | +02e
        move.w  d1,0x28(a6)                     | +034
        move.w  d2,0x2a(a6)                     | +038
        cmpi.w  #0x0,0x28(a6)                   | +03c
        bge.w   .L08ec9e                        | +042
        clr.b   0x3a(a6)                        | +046
        bra.w   .L08eca4                        | +04a
.L08ec9e:
        move.b  #0x1,0x3a(a6)                   | +04e
.L08eca4:
        move.w  #0x4,0x70(a6)                   | +054
        lea     .L08ecb0(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ecb0:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ecce                        | +074
        lea     TaskHandler_08eb6c(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L08ecce:
        jsr     0x28d70.l                       | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_08ecda  @ $08ECDA  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ecda, "ax", @progbits
        .global TaskHandler_08ecda
TaskHandler_08ecda:
        jsr     Sub_0008F108(pc)                | +000
        jsr     Sub_0008F002(pc)                | +004
        move.w  #0x125,d1                       | +008
        jsr     0x236e.l                        | +00c
        lea     0x2f440a.l,a0                   | +012
        jsr     0x28cd4.l                       | +018

| ----------------------------------------------------------------------------
|  TaskHandler_08ecf8  @ $08ECF8  (132 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ecf8, "ax", @progbits
        .global TaskHandler_08ecf8
TaskHandler_08ecf8:
        move.w  #0xff,d0                        | +000
        jsr     0x5ea1c.l                       | +004
        move.w  d0,0x34(a6)                     | +00a
        move.w  #0xff,d0                        | +00e
        jsr     0x5ea1c.l                       | +012
        move.w  d0,d1                           | +018
        addi.w  #0x100,d1                       | +01a
        move.w  0x34(a6),d0                     | +01e
        jsr     0x13c0e.l                       | +022
        move.w  d1,0x28(a6)                     | +028
        move.w  d2,0x2a(a6)                     | +02c
        move.w  #0x3,d0                         | +030
        jsr     0x5ea1c.l                       | +034
        addi.w  #0x2,d0                         | +03a
        move.w  d0,0x70(a6)                     | +03e
        cmpi.w  #0x0,0x28(a6)                   | +042
        bge.w   .L08ed4c                        | +048
        clr.b   0x3a(a6)                        | +04c
        bra.w   .L08ed52                        | +050
.L08ed4c:
        move.b  #0x1,0x3a(a6)                   | +054
.L08ed52:
        lea     .L08ed58(pc),a1                 | +05a
        move.l  a1,(a6)                         | +05e
.L08ed58:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +060
        jsr     0x2783a.l                       | +064
        subq.w  #0x1,0x70(a6)                   | +06a
        cmpi.w  #0x0,0x70(a6)                   | +06e
        bgt.w   .L08ed76                        | +074
        lea     TaskHandler_08ecf8(pc),a1       | +078
        move.l  a1,(a6)                         | +07c
.L08ed76:
        jsr     0x28d70.l                       | +07e

| ----------------------------------------------------------------------------
|  TaskHandler_08ed82  @ $08ED82  (58 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ed82, "ax", @progbits
        .global TaskHandler_08ed82
TaskHandler_08ed82:
        move.w  #0x0,0x38(a6)                   | +000
        jsr     Sub_0008F002(pc)                | +006
        move.w  #0x12a,d1                       | +00a
        jsr     0x236e.l                        | +00e
        lea     0x2f4426.l,a0                   | +014
        jsr     0x28cd4.l                       | +01a
        lea     .L08eda8(pc),a1                 | +020
        move.l  a1,(a6)                         | +024
.L08eda8:
        jsr     0x2783a.l                       | +026
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +02c
        bcc.w   JsrAbsThunk_08edbe              | +030
        jmp     0x518.l                         | +034

| ----------------------------------------------------------------------------
|  TaskHandler_08edbc  @ $08EDBC  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08edbc, "ax", @progbits
        .global TaskHandler_08edbc
TaskHandler_08edbc:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Sub_0008EDC6  @ $08EDC6  (70 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008EDC6, "ax", @progbits
        .global Sub_0008EDC6
Sub_0008EDC6:
        lea     0x2f4442.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   .L08ede2                        | +00c
        lea     0x2f44ae.l,a0                   | +010
        jsr     0x28cd4.l                       | +016
.L08ede2:
        move.w  #0x0,0x38(a6)                   | +01c
        move.w  #0xf4,d1                        | +022
        jsr     0x236e.l                        | +026
        lea     .L08edf8(pc),a1                 | +02c
        move.l  a1,(a6)                         | +030
.L08edf8:
        jsr     0x4407a.l                       | +032
        jsr     Screen_InBoundsX_Latched_08d184(pc) | +038
        bcc.w   JsrAbsThunk_08ee0e              | +03c
        jmp     0x518.l                         | +040

| ----------------------------------------------------------------------------
|  TaskHandler_08ee0c  @ $08EE0C  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ee0c, "ax", @progbits
        .global TaskHandler_08ee0c
TaskHandler_08ee0c:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  TaskHandler_08ee16  @ $08EE16  (92 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ee16, "ax", @progbits
        .global TaskHandler_08ee16
TaskHandler_08ee16:
        jsr     Sub_0008F002(pc)                | +000
        jsr     Sub_0008F010(pc)                | +004
        clr.w   d0                              | +008
        move.b  0x99(a6),d0                     | +00a
        lsl.w   #0x5,d0                         | +00e
        neg.w   d0                              | +010
        move.w  d0,0x2a(a6)                     | +012
        move.w  #0x0,0x38(a6)                   | +016
        move.w  #0xf4,d1                        | +01c
        jsr     0x236e.l                        | +020
        lea     0x2f44ca.l,a0                   | +026
        jsr     0x28cd4.l                       | +02c
        lea     .L08ee4e(pc),a1                 | +032
        move.l  a1,(a6)                         | +036
.L08ee4e:
        jsr     Pos_IntegrateXY88_08d2f8(pc)    | +038
        cmpi.w  #0x190,0x24(a6)                 | +03c
        bgt.w   .L08ee6e                        | +042
        lea     0x2f44d6.l,a0                   | +046
        jsr     0x28cd4.l                       | +04c
        lea     TaskHandler_08ee7a(pc),a1       | +052
        move.l  a1,(a6)                         | +056
.L08ee6e:
        jsr     PcThunkTarget_08efb0(pc)        | +058

| ----------------------------------------------------------------------------
|  TaskHandler_08ee7a  @ $08EE7A  (28 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ee7a, "ax", @progbits
        .global TaskHandler_08ee7a
TaskHandler_08ee7a:
        jsr     0x4407a.l                       | +000
        jsr     PcThunkTarget_08efb0(pc)        | +006
        jsr     0x28d70.l                       | +00a
        bcc.w   .L08ee94                        | +010
        jmp     0x518.l                         | +014
.L08ee94:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_08ee96  @ $08EE96  (162 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ee96, "ax", @progbits
        .global TaskHandler_08ee96
TaskHandler_08ee96:
        lea     0x2f40c6.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        move.b  #0x0,0x74(a6)                   | +00c
        bra.w   .L08eed4                        | +012
        lea     0x2f40f2.l,a0                   | +016
        jsr     0x28cd4.l                       | +01c
        move.b  #0x1,0x74(a6)                   | +022
        bra.w   .L08eed4                        | +028
        lea     0x2f411e.l,a0                   | +02c
        jsr     0x28cd4.l                       | +032
        move.b  #0x2,0x74(a6)                   | +038
.L08eed4:
        move.w  #0x1f,d0                        | +03e
        jsr     0x5ea1c.l                       | +042
        neg.w   d0                              | +048
        move.w  d0,0x2a(a6)                     | +04a
        move.w  #0x1f,d0                        | +04e
        jsr     0x5ea1c.l                       | +052
        subi.w  #0x30,d0                        | +058
        move.w  d0,0x2e(a6)                     | +05c
        jsr     Sub_0008F002(pc)                | +060
        move.w  #0xad,d1                        | +064
        jsr     0x236e.l                        | +068
        movea.l 0xc(a6),a1                      | +06e
        movea.l 0xc(a1),a2                      | +072
        move.w  0x24(a2),0x5c(a6)               | +076
        lea     .L08ef18(pc),a1                 | +07c
        move.l  a1,(a6)                         | +080
.L08ef18:
        jsr     Pos_IntegrateXY88_Accel_08d34e(pc) | +082
        move.w  0x5c(a6),d0                     | +086
        cmp.w   0x24(a6),d0                     | +08a
        blt.w   .L08ef2e                        | +08e
        lea     TaskHandler_08ef40(pc),a1       | +092
        move.l  a1,(a6)                         | +096
.L08ef2e:
        jsr     0x2783a.l                       | +098
        jsr     PcThunkTarget_08efb0(pc)        | +09e

| ----------------------------------------------------------------------------
|  TaskHandler_08ef40  @ $08EF40  (112 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08ef40, "ax", @progbits
        .global TaskHandler_08ef40
TaskHandler_08ef40:
        cmpi.b  #0x0,0x74(a6)                   | +000
        bne.w   .L08ef5a                        | +006
        lea     0x2f40d2.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        bra.w   .L08ef8e                        | +016
.L08ef5a:
        cmpi.b  #0x0,0x74(a6)                   | +01a
        bne.w   .L08ef74                        | +020
        lea     0x2f40fe.l,a0                   | +024
        jsr     0x28cd4.l                       | +02a
        bra.w   .L08ef8e                        | +030
.L08ef74:
        cmpi.b  #0x0,0x74(a6)                   | +034
        bne.w   .L08ef8e                        | +03a
        lea     0x2f412a.l,a0                   | +03e
        jsr     0x28cd4.l                       | +044
        bra.w   .L08ef8e                        | +04a
.L08ef8e:
        lea     .L08ef94(pc),a1                 | +04e
        move.l  a1,(a6)                         | +052
.L08ef94:
        jsr     0x2783a.l                       | +054
        jsr     PcThunkTarget_08efb0(pc)        | +05a
        jsr     0x28d70.l                       | +05e
        bcc.w   .L08efae                        | +064
        jmp     0x518.l                         | +068
.L08efae:
        rts                                     | +06e

| ----------------------------------------------------------------------------
|  PcThunkTarget_08efb0  @ $08EFB0  (16 B)
| ----------------------------------------------------------------------------
        .section .text.PcThunkTarget_08efb0, "ax", @progbits
        .global PcThunkTarget_08efb0
PcThunkTarget_08efb0:
        movea.l #0xffffffff,a0                  | +000
        jsr     0x5dd5c.l                       | +006
        bcc.w   Jsr5B6Rts_08efcc                | +00c

| ----------------------------------------------------------------------------
|  Sub_0008EFCE  @ $08EFCE  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008EFCE, "ax", @progbits
        .global Sub_0008EFCE
Sub_0008EFCE:
        lea     0x100440.l,a1                   | +000
        move.w  0x22(a6),d1                     | +006
        sub.w   0x22(a1),d1                     | +00a
        cmp.w   d0,d1                           | +00e
        blt.w   SetXN_08effc                    | +010
        lea     0x1004e0.l,a1                   | +014
        move.w  0x22(a6),d1                     | +01a
        sub.w   0x22(a1),d1                     | +01e
        cmp.w   d0,d1                           | +022
        blt.w   SetXN_08effc                    | +024

| ----------------------------------------------------------------------------
|  Sub_0008F002  @ $08F002  (14 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F002, "ax", @progbits
        .global Sub_0008F002
Sub_0008F002:
        andi.b  #0x1,0x98(a6)                   | +000
        move.b  0x98(a6),0x3a(a6)               | +006
        rts                                     | +00c

| ----------------------------------------------------------------------------
|  Sub_0008F010  @ $08F010  (28 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F010, "ax", @progbits
        .global Sub_0008F010
Sub_0008F010:
        clr.w   d0                              | +000
        move.b  0x99(a6),d0                     | +002
        lsl.w   #0x5,d0                         | +006
        move.w  d0,0x28(a6)                     | +008
        cmpi.b  #0x0,0x3a(a6)                   | +00c
        bne.w   .L08f02a                        | +012
        neg.w   0x28(a6)                        | +016
.L08f02a:
        rts                                     | +01a

| ----------------------------------------------------------------------------
|  Sub_0008F02C  @ $08F02C  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F02C, "ax", @progbits
        .global Sub_0008F02C
Sub_0008F02C:
        clr.w   d0                              | +000
        move.b  0x99(a6),d0                     | +002
        move.w  0x106f50.l,0x72(a6)             | +006
        add.w   d0,0x72(a6)                     | +00e
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Sub_0008F040  @ $08F040  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F040, "ax", @progbits
        .global Sub_0008F040
Sub_0008F040:
        cmpi.b  #0x0,0x99(a6)                   | +000
        bne.w   TaskHandler_08f050              | +006

| ----------------------------------------------------------------------------
|  TaskHandler_08f050  @ $08F050  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f050, "ax", @progbits
        .global TaskHandler_08f050
TaskHandler_08f050:
        move.w  0x106f50.l,d0                   | +000
        cmp.w   0x72(a6),d0                     | +006
        bcs.w   ClearXN_08f068                  | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_08f064  @ $08F064  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f064, "ax", @progbits
        .global TaskHandler_08f064
TaskHandler_08f064:
        bra.w   TaskHandler_08f06e              | +000

| ----------------------------------------------------------------------------
|  TaskHandler_08f06e  @ $08F06E  (2 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f06e, "ax", @progbits
        .global TaskHandler_08f06e
TaskHandler_08f06e:
        rts                                     | +000

| ----------------------------------------------------------------------------
|  Sub_0008F070  @ $08F070  (12 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F070, "ax", @progbits
        .global Sub_0008F070
Sub_0008F070:
        clr.w   d0                              | +000
        move.b  0x9a(a6),d0                     | +002
        addi.w  #0xfa,d0                        | +006
        move.w  d0,d1                           | +00a

| ----------------------------------------------------------------------------
|  Sub_0008F084  @ $08F084  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F084, "ax", @progbits
        .global Sub_0008F084
Sub_0008F084:
        cmpi.b  #0x0,0x9a(a6)                   | +000
        bne.w   .L08f098                        | +006
        move.w  #0x14a,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L08f098:
        cmpi.b  #0x1,0x9a(a6)                   | +014
        bne.w   .L08f0ac                        | +01a
        move.w  #0xf6,d1                        | +01e
        jsr     0x236e.l                        | +022
.L08f0ac:
        cmpi.b  #0x1,0x9a(a6)                   | +028
        bne.w   .L08f0c4                        | +02e
        move.w  #0x13d,d1                       | +032
        jsr     0x236e.l                        | +036
        bra.w   JsrAbsRts_08f0ce                | +03c
.L08f0c4:
        move.w  #0x14a,d1                       | +040

| ----------------------------------------------------------------------------
|  Sub_0008F0D0  @ $08F0D0  (48 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F0D0, "ax", @progbits
        .global Sub_0008F0D0
Sub_0008F0D0:
        cmpi.b  #0x0,0x9a(a6)                   | +000
        bne.w   .L08f0e4                        | +006
        move.w  #0x149,d1                       | +00a
        jsr     0x236e.l                        | +00e
.L08f0e4:
        cmpi.b  #0x1,0x9a(a6)                   | +014
        bne.w   .L08f0fc                        | +01a
        move.w  #0xf7,d1                        | +01e
        jsr     0x236e.l                        | +022
        bra.w   JsrAbsRts_08f106                | +028
.L08f0fc:
        move.w  #0x149,d1                       | +02c

| ----------------------------------------------------------------------------
|  Sub_0008F108  @ $08F108  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F108, "ax", @progbits
        .global Sub_0008F108
Sub_0008F108:
        move.w  #0x8000,0x38(a6)                | +000
        andi.w  #0xffe3,0x38(a6)                | +006
        ori.w   #0x18,0x38(a6)                  | +00c
        rts                                     | +012

| ----------------------------------------------------------------------------
|  TaskHandler_08f11c  @ $08F11C  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f11c, "ax", @progbits
        .global TaskHandler_08f11c
TaskHandler_08f11c:
        movea.l 0x8(a6),a1                      | +000
        move.b  0x10(a6),d0                     | +004
        cmp.b   0x10(a1),d0                     | +008
        bcs.w   SetXN_08f132                    | +00c

| ----------------------------------------------------------------------------
|  Sub_0008F138  @ $08F138  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F138, "ax", @progbits
        .global Sub_0008F138
Sub_0008F138:
        clr.w   (a0)                            | +000
        clr.w   0x2(a0)                         | +002
        clr.w   0x4(a0)                         | +006
        clr.w   0x6(a0)                         | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Sub_0008F148  @ $08F148  (16 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0008F148, "ax", @progbits
        .global Sub_0008F148
Sub_0008F148:
        move.w  0x4(a0),(a0)                    | +000
        move.w  0x2(a0),0x4(a0)                 | +004
        clr.w   0x6(a0)                         | +00a
        rts                                     | +00e

| ----------------------------------------------------------------------------
|  Subsystem_AudioSceneInit_08F158  @ $08F158  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Subsystem_AudioSceneInit_08F158, "ax", @progbits
        .global Subsystem_AudioSceneInit_08F158
Subsystem_AudioSceneInit_08F158:
        lea     0x10e2f2.l,a0                   | +000
        bsr.b   Sub_0008F138                    | +006
        lea     0x10e33a.l,a0                   | +008
        bsr.b   Sub_0008F138                    | +00e
        lea     0x10e362.l,a0                   | +010
        bsr.b   Sub_0008F138                    | +016
        rts                                     | +018
        lea     0x10e2f2.l,a0                   | +01a
        bsr.b   Sub_0008F148                    | +020
        lea     0x10e33a.l,a0                   | +022
        bsr.b   Sub_0008F148                    | +028
        lea     0x10e362.l,a0                   | +02a
        bsr.b   Sub_0008F148                    | +030
        rts                                     | +032

| ----------------------------------------------------------------------------
|  TaskHandler_08f18c  @ $08F18C  (82 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f18c, "ax", @progbits
        .global TaskHandler_08f18c
TaskHandler_08f18c:
        move.w  0x22(a6),d0                     | +000
        add.w   0x106f50.l,d0                   | +004
        move.w  d0,0x22(a6)                     | +00a
        clr.w   d1                              | +00e
        move.b  0x98(a6),d1                     | +010
        lsl.w   #0x4,d1                         | +014
        add.w   d1,d0                           | +016
        move.w  d0,0x70(a6)                     | +018
        move.w  0x24(a6),d0                     | +01c
        subi.w  #0x200,d0                       | +020
        neg.w   d0                              | +024
        add.w   0x106f54.l,d0                   | +026
        move.w  d0,0x24(a6)                     | +02c
        clr.w   d1                              | +030
        move.b  0x99(a6),d1                     | +032
        lsl.w   #0x4,d1                         | +036
        add.w   d1,d0                           | +038
        move.w  d0,0x72(a6)                     | +03a
        lea     .L08f1d0(pc),a1                 | +03e
        move.l  a1,(a6)                         | +042
.L08f1d0:
        move.w  0x70(a6),d0                     | +044
        cmp.w   0x106f50.l,d0                   | +048
        bcc.w   TaskHandler_08f1ec              | +04e

| ----------------------------------------------------------------------------
|  TaskHandler_08f1ec  @ $08F1EC  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f1ec, "ax", @progbits
        .global TaskHandler_08f1ec
TaskHandler_08f1ec:
        lea     0x10e2f2.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        adda.w  0x2(a0),a1                      | +00a
        move.w  0x22(a6),(a1)                   | +00e
        move.w  0x24(a6),0x2(a1)                | +012
        move.w  0x70(a6),0x4(a1)                | +018
        move.w  0x72(a6),0x6(a1)                | +01e
        cmpi.w  #0x4,0x6(a0)                    | +024
        bcc.w   .L08f232                        | +02a
        move.w  0x2(a0),d7                      | +02e
        addq.w  #0x8,d7                         | +032
        cmpi.w  #0x40,d7                        | +034
        bcs.w   .L08f22a                        | +038
        clr.w   d7                              | +03c
.L08f22a:
        move.w  d7,0x2(a0)                      | +03e
        addq.w  #0x1,0x6(a0)                    | +042
.L08f232:
        rts                                     | +046

| ----------------------------------------------------------------------------
|  TaskHandler_08f234  @ $08F234  (108 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f234, "ax", @progbits
        .global TaskHandler_08f234
TaskHandler_08f234:
        move.w  0x22(a6),d0                     | +000
        move.w  0x24(a6),d1                     | +004
        add.w   0x106f50.l,d0                   | +008
        subi.w  #0x200,d1                       | +00e
        neg.w   d1                              | +012
        add.w   0x106f54.l,d1                   | +014
        move.w  d0,d2                           | +01a
        move.w  d1,d3                           | +01c
        add.w   (a0),d0                         | +01e
        add.w   0x2(a0),d2                      | +020
        add.w   0x4(a0),d1                      | +024
        add.w   0x6(a0),d3                      | +028
        lea     0x10e2f2.l,a0                   | +02c
        lea     0x8(a0),a1                      | +032
        adda.w  0x2(a0),a1                      | +036
        move.w  d0,(a1)                         | +03a
        move.w  d1,0x2(a1)                      | +03c
        move.w  d2,0x4(a1)                      | +040
        move.w  d3,0x6(a1)                      | +044
        cmpi.w  #0x4,0x6(a0)                    | +048
        bcc.w   .L08f29e                        | +04e
        move.w  0x2(a0),d7                      | +052
        addq.w  #0x8,d7                         | +056
        cmpi.w  #0x40,d7                        | +058
        bcs.w   .L08f296                        | +05c
        clr.w   d7                              | +060
.L08f296:
        move.w  d7,0x2(a0)                      | +062
        addq.w  #0x1,0x6(a0)                    | +066
.L08f29e:
        rts                                     | +06a

| ----------------------------------------------------------------------------
|  TaskHandler_08f2a0  @ $08F2A0  (78 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f2a0, "ax", @progbits
        .global TaskHandler_08f2a0
TaskHandler_08f2a0:
        move.w  0x22(a6),d0                     | +000
        add.w   0x106f50.l,d0                   | +004
        move.w  0x24(a6),d1                     | +00a
        subi.w  #0x200,d1                       | +00e
        neg.w   d1                              | +012
        add.w   0x106f54.l,d1                   | +014
        lea     0x10e2f2.l,a0                   | +01a
        lea     0x8(a0),a1                      | +020
        move.w  (a0),d7                         | +024
        .global TaskHandler_08f2a0__L08f2c6
TaskHandler_08f2a0__L08f2c6:
        cmp.w   0x4(a0),d7                      | +026
        beq.w   ClearC_08f302                   | +02a
        cmp.w   (a1,d7.w),d0                    | +02e
        bcs.w   TaskHandler_08f2f4              | +032
        cmp.w   0x4(a1,d7.w),d0                 | +036
        bhi.w   TaskHandler_08f2f4              | +03a
        cmp.w   0x2(a1,d7.w),d1                 | +03e
        bcs.w   TaskHandler_08f2f4              | +042
        cmp.w   0x6(a1,d7.w),d1                 | +046
        bhi.w   TaskHandler_08f2f4              | +04a

| ----------------------------------------------------------------------------
|  TaskHandler_08f2f4  @ $08F2F4  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f2f4, "ax", @progbits
        .global TaskHandler_08f2f4
TaskHandler_08f2f4:
        addq.w  #0x8,d7                         | +000
        cmpi.w  #0x40,d7                        | +002
        bcs.w   .L08f300                        | +006
        clr.w   d7                              | +00a
.L08f300:
        bra.b   TaskHandler_08f2a0__L08f2c6     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_08f344  @ $08F344  (72 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f344, "ax", @progbits
        .global TaskHandler_08f344
TaskHandler_08f344:
        lea     0x10e33a.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.w  (a0),d7                         | +00a
        .global TaskHandler_08f344__L08f350
TaskHandler_08f344__L08f350:
        cmp.w   0x4(a0),d7                      | +00c
        beq.w   ClearC_08f3a0                   | +010
        move.w  (a1,d7.w),d0                    | +014
        move.w  0x2(a1,d7.w),d1                 | +018
        sub.w   0x22(a6),d0                     | +01c
        smi.b   d2                              | +020
        sub.w   0x24(a6),d1                     | +022
        addi.w  #0x40,d0                        | +026
        addi.w  #0x30,d1                        | +02a
        cmpi.w  #0x80,d0                        | +02e
        bcc.w   TaskHandler_08f392              | +032
        cmpi.w  #0x60,d1                        | +036
        bcc.w   TaskHandler_08f392              | +03a
        btst    #0x0,0x3a(a6)                   | +03e
        seq.b   d0                              | +044
        eor.b   d2,d0                           | +046

| ----------------------------------------------------------------------------
|  TaskHandler_08f392  @ $08F392  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f392, "ax", @progbits
        .global TaskHandler_08f392
TaskHandler_08f392:
        addq.w  #0x4,d7                         | +000
        cmpi.w  #0x20,d7                        | +002
        bcs.w   .L08f39e                        | +006
        clr.w   d7                              | +00a
.L08f39e:
        bra.b   TaskHandler_08f344__L08f350     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_08f3a6  @ $08F3A6  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f3a6, "ax", @progbits
        .global TaskHandler_08f3a6
TaskHandler_08f3a6:
        move.b  0x10e2f0.l,d0                   | +000
        addq.b  #0x1,d0                         | +006
        andi.b  #0x3f,d0                        | +008
        addi.b  #0x40,d0                        | +00c
        move.b  d0,0x10e2f0.l                   | +010
        rts                                     | +016

| ----------------------------------------------------------------------------
|  TaskHandler_08f3be  @ $08F3BE  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_08f3be, "ax", @progbits
        .global TaskHandler_08f3be
TaskHandler_08f3be:
        lea     0x10e362.l,a0                   | +000
        lea     0x8(a0),a1                      | +006
        move.b  #0xfe,d3                        | +00a
        move.w  (a0),d7                         | +00e
.L08f3ce:
        cmp.w   0x4(a0),d7                      | +010
        beq.w   .L08f3f8                        | +014
        cmp.b   0x4(a1,d7.w),d2                 | +018
        bne.w   .L08f3ea                        | +01c
        move.b  0x5(a1,d7.w),d4                 | +020
        bmi.w   .L08f3ea                        | +024
        move.b  #0xff,d3                        | +028
.L08f3ea:
        addq.w  #0x6,d7                         | +02c
        cmpi.w  #0x30,d7                        | +02e
        bcs.w   .L08f3f6                        | +032
        clr.w   d7                              | +036
.L08f3f6:
        bra.b   .L08f3ce                        | +038
.L08f3f8:
        move.w  0x22(a6),d5                     | +03a
        subi.w  #0x10,d5                        | +03e
