| ============================================================================
|  Metal Slug 1 (Neo Geo, M68000) — decompilación matching
|  Wave ??? — (borrador)
|  Región: $048A44..$049430  (2,242 B, 45 entradas, 31 huecos)
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
|  Sub_00048A44  @ $048A44  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048A44, "ax", @progbits
        .global Sub_00048A44
Sub_00048A44:
        jsr     Sub_0004936E(pc)                | +000
        subq.w  #0x1,0x72(a6)                   | +004
        jsr     0x2870a.l                       | +008
        bcc.w   .L048a72                        | +00e
        lea     Sub_00048CA4(pc),a1             | +012
        jsr     0x4ae.l                         | +016
        jsr     0x5dd02.l                       | +01c
        addi.w  #0x10,0x24(a0)                  | +022
        jsr     0x49ff2.l                       | +028
.L048a72:
        movea.l #0xffffffff,a0                  | +02e
        lea     0x28e196.l,a0                   | +034
        jsr     0x5dd5c.l                       | +03a
        bcc.w   SetHandlerRts_048a8e            | +040

| ----------------------------------------------------------------------------
|  Sub_00048A90  @ $048A90  (134 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048A90, "ax", @progbits
        .global Sub_00048A90
Sub_00048A90:
        jsr     Sub_0004936E(pc)                | +000
        subq.w  #0x1,0x72(a6)                   | +004
        jsr     0x2870a.l                       | +008
        bcc.w   .L048ad2                        | +00e
        lea     Sub_00048CA4(pc),a1             | +012
        jsr     0x4ae.l                         | +016
        jsr     0x5dd02.l                       | +01c
        addi.w  #0x18,0x24(a0)                  | +022
        movea.l 0x50(a6),a1                     | +028
        move.w  0x28(a1),d0                     | +02c
        asr.w   #0x4,d0                         | +030
        move.w  d0,0x28(a0)                     | +032
        jsr     0x49fd0.l                       | +036
        move.b  #0x2,0x83(a6)                   | +03c
.L048ad2:
        cmpi.b  #0x0,0x9e(a6)                   | +042
        bge.w   .L048af0                        | +048
        cmpi.w  #0xffe0,0x22(a6)                | +04c
        bgt.w   .L048aec                        | +052
        lea     JmpToScheduler_048b1e(pc),a1    | +056
        move.l  a1,(a6)                         | +05a
.L048aec:
        bra.w   .L048b00                        | +05c
.L048af0:
        cmpi.w  #0x160,0x22(a6)                 | +060
        blt.w   .L048b00                        | +066
        lea     JmpToScheduler_048b1e(pc),a1    | +06a
        move.l  a1,(a6)                         | +06e
.L048b00:
        movea.l #0xffffffff,a0                  | +070
        lea     0x28e19e.l,a0                   | +076
        jsr     0x5dd5c.l                       | +07c
        bcc.w   SetHandlerRts_048b1c            | +082

| ----------------------------------------------------------------------------
|  TaskHandler_048b34  @ $048B34  (34 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048b34, "ax", @progbits
        .global TaskHandler_048b34
TaskHandler_048b34:
        move.w  #0x2000,d0                      | +000
        jsr     0x28134.l                       | +004
        andi.w  #0xffe3,0x38(a6)                | +00a
        ori.w   #0x14,0x38(a6)                  | +010
        jsr     0x13600.l                       | +016
        jmp     0x77f6a.l                       | +01c

| ----------------------------------------------------------------------------
|  TaskHandler_048b56  @ $048B56  (66 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048b56, "ax", @progbits
        .global TaskHandler_048b56
TaskHandler_048b56:
        move.w  #0x4,d1                         | +000
        jsr     0x236e.l                        | +004
        lea     0x28f556.l,a0                   | +00a
        jsr     0x28cd4.l                       | +010
        lea     .L048b72(pc),a1                 | +016
        move.l  a1,(a6)                         | +01a
.L048b72:
        jsr     0x2783a.l                       | +01c
        jsr     0x28d70.l                       | +022
        bcc.w   .L048b88                        | +028
        lea     Jsr5B6ThenJmpScheduler_048b26(pc),a1 | +02c
        move.l  a1,(a6)                         | +030
.L048b88:
        movea.l #0xffffffff,a0                  | +032
        jsr     0x5dd56.l                       | +038
        bcc.w   SetHandlerRts_048b9e            | +03e

| ----------------------------------------------------------------------------
|  TaskHandler_048ba0  @ $048BA0  (252 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048ba0, "ax", @progbits
        .global TaskHandler_048ba0
TaskHandler_048ba0:
        lea     0x2bfdb8.l,a0                   | +000
        jsr     0x799de.l                       | +006
        btst    #0x0,0x3a(a6)                   | +00c
        bne.w   .L048bb8                        | +012
        neg.w   d0                              | +016
.L048bb8:
        move.w  d0,0x28(a6)                     | +018
        clr.w   0x2a(a6)                        | +01c
        bra.w   .L048c04                        | +020
        move.b  0x98(a6),0x7b(a6)               | +024
        lea     0x2bfdb8.l,a0                   | +02a
        jsr     0x799de.l                       | +030
        move.b  0x7b(a6),d3                     | +036
        andi.w  #0xf,d3                         | +03a
        lsl.w   #0x5,d3                         | +03e
        lea     0x2c07ac.l,a1                   | +040
        lea     0x2c072c.l,a2                   | +046
        move.w  (a1,d3.w),d1                    | +04c
        move.w  (a2,d3.w),d2                    | +050
        muls.w  d0,d1                           | +054
        muls.w  d0,d2                           | +056
        asr.l   #0x8,d1                         | +058
        asr.l   #0x8,d2                         | +05a
        move.w  d1,0x28(a6)                     | +05c
        move.w  d2,0x2a(a6)                     | +060
.L048c04:
        move.w  #0x96,0x70(a6)                  | +064
        move.w  #0x17b,d1                       | +06a
        jsr     0x236e.l                        | +06e
        move.w  #0xd000,d0                      | +074
        jsr     0x28134.l                       | +078
        andi.w  #0xffe3,0x38(a6)                | +07e
        ori.w   #0x14,0x38(a6)                  | +084
        bset    #0x4,0x6b(a6)                   | +08a
        move.w  #0x1064,d0                      | +090
        jsr     0x2352.l                        | +094
        lea     0x28f51c.l,a0                   | +09a
        jsr     0x28cd4.l                       | +0a0
        lea     .L048c4c(pc),a1                 | +0a6
        move.l  a1,(a6)                         | +0aa
.L048c4c:
        jsr     0x27cee.l                       | +0ac
        bcc.w   .L048c5c                        | +0b2
        lea     TaskHandler_048b34(pc),a1       | +0b6
        move.l  a1,(a6)                         | +0ba
.L048c5c:
        jsr     0x28d70.l                       | +0bc
        subq.w  #0x1,0x70(a6)                   | +0c2
        cmpi.w  #0x0,0x70(a6)                   | +0c6
        bgt.w   .L048c76                        | +0cc
        lea     TaskHandler_048b34(pc),a1       | +0d0
        move.l  a1,(a6)                         | +0d4
.L048c76:
        jsr     0x283d8.l                       | +0d6
        btst    #0x1,0x13(a6)                   | +0dc
        beq.w   .L048c8c                        | +0e2
        lea     TaskHandler_048b34(pc),a1       | +0e6
        move.l  a1,(a6)                         | +0ea
.L048c8c:
        movea.l #0xffffffff,a0                  | +0ec
        jsr     0x5dd56.l                       | +0f2
        bcc.w   SetHandlerRts_048ca2            | +0f8

| ----------------------------------------------------------------------------
|  Sub_00048CA4  @ $048CA4  (98 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048CA4, "ax", @progbits
        .global Sub_00048CA4
Sub_00048CA4:
        move.w  #0xffbc,d0                      | +000
        jsr     0x5dca4.l                       | +004
        move.w  d0,0x28(a6)                     | +00a
        move.w  #0x32a,0x2a(a6)                 | +00e
        move.w  #0xffca,0x2e(a6)                | +014
        move.w  #0x0,0x2c(a6)                   | +01a
        move.w  #0x38,d1                        | +020
        jsr     0x236e.l                        | +024
        lea     0x29ca36.l,a0                   | +02a
        jsr     0x28cd4.l                       | +030
        lea     .L048ce0(pc),a1                 | +036
        move.l  a1,(a6)                         | +03a
.L048ce0:
        jsr     0x27bc8.l                       | +03c
        bcc.w   .L048cf0                        | +042
        lea     Jsr5B6ThenJmpScheduler_048b26(pc),a1 | +046
        move.l  a1,(a6)                         | +04a
.L048cf0:
        jsr     0x28d70.l                       | +04c
        movea.l #0xffffffff,a0                  | +052
        jsr     0x5dd56.l                       | +058
        bcc.w   SetHandlerRts_048d0c            | +05e

| ----------------------------------------------------------------------------
|  Sub_00048D0E  @ $048D0E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048D0E, "ax", @progbits
        .global Sub_00048D0E
Sub_00048D0E:
        move.w  #0x18d,d1                       | +000
        jsr     0x236e.l                        | +004
        move.l  #0x28e0e6,0x48(a6)              | +00a
        lea     0x2bfe8a.l,a0                   | +012
        jsr     0x799de.l                       | +018
        move.w  d0,0x66(a6)                     | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_048d30  @ $048D30  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048d30, "ax", @progbits
        .global TaskHandler_048d30
TaskHandler_048d30:
        lea     0x28ef54.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        lea     .L048d42(pc),a1                 | +00c
        move.l  a1,(a6)                         | +010
.L048d42:
        jsr     0x5e45a.l                       | +012
        bcs.w   Jsr5B6ThenJmpScheduler_048b26   | +018
        jsr     0x5e506.l                       | +01c
        move.w  0x78(a0),0x78(a6)               | +022
        subq.w  #0x1,0x38(a6)                   | +028
        jsr     0x28d70.l                       | +02c
        movea.l 0xc(a6),a0                      | +032
        cmpi.b  #0x1,0x83(a0)                   | +036
        bne.w   .L048d76                        | +03c
        lea     TaskHandler_048d7a(pc),a1       | +040
        move.l  a1,(a6)                         | +044
.L048d76:
        bra.w   TaskHandler_048e2c              | +046

| ----------------------------------------------------------------------------
|  TaskHandler_048d7a  @ $048D7A  (98 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048d7a, "ax", @progbits
        .global TaskHandler_048d7a
TaskHandler_048d7a:
        movea.l 0xc(a6),a0                      | +000
        move.w  0x78(a0),d0                     | +004
        lsr.w   #0x1,d0                         | +008
        andi.w  #0xf,d0                         | +00a
        movea.l #0x28e26e,a0                    | +00e
        lsl.w   #0x2,d0                         | +014
        movea.l (a0,d0.w),a0                    | +016
        cmpa.l  #0xffffffff,a0                  | +01a
        beq.w   .L048da4                        | +020
        jsr     0x28cd4.l                       | +024
.L048da4:
        lea     .L048daa(pc),a1                 | +02a
        move.l  a1,(a6)                         | +02e
.L048daa:
        jsr     0x5e45a.l                       | +030
        bcs.w   Jsr5B6ThenJmpScheduler_048b26   | +036
        jsr     0x5e506.l                       | +03a
        subq.w  #0x1,0x38(a6)                   | +040
        jsr     0x28d70.l                       | +044
        movea.l 0xc(a6),a0                      | +04a
        cmpi.b  #0x0,0x83(a0)                   | +04e
        bne.w   .L048dd8                        | +054
        lea     TaskHandler_048d30(pc),a1       | +058
        move.l  a1,(a6)                         | +05c
.L048dd8:
        bra.w   TaskHandler_048e2c              | +05e

| ----------------------------------------------------------------------------
|  TaskHandler_048ddc  @ $048DDC  (16 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048ddc, "ax", @progbits
        .global TaskHandler_048ddc
TaskHandler_048ddc:
        lea     0x28f3cc.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        bra.w   TaskHandler_048dec__L048df8     | +00c

| ----------------------------------------------------------------------------
|  TaskHandler_048dec  @ $048DEC  (56 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048dec, "ax", @progbits
        .global TaskHandler_048dec
TaskHandler_048dec:
        lea     0x28f478.l,a0                   | +000
        jsr     0x28cd4.l                       | +006
        .global TaskHandler_048dec__L048df8
TaskHandler_048dec__L048df8:
        move.l  #0xffffffff,0x48(a6)            | +00c
        lea     .L048e06(pc),a1                 | +014
        move.l  a1,(a6)                         | +018
.L048e06:
        jsr     0x2783a.l                       | +01a
        move.b  0x106f28.l,d0                   | +020
        andi.b  #0x1,d0                         | +026
        beq.w   SetHandlerRts_048e2a            | +02a
        jsr     0x28d70.l                       | +02e
        bcc.w   SetHandlerRts_048e2a            | +034

| ----------------------------------------------------------------------------
|  TaskHandler_048e2c  @ $048E2C  (14 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048e2c, "ax", @progbits
        .global TaskHandler_048e2c
TaskHandler_048e2c:
        movea.l 0xc(a6),a0                      | +000
        cmpi.b  #0x3,0x83(a0)                   | +004
        bne.w   TaskHandler_048e42              | +00a

| ----------------------------------------------------------------------------
|  TaskHandler_048e42  @ $048E42  (10 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048e42, "ax", @progbits
        .global TaskHandler_048e42
TaskHandler_048e42:
        cmpi.b  #0x2,0x83(a0)                   | +000
        bne.w   TaskHandler_048e54              | +006

| ----------------------------------------------------------------------------
|  TaskHandler_048e54  @ $048E54  (74 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_048e54, "ax", @progbits
        .global TaskHandler_048e54
TaskHandler_048e54:
        jsr     0x2870a.l                       | +000
        bcc.w   .L048e64                        | +006
        bclr    #0x3,0x13(a6)                   | +00a
.L048e64:
        jsr     0x28758.l                       | +010
        bcc.w   .L048e94                        | +016
        movea.l 0xc(a6),a0                      | +01a
        move.l  a0,0x50(a0)                     | +01e
        move.b  0x58(a6),0x58(a0)               | +022
        move.w  0x22(a0),0x54(a0)               | +028
        move.w  0x24(a0),0x56(a0)               | +02e
        bset    #0x3,0x13(a0)                   | +034
        lea     TaskHandler_048dec(pc),a1       | +03a
        move.l  a1,(a6)                         | +03e
.L048e94:
        jsr     0x5e45a.l                       | +040
        bcc.w   SetHandlerRts_048ea4            | +046

| ----------------------------------------------------------------------------
|  Sub_00048EA6  @ $048EA6  (94 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048EA6, "ax", @progbits
        .global Sub_00048EA6
Sub_00048EA6:
        jsr     0x267e2.l                       | +000
        cmpi.w  #0xa0,0x22(a6)                  | +006
        bgt.w   .L048ebc                        | +00c
        ori.b   #0x1,0x3a(a6)                   | +010
.L048ebc:
        move.w  #0x38,d1                        | +016
        jsr     0x236e.l                        | +01a
        move.w  #0x1,0x66(a6)                   | +020
        move.l  #0x28df42,0x48(a6)              | +026
        move.l  #0x28e364,0x5c(a6)              | +02e
        lea     0x2bfc32.l,a0                   | +036
        jsr     0x799de.l                       | +03c
        move.w  d0,0x72(a6)                     | +042
        move.w  #0x8000,d0                      | +046
        jsr     0x28134.l                       | +04a
        andi.w  #0xffe3,0x38(a6)                | +050
        ori.w   #0x18,0x38(a6)                  | +056
        rts                                     | +05c

| ----------------------------------------------------------------------------
|  Sub_00048F04  @ $048F04  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048F04, "ax", @progbits
        .global Sub_00048F04
Sub_00048F04:
        move.w  0x36(a6),d0                     | +000
        move.w  d0,d1                           | +004
        btst    #0x0,0x3a(a6)                   | +006
        bne.w   .L048f16                        | +00c
        neg.w   d0                              | +010
.L048f16:
        move.w  d0,0x28(a6)                     | +012
        lsr.w   #0x7,d1                         | +016
        andi.w  #0xc,d1                         | +018
        lea     0x28e354.l,a0                   | +01c
        move.l  (a0,d1.w),0x5c(a6)              | +022
        rts                                     | +028

| ----------------------------------------------------------------------------
|  Sub_00048F2E  @ $048F2E  (32 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048F2E, "ax", @progbits
        .global Sub_00048F2E
Sub_00048F2E:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   .L048f48                        | +00c
        jsr     0x27c8c.l                       | +010
        bra.w   ClearXN_048f4e                  | +016
.L048f48:
        jsr     0x27a92.l                       | +01a

| ----------------------------------------------------------------------------
|  Sub_00048F54  @ $048F54  (92 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048F54, "ax", @progbits
        .global Sub_00048F54
Sub_00048F54:
        move.w  #0xb0,d1                        | +000
        subq.b  #0x2,0x7f(a6)                   | +004
        move.b  0x7f(a6),d0                     | +008
        andi.w  #0xff,d0                        | +00c
        jsr     0x13c0e.l                       | +010
        add.w   0x8c(a6),d1                     | +016
        move.w  d1,0x28(a6)                     | +01a
        asr.w   #0x1,d2                         | +01e
        cmpi.w  #0x0,d2                         | +020
        ble.w   .L048f7e                        | +024
        neg.w   d2                              | +028
.L048f7e:
        addi.w  #0x20,d2                        | +02a
        sub.w   0x8e(a6),d2                     | +02e
        move.w  d2,0x2a(a6)                     | +032
        jsr     0x27cee.l                       | +036
        tst.b   0x80(a6)                        | +03c
        beq.w   .L048fa4                        | +040
        jsr     0x27fac.l                       | +044
        eori.b  #0x11,ccr                       | +04a
        rts                                     | +04e
.L048fa4:
        jsr     0x27eba.l                       | +050
        eori.b  #0x11,ccr                       | +056
        rts                                     | +05a

| ----------------------------------------------------------------------------
|  Sub_00048FB0  @ $048FB0  (22 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048FB0, "ax", @progbits
        .global Sub_00048FB0
Sub_00048FB0:
        jsr     0x2783a.l                       | +000
        jsr     0x27eba.l                       | +006
        bcc.w   ClearXN_048fc6                  | +00c
        jsr     0x27c8c.l                       | +010

| ----------------------------------------------------------------------------
|  Sub_00048FCC  @ $048FCC  (68 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00048FCC, "ax", @progbits
        .global Sub_00048FCC
Sub_00048FCC:
        jsr     Sub_000490FA(pc)                | +000
        bcs.w   .L049000                        | +004
        cmpi.b  #0xc0,0x7d(a6)                  | +008
        beq.w   .L049004                        | +00e
        jsr     TaskHandler_049128(pc)          | +012
        bcc.w   .L049004                        | +016
        movea.l 0x94(a6),a0                     | +01a
        jsr     TaskHandler_04914c(pc)          | +01e
        jsr     0x5e9b6.l                       | +022
        andi.w  #0x1,d0                         | +028
        beq.w   .L049004                        | +02c
        bra.w   .L04900a                        | +030
.L049000:
        clr.w   d0                              | +034
        rts                                     | +036
.L049004:
        move.w  #0x1,d0                         | +038
        rts                                     | +03c
.L04900a:
        move.w  #0x2,d0                         | +03e
        rts                                     | +042

| ----------------------------------------------------------------------------
|  Sub_00049010  @ $049010  (56 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049010, "ax", @progbits
        .global Sub_00049010
Sub_00049010:
        cmpi.b  #0x5,0x77(a6)                   | +000
        bgt.w   .L049030                        | +006
        cmpi.w  #0x0,0x72(a6)                   | +00a
        bgt.w   ClearXN_04904e                  | +010
        clr.w   0x72(a6)                        | +014
        jsr     TaskHandler_049128(pc)          | +018
        bcc.w   ClearXN_04904e                  | +01c
.L049030:
        tst.b   0x98(a6)                        | +020
        beq.w   SetXN_049048                    | +024
        jsr     Sub_0004939C(pc)                | +028
        bcc.w   SetXN_049048                    | +02c
        jsr     TaskHandler_04914c(pc)          | +030
        bcc.w   ClearXN_04904e                  | +034

| ----------------------------------------------------------------------------
|  Sub_00049054  @ $049054  (136 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049054, "ax", @progbits
        .global Sub_00049054
Sub_00049054:
        movea.l 0x94(a6),a0                     | +000
        jsr     0x5e070.l                       | +004
        addq.w  #0x8,d0                         | +00a
        lsr.w   #0x4,d0                         | +00c
        andi.w  #0xf,d0                         | +00e
        tst.w   d0                              | +012
        beq.w   .L04909a                        | +014
        move.w  #0x1,d1                         | +018
        subq.w  #0x4,d0                         | +01c
        bcc.w   .L049084                        | +01e
        cmpi.w  #0x10,0x78(a6)                  | +022
        beq.w   ClearXN_0490f4                  | +028
        bra.w   TaskHandler_0490e2              | +02c
.L049084:
        move.w  #0xffff,d1                      | +030
        subq.w  #0x4,d0                         | +034
        bcc.w   .L04909e                        | +036
        tst.w   0x78(a6)                        | +03a
        beq.w   ClearXN_0490f4                  | +03e
        bra.w   TaskHandler_0490e2              | +042
.L04909a:
        move.w  #0x8,d0                         | +046
.L04909e:
        add.w   d0,d0                           | +04a
        cmp.w   0x78(a6),d0                     | +04c
        beq.w   .L0490ce                        | +050
        bgt.w   .L0490bc                        | +054
        move.w  #0xffff,d1                      | +058
        tst.w   0x78(a6)                        | +05c
        beq.w   ClearXN_0490f4                  | +060
        bra.w   TaskHandler_0490e2              | +064
.L0490bc:
        move.w  #0x1,d1                         | +068
        cmpi.w  #0x10,0x78(a6)                  | +06c
        beq.w   ClearXN_0490f4                  | +072
        bra.w   TaskHandler_0490e2              | +076
.L0490ce:
        cmpi.w  #0x0,0x72(a6)                   | +07a
        bgt.w   ClearXN_0490f4                  | +080
        clr.w   0x72(a6)                        | +084

| ----------------------------------------------------------------------------
|  TaskHandler_0490e2  @ $0490E2  (18 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0490e2, "ax", @progbits
        .global TaskHandler_0490e2
TaskHandler_0490e2:
        move.b  0x106f28.l,d0                   | +000
        andi.b  #0x3,d0                         | +006
        bne.w   ClearXN_0490f4                  | +00a
        add.w   d1,0x78(a6)                     | +00e

| ----------------------------------------------------------------------------
|  Sub_000490FA  @ $0490FA  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000490FA, "ax", @progbits
        .global Sub_000490FA
Sub_000490FA:
        jsr     0x5e0d4.l                       | +000
        bcs.w   SetXN_04911c                    | +006
        jsr     TaskHandler_049128(pc)          | +00a
        bcc.w   ClearXN_049122                  | +00e
        jsr     Sub_0004939C(pc)                | +012
        bcc.w   SetXN_04911c                    | +016
        jsr     TaskHandler_04914c(pc)          | +01a
        bcc.w   ClearXN_049122                  | +01e

| ----------------------------------------------------------------------------
|  TaskHandler_049128  @ $049128  (36 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_049128, "ax", @progbits
        .global TaskHandler_049128
TaskHandler_049128:
        move.b  0x99(a6),d0                     | +000
        andi.w  #0x3,d0                         | +004
        lsl.w   #0x2,d0                         | +008
        lea     0x2bfe3a.l,a1                   | +00a
        movea.l (a1,d0.w),a1                    | +010
        movea.l 0x94(a6),a0                     | +014
        jsr     0x5e260.l                       | +018
        eori.b  #0x11,ccr                       | +01e
        rts                                     | +022

| ----------------------------------------------------------------------------
|  TaskHandler_04914c  @ $04914C  (26 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04914c, "ax", @progbits
        .global TaskHandler_04914c
TaskHandler_04914c:
        movea.l 0x94(a6),a0                     | +000
        jsr     0x5e070.l                       | +004
        move.b  d0,d1                           | +00a
        add.b   0x7e(a6),d1                     | +00c
        and.b   0x7d(a6),d1                     | +010
        cmp.b   d0,d1                           | +014
        bne.w   ClearXN_04916c                  | +016

| ----------------------------------------------------------------------------
|  Sub_00049172  @ $049172  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049172, "ax", @progbits
        .global Sub_00049172
Sub_00049172:
        jsr     TaskHandler_049128(pc)          | +000
        bcs.w   ClearXN_04918a                  | +004
        movea.l 0x94(a6),a0                     | +008
        jsr     TaskHandler_049388(pc)          | +00c
        cmpi.w  #0xc0,d0                        | +010
        bgt.w   SetXN_049190                    | +014

| ----------------------------------------------------------------------------
|  Sub_00049196  @ $049196  (60 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049196, "ax", @progbits
        .global Sub_00049196
Sub_00049196:
        jsr     Sub_000492F8(pc)                | +000
        bcs.w   ClearXN_0491d8                  | +004
        lea     0x28e1b6.l,a0                   | +008
        jsr     0x5e086.l                       | +00e
        bcs.w   ClearXN_0491d8                  | +014
        jsr     TaskHandler_0493c0(pc)          | +018
        bcs.w   ClearXN_0491d8                  | +01c
        jsr     TaskHandler_049388(pc)          | +020
        lea     0x2bfe6a.l,a5                   | +024
        move.b  0x9a(a6),d5                     | +02a
        andi.w  #0x3,d5                         | +02e
        lsl.w   #0x3,d5                         | +032
        cmp.w   (a5,d5.w),d0                    | +034
        bge.w   ClearXN_0491d8                  | +038

| ----------------------------------------------------------------------------
|  Sub_000491DE  @ $0491DE  (52 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000491DE, "ax", @progbits
        .global Sub_000491DE
Sub_000491DE:
        lea     0x28e1b6.l,a0                   | +000
        jsr     0x5e086.l                       | +006
        bcs.w   ClearXN_049218                  | +00c
        jsr     TaskHandler_0493c0(pc)          | +010
        bcs.w   ClearXN_049218                  | +014
        jsr     TaskHandler_049388(pc)          | +018
        lea     0x2bfe6a.l,a5                   | +01c
        move.b  0x9a(a6),d5                     | +022
        andi.w  #0x3,d5                         | +026
        lsl.w   #0x3,d5                         | +02a
        cmp.w   0x2(a5,d5.w),d0                 | +02c
        bge.w   ClearXN_049218                  | +030

| ----------------------------------------------------------------------------
|  Sub_0004921E  @ $04921E  (44 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004921E, "ax", @progbits
        .global Sub_0004921E
Sub_0004921E:
        lea     0x28e1b6.l,a0                   | +000
        jsr     0x5e086.l                       | +006
        bcs.w   ClearXN_049250                  | +00c
        jsr     TaskHandler_049388(pc)          | +010
        lea     0x2bfe6a.l,a5                   | +014
        move.b  0x9a(a6),d5                     | +01a
        andi.w  #0x3,d5                         | +01e
        lsl.w   #0x3,d5                         | +022
        cmp.w   0x4(a5,d5.w),d0                 | +024
        bge.w   ClearXN_049250                  | +028

| ----------------------------------------------------------------------------
|  Sub_00049256  @ $049256  (20 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049256, "ax", @progbits
        .global Sub_00049256
Sub_00049256:
        addq.b  #0x1,0x76(a6)                   | +000
        cmpi.b  #0x1e,0x76(a6)                  | +004
        blt.w   ClearXN_04929e                  | +00a
        move.b  #0x1e,0x76(a6)                  | +00e

| ----------------------------------------------------------------------------
|  Sub_0004926A  @ $04926A  (46 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004926A, "ax", @progbits
        .global Sub_0004926A
Sub_0004926A:
        lea     0x28e1a6.l,a0                   | +000
        jsr     0x5e086.l                       | +006
        bcc.w   ClearXN_04929e                  | +00c
        lea     0x28e1ae.l,a0                   | +010
        jsr     0x5e086.l                       | +016
        bcc.w   SetXN_049298                    | +01c
        movea.l 0x94(a6),a0                     | +020
        jsr     0x5e618.l                       | +024
        bcc.w   ClearXN_04929e                  | +02a

| ----------------------------------------------------------------------------
|  Sub_000492A4  @ $0492A4  (30 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000492A4, "ax", @progbits
        .global Sub_000492A4
Sub_000492A4:
        jsr     0x5e0d4.l                       | +000
        bcs.w   ClearXN_0492c8                  | +006
        jsr     TaskHandler_0493c0(pc)          | +00a
        bcs.w   ClearXN_0492c8                  | +00e
        jsr     TaskHandler_049388(pc)          | +012
        cmpi.w  #0x30,d0                        | +016
        bgt.w   ClearXN_0492c8                  | +01a

| ----------------------------------------------------------------------------
|  TaskHandler_0492ce  @ $0492CE  (30 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0492ce, "ax", @progbits
        .global TaskHandler_0492ce
TaskHandler_0492ce:
        jsr     0x5e0d4.l                       | +000
        bcs.w   ClearXN_0492f2                  | +006
        jsr     TaskHandler_0493c0(pc)          | +00a
        bcs.w   ClearXN_0492f2                  | +00e
        jsr     TaskHandler_049388(pc)          | +012
        cmpi.w  #0x60,d0                        | +016
        bgt.w   ClearXN_0492f2                  | +01a

| ----------------------------------------------------------------------------
|  Sub_000492F8  @ $0492F8  (40 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000492F8, "ax", @progbits
        .global Sub_000492F8
Sub_000492F8:
        cmpi.w  #0x20,0x22(a6)                  | +000
        bgt.w   .L04930c                        | +006
        btst    #0x0,0x7c(a6)                   | +00a
        beq.w   SetXN_049326                    | +010
.L04930c:
        cmpi.w  #0x120,0x22(a6)                 | +014
        blt.w   ClearXN_049320                  | +01a
        btst    #0x0,0x7c(a6)                   | +01e
        bne.w   SetXN_049326                    | +024

| ----------------------------------------------------------------------------
|  Sub_0004932C  @ $04932C  (10 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004932C, "ax", @progbits
        .global Sub_0004932C
Sub_0004932C:
        jsr     0x27eba.l                       | +000
        bcc.w   TaskHandler_04933c              | +006

| ----------------------------------------------------------------------------
|  TaskHandler_04933c  @ $04933C  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_04933c, "ax", @progbits
        .global TaskHandler_04933c
TaskHandler_04933c:
        clr.b   0x75(a6)                        | +000

| ----------------------------------------------------------------------------
|  Sub_00049346  @ $049346  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_00049346, "ax", @progbits
        .global Sub_00049346
Sub_00049346:
        btst    #0x5,0x5a(a6)                   | +000
        beq.w   TaskHandler_049364              | +006
        addq.b  #0x1,0x74(a6)                   | +00a
        cmpi.b  #0x1e,0x74(a6)                  | +00e
        blt.w   TaskHandler_049364              | +014

| ----------------------------------------------------------------------------
|  TaskHandler_049364  @ $049364  (4 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_049364, "ax", @progbits
        .global TaskHandler_049364
TaskHandler_049364:
        clr.b   0x74(a6)                        | +000

| ----------------------------------------------------------------------------
|  Sub_0004936E  @ $04936E  (26 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004936E, "ax", @progbits
        .global Sub_0004936E
Sub_0004936E:
        movea.l 0x94(a6),a0                     | +000
        jsr     0x5e338.l                       | +004
        bcc.w   .L049386                        | +00a
        jsr     0x5e1ea.l                       | +00e
        move.l  a0,0x94(a6)                     | +014
.L049386:
        rts                                     | +018

| ----------------------------------------------------------------------------
|  TaskHandler_049388  @ $049388  (20 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_049388, "ax", @progbits
        .global TaskHandler_049388
TaskHandler_049388:
        move.w  0x22(a6),d0                     | +000
        sub.w   0x22(a0),d0                     | +004
        cmpi.w  #0x0,d0                         | +008
        bge.w   .L04939a                        | +00c
        neg.w   d0                              | +010
.L04939a:
        rts                                     | +012

| ----------------------------------------------------------------------------
|  Sub_0004939C  @ $04939C  (24 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004939C, "ax", @progbits
        .global Sub_0004939C
Sub_0004939C:
        move.w  0x24(a0),d0                     | +000
        sub.w   0x24(a6),d0                     | +004
        cmpi.w  #0x18,d0                        | +008
        bgt.w   SetXN_0493ba                    | +00c
        cmpi.w  #0xfff0,d0                      | +010
        blt.w   SetXN_0493ba                    | +014

| ----------------------------------------------------------------------------
|  TaskHandler_0493c0  @ $0493C0  (24 B)
| ----------------------------------------------------------------------------
        .section .text.TaskHandler_0493c0, "ax", @progbits
        .global TaskHandler_0493c0
TaskHandler_0493c0:
        move.w  0x24(a0),d0                     | +000
        sub.w   0x24(a6),d0                     | +004
        cmpi.w  #0x18,d0                        | +008
        bgt.w   SetXN_0493de                    | +00c
        cmpi.w  #0xfff0,d0                      | +010
        blt.w   SetXN_0493de                    | +014

| ----------------------------------------------------------------------------
|  Sub_000493E4  @ $0493E4  (42 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_000493E4, "ax", @progbits
        .global Sub_000493E4
Sub_000493E4:
        lea     0x28e2d2.l,a0                   | +000
        move.w  0x78(a6),d0                     | +006
        lsr.w   #0x1,d0                         | +00a
        addq.w  #0x8,d0                         | +00c
        move.b  d0,0x7b(a6)                     | +00e
        bra.w   Sub_0004940E__L049418           | +012
        move.b  #0x8,0x7b(a6)                   | +016
        btst    #0x0,0x3a(a6)                   | +01c
        beq.w   Sub_0004940E                    | +022
        clr.b   0x7b(a6)                        | +026

| ----------------------------------------------------------------------------
|  Sub_0004940E  @ $04940E  (34 B)
| ----------------------------------------------------------------------------
        .section .text.Sub_0004940E, "ax", @progbits
        .global Sub_0004940E
Sub_0004940E:
        lea     0x28e292.l,a0                   | +000
        move.b  0x7b(a6),d0                     | +006
        .global Sub_0004940E__L049418
Sub_0004940E__L049418:
        andi.l  #0xf,d0                         | +00a
        lsl.w   #0x2,d0                         | +010
        add.l   a0,d0                           | +012
        move.l  d0,0x5c(a6)                     | +014
        lea     TaskHandler_048b56(pc),a1       | +018
        jsr     0x4ae.l                         | +01c
